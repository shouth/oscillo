use crate::syntax::OscSyntaxKind::*;

use crate::parser::common::{
    parse_arguments, parse_argument_spcifications, parse_qualified_identifier,
};
use crate::parser::decl::parse_type_declarator;
use crate::parser::expr::parse_expr;
use crate::parser::osc_file::parse_structured_identifier;
use crate::parser::Parser;

pub fn parse_method_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(DEF_KW);
    parse_qualified_identifier(p);
    if p.check(LEFT_PAREN) {
        parse_argument_spcifications(p);
    }

    if p.check(ARROW) {
        parse_method_return_type(p);
    }

    parse_method_implementation(p);
    p.close(checkpoint, METHOD_DECLARATION);
}

pub fn parse_method_return_type(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ARROW);
    parse_type_declarator(p);
    p.close(checkpoint, METHOD_RETURN_TYPE);
}

pub fn parse_method_implementation(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(IS_KW);
    p.eat(ONLY_KW); // method qualifier (optional)

    let body_checkpoint = p.open();
    if p.eat(EXPRESSION_KW) {
        parse_expr(p);
        p.close(body_checkpoint, METHOD_EXPRESSION_BODY);
    } else if p.eat(UNDEFINED_KW) {
        // undefined method body
    } else if p.eat(EXTERNAL_KW) {
        parse_structured_identifier(p);
        if p.check(LEFT_PAREN) {
            parse_arguments(p);
        }
        p.close(body_checkpoint, METHOD_EXTERNAL_BODY);
    }
    p.close(checkpoint, METHOD_IMPLEMENTATION);
}
