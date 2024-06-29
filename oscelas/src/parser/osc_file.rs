use crate::parser::common::first_qualified_identifier;
use crate::syntax::{OscSyntaxKind::*, OscSyntaxKindSet};

use crate::parser::decl::{first_osc_declaration, parse_osc_declaration};
use crate::parser::Parser;

pub fn parse_osc_file(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.eof() {
        if p.check(IMPORT_KW) {
            parse_prelude_statement(p);
        } else if p.check(first_main_statement()) {
            parse_main_statement(p);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(checkpoint, OSC_FILE);
}

fn parse_prelude_statement(p: &mut Parser) {
    if p.check(IMPORT_KW) {
        parse_import_statement(p);
    } else {
        p.unexpected();
    }
}

pub fn parse_import_statement(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(IMPORT_KW);
    parse_import_reference(p);
    p.expect(NEWLINE);
    p.close(checkpoint, IMPORT_STATEMENT);
}

fn parse_import_reference(p: &mut Parser) {
    if p.eat(STRING_LITERAL) {
        // string literal
    } else {
        parse_structured_identifier(p);
    }
}

pub fn parse_structured_identifier(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(IDENTIFIER);
    while p.eat(DOT) {
        p.expect(IDENTIFIER);
        p.close(checkpoint.clone(), PREFIXED_STRUCTURED_IDENTIFIER);
    }
}

fn first_main_statement() -> OscSyntaxKindSet {
    NAMESPACE_KW | EXPORT_KW | first_osc_declaration()
}

fn parse_main_statement(p: &mut Parser) {
    if p.check(NAMESPACE_KW) {
        parse_namespace_statement(p);
    } else if p.check(EXPORT_KW) {
        parse_export_statement(p);
    } else if p.check(first_osc_declaration()) {
        parse_osc_declaration(p);
    } else {
        p.unexpected();
    }
}

pub fn parse_namespace_statement(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(NAMESPACE_KW);
    p.expect(IDENTIFIER | NULL_KW); // namespace name

    if p.check(USE_KW) {
        parse_namespace_use_clause(p);
    }

    p.expect(NEWLINE);
    p.close(checkpoint, NAMESPACE_STATEMENT);
}

fn parse_namespace_use_clause(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(USE_KW);
    parse_namespace_list(p);
    p.close(checkpoint, NAMESPACE_USE_CLAUSE);
}

fn parse_namespace_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(NEWLINE) && !p.eof() {
        if p.check(first_qualified_identifier()) {
            let element_checkpoint = p.open();
            p.expect(IDENTIFIER | NULL_KW);
            if !p.check(NEWLINE) {
                p.expect(COMMA);
            }
            p.close(element_checkpoint, NAMESPACE_LIST_ELEMENT);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(checkpoint, NAMESPACE_LIST);
}

pub fn parse_export_statement(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EXPORT_KW);
    parse_export_specificatoin_list(p);
    p.expect(NEWLINE);
    p.close(checkpoint, EXPORT_STATEMENT);
}

pub fn parse_export_specificatoin_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(NEWLINE) && !p.eof() {
        if !p.check(first_qualified_identifier() | STAR) {
            let element_checkpoint = p.open();
            parse_export_specification(p);
            if !p.check(NEWLINE) {
                p.expect(COMMA);
            }
            p.close(element_checkpoint, EXPORT_SPECIFICATION_LIST_ELEMENT);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(checkpoint, EXPORT_SPECIFICATION_LIST);
}

pub fn parse_export_specification(p: &mut Parser) {
    let checkpoint = p.open();
    if p.eat(NULL_KW) {
        p.expect(COLON_COLON);
        if p.eat(IDENTIFIER) {
            p.close(checkpoint, PREFIXED_IDENTIFIER);
        } else if p.eat(STAR) {
            p.close(checkpoint, EXPORT_WILDCARD_SPECIFICATION);
        } else {
            p.unexpected();
        }
    } else if p.eat(IDENTIFIER) {
        if p.eat(COLON) {
            if p.eat(IDENTIFIER) {
                p.close(checkpoint, PREFIXED_IDENTIFIER);
            } else if p.eat(STAR) {
                p.close(checkpoint, EXPORT_WILDCARD_SPECIFICATION);
            } else {
                p.unexpected();
            }
        } else {
            // identifier
        }
    } else if p.eat(STAR) {
        p.close(checkpoint, EXPORT_WILDCARD_SPECIFICATION);
    } else {
        p.unexpected();
    }
}
