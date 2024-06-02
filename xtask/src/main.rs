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

fn syntax_kind_name(data: &TokenSpecData) -> String {
    match &data.kind {
        TokenSpecKind::Punct { name } => format!("{}", name.to_case(Case::UpperSnake)),
        TokenSpecKind::Keyword => format!("{}_KW", data.token.to_case(Case::UpperSnake)),
        _ => format!("{}", data.token.to_case(Case::UpperSnake)),
    }
}

fn is_repitive_rule(data: &RuleSpecData) -> bool {
    matches!(
        data.kind,
        RuleSpecKind::List { .. } | RuleSpecKind::SeparatedList { .. }
    )
}

fn generate_dsl_syntax() {
    let spec = oscdsl::load_spec().expect("Failed to load the syntax spec");
    generate_dsl_syntax_kinds(&spec);
    generate_dsl_syntax_node(&spec);
    generate_dsl_syntax_factory(&spec);
}

fn generate_dsl_syntax_kinds(syntax: &SyntaxSpec) {
    let gen_dir = project_root().join("oscelas/src/syntax/generated");

    let sh = Shell::new().expect("Failed to create a shell");

    let token_kind_idents = syntax
        .tokens
        .iter()
        .map(|data| format_ident!("{}", syntax_kind_name(&data)));

    let rule_kind_idents = syntax
        .rules
        .iter()
        .map(|data| format_ident!("{}", data.name.to_case(Case::UpperSnake)));

    let list_rule_kind_idents = syntax
        .rules
        .iter()
        .zip(rule_kind_idents.clone())
        .filter_map(|(data, ident)| is_repitive_rule(data).then(|| ident));

    let to_string_arms = syntax
        .tokens
        .iter()
        .zip(token_kind_idents.clone())
        .filter_map(|(TokenSpecData { token, kind }, ident)| {
            matches!(kind, TokenSpecKind::Punct { .. } | TokenSpecKind::Keyword)
                .then(|| quote! { #ident => #token })
        });

    let code = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum OscDslSyntaxKind {
            #[doc(hidden)]
            TOMBSTONE,
            EOF,
            #(#token_kind_idents,)*
            #(#rule_kind_idents,)*

            #[doc(hidden)]
            __LAST,
        }

        use self::OscDslSyntaxKind::*;
        impl OscDslSyntaxKind {
            pub fn is_list(self) -> bool {
                matches!(self, #(#list_rule_kind_idents)|*)
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
        let rule_name_ident = format_ident!("{}", rule.name);

        match &rule.kind {
            RuleSpecKind::Aggregate(fields) => {
                let methods = fields.iter().enumerate().map(|(i, item)| {
                    let method_name_ident = match &item.label {
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
                            if is_repitive_rule(&spec[*rule]) {
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
                            if is_repitive_rule(&spec[*rule]) {
                                quote! { support::list }
                            } else if item.mandatory {
                                quote! { support::required_node }
                            } else {
                                quote! { support::node }
                            }
                        }
                    };

                    quote! {
                        pub fn #method_name_ident(&self) -> #retern_type {
                            #support_method(&self.syntax, #i)
                        }
                    }
                });

                let syntax_kind_ident = format_ident!("{}", rule.name.to_case(Case::UpperSnake));
                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #rule_name_ident {
                        syntax: SyntaxNode<OscDslLanguage>,
                    }

                    impl #rule_name_ident {
                        #(#methods)*
                    }

                    impl AstNode for #rule_name_ident {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> =
                            SyntaxKindSet::from_raw(RawSyntaxKind(#syntax_kind_ident as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #syntax_kind_ident
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
            kind @ (RuleSpecKind::List { element } | RuleSpecKind::SeparatedList { element, .. }) => {
                let syntax_kind_ident = format_ident!("{}", rule.name.to_case(Case::UpperSnake));
                let element_kind_ident = format_ident!("{}", spec[*element].name.to_case(Case::Pascal));
                let list_trait_ident = match kind {
                    RuleSpecKind::List { .. } => quote! { AstNodeList },
                    RuleSpecKind::SeparatedList { .. } => quote! { AstSeparatedList },
                    _ => unreachable!(),
                };

                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #rule_name_ident {
                        syntax_list: SyntaxList<OscDslLanguage>,
                    }

                    impl AstNode for #rule_name_ident {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> =
                            SyntaxKindSet::from_raw(RawSyntaxKind(#syntax_kind_ident as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #syntax_kind_ident
                        }

                        fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
                            Self::can_cast(syntax.kind()).then(|| Self { syntax_list: syntax.into_list() })
                        }

                        fn syntax(&self) -> &SyntaxNode<Self::Language> {
                            self.syntax_list.node()
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            self.syntax_list.into_node()
                        }
                    }

                    impl #list_trait_ident for #rule_name_ident {
                        type Language = OscDslLanguage;
                        type Node = #element_kind_ident;

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
                let variant_item_idents = variants.iter().map(|item| {
                    match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal)),
                    }
                });

                let variant_rule_idents = variants.iter().map(|item| {
                    format_ident!("{}", spec[item.rule].name.to_case(Case::Pascal))
                });

                let variant_ident_pairs = variant_item_idents.clone().zip(variant_rule_idents.clone());

                let items = variant_ident_pairs.clone()
                    .map(|(item, rule)| quote! { #item(#rule) });

                let methods = variants.iter().zip(variant_ident_pairs.clone())
                    .map(|(item, (item_ident, rule_ident))| {
                        let method_name = match &item.label {
                            Some(label) => format_ident!("as_{}", label.to_case(Case::Snake)),
                            None => format_ident!("as_{}", spec[item.rule].name.to_case(Case::Snake)),
                        };

                        quote! {
                            pub fn #method_name(&self) -> Option<&#rule_ident> {
                                match self {
                                    #rule_name_ident::#item_ident(node) => Some(node),
                                    _ => None,
                                }
                            }
                        }
                    });

                let syntax_kind_idents = variants.iter().map(|item| {
                    format_ident!("{}", spec[item.rule].name.to_case(Case::UpperSnake))
                });

                let kind_set = variant_rule_idents.clone()
                    .enumerate()
                    .map(|(i, rule_ident)| {
                        if i == 0 {
                            quote! { #rule_ident::KIND_SET }
                        } else {
                            quote! { .union(#rule_ident::KIND_SET) }
                        }
                    });

                let cast_arms = syntax_kind_idents.clone()
                    .zip(variant_ident_pairs.clone())
                    .map(|(kind_ident, (item_ident, rule_ident))| {
                        quote! { #kind_ident => Self::#item_ident(#rule_ident::cast(syntax)?) }
                    });

                let syntax_arms = variant_item_idents.clone()
                    .map(|item_ident| quote! { Self::#item_ident(node) => node.syntax() });

                let into_syntax_arms = variant_item_idents.clone()
                    .map(|item_ident| quote! { Self::#item_ident(node) => node.into_syntax() });

                quote! {
                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub enum #rule_name_ident {
                        #(#items,)*
                    }

                    impl #rule_name_ident {
                        #(#methods)*
                    }

                    impl AstNode for #rule_name_ident {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> = #(#kind_set)*;

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            matches!(kind, #(#syntax_kind_idents)|*)
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
                                #(#syntax_arms,)*
                            }
                        }

                        fn into_syntax(self) -> SyntaxNode<Self::Language> {
                            match self {
                                #(#into_syntax_arms,)*
                            }
                        }
                    }
                }
            }
            RuleSpecKind::TokenVariant(variants) => {
                let variant_item_idents = variants.iter()
                    .map(|item| match &item.label {
                        Some(label) => format_ident!("{}", label.to_case(Case::Pascal)),
                        None => match &spec[item.token].kind {
                            TokenSpecKind::Punct { name } => {
                                format_ident!("{}", name.to_case(Case::Pascal))
                            }
                            _ => format_ident!("{}", spec[item.token].token.to_case(Case::Pascal)),
                        },
                    }
                );

                let syntax_kind_ident = format_ident!("{}", rule.name.to_case(Case::UpperSnake));

                let variant_kind_ident = format_ident!("{rule_name_ident}Kind");

                let kind_arms = variants.iter()
                    .zip(variant_item_idents.clone())
                    .map(|(item, item_ident)| {
                        let key = format_ident!("{}", syntax_kind_name(&spec[item.token]));
                        quote! { #key => #variant_kind_ident::#item_ident }
                    });

                quote! {
                    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                    pub enum #variant_kind_ident {
                        #(#variant_item_idents,)*
                    }

                    #[derive(Debug, Clone, PartialEq, Eq)]
                    pub struct #rule_name_ident {
                        syntax: SyntaxNode<OscDslLanguage>,
                    }

                    impl #rule_name_ident {
                        pub fn kind(&self) -> #variant_kind_ident {
                            match self.syntax.kind() {
                                #(#kind_arms,)*
                                _ => unreachable!(),
                            }
                        }

                        pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
                            support::required_token(&self.syntax, 0usize)
                        }
                    }

                    impl AstNode for #rule_name_ident {
                        type Language = OscDslLanguage;

                        const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(#syntax_kind_ident as u16));

                        fn can_cast(kind: OscDslSyntaxKind) -> bool {
                            kind == #syntax_kind_ident
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
            support, AstNode, AstNodeList, AstSeparatedList, RawSyntaxKind, SyntaxKindSet, SyntaxList, SyntaxNode, SyntaxResult, SyntaxToken
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

fn generate_dsl_syntax_factory(spec: &SyntaxSpec) {
    let gen_dir = project_root().join("oscelas/src/syntax/generated");

    let sh = Shell::new().expect("Failed to create a shell");

    let make_syntax_arms = spec.rules.iter()
        .filter_map(|data| {
            let syntax_kind_ident = format_ident!("{}", data.name.to_case(Case::UpperSnake));
            let body = match &data.kind {
                RuleSpecKind::Aggregate(variants) => {
                    let slot_size = variants.len();
                    let slot_checks = variants
                        .iter()
                        .map(|item| {
                            let condition = match &item.inner {
                                RuleOrToken::Rule(rule) => {
                                    let rule_ident = format_ident!("{}", spec[*rule].name.to_case(Case::Pascal));
                                    quote! {
                                        #rule_ident::can_cast(element.kind())
                                    }
                                }
                                RuleOrToken::Token(token) => {
                                    let token_kind_ident = format_ident!("{}", syntax_kind_name(&spec[*token]));
                                    quote! {
                                        element.kind() == #token_kind_ident
                                    }
                                }
                            };
                            quote! {
                                if let Some(element) = &current_element {
                                    if #condition {
                                        slots.mark_present();
                                        current_element = elements.next();
                                    }
                                }
                                slots.next_slot();
                            }
                        });

                    quote! {{
                        let mut elements = (&children).into_iter();
                        let mut slots: RawNodeSlots<#slot_size> = RawNodeSlots::default();
                        let mut current_element = elements.next();

                        #(#slot_checks)*

                        if current_element.is_some() {
                            return RawSyntaxNode::new(#syntax_kind_ident.to_bogus(), children.into_iter().map(Some));
                        }
                        slots.into_node(kind, children)
                    }}
                }
                RuleSpecKind::List { element } => {
                    let rule_ident = format_ident!("{}", spec[*element].name.to_case(Case::Pascal));
                    quote! {
                        Self::make_node_list_syntax(kind, children, #rule_ident::can_cast)
                    }
                }
                RuleSpecKind::SeparatedList { separator, element, allow_trailing } => {
                    let rule_ident = format_ident!("{}", spec[*element].name.to_case(Case::Pascal));
                    let separator_kind_ident = format_ident!("{}", syntax_kind_name(&spec[*separator]));
                    quote! {
                        Self::make_separated_list_syntax(kind, children, #rule_ident::can_cast, #separator_kind_ident, #allow_trailing)
                    }
                }
                RuleSpecKind::TokenVariant(variants) => {
                    let slot_size = variants.len();
                    let token_kind_idents = variants
                        .iter()
                        .map(|item| format_ident!("{}", syntax_kind_name(&spec[item.token])));

                    quote! {{
                        let mut elements = (&children).into_iter();
                        let mut slots: RawNodeSlots<#slot_size> = RawNodeSlots::default();
                        let mut current_element = elements.next();

                        if let Some(element) = &current_element {
                            if matches!(element.kind(), #(#token_kind_idents)|*) {
                                slots.mark_present();
                                current_element = elements.next();
                            }
                        }

                        if current_element.is_some() {
                            return RawSyntaxNode::new(#syntax_kind_ident.to_bogus(), children.into_iter().map(Some));
                        }
                        slots.into_node(kind, children)
                    }}
                }
                RuleSpecKind::RuleVariant(_) => {
                    return None;
                }
            };
            Some(quote! { #syntax_kind_ident => #body })
        });

    let code = quote! {
        use super::{OscDslSyntaxKind::{self, *},*,};
        use biome_rowan::{AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind};

        #[derive(Debug)]
        pub struct OscDslSyntaxFactory;

        impl SyntaxFactory for OscDslSyntaxFactory {
            type Kind = OscDslSyntaxKind;
            fn make_syntax(
                kind: Self::Kind,
                children: ParsedChildren<Self::Kind>,
            ) -> RawSyntaxNode<Self::Kind> {
                match kind {
                    #(#make_syntax_arms,)*
                    _ => unreachable!("Is {:?} a token?", kind),
                }
            }
        }
    };

    let code = cmd!(sh, "rustfmt --emit stdout")
        .stdin(code.to_string())
        .read()
        .expect("Failed to format the code");

    sh.write_file(gen_dir.join("factory.rs"), code)
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
