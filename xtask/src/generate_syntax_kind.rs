use quote::{format_ident, quote};

use crate::{grammar::{Grammar, RuleKind}, syntax_name::SyntaxKindName};

pub trait StaticToken {
    fn static_token(&self) -> Option<&str>;
}

pub fn generate_syntax_kind<T>(grammar: &Grammar<T>) -> String
where
    T: StaticToken + SyntaxKindName,
{
    let token_kind_idents = grammar
        .tokens()
        .map(|index| format_ident!("{}", grammar[index].syntax_kind_name()));

    let rule_kind_idents = grammar
        .rules()
        .filter(|index| !matches!(grammar[*index].kind, RuleKind::Choice(_)))
        .map(|index| format_ident!("{}", grammar[index].syntax_kind_name()));

    let static_token_arms = grammar
        .tokens()
        .filter_map(|index| {
            grammar[index].static_token()
                .map(|token| {
                    let kind = format_ident!("{}", grammar[index].syntax_kind_name());
                    quote! {
                        #kind => Some(#token),
                    }
                })
        });

    let code = quote! {
        #![allow(bad_style, missing_docs, unreachable_pub)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(u16)]
        pub enum OscSyntaxKind {
            EOF,
            #(#token_kind_idents,)*
            #(#rule_kind_idents,)*

            #[doc(hidden)]
            __LAST,
        }

        use OscSyntaxKind::*;
        impl OscSyntaxKind {
            pub fn static_token(&self) -> Option<&'static str> {
                match self {
                    #(#static_token_arms)*
                    _ => None,
                }
            }
        }
    };

    code.to_string()
}
