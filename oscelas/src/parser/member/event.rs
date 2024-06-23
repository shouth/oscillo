use crate::parser::common::parse_argument_list_specification;
use crate::syntax::OscSyntaxKind::*;

use crate::parser::Parser;
use crate::parser::common::parse_qualified_identifier;
use crate::parser::expr::parse_expr;

pub fn parse_event_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EVENT_KW);
    parse_qualified_identifier(p);

    if p.check(LEFT_PAREN) {
        parse_event_argument_list_specification(p);
    }

    if p.check(IS_KW) {
        parse_event_is_clause(p);
    }

    p.expect(NEWLINE);
    p.close(checkpoint, EVENT_DECLARATION);
}

pub fn parse_event_argument_list_specification(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(LEFT_PAREN);
    parse_argument_list_specification(p);
    p.expect(RIGHT_PAREN);
    p.close(checkpoint, EVENT_ARGUMENT_LIST_SPECIFICATION);
}

pub fn parse_event_is_clause(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(IS_KW);
    parse_event_specification(p);
    p.close(checkpoint, EVENT_IS_CLAUSE);
}

pub fn parse_event_specification(p: &mut Parser) {
    if p.check(AT) {
        parse_event_reference_specification(p);
    } else {
        parse_event_condition(p);
    }
}

pub fn parse_event_reference_specification(p: &mut Parser) {
    let checkpoint = p.open();
    parse_event_reference(p);
    if p.check_any(&[IF_KW, AS_KW]) {
        parse_event_reference_condition(p);
    }
    p.close(checkpoint, EVENT_REFERENCE_SPECIFICATION);
}

pub fn parse_event_reference(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(AT);
    parse_expr(p);
    p.close(checkpoint, EVENT_REFERENCE);
}

pub fn parse_event_reference_condition(p: &mut Parser) {
    let checkpoint = p.open();

    let decl_checkpoint = p.open();
    if p.eat(AS_KW) {
        parse_qualified_identifier(p);
        p.close(decl_checkpoint, EVENT_FIELD_DECL);
    }

    p.expect(IF_KW);
    parse_event_condition(p);
    p.close(checkpoint, EVENT_REFERENCE_CONDITION);
}

pub fn parse_event_condition(p: &mut Parser) {
    let checkpoint = p.open();

    if p.eat(RISE_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint, RISE_EXPRESSION);
    } else if p.eat(FALL_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint, FALL_EXPRESSION);
    } else if p.eat(ELAPSED_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint, ELAPSED_EXPRESSION);
    } else if p.eat(EVERY_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p);

        if p.eat(COMMA) {
            let offset_checkpoint = p.open();
            p.expect(OFFSET_KW);
            p.expect(COLON);
            parse_expr(p);
            p.close(offset_checkpoint, EVERY_EXP_OFFSET);
        }

        p.expect(RIGHT_PAREN);
        p.close(checkpoint, EVERY_EXPRESSION);
    } else {
        parse_expr(p);
    }
}
