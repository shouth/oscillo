
use crate::parser::common::{check_qualifed_identifier, parse_qualified_identifier};
use crate::parser::{error_unexpected, Parser};
use crate::parser::decl::parse_type_declarator;
use crate::parser::expr::parse_expr;
use crate::parser::member::parse_constraint_declaration;
use crate::syntax::OscSyntaxKind::*;

fn parse_field_name_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(COLON | EOF) {
        let element_checkpoint = p.open();
        parse_qualified_identifier(p);
        if !p.check(COLON) {
            p.expect(COMMA);
        }
        p.close(element_checkpoint, FIELD_NAME_LIST_ELEMENT);
    }
    p.close(checkpoint, FIELD_NAME_LIST);
}

pub fn check_field_declaration(p: &mut Parser) -> bool {
    p.check(VAR_KW) || check_qualifed_identifier(p)
}

pub fn parse_field_declaration(p: &mut Parser) {
    if p.check(VAR_KW) {
        parse_variable_declaration(p);
    } else if check_qualifed_identifier(p) {
        parse_parameter_declaration(p);
    } else {
        error_unexpected(p);
    }
}

pub fn parse_variable_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(VAR_KW);
    parse_field_name_list(p);
    p.expect(COLON);
    parse_type_declarator(p);

    let init_checkpoint = p.open();
    if p.eat(EQUAL) {
        parse_expr(p);
        p.close(init_checkpoint, VARIABLE_INITIALIZER_CLAUSE);
    }
    p.expect(NEWLINE);
    p.close(checkpoint, VARIABLE_DECLARATION);
}

pub fn parse_parameter_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    parse_field_name_list(p);
    p.expect(COLON);
    parse_type_declarator(p);

    let init_checkpoint = p.open();
    if p.eat(EQUAL) {
        parse_expr(p);
        p.close(init_checkpoint, PARAMETER_INITIALIZER_CLAUSE);
    }

    if p.check(WITH_KW) {
        parse_parameter_with_declaration(p);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        error_unexpected(p);
    }
    p.close(checkpoint, PARAMETER_DECLARATION);
}

fn parse_parameter_with_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(WITH_KW);
    p.expect(COLON);
    p.expect(NEWLINE);
    p.expect(INDENT);

    let list_checkpoint = p.open();
    while !p.check(DEDENT | EOF) {
        if p.check(KEEP_KW | REMOVE_DEFAULT_KW) {
            parse_constraint_declaration(p);
        } else {
            error_unexpected(p);
            p.error();
        }
    }
    p.close(list_checkpoint, PARAMETER_WITH_MEMBER_LIST);

    p.expect(DEDENT);
    p.close(checkpoint, PARAMETER_WITH_DECLARATION);
}
