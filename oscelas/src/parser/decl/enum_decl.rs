use crate::syntax::OscSyntaxKind::*;

use crate::parser::Parser;
use crate::parser::decl::parse_qualified_identifier;
use crate::parser::expr::parse_expr;

pub fn parse_enum_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ENUM_KW);
    parse_qualified_identifier(p);
    p.expect(COLON);
    p.expect(LEFT_BRACKET);
    parse_enum_member_decl_list(p);
    p.expect(RIGHT_BRACKET);
    p.expect(NEWLINE);
    p.close(checkpoint, ENUM_DECLARATION);
}

pub fn parse_enum_member_decl_list(p: &mut Parser) {
    let checkpoint = p.open();
    let mut flag = true;
    while flag {
        let element_checkpoint = p.open();
        parse_qualified_identifier(p);
        let init_checkpoint = p.open();
        if p.eat(EQUAL) {
            parse_expr(p);
            p.close(init_checkpoint, ENUM_INITIALIZER_CLAUSE);
        }
        flag = p.eat(COMMA);
        p.close(element_checkpoint, ENUM_MEMBER_DECL);
    }
    p.close(checkpoint, ENUM_MEMBER_DECL_LIST);
}
