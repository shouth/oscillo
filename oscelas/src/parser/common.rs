use crate::parser::{error_unexpected, Parser};
use crate::parser::decl::parse_type_declarator;
use crate::parser::expr::{parse_expr, parse_remaining_expr};
use crate::syntax::OscSyntaxKind::*;

pub fn check_qualifed_identifier(p: &mut Parser) -> bool {
    p.check(IDENTIFIER | NULL_KW | COLON_COLON)
}

pub fn parse_qualified_identifier(p: &mut Parser) {
    let checkpoint = p.open();
    if p.eat(NULL_KW) {
        p.expect(COLON_COLON);
        p.expect(IDENTIFIER);
        p.close(checkpoint, PREFIXED_IDENTIFIER);
    } else if p.eat(IDENTIFIER) {
        if p.eat(COLON_COLON) {
            p.expect(IDENTIFIER);
            p.close(checkpoint, PREFIXED_IDENTIFIER);
        }
    } else if p.eat(COLON_COLON) {
        p.expect(IDENTIFIER);
        p.close(checkpoint, PREFIXED_IDENTIFIER);
    } else {
        error_unexpected(p);
    }
}

pub fn parse_argument_spcifications(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(LEFT_PAREN);

    let list_checkpoint = p.open();
    while !p.check(RIGHT_PAREN | EOF) {
        let element_checkpoint = p.open();
        parse_qualified_identifier(p);
        p.expect(COLON);
        parse_type_declarator(p);

        let init_checkpoint = p.open();
        if p.eat(EQUAL) {
            parse_expr(p);
            p.close(init_checkpoint, ARGUMENT_INITIALIZER_CLAUSE);
        }

        if !p.check(RIGHT_PAREN) {
            p.expect(COMMA);
        }
        p.close(element_checkpoint, ARGUMENT_SPECIFICATION);
    }
    p.close(list_checkpoint, ARGUMENT_SPECIFICATION_LIST);

    p.expect(RIGHT_PAREN);
    p.close(checkpoint, ARGUMENT_SPECIFICATIONS);
}

pub fn parse_arguments(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(LEFT_PAREN);

    while !p.check(RIGHT_PAREN | EOF) {
        let argument_checkpoint = p.open();
        if p.check(IDENTIFIER | NULL_KW | COLON_COLON) {
            let expr_checkpoint = p.open();
            parse_qualified_identifier(p);

            if p.eat(COLON) {
                parse_expr(p);
                if !p.check(RIGHT_PAREN) {
                    p.expect(COMMA);
                }
                p.close(argument_checkpoint, NAMED_ARGUMENT);
            } else {
                parse_remaining_expr(p, expr_checkpoint);
                if !p.check(RIGHT_PAREN) {
                    p.expect(COMMA);
                }
                p.close(argument_checkpoint, POSITIONAL_ARGUMENT);
            }
        } else {
            parse_expr(p);
            if !p.check(RIGHT_PAREN) {
                p.expect(COMMA);
            }
            p.close(argument_checkpoint, POSITIONAL_ARGUMENT);
        }
    }

    p.expect(RIGHT_PAREN);
    p.close(checkpoint, ARGUMENT_LIST);
}
