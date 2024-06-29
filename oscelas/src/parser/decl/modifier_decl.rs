use crate::parser::decl::parse_qualified_behavior_name;
use crate::parser::member::parse_structured_type_member_list;
use crate::parser::{error_unexpected, Parser};
use crate::syntax::OscSyntaxKind::*;

pub fn parse_modifier_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(MODIFIER_KW);
    parse_qualified_behavior_name(p);

    if p.check(OF_KW) {
        parse_modifier_of_clause(p);
    }

    if p.check(COLON) {
        parse_modifier_body(p);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        error_unexpected(p);
    }

    p.close(checkpoint, MODIFIER_DECLARATION);
}

fn parse_modifier_of_clause(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(OF_KW);
    parse_qualified_behavior_name(p);
    p.close(checkpoint, MODIFIER_OF_CLAUSE);
}

fn parse_modifier_body(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);
    parse_structured_type_member_list(p);
    p.expect(DEDENT);
    p.close(checkpoint, MODIFIER_BODY);
}
