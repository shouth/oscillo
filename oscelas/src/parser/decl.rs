mod action_decl;
mod actor_decl;
mod enum_decl;
mod modifier_decl;
mod physical_type_decl;
mod scenario_decl;
mod struct_decl;
mod type_extension;

pub use action_decl::*;
pub use actor_decl::*;
pub use enum_decl::*;
pub use modifier_decl::*;
pub use physical_type_decl::*;
pub use scenario_decl::*;
pub use struct_decl::*;
pub use type_extension::*;

use crate::syntax::OscDslSyntaxKind::*;

use crate::parser::Parser;
use crate::parser::common::parse_qualified_identifier;
use crate::parser::member::parse_parameter_declaration;

pub fn parse_qualified_behavior_name(p: &mut Parser) {
    let checkpoint = p.open();
    parse_qualified_identifier(p);
    if p.eat(DOT) {
        parse_qualified_identifier(p);
        p.close(checkpoint, PREFIXED_BEHAVIOR_NAME);
    }
}

pub fn parse_type_declarator(p: &mut Parser) {
    if p.check_any(&[LIST_KW]) {
        parse_aggregate_type_declarator(p);
    } else {
        parse_non_aggregate_type_declarator(p);
    }
}

pub fn parse_non_aggregate_type_declarator(p: &mut Parser) {
    if p.eat_any(&[INT_KW, UINT_KW, FLOAT_KW, BOOL_KW, STRING_KW]) {
        // primitive type
    } else if p.check_any(&[IDENTIFIER, NULL_KW, COLON_COLON]) {
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
