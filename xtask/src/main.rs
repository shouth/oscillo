use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{command, Command};
use xshell::{cmd, Shell};

fn project_root() -> PathBuf {
    let dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());
    PathBuf::from(dir).parent().unwrap().to_owned()
}

fn generate_char_set() {
    let tmp_dir =
        tempdir::TempDir::new("oscelas-xtask").expect("Failed to create a temporary directory");
    let unic_ver = "15.1.0";
    let ucd_dir = tmp_dir.path().join("UCD");
    let gen_dir = project_root().join("oscelas/src/chars/generated");

    let sh = Shell::new().expect("Failed to create a shell");
    cmd!(
        sh,
        "curl -L https://www.unicode.org/Public/zipped/{unic_ver}/UCD.zip -o {ucd_dir}.zip"
    )
    .run()
    .expect("Failed to download UCD.zip");

    cmd!(sh, "unzip -q {ucd_dir}.zip -d {ucd_dir}")
        .run()
        .expect("Failed to unzip UCD.zip");

    do_generate_char_set(
        &ucd_dir,
        &gen_dir.join("id_start_char_set.rs"),
        "ID_START_CHAR_SET",
        &["Lu", "Ll", "Lt", "Lm", "Lo", "Nl"],
    );
    do_generate_char_set(
        &ucd_dir,
        &gen_dir.join("id_char_set.rs"),
        "ID_CHAR_SET",
        &["Lu", "Ll", "Lt", "Lm", "Lo", "Nl", "Mc", "Mn", "Nd", "Pc"],
    );
}

fn do_generate_char_set(ucd_path: &Path, out_path: &Path, var_name: &str, categories: &[&str]) {
    let sh = Shell::new().expect("Failed to create a shell");
    let categories_str = categories.join(",");

    let code = cmd!(sh, "ucd-generate general-category {ucd_path} --trie-set --include {categories_str} --combined --name {var_name}")
        .read()
        .expect("Failed to generate char set");

    sh.write_file(out_path, code)
        .expect("Failed to write char set");
}

fn main() {
    let matches = command!()
        .subcommand(Command::new("generate_char_set"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("generate_char_set") {
        generate_char_set();
    }
}
