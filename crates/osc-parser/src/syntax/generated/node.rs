use super::OscSyntaxKind::{self, *};
use crate::syntax::{support, TypedNode};
use syntree::Node;
type OscNode<'a> = Node<'a, OscSyntaxKind, u32, usize>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotToken<'a>(OscNode<'a>);
impl<'a> TypedNode for DotToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DOT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotDotToken<'a>(OscNode<'a>);
impl<'a> TypedNode for DotDotToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DOT_DOT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommaToken<'a>(OscNode<'a>);
impl<'a> TypedNode for CommaToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COMMA
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColonToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ColonToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COLON
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColonColonToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ColonColonToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COLON_COLON
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignToken<'a>(OscNode<'a>);
impl<'a> TypedNode for AssignToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ASSIGN
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtToken<'a>(OscNode<'a>);
impl<'a> TypedNode for AtToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == AT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrowToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ArrowToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARROW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftParenToken<'a>(OscNode<'a>);
impl<'a> TypedNode for LeftParenToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LEFT_PAREN
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightParenToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RightParenToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RIGHT_PAREN
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftBracketToken<'a>(OscNode<'a>);
impl<'a> TypedNode for LeftBracketToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LEFT_BRACKET
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightBracketToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RightBracketToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RIGHT_BRACKET
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionToken<'a>(OscNode<'a>);
impl<'a> TypedNode for QuestionToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == QUESTION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExclamationToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ExclamationToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXCLAMATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FatArrowToken<'a>(OscNode<'a>);
impl<'a> TypedNode for FatArrowToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FAT_ARROW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualToken<'a>(OscNode<'a>);
impl<'a> TypedNode for EqualToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EQUAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotEqualToken<'a>(OscNode<'a>);
impl<'a> TypedNode for NotEqualToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NOT_EQUAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessToken<'a>(OscNode<'a>);
impl<'a> TypedNode for LessToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LESS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessEqualToken<'a>(OscNode<'a>);
impl<'a> TypedNode for LessEqualToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LESS_EQUAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreaterToken<'a>(OscNode<'a>);
impl<'a> TypedNode for GreaterToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == GREATER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreaterEqualToken<'a>(OscNode<'a>);
impl<'a> TypedNode for GreaterEqualToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == GREATER_EQUAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlusToken<'a>(OscNode<'a>);
impl<'a> TypedNode for PlusToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PLUS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinusToken<'a>(OscNode<'a>);
impl<'a> TypedNode for MinusToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MINUS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarToken<'a>(OscNode<'a>);
impl<'a> TypedNode for StarToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STAR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlashToken<'a>(OscNode<'a>);
impl<'a> TypedNode for SlashToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SLASH
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PercentToken<'a>(OscNode<'a>);
impl<'a> TypedNode for PercentToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PERCENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AToken<'a>(OscNode<'a>);
impl<'a> TypedNode for AToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == A_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ActionToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTION_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ActorToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTOR_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndToken<'a>(OscNode<'a>);
impl<'a> TypedNode for AndToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == AND_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsToken<'a>(OscNode<'a>);
impl<'a> TypedNode for AsToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == AS_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolToken<'a>(OscNode<'a>);
impl<'a> TypedNode for BoolToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BOOL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallToken<'a>(OscNode<'a>);
impl<'a> TypedNode for CallToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == CALL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CdToken<'a>(OscNode<'a>);
impl<'a> TypedNode for CdToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == CD_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverToken<'a>(OscNode<'a>);
impl<'a> TypedNode for CoverToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COVER_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefToken<'a>(OscNode<'a>);
impl<'a> TypedNode for DefToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DEF_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultToken<'a>(OscNode<'a>);
impl<'a> TypedNode for DefaultToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DEFAULT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoToken<'a>(OscNode<'a>);
impl<'a> TypedNode for DoToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DO_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElapsedToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ElapsedToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ELAPSED_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitToken<'a>(OscNode<'a>);
impl<'a> TypedNode for EmitToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EMIT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumToken<'a>(OscNode<'a>);
impl<'a> TypedNode for EnumToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventToken<'a>(OscNode<'a>);
impl<'a> TypedNode for EventToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryToken<'a>(OscNode<'a>);
impl<'a> TypedNode for EveryToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVERY_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ExportToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPORT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ExpressionToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPRESSION_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ExtendToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXTEND_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ExternalToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXTERNAL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactorToken<'a>(OscNode<'a>);
impl<'a> TypedNode for FactorToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FACTOR_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallToken<'a>(OscNode<'a>);
impl<'a> TypedNode for FallToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FALL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FalseToken<'a>(OscNode<'a>);
impl<'a> TypedNode for FalseToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FALSE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatToken<'a>(OscNode<'a>);
impl<'a> TypedNode for FloatToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FLOAT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalToken<'a>(OscNode<'a>);
impl<'a> TypedNode for GlobalToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == GLOBAL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardToken<'a>(OscNode<'a>);
impl<'a> TypedNode for HardToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == HARD_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfToken<'a>(OscNode<'a>);
impl<'a> TypedNode for IfToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IF_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ImportToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IMPORT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InToken<'a>(OscNode<'a>);
impl<'a> TypedNode for InToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IN_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfToken<'a>(OscNode<'a>);
impl<'a> TypedNode for InfToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == INF_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InheritsToken<'a>(OscNode<'a>);
impl<'a> TypedNode for InheritsToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == INHERITS_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntToken<'a>(OscNode<'a>);
impl<'a> TypedNode for IntToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == INT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IsToken<'a>(OscNode<'a>);
impl<'a> TypedNode for IsToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IS_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ItToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KToken<'a>(OscNode<'a>);
impl<'a> TypedNode for KToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == K_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepToken<'a>(OscNode<'a>);
impl<'a> TypedNode for KeepToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == KEEP_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KgToken<'a>(OscNode<'a>);
impl<'a> TypedNode for KgToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == KG_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ListToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LIST_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MToken<'a>(OscNode<'a>);
impl<'a> TypedNode for MToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == M_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ModifierToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MODIFIER_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MolToken<'a>(OscNode<'a>);
impl<'a> TypedNode for MolToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MOL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceToken<'a>(OscNode<'a>);
impl<'a> TypedNode for NamespaceToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMESPACE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NanToken<'a>(OscNode<'a>);
impl<'a> TypedNode for NanToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAN_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotToken<'a>(OscNode<'a>);
impl<'a> TypedNode for NotToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NOT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullToken<'a>(OscNode<'a>);
impl<'a> TypedNode for NullToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NULL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfToken<'a>(OscNode<'a>);
impl<'a> TypedNode for OfToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == OF_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OffsetToken<'a>(OscNode<'a>);
impl<'a> TypedNode for OffsetToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == OFFSET_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnToken<'a>(OscNode<'a>);
impl<'a> TypedNode for OnToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ON_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneOfToken<'a>(OscNode<'a>);
impl<'a> TypedNode for OneOfToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ONE_OF_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnlyToken<'a>(OscNode<'a>);
impl<'a> TypedNode for OnlyToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ONLY_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrToken<'a>(OscNode<'a>);
impl<'a> TypedNode for OrToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == OR_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ParallelToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARALLEL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RadToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RAD_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RangeToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RANGE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RecordToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RECORD_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDefaultToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RemoveDefaultToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == REMOVE_DEFAULT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiseToken<'a>(OscNode<'a>);
impl<'a> TypedNode for RiseToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RISE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SToken<'a>(OscNode<'a>);
impl<'a> TypedNode for SToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == S_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleToken<'a>(OscNode<'a>);
impl<'a> TypedNode for SampleToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SAMPLE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ScenarioToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SCENARIO_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialToken<'a>(OscNode<'a>);
impl<'a> TypedNode for SerialToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SERIAL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiToken<'a>(OscNode<'a>);
impl<'a> TypedNode for SiToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringToken<'a>(OscNode<'a>);
impl<'a> TypedNode for StringToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRING_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructToken<'a>(OscNode<'a>);
impl<'a> TypedNode for StructToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrueToken<'a>(OscNode<'a>);
impl<'a> TypedNode for TrueToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TRUE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeToken<'a>(OscNode<'a>);
impl<'a> TypedNode for TypeToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TYPE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UintToken<'a>(OscNode<'a>);
impl<'a> TypedNode for UintToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UINT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndefinedToken<'a>(OscNode<'a>);
impl<'a> TypedNode for UndefinedToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNDEFINED_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitToken<'a>(OscNode<'a>);
impl<'a> TypedNode for UnitToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNIT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntilToken<'a>(OscNode<'a>);
impl<'a> TypedNode for UntilToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNTIL_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseToken<'a>(OscNode<'a>);
impl<'a> TypedNode for UseToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == USE_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarToken<'a>(OscNode<'a>);
impl<'a> TypedNode for VarToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == VAR_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitToken<'a>(OscNode<'a>);
impl<'a> TypedNode for WaitToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == WAIT_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithToken<'a>(OscNode<'a>);
impl<'a> TypedNode for WithToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == WITH_KW
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteralToken<'a>(OscNode<'a>);
impl<'a> TypedNode for IntegerLiteralToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == INTEGER_LITERAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatLiteralToken<'a>(OscNode<'a>);
impl<'a> TypedNode for FloatLiteralToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FLOAT_LITERAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteralToken<'a>(OscNode<'a>);
impl<'a> TypedNode for StringLiteralToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRING_LITERAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewlineToken<'a>(OscNode<'a>);
impl<'a> TypedNode for NewlineToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NEWLINE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentToken<'a>(OscNode<'a>);
impl<'a> TypedNode for IndentToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == INDENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DedentToken<'a>(OscNode<'a>);
impl<'a> TypedNode for DedentToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DEDENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierToken<'a>(OscNode<'a>);
impl<'a> TypedNode for IdentifierToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IDENTIFIER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorToken<'a>(OscNode<'a>);
impl<'a> TypedNode for ErrorToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ERROR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitespaceToken<'a>(OscNode<'a>);
impl<'a> TypedNode for WhitespaceToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == WHITESPACE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentToken<'a>(OscNode<'a>);
impl<'a> TypedNode for CommentToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COMMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrivialNewlineToken<'a>(OscNode<'a>);
impl<'a> TypedNode for TrivialNewlineToken<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TRIVIAL_NEWLINE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualifiedIdentifier<'a> {
    IdentifierToken(IdentifierToken<'a>),
    PrefixedIdentifier(PrefixedIdentifier<'a>),
}
impl QualifiedIdentifier<'_> {
    pub fn as_identifier_token(&self) -> Option<IdentifierToken> {
        match self {
            Self::IdentifierToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_identifier(&self) -> Option<PrefixedIdentifier> {
        match self {
            Self::PrefixedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for QualifiedIdentifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IDENTIFIER | PREFIXED_IDENTIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER => Some(Self::IdentifierToken(IdentifierToken::cast(node.clone())?)),
            PREFIXED_IDENTIFIER => Some(Self::PrefixedIdentifier(PrefixedIdentifier::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::IdentifierToken(node) => node.syntax(),
            Self::PrefixedIdentifier(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedIdentifier<'a>(OscNode<'a>);
impl PrefixedIdentifier<'_> {
    pub fn namespace_name(&self) -> Option<NamespaceName> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_colon_token(&self) -> Option<ColonColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn identifier_token(&self) -> Option<IdentifierToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PrefixedIdentifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_IDENTIFIER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceName<'a> {
    IdentifierToken(IdentifierToken<'a>),
    NullToken(NullToken<'a>),
}
impl NamespaceName<'_> {
    pub fn as_identifier_token(&self) -> Option<IdentifierToken> {
        match self {
            Self::IdentifierToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_null_token(&self) -> Option<NullToken> {
        match self {
            Self::NullToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for NamespaceName<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IDENTIFIER | NULL_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER => Some(Self::IdentifierToken(IdentifierToken::cast(node.clone())?)),
            NULL_KW => Some(Self::NullToken(NullToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::IdentifierToken(node) => node.syntax(),
            Self::NullToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolLiteral<'a> {
    TrueToken(TrueToken<'a>),
    FalseToken(FalseToken<'a>),
}
impl BoolLiteral<'_> {
    pub fn as_true_token(&self) -> Option<TrueToken> {
        match self {
            Self::TrueToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_false_token(&self) -> Option<FalseToken> {
        match self {
            Self::FalseToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BoolLiteral<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, TRUE_KW | FALSE_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TRUE_KW => Some(Self::TrueToken(TrueToken::cast(node.clone())?)),
            FALSE_KW => Some(Self::FalseToken(FalseToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::TrueToken(node) => node.syntax(),
            Self::FalseToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLiteral<'a>(OscNode<'a>);
impl PhysicalLiteral<'_> {
    pub fn number_literal(&self) -> Option<NumberLiteral> {
        support::child(&self.0, 0usize)
    }
    pub fn unit_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PhysicalLiteral<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PHYSICAL_LITERAL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberLiteral<'a> {
    FloatLiteralToken(FloatLiteralToken<'a>),
    IntegerLiteralToken(IntegerLiteralToken<'a>),
}
impl NumberLiteral<'_> {
    pub fn as_float_literal_token(&self) -> Option<FloatLiteralToken> {
        match self {
            Self::FloatLiteralToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_integer_literal_token(&self) -> Option<IntegerLiteralToken> {
        match self {
            Self::IntegerLiteralToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for NumberLiteral<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, FLOAT_LITERAL | INTEGER_LITERAL)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            FLOAT_LITERAL => Some(Self::FloatLiteralToken(FloatLiteralToken::cast(
                node.clone(),
            )?)),
            INTEGER_LITERAL => Some(Self::IntegerLiteralToken(IntegerLiteralToken::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::FloatLiteralToken(node) => node.syntax(),
            Self::IntegerLiteralToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscFile<'a>(OscNode<'a>);
impl<'a> OscFile<'a> {
    pub fn osc_statement(&self) -> impl Iterator<Item = OscStatement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for OscFile<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == OSC_FILE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscStatement<'a> {
    PreludeStatement(PreludeStatement<'a>),
    MainStatement(MainStatement<'a>),
}
impl OscStatement<'_> {
    pub fn as_prelude_statement(&self) -> Option<PreludeStatement> {
        match self {
            Self::PreludeStatement(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_main_statement(&self) -> Option<MainStatement> {
        match self {
            Self::MainStatement(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for OscStatement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IMPORT_STATEMENT
                | NAMESPACE_STATEMENT
                | EXPORT_STATEMENT
                | PHYSICAL_TYPE_DECLARATION
                | UNIT_DECLARATION
                | ENUM_DECLARATION
                | STRUCT_DECLARATION
                | ACTOR_DECLARATION
                | ACTION_DECLARATION
                | SCENARIO_DECLARATION
                | MODIFIER_DECLARATION
                | TYPE_EXTENSION
                | GLOBAL_PARAMETER_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IMPORT_STATEMENT => Some(Self::PreludeStatement(PreludeStatement::cast(
                node.clone(),
            )?)),
            NAMESPACE_STATEMENT
            | EXPORT_STATEMENT
            | PHYSICAL_TYPE_DECLARATION
            | UNIT_DECLARATION
            | ENUM_DECLARATION
            | STRUCT_DECLARATION
            | ACTOR_DECLARATION
            | ACTION_DECLARATION
            | SCENARIO_DECLARATION
            | MODIFIER_DECLARATION
            | TYPE_EXTENSION
            | GLOBAL_PARAMETER_DECLARATION => {
                Some(Self::MainStatement(MainStatement::cast(node.clone())?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::PreludeStatement(node) => node.syntax(),
            Self::MainStatement(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreludeStatement<'a> {
    ImportStatement(ImportStatement<'a>),
}
impl PreludeStatement<'_> {
    pub fn as_import_statement(&self) -> Option<ImportStatement> {
        match self {
            Self::ImportStatement(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for PreludeStatement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IMPORT_STATEMENT)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IMPORT_STATEMENT => Some(Self::ImportStatement(ImportStatement::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ImportStatement(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainStatement<'a> {
    NamespaceStatement(NamespaceStatement<'a>),
    ExportStatement(ExportStatement<'a>),
    OscDeclaration(OscDeclaration<'a>),
}
impl MainStatement<'_> {
    pub fn as_namespace_statement(&self) -> Option<NamespaceStatement> {
        match self {
            Self::NamespaceStatement(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_export_statement(&self) -> Option<ExportStatement> {
        match self {
            Self::ExportStatement(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_osc_declaration(&self) -> Option<OscDeclaration> {
        match self {
            Self::OscDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for MainStatement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            NAMESPACE_STATEMENT
                | EXPORT_STATEMENT
                | PHYSICAL_TYPE_DECLARATION
                | UNIT_DECLARATION
                | ENUM_DECLARATION
                | STRUCT_DECLARATION
                | ACTOR_DECLARATION
                | ACTION_DECLARATION
                | SCENARIO_DECLARATION
                | MODIFIER_DECLARATION
                | TYPE_EXTENSION
                | GLOBAL_PARAMETER_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            NAMESPACE_STATEMENT => Some(Self::NamespaceStatement(NamespaceStatement::cast(
                node.clone(),
            )?)),
            EXPORT_STATEMENT => Some(Self::ExportStatement(ExportStatement::cast(node.clone())?)),
            PHYSICAL_TYPE_DECLARATION
            | UNIT_DECLARATION
            | ENUM_DECLARATION
            | STRUCT_DECLARATION
            | ACTOR_DECLARATION
            | ACTION_DECLARATION
            | SCENARIO_DECLARATION
            | MODIFIER_DECLARATION
            | TYPE_EXTENSION
            | GLOBAL_PARAMETER_DECLARATION => {
                Some(Self::OscDeclaration(OscDeclaration::cast(node.clone())?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::NamespaceStatement(node) => node.syntax(),
            Self::ExportStatement(node) => node.syntax(),
            Self::OscDeclaration(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportStatement<'a>(OscNode<'a>);
impl ImportStatement<'_> {
    pub fn import_token(&self) -> Option<ImportToken> {
        support::child(&self.0, 0usize)
    }
    pub fn import_reference(&self) -> Option<ImportReference> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ImportStatement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == IMPORT_STATEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportReference<'a> {
    StringLiteralToken(StringLiteralToken<'a>),
    StructuredIdentifier(StructuredIdentifier<'a>),
}
impl ImportReference<'_> {
    pub fn as_string_literal_token(&self) -> Option<StringLiteralToken> {
        match self {
            Self::StringLiteralToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_structured_identifier(&self) -> Option<StructuredIdentifier> {
        match self {
            Self::StructuredIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ImportReference<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            STRING_LITERAL | IDENTIFIER | PREFIXED_STRUCTURED_IDENTIFIER
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            STRING_LITERAL => Some(Self::StringLiteralToken(StringLiteralToken::cast(
                node.clone(),
            )?)),
            IDENTIFIER | PREFIXED_STRUCTURED_IDENTIFIER => Some(Self::StructuredIdentifier(
                StructuredIdentifier::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::StringLiteralToken(node) => node.syntax(),
            Self::StructuredIdentifier(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuredIdentifier<'a> {
    IdentifierToken(IdentifierToken<'a>),
    PrefixedStructuredIdentifier(PrefixedStructuredIdentifier<'a>),
}
impl StructuredIdentifier<'_> {
    pub fn as_identifier_token(&self) -> Option<IdentifierToken> {
        match self {
            Self::IdentifierToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_structured_identifier(&self) -> Option<PrefixedStructuredIdentifier> {
        match self {
            Self::PrefixedStructuredIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for StructuredIdentifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IDENTIFIER | PREFIXED_STRUCTURED_IDENTIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER => Some(Self::IdentifierToken(IdentifierToken::cast(node.clone())?)),
            PREFIXED_STRUCTURED_IDENTIFIER => Some(Self::PrefixedStructuredIdentifier(
                PrefixedStructuredIdentifier::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::IdentifierToken(node) => node.syntax(),
            Self::PrefixedStructuredIdentifier(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedStructuredIdentifier<'a>(OscNode<'a>);
impl PrefixedStructuredIdentifier<'_> {
    pub fn structured_identifier(&self) -> Option<StructuredIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn identifier_token(&self) -> Option<IdentifierToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PrefixedStructuredIdentifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_STRUCTURED_IDENTIFIER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceStatement<'a>(OscNode<'a>);
impl NamespaceStatement<'_> {
    pub fn namespace_token(&self) -> Option<NamespaceToken> {
        support::child(&self.0, 0usize)
    }
    pub fn namespace_name(&self) -> Option<NamespaceName> {
        support::child(&self.0, 0usize)
    }
    pub fn namespace_use_clause(&self) -> Option<NamespaceUseClause> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for NamespaceStatement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMESPACE_STATEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportStatement<'a>(OscNode<'a>);
impl ExportStatement<'_> {
    pub fn export_token(&self) -> Option<ExportToken> {
        support::child(&self.0, 0usize)
    }
    pub fn export_specification_list(&self) -> Option<ExportSpecificationList> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ExportStatement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPORT_STATEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscDeclaration<'a> {
    PhysicalTypeDeclaration(PhysicalTypeDeclaration<'a>),
    UnitDeclaration(UnitDeclaration<'a>),
    EnumDeclaration(EnumDeclaration<'a>),
    StructDeclaration(StructDeclaration<'a>),
    ActorDeclaration(ActorDeclaration<'a>),
    ActionDeclaration(ActionDeclaration<'a>),
    ScenarioDeclaration(ScenarioDeclaration<'a>),
    ModifierDeclaration(ModifierDeclaration<'a>),
    TypeExtension(TypeExtension<'a>),
    GlobalParameterDeclaration(GlobalParameterDeclaration<'a>),
}
impl OscDeclaration<'_> {
    pub fn as_physical_type_declaration(&self) -> Option<PhysicalTypeDeclaration> {
        match self {
            Self::PhysicalTypeDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_unit_declaration(&self) -> Option<UnitDeclaration> {
        match self {
            Self::UnitDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_enum_declaration(&self) -> Option<EnumDeclaration> {
        match self {
            Self::EnumDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_struct_declaration(&self) -> Option<StructDeclaration> {
        match self {
            Self::StructDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_actor_declaration(&self) -> Option<ActorDeclaration> {
        match self {
            Self::ActorDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_action_declaration(&self) -> Option<ActionDeclaration> {
        match self {
            Self::ActionDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_scenario_declaration(&self) -> Option<ScenarioDeclaration> {
        match self {
            Self::ScenarioDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_modifier_declaration(&self) -> Option<ModifierDeclaration> {
        match self {
            Self::ModifierDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_type_extension(&self) -> Option<TypeExtension> {
        match self {
            Self::TypeExtension(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_global_parameter_declaration(&self) -> Option<GlobalParameterDeclaration> {
        match self {
            Self::GlobalParameterDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for OscDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            PHYSICAL_TYPE_DECLARATION
                | UNIT_DECLARATION
                | ENUM_DECLARATION
                | STRUCT_DECLARATION
                | ACTOR_DECLARATION
                | ACTION_DECLARATION
                | SCENARIO_DECLARATION
                | MODIFIER_DECLARATION
                | TYPE_EXTENSION
                | GLOBAL_PARAMETER_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PHYSICAL_TYPE_DECLARATION => Some(Self::PhysicalTypeDeclaration(
                PhysicalTypeDeclaration::cast(node.clone())?,
            )),
            UNIT_DECLARATION => Some(Self::UnitDeclaration(UnitDeclaration::cast(node.clone())?)),
            ENUM_DECLARATION => Some(Self::EnumDeclaration(EnumDeclaration::cast(node.clone())?)),
            STRUCT_DECLARATION => Some(Self::StructDeclaration(StructDeclaration::cast(
                node.clone(),
            )?)),
            ACTOR_DECLARATION => Some(Self::ActorDeclaration(ActorDeclaration::cast(
                node.clone(),
            )?)),
            ACTION_DECLARATION => Some(Self::ActionDeclaration(ActionDeclaration::cast(
                node.clone(),
            )?)),
            SCENARIO_DECLARATION => Some(Self::ScenarioDeclaration(ScenarioDeclaration::cast(
                node.clone(),
            )?)),
            MODIFIER_DECLARATION => Some(Self::ModifierDeclaration(ModifierDeclaration::cast(
                node.clone(),
            )?)),
            TYPE_EXTENSION => Some(Self::TypeExtension(TypeExtension::cast(node.clone())?)),
            GLOBAL_PARAMETER_DECLARATION => Some(Self::GlobalParameterDeclaration(
                GlobalParameterDeclaration::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::PhysicalTypeDeclaration(node) => node.syntax(),
            Self::UnitDeclaration(node) => node.syntax(),
            Self::EnumDeclaration(node) => node.syntax(),
            Self::StructDeclaration(node) => node.syntax(),
            Self::ActorDeclaration(node) => node.syntax(),
            Self::ActionDeclaration(node) => node.syntax(),
            Self::ScenarioDeclaration(node) => node.syntax(),
            Self::ModifierDeclaration(node) => node.syntax(),
            Self::TypeExtension(node) => node.syntax(),
            Self::GlobalParameterDeclaration(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceUseClause<'a>(OscNode<'a>);
impl NamespaceUseClause<'_> {
    pub fn use_token(&self) -> Option<UseToken> {
        support::child(&self.0, 0usize)
    }
    pub fn namespace_list(&self) -> Option<NamespaceList> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for NamespaceUseClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMESPACE_USE_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceList<'a>(OscNode<'a>);
impl<'a> NamespaceList<'a> {
    pub fn namespace_list_element(&self) -> impl Iterator<Item = NamespaceListElement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for NamespaceList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMESPACE_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceListElement<'a>(OscNode<'a>);
impl NamespaceListElement<'_> {
    pub fn namespace_name(&self) -> Option<NamespaceName> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for NamespaceListElement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMESPACE_LIST_ELEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSpecificationList<'a>(OscNode<'a>);
impl<'a> ExportSpecificationList<'a> {
    pub fn export_specification_list_element(
        &self,
    ) -> impl Iterator<Item = ExportSpecificationListElement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ExportSpecificationList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPORT_SPECIFICATION_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSpecificationListElement<'a>(OscNode<'a>);
impl ExportSpecificationListElement<'_> {
    pub fn export_specification(&self) -> Option<ExportSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ExportSpecificationListElement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPORT_SPECIFICATION_LIST_ELEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSpecification<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    ExportWildcardSpecification(ExportWildcardSpecification<'a>),
}
impl ExportSpecification<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_export_wildcard_specification(&self) -> Option<ExportWildcardSpecification> {
        match self {
            Self::ExportWildcardSpecification(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ExportSpecification<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | EXPORT_WILDCARD_SPECIFICATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => Some(Self::QualifiedIdentifier(
                QualifiedIdentifier::cast(node.clone())?,
            )),
            EXPORT_WILDCARD_SPECIFICATION => Some(Self::ExportWildcardSpecification(
                ExportWildcardSpecification::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::ExportWildcardSpecification(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportWildcardSpecification<'a>(OscNode<'a>);
impl ExportWildcardSpecification<'_> {
    pub fn namespace_name(&self) -> Option<NamespaceName> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_colon_token(&self) -> Option<ColonColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn star_token(&self) -> Option<StarToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ExportWildcardSpecification<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPORT_WILDCARD_SPECIFICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTypeDeclaration<'a>(OscNode<'a>);
impl PhysicalTypeDeclaration<'_> {
    pub fn type_token(&self) -> Option<TypeToken> {
        support::child(&self.0, 0usize)
    }
    pub fn physical_type_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn is_token(&self) -> Option<IsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn base_unit_specifier(&self) -> Option<BaseUnitSpecifier> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PhysicalTypeDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PHYSICAL_TYPE_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitDeclaration<'a>(OscNode<'a>);
impl UnitDeclaration<'_> {
    pub fn unit_token(&self) -> Option<UnitToken> {
        support::child(&self.0, 0usize)
    }
    pub fn unit_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn of_token(&self) -> Option<OfToken> {
        support::child(&self.0, 0usize)
    }
    pub fn physical_type_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 1usize)
    }
    pub fn is_token(&self) -> Option<IsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn unit_specifier(&self) -> Option<UnitSpecifier> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for UnitDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNIT_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclaration<'a>(OscNode<'a>);
impl EnumDeclaration<'_> {
    pub fn enum_token(&self) -> Option<EnumToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_decls(&self) -> Option<EnumMemberDecls> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDeclaration<'a>(OscNode<'a>);
impl StructDeclaration<'_> {
    pub fn struct_token(&self) -> Option<StructToken> {
        support::child(&self.0, 0usize)
    }
    pub fn struct_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn struct_inherits_clause(&self) -> Option<StructInheritsClause> {
        support::child(&self.0, 0usize)
    }
    pub fn struct_body_or_newline(&self) -> Option<StructBodyOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCT_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorDeclaration<'a>(OscNode<'a>);
impl ActorDeclaration<'_> {
    pub fn actor_token(&self) -> Option<ActorToken> {
        support::child(&self.0, 0usize)
    }
    pub fn actor_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn actor_inherits_clause(&self) -> Option<ActorInheritsClause> {
        support::child(&self.0, 0usize)
    }
    pub fn actor_body_or_newline(&self) -> Option<ActorBodyOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActorDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTOR_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDeclaration<'a>(OscNode<'a>);
impl ActionDeclaration<'_> {
    pub fn action_token(&self) -> Option<ActionToken> {
        support::child(&self.0, 0usize)
    }
    pub fn action_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
    pub fn action_inherits_clause(&self) -> Option<ActionInheritsClause> {
        support::child(&self.0, 0usize)
    }
    pub fn action_body_or_newline(&self) -> Option<ActionBodyOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActionDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTION_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioDeclaration<'a>(OscNode<'a>);
impl ScenarioDeclaration<'_> {
    pub fn scenario_token(&self) -> Option<ScenarioToken> {
        support::child(&self.0, 0usize)
    }
    pub fn scenario_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
    pub fn scenario_inherits_clause(&self) -> Option<ScenarioInheritsClause> {
        support::child(&self.0, 0usize)
    }
    pub fn scenario_body_or_newline(&self) -> Option<ScenarioBodyOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ScenarioDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SCENARIO_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierDeclaration<'a>(OscNode<'a>);
impl ModifierDeclaration<'_> {
    pub fn modifier_token(&self) -> Option<ModifierToken> {
        support::child(&self.0, 0usize)
    }
    pub fn modifier_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
    pub fn modifier_of_clause(&self) -> Option<ModifierOfClause> {
        support::child(&self.0, 0usize)
    }
    pub fn modifier_body_or_newline(&self) -> Option<ModifierBodyOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MODIFIER_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExtension<'a>(OscNode<'a>);
impl TypeExtension<'_> {
    pub fn extend_token(&self) -> Option<ExtendToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
    pub fn type_extension_body(&self) -> Option<TypeExtensionBody> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for TypeExtension<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TYPE_EXTENSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalParameterDeclaration<'a>(OscNode<'a>);
impl GlobalParameterDeclaration<'_> {
    pub fn global_token(&self) -> Option<GlobalToken> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_declaration(&self) -> Option<ParameterDeclaration> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for GlobalParameterDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == GLOBAL_PARAMETER_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDeclarator<'a> {
    NonAggregateTypeDeclarator(NonAggregateTypeDeclarator<'a>),
    AggregateTypeDeclarator(AggregateTypeDeclarator<'a>),
}
impl TypeDeclarator<'_> {
    pub fn as_non_aggregate_type_declarator(&self) -> Option<NonAggregateTypeDeclarator> {
        match self {
            Self::NonAggregateTypeDeclarator(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_aggregate_type_declarator(&self) -> Option<AggregateTypeDeclarator> {
        match self {
            Self::AggregateTypeDeclarator(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for TypeDeclarator<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            INT_KW
                | UINT_KW
                | FLOAT_KW
                | BOOL_KW
                | STRING_KW
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_BEHAVIOR_NAME
                | LIST_TYPE_DECLARATOR
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            INT_KW
            | UINT_KW
            | FLOAT_KW
            | BOOL_KW
            | STRING_KW
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PREFIXED_BEHAVIOR_NAME => Some(Self::NonAggregateTypeDeclarator(
                NonAggregateTypeDeclarator::cast(node.clone())?,
            )),
            LIST_TYPE_DECLARATOR => Some(Self::AggregateTypeDeclarator(
                AggregateTypeDeclarator::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::NonAggregateTypeDeclarator(node) => node.syntax(),
            Self::AggregateTypeDeclarator(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonAggregateTypeDeclarator<'a> {
    PrimitiveType(PrimitiveType<'a>),
    QualifiedBehaviorName(QualifiedBehaviorName<'a>),
}
impl NonAggregateTypeDeclarator<'_> {
    pub fn as_primitive_type(&self) -> Option<PrimitiveType> {
        match self {
            Self::PrimitiveType(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_qualified_behavior_name(&self) -> Option<QualifiedBehaviorName> {
        match self {
            Self::QualifiedBehaviorName(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for NonAggregateTypeDeclarator<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            INT_KW
                | UINT_KW
                | FLOAT_KW
                | BOOL_KW
                | STRING_KW
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_BEHAVIOR_NAME
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            INT_KW | UINT_KW | FLOAT_KW | BOOL_KW | STRING_KW => {
                Some(Self::PrimitiveType(PrimitiveType::cast(node.clone())?))
            }
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_BEHAVIOR_NAME => Some(
                Self::QualifiedBehaviorName(QualifiedBehaviorName::cast(node.clone())?),
            ),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::PrimitiveType(node) => node.syntax(),
            Self::QualifiedBehaviorName(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregateTypeDeclarator<'a> {
    ListTypeDeclarator(ListTypeDeclarator<'a>),
}
impl AggregateTypeDeclarator<'_> {
    pub fn as_list_type_declarator(&self) -> Option<ListTypeDeclarator> {
        match self {
            Self::ListTypeDeclarator(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for AggregateTypeDeclarator<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, LIST_TYPE_DECLARATOR)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            LIST_TYPE_DECLARATOR => Some(Self::ListTypeDeclarator(ListTypeDeclarator::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ListTypeDeclarator(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveType<'a> {
    IntToken(IntToken<'a>),
    UintToken(UintToken<'a>),
    FloatToken(FloatToken<'a>),
    BoolToken(BoolToken<'a>),
    StringToken(StringToken<'a>),
}
impl PrimitiveType<'_> {
    pub fn as_int_token(&self) -> Option<IntToken> {
        match self {
            Self::IntToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_uint_token(&self) -> Option<UintToken> {
        match self {
            Self::UintToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_float_token(&self) -> Option<FloatToken> {
        match self {
            Self::FloatToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_bool_token(&self) -> Option<BoolToken> {
        match self {
            Self::BoolToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_string_token(&self) -> Option<StringToken> {
        match self {
            Self::StringToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for PrimitiveType<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, INT_KW | UINT_KW | FLOAT_KW | BOOL_KW | STRING_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            INT_KW => Some(Self::IntToken(IntToken::cast(node.clone())?)),
            UINT_KW => Some(Self::UintToken(UintToken::cast(node.clone())?)),
            FLOAT_KW => Some(Self::FloatToken(FloatToken::cast(node.clone())?)),
            BOOL_KW => Some(Self::BoolToken(BoolToken::cast(node.clone())?)),
            STRING_KW => Some(Self::StringToken(StringToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::IntToken(node) => node.syntax(),
            Self::UintToken(node) => node.syntax(),
            Self::FloatToken(node) => node.syntax(),
            Self::BoolToken(node) => node.syntax(),
            Self::StringToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QualifiedBehaviorName<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    PrefixedBehaviorName(PrefixedBehaviorName<'a>),
}
impl QualifiedBehaviorName<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_behavior_name(&self) -> Option<PrefixedBehaviorName> {
        match self {
            Self::PrefixedBehaviorName(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for QualifiedBehaviorName<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_BEHAVIOR_NAME
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => Some(Self::QualifiedIdentifier(
                QualifiedIdentifier::cast(node.clone())?,
            )),
            PREFIXED_BEHAVIOR_NAME => Some(Self::PrefixedBehaviorName(PrefixedBehaviorName::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::PrefixedBehaviorName(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTypeDeclarator<'a>(OscNode<'a>);
impl ListTypeDeclarator<'_> {
    pub fn list_token(&self) -> Option<ListToken> {
        support::child(&self.0, 0usize)
    }
    pub fn of_token(&self) -> Option<OfToken> {
        support::child(&self.0, 0usize)
    }
    pub fn non_aggregate_type_declarator(&self) -> Option<NonAggregateTypeDeclarator> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ListTypeDeclarator<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LIST_TYPE_DECLARATOR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseUnitSpecifier<'a> {
    SiBaseUnitSpecifier(SiBaseUnitSpecifier<'a>),
}
impl BaseUnitSpecifier<'_> {
    pub fn as_si_base_unit_specifier(&self) -> Option<SiBaseUnitSpecifier> {
        match self {
            Self::SiBaseUnitSpecifier(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BaseUnitSpecifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SI_BASE_UNIT_SPECIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SI_BASE_UNIT_SPECIFIER => Some(Self::SiBaseUnitSpecifier(SiBaseUnitSpecifier::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::SiBaseUnitSpecifier(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitSpecifier<'a> {
    SiUnitSpecifier(SiUnitSpecifier<'a>),
}
impl UnitSpecifier<'_> {
    pub fn as_si_unit_specifier(&self) -> Option<SiUnitSpecifier> {
        match self {
            Self::SiUnitSpecifier(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for UnitSpecifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SI_UNIT_SPECIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SI_UNIT_SPECIFIER => Some(Self::SiUnitSpecifier(SiUnitSpecifier::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::SiUnitSpecifier(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseUnitSpecifier<'a>(OscNode<'a>);
impl SiBaseUnitSpecifier<'_> {
    pub fn si_token(&self) -> Option<SiToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn si_base_exponent_list(&self) -> Option<SiBaseExponentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SiBaseUnitSpecifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_BASE_UNIT_SPECIFIER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiUnitSpecifier<'a>(OscNode<'a>);
impl SiUnitSpecifier<'_> {
    pub fn si_token(&self) -> Option<SiToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn si_unit_argument_list(&self) -> Option<SiUnitArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SiUnitSpecifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_UNIT_SPECIFIER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseExponentList<'a>(OscNode<'a>);
impl<'a> SiBaseExponentList<'a> {
    pub fn si_base_exponent(&self) -> impl Iterator<Item = SiBaseExponent<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for SiBaseExponentList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_BASE_EXPONENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseExponent<'a>(OscNode<'a>);
impl SiBaseExponent<'_> {
    pub fn si_base_unit_name(&self) -> Option<SiBaseUnitName> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SiBaseExponent<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_BASE_EXPONENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiBaseUnitName<'a> {
    KgToken(KgToken<'a>),
    MToken(MToken<'a>),
    SToken(SToken<'a>),
    AToken(AToken<'a>),
    KToken(KToken<'a>),
    MolToken(MolToken<'a>),
    CdToken(CdToken<'a>),
    RadToken(RadToken<'a>),
}
impl SiBaseUnitName<'_> {
    pub fn as_kg_token(&self) -> Option<KgToken> {
        match self {
            Self::KgToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_m_token(&self) -> Option<MToken> {
        match self {
            Self::MToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_s_token(&self) -> Option<SToken> {
        match self {
            Self::SToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_a_token(&self) -> Option<AToken> {
        match self {
            Self::AToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_k_token(&self) -> Option<KToken> {
        match self {
            Self::KToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_mol_token(&self) -> Option<MolToken> {
        match self {
            Self::MolToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_cd_token(&self) -> Option<CdToken> {
        match self {
            Self::CdToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_rad_token(&self) -> Option<RadToken> {
        match self {
            Self::RadToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for SiBaseUnitName<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KG_KW | M_KW | S_KW | A_KW | K_KW | MOL_KW | CD_KW | RAD_KW
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KG_KW => Some(Self::KgToken(KgToken::cast(node.clone())?)),
            M_KW => Some(Self::MToken(MToken::cast(node.clone())?)),
            S_KW => Some(Self::SToken(SToken::cast(node.clone())?)),
            A_KW => Some(Self::AToken(AToken::cast(node.clone())?)),
            K_KW => Some(Self::KToken(KToken::cast(node.clone())?)),
            MOL_KW => Some(Self::MolToken(MolToken::cast(node.clone())?)),
            CD_KW => Some(Self::CdToken(CdToken::cast(node.clone())?)),
            RAD_KW => Some(Self::RadToken(RadToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::KgToken(node) => node.syntax(),
            Self::MToken(node) => node.syntax(),
            Self::SToken(node) => node.syntax(),
            Self::AToken(node) => node.syntax(),
            Self::KToken(node) => node.syntax(),
            Self::MolToken(node) => node.syntax(),
            Self::CdToken(node) => node.syntax(),
            Self::RadToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'a> {
    TernaryExp(TernaryExp<'a>),
    LogicalExp(LogicalExp<'a>),
    BinaryExp(BinaryExp<'a>),
    UnaryExp(UnaryExp<'a>),
    CastExp(CastExp<'a>),
    TypeTestExp(TypeTestExp<'a>),
    ElementAccess(ElementAccess<'a>),
    FunctionApplication(FunctionApplication<'a>),
    MemberReference(MemberReference<'a>),
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    ItToken(ItToken<'a>),
    ParenthesizedExp(ParenthesizedExp<'a>),
    LiteralExp(LiteralExp<'a>),
    EnumValueReference(EnumValueReference<'a>),
    ListConstructor(ListConstructor<'a>),
    RangeConstructor(RangeConstructor<'a>),
}
impl Expression<'_> {
    pub fn as_ternary_exp(&self) -> Option<TernaryExp> {
        match self {
            Self::TernaryExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_logical_exp(&self) -> Option<LogicalExp> {
        match self {
            Self::LogicalExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_binary_exp(&self) -> Option<BinaryExp> {
        match self {
            Self::BinaryExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_unary_exp(&self) -> Option<UnaryExp> {
        match self {
            Self::UnaryExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_cast_exp(&self) -> Option<CastExp> {
        match self {
            Self::CastExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_type_test_exp(&self) -> Option<TypeTestExp> {
        match self {
            Self::TypeTestExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_element_access(&self) -> Option<ElementAccess> {
        match self {
            Self::ElementAccess(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_function_application(&self) -> Option<FunctionApplication> {
        match self {
            Self::FunctionApplication(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_member_reference(&self) -> Option<MemberReference> {
        match self {
            Self::MemberReference(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_it_token(&self) -> Option<ItToken> {
        match self {
            Self::ItToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_parenthesized_exp(&self) -> Option<ParenthesizedExp> {
        match self {
            Self::ParenthesizedExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_literal_exp(&self) -> Option<LiteralExp> {
        match self {
            Self::LiteralExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_enum_value_reference(&self) -> Option<EnumValueReference> {
        match self {
            Self::EnumValueReference(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_list_constructor(&self) -> Option<ListConstructor> {
        match self {
            Self::ListConstructor(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_range_constructor(&self) -> Option<RangeConstructor> {
        match self {
            Self::RangeConstructor(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for Expression<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            TERNARY_EXP
                | LOGICAL_EXP
                | BINARY_EXP
                | UNARY_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | MEMBER_REFERENCE
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | IT_KW
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | PARENTHESES_RANGE_CONSTRUCTOR
                | BRACKETS_RANGE_CONSTRUCTOR
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TERNARY_EXP => Some(Self::TernaryExp(TernaryExp::cast(node.clone())?)),
            LOGICAL_EXP => Some(Self::LogicalExp(LogicalExp::cast(node.clone())?)),
            BINARY_EXP => Some(Self::BinaryExp(BinaryExp::cast(node.clone())?)),
            UNARY_EXP => Some(Self::UnaryExp(UnaryExp::cast(node.clone())?)),
            CAST_EXP => Some(Self::CastExp(CastExp::cast(node.clone())?)),
            TYPE_TEST_EXP => Some(Self::TypeTestExp(TypeTestExp::cast(node.clone())?)),
            ELEMENT_ACCESS => Some(Self::ElementAccess(ElementAccess::cast(node.clone())?)),
            FUNCTION_APPLICATION => Some(Self::FunctionApplication(FunctionApplication::cast(
                node.clone(),
            )?)),
            MEMBER_REFERENCE => Some(Self::MemberReference(MemberReference::cast(node.clone())?)),
            IDENTIFIER | PREFIXED_IDENTIFIER => Some(Self::QualifiedIdentifier(
                QualifiedIdentifier::cast(node.clone())?,
            )),
            IT_KW => Some(Self::ItToken(ItToken::cast(node.clone())?)),
            PARENTHESIZED_EXP => Some(Self::ParenthesizedExp(ParenthesizedExp::cast(
                node.clone(),
            )?)),
            INTEGER_LITERAL | FLOAT_LITERAL | PHYSICAL_LITERAL | TRUE_KW | FALSE_KW
            | STRING_LITERAL => Some(Self::LiteralExp(LiteralExp::cast(node.clone())?)),
            ENUM_VALUE_REFERENCE => Some(Self::EnumValueReference(EnumValueReference::cast(
                node.clone(),
            )?)),
            LIST_CONSTRUCTOR => Some(Self::ListConstructor(ListConstructor::cast(node.clone())?)),
            PARENTHESES_RANGE_CONSTRUCTOR | BRACKETS_RANGE_CONSTRUCTOR => Some(
                Self::RangeConstructor(RangeConstructor::cast(node.clone())?),
            ),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::TernaryExp(node) => node.syntax(),
            Self::LogicalExp(node) => node.syntax(),
            Self::BinaryExp(node) => node.syntax(),
            Self::UnaryExp(node) => node.syntax(),
            Self::CastExp(node) => node.syntax(),
            Self::TypeTestExp(node) => node.syntax(),
            Self::ElementAccess(node) => node.syntax(),
            Self::FunctionApplication(node) => node.syntax(),
            Self::MemberReference(node) => node.syntax(),
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::ItToken(node) => node.syntax(),
            Self::ParenthesizedExp(node) => node.syntax(),
            Self::LiteralExp(node) => node.syntax(),
            Self::EnumValueReference(node) => node.syntax(),
            Self::ListConstructor(node) => node.syntax(),
            Self::RangeConstructor(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiUnitArgumentList<'a>(OscNode<'a>);
impl<'a> SiUnitArgumentList<'a> {
    pub fn si_unit_argument(&self) -> impl Iterator<Item = SiUnitArgument<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for SiUnitArgumentList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_UNIT_ARGUMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiUnitArgument<'a>(OscNode<'a>);
impl SiUnitArgument<'_> {
    pub fn si_unit_argument_name(&self) -> Option<SiUnitArgumentName> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SiUnitArgument<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SI_UNIT_ARGUMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiUnitArgumentName<'a> {
    FactorToken(FactorToken<'a>),
    OffsetToken(OffsetToken<'a>),
    SiBaseUnitName(SiBaseUnitName<'a>),
}
impl SiUnitArgumentName<'_> {
    pub fn as_factor_token(&self) -> Option<FactorToken> {
        match self {
            Self::FactorToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_offset_token(&self) -> Option<OffsetToken> {
        match self {
            Self::OffsetToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_si_base_unit_name(&self) -> Option<SiBaseUnitName> {
        match self {
            Self::SiBaseUnitName(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for SiUnitArgumentName<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            FACTOR_KW | OFFSET_KW | KG_KW | M_KW | S_KW | A_KW | K_KW | MOL_KW | CD_KW | RAD_KW
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            FACTOR_KW => Some(Self::FactorToken(FactorToken::cast(node.clone())?)),
            OFFSET_KW => Some(Self::OffsetToken(OffsetToken::cast(node.clone())?)),
            KG_KW | M_KW | S_KW | A_KW | K_KW | MOL_KW | CD_KW | RAD_KW => {
                Some(Self::SiBaseUnitName(SiBaseUnitName::cast(node.clone())?))
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::FactorToken(node) => node.syntax(),
            Self::OffsetToken(node) => node.syntax(),
            Self::SiBaseUnitName(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDecls<'a>(OscNode<'a>);
impl EnumMemberDecls<'_> {
    pub fn left_bracket_token(&self) -> Option<LeftBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_decl_list(&self) -> Option<EnumMemberDeclList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_bracket_token(&self) -> Option<RightBracketToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumMemberDecls<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_MEMBER_DECLS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDeclList<'a>(OscNode<'a>);
impl<'a> EnumMemberDeclList<'a> {
    pub fn enum_member_decl(&self) -> impl Iterator<Item = EnumMemberDecl<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for EnumMemberDeclList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_MEMBER_DECL_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDecl<'a>(OscNode<'a>);
impl EnumMemberDecl<'_> {
    pub fn enum_member_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_initializer_clause(&self) -> Option<EnumInitializerClause> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumMemberDecl<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_MEMBER_DECL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumInitializerClause<'a>(OscNode<'a>);
impl EnumInitializerClause<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumInitializerClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_INITIALIZER_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsClause<'a>(OscNode<'a>);
impl StructInheritsClause<'_> {
    pub fn inherits_token(&self) -> Option<InheritsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn base_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn struct_inherits_condition(&self) -> Option<StructInheritsCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructInheritsClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCT_INHERITS_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructBodyOrNewline<'a> {
    StructBody(StructBody<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl StructBodyOrNewline<'_> {
    pub fn as_struct_body(&self) -> Option<StructBody> {
        match self {
            Self::StructBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for StructBodyOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, STRUCT_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            STRUCT_BODY => Some(Self::StructBody(StructBody::cast(node.clone())?)),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::StructBody(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsCondition<'a>(OscNode<'a>);
impl StructInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructInheritsCondition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCT_INHERITS_CONDITION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructBody<'a>(OscNode<'a>);
impl StructBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_type_member_list(&self) -> Option<StructuredTypeMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCT_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredTypeMemberList<'a>(OscNode<'a>);
impl<'a> StructuredTypeMemberList<'a> {
    pub fn structured_type_member(&self) -> impl Iterator<Item = StructuredTypeMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for StructuredTypeMemberList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCTURED_TYPE_MEMBER_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsClause<'a>(OscNode<'a>);
impl ActorInheritsClause<'_> {
    pub fn inherits_token(&self) -> Option<InheritsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn base_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn actor_inherits_condition(&self) -> Option<ActorInheritsCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActorInheritsClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTOR_INHERITS_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorBodyOrNewline<'a> {
    ActorBody(ActorBody<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl ActorBodyOrNewline<'_> {
    pub fn as_actor_body(&self) -> Option<ActorBody> {
        match self {
            Self::ActorBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ActorBodyOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ACTOR_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ACTOR_BODY => Some(Self::ActorBody(ActorBody::cast(node.clone())?)),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ActorBody(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsCondition<'a>(OscNode<'a>);
impl ActorInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActorInheritsCondition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTOR_INHERITS_CONDITION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorBody<'a>(OscNode<'a>);
impl ActorBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_type_member_list(&self) -> Option<StructuredTypeMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActorBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTOR_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsClause<'a>(OscNode<'a>);
impl ScenarioInheritsClause<'_> {
    pub fn inherits_token(&self) -> Option<InheritsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn base_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
    pub fn scenario_inherits_condition(&self) -> Option<ScenarioInheritsCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ScenarioInheritsClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SCENARIO_INHERITS_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioBodyOrNewline<'a> {
    ScenarioBody(ScenarioBody<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl ScenarioBodyOrNewline<'_> {
    pub fn as_scenario_body(&self) -> Option<ScenarioBody> {
        match self {
            Self::ScenarioBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ScenarioBodyOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SCENARIO_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SCENARIO_BODY => Some(Self::ScenarioBody(ScenarioBody::cast(node.clone())?)),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ScenarioBody(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsCondition<'a>(OscNode<'a>);
impl ScenarioInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ScenarioInheritsCondition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SCENARIO_INHERITS_CONDITION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBody<'a>(OscNode<'a>);
impl ScenarioBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_type_member_list(&self) -> Option<StructuredTypeMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ScenarioBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SCENARIO_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedBehaviorName<'a>(OscNode<'a>);
impl PrefixedBehaviorName<'_> {
    pub fn actor_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn behavior_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 1usize)
    }
}
impl<'a> TypedNode for PrefixedBehaviorName<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_BEHAVIOR_NAME
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsClause<'a>(OscNode<'a>);
impl ActionInheritsClause<'_> {
    pub fn inherits_token(&self) -> Option<InheritsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn base_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
    pub fn action_inherits_condition(&self) -> Option<ActionInheritsCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActionInheritsClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTION_INHERITS_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionBodyOrNewline<'a> {
    ActionBody(ActionBody<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl ActionBodyOrNewline<'_> {
    pub fn as_action_body(&self) -> Option<ActionBody> {
        match self {
            Self::ActionBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ActionBodyOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ACTION_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ACTION_BODY => Some(Self::ActionBody(ActionBody::cast(node.clone())?)),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ActionBody(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBody<'a>(OscNode<'a>);
impl ActionBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_type_member_list(&self) -> Option<StructuredTypeMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActionBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTION_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsCondition<'a>(OscNode<'a>);
impl ActionInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActionInheritsCondition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTION_INHERITS_CONDITION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierOfClause<'a>(OscNode<'a>);
impl ModifierOfClause<'_> {
    pub fn of_token(&self) -> Option<OfToken> {
        support::child(&self.0, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierOfClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MODIFIER_OF_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierBodyOrNewline<'a> {
    ModifierBody(ModifierBody<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl ModifierBodyOrNewline<'_> {
    pub fn as_modifier_body(&self) -> Option<ModifierBody> {
        match self {
            Self::ModifierBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ModifierBodyOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, MODIFIER_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            MODIFIER_BODY => Some(Self::ModifierBody(ModifierBody::cast(node.clone())?)),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ModifierBody(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierBody<'a>(OscNode<'a>);
impl ModifierBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_type_member_list(&self) -> Option<StructuredTypeMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MODIFIER_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExtensionBody<'a> {
    EnumTypeExtensionBody(EnumTypeExtensionBody<'a>),
    StructuredTypeExtensionBody(StructuredTypeExtensionBody<'a>),
}
impl TypeExtensionBody<'_> {
    pub fn as_enum_type_extension_body(&self) -> Option<EnumTypeExtensionBody> {
        match self {
            Self::EnumTypeExtensionBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_structured_type_extension_body(&self) -> Option<StructuredTypeExtensionBody> {
        match self {
            Self::StructuredTypeExtensionBody(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for TypeExtensionBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            ENUM_TYPE_EXTENSION_BODY | STRUCTURED_TYPE_EXTENSION_BODY
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ENUM_TYPE_EXTENSION_BODY => Some(Self::EnumTypeExtensionBody(
                EnumTypeExtensionBody::cast(node.clone())?,
            )),
            STRUCTURED_TYPE_EXTENSION_BODY => Some(Self::StructuredTypeExtensionBody(
                StructuredTypeExtensionBody::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::EnumTypeExtensionBody(node) => node.syntax(),
            Self::StructuredTypeExtensionBody(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumTypeExtensionBody<'a>(OscNode<'a>);
impl EnumTypeExtensionBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_decls(&self) -> Option<EnumMemberDecls> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumTypeExtensionBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_TYPE_EXTENSION_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredTypeExtensionBody<'a>(OscNode<'a>);
impl StructuredTypeExtensionBody<'_> {
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_type_member_list(&self) -> Option<StructuredTypeMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructuredTypeExtensionBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCTURED_TYPE_EXTENSION_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration<'a>(OscNode<'a>);
impl ParameterDeclaration<'_> {
    pub fn field_name_list(&self) -> Option<FieldNameList> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_declarator(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_initializer_clause(&self) -> Option<ParameterInitializerClause> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_with_declaration_or_newline(
        &self,
    ) -> Option<ParameterWithDeclarationOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParameterDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARAMETER_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuredTypeMember<'a> {
    EventDeclaration(EventDeclaration<'a>),
    FieldDeclaration(FieldDeclaration<'a>),
    ConstraintDeclaration(ConstraintDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    CoverageDeclaration(CoverageDeclaration<'a>),
    ModifierApplication(ModifierApplication<'a>),
    BehaviorSpecification(BehaviorSpecification<'a>),
}
impl StructuredTypeMember<'_> {
    pub fn as_event_declaration(&self) -> Option<EventDeclaration> {
        match self {
            Self::EventDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_field_declaration(&self) -> Option<FieldDeclaration> {
        match self {
            Self::FieldDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_constraint_declaration(&self) -> Option<ConstraintDeclaration> {
        match self {
            Self::ConstraintDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_method_declaration(&self) -> Option<MethodDeclaration> {
        match self {
            Self::MethodDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_coverage_declaration(&self) -> Option<CoverageDeclaration> {
        match self {
            Self::CoverageDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_modifier_application(&self) -> Option<ModifierApplication> {
        match self {
            Self::ModifierApplication(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_behavior_specification(&self) -> Option<BehaviorSpecification> {
        match self {
            Self::BehaviorSpecification(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for StructuredTypeMember<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            EVENT_DECLARATION
                | PARAMETER_DECLARATION
                | VARIABLE_DECLARATION
                | KEEP_CONSTRAINT_DECLARATION
                | REMOVE_DEFAULT_DECLARATION
                | METHOD_DECLARATION
                | COVERAGE_DECLARATION
                | MODIFIER_APPLICATION
                | ON_DIRECTIVE
                | DO_DIRECTIVE
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_DECLARATION => Some(Self::EventDeclaration(EventDeclaration::cast(
                node.clone(),
            )?)),
            PARAMETER_DECLARATION | VARIABLE_DECLARATION => Some(Self::FieldDeclaration(
                FieldDeclaration::cast(node.clone())?,
            )),
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => Some(
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(node.clone())?),
            ),
            METHOD_DECLARATION => Some(Self::MethodDeclaration(MethodDeclaration::cast(
                node.clone(),
            )?)),
            COVERAGE_DECLARATION => Some(Self::CoverageDeclaration(CoverageDeclaration::cast(
                node.clone(),
            )?)),
            MODIFIER_APPLICATION => Some(Self::ModifierApplication(ModifierApplication::cast(
                node.clone(),
            )?)),
            ON_DIRECTIVE | DO_DIRECTIVE => Some(Self::BehaviorSpecification(
                BehaviorSpecification::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::EventDeclaration(node) => node.syntax(),
            Self::FieldDeclaration(node) => node.syntax(),
            Self::ConstraintDeclaration(node) => node.syntax(),
            Self::MethodDeclaration(node) => node.syntax(),
            Self::CoverageDeclaration(node) => node.syntax(),
            Self::ModifierApplication(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventDeclaration<'a>(OscNode<'a>);
impl EventDeclaration<'_> {
    pub fn event_token(&self) -> Option<EventToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_specifications(&self) -> Option<ArgumentSpecifications> {
        support::child(&self.0, 0usize)
    }
    pub fn event_is_clause(&self) -> Option<EventIsClause> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDeclaration<'a> {
    ParameterDeclaration(ParameterDeclaration<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
}
impl FieldDeclaration<'_> {
    pub fn as_parameter_declaration(&self) -> Option<ParameterDeclaration> {
        match self {
            Self::ParameterDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_variable_declaration(&self) -> Option<VariableDeclaration> {
        match self {
            Self::VariableDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for FieldDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, PARAMETER_DECLARATION | VARIABLE_DECLARATION)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PARAMETER_DECLARATION => Some(Self::ParameterDeclaration(ParameterDeclaration::cast(
                node.clone(),
            )?)),
            VARIABLE_DECLARATION => Some(Self::VariableDeclaration(VariableDeclaration::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ParameterDeclaration(node) => node.syntax(),
            Self::VariableDeclaration(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintDeclaration<'a> {
    KeepConstraintDeclaration(KeepConstraintDeclaration<'a>),
    RemoveDefaultDeclaration(RemoveDefaultDeclaration<'a>),
}
impl ConstraintDeclaration<'_> {
    pub fn as_keep_constraint_declaration(&self) -> Option<KeepConstraintDeclaration> {
        match self {
            Self::KeepConstraintDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_remove_default_declaration(&self) -> Option<RemoveDefaultDeclaration> {
        match self {
            Self::RemoveDefaultDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ConstraintDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KEEP_CONSTRAINT_DECLARATION => Some(Self::KeepConstraintDeclaration(
                KeepConstraintDeclaration::cast(node.clone())?,
            )),
            REMOVE_DEFAULT_DECLARATION => Some(Self::RemoveDefaultDeclaration(
                RemoveDefaultDeclaration::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::KeepConstraintDeclaration(node) => node.syntax(),
            Self::RemoveDefaultDeclaration(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDeclaration<'a>(OscNode<'a>);
impl MethodDeclaration<'_> {
    pub fn def_token(&self) -> Option<DefToken> {
        support::child(&self.0, 0usize)
    }
    pub fn method_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_specifications(&self) -> Option<ArgumentSpecifications> {
        support::child(&self.0, 0usize)
    }
    pub fn method_return_type(&self) -> Option<MethodReturnType> {
        support::child(&self.0, 0usize)
    }
    pub fn method_implementation(&self) -> Option<MethodImplementation> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == METHOD_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageDeclaration<'a>(OscNode<'a>);
impl CoverageDeclaration<'_> {
    pub fn coverage_operator(&self) -> Option<CoverageOperator> {
        support::child(&self.0, 0usize)
    }
    pub fn arguments(&self) -> Option<Arguments> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for CoverageDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COVERAGE_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierApplication<'a>(OscNode<'a>);
impl ModifierApplication<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierApplication<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MODIFIER_APPLICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorSpecification<'a> {
    OnDirective(OnDirective<'a>),
    DoDirective(DoDirective<'a>),
}
impl BehaviorSpecification<'_> {
    pub fn as_on_directive(&self) -> Option<OnDirective> {
        match self {
            Self::OnDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_do_directive(&self) -> Option<DoDirective> {
        match self {
            Self::DoDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BehaviorSpecification<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ON_DIRECTIVE | DO_DIRECTIVE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ON_DIRECTIVE => Some(Self::OnDirective(OnDirective::cast(node.clone())?)),
            DO_DIRECTIVE => Some(Self::DoDirective(DoDirective::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::OnDirective(node) => node.syntax(),
            Self::DoDirective(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSpecifications<'a>(OscNode<'a>);
impl ArgumentSpecifications<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_specification_list(&self) -> Option<ArgumentSpecificationList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ArgumentSpecifications<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_SPECIFICATIONS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventIsClause<'a>(OscNode<'a>);
impl EventIsClause<'_> {
    pub fn is_token(&self) -> Option<IsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventIsClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_IS_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventSpecification<'a> {
    EventReferenceSpecification(EventReferenceSpecification<'a>),
    EventCondition(EventCondition<'a>),
}
impl EventSpecification<'_> {
    pub fn as_event_reference_specification(&self) -> Option<EventReferenceSpecification> {
        match self {
            Self::EventReferenceSpecification(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_event_condition(&self) -> Option<EventCondition> {
        match self {
            Self::EventCondition(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for EventSpecification<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            EVENT_REFERENCE_SPECIFICATION
                | TERNARY_EXP
                | LOGICAL_EXP
                | BINARY_EXP
                | UNARY_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | MEMBER_REFERENCE
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | IT_KW
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | PARENTHESES_RANGE_CONSTRUCTOR
                | BRACKETS_RANGE_CONSTRUCTOR
                | RISE_EXPRESSION
                | FALL_EXPRESSION
                | ELAPSED_EXPRESSION
                | EVERY_EXPRESSION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_REFERENCE_SPECIFICATION => Some(Self::EventReferenceSpecification(
                EventReferenceSpecification::cast(node.clone())?,
            )),
            TERNARY_EXP
            | LOGICAL_EXP
            | BINARY_EXP
            | UNARY_EXP
            | CAST_EXP
            | TYPE_TEST_EXP
            | ELEMENT_ACCESS
            | FUNCTION_APPLICATION
            | MEMBER_REFERENCE
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | IT_KW
            | PARENTHESIZED_EXP
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | PHYSICAL_LITERAL
            | TRUE_KW
            | FALSE_KW
            | STRING_LITERAL
            | ENUM_VALUE_REFERENCE
            | LIST_CONSTRUCTOR
            | PARENTHESES_RANGE_CONSTRUCTOR
            | BRACKETS_RANGE_CONSTRUCTOR
            | RISE_EXPRESSION
            | FALL_EXPRESSION
            | ELAPSED_EXPRESSION
            | EVERY_EXPRESSION => Some(Self::EventCondition(EventCondition::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::EventReferenceSpecification(node) => node.syntax(),
            Self::EventCondition(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReferenceSpecification<'a>(OscNode<'a>);
impl EventReferenceSpecification<'_> {
    pub fn event_reference(&self) -> Option<EventReference> {
        support::child(&self.0, 0usize)
    }
    pub fn event_reference_condition(&self) -> Option<EventReferenceCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventReferenceSpecification<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_REFERENCE_SPECIFICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventCondition<'a> {
    Expression(Expression<'a>),
    RiseExpression(RiseExpression<'a>),
    FallExpression(FallExpression<'a>),
    ElapsedExpression(ElapsedExpression<'a>),
    EveryExpression(EveryExpression<'a>),
}
impl EventCondition<'_> {
    pub fn as_expression(&self) -> Option<Expression> {
        match self {
            Self::Expression(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_rise_expression(&self) -> Option<RiseExpression> {
        match self {
            Self::RiseExpression(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_fall_expression(&self) -> Option<FallExpression> {
        match self {
            Self::FallExpression(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_elapsed_expression(&self) -> Option<ElapsedExpression> {
        match self {
            Self::ElapsedExpression(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_every_expression(&self) -> Option<EveryExpression> {
        match self {
            Self::EveryExpression(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for EventCondition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            TERNARY_EXP
                | LOGICAL_EXP
                | BINARY_EXP
                | UNARY_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | MEMBER_REFERENCE
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | IT_KW
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | PARENTHESES_RANGE_CONSTRUCTOR
                | BRACKETS_RANGE_CONSTRUCTOR
                | RISE_EXPRESSION
                | FALL_EXPRESSION
                | ELAPSED_EXPRESSION
                | EVERY_EXPRESSION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TERNARY_EXP
            | LOGICAL_EXP
            | BINARY_EXP
            | UNARY_EXP
            | CAST_EXP
            | TYPE_TEST_EXP
            | ELEMENT_ACCESS
            | FUNCTION_APPLICATION
            | MEMBER_REFERENCE
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | IT_KW
            | PARENTHESIZED_EXP
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | PHYSICAL_LITERAL
            | TRUE_KW
            | FALSE_KW
            | STRING_LITERAL
            | ENUM_VALUE_REFERENCE
            | LIST_CONSTRUCTOR
            | PARENTHESES_RANGE_CONSTRUCTOR
            | BRACKETS_RANGE_CONSTRUCTOR => Some(Self::Expression(Expression::cast(node.clone())?)),
            RISE_EXPRESSION => Some(Self::RiseExpression(RiseExpression::cast(node.clone())?)),
            FALL_EXPRESSION => Some(Self::FallExpression(FallExpression::cast(node.clone())?)),
            ELAPSED_EXPRESSION => Some(Self::ElapsedExpression(ElapsedExpression::cast(
                node.clone(),
            )?)),
            EVERY_EXPRESSION => Some(Self::EveryExpression(EveryExpression::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::Expression(node) => node.syntax(),
            Self::RiseExpression(node) => node.syntax(),
            Self::FallExpression(node) => node.syntax(),
            Self::ElapsedExpression(node) => node.syntax(),
            Self::EveryExpression(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReference<'a>(OscNode<'a>);
impl EventReference<'_> {
    pub fn at_token(&self) -> Option<AtToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventReference<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_REFERENCE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReferenceCondition<'a>(OscNode<'a>);
impl EventReferenceCondition<'_> {
    pub fn event_field_decl(&self) -> Option<EventFieldDecl> {
        support::child(&self.0, 0usize)
    }
    pub fn if_token(&self) -> Option<IfToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_condition(&self) -> Option<EventCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventReferenceCondition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_REFERENCE_CONDITION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFieldDecl<'a>(OscNode<'a>);
impl EventFieldDecl<'_> {
    pub fn as_token(&self) -> Option<AsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventFieldDecl<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_FIELD_DECL
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiseExpression<'a>(OscNode<'a>);
impl RiseExpression<'_> {
    pub fn rise_token(&self) -> Option<RiseToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for RiseExpression<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == RISE_EXPRESSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallExpression<'a>(OscNode<'a>);
impl FallExpression<'_> {
    pub fn fall_token(&self) -> Option<FallToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for FallExpression<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FALL_EXPRESSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElapsedExpression<'a>(OscNode<'a>);
impl ElapsedExpression<'_> {
    pub fn elapsed_token(&self) -> Option<ElapsedToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn duration(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ElapsedExpression<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ELAPSED_EXPRESSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryExpression<'a>(OscNode<'a>);
impl EveryExpression<'_> {
    pub fn every_token(&self) -> Option<EveryToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn duration(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn every_exp_offset(&self) -> Option<EveryExpOffset> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EveryExpression<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVERY_EXPRESSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryExpOffset<'a>(OscNode<'a>);
impl EveryExpOffset<'_> {
    pub fn offset_token(&self) -> Option<OffsetToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn duration(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EveryExpOffset<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVERY_EXP_OFFSET
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration<'a>(OscNode<'a>);
impl VariableDeclaration<'_> {
    pub fn var_token(&self) -> Option<VarToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name_list(&self) -> Option<FieldNameList> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_declarator(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
    pub fn variable_initializer_clause(&self) -> Option<VariableInitializerClause> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for VariableDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == VARIABLE_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldNameList<'a>(OscNode<'a>);
impl<'a> FieldNameList<'a> {
    pub fn field_name_list_element(&self) -> impl Iterator<Item = FieldNameListElement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for FieldNameList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FIELD_NAME_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterInitializerClause<'a>(OscNode<'a>);
impl ParameterInitializerClause<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn default_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParameterInitializerClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARAMETER_INITIALIZER_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterWithDeclarationOrNewline<'a> {
    ParameterWithDeclaration(ParameterWithDeclaration<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl ParameterWithDeclarationOrNewline<'_> {
    pub fn as_parameter_with_declaration(&self) -> Option<ParameterWithDeclaration> {
        match self {
            Self::ParameterWithDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ParameterWithDeclarationOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, PARAMETER_WITH_DECLARATION | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PARAMETER_WITH_DECLARATION => Some(Self::ParameterWithDeclaration(
                ParameterWithDeclaration::cast(node.clone())?,
            )),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ParameterWithDeclaration(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldNameListElement<'a>(OscNode<'a>);
impl FieldNameListElement<'_> {
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for FieldNameListElement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FIELD_NAME_LIST_ELEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableInitializerClause<'a>(OscNode<'a>);
impl VariableInitializerClause<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn variable_default_value(&self) -> Option<VariableDefaultValue> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for VariableInitializerClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == VARIABLE_INITIALIZER_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableDefaultValue<'a> {
    Expression(Expression<'a>),
    SampleExpression(SampleExpression<'a>),
}
impl VariableDefaultValue<'_> {
    pub fn as_expression(&self) -> Option<Expression> {
        match self {
            Self::Expression(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_sample_expression(&self) -> Option<SampleExpression> {
        match self {
            Self::SampleExpression(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for VariableDefaultValue<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            TERNARY_EXP
                | LOGICAL_EXP
                | BINARY_EXP
                | UNARY_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | MEMBER_REFERENCE
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | IT_KW
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | PARENTHESES_RANGE_CONSTRUCTOR
                | BRACKETS_RANGE_CONSTRUCTOR
                | SAMPLE_EXPRESSION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TERNARY_EXP
            | LOGICAL_EXP
            | BINARY_EXP
            | UNARY_EXP
            | CAST_EXP
            | TYPE_TEST_EXP
            | ELEMENT_ACCESS
            | FUNCTION_APPLICATION
            | MEMBER_REFERENCE
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | IT_KW
            | PARENTHESIZED_EXP
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | PHYSICAL_LITERAL
            | TRUE_KW
            | FALSE_KW
            | STRING_LITERAL
            | ENUM_VALUE_REFERENCE
            | LIST_CONSTRUCTOR
            | PARENTHESES_RANGE_CONSTRUCTOR
            | BRACKETS_RANGE_CONSTRUCTOR => Some(Self::Expression(Expression::cast(node.clone())?)),
            SAMPLE_EXPRESSION => Some(Self::SampleExpression(SampleExpression::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::Expression(node) => node.syntax(),
            Self::SampleExpression(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleExpression<'a>(OscNode<'a>);
impl SampleExpression<'_> {
    pub fn sample_token(&self) -> Option<SampleToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn first_comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn second_comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 1usize)
    }
    pub fn default_value(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SampleExpression<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SAMPLE_EXPRESSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithDeclaration<'a>(OscNode<'a>);
impl ParameterWithDeclaration<'_> {
    pub fn with_token(&self) -> Option<WithToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_with_member_list(&self) -> Option<ParameterWithMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParameterWithDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARAMETER_WITH_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithMemberList<'a>(OscNode<'a>);
impl<'a> ParameterWithMemberList<'a> {
    pub fn parameter_with_member(&self) -> impl Iterator<Item = ParameterWithMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ParameterWithMemberList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARAMETER_WITH_MEMBER_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterWithMember<'a> {
    ConstraintDeclaration(ConstraintDeclaration<'a>),
}
impl ParameterWithMember<'_> {
    pub fn as_constraint_declaration(&self) -> Option<ConstraintDeclaration> {
        match self {
            Self::ConstraintDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ParameterWithMember<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => Some(
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(node.clone())?),
            ),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ConstraintDeclaration(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepConstraintDeclaration<'a>(OscNode<'a>);
impl KeepConstraintDeclaration<'_> {
    pub fn keep_token(&self) -> Option<KeepToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn constraint_qualifier(&self) -> Option<ConstraintQualifier> {
        support::child(&self.0, 0usize)
    }
    pub fn constraint(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for KeepConstraintDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == KEEP_CONSTRAINT_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDefaultDeclaration<'a>(OscNode<'a>);
impl RemoveDefaultDeclaration<'_> {
    pub fn remove_default_token(&self) -> Option<RemoveDefaultToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for RemoveDefaultDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == REMOVE_DEFAULT_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintQualifier<'a> {
    DefaultToken(DefaultToken<'a>),
    HardToken(HardToken<'a>),
}
impl ConstraintQualifier<'_> {
    pub fn as_default_token(&self) -> Option<DefaultToken> {
        match self {
            Self::DefaultToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_hard_token(&self) -> Option<HardToken> {
        match self {
            Self::HardToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ConstraintQualifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, DEFAULT_KW | HARD_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            DEFAULT_KW => Some(Self::DefaultToken(DefaultToken::cast(node.clone())?)),
            HARD_KW => Some(Self::HardToken(HardToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::DefaultToken(node) => node.syntax(),
            Self::HardToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodReturnType<'a>(OscNode<'a>);
impl MethodReturnType<'_> {
    pub fn arrow_token(&self) -> Option<ArrowToken> {
        support::child(&self.0, 0usize)
    }
    pub fn return_type(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodReturnType<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == METHOD_RETURN_TYPE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodImplementation<'a>(OscNode<'a>);
impl MethodImplementation<'_> {
    pub fn is_token(&self) -> Option<IsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn method_qualifier(&self) -> Option<MethodQualifier> {
        support::child(&self.0, 0usize)
    }
    pub fn method_body(&self) -> Option<MethodBody> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodImplementation<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == METHOD_IMPLEMENTATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodQualifier<'a> {
    OnlyToken(OnlyToken<'a>),
}
impl MethodQualifier<'_> {
    pub fn as_only_token(&self) -> Option<OnlyToken> {
        match self {
            Self::OnlyToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for MethodQualifier<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ONLY_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ONLY_KW => Some(Self::OnlyToken(OnlyToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::OnlyToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodBody<'a> {
    MethodExpressionBody(MethodExpressionBody<'a>),
    UndefinedToken(UndefinedToken<'a>),
    MethodExternalBody(MethodExternalBody<'a>),
}
impl MethodBody<'_> {
    pub fn as_method_expression_body(&self) -> Option<MethodExpressionBody> {
        match self {
            Self::MethodExpressionBody(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_undefined_token(&self) -> Option<UndefinedToken> {
        match self {
            Self::UndefinedToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_method_external_body(&self) -> Option<MethodExternalBody> {
        match self {
            Self::MethodExternalBody(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for MethodBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            METHOD_EXPRESSION_BODY | UNDEFINED_KW | METHOD_EXTERNAL_BODY
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            METHOD_EXPRESSION_BODY => Some(Self::MethodExpressionBody(MethodExpressionBody::cast(
                node.clone(),
            )?)),
            UNDEFINED_KW => Some(Self::UndefinedToken(UndefinedToken::cast(node.clone())?)),
            METHOD_EXTERNAL_BODY => Some(Self::MethodExternalBody(MethodExternalBody::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::MethodExpressionBody(node) => node.syntax(),
            Self::UndefinedToken(node) => node.syntax(),
            Self::MethodExternalBody(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExpressionBody<'a>(OscNode<'a>);
impl MethodExpressionBody<'_> {
    pub fn expression_token(&self) -> Option<ExpressionToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodExpressionBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == METHOD_EXPRESSION_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExternalBody<'a>(OscNode<'a>);
impl MethodExternalBody<'_> {
    pub fn external_token(&self) -> Option<ExternalToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_identifier(&self) -> Option<StructuredIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn arguments(&self) -> Option<Arguments> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodExternalBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == METHOD_EXTERNAL_BODY
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments<'a>(OscNode<'a>);
impl Arguments<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for Arguments<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENTS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoverageOperator<'a> {
    CoverToken(CoverToken<'a>),
    RecordToken(RecordToken<'a>),
}
impl CoverageOperator<'_> {
    pub fn as_cover_token(&self) -> Option<CoverToken> {
        match self {
            Self::CoverToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_record_token(&self) -> Option<RecordToken> {
        match self {
            Self::RecordToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for CoverageOperator<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, COVER_KW | RECORD_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            COVER_KW => Some(Self::CoverToken(CoverToken::cast(node.clone())?)),
            RECORD_KW => Some(Self::RecordToken(RecordToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::CoverToken(node) => node.syntax(),
            Self::RecordToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnDirective<'a>(OscNode<'a>);
impl OnDirective<'_> {
    pub fn on_token(&self) -> Option<OnToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn on_member_list(&self) -> Option<OnMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for OnDirective<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ON_DIRECTIVE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoDirective<'a>(OscNode<'a>);
impl DoDirective<'_> {
    pub fn do_token(&self) -> Option<DoToken> {
        support::child(&self.0, 0usize)
    }
    pub fn do_member(&self) -> Option<DoMember> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for DoDirective<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DO_DIRECTIVE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnMemberList<'a>(OscNode<'a>);
impl<'a> OnMemberList<'a> {
    pub fn on_member(&self) -> impl Iterator<Item = OnMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for OnMemberList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ON_MEMBER_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnMember<'a> {
    CallDirective(CallDirective<'a>),
    EmitDirective(EmitDirective<'a>),
}
impl OnMember<'_> {
    pub fn as_call_directive(&self) -> Option<CallDirective> {
        match self {
            Self::CallDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_emit_directive(&self) -> Option<EmitDirective> {
        match self {
            Self::EmitDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for OnMember<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, CALL_DIRECTIVE | EMIT_DIRECTIVE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            CALL_DIRECTIVE => Some(Self::CallDirective(CallDirective::cast(node.clone())?)),
            EMIT_DIRECTIVE => Some(Self::EmitDirective(EmitDirective::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::CallDirective(node) => node.syntax(),
            Self::EmitDirective(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallDirective<'a>(OscNode<'a>);
impl CallDirective<'_> {
    pub fn call_token(&self) -> Option<CallToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for CallDirective<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == CALL_DIRECTIVE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitDirective<'a>(OscNode<'a>);
impl EmitDirective<'_> {
    pub fn emit_token(&self) -> Option<EmitToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn arguments(&self) -> Option<Arguments> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EmitDirective<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EMIT_DIRECTIVE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMember<'a>(OscNode<'a>);
impl DoMember<'_> {
    pub fn label_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn do_member_body(&self) -> Option<DoMemberBody> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for DoMember<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DO_MEMBER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoMemberBody<'a> {
    Composition(Composition<'a>),
    BehaviorInvocation(BehaviorInvocation<'a>),
    WaitDirective(WaitDirective<'a>),
    EmitDirective(EmitDirective<'a>),
    CallDirective(CallDirective<'a>),
}
impl DoMemberBody<'_> {
    pub fn as_composition(&self) -> Option<Composition> {
        match self {
            Self::Composition(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_behavior_invocation(&self) -> Option<BehaviorInvocation> {
        match self {
            Self::BehaviorInvocation(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_wait_directive(&self) -> Option<WaitDirective> {
        match self {
            Self::WaitDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_emit_directive(&self) -> Option<EmitDirective> {
        match self {
            Self::EmitDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_call_directive(&self) -> Option<CallDirective> {
        match self {
            Self::CallDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for DoMemberBody<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            COMPOSITION | BEHAVIOR_INVOCATION | WAIT_DIRECTIVE | EMIT_DIRECTIVE | CALL_DIRECTIVE
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            COMPOSITION => Some(Self::Composition(Composition::cast(node.clone())?)),
            BEHAVIOR_INVOCATION => Some(Self::BehaviorInvocation(BehaviorInvocation::cast(
                node.clone(),
            )?)),
            WAIT_DIRECTIVE => Some(Self::WaitDirective(WaitDirective::cast(node.clone())?)),
            EMIT_DIRECTIVE => Some(Self::EmitDirective(EmitDirective::cast(node.clone())?)),
            CALL_DIRECTIVE => Some(Self::CallDirective(CallDirective::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::Composition(node) => node.syntax(),
            Self::BehaviorInvocation(node) => node.syntax(),
            Self::WaitDirective(node) => node.syntax(),
            Self::EmitDirective(node) => node.syntax(),
            Self::CallDirective(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composition<'a>(OscNode<'a>);
impl Composition<'_> {
    pub fn composition_operator(&self) -> Option<CompositionOperator> {
        support::child(&self.0, 0usize)
    }
    pub fn arguments(&self) -> Option<Arguments> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn do_member_list(&self) -> Option<DoMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn behavior_with_declaration(&self) -> Option<BehaviorWithDeclaration> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for Composition<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COMPOSITION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorInvocation<'a>(OscNode<'a>);
impl BehaviorInvocation<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn behavior_with_declaration_or_newline(&self) -> Option<BehaviorWithDeclarationOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for BehaviorInvocation<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BEHAVIOR_INVOCATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitDirective<'a>(OscNode<'a>);
impl WaitDirective<'_> {
    pub fn wait_token(&self) -> Option<WaitToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for WaitDirective<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == WAIT_DIRECTIVE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionOperator<'a> {
    SerialToken(SerialToken<'a>),
    OneOfToken(OneOfToken<'a>),
    ParallelToken(ParallelToken<'a>),
}
impl CompositionOperator<'_> {
    pub fn as_serial_token(&self) -> Option<SerialToken> {
        match self {
            Self::SerialToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_one_of_token(&self) -> Option<OneOfToken> {
        match self {
            Self::OneOfToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_parallel_token(&self) -> Option<ParallelToken> {
        match self {
            Self::ParallelToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for CompositionOperator<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SERIAL_KW | ONE_OF_KW | PARALLEL_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SERIAL_KW => Some(Self::SerialToken(SerialToken::cast(node.clone())?)),
            ONE_OF_KW => Some(Self::OneOfToken(OneOfToken::cast(node.clone())?)),
            PARALLEL_KW => Some(Self::ParallelToken(ParallelToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::SerialToken(node) => node.syntax(),
            Self::OneOfToken(node) => node.syntax(),
            Self::ParallelToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberList<'a>(OscNode<'a>);
impl<'a> DoMemberList<'a> {
    pub fn do_member(&self) -> impl Iterator<Item = DoMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for DoMemberList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DO_MEMBER_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithDeclaration<'a>(OscNode<'a>);
impl BehaviorWithDeclaration<'_> {
    pub fn with_token(&self) -> Option<WithToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
    pub fn indent_token(&self) -> Option<IndentToken> {
        support::child(&self.0, 0usize)
    }
    pub fn behavior_with_member_list(&self) -> Option<BehaviorWithMemberList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for BehaviorWithDeclaration<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BEHAVIOR_WITH_DECLARATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorWithDeclarationOrNewline<'a> {
    BehaviorWithDeclaration(BehaviorWithDeclaration<'a>),
    NewlineToken(NewlineToken<'a>),
}
impl BehaviorWithDeclarationOrNewline<'_> {
    pub fn as_behavior_with_declaration(&self) -> Option<BehaviorWithDeclaration> {
        match self {
            Self::BehaviorWithDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_newline_token(&self) -> Option<NewlineToken> {
        match self {
            Self::NewlineToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BehaviorWithDeclarationOrNewline<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, BEHAVIOR_WITH_DECLARATION | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            BEHAVIOR_WITH_DECLARATION => Some(Self::BehaviorWithDeclaration(
                BehaviorWithDeclaration::cast(node.clone())?,
            )),
            NEWLINE => Some(Self::NewlineToken(NewlineToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::BehaviorWithDeclaration(node) => node.syntax(),
            Self::NewlineToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithMemberList<'a>(OscNode<'a>);
impl<'a> BehaviorWithMemberList<'a> {
    pub fn behavior_with_member(&self) -> impl Iterator<Item = BehaviorWithMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for BehaviorWithMemberList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BEHAVIOR_WITH_MEMBER_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorWithMember<'a> {
    ConstraintDeclaration(ConstraintDeclaration<'a>),
    ModifierApplication(ModifierApplication<'a>),
    UntilDirective(UntilDirective<'a>),
}
impl BehaviorWithMember<'_> {
    pub fn as_constraint_declaration(&self) -> Option<ConstraintDeclaration> {
        match self {
            Self::ConstraintDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_modifier_application(&self) -> Option<ModifierApplication> {
        match self {
            Self::ModifierApplication(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_until_directive(&self) -> Option<UntilDirective> {
        match self {
            Self::UntilDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BehaviorWithMember<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KEEP_CONSTRAINT_DECLARATION
                | REMOVE_DEFAULT_DECLARATION
                | MODIFIER_APPLICATION
                | UNTIL_DIRECTIVE
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => Some(
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(node.clone())?),
            ),
            MODIFIER_APPLICATION => Some(Self::ModifierApplication(ModifierApplication::cast(
                node.clone(),
            )?)),
            UNTIL_DIRECTIVE => Some(Self::UntilDirective(UntilDirective::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ConstraintDeclaration(node) => node.syntax(),
            Self::ModifierApplication(node) => node.syntax(),
            Self::UntilDirective(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntilDirective<'a>(OscNode<'a>);
impl UntilDirective<'_> {
    pub fn until_token(&self) -> Option<UntilToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for UntilDirective<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNTIL_DIRECTIVE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSpecificationList<'a>(OscNode<'a>);
impl<'a> ArgumentSpecificationList<'a> {
    pub fn argument_specification(&self) -> impl Iterator<Item = ArgumentSpecification<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ArgumentSpecificationList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_SPECIFICATION_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSpecification<'a>(OscNode<'a>);
impl ArgumentSpecification<'_> {
    pub fn argument_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_declarator(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_initializer_clause(&self) -> Option<ArgumentInitializerClause> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ArgumentSpecification<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_SPECIFICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentInitializerClause<'a>(OscNode<'a>);
impl ArgumentInitializerClause<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn default_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ArgumentInitializerClause<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_INITIALIZER_CLAUSE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentList<'a>(OscNode<'a>);
impl<'a> ArgumentList<'a> {
    pub fn argument(&self) -> impl Iterator<Item = Argument<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ArgumentList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Argument<'a> {
    PositionalArgument(PositionalArgument<'a>),
    NamedArgument(NamedArgument<'a>),
}
impl Argument<'_> {
    pub fn as_positional_argument(&self) -> Option<PositionalArgument> {
        match self {
            Self::PositionalArgument(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_named_argument(&self) -> Option<NamedArgument> {
        match self {
            Self::NamedArgument(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for Argument<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, POSITIONAL_ARGUMENT | NAMED_ARGUMENT)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            POSITIONAL_ARGUMENT => Some(Self::PositionalArgument(PositionalArgument::cast(
                node.clone(),
            )?)),
            NAMED_ARGUMENT => Some(Self::NamedArgument(NamedArgument::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::PositionalArgument(node) => node.syntax(),
            Self::NamedArgument(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalArgument<'a>(OscNode<'a>);
impl PositionalArgument<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PositionalArgument<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == POSITIONAL_ARGUMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgument<'a>(OscNode<'a>);
impl NamedArgument<'_> {
    pub fn label(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for NamedArgument<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMED_ARGUMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryExp<'a>(OscNode<'a>);
impl TernaryExp<'_> {
    pub fn condition(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn question_token(&self) -> Option<QuestionToken> {
        support::child(&self.0, 0usize)
    }
    pub fn then_expr(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn else_expr(&self) -> Option<Expression> {
        support::child(&self.0, 2usize)
    }
}
impl<'a> TypedNode for TernaryExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TERNARY_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalExp<'a>(OscNode<'a>);
impl LogicalExp<'_> {
    pub fn lhs_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn logical_op(&self) -> Option<LogicalOp> {
        support::child(&self.0, 0usize)
    }
    pub fn rhs_expr(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
}
impl<'a> TypedNode for LogicalExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LOGICAL_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExp<'a>(OscNode<'a>);
impl BinaryExp<'_> {
    pub fn lhs_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn binary_op(&self) -> Option<BinaryOp> {
        support::child(&self.0, 0usize)
    }
    pub fn rhs_expr(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
}
impl<'a> TypedNode for BinaryExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BINARY_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExp<'a>(OscNode<'a>);
impl UnaryExp<'_> {
    pub fn unary_op(&self) -> Option<UnaryOp> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for UnaryExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNARY_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExp<'a>(OscNode<'a>);
impl CastExp<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn as_token(&self) -> Option<AsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_declarator(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for CastExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == CAST_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTestExp<'a>(OscNode<'a>);
impl TypeTestExp<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn is_token(&self) -> Option<IsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_declarator(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for TypeTestExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TYPE_TEST_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementAccess<'a>(OscNode<'a>);
impl ElementAccess<'_> {
    pub fn list_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn left_bracket_token(&self) -> Option<LeftBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn index_expr(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
    pub fn right_bracket_token(&self) -> Option<RightBracketToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ElementAccess<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ELEMENT_ACCESS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionApplication<'a>(OscNode<'a>);
impl FunctionApplication<'_> {
    pub fn function_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn arguments(&self) -> Option<Arguments> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for FunctionApplication<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FUNCTION_APPLICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberReference<'a>(OscNode<'a>);
impl MemberReference<'_> {
    pub fn object_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MemberReference<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MEMBER_REFERENCE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExp<'a>(OscNode<'a>);
impl ParenthesizedExp<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParenthesizedExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARENTHESIZED_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralExp<'a> {
    IntegerLiteralToken(IntegerLiteralToken<'a>),
    FloatLiteralToken(FloatLiteralToken<'a>),
    PhysicalLiteral(PhysicalLiteral<'a>),
    BoolLiteral(BoolLiteral<'a>),
    StringLiteralToken(StringLiteralToken<'a>),
}
impl LiteralExp<'_> {
    pub fn as_integer_literal_token(&self) -> Option<IntegerLiteralToken> {
        match self {
            Self::IntegerLiteralToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_float_literal_token(&self) -> Option<FloatLiteralToken> {
        match self {
            Self::FloatLiteralToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_physical_literal(&self) -> Option<PhysicalLiteral> {
        match self {
            Self::PhysicalLiteral(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_bool_literal(&self) -> Option<BoolLiteral> {
        match self {
            Self::BoolLiteral(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_string_literal_token(&self) -> Option<StringLiteralToken> {
        match self {
            Self::StringLiteralToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for LiteralExp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            INTEGER_LITERAL => Some(Self::IntegerLiteralToken(IntegerLiteralToken::cast(
                node.clone(),
            )?)),
            FLOAT_LITERAL => Some(Self::FloatLiteralToken(FloatLiteralToken::cast(
                node.clone(),
            )?)),
            PHYSICAL_LITERAL => Some(Self::PhysicalLiteral(PhysicalLiteral::cast(node.clone())?)),
            TRUE_KW | FALSE_KW => Some(Self::BoolLiteral(BoolLiteral::cast(node.clone())?)),
            STRING_LITERAL => Some(Self::StringLiteralToken(StringLiteralToken::cast(
                node.clone(),
            )?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::IntegerLiteralToken(node) => node.syntax(),
            Self::FloatLiteralToken(node) => node.syntax(),
            Self::PhysicalLiteral(node) => node.syntax(),
            Self::BoolLiteral(node) => node.syntax(),
            Self::StringLiteralToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueReference<'a>(OscNode<'a>);
impl EnumValueReference<'_> {
    pub fn enum_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn exclamation_token(&self) -> Option<ExclamationToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 1usize)
    }
}
impl<'a> TypedNode for EnumValueReference<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_VALUE_REFERENCE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListConstructor<'a>(OscNode<'a>);
impl ListConstructor<'_> {
    pub fn left_bracket_token(&self) -> Option<LeftBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression_list(&self) -> Option<ExpressionList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_bracket_token(&self) -> Option<RightBracketToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ListConstructor<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LIST_CONSTRUCTOR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeConstructor<'a> {
    ParenthesesRangeConstructor(ParenthesesRangeConstructor<'a>),
    BracketsRangeConstructor(BracketsRangeConstructor<'a>),
}
impl RangeConstructor<'_> {
    pub fn as_parentheses_range_constructor(&self) -> Option<ParenthesesRangeConstructor> {
        match self {
            Self::ParenthesesRangeConstructor(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_brackets_range_constructor(&self) -> Option<BracketsRangeConstructor> {
        match self {
            Self::BracketsRangeConstructor(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for RangeConstructor<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            PARENTHESES_RANGE_CONSTRUCTOR | BRACKETS_RANGE_CONSTRUCTOR
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PARENTHESES_RANGE_CONSTRUCTOR => Some(Self::ParenthesesRangeConstructor(
                ParenthesesRangeConstructor::cast(node.clone())?,
            )),
            BRACKETS_RANGE_CONSTRUCTOR => Some(Self::BracketsRangeConstructor(
                BracketsRangeConstructor::cast(node.clone())?,
            )),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ParenthesesRangeConstructor(node) => node.syntax(),
            Self::BracketsRangeConstructor(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalOp<'a> {
    FatArrowToken(FatArrowToken<'a>),
    OrToken(OrToken<'a>),
    AndToken(AndToken<'a>),
}
impl LogicalOp<'_> {
    pub fn as_fat_arrow_token(&self) -> Option<FatArrowToken> {
        match self {
            Self::FatArrowToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_or_token(&self) -> Option<OrToken> {
        match self {
            Self::OrToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_and_token(&self) -> Option<AndToken> {
        match self {
            Self::AndToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for LogicalOp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, FAT_ARROW | OR_KW | AND_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            FAT_ARROW => Some(Self::FatArrowToken(FatArrowToken::cast(node.clone())?)),
            OR_KW => Some(Self::OrToken(OrToken::cast(node.clone())?)),
            AND_KW => Some(Self::AndToken(AndToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::FatArrowToken(node) => node.syntax(),
            Self::OrToken(node) => node.syntax(),
            Self::AndToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp<'a> {
    EqualToken(EqualToken<'a>),
    NotEqualToken(NotEqualToken<'a>),
    LessToken(LessToken<'a>),
    LessEqualToken(LessEqualToken<'a>),
    GreaterToken(GreaterToken<'a>),
    GreaterEqualToken(GreaterEqualToken<'a>),
    InToken(InToken<'a>),
    PlusToken(PlusToken<'a>),
    MinusToken(MinusToken<'a>),
    StarToken(StarToken<'a>),
    SlashToken(SlashToken<'a>),
    PercentToken(PercentToken<'a>),
}
impl BinaryOp<'_> {
    pub fn as_equal_token(&self) -> Option<EqualToken> {
        match self {
            Self::EqualToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_not_equal_token(&self) -> Option<NotEqualToken> {
        match self {
            Self::NotEqualToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_less_token(&self) -> Option<LessToken> {
        match self {
            Self::LessToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_less_equal_token(&self) -> Option<LessEqualToken> {
        match self {
            Self::LessEqualToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_greater_token(&self) -> Option<GreaterToken> {
        match self {
            Self::GreaterToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_greater_equal_token(&self) -> Option<GreaterEqualToken> {
        match self {
            Self::GreaterEqualToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_in_token(&self) -> Option<InToken> {
        match self {
            Self::InToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_plus_token(&self) -> Option<PlusToken> {
        match self {
            Self::PlusToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_minus_token(&self) -> Option<MinusToken> {
        match self {
            Self::MinusToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_star_token(&self) -> Option<StarToken> {
        match self {
            Self::StarToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_slash_token(&self) -> Option<SlashToken> {
        match self {
            Self::SlashToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_percent_token(&self) -> Option<PercentToken> {
        match self {
            Self::PercentToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BinaryOp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            EQUAL
                | NOT_EQUAL
                | LESS
                | LESS_EQUAL
                | GREATER
                | GREATER_EQUAL
                | IN_KW
                | PLUS
                | MINUS
                | STAR
                | SLASH
                | PERCENT
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EQUAL => Some(Self::EqualToken(EqualToken::cast(node.clone())?)),
            NOT_EQUAL => Some(Self::NotEqualToken(NotEqualToken::cast(node.clone())?)),
            LESS => Some(Self::LessToken(LessToken::cast(node.clone())?)),
            LESS_EQUAL => Some(Self::LessEqualToken(LessEqualToken::cast(node.clone())?)),
            GREATER => Some(Self::GreaterToken(GreaterToken::cast(node.clone())?)),
            GREATER_EQUAL => Some(Self::GreaterEqualToken(GreaterEqualToken::cast(
                node.clone(),
            )?)),
            IN_KW => Some(Self::InToken(InToken::cast(node.clone())?)),
            PLUS => Some(Self::PlusToken(PlusToken::cast(node.clone())?)),
            MINUS => Some(Self::MinusToken(MinusToken::cast(node.clone())?)),
            STAR => Some(Self::StarToken(StarToken::cast(node.clone())?)),
            SLASH => Some(Self::SlashToken(SlashToken::cast(node.clone())?)),
            PERCENT => Some(Self::PercentToken(PercentToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::EqualToken(node) => node.syntax(),
            Self::NotEqualToken(node) => node.syntax(),
            Self::LessToken(node) => node.syntax(),
            Self::LessEqualToken(node) => node.syntax(),
            Self::GreaterToken(node) => node.syntax(),
            Self::GreaterEqualToken(node) => node.syntax(),
            Self::InToken(node) => node.syntax(),
            Self::PlusToken(node) => node.syntax(),
            Self::MinusToken(node) => node.syntax(),
            Self::StarToken(node) => node.syntax(),
            Self::SlashToken(node) => node.syntax(),
            Self::PercentToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp<'a> {
    NotToken(NotToken<'a>),
    MinusToken(MinusToken<'a>),
}
impl UnaryOp<'_> {
    pub fn as_not_token(&self) -> Option<NotToken> {
        match self {
            Self::NotToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_minus_token(&self) -> Option<MinusToken> {
        match self {
            Self::MinusToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for UnaryOp<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, NOT_KW | MINUS)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            NOT_KW => Some(Self::NotToken(NotToken::cast(node.clone())?)),
            MINUS => Some(Self::MinusToken(MinusToken::cast(node.clone())?)),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::NotToken(node) => node.syntax(),
            Self::MinusToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionList<'a>(OscNode<'a>);
impl<'a> ExpressionList<'a> {
    pub fn expression_list_element(&self) -> impl Iterator<Item = ExpressionListElement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ExpressionList<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPRESSION_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionListElement<'a>(OscNode<'a>);
impl ExpressionListElement<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ExpressionListElement<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXPRESSION_LIST_ELEMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesRangeConstructor<'a>(OscNode<'a>);
impl ParenthesesRangeConstructor<'_> {
    pub fn range_token(&self) -> Option<RangeToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn begin_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn end_expr(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParenthesesRangeConstructor<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARENTHESES_RANGE_CONSTRUCTOR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketsRangeConstructor<'a>(OscNode<'a>);
impl BracketsRangeConstructor<'_> {
    pub fn left_bracket_token(&self) -> Option<LeftBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn begin_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_dot_token(&self) -> Option<DotDotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn end_expr(&self) -> Option<Expression> {
        support::child(&self.0, 1usize)
    }
    pub fn right_bracket_token(&self) -> Option<RightBracketToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for BracketsRangeConstructor<'a> {
    type Value = OscSyntaxKind;
    type Node = OscNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BRACKETS_RANGE_CONSTRUCTOR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}