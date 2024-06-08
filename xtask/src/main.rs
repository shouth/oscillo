mod generate_char_set;
mod generate_syntax_kind;
mod generate_syntax_node;
mod grammar;
mod oscdsl;
mod syntax_name;

use clap::{Parser, Subcommand};
use std::env;
use std::path::{Path, PathBuf};
use xshell::{cmd, Shell};

use generate_char_set::generate_char_set;
use generate_syntax_kind::generate_syntax_kind;
use generate_syntax_node::generate_syntax_node;

pub fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir).parent().unwrap().to_owned()
}

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Download {
        #[clap(subcommand)]
        command: DownloadCommand,
    },
    Generate {
        #[clap(subcommand)]
        command: GenerateCommand,
    },
}

#[derive(Subcommand)]
enum DownloadCommand {
    Ucd {
        #[clap(long)]
        version: Option<String>,

        #[clap(long)]
        path: String,
    },
}

#[derive(Subcommand)]
enum GenerateCommand {
    CharSet {
        #[clap(long)]
        ucd_path: Option<String>,
    },
    SyntaxKind,
    SyntaxNode,
}

fn download_ucd(version: String, path: &Path) {
    let sh = Shell::new().expect("Failed to create a shell");
    cmd!(
        sh,
        "curl -L https://www.unicode.org/Public/zipped/{version}/UCD.zip -o {path}.zip"
    )
    .run()
    .expect("Failed to download UCD.zip");

    cmd!(sh, "unzip -q {path}.zip -d {path}")
        .run()
        .expect("Failed to unzip UCD.zip");

    cmd!(sh, "rm {path}.zip")
        .run()
        .expect("Failed to remove UCD.zip");
}

fn format_src(src: &str) -> String {
    let sh = Shell::new().expect("Failed to create a shell");
    cmd!(sh, "rustfmt --emit=stdout")
        .stdin(src)
        .read()
        .expect("Failed to format source code")
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Download { command } => match command {
            DownloadCommand::Ucd { version, path } => {
                let version = version.unwrap_or_else(|| "latest".to_owned());
                let path = PathBuf::from(path);
                download_ucd(version, &path);
            }
        },
        Command::Generate { command } => match command {
            GenerateCommand::CharSet { ucd_path } => {
                let sh = Shell::new().expect("Failed to create a shell");
                let (id_start_char_set_src, id_char_set_src) = match ucd_path {
                    Some(ucd_path) => {
                        let ucd_path = Path::new(&ucd_path);
                        generate_char_set(ucd_path)
                    }
                    None => {
                        let temp_dir = sh
                            .create_temp_dir()
                            .expect("Failed to create a temporary directory");
                        download_ucd("latest".to_owned(), temp_dir.path());
                        generate_char_set(temp_dir.path())
                    }
                };

                let base_path = project_root().join("oscelas/src/chars/generated");
                sh.write_file(
                    base_path.join("id_start_char_set.rs"),
                    id_start_char_set_src,
                )
                .expect("Failed to write id_start_char_set.rs");
                sh.write_file(base_path.join("id_char_set.rs"), id_char_set_src)
                    .expect("Failed to write id_char_set.rs");
            }
            GenerateCommand::SyntaxKind => {
                let sh = Shell::new().expect("Failed to create a shell");
                let grammar = oscdsl::grammar();
                let syntax_kind_src = generate_syntax_kind(&grammar);
                let syntax_kind_src = format_src(&syntax_kind_src);

                let base_path = project_root().join("oscelas/src/syntax/generated");
                sh.write_file(base_path.join("kind.rs"), syntax_kind_src)
                    .expect("Failed to write kind.rs");
            }
            GenerateCommand::SyntaxNode => {
                let sh = Shell::new().expect("Failed to create a shell");
                let grammar = oscdsl::grammar();
                let syntax_node_src = generate_syntax_node(&grammar);
                let syntax_node_src = format_src(&syntax_node_src);

                let base_path = project_root().join("oscelas/src/syntax/generated");
                sh.write_file(base_path.join("node.rs"), syntax_node_src)
                    .expect("Failed to write node.rs");
            }
        },
    }
}
