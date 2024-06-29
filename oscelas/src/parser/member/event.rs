use crate::parser::common::{parse_argument_spcifications, parse_qualified_identifier};
use crate::parser::expr::parse_expr;
use crate::parser::member::first_structured_type_member;
use crate::parser::Parser;
use crate::syntax::{OscSyntaxKind::*, OscSyntaxKindSet};

pub fn parse_event_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EVENT_KW);
    parse_qualified_identifier(p);

    if p.check(LEFT_PAREN) {
        parse_argument_spcifications(p);
    }

    if p.check(IS_KW) {
        parse_event_is_clause(p, DEDENT | NEWLINE | first_structured_type_member());
    }

    p.expect(NEWLINE);
    p.close(checkpoint, EVENT_DECLARATION);
}

fn parse_event_is_clause(p: &mut Parser, recovery: OscSyntaxKindSet) {
    let checkpoint = p.open();
    p.expect(IS_KW);
    parse_event_specification(p, recovery);
    p.close(checkpoint, EVENT_IS_CLAUSE);
}

pub fn parse_event_specification(p: &mut Parser, recovery: OscSyntaxKindSet) {
    if p.check(AT) {
        parse_event_reference_specification(p, recovery);
    } else {
        parse_event_condition(p, recovery);
    }
}

fn parse_event_reference_specification(p: &mut Parser, recovery: OscSyntaxKindSet) {
    let checkpoint = p.open();
    parse_event_reference(p, IF_KW | AS_KW | recovery);
    if p.check(IF_KW | AS_KW) {
        parse_event_reference_condition(p, recovery);
    }
    p.close(checkpoint, EVENT_REFERENCE_SPECIFICATION);
}

fn parse_event_reference(p: &mut Parser, recovery: OscSyntaxKindSet) {
    let checkpoint = p.open();
    p.expect(AT);
    parse_expr(p, recovery);
    p.close(checkpoint, EVENT_REFERENCE);
}

fn parse_event_reference_condition(p: &mut Parser, recovery: OscSyntaxKindSet) {
    let checkpoint = p.open();

    let decl_checkpoint = p.open();
    if p.eat(AS_KW) {
        parse_qualified_identifier(p);
        p.close(decl_checkpoint, EVENT_FIELD_DECL);
    }

    p.expect(IF_KW);
    parse_event_condition(p, recovery);
    p.close(checkpoint, EVENT_REFERENCE_CONDITION);
}

fn parse_event_condition(p: &mut Parser, recovery: OscSyntaxKindSet) {
    let checkpoint = p.open();

    if p.eat(RISE_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p, RIGHT_PAREN | recovery);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint, RISE_EXPRESSION);
    } else if p.eat(FALL_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p, RIGHT_PAREN | recovery);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint, FALL_EXPRESSION);
    } else if p.eat(ELAPSED_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p, RIGHT_PAREN | recovery);
        p.expect(RIGHT_PAREN);
        p.close(checkpoint, ELAPSED_EXPRESSION);
    } else if p.eat(EVERY_KW) {
        p.expect(LEFT_PAREN);
        parse_expr(p, COMMA | RIGHT_PAREN | recovery);

        if p.eat(COMMA) {
            let offset_checkpoint = p.open();
            p.expect(OFFSET_KW);
            p.expect(COLON);
            parse_expr(p, RIGHT_PAREN | recovery);
            p.close(offset_checkpoint, EVERY_EXP_OFFSET);
        }

        p.expect(RIGHT_PAREN);
        p.close(checkpoint, EVERY_EXPRESSION);
    } else {
        parse_expr(p, recovery);
    }
}
