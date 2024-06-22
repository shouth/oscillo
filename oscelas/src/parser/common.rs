use crate::syntax::OscDslSyntaxKind::*;

use crate::parser::Parser;
use crate::parser::decl::parse_type_declarator;
use crate::parser::expr::{parse_expr, parse_trailing_expr};

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

pub fn parse_argument_list_specification(p: &mut Parser) {
    let checkpoint = p.open();
    let mut flag = true;
    while flag {
        let argument_checkpoint = p.open();
        parse_qualified_identifier(p);
        p.expect(COLON);
        parse_type_declarator(p);
        if p.eat(EQUAL) {
            parse_expr(p);
        }
        flag = p.eat(COMMA);
        p.close(argument_checkpoint, ARGUMENT_SPECIFICATION);
    }
    p.close(checkpoint, ARGUMENT_LIST_SPECIFICATION);
}

pub fn parse_argument_list(p: &mut Parser) {
    let checkpoint = p.open();
    let mut flag = true;
    while flag {
        let argument_checkpoint = p.open();
        if p.check_any(&[IDENTIFIER, NULL_KW, COLON_COLON]) {
            let expr_checkpoint = p.open();
            parse_qualified_identifier(p);

            if p.eat(COLON) {
                parse_expr(p);
                flag = p.eat(COMMA);
                p.close(argument_checkpoint, NAMED_ARGUMENT);
            } else {
                parse_trailing_expr(p, expr_checkpoint);
                flag = p.eat(COMMA);
                p.close(argument_checkpoint, POSITIONAL_ARGUMENT);
            }
        } else {
            parse_expr(p);
            flag = p.eat(COMMA);
            p.close(argument_checkpoint, POSITIONAL_ARGUMENT);
        }
    }
    p.close(checkpoint, ARGUMENT_LIST);
}
