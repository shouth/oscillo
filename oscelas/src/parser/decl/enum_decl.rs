use crate::parser::Parser;
use crate::parser::decl::parse_qualified_identifier;
use crate::parser::expr::parse_expr;
use crate::syntax::OscSyntaxKind::*;

pub fn parse_enum_declaration(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(ENUM_KW);
    parse_qualified_identifier(p);
    p.expect(COLON);
    if p.check(LEFT_BRACKET) {
        parse_enum_member_decls(p);
    }
    p.expect(NEWLINE);
    p.close(checkpoint, ENUM_DECLARATION);
}

pub fn parse_enum_member_decls(p: &mut Parser) {
    let checkpoint = p.open();
    p.expect(LEFT_BRACKET);

    let list_checkpoint = p.open();
    while !p.check(RIGHT_BRACKET | EOF) {
        let element_checkpoint = p.open();
        parse_qualified_identifier(p);
        let init_checkpoint = p.open();
        if p.eat(EQUAL) {
            parse_expr(p);
            p.close(init_checkpoint, ENUM_INITIALIZER_CLAUSE);
        }
        if !p.check(RIGHT_BRACKET) {
            p.expect(COMMA);
        }
        p.close(element_checkpoint, ENUM_MEMBER_DECL);
    }
    p.close(list_checkpoint, ENUM_MEMBER_DECL_LIST);

    p.expect(RIGHT_BRACKET);
    p.close(checkpoint, ENUM_MEMBER_DECLS);
}
