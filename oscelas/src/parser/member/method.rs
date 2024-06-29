use crate::parser::common::{
    parse_arguments, parse_argument_spcifications, parse_qualified_identifier,
};
use crate::parser::decl::parse_type_declarator;
use crate::parser::expr::parse_expr;
use crate::parser::member::first_structured_type_member;
use crate::parser::osc_file::parse_structured_identifier;
use crate::parser::Parser;
use crate::syntax::OscSyntaxKind::*;

pub fn parse_method_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(DEF_KW);
    parse_qualified_identifier(p);
    if p.check(LEFT_PAREN) {
        parse_argument_spcifications(p);
    }

    if p.check(ARROW) {
        let checkpoint = p.open();
        p.expect(ARROW);
        parse_type_declarator(p);
        p.close(checkpoint, METHOD_RETURN_TYPE);
    }

    let impl_checkpoint = p.open();
    p.expect(IS_KW);
    p.eat(ONLY_KW); // method qualifier (optional)

    let body_checkpoint = p.open();
    if p.eat(EXPRESSION_KW) {
        parse_expr(p, NEWLINE | DEDENT | first_structured_type_member());
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
    p.close(impl_checkpoint, METHOD_IMPLEMENTATION);

    p.expect(NEWLINE);
    p.close(checkpoint, METHOD_DECLARATION);
}
