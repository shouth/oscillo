use std::path::Path;

use xshell::{cmd, Shell};

pub fn generate_char_set(ucd_path: &Path) -> (String, String) {
    let id_start_char_set_src = do_generate_char_set(
        ucd_path,
        "ID_START_CHAR_SET",
        &["Lu", "Ll", "Lt", "Lm", "Lo", "Nl"],
    );
    let id_char_set_src = do_generate_char_set(
        ucd_path,
        "ID_CHAR_SET",
        &["Lu", "Ll", "Lt", "Lm", "Lo", "Nl", "Mc", "Mn", "Nd", "Pc"],
    );

    (id_start_char_set_src, id_char_set_src)
}

fn do_generate_char_set(ucd_path: &Path, var_name: &str, categories: &[&str]) -> String {
    let sh = Shell::new().expect("Failed to create a shell");
    let categories_str = categories.join(",");

    cmd!(sh, "ucd-generate general-category {ucd_path} --trie-set --include {categories_str} --combined --name {var_name}")
        .read()
        .expect("Failed to generate char set")
}
