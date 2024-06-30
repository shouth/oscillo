mod syntax;

use std::env;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

use syntax::{generate_char_set, generate_syntax_kind, generate_syntax_node, osc};

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
    Serve {
        #[clap(subcommand)]
        command: ServeCommand,
    }
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
    Syntax,
}

#[derive(Subcommand)]
enum ServeCommand {
    AstExplorer,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                        let temp_dir = sh.create_temp_dir()?;
                        download_ucd("latest".to_owned(), temp_dir.path());
                        generate_char_set(temp_dir.path())
                    }
                };

                let base_path = project_root().join("oscelas/src/chars/generated");
                sh.write_file(
                    base_path.join("id_start_char_set.rs"),
                    id_start_char_set_src,
                )?;
                sh.write_file(base_path.join("id_char_set.rs"), id_char_set_src)?;
            }
            GenerateCommand::Syntax => {
                let sh = Shell::new().expect("Failed to create a shell");
                let grammar = osc::grammar();

                let syntax_kind_src = generate_syntax_kind(&grammar);
                let syntax_kind_src = format_src(&syntax_kind_src);

                let syntax_node_src = generate_syntax_node(&grammar);
                let syntax_node_src = format_src(&syntax_node_src);

                let base_path = project_root().join("crates/osc-parser/src/syntax/generated");
                sh.write_file(base_path.join("node.rs"), syntax_node_src)?;
                sh.write_file(base_path.join("kind.rs"), syntax_kind_src)?;
            }
        },
        Command::Serve { command } => match command {
            ServeCommand::AstExplorer => {
                let sh = Shell::new().expect("Failed to create a shell");
                let base_path = project_root().join("crates/osc-ast-explorer");
                sh.change_dir(base_path);
                cmd!(sh, "trunk serve").run()?;
            }
        }
    }

    Ok(())
}
