use crate::parser::decl::parse_qualified_behavior_name;
use crate::parser::expr::parse_expr;
use crate::parser::member::parse_structured_type_member_list;
use crate::parser::{error_unexpected, Parser};
use crate::syntax::OscSyntaxKind::*;

pub fn parse_actor_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ACTOR_KW);
    parse_qualified_behavior_name(p);

    if p.check(INHERITS_KW) {
        parse_actor_inherits_clause(p);
    }

    if p.check(COLON) {
        parse_actor_body(p);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        error_unexpected(p);
    }

    p.close(checkpoint, ACTOR_DECLARATION);
}

fn parse_actor_inherits_clause(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(INHERITS_KW);
    parse_qualified_behavior_name(p);

    let condition_checkpoint = p.open();
    if p.eat(LEFT_PAREN) {
        parse_expr(p);
        p.expect(RIGHT_PAREN);
        p.close(condition_checkpoint, ACTOR_INHERITS_CONDITION);
    }
    p.close(checkpoint, ACTOR_INHERITS_CLAUSE);
}

fn parse_actor_body(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);
    parse_structured_type_member_list(p);
    p.expect(DEDENT);
    p.close(checkpoint, ACTOR_BODY);
}
