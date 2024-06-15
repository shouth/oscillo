mod cursor;
mod lexer;
mod lexical_analyzer;
mod lookahead;
mod raw_lexer;

use crate::syntax::OscDslSyntaxKind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct LexedToken {
    pub kind: OscDslSyntaxKind,
    pub length: usize,
}


#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use lexical_analyzer::LexicalAnalyzer;

    use crate::syntax::OscDslSyntaxKind::*;

    use super::*;

    fn tokenize(source: &str) -> Vec<&str> {
        let mut result = Vec::new();
        let mut lexer = Lexer::new(source);
        let mut offset = 0;
        loop {
            let token = lexer.next_token();
            if token.kind == EOF {
                return result;
            }
            result.push(&source[offset..][..token.length]);
            offset += token.length;
        }
    }

    #[test]
    fn test_numeric() {
        assert_eq!(tokenize("0x"), ["0", "x"]);
        assert_eq!(tokenize("0x0"), ["0x0"]);

        assert_eq!(tokenize("123."), ["123", "."]);
        assert_eq!(tokenize("123.."), ["123", ".."]);
        assert_eq!(tokenize(".123"), [".123"]);
        assert_eq!(tokenize("..123"), ["..", "123"]);

        assert_eq!(tokenize("1.23"), ["1.23"]);
        assert_eq!(tokenize("+1.23"), ["+1.23"]);
        assert_eq!(tokenize("-1.23"), ["-1.23"]);

        assert_eq!(tokenize("1.23e456"), ["1.23e456"]);
        assert_eq!(tokenize("1.23e 456"), ["1.23", "e", " ", "456"]);
        assert_eq!(tokenize("1.23e+456"), ["1.23e+456"]);
        assert_eq!(tokenize("1.23ee+456"), ["1.23", "ee", "+", "456"]);
        assert_eq!(tokenize("1.23e +456"), ["1.23", "e", " ", "+", "456"]);
        assert_eq!(tokenize("1.23e-456"), ["1.23e-456"]);
        assert_eq!(tokenize("1.23ee-456"), ["1.23", "ee", "-456"]);
        assert_eq!(tokenize("1.23e -456"), ["1.23", "e", " ", "-456"]);
    }

    #[test]
    fn test_analyzer() {
        let source = r#"
scenario vehicle.two_phases:
    do serial (duration : [10s..30s]):
        phase1: actor.drive() with:
            speed(speed: 0kph, at: start)
            speed(speed: 10kph, at: end)

        phase2: actor.drive() with:
            speed(speed: [10kph..15kph])
"#;

        let expected = [
            (TRIVIAL_NEWLINE, "\n"),
            (IDENTIFIER, "scenario"),
            (WHITESPACE, " "),
            (IDENTIFIER, "vehicle"),
            (DOT, "."),
            (IDENTIFIER, "two_phases"),
            (COLON, ":"),
            (NEWLINE, "\n"),
            (INDENT, ""),
            (WHITESPACE, "    "),
            (IDENTIFIER, "do"),
            (WHITESPACE, " "),
            (IDENTIFIER, "serial"),
            (WHITESPACE, " "),
            (LEFT_PAREN, "("),
            (IDENTIFIER, "duration"),
            (WHITESPACE, " "),
            (COLON, ":"),
            (WHITESPACE, " "),
            (LEFT_BRACKET, "["),
            (INTEGER_LITERAL, "10"),
            (IDENTIFIER, "s"),
            (DOT_DOT, ".."),
            (INTEGER_LITERAL, "30"),
            (IDENTIFIER, "s"),
            (RIGHT_BRACKET, "]"),
            (RIGHT_PAREN, ")"),
            (COLON, ":"),
            (NEWLINE, "\n"),
            (INDENT, ""),
            (WHITESPACE, "        "),
            (IDENTIFIER, "phase1"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (IDENTIFIER, "actor"),
            (DOT, "."),
            (IDENTIFIER, "drive"),
            (LEFT_PAREN, "("),
            (RIGHT_PAREN, ")"),
            (WHITESPACE, " "),
            (IDENTIFIER, "with"),
            (COLON, ":"),
            (NEWLINE, "\n"),
            (INDENT, ""),
            (WHITESPACE, "            "),
            (IDENTIFIER, "speed"),
            (LEFT_PAREN, "("),
            (IDENTIFIER, "speed"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (INTEGER_LITERAL, "0"),
            (IDENTIFIER, "kph"),
            (COMMA, ","),
            (WHITESPACE, " "),
            (IDENTIFIER, "at"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (IDENTIFIER, "start"),
            (RIGHT_PAREN, ")"),
            (NEWLINE, "\n"),
            (WHITESPACE, "            "),
            (IDENTIFIER, "speed"),
            (LEFT_PAREN, "("),
            (IDENTIFIER, "speed"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (INTEGER_LITERAL, "10"),
            (IDENTIFIER, "kph"),
            (COMMA, ","),
            (WHITESPACE, " "),
            (IDENTIFIER, "at"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (IDENTIFIER, "end"),
            (RIGHT_PAREN, ")"),
            (NEWLINE, "\n"),
            (TRIVIAL_NEWLINE, "\n"),
            (DEDENT, ""),
            (WHITESPACE, "        "),
            (IDENTIFIER, "phase2"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (IDENTIFIER, "actor"),
            (DOT, "."),
            (IDENTIFIER, "drive"),
            (LEFT_PAREN, "("),
            (RIGHT_PAREN, ")"),
            (WHITESPACE, " "),
            (IDENTIFIER, "with"),
            (COLON, ":"),
            (NEWLINE, "\n"),
            (INDENT, ""),
            (WHITESPACE, "            "),
            (IDENTIFIER, "speed"),
            (LEFT_PAREN, "("),
            (IDENTIFIER, "speed"),
            (COLON, ":"),
            (WHITESPACE, " "),
            (LEFT_BRACKET, "["),
            (INTEGER_LITERAL, "10"),
            (IDENTIFIER, "kph"),
            (DOT_DOT, ".."),
            (INTEGER_LITERAL, "15"),
            (IDENTIFIER, "kph"),
            (RIGHT_BRACKET, "]"),
            (RIGHT_PAREN, ")"),
            (NEWLINE, "\n"),
            (DEDENT, ""),
            (DEDENT, ""),
            (DEDENT, ""),
            (EOF, ""),
        ];

        let mut lexer = LexicalAnalyzer::new(source);
        let mut offset = 0;
        for expected in &expected {
            let token = lexer.next_token();
            let fragment = &source[offset..][..token.length];
            offset += token.length;

            let actual = (token.kind, fragment);
            assert_eq!(actual, *expected);
        }
    }
}
