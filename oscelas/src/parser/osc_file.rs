use crate::syntax::OscSyntaxKind::*;

use crate::parser::Parser;

use super::decl::{parse_action_declaration, parse_actor_declaration, parse_enum_declaration, parse_global_parameter_declaration, parse_modifier_declaration, parse_physical_type_decl, parse_scenario_declaration, parse_struct_declaration, parse_type_extenstion, parse_unit_declaration};

pub fn parse_osc_file(p: &mut Parser) {
    let checkpoint = p.open();
    parse_prelude_statement_list(p);
    parse_main_statement_list(p);
    p.close(checkpoint, OSC_FILE);
}

pub fn parse_prelude_statement_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(EOF) {
        if p.check(IMPORT_KW) {
            parse_import_statement(p);
        } else {
            break;
        }
    }
    p.close(checkpoint, PRELUDE_STATEMENT_LIST);
}

pub fn parse_import_statement(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(IMPORT_KW);
    parse_import_reference(p);
    p.expect(NEWLINE);
    p.close(checkpoint, IMPORT_STATEMENT);
}

pub fn parse_import_reference(p: &mut Parser) {
    if p.eat(STRING_LITERAL) {
        // string literal
    } else {
        parse_structured_identifier(p);
    }
}

pub fn parse_structured_identifier(p: &mut Parser) {
    let checkpoint = p.open();
    let mut flag = true;
    while flag {
        let element_checkpoint = p.open();
        p.expect(IDENTIFIER);
        flag = p.eat(DOT);
        p.close(element_checkpoint, STRUCTURED_IDENTIFIER_ELEMENT);
    }
    p.close(checkpoint, STRUCTURED_IDENTIFIER);
}

pub fn parse_main_statement_list(p: &mut Parser) {
    let checkpoint = p.open();
    while !p.check(EOF) {
        if p.check(NAMESPACE_KW) {
            parse_namespace_statement(p);
        } else if p.check(EXPORT_KW) {
            parse_export_statement(p);
        } else if p.check(TYPE_KW | UNIT_KW | ENUM_KW | STRUCT_KW | ACTOR_KW | ACTION_KW | SCENARIO_KW | MODIFIER_KW | EXTEND_KW | GLOBAL_KW) {
            parse_osc_declaration(p);
        } else {
            break;
        }
    }
    p.close(checkpoint, MAIN_STATEMENT_LIST);
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

pub fn parse_namespace_use_clause(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(USE_KW);
    parse_namespace_list(p);
    p.close(checkpoint, NAMESPACE_USE_CLAUSE);
}

pub fn parse_namespace_list(p: &mut Parser) {
    let checkpoint = p.open();
    let mut flag = true;
    while flag {
        let element_checkpoint = p.open();
        p.expect(IDENTIFIER | NULL_KW);
        flag = p.eat(COMMA);
        p.close(element_checkpoint, NAMESPACE_LIST_ELEMENT);
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
    let mut flag = true;
    while flag {
        let element_checkpoint = p.open();
        parse_export_specification(p);
        flag = p.eat(COMMA);
        p.close(element_checkpoint, EXPORT_SPECIFICATION_LIST_ELEMENT);
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

pub fn parse_osc_declaration(p: &mut Parser) {
    if p.check(TYPE_KW) {
        parse_physical_type_decl(p);
    } else if p.check(UNIT_KW) {
        parse_unit_declaration(p);
    } else if p.check(ENUM_KW) {
        parse_enum_declaration(p);
    } else if p.check(STRUCT_KW) {
        parse_struct_declaration(p);
    } else if p.check(ACTOR_KW) {
        parse_actor_declaration(p);
    } else if p.check(ACTION_KW) {
        parse_action_declaration(p);
    } else if p.check(SCENARIO_KW) {
        parse_scenario_declaration(p);
    } else if p.check(MODIFIER_KW) {
        parse_modifier_declaration(p);
    } else if p.check(EXTEND_KW) {
        parse_type_extenstion(p);
    } else if p.check(GLOBAL_KW) {
        parse_global_parameter_declaration(p);
    } else {
        p.unexpected();
    }
}
