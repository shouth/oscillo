use crate::parser::Parser;
use crate::parser::common::parse_qualified_identifier;
use crate::parser::expr::parse_expr;
use crate::syntax::OscSyntaxKind::*;

pub fn parse_physical_type_decl(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(TYPE_KW);
    parse_qualified_identifier(p);
    p.expect(IS_KW);
    parse_base_unit_specifier(p);
    p.expect(NEWLINE);
    p.close(checkpoint, PHYSICAL_TYPE_DECLARATION);
}

pub fn parse_unit_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(UNIT_KW);
    parse_qualified_identifier(p);
    p.expect(OF_KW);
    parse_qualified_identifier(p);
    p.expect(IS_KW);
    parse_unit_specifier(p);
    p.expect(NEWLINE);
    p.close(checkpoint, UNIT_DECLARATION);
}

fn parse_base_unit_specifier(p: &mut Parser) {
    parse_si_base_unit_specifier(p);
}

fn parse_unit_specifier(p: &mut Parser) {
    parse_si_unit_specifier(p);
}

fn parse_si_base_unit_specifier(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SI_KW);
    p.expect(LEFT_PAREN);
    parse_si_base_exponent_list(p);
    p.expect(RIGHT_PAREN);
    p.close(checkpoint, SI_BASE_UNIT_SPECIFIER);
}

fn parse_si_base_exponent_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(RIGHT_PAREN | EOF) {
        let element_checkpoint = p.open();
        parse_si_base_unit_name(p);
        p.expect(COLON);
        parse_expr(p);
        if !p.check(RIGHT_PAREN) {
            p.expect(COMMA);
        }
        p.close(element_checkpoint, SI_BASE_EXPONENT);
    }
    p.close(checkpoint, SI_BASE_EXPONENT_LIST);
}

fn parse_si_unit_specifier(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SI_KW);
    p.expect(LEFT_PAREN);
    parse_si_unit_argument_list(p);
    p.expect(RIGHT_PAREN);
    p.close(checkpoint, SI_UNIT_SPECIFIER);
}

fn parse_si_unit_argument_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(RIGHT_PAREN | EOF) {
        let element_checkpoint = p.open();
        parse_si_unit_argument_name(p);
        p.expect(COLON);
        parse_expr(p);
        if !p.check(RIGHT_PAREN) {
            p.expect(COMMA);
        }
        p.close(element_checkpoint, SI_UNIT_ARGUMENT);
    }
    p.close(checkpoint, SI_UNIT_ARGUMENT_LIST);
}

fn parse_si_unit_argument_name(p: &mut Parser) {
    p.expect(FACTOR_KW | OFFSET_KW);
}

fn parse_si_base_unit_name(p: &mut Parser) {
    p.expect(KG_KW | M_KW | S_KW | A_KW | K_KW | MOL_KW | CD_KW | RAD_KW);
}
