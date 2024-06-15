use quote::{format_ident, quote};

use crate::{grammar::{Grammar, RuleKind, Terminal}, syntax_name::SyntaxKindName};

pub fn generate_syntax_kind<T>(grammar: &Grammar<T>) -> String
where
    T: Terminal + SyntaxKindName,
{
    let token_kind_idents = grammar
        .tokens()
        .map(|index| format_ident!("{}", grammar[index].syntax_kind_name()));

    let rule_kind_idents = grammar
        .rules()
        .filter(|index| !matches!(grammar[*index].kind, RuleKind::Choice(_)))
        .map(|index| format_ident!("{}", grammar[index].syntax_kind_name()));

    let code = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum OscDslSyntaxKind {
            EOF,
            #(#token_kind_idents,)*
            #(#rule_kind_idents,)*

            #[doc(hidden)]
            __LAST,
        }
    };

    code.to_string()
}
