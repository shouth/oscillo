use crate::parser::decl::parse_qualified_behavior_name;
use crate::parser::member::{
    check_field_declaration, parse_constraint_declaration, parse_coverage_declaration, parse_event_declaration, parse_field_declaration, parse_method_declaration, parse_on_directive
};
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
    parse_modifier_member_item_list(p);
    p.expect(DEDENT);
    p.close(checkpoint, MODIFIER_BODY);
}

fn parse_modifier_member_item_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(DEDENT | EOF) {
        if p.check(ON_KW) {
            parse_on_directive(p);
        } else if p.check(EVENT_KW) {
            parse_event_declaration(p);
        } else if p.check(KEEP_KW | REMOVE_DEFAULT_KW) {
            parse_constraint_declaration(p);
        } else if p.check(DEF_KW) {
            parse_method_declaration(p);
        } else if p.check(COVER_KW | RECORD_KW) {
            parse_coverage_declaration(p);
        } else if p.check(MODIFIER_KW) {
            parse_modifier_declaration(p);
        } else if check_field_declaration(p) {
            parse_field_declaration(p);
        } else {
            error_unexpected(p);
            p.error();
        }
    }
    p.close(checkpoint, MODIFIER_MEMBER_LIST);
}
