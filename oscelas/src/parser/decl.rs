use crate::parser::common::{first_qualified_identifier, parse_qualified_identifier};
use crate::parser::expr::parse_expr;
use crate::parser::member::{parse_parameter_declaration, parse_structured_type_member_list};
use crate::parser::Parser;
use crate::syntax::{OscSyntaxKind::*, OscSyntaxKindSet};

pub fn first_osc_declaration() -> OscSyntaxKindSet {
    TYPE_KW
        | UNIT_KW
        | ENUM_KW
        | STRUCT_KW
        | ACTOR_KW
        | ACTION_KW
        | SCENARIO_KW
        | MODIFIER_KW
        | EXTEND_KW
        | GLOBAL_KW
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

pub fn parse_qualified_behavior_name(p: &mut Parser) {
    let checkpoint = p.open();
    parse_qualified_identifier(p);
    if p.eat(DOT) {
        parse_qualified_identifier(p);
        p.close(checkpoint, PREFIXED_BEHAVIOR_NAME);
    }
}

pub fn parse_type_declarator(p: &mut Parser) {
    if p.check(LIST_KW) {
        parse_aggregate_type_declarator(p);
    } else {
        parse_non_aggregate_type_declarator(p);
    }
}

pub fn parse_non_aggregate_type_declarator(p: &mut Parser) {
    if p.eat(INT_KW | UINT_KW | FLOAT_KW | BOOL_KW | STRING_KW) {
        // primitive type
    } else if p.check(IDENTIFIER | NULL_KW | COLON_COLON) {
        parse_qualified_behavior_name(p);
    } else {
        p.unexpected();
    }
}

pub fn parse_aggregate_type_declarator(p: &mut Parser) {
    let checkpoint = p.open();
    if p.eat(LIST_KW) {
        p.expect(OF_KW);
        parse_non_aggregate_type_declarator(p);
        p.close(checkpoint, LIST_TYPE_DECLARATOR);
    } else {
        p.unexpected();
    }
}

pub fn parse_global_parameter_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(GLOBAL_KW);
    parse_parameter_declaration(p);
    p.close(checkpoint, GLOBAL_PARAMETER_DECLARATION);
}

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

fn first_si_base_unit_name() -> OscSyntaxKindSet {
    KG_KW | M_KW | S_KW | A_KW | K_KW | MOL_KW | CD_KW | RAD_KW
}

fn parse_si_base_unit_specifier(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SI_KW);
    p.expect(LEFT_PAREN);

    let list_checkpoint = p.open();
    while !p.check(RIGHT_PAREN) && !p.eof() {
        if p.check(first_si_base_unit_name()) {
            let element_checkpoint = p.open();
            p.expect(first_si_base_unit_name());
            p.expect(COLON);
            parse_expr(p, COMMA | RIGHT_PAREN | NEWLINE);
            if !p.check(RIGHT_PAREN) {
                p.expect(COMMA);
            }
            p.close(element_checkpoint, SI_BASE_EXPONENT);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(list_checkpoint, SI_BASE_EXPONENT_LIST);

    p.expect(RIGHT_PAREN);
    p.close(checkpoint, SI_BASE_UNIT_SPECIFIER);
}

fn first_si_unit_specifier() -> OscSyntaxKindSet {
    FACTOR_KW | OFFSET_KW | first_si_base_unit_name()
}

fn parse_si_unit_specifier(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SI_KW);
    p.expect(LEFT_PAREN);

    let list_checkpoint = p.open();
    while !p.check(RIGHT_PAREN) && !p.eof() {
        if p.check(first_si_unit_specifier()) {
            let element_checkpoint = p.open();
            p.expect(first_si_unit_specifier());
            p.expect(COLON);
            parse_expr(p, COMMA | RIGHT_PAREN | NEWLINE);
            if !p.check(RIGHT_PAREN) {
                p.expect(COMMA);
            }
            p.close(element_checkpoint, SI_UNIT_ARGUMENT);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(list_checkpoint, SI_UNIT_ARGUMENT_LIST);

    p.expect(RIGHT_PAREN);
    p.close(checkpoint, SI_UNIT_SPECIFIER);
}

pub fn parse_enum_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ENUM_KW);
    parse_qualified_identifier(p);
    p.expect(COLON);
    parse_enum_member_decls(p);
    p.expect(NEWLINE);
    p.close(checkpoint, ENUM_DECLARATION);
}

pub fn parse_enum_member_decls(p: &mut Parser) {
    let checkpoint = p.open();
    if !p.expect(LEFT_BRACKET) {
        return;
    }

    // allow empty enum when parsing
    let list_checkpoint = p.open();
    while !p.check(RIGHT_BRACKET) && !p.eof() {
        if p.check(first_qualified_identifier()) {
            let element_checkpoint = p.open();
            parse_qualified_identifier(p);
            let init_checkpoint = p.open();
            if p.eat(EQUAL) {
                parse_expr(p, RIGHT_BRACKET | COMMA);
                p.close(init_checkpoint, ENUM_INITIALIZER_CLAUSE);
            }
            if !p.check(RIGHT_BRACKET) {
                p.expect(COMMA);
            }
            p.close(element_checkpoint, ENUM_MEMBER_DECL);
        } else {
            p.unexpected();
            p.error();
        }
    }
    p.close(list_checkpoint, ENUM_MEMBER_DECL_LIST);

    p.expect(RIGHT_BRACKET);
    p.close(checkpoint, ENUM_MEMBER_DECLS);
}

pub fn parse_struct_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(STRUCT_KW);
    parse_qualified_behavior_name(p);

    if p.check(INHERITS_KW) {
        let checkpoint = p.open();
        p.expect(INHERITS_KW);
        parse_qualified_behavior_name(p);

        let condition_checkpoint = p.open();
        if p.eat(LEFT_PAREN) {
            parse_qualified_behavior_name(p);
            p.expect(RIGHT_PAREN);
            p.close(condition_checkpoint, STRUCT_INHERITS_CONDITION);
        }
        p.close(checkpoint, STRUCT_INHERITS_CLAUSE);
    }

    if p.check(COLON) {
        let checkpoint = p.open();
        p.expect(COLON);
        p.expect(NEWLINE);
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, STRUCT_BODY);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        p.unexpected();
    }

    p.close(checkpoint, STRUCT_DECLARATION);
}

pub fn parse_actor_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ACTOR_KW);
    parse_qualified_behavior_name(p);

    if p.check(INHERITS_KW) {
        let checkpoint = p.open();
        p.expect(INHERITS_KW);
        parse_qualified_behavior_name(p);

        let condition_checkpoint = p.open();
        if p.eat(LEFT_PAREN) {
            parse_expr(p, RIGHT_PAREN | COLON | NEWLINE);
            p.expect(RIGHT_PAREN);
            p.close(condition_checkpoint, ACTOR_INHERITS_CONDITION);
        }
        p.close(checkpoint, ACTOR_INHERITS_CLAUSE);
    }

    if p.check(COLON) {
        let checkpoint = p.open();
        p.expect(COLON);
        p.expect(NEWLINE);
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, ACTOR_BODY);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        p.unexpected();
    }

    p.close(checkpoint, ACTOR_DECLARATION);
}

pub fn parse_scenario_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(SCENARIO_KW);
    parse_qualified_behavior_name(p);

    if p.check(INHERITS_KW) {
        let checkpoint = p.open();
        p.expect(INHERITS_KW);
        parse_qualified_behavior_name(p);

        let condition_checkpoint = p.open();
        if p.eat(LEFT_PAREN) {
            parse_expr(p, RIGHT_PAREN | COLON | NEWLINE);
            p.expect(RIGHT_PAREN);
            p.close(condition_checkpoint, SCENARIO_INHERITS_CONDITION);
        }
        p.close(checkpoint, SCENARIO_INHERITS_CLAUSE);
    }

    if p.check(COLON) {
        let checkpoint = p.open();
        p.expect(COLON);
        p.expect(NEWLINE);
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, SCENARIO_BODY);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        p.unexpected();
    }

    p.close(checkpoint, SCENARIO_DECLARATION);
}

pub fn parse_action_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ACTION_KW);
    parse_qualified_behavior_name(p);

    if p.check(INHERITS_KW) {
        let checkpoint = p.open();
        p.expect(INHERITS_KW);
        parse_qualified_behavior_name(p);

        let condition_checkpoint = p.open();
        if p.eat(LEFT_PAREN) {
            parse_expr(p, RIGHT_PAREN | COLON | NEWLINE);
            p.expect(RIGHT_PAREN);
            p.close(condition_checkpoint, ACTION_INHERITS_CONDITION);
        }
        p.close(checkpoint, ACTION_INHERITS_CLAUSE);
    }

    if p.check(COLON) {
        let checkpoint = p.open();
        p.expect(COLON);
        p.expect(NEWLINE);
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, ACTION_BODY);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        p.unexpected();
    }

    p.close(checkpoint, ACTION_DECLARATION);
}

pub fn parse_modifier_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(MODIFIER_KW);
    parse_qualified_behavior_name(p);

    if p.check(OF_KW) {
        let checkpoint = p.open();
        p.expect(OF_KW);
        parse_qualified_behavior_name(p);
        p.close(checkpoint, MODIFIER_OF_CLAUSE);
    }

    if p.check(COLON) {
        let checkpoint = p.open();
        p.expect(COLON);
        p.expect(NEWLINE);
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, MODIFIER_BODY);
    } else if p.eat(NEWLINE) {
        // new line
    } else {
        p.unexpected();
    }

    p.close(checkpoint, MODIFIER_DECLARATION);
}

pub fn parse_type_extenstion(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EXTEND_KW);
    parse_qualified_behavior_name(p);

    let body_checkpoint = p.open();
    p.expect(COLON);
    if p.check(LEFT_BRACKET) {
        parse_enum_member_decls(p);
        p.expect(NEWLINE);
        p.close(body_checkpoint, ENUM_TYPE_EXTENSION_BODY);
    } else if p.eat(NEWLINE) {
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(body_checkpoint, STRUCTURED_TYPE_EXTENSION_BODY);
    } else {
        p.unexpected();
    }

    p.close(checkpoint, TYPE_EXTENSION);
}
