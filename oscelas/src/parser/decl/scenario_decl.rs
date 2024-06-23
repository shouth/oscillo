use crate::syntax::OscSyntaxKind::*;

use crate::parser::Parser;
use crate::parser::decl::{parse_qualified_behavior_name, parse_modifier_declaration};
use crate::parser::expr::parse_expr;
use crate::parser::member::{
    parse_behavior_specification, parse_constraint_declaration, parse_coverage_declaration, parse_event_declaration, parse_field_declaration, parse_method_declaration
};

pub fn parse_scenario_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SCENARIO_KW);
    parse_qualified_behavior_name(p);

    if p.check(INHERITS_KW) {
        parse_scenario_inherits_clause(p);
    }

    if p.check(COLON) {
        parse_scenario_body(p);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        p.unexpected();
    }

    p.close(checkpoint, SCENARIO_DECLARATION);
}

pub fn parse_scenario_inherits_clause(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(INHERITS_KW);
    parse_qualified_behavior_name(p);

    let condition_checkpoint = p.open();
    if p.eat(LEFT_PAREN) {
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(condition_checkpoint, SCENARIO_INHERITS_CONDITION);
    }
    p.close(checkpoint, SCENARIO_INHERITS_CLAUSE);
}

pub fn parse_scenario_body(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);
    parse_scenario_member_list(p);
    p.expect(DEDENT);
    p.close(checkpoint, SCENARIO_BODY);
}

pub fn parse_scenario_member_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(DEDENT) {
        if p.check_any(&[ON_KW, DO_KW]) {
            parse_behavior_specification(p);
        } else if p.check(EVENT_KW) {
            parse_event_declaration(p);
        } else if p.check_any(&[KEEP_KW, REMOVE_DEFAULT_KW]) {
            parse_constraint_declaration(p);
        } else if p.check(DEF_KW) {
            parse_method_declaration(p);
        } else if p.check_any(&[COVER_KW, RECORD_KW]) {
            parse_coverage_declaration(p);
        } else if p.check(MODIFIER_KW) {
            parse_modifier_declaration(p);
        } else {
            parse_field_declaration(p);
        }
    }
    p.close(checkpoint, SCENARIO_MEMBER_LIST);
}
