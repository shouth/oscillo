mod behavior;
mod event;
mod field;
mod method;

pub use behavior::*;
pub use event::*;
pub use field::*;
pub use method::*;

use crate::syntax::OscSyntaxKind::*;

use crate::parser::Parser;
use crate::parser::common::{parse_arguments, parse_qualified_identifier};
use crate::parser::expr::parse_expr;

pub fn parse_field_name_list(p: &mut Parser) {
    let checkpoint = p.open();
    let mut flag = true;
    while flag {
        let element_checkpoint = p.open();
        parse_qualified_identifier(p);
        flag = p.eat(COMMA);
        p.close(element_checkpoint, FIELD_NAME_LIST_ELEMENT);
    }
    p.close(checkpoint, FIELD_NAME_LIST);
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
    p.expect(LEFT_PAREN);
    p.eat(DEFAULT_KW | HARD_KW); // constant qualifier (optional)
    parse_expr(p);
    p.expect(RIGHT_PAREN);
    p.expect(NEWLINE);
    p.close(checkpoint, KEEP_CONSTRAINT_DECLARATION);
}

pub fn parse_remove_default_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(REMOVE_DEFAULT_KW);
    p.expect(LEFT_PAREN);
    parse_expr(p);
    p.expect(RIGHT_PAREN);
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
    parse_expr(p);
    p.expect(NEWLINE);
    p.close(checkpoint, MODIFIER_APPLICATION);
}
