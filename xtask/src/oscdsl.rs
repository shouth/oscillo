use crate::{grammar::{Grammar, Terminal}, syntax_name::{SyntaxKindName, SyntaxMemberName, SyntaxNodeName}};
use convert_case::{Case, Casing};
use ungrammar::Grammar as Ungrammar;

const PUNCTS: &[(&str, &str)] = &[
    (".", "dot"),
    ("..", "dot_dot"),
    (",", "comma"),
    (":", "colon"),
    ("::", "colon_colon"),
    ("=", "assign"),
    ("@", "at"),
    ("->", "arrow"),
    ("(", "left_paren"),
    (")", "right_paren"),
    ("[", "left_bracket"),
    ("]", "right_bracket"),
    ("?", "question"),
    ("!", "exclamation"),
    ("=>", "fat_arrow"),
    ("==", "equal"),
    ("!=", "not_equal"),
    ("<", "less"),
    ("<=", "less_equal"),
    (">", "greater"),
    (">=", "greater_equal"),
    ("+", "plus"),
    ("-", "minus"),
    ("*", "star"),
    ("/", "slash"),
    ("%", "percent"),
];

const KEYWORDS: &[&str] = &[
    "A",
    "action",
    "actor",
    "and",
    "as",
    "bool",
    "call",
    "cd",
    "cover",
    "def",
    "default",
    "do",
    "elapsed",
    "emit",
    "enum",
    "event",
    "every",
    "export",
    "expression",
    "extend",
    "external",
    "factor",
    "fall",
    "false",
    "float",
    "global",
    "hard",
    "if",
    "import",
    "in",
    "inf",
    "inherits",
    "int",
    "is",
    "it",
    "K",
    "keep",
    "kg",
    "list",
    "m",
    "modifier",
    "mol",
    "namespace",
    "nan",
    "not",
    "null",
    "of",
    "offset",
    "on",
    "one_of",
    "only",
    "or",
    "parallel",
    "rad",
    "range",
    "record",
    "remove_default",
    "rise",
    "s",
    "sample",
    "scenario",
    "serial",
    "SI",
    "string",
    "struct",
    "true",
    "type",
    "uint",
    "undefined",
    "unit",
    "until",
    "use",
    "var",
    "wait",
    "with",
];

const LITERALS: &[&str] = &["integer_literal", "float_literal", "string_literal"];

const TOKENS: &[&str] = &["newline", "indent", "dedent", "identifier", "error"];

const TRIVIALS: &[&str] = &["whitespace", "comment", "trivial_newline"];

pub struct Token {
    name: String,
    terminal: String,
    kind: TokenKind,
}

impl Terminal for Token {
    fn terminal(&self) -> &str {
        &self.terminal
    }
}

impl SyntaxKindName for Token {
    fn syntax_kind_name(&self) -> String {
        match &self.kind {
            TokenKind::Keyword => format!("{}_KW", self.name.to_case(Case::UpperSnake)),
            _ => self.name.to_case(Case::UpperSnake),
        }
    }
}

impl SyntaxNodeName for Token {
    fn syntax_node_name(&self) -> String {
        format!("{}Token", self.name.to_case(Case::UpperCamel))
    }
}

impl SyntaxMemberName for Token {
    fn syntax_member_name(&self) -> String {
        format!("{}_token", self.name.to_case(Case::Snake))
    }
}

pub enum TokenKind {
    Punctuation,
    Keyword,
    Literal,
    Token,
    Trivial,
}

pub fn grammar() -> Grammar<Token> {
    let ungrammar = include_str!("oscdsl.ungram");
    let ungrammar = ungrammar.parse::<Ungrammar>().unwrap();
    let tokens = std::iter::empty()
        .chain(PUNCTS.iter().map(|x| (x.1, x.0, TokenKind::Punctuation)))
        .chain(KEYWORDS.iter().map(|&x| (x, x, TokenKind::Keyword)))
        .chain(LITERALS.iter().map(|&x| (x, x, TokenKind::Literal)))
        .chain(TOKENS.iter().map(|&x| (x, x, TokenKind::Token)))
        .chain(TRIVIALS.iter().map(|&x| (x, x, TokenKind::Trivial)))
        .map(|(name, terminal, kind)| Token {
            name: name.to_string(),
            terminal: terminal.to_string(),
            kind,
        })
        .collect();
    Grammar::try_new(&ungrammar, tokens).unwrap()
}
