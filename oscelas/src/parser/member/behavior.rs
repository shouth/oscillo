use crate::parser::common::{parse_arguments, parse_qualified_identifier};
use crate::parser::expr::{check_expr, parse_expr, parse_remaining_expr};
use crate::parser::member::{
    parse_constraint_declaration, parse_event_specification, parse_modifier_application,
};
use crate::parser::{error_unexpected, Parser};
use crate::syntax::OscSyntaxKind::*;

pub fn parse_behavior_specification(p: &mut Parser) {
    if p.check(ON_KW) {
        parse_on_directive(p);
    } else if p.check(DO_KW) {
        parse_do_directive(p);
    } else {
        error_unexpected(p);
    }
}

pub fn parse_on_directive(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ON_KW);
    parse_event_specification(p);
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);

    let list_checkpoint = p.open();
    while !p.check(DEDENT | EOF) {
        if p.check(CALL_KW | EMIT_KW) {
            parse_on_member(p);
        } else {
            error_unexpected(p);
            p.error();
        }
    }
    p.close(list_checkpoint, ON_MEMBER_LIST);

    p.expect(DEDENT);
    p.close(checkpoint, ON_DIRECTIVE);
}

pub fn parse_on_member(p: &mut Parser) {
    if p.check(CALL_KW) {
        parse_call_directive(p);
    } else if p.check(EMIT_KW) {
        parse_emit_directive(p);
    } else {
        error_unexpected(p);
    }
}

pub fn parse_do_directive(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(DO_KW);
    parse_do_member(p);
    p.close(checkpoint, DO_DIRECTIVE);
}

pub fn check_do_member(p: &mut Parser) -> bool {
    p.check(SERIAL_KW | ONE_OF_KW | PARALLEL_KW | WAIT_KW | EMIT_KW | CALL_KW) || check_expr(p)
}

pub fn parse_do_member(p: &mut Parser) {
    let checkpoint = p.open();
    if p.check(SERIAL_KW | ONE_OF_KW | PARALLEL_KW | WAIT_KW | EMIT_KW | CALL_KW) {
        parse_do_member_body(p);
    } else {
        parse_qualified_identifier(p);
        if p.eat(COLON) {
            parse_do_member_body(p);
        } else {
            parse_remaining_expr(p, checkpoint.clone());
            parse_behavior_with_declaration_or_newline(p);
            p.close(checkpoint.clone(), BEHAVIOR_INVOCATION);
        }
    }
    p.close(checkpoint, DO_MEMBER);
}

fn parse_do_member_body(p: &mut Parser) {
    if p.check(SERIAL_KW | ONE_OF_KW | PARALLEL_KW) {
        parse_composition(p);
    } else if p.check(WITH_KW) {
        parse_wait_directive(p);
    } else if p.check(EMIT_KW) {
        parse_emit_directive(p);
    } else if p.check(CALL_KW) {
        parse_call_directive(p);
    } else if check_expr(p) {
        parse_behavior_invocation(p);
    } else {
        error_unexpected(p);
    }
}

pub fn parse_composition(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SERIAL_KW | ONE_OF_KW | PARALLEL_KW);
    if p.check(LEFT_PAREN) {
        parse_arguments(p);
    }
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);

    let list_checkpoint = p.open();
    while !p.check(DEDENT | EOF) {
        if check_do_member(p) {
            parse_do_member(p);
        } else {
            error_unexpected(p);
            p.error();
        }
    }
    p.close(list_checkpoint, DO_MEMBER_LIST);
    p.expect(DEDENT);

    if p.check(WITH_KW) {
        parse_behavior_with_declaration(p);
    }
    p.close(checkpoint, COMPOSITION);
}

pub fn parse_behavior_invocation(p: &mut Parser) {
    let checkpoint = p.open();
    parse_expr(p);
    parse_behavior_with_declaration_or_newline(p);
    p.close(checkpoint, BEHAVIOR_INVOCATION);
}

fn parse_behavior_with_declaration_or_newline(p: &mut Parser) {
    if p.check(WITH_KW) {
        parse_behavior_with_declaration(p);
    } else if p.eat(NEWLINE) {
        // newline
    } else {
        error_unexpected(p);
    }
}

fn parse_behavior_with_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(WITH_KW);
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);

    let list_checkpoint = p.open();
    while !p.check(DEDENT | EOF) {
        if check_behavior_with_member(p) {
            parse_behavior_with_member(p);
        } else {
            error_unexpected(p);
            p.error();
        }
    }
    p.close(list_checkpoint, BEHAVIOR_WITH_MEMBER_LIST);
    p.expect(DEDENT);
    p.close(checkpoint, BEHAVIOR_WITH_DECLARATION);
}

fn check_behavior_with_member(p: &mut Parser) -> bool {
    p.check(KEEP_KW | REMOVE_DEFAULT_KW | UNTIL_KW) || check_expr(p)
}

fn parse_behavior_with_member(p: &mut Parser) {
    if p.check(KEEP_KW | REMOVE_DEFAULT_KW) {
        parse_constraint_declaration(p);
    } else if p.check(UNTIL_KW) {
        parse_until_directive(p);
    } else if check_expr(p) {
        parse_modifier_application(p);
    } else {
        error_unexpected(p);
    }
}

pub fn parse_wait_directive(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(WITH_KW);
    parse_event_specification(p);
    p.expect(NEWLINE);
    p.close(checkpoint, WAIT_DIRECTIVE);
}

pub fn parse_emit_directive(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EMIT_KW);
    parse_qualified_identifier(p);
    if p.check(LEFT_PAREN) {
        parse_arguments(p);
    }
    p.expect(NEWLINE);
    p.close(checkpoint, EMIT_DIRECTIVE);
}

pub fn parse_call_directive(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(CALL_KW);
    parse_expr(p);
    p.expect(NEWLINE);
    p.close(checkpoint, CALL_DIRECTIVE);
}

pub fn parse_until_directive(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(UNTIL_KW);
    parse_event_specification(p);
    p.expect(NEWLINE);
    p.close(checkpoint, UNTIL_DIRECTIVE);
}
