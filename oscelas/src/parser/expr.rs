use crate::syntax::OscDslSyntaxKind::*;

use crate::parser::{Checkpoint, Parser};
use crate::parser::common::{parse_argument_list, parse_qualified_identifier};
use crate::parser::decl::parse_type_declarator;

pub fn parse_expr(p: &mut Parser) {
    parse_expr_with_power(p, 0);
}

pub fn parse_trailing_expr(p: &mut Parser, checkpoint: Checkpoint) {
    parse_trailing_expr_with_power(p, checkpoint, 0);
}

fn parse_expr_with_power(p: &mut Parser, power: u8) {
    let checkpoint = p.open();

    if p.eat_any(&[NOT_KW, MINUS]) {
        parse_expr_with_power(p, 100);
        p.close(checkpoint.clone(), UNARY_EXP);
    } else if p.eat(LEFT_PAREN) {
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint.clone(), PARENTHESIZED_EXP);
    } else if p.eat(LEFT_BRACKET) {
        let list_checkpoint = p.open();
        let element_checkpoint = p.open();
        parse_expr(p);
        if p.eat(COMMA) {
            p.close(element_checkpoint, EXPRESSION_LIST_ELEMENT);
            let mut flag = true;
            while flag {
                let element_checkpoint = p.open();
                parse_expr(p);
                flag = p.eat(COMMA);
                p.close(element_checkpoint, EXPRESSION_LIST_ELEMENT);
            }
            p.close(list_checkpoint, EXPRESSION_LIST);
            p.expect(RIGHT_BRACKET);
            p.close(checkpoint.clone(), LIST_CONSTRUCTOR);
        } else if p.eat(DOT_DOT) {
            parse_expr(p);
            p.expect(RIGHT_BRACKET);
            p.close(checkpoint.clone(), BRACKETS_RANGE_CONSTRUCTOR);
        } else {
            p.unexpected();
        }
    } else if p.eat(RANGE_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p);
        p.expect(COMMA);
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint.clone(), PARENTHESES_RANGE_CONSTRUCTOR);
    } else if p.check_any(&[IDENTIFIER, NULL_KW, COLON_COLON]){
        parse_qualified_identifier(p);
        if p.eat(EXCLAMATION) {
            parse_qualified_identifier(p);
            p.close(checkpoint.clone(), ENUM_VALUE_REFERENCE);
        } else {
            // qualified identifier
        }
    } else if p.eat(IT_KW) {
        // `it` expression
    } else if p.eat_any(&[INTEGER_LITERAL, FLOAT_LITERAL]) {
        if !p.has_leading_trivia() && p.check_any(&[IDENTIFIER, NULL_KW, COLON_COLON]) {
            parse_qualified_identifier(p);
            p.close(checkpoint.clone(), PHYSICAL_LITERAL);
        } else {
            // number literals
        }
    } else if p.eat_any(&[TRUE_KW, FALSE_KW]){
        // boolean literals
    } else if p.eat(STRING_LITERAL) {
        // string literals
    }

    parse_trailing_expr_with_power(p, checkpoint, power);
}

fn parse_trailing_expr_with_power(p: &mut Parser, checkpoint: Checkpoint, power: u8) {
    loop {
        // As of version 2.1.0, postfix operators have higher precedence than any other operators.
        if p.eat(DOT) {
            if p.eat(AS_KW) {
                p.expect(LEFT_PAREN);
                parse_type_declarator(p);
                p.expect(RIGHT_PAREN);
                p.close(checkpoint.clone(), CAST_EXP);
            } else if p.eat(IS_KW) {
                p.expect(LEFT_PAREN);
                parse_type_declarator(p);
                p.expect(RIGHT_PAREN);
                p.close(checkpoint.clone(), TYPE_TEST_EXP);
            } else if p.check_any(&[IDENTIFIER, NULL_KW, COLON_COLON]) {
                parse_qualified_identifier(p);
                p.close(checkpoint.clone(), MEMBER_REFERENCE);
            } else {
                p.unexpected();
            }
        } else if p.eat(LEFT_BRACKET) {
            parse_expr(p);
            p.expect(RIGHT_BRACKET);
            p.close(checkpoint.clone(), ELEMENT_ACCESS);
        } else if p.eat(LEFT_PAREN) {
            parse_argument_list(p);
            p.expect(RIGHT_PAREN);
            p.close(checkpoint.clone(), FUNCTION_APPLICATION);
        } else if power < 11 && p.eat(QUESTION) {
            parse_expr(p);
            p.expect(COLON);
            parse_expr_with_power(p, 10);
            p.close(checkpoint.clone(), TERNARY_EXP);
        } else if power < 20 && p.eat(FAT_ARROW) {
            parse_expr_with_power(p, 21);
            p.close(checkpoint.clone(), LOGICAL_EXP);
        } else if power < 30 && p.eat(OR_KW) {
            parse_expr_with_power(p, 31);
            p.close(checkpoint.clone(), LOGICAL_EXP);
        } else if power < 40 && p.eat(AND_KW) {
            parse_expr_with_power(p, 41);
            p.close(checkpoint.clone(), LOGICAL_EXP);
        } else if power < 50 && p.eat_any(&[EQUAL, NOT_EQUAL, LESS, LESS_EQUAL, GREATER, GREATER_EQUAL, IN_KW]) {
            parse_expr_with_power(p, 51);
            p.close(checkpoint.clone(), BINARY_EXP);
        } else if power < 60 && p.eat_any(&[PLUS, MINUS]) {
            parse_expr_with_power(p, 61);
            p.close(checkpoint.clone(), BINARY_EXP);
        } else if power < 70 && p.eat_any(&[STAR, SLASH, PERCENT]) {
            parse_expr_with_power(p, 71);
            p.close(checkpoint.clone(), BINARY_EXP);
        } else {
            return;
        }
    }
}
