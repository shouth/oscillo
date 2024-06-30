mod behavior;
mod event;
mod field;
mod method;

pub use behavior::*;
pub use event::*;
pub use field::*;
pub use method::*;

use crate::parser::decl::parse_modifier_declaration;
use crate::parser::Parser;
use crate::parser::common::parse_arguments;
use crate::parser::expr::parse_expr;
use crate::syntax::{OscSyntaxKind::*, OscSyntaxKindSet};

pub fn first_structured_type_member() -> OscSyntaxKindSet {
    ON_KW | DO_KW | EVENT_KW | KEEP_KW | REMOVE_DEFAULT_KW | DEF_KW | COVER_KW | RECORD_KW | MODIFIER_KW | first_field_declaration()
}

pub fn parse_structured_type_member_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(DEDENT) && !p.eof() {
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
        } else if p.check(first_field_declaration()) {
            parse_field_declaration(p);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(checkpoint, STRUCTURED_TYPE_MEMBER_LIST);
}

pub fn parse_constraint_declaration(p: &mut Parser) {
    if p.check(KEEP_KW) {
        parse_keep_constraint_declaration(p);
    } else if p.check(REMOVE_DEFAULT_KW) {
        parse_remove_default_declaration(p);
    } else {
        p.unexpected();
    }
}

pub fn parse_keep_constraint_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(KEEP_KW);
    let left = p.left(LEFT_PAREN);
    p.eat(DEFAULT_KW | HARD_KW); // constant qualifier (optional)
    parse_expr(p, RIGHT_PAREN | NEWLINE | DEDENT | first_structured_type_member());
    p.right(left, RIGHT_PAREN);
    p.expect(NEWLINE);
    p.close(checkpoint, KEEP_CONSTRAINT_DECLARATION);
}

pub fn parse_remove_default_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(REMOVE_DEFAULT_KW);
    let left = p.left(LEFT_PAREN);
    parse_expr(p, RIGHT_PAREN | NEWLINE | DEDENT | first_structured_type_member());
    p.right(left, RIGHT_PAREN);
    p.expect(NEWLINE);
    p.close(checkpoint, REMOVE_DEFAULT_DECLARATION);
}

pub fn parse_coverage_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(COVER_KW | RECORD_KW);
    if p.check(LEFT_PAREN) {
        parse_arguments(p);
    }
    p.expect(NEWLINE);
    p.close(checkpoint, COVERAGE_DECLARATION);
}

pub fn parse_modifier_application(p: &mut Parser) {
    let checkpoint = p.open();
    parse_expr(p, NEWLINE | DEDENT | first_structured_type_member());
    p.expect(NEWLINE);
    p.close(checkpoint, MODIFIER_APPLICATION);
}
