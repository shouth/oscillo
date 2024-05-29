use crate::syntax_spec::{SyntaxSpec, TokenSet};
use ungrammar::Grammar;

const PUNCTUAIONS: &[(&str, &str)] = &[
    (".", "dot"),
    ("..", "dot_dot"),
    (",", "comma"),
    (":", "colon"),
    ("::", "colon_colon"),
    ("=", "assign"),
    ("@", "at"),
    ("->", "arrow"),
    ("(", "l_paren"),
    (")", "r_paren"),
    ("[", "l_bracket"),
    ("]", "r_bracket"),
    ("?", "question"),
    ("!", "excalamation"),
    ("=>", "fat_arrow"),
    ("==", "equal"),
    ("!=", "not_equal"),
    ("<", "less"),
    ("<=", "less_eq"),
    (">", "greater"),
    (">=", "greater_eq"),
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

const TOKENS: &[&str] = &["newline", "indent", "dedent", "identifier"];

const TRIVIALS: &[&str] = &["whitespace", "comment", "nonlogical_newline"];

pub fn load_grammar() -> Result<Grammar, Box<dyn std::error::Error>> {
    let content = include_str!("oscdsl.ungram");
    content.parse::<Grammar>().map_err(Into::into)
}

pub fn load_spec() -> Result<SyntaxSpec, Box<dyn std::error::Error>> {
    let grammar = load_grammar()?;
    let token_set = TokenSet {
        punctuations: PUNCTUAIONS,
        keywords: KEYWORDS,
        literals: LITERALS,
        tokens: TOKENS,
        trivials: TRIVIALS,
    };
    (&grammar, &token_set).try_into()
}
