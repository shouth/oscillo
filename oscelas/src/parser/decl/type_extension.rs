use crate::parser::decl::{
    parse_enum_member_decls, parse_modifier_declaration, parse_qualified_behavior_name,
};
use crate::parser::member::{
    check_field_declaration, parse_behavior_specification, parse_constraint_declaration,
    parse_coverage_declaration, parse_event_declaration, parse_field_declaration,
    parse_method_declaration,
};
use crate::parser::{error_unexpected, Parser};
use crate::syntax::OscSyntaxKind::*;

pub fn parse_type_extenstion(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EXTEND_KW);
    parse_qualified_behavior_name(p);
    parse_type_extension_body(p);
    p.close(checkpoint, TYPE_EXTENSION);
}

fn parse_type_extension_body(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(COLON);
    if p.check(LEFT_BRACKET) {
        parse_enum_member_decls(p);
        p.expect(NEWLINE);
        p.close(checkpoint, ENUM_TYPE_EXTENSION_BODY);
    } else if p.eat(NEWLINE) {
        p.expect(INDENT);
        parse_extension_member_decl_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, STRUCTURED_TYPE_EXTENSION_BODY);
    } else {
        error_unexpected(p);
    }
}

fn parse_extension_member_decl_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(DEDENT | EOF) {
        if p.check(ON_KW | DO_KW) {
            parse_behavior_specification(p);
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
    p.close(checkpoint, EXTENSION_MEMBER_LIST);
}
