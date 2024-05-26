mod syntax;

use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{command, Command};
use quote::{format_ident, quote};
use syntax::{Tokens, DSL_TOKENS};
use ungrammar::Grammar;
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

fn generate_dsl_syntax() {
    let grammar = include_str!("osc_dsl.ungram");
    let grammar = grammar
        .parse::<Grammar>()
        .expect("Failed to parse the grammar");

    generate_dsl_syntax_kinds(&DSL_TOKENS, &grammar);
}

fn generate_dsl_syntax_kinds(tokens: &Tokens, grammer: &Grammar) {
    let gen_dir = project_root().join("oscelas/src/syntax/generated");

    let sh = Shell::new().expect("Failed to create a shell");

    let puncts = tokens
        .punct
        .iter()
        .map(|(_, name)| format_ident!("{name}"));

    let keywords = tokens
        .keyword
        .iter()
        .map(|kw| kw.to_ascii_uppercase())
        .map(|kw| format_ident!("{kw}_KW"));

    let literals = tokens
        .literal
        .iter()
        .map(|lit| format_ident!("{lit}"));

    let tokens = tokens
        .token
        .iter()
        .map(|tok| format_ident!("{tok}"));

    let nodes = grammer
        .iter()
        .map(|node| to_upper_snake_case(&grammer[node].name))
        .map(|node| format_ident!("{node}"));

    let code = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum SyntaxKind {
            #[doc(hidden)]
            TOMBSTONE,
            EOF,
            #(#puncts,)*
            #(#keywords,)*
            #(#literals,)*
            #(#tokens,)*
            #(#nodes,)*

            #[doc(hidden)]
            __LAST,
        }
    };

    let code = cmd!(sh, "rustfmt --emit stdout")
        .stdin(code.to_string())
        .read()
        .expect("Failed to format the code");

    sh.write_file(gen_dir.join("syntax_kind.rs"), code)
        .expect("Failed to write syntax_kind.rs");
}

fn to_upper_snake_case(s: &str) -> String {
    // `s` is expected to be represented in UpperCamelCase
    let mut result = String::new();
    let mut first = true;
    for c in s.chars() {
        if c.is_ascii_uppercase() && !first {
            result.push('_');
        }
        first = false;
        result.push(c.to_ascii_uppercase());
    }
    result
}

fn main() {
    let matches = command!()
        .subcommand(Command::new("generate_char_set"))
        .subcommand(Command::new("generate_dsl_syntax"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("generate_char_set") {
        generate_char_set();
    } else if let Some(_) = matches.subcommand_matches("generate_dsl_syntax") {
        generate_dsl_syntax();
    }
}
