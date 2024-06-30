use crate::diagnostic::SyntaxDiagnostic;
use crate::parser::common::{parse_arguments, parse_qualified_identifier};
use crate::parser::decl::parse_type_declarator;
use crate::parser::{Checkpoint, Parser};
use crate::syntax::{OscSyntaxKind::*, OscSyntaxKindSet};

pub fn first_expr() -> OscSyntaxKindSet {
    NOT_KW | MINUS | LEFT_PAREN | LEFT_BRACKET | RANGE_KW | IDENTIFIER | NULL_KW | COLON_COLON
        | IT_KW | INTEGER_LITERAL | FLOAT_LITERAL | TRUE_KW | FALSE_KW | STRING_LITERAL
}

pub fn parse_expr(p: &mut Parser, recovery: OscSyntaxKindSet) {
    parse_leading_expr(p, 0, recovery);
}

pub fn parse_remaining_expr(p: &mut Parser, checkpoint: Checkpoint, recovery: OscSyntaxKindSet) {
    parse_trailing_expr(p, checkpoint, 0, recovery);
}

fn parse_leading_expr(p: &mut Parser, power: u8, recovery: OscSyntaxKindSet) {
    let checkpoint = p.open();

    if p.eat(NOT_KW | MINUS) {
        parse_leading_expr(p, 100, recovery);
        p.close(checkpoint.clone(), UNARY_EXP);
    } else if p.check(LEFT_PAREN) {
        let left = p.left(LEFT_PAREN);
        parse_expr(p, RIGHT_PAREN | recovery);
        p.right(left, RIGHT_PAREN);
        p.close(checkpoint.clone(), PARENTHESIZED_EXP);
    } else if p.check(LEFT_BRACKET) {
        let left = p.left(LEFT_BRACKET);
        let list_checkpoint = p.open();
        if p.check(RIGHT_BRACKET) {
            // allow empty list when parsing
            p.right(left, RIGHT_BRACKET);
            p.close(list_checkpoint, EXPRESSION_LIST);
        } else {
            let mut element_checkpoint = p.open();
            parse_expr(p, DOT_DOT | RIGHT_BRACKET | COMMA | recovery);
            if p.eat(DOT_DOT) {
                parse_expr(p, RIGHT_BRACKET | recovery);
                p.right(left, RIGHT_BRACKET);
                p.close(checkpoint.clone(), BRACKETS_RANGE_CONSTRUCTOR);
            } else {
                while !p.check(RIGHT_BRACKET) && !p.eof() {
                    p.expect(COMMA);
                    p.close(element_checkpoint, EXPRESSION_LIST_ELEMENT);
                    element_checkpoint = p.open();
                    parse_expr(p, RIGHT_BRACKET | COMMA | recovery);
                }
                p.close(element_checkpoint, EXPRESSION_LIST_ELEMENT);
                p.right(left, RIGHT_BRACKET);
                p.close(list_checkpoint, EXPRESSION_LIST);
            }
        }
    } else if p.eat(RANGE_KW) {
        let left = p.left(LEFT_PAREN);
        parse_expr(p, COMMA | RIGHT_PAREN | recovery);
        p.expect(COMMA);
        parse_expr(p, RIGHT_PAREN | recovery);
        p.right(left, RIGHT_PAREN);
        p.close(checkpoint.clone(), PARENTHESES_RANGE_CONSTRUCTOR);
    } else if p.check(IDENTIFIER | NULL_KW | COLON_COLON) {
        parse_qualified_identifier(p);
        if p.eat(EXCLAMATION) {
            parse_qualified_identifier(p);
            p.close(checkpoint.clone(), ENUM_VALUE_REFERENCE);
        } else {
            // qualified identifier
        }
    } else if p.eat(IT_KW) {
        // `it` expression
    } else if p.eat(INTEGER_LITERAL | FLOAT_LITERAL) {
        if !p.has_leading_trivia() && p.check(IDENTIFIER | NULL_KW | COLON_COLON) {
            parse_qualified_identifier(p);
            p.close(checkpoint.clone(), PHYSICAL_LITERAL);
        } else {
            // number literals
        }
    } else if p.eat(TRUE_KW | FALSE_KW) {
        // boolean literals
    } else if p.eat(STRING_LITERAL) {
        // string literals
    } else {
        let range = p.current_token_range();
        let found = p.current_token_kind();
        p.diagnostic(SyntaxDiagnostic::ExpectedExpression { range, found });
        p.recover(recovery);
        return;
    }

    parse_trailing_expr(p, checkpoint, power, recovery);
}

fn parse_trailing_expr(p: &mut Parser, checkpoint: Checkpoint, power: u8, recovery: OscSyntaxKindSet) {
    loop {
        // As of version 2.1.0, postfix operators have higher precedence than any other operators.
        if p.eat(DOT) {
            if p.eat(AS_KW) {
                let left = p.left(LEFT_PAREN);
                parse_type_declarator(p);
                p.right(left, RIGHT_PAREN);
                p.close(checkpoint.clone(), CAST_EXP);
            } else if p.eat(IS_KW) {
                let left = p.left(LEFT_PAREN);
                parse_type_declarator(p);
                p.right(left, RIGHT_PAREN);
                p.close(checkpoint.clone(), TYPE_TEST_EXP);
            } else if p.check(IDENTIFIER | NULL_KW | COLON_COLON) {
                parse_qualified_identifier(p);
                p.close(checkpoint.clone(), MEMBER_REFERENCE);
            } else {
                p.unexpected();
            }
        } else if p.check(LEFT_BRACKET) {
            let left = p.left(LEFT_BRACKET);
            parse_expr(p, RIGHT_BRACKET | recovery);
            p.right(left, RIGHT_BRACKET);
            p.close(checkpoint.clone(), ELEMENT_ACCESS);
        } else if p.check(LEFT_PAREN) {
            parse_arguments(p);
            p.close(checkpoint.clone(), FUNCTION_APPLICATION);
        } else if power < 11 && p.eat(QUESTION) {
            parse_expr(p, COLON | recovery);
            p.expect(COLON);
            parse_leading_expr(p, 10, recovery);
            p.close(checkpoint.clone(), TERNARY_EXP);
        } else if power < 20 && p.eat(FAT_ARROW) {
            parse_leading_expr(p, 21, recovery);
            p.close(checkpoint.clone(), LOGICAL_EXP);
        } else if power < 30 && p.eat(OR_KW) {
            parse_leading_expr(p, 31, recovery);
            p.close(checkpoint.clone(), LOGICAL_EXP);
        } else if power < 40 && p.eat(AND_KW) {
            parse_leading_expr(p, 41, recovery);
            p.close(checkpoint.clone(), LOGICAL_EXP);
        } else if power < 50
            && p.eat(EQUAL | NOT_EQUAL | LESS | LESS_EQUAL | GREATER | GREATER_EQUAL | IN_KW)
        {
            parse_leading_expr(p, 51, recovery);
            p.close(checkpoint.clone(), BINARY_EXP);
        } else if power < 60 && p.eat(PLUS | MINUS) {
            parse_leading_expr(p, 61, recovery);
            p.close(checkpoint.clone(), BINARY_EXP);
        } else if power < 70 && p.eat(STAR | SLASH | PERCENT) {
            parse_leading_expr(p, 71, recovery);
            p.close(checkpoint.clone(), BINARY_EXP);
        } else {
            p.recover(recovery);
            return;
        }
    }
}
