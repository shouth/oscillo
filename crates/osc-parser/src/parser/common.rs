use crate::parser::Parser;
use crate::parser::decl::parse_type_declarator;
use crate::parser::expr::{first_expr, parse_expr, parse_remaining_expr};
use crate::syntax::{OscSyntaxKind::*, OscSyntaxKindSet};

pub fn first_qualified_identifier() -> OscSyntaxKindSet {
    NULL_KW | IDENTIFIER | COLON_COLON
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
        p.unexpected();
    }
}

pub fn parse_argument_spcifications(p: &mut Parser) {
    let checkpoint = p.open();
    let left = p.left(LEFT_PAREN);

    let list_checkpoint = p.open();
    while !p.check(RIGHT_PAREN) || !p.eof() {
        if p.check(first_qualified_identifier()) {
            let element_checkpoint = p.open();
            parse_qualified_identifier(p);
            p.expect(COLON);
            parse_type_declarator(p);

            let init_checkpoint = p.open();
            if p.eat(EQUAL) {
                parse_expr(p, COMMA | RIGHT_PAREN);
                p.close(init_checkpoint, ARGUMENT_INITIALIZER_CLAUSE);
            }

            if !p.check(RIGHT_PAREN) {
                p.expect(COMMA);
            }
            p.close(element_checkpoint, ARGUMENT_SPECIFICATION);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(list_checkpoint, ARGUMENT_SPECIFICATION_LIST);

    p.right(left, RIGHT_PAREN);
    p.close(checkpoint, ARGUMENT_SPECIFICATIONS);
}

pub fn parse_arguments(p: &mut Parser) {
    let checkpoint = p.open();
    let left = p.left(LEFT_PAREN);

    while !p.check(RIGHT_PAREN) && !p.eof() {
        let argument_checkpoint = p.open();
        if p.check(first_qualified_identifier()) {
            let expr_checkpoint = p.open();
            parse_qualified_identifier(p);

            if p.eat(COLON) {
                parse_expr(p, COMMA | RIGHT_PAREN);
                if !p.check(RIGHT_PAREN) {
                    p.expect(COMMA);
                }
                p.close(argument_checkpoint, NAMED_ARGUMENT);
            } else {
                parse_remaining_expr(p, expr_checkpoint, COMMA | RIGHT_PAREN);
                if !p.check(RIGHT_PAREN) {
                    p.expect(COMMA);
                }
                p.close(argument_checkpoint, POSITIONAL_ARGUMENT);
            }
        } else if p.check(first_expr()) {
            parse_expr(p, COMMA | RIGHT_PAREN);
            if !p.check(RIGHT_PAREN) {
                p.expect(COMMA);
            }
            p.close(argument_checkpoint, POSITIONAL_ARGUMENT);
        } else {
            p.unexpected();
            p.error();
        }
    }

    p.right(left, RIGHT_PAREN);
    p.close(checkpoint, ARGUMENT_LIST);
}
