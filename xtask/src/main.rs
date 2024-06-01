mod oscdsl;
mod syntax_spec;

use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{command, Command};
use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use syntax_spec::{
    RuleOrToken, RuleSpecData, RuleSpecKind, SyntaxSpec, TokenSpecData, TokenSpecKind,
};
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
    let spec = oscdsl::load_spec().expect("Failed to load the syntax spec");
    generate_dsl_syntax_kinds(&spec);
    generate_dsl_syntax_node(&spec);
}

fn generate_dsl_syntax_kinds(syntax: &SyntaxSpec) {
    let gen_dir = project_root().join("oscelas/src/syntax/generated");

    let sh = Shell::new().expect("Failed to create a shell");

    let token_kinds = syntax
        .tokens
        .iter()
        .map(|TokenSpecData { token, kind }| match kind {
            TokenSpecKind::Punct { name } => format_ident!("{}", name.to_ascii_uppercase()),
            TokenSpecKind::Keyword => format_ident!("{}_KW", token.to_ascii_uppercase()),
            _ => format_ident!("{}", token.to_ascii_uppercase()),
        })
        .map(|name| format_ident!("{name}"));

    let node_kinds = syntax
        .rules
        .iter()
        .map(|RuleSpecData { name, .. }| format_ident!("{}", name.to_case(Case::UpperSnake)));

    let list_kinds = syntax
        .rules
        .iter()
        .filter_map(|RuleSpecData { name, kind }| {
            matches!(kind, RuleSpecKind::List { .. })
                .then(|| format_ident!("{}", name.to_case(Case::UpperSnake)))
        });

    let to_string_arms =
        syntax
            .tokens
            .iter()
            .filter_map(|TokenSpecData { token, kind }| match kind {
                TokenSpecKind::Punct { name } => {
                    let key = format_ident!("{}", name.to_case(Case::UpperSnake));
                    Some(quote! { #key => #token })
                }
                TokenSpecKind::Keyword => {
                    let key = format_ident!("{}_KW", token.to_case(Case::UpperSnake));
                    Some(quote! { #key => #token })
                }
                _ => None,
            });

    let code = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum OscDslSyntaxKind {
            #[doc(hidden)]
            TOMBSTONE,
            EOF,
            #(#token_kinds,)*
            #(#node_kinds,)*

            #[doc(hidden)]
            __LAST,
        }

        use self::OscDslSyntaxKind::*;
        impl OscDslSyntaxKind {
            pub fn is_list(self) -> bool {
                matches!(self, #(#list_kinds)|*)
            }

            pub fn to_string(self) -> Option<&'static str> {
                let result = match self {
                    #(#to_string_arms,)*
                    _ => return None,
                };
                return Some(result);
            }
        }
    };

    let code = cmd!(sh, "rustfmt --emit stdout")
        .stdin(code.to_string())
        .read()
        .expect("Failed to format the code");

    sh.write_file(gen_dir.join("kind.rs"), code)
        .expect("Failed to write kind.rs");
}

fn generate_dsl_syntax_node(spec: &SyntaxSpec) {
    let gen_dir = project_root().join("oscelas/src/syntax/generated");

    let sh = Shell::new().expect("Failed to create a shell");

    let ast_nodes = spec.rules.iter().map(|rule| {
        let name = format_ident!("{}", rule.name);

        match &rule.kind {
            RuleSpecKind::Aggregate(fields) => {
                let methods = fields.iter().enumerate().map(|(i, item)| {
                    let name = match &item.label {
                        Some(label) => format_ident!("{}", label),
                        None => match &item.inner {
                            RuleOrToken::Token(token) => {
                                let name = match &spec[*token].kind {
                                    TokenSpecKind::Punct { name } => name,
                                    _ => &spec[*token].token,
                                };
                                format_ident!("{}_token", name.to_case(Case::Snake))
                            }
                            RuleOrToken::Rule(rule) => {
                                format_ident!("{}", spec[*rule].name.to_case(Case::Snake))
                            }
                        },
                    };
                    let retern_type = match &item.inner {
                        RuleOrToken::Token(_) => {
                            if item.mandatory {
                                quote! { SyntaxResult<SyntaxToken<OscDslLanguage>> }
                            } else {
                                quote! { Option<SyntaxToken<OscDslLanguage>> }
                            }
                        }
                        RuleOrToken::Rule(rule) => {
                            let node = format_ident!("{}", spec[*rule].name.to_case(Case::Pascal));
                            if matches!(&spec[*rule].kind, RuleSpecKind::List {..} | RuleSpecKind::SeparatedList { .. }) {
                                quote! { #node }
                            } else if item.mandatory {
                                quote! { SyntaxResult<#node> }
                            } else {
                                quote! { Option<#node> }
                            }
                        }
                    };
                    let support_method = match &item.inner {
                        RuleOrToken::Token(_) => {
                            if item.mandatory {
                                quote! { support::required_token }
                            } else {
                                quote! { support::token }
                            }
                        }
                        RuleOrToken::Rule(rule) => {
                            if matches!(&spec[*rule].kind, RuleSpecKind::List {..} | RuleSpecKind::SeparatedList { .. }) {
                                quote! { support::list }
                            } else if item.mandatory {
                                quote! { support::required_node }
                            } else {
                                quote! { support::node }
                            }
                        }
                    };

                    quote! {
                        pub fn #name(&self) -> #retern_type {
                            #support_method(&self.syntax, #i)
                        }
                    }
                });

                let kind = format_ident!("{}", rule.name.to_case(Case::UpperSnake));

                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #name {
                        syntax: SyntaxNode<OscDslLanguage>,
                    }

                    impl #name {
                        #(#methods)*
                    }

                    impl AstNode for #name {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> =
                            SyntaxKindSet::from_raw(RawSyntaxKind(#kind as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #kind
                        }

                        fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
                            Self::can_cast(syntax.kind()).then(|| Self { syntax })
                        }

                        fn syntax(&self) -> &SyntaxNode<Self::Language> {
                            &self.syntax
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            self.syntax
                        }
                    }
                }
            }
            RuleSpecKind::List { element } => {
                let kind = format_ident!("{}", rule.name.to_case(Case::UpperSnake));
                let element_kind = format_ident!("{}", spec[*element].name.to_case(Case::Pascal));

                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #name {
                        syntax_list: SyntaxList<OscDslLanguage>,
                    }

                    impl AstNode for #name {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> =
                            SyntaxKindSet::from_raw(RawSyntaxKind(#kind as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #kind
                        }

                        fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
                            let syntax_list = syntax.into_list();
                            Self::can_cast(syntax.kind()).then(|| Self { syntax_list })
                        }

                        fn syntax(&self) -> &SyntaxNode<Self::Language> {
                            self.syntax_list.node()
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            self.syntax_list.into_node()
                        }
                    }

                    impl AstNodeList for #name {
                        type Language = OscDslLanguage;
                        type Node = #element_kind;

                        fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
                            &self.syntax_list
                        }

                        fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
                            self.syntax_list
                        }
                    }
                }
            }
            RuleSpecKind::SeparatedList { element, .. } => {
                let kind = format_ident!("{}", rule.name.to_case(Case::UpperSnake));
                let element_kind = format_ident!("{}", spec[*element].name.to_case(Case::Pascal));

                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #name {
                        syntax_list: SyntaxList<OscDslLanguage>,
                    }

                    impl AstNode for #name {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> =
                            SyntaxKindSet::from_raw(RawSyntaxKind(#kind as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #kind
                        }

                        fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
                            let syntax_list = syntax.into_list();
                            Self::can_cast(syntax.kind()).then(|| Self { syntax_list })
                        }

                        fn syntax(&self) -> &SyntaxNode<Self::Language> {
                            self.syntax_list.node()
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            self.syntax_list.into_node()
                        }
                    }

                    impl AstSeparatedList for #name {
                        type Language = OscDslLanguage;
                        type Node = #element_kind;

                        fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
                            &self.syntax_list
                        }

                        fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
                            self.syntax_list
                        }
                    }
                }
            }
            RuleSpecKind::RuleVariant(variants) => {
                let items = variants.iter().map(|item| {
                    let node_type = format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal));
                    let name = match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => node_type.clone(),
                    };
                    quote! { #name(#node_type) }
                });

                let methods = variants.iter().map(|item| {
                    let method_name = match &item.label {
                        Some(label) => format_ident!("as_{}", label.to_case(Case::Snake)),
                        None => format_ident!("as_{}", spec[item.rule].name.to_case(Case::Snake)),
                    };
                    let node_type = format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal));
                    let kind = match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal)),
                    };

                    quote! {
                        pub fn #method_name(&self) -> Option<&#node_type> {
                            match self {
                                #name::#kind(node) => Some(node),
                                _ => None,
                            }
                        }
                    }
                });

                let kinds = variants.iter().map(|item| {
                    let kind = format_ident!("{}", spec[item.rule].name.to_case(Case::UpperSnake));
                    quote! { #kind }
                });

                let kind_set = variants.iter().enumerate().map(|(i, item)| {
                    let node = format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal));
                    if i == 0 {
                        quote! { #node::KIND_SET }
                    } else {
                        quote! { .union(#node::KIND_SET) }
                    }
                });

                let cast_arms = variants.iter().map(|item| {
                    let kind = format_ident!("{}", spec[item.rule].name.to_case(Case::UpperSnake));
                    let name = match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal)),
                    };
                    let node = format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal));

                    quote! { #kind => Self::#name(#node::cast(syntax)?) }
                });

                let syntax_arms = variants.iter().map(|item| {
                    let kind = format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal));
                    quote! { Self::#kind(node) => node.syntax(), }
                });

                let into_syntax_arms = variants.iter().map(|item| {
                    let kind = format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal));
                    quote! { Self::#kind(node) => node.into_syntax(), }
                });

                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub enum #name {
                        #(#items),*
                    }

                    impl #name {
                        #(#methods)*
                    }

                    impl AstNode for #name {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> = #(#kind_set)*;

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            matches!(kind, #(#kinds)|*)
                        }

                        fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
                            let result = match syntax.kind() {
                                #(#cast_arms,)*
                                _ => return None,
                            };
                            Some(result)
                        }

                        fn syntax(&self) -> &SyntaxNode<Self::Language> {
                            match &self {
                                #(#syntax_arms)*
                            }
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            match self {
                                #(#into_syntax_arms)*
                            }
                        }
                    }
                }
            }
            RuleSpecKind::TokenVariant(variants) => {
                let items = variants.iter().map(|item| {
                    let name = match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => match &spec[item.token].kind {
                            TokenSpecKind::Punct { name } => {
                                format_ident!("{}", name.to_case(Case::Pascal))
                            }
                            _ => format_ident!("{}", spec[item.token].token.to_case(Case::Pascal)),
                        },
                    };
                    quote! { #name }
                });
                let kind = format_ident!("{}", rule.name.to_case(Case::UpperSnake));
                let kind_name = format_ident!("{name}Kind");
                let kind_arms = variants.iter().map(|item| {
                    let key = match &spec[item.token].kind {
                        TokenSpecKind::Punct { name } => format_ident!("{}", name.to_case(Case::UpperSnake)),
                        TokenSpecKind::Keyword => format_ident!("{}_KW", spec[item.token].token.to_case(Case::UpperSnake)),
                        _ => format_ident!("{}", spec[item.token].token.to_case(Case::UpperSnake)),
                    };
                    let value = match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => match &spec[item.token].kind {
                            TokenSpecKind::Punct { name } => {
                                format_ident!("{}", name.to_case(Case::Pascal))
                            }
                            _ => format_ident!("{}", spec[item.token].token.to_case(Case::Pascal)),
                        },
                    };
                    quote! { #key => #kind_name::#value }
                });

                quote! {
                    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                    pub enum #kind_name {
                        #(#items),*
                    }

                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #name {
                        syntax: SyntaxNode<OscDslLanguage>,
                    }

                    impl #name {
                        pub fn kind(&self) -> #kind_name {
                            match self.syntax.kind() {
                                #(#kind_arms,)*
                                _ => unreachable!(),
                            }
                        }

                        pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
                            support::required_token(&self.syntax, 0usize)
                        }
                    }

                    impl AstNode for #name {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(#kind as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #kind
                        }

                        fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
                            Self::can_cast(syntax.kind()).then(|| Self { syntax })
                        }

                        fn syntax(&self) -> &SyntaxNode<Self::Language> {
                            &self.syntax
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            self.syntax
                        }
                    }
                }
            }
        }
    });

    let code = quote! {
        use super::OscDslSyntaxKind::{self, *};
        use crate::syntax::OscDslLanguage;
        use biome_rowan::{
            support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxList, SyntaxNode, SyntaxResult, SyntaxToken
        };

        #(#ast_nodes)*
    };

    let code = cmd!(sh, "rustfmt --emit stdout")
        .stdin(code.to_string())
        .read()
        .expect("Failed to format the code");

    sh.write_file(gen_dir.join("node.rs"), code)
        .expect("Failed to write node.rs");
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
