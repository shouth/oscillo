use convert_case::{Case, Casing};

use crate::syntax::{Grammar, NodeIndex, Rule, SequenceItem};

pub trait SyntaxKindName {
    fn syntax_kind_name(&self) -> String;
}

impl SyntaxKindName for Rule {
    fn syntax_kind_name(&self) -> String {
        self.name.to_case(Case::UpperSnake)
    }
}

impl<Token> SyntaxKindName for (&Grammar<Token>, &NodeIndex)
where
    Token: SyntaxKindName
{
    fn syntax_kind_name(&self) -> String {
        let (grammar, index) = self;
        match index {
            NodeIndex::Rule(index) => grammar[*index].syntax_kind_name(),
            NodeIndex::Token(index) => grammar[*index].syntax_kind_name(),
        }
    }
}

impl<Token> SyntaxKindName for (&Grammar<Token>, &SequenceItem)
where
    Token: SyntaxKindName
{
    fn syntax_kind_name(&self) -> String {
        let (grammar, item) = self;
        (*grammar, &item.node).syntax_kind_name()
    }
}

pub trait SyntaxNodeName {
    fn syntax_node_name(&self) -> String;
}

impl SyntaxNodeName for Rule {
    fn syntax_node_name(&self) -> String {
        self.name.to_case(Case::UpperCamel)
    }
}

impl<Token> SyntaxNodeName for (&Grammar<Token>, &NodeIndex)
where
    Token: SyntaxNodeName
{
    fn syntax_node_name(&self) -> String {
        let (grammar, index) = self;
        match index {
            NodeIndex::Rule(index) => grammar[*index].syntax_node_name(),
            NodeIndex::Token(index) => grammar[*index].syntax_node_name(),
        }
    }
}

impl<Token> SyntaxNodeName for (&Grammar<Token>, &SequenceItem)
where
    Token: SyntaxNodeName
{
    fn syntax_node_name(&self) -> String {
        let (grammar, item) = self;
        (*grammar, &item.node).syntax_node_name()
    }
}

pub trait SyntaxMemberName {
    fn syntax_member_name(&self) -> String;
}

impl SyntaxMemberName for Rule {
    fn syntax_member_name(&self) -> String {
        self.name.to_case(Case::Snake)
    }
}

impl<Token> SyntaxMemberName for (&Grammar<Token>, &NodeIndex)
where
    Token: SyntaxMemberName
{
    fn syntax_member_name(&self) -> String {
        let (grammar, index) = self;
        match index {
            NodeIndex::Rule(index) => grammar[*index].syntax_member_name(),
            NodeIndex::Token(index) => grammar[*index].syntax_member_name(),
        }
    }
}

impl<Token> SyntaxMemberName for (&Grammar<Token>, &SequenceItem)
where
    Token: SyntaxMemberName
{
    fn syntax_member_name(&self) -> String {
        let (grammar, item) = self;
        match &item.label {
            Some(label) => label.to_case(Case::Snake),
            None => (*grammar, &item.node).syntax_member_name(),
        }
    }
}
