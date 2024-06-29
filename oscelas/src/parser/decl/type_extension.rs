use crate::parser::decl::{parse_enum_member_decls, parse_qualified_behavior_name};
use crate::parser::member::parse_structured_type_member_list;
use crate::parser::{error_unexpected, Parser};
use crate::syntax::OscSyntaxKind::*;

pub fn parse_type_extenstion(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(EXTEND_KW);
    parse_qualified_behavior_name(p);
    parse_type_extension_body(p);
    p.close(checkpoint, TYPE_EXTENSION);
}

fn parse_type_extension_body(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(COLON);
    if p.check(LEFT_BRACKET) {
        parse_enum_member_decls(p);
        p.expect(NEWLINE);
        p.close(checkpoint, ENUM_TYPE_EXTENSION_BODY);
    } else if p.eat(NEWLINE) {
        p.expect(INDENT);
        parse_structured_type_member_list(p);
        p.expect(DEDENT);
        p.close(checkpoint, STRUCTURED_TYPE_EXTENSION_BODY);
    } else {
        error_unexpected(p);
    }
}
