use quote::{format_ident, quote};

use crate::{
    grammar::{Grammar, NodeIndex, RuleKind},
    syntax_name::{SyntaxKindName, SyntaxMemberName, SyntaxNodeName},
};

pub fn generate_syntax_node<T>(grammar: &Grammar<T>) -> String
where
    T: SyntaxKindName + SyntaxNodeName + SyntaxMemberName,
{
    let token_nodes = grammar.tokens().map(|token| {
        let token = &grammar[token];
        let node_name_ident = format_ident!("{}", token.syntax_node_name());
        let node_kind_ident = format_ident!("{}", token.syntax_kind_name());
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #node_name_ident<'a>(OscDslNode<'a>);

            impl<'a> TypedNode for #node_name_ident<'a> {
                type Value = OscDslSyntaxKind;
                type Node = OscDslNode<'a>;

                fn can_cast(value: Self::Value) -> bool {
                    value == #node_kind_ident
                }
                fn cast(node: Self::Node) -> Option<Self> {
                    Self::can_cast(*node.value()).then(|| Self(node))
                }
                fn syntax(&self) -> &Self::Node {
                    &self.0
                }
            }
        }
    });

    let rule_nodes = grammar.rules()
        .map(|rule| {
            let rule = &grammar[rule];
            let node_name_ident = format_ident!("{}", rule.syntax_node_name());
            let node_kind_ident = format_ident!("{}", rule.syntax_kind_name());
            match &rule.kind {
                RuleKind::Sequence(items) => {
                    let members = items.iter().enumerate().map(|(i, item)| {
                        let member_name_ident = format_ident!("{}", (grammar, item).syntax_member_name());
                        let member_node_ident = format_ident!("{}", (grammar, item).syntax_node_name());
                        let count = items.iter().take(i).filter(|other| other.node == item.node).count();

                        quote! {
                            pub fn #member_name_ident(&self) -> Option<#member_node_ident> {
                                support::child(&self.0, #count)
                            }
                        }
                    });

                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #node_name_ident<'a>(OscDslNode<'a>);

                        impl #node_name_ident<'_> {
                            #(#members)*
                        }

                        impl<'a> TypedNode for #node_name_ident<'a> {
                            type Value = OscDslSyntaxKind;
                            type Node = OscDslNode<'a>;

                            fn can_cast(value: Self::Value) -> bool {
                                value == #node_kind_ident
                            }
                            fn cast(node: Self::Node) -> Option<Self> {
                                Self::can_cast(*node.value()).then(|| Self(node))
                            }
                            fn syntax(&self) -> &Self::Node {
                                &self.0
                            }
                        }
                    }
                }
                RuleKind::Choice(nodes) => {
                    let variants = nodes
                        .iter()
                        .map(|node| {
                            let node_name_ident = format_ident!("{}", (grammar, node).syntax_node_name());
                            quote! {
                                #node_name_ident(#node_name_ident<'a>)
                            }
                        });

                    let members = nodes.iter().map(|node| {
                        let member_name_ident = format_ident!("as_{}", (grammar, node).syntax_member_name());
                        let member_node_ident = format_ident!("{}", (grammar, node).syntax_node_name());

                        quote! {
                            pub fn #member_name_ident(&self) -> Option<#member_node_ident> {
                                match self {
                                    Self::#member_node_ident(node) => Some(node.clone()),
                                    _ => None
                                }
                            }
                        }
                    });

                    fn generate_syntax_kind_name_groups<T>(grammar: &Grammar<T>, nodes: &[NodeIndex]) -> Vec<Vec<String>>
                    where
                        T: SyntaxKindName,
                    {
                        let mut result = Vec::new();
                        for node in nodes {
                            match node {
                                NodeIndex::Rule(rule) => match &grammar[*rule].kind {
                                    RuleKind::Choice(nodes) => {
                                        let group = generate_syntax_kind_name_groups(grammar, nodes)
                                            .iter()
                                            .flatten()
                                            .map(|name| name.to_owned())
                                            .collect();
                                        result.push(group);
                                    }
                                    _ => {
                                        let name = grammar[*rule].syntax_kind_name();
                                        result.push(vec![name]);
                                    }
                                }
                                NodeIndex::Token(token) => {
                                    let name = grammar[*token].syntax_kind_name();
                                    result.push(vec![name]);
                                }
                            }
                        }
                        result
                    }
                    let syntax_kind_name_groups = generate_syntax_kind_name_groups(grammar, nodes);

                    let can_cast_idents = syntax_kind_name_groups.iter().flatten()
                        .map(|name| format_ident!("{name}"));

                    let cast_arms = syntax_kind_name_groups.iter().zip(nodes.iter()).map(|(group, node)| {
                        let node_name_ident = format_ident!("{}", (grammar, node).syntax_node_name());
                        let group_idents = group.iter().map(|name| format_ident!("{}", name));

                        quote! {
                            #(#group_idents)|* => #node_name_ident::cast(node.clone()).map(Self::#node_name_ident)
                        }
                    });

                    let syntax_arms = nodes.iter().map(|node| {
                        let node_name_ident = format_ident!("{}", (grammar, node).syntax_node_name());

                        quote! {
                            Self::#node_name_ident(node) => node.syntax()
                        }
                    });

                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub enum #node_name_ident<'a> {
                            #(#variants,)*
                        }

                        impl #node_name_ident<'_> {
                            #(#members)*
                        }

                        impl<'a> TypedNode for #node_name_ident<'a> {
                            type Value = OscDslSyntaxKind;
                            type Node = OscDslNode<'a>;

                            fn can_cast(value: Self::Value) -> bool {
                                matches!(value, #(#can_cast_idents)|*)
                            }
                            fn cast(node: Self::Node) -> Option<Self> {
                                match *node.value() {
                                    #(#cast_arms,)*
                                    _ => None
                                }
                            }
                            fn syntax(&self) -> &Self::Node {
                                match self {
                                    #(#syntax_arms,)*
                                }
                            }
                        }
                    }
                }
                RuleKind::Repeat(node, delimiter) => {
                    let member_node_ident = format_ident!("{}", (grammar, node).syntax_node_name());
                    let member_name_ident = format_ident!("{}", (grammar, node).syntax_member_name());

                    let delimiter_member = match delimiter {
                        Some(node) => {
                            let member_node_ident = format_ident!("{}", (grammar, node).syntax_node_name());
                            let member_name_ident = format_ident!("{}", (grammar, node).syntax_member_name());

                            quote! {
                                pub fn #member_name_ident(&self) -> impl Iterator<Item = #member_node_ident<'a>> + 'a {
                                    support::children(&self.0)
                                }
                            }
                        }
                        None => quote! {},
                    };

                    quote! {
                        #[derive(Debug, Clone, PartialEq, Eq)]
                        pub struct #node_name_ident<'a>(OscDslNode<'a>);

                        impl<'a> #node_name_ident<'a> {
                            #delimiter_member

                            pub fn #member_name_ident(&self) -> impl Iterator<Item = #member_node_ident<'a>> + 'a {
                                support::children(&self.0)
                            }
                        }

                        impl<'a> TypedNode for #node_name_ident<'a> {
                            type Value = OscDslSyntaxKind;
                            type Node = OscDslNode<'a>;

                            fn can_cast(value: Self::Value) -> bool {
                                value == #node_kind_ident
                            }
                            fn cast(node: Self::Node) -> Option<Self> {
                                Self::can_cast(*node.value()).then(|| Self(node))
                            }
                            fn syntax(&self) -> &Self::Node {
                                &self.0
                            }
                        }
                    }
                }
            }
        });

    let code = quote! {
        use syntree::Node;
        use crate::syntax::{support, TypedNode};
        use super::OscDslSyntaxKind::{self, *};

        type OscDslNode<'a> = Node<'a, OscDslSyntaxKind, u32, usize>;

        #(#token_nodes)*
        #(#rule_nodes)*
    };

    code.to_string()
}
