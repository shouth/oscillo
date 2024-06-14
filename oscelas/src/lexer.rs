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
    fn test() {
        let source = r#"
scenario vehicle.two_phases:
    do serial (duration : [10s..30s]):
        phase1: actor.drive() with:
            speed(speed: 0kph, at: start)
            speed(speed: 10kph, at: end)

        phase2: actor.drive() with:
            speed(speed: [10kph..15kph])
"#;

        let mut lexer = LexicalAnalyzer::new(source);
        let mut offset = 0;
        loop {
            let token = lexer.next_token();
            let fragment = &source[offset..][..token.length];
            offset += token.length;
            println!("{:?} {:?}", token.kind, fragment);
            if token.kind == EOF {
                break;
            }
        }
    }
}
