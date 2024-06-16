use super::OscDslSyntaxKind::{self, *};
use crate::syntax::{support, TypedNode};
use syntree::Node;
type OscDslNode<'a> = Node<'a, OscDslSyntaxKind, u32, usize>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for DotToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DotDotToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for DotDotToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct CommaToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for CommaToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ColonToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ColonToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ColonColonToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ColonColonToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct AssignToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for AssignToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct AtToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for AtToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ArrowToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ArrowToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct LeftParenToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for LeftParenToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RightParenToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RightParenToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct LeftBracketToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for LeftBracketToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RightBracketToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RightBracketToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct QuestionToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for QuestionToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExclamationToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ExclamationToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FatArrowToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for FatArrowToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EqualToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for EqualToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NotEqualToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for NotEqualToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct LessToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for LessToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct LessEqualToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for LessEqualToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct GreaterToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for GreaterToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct GreaterEqualToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for GreaterEqualToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct PlusToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for PlusToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct MinusToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for MinusToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StarToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for StarToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SlashToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for SlashToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct PercentToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for PercentToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct AToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for AToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActionToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ActionToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActorToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ActorToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct AndToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for AndToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct AsToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for AsToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct BoolToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for BoolToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct CallToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for CallToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct CdToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for CdToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct CoverToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for CoverToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DefToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for DefToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DefaultToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for DefaultToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DoToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for DoToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ElapsedToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ElapsedToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EmitToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for EmitToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EnumToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for EnumToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EventToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for EventToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EveryToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for EveryToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExportToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ExportToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExpressionToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ExpressionToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExtendToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ExtendToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExternalToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ExternalToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FactorToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for FactorToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FallToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for FallToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FalseToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for FalseToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FloatToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for FloatToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct GlobalToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for GlobalToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct HardToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for HardToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct IfToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for IfToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ImportToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ImportToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct InToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for InToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct InfToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for InfToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct InheritsToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for InheritsToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct IntToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for IntToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct IsToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for IsToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ItToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ItToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct KToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for KToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct KeepToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for KeepToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct KgToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for KgToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ListToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ListToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct MToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for MToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ModifierToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ModifierToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct MolToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for MolToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NamespaceToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for NamespaceToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NanToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for NanToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NotToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for NotToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NullToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for NullToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OfToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for OfToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OffsetToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for OffsetToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OnToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for OnToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OneOfToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for OneOfToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OnlyToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for OnlyToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OrToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for OrToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ParallelToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ParallelToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RadToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RadToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RangeToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RangeToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RecordToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RecordToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RemoveDefaultToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RemoveDefaultToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RiseToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for RiseToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for SToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SampleToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for SampleToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ScenarioToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ScenarioToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SerialToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for SerialToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SiToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for SiToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StringToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for StringToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StructToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for StructToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct TrueToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for TrueToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct TypeToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for TypeToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UintToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for UintToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UndefinedToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for UndefinedToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UnitToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for UnitToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UntilToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for UntilToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UseToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for UseToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct VarToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for VarToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct WaitToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for WaitToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct WithToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for WithToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct IntegerLiteralToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for IntegerLiteralToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FloatLiteralToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for FloatLiteralToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StringLiteralToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for StringLiteralToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NewlineToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for NewlineToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct IndentToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for IndentToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DedentToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for DedentToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct IdentifierToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for IdentifierToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ErrorToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for ErrorToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct WhitespaceToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for WhitespaceToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct CommentToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for CommentToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct TrivialNewlineToken<'a>(OscDslNode<'a>);
impl<'a> TypedNode for TrivialNewlineToken<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IDENTIFIER | PREFIXED_IDENTIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER => IdentifierToken::cast(node.clone()).map(Self::IdentifierToken),
            PREFIXED_IDENTIFIER => {
                PrefixedIdentifier::cast(node.clone()).map(Self::PrefixedIdentifier)
            }
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
pub struct PrefixedIdentifier<'a>(OscDslNode<'a>);
impl PrefixedIdentifier<'_> {
    pub fn namespace_name(&self) -> Option<IdentifierToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_colon_token(&self) -> Option<ColonColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn identifier_token(&self) -> Option<IdentifierToken> {
        support::child(&self.0, 1usize)
    }
}
impl<'a> TypedNode for PrefixedIdentifier<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, TRUE_KW | FALSE_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TRUE_KW => TrueToken::cast(node.clone()).map(Self::TrueToken),
            FALSE_KW => FalseToken::cast(node.clone()).map(Self::FalseToken),
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
pub struct PhysicalLiteral<'a>(OscDslNode<'a>);
impl PhysicalLiteral<'_> {
    pub fn number_literal(&self) -> Option<NumberLiteral> {
        support::child(&self.0, 0usize)
    }
    pub fn unit_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PhysicalLiteral<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, FLOAT_LITERAL | INTEGER_LITERAL)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            FLOAT_LITERAL => FloatLiteralToken::cast(node.clone()).map(Self::FloatLiteralToken),
            INTEGER_LITERAL => {
                IntegerLiteralToken::cast(node.clone()).map(Self::IntegerLiteralToken)
            }
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
pub struct OscFile<'a>(OscDslNode<'a>);
impl OscFile<'_> {
    pub fn prelude_statement_list(&self) -> Option<PreludeStatementList> {
        support::child(&self.0, 0usize)
    }
    pub fn main_statement_list(&self) -> Option<MainStatementList> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for OscFile<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct PreludeStatementList<'a>(OscDslNode<'a>);
impl<'a> PreludeStatementList<'a> {
    pub fn prelude_statement(&self) -> impl Iterator<Item = PreludeStatement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for PreludeStatementList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PRELUDE_STATEMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainStatementList<'a>(OscDslNode<'a>);
impl<'a> MainStatementList<'a> {
    pub fn main_statement(&self) -> impl Iterator<Item = MainStatement<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for MainStatementList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MAIN_STATEMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IMPORT_STATEMENT)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IMPORT_STATEMENT => ImportStatement::cast(node.clone()).map(Self::ImportStatement),
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
                | ENUM_TYPE_EXTENSION
                | STRUCTURED_TYPE_EXTENSION
                | GLOBAL_PARAMETER_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            NAMESPACE_STATEMENT => {
                NamespaceStatement::cast(node.clone()).map(Self::NamespaceStatement)
            }
            EXPORT_STATEMENT => ExportStatement::cast(node.clone()).map(Self::ExportStatement),
            PHYSICAL_TYPE_DECLARATION
            | UNIT_DECLARATION
            | ENUM_DECLARATION
            | STRUCT_DECLARATION
            | ACTOR_DECLARATION
            | ACTION_DECLARATION
            | SCENARIO_DECLARATION
            | MODIFIER_DECLARATION
            | ENUM_TYPE_EXTENSION
            | STRUCTURED_TYPE_EXTENSION
            | GLOBAL_PARAMETER_DECLARATION => {
                OscDeclaration::cast(node.clone()).map(Self::OscDeclaration)
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
pub struct ImportStatement<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, STRING_LITERAL | STRUCTURED_IDENTIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            STRING_LITERAL => StringLiteralToken::cast(node.clone()).map(Self::StringLiteralToken),
            STRUCTURED_IDENTIFIER => {
                StructuredIdentifier::cast(node.clone()).map(Self::StructuredIdentifier)
            }
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
pub struct StructuredIdentifier<'a>(OscDslNode<'a>);
impl<'a> StructuredIdentifier<'a> {
    pub fn dot_token(&self) -> impl Iterator<Item = DotToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn identifier_token(&self) -> impl Iterator<Item = IdentifierToken<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for StructuredIdentifier<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCTURED_IDENTIFIER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceStatement<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExportStatement<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
                | ENUM_TYPE_EXTENSION
                | STRUCTURED_TYPE_EXTENSION
                | GLOBAL_PARAMETER_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PHYSICAL_TYPE_DECLARATION => {
                PhysicalTypeDeclaration::cast(node.clone()).map(Self::PhysicalTypeDeclaration)
            }
            UNIT_DECLARATION => UnitDeclaration::cast(node.clone()).map(Self::UnitDeclaration),
            ENUM_DECLARATION => EnumDeclaration::cast(node.clone()).map(Self::EnumDeclaration),
            STRUCT_DECLARATION => {
                StructDeclaration::cast(node.clone()).map(Self::StructDeclaration)
            }
            ACTOR_DECLARATION => ActorDeclaration::cast(node.clone()).map(Self::ActorDeclaration),
            ACTION_DECLARATION => {
                ActionDeclaration::cast(node.clone()).map(Self::ActionDeclaration)
            }
            SCENARIO_DECLARATION => {
                ScenarioDeclaration::cast(node.clone()).map(Self::ScenarioDeclaration)
            }
            MODIFIER_DECLARATION => {
                ModifierDeclaration::cast(node.clone()).map(Self::ModifierDeclaration)
            }
            ENUM_TYPE_EXTENSION | STRUCTURED_TYPE_EXTENSION => {
                TypeExtension::cast(node.clone()).map(Self::TypeExtension)
            }
            GLOBAL_PARAMETER_DECLARATION => {
                GlobalParameterDeclaration::cast(node.clone()).map(Self::GlobalParameterDeclaration)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IDENTIFIER | NULL_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER => IdentifierToken::cast(node.clone()).map(Self::IdentifierToken),
            NULL_KW => NullToken::cast(node.clone()).map(Self::NullToken),
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
pub struct NamespaceUseClause<'a>(OscDslNode<'a>);
impl NamespaceUseClause<'_> {
    pub fn use_token(&self) -> Option<UseToken> {
        support::child(&self.0, 0usize)
    }
    pub fn namespace_list(&self) -> Option<NamespaceList> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for NamespaceUseClause<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct NamespaceList<'a>(OscDslNode<'a>);
impl<'a> NamespaceList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn namespace_name(&self) -> impl Iterator<Item = NamespaceName<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for NamespaceList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ExportSpecificationList<'a>(OscDslNode<'a>);
impl<'a> ExportSpecificationList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn export_specification(&self) -> impl Iterator<Item = ExportSpecification<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ExportSpecificationList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | STAR | PREFIXED_EXPORT_WILDCARD_SPECIFICATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            STAR | PREFIXED_EXPORT_WILDCARD_SPECIFICATION => {
                ExportWildcardSpecification::cast(node.clone())
                    .map(Self::ExportWildcardSpecification)
            }
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
pub enum ExportWildcardSpecification<'a> {
    StarToken(StarToken<'a>),
    PrefixedExportWildcardSpecification(PrefixedExportWildcardSpecification<'a>),
}
impl ExportWildcardSpecification<'_> {
    pub fn as_star_token(&self) -> Option<StarToken> {
        match self {
            Self::StarToken(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_export_wildcard_specification(
        &self,
    ) -> Option<PrefixedExportWildcardSpecification> {
        match self {
            Self::PrefixedExportWildcardSpecification(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ExportWildcardSpecification<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, STAR | PREFIXED_EXPORT_WILDCARD_SPECIFICATION)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            STAR => StarToken::cast(node.clone()).map(Self::StarToken),
            PREFIXED_EXPORT_WILDCARD_SPECIFICATION => {
                PrefixedExportWildcardSpecification::cast(node.clone())
                    .map(Self::PrefixedExportWildcardSpecification)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::StarToken(node) => node.syntax(),
            Self::PrefixedExportWildcardSpecification(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedExportWildcardSpecification<'a>(OscDslNode<'a>);
impl PrefixedExportWildcardSpecification<'_> {
    pub fn namespace_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_colon_token(&self) -> Option<ColonColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn star_token(&self) -> Option<StarToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PrefixedExportWildcardSpecification<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_EXPORT_WILDCARD_SPECIFICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTypeDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UnitDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EnumDeclaration<'a>(OscDslNode<'a>);
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
    pub fn left_bracket_token(&self) -> Option<LeftBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_decl_list(&self) -> Option<EnumMemberDeclList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_bracket_token(&self) -> Option<RightBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumDeclaration<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StructDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActorDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActionDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ScenarioDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ModifierDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub enum TypeExtension<'a> {
    EnumTypeExtension(EnumTypeExtension<'a>),
    StructuredTypeExtension(StructuredTypeExtension<'a>),
}
impl TypeExtension<'_> {
    pub fn as_enum_type_extension(&self) -> Option<EnumTypeExtension> {
        match self {
            Self::EnumTypeExtension(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_structured_type_extension(&self) -> Option<StructuredTypeExtension> {
        match self {
            Self::StructuredTypeExtension(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for TypeExtension<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ENUM_TYPE_EXTENSION | STRUCTURED_TYPE_EXTENSION)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ENUM_TYPE_EXTENSION => {
                EnumTypeExtension::cast(node.clone()).map(Self::EnumTypeExtension)
            }
            STRUCTURED_TYPE_EXTENSION => {
                StructuredTypeExtension::cast(node.clone()).map(Self::StructuredTypeExtension)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::EnumTypeExtension(node) => node.syntax(),
            Self::StructuredTypeExtension(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalParameterDeclaration<'a>(OscDslNode<'a>);
impl GlobalParameterDeclaration<'_> {
    pub fn global_token(&self) -> Option<GlobalToken> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_declaration(&self) -> Option<ParameterDeclaration> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for GlobalParameterDeclaration<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PREFIXED_BEHAVIOR_NAME => {
                NonAggregateTypeDeclarator::cast(node.clone()).map(Self::NonAggregateTypeDeclarator)
            }
            LIST_TYPE_DECLARATOR => {
                AggregateTypeDeclarator::cast(node.clone()).map(Self::AggregateTypeDeclarator)
            }
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
    TypeReference(TypeReference<'a>),
}
impl NonAggregateTypeDeclarator<'_> {
    pub fn as_primitive_type(&self) -> Option<PrimitiveType> {
        match self {
            Self::PrimitiveType(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_type_reference(&self) -> Option<TypeReference> {
        match self {
            Self::TypeReference(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for NonAggregateTypeDeclarator<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_BEHAVIOR_NAME
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            INT_KW | UINT_KW | FLOAT_KW | BOOL_KW | STRING_KW => {
                PrimitiveType::cast(node.clone()).map(Self::PrimitiveType)
            }
            IDENTIFIER
            | PREFIXED_IDENTIFIER
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PREFIXED_BEHAVIOR_NAME => TypeReference::cast(node.clone()).map(Self::TypeReference),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::PrimitiveType(node) => node.syntax(),
            Self::TypeReference(node) => node.syntax(),
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, LIST_TYPE_DECLARATOR)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            LIST_TYPE_DECLARATOR => {
                ListTypeDeclarator::cast(node.clone()).map(Self::ListTypeDeclarator)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, INT_KW | UINT_KW | FLOAT_KW | BOOL_KW | STRING_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            INT_KW => IntToken::cast(node.clone()).map(Self::IntToken),
            UINT_KW => UintToken::cast(node.clone()).map(Self::UintToken),
            FLOAT_KW => FloatToken::cast(node.clone()).map(Self::FloatToken),
            BOOL_KW => BoolToken::cast(node.clone()).map(Self::BoolToken),
            STRING_KW => StringToken::cast(node.clone()).map(Self::StringToken),
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
pub enum TypeReference<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    QualifiedBehaviorName(QualifiedBehaviorName<'a>),
}
impl TypeReference<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
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
impl<'a> TypedNode for TypeReference<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER
                | PREFIXED_IDENTIFIER
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_BEHAVIOR_NAME
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_BEHAVIOR_NAME => {
                QualifiedBehaviorName::cast(node.clone()).map(Self::QualifiedBehaviorName)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::QualifiedBehaviorName(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTypeDeclarator<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_BEHAVIOR_NAME
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            PREFIXED_BEHAVIOR_NAME => {
                PrefixedBehaviorName::cast(node.clone()).map(Self::PrefixedBehaviorName)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SI_BASE_UNIT_SPECIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SI_BASE_UNIT_SPECIFIER => {
                SiBaseUnitSpecifier::cast(node.clone()).map(Self::SiBaseUnitSpecifier)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SI_UNIT_SPECIFIER)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SI_UNIT_SPECIFIER => SiUnitSpecifier::cast(node.clone()).map(Self::SiUnitSpecifier),
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
pub struct SiBaseUnitSpecifier<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SiUnitSpecifier<'a>(OscDslNode<'a>);
impl SiUnitSpecifier<'_> {
    pub fn si_token(&self) -> Option<SiToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn si_base_exponent_list(&self) -> Option<SiBaseExponentList> {
        support::child(&self.0, 0usize)
    }
    pub fn trailing_si_factor(&self) -> Option<TrailingSiFactor> {
        support::child(&self.0, 0usize)
    }
    pub fn trailing_si_offset(&self) -> Option<TrailingSiOffset> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SiUnitSpecifier<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SiBaseExponentList<'a>(OscDslNode<'a>);
impl<'a> SiBaseExponentList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn si_base_exponent(&self) -> impl Iterator<Item = SiBaseExponent<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for SiBaseExponentList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SiBaseExponent<'a>(OscDslNode<'a>);
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
}
impl<'a> TypedNode for SiBaseExponent<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KG_KW | M_KW | S_KW | A_KW | K_KW | MOL_KW | CD_KW | RAD_KW
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KG_KW => KgToken::cast(node.clone()).map(Self::KgToken),
            M_KW => MToken::cast(node.clone()).map(Self::MToken),
            S_KW => SToken::cast(node.clone()).map(Self::SToken),
            A_KW => AToken::cast(node.clone()).map(Self::AToken),
            K_KW => KToken::cast(node.clone()).map(Self::KToken),
            MOL_KW => MolToken::cast(node.clone()).map(Self::MolToken),
            CD_KW => CdToken::cast(node.clone()).map(Self::CdToken),
            RAD_KW => RadToken::cast(node.clone()).map(Self::RadToken),
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
    TernaryOpExp(TernaryOpExp<'a>),
    LogicalOpExp(LogicalOpExp<'a>),
    BinaryOpExp(BinaryOpExp<'a>),
    UnaryOpExp(UnaryOpExp<'a>),
    CastExp(CastExp<'a>),
    TypeTestExp(TypeTestExp<'a>),
    ElementAccess(ElementAccess<'a>),
    FunctionApplication(FunctionApplication<'a>),
    FieldAccess(FieldAccess<'a>),
    ItExp(ItExp<'a>),
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    ParenthesizedExp(ParenthesizedExp<'a>),
    LiteralExp(LiteralExp<'a>),
    EnumValueReference(EnumValueReference<'a>),
    ListConstructor(ListConstructor<'a>),
    RangeConstructor(RangeConstructor<'a>),
}
impl Expression<'_> {
    pub fn as_ternary_op_exp(&self) -> Option<TernaryOpExp> {
        match self {
            Self::TernaryOpExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_logical_op_exp(&self) -> Option<LogicalOpExp> {
        match self {
            Self::LogicalOpExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_binary_op_exp(&self) -> Option<BinaryOpExp> {
        match self {
            Self::BinaryOpExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_unary_op_exp(&self) -> Option<UnaryOpExp> {
        match self {
            Self::UnaryOpExp(node) => Some(node.clone()),
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
    pub fn as_field_access(&self) -> Option<FieldAccess> {
        match self {
            Self::FieldAccess(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_it_exp(&self) -> Option<ItExp> {
        match self {
            Self::ItExp(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            TERNARY_OP_EXP
                | LOGICAL_OP_EXP
                | BINARY_OP_EXP
                | UNARY_OP_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | FIELD_ACCESS
                | IT_KW
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | PARENTHESES_RANGE_CONSTRUCTOR
                | BRACKETS_RANGE_CONSTRUCTOR
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TERNARY_OP_EXP => TernaryOpExp::cast(node.clone()).map(Self::TernaryOpExp),
            LOGICAL_OP_EXP => LogicalOpExp::cast(node.clone()).map(Self::LogicalOpExp),
            BINARY_OP_EXP => BinaryOpExp::cast(node.clone()).map(Self::BinaryOpExp),
            UNARY_OP_EXP => UnaryOpExp::cast(node.clone()).map(Self::UnaryOpExp),
            CAST_EXP => CastExp::cast(node.clone()).map(Self::CastExp),
            TYPE_TEST_EXP => TypeTestExp::cast(node.clone()).map(Self::TypeTestExp),
            ELEMENT_ACCESS => ElementAccess::cast(node.clone()).map(Self::ElementAccess),
            FUNCTION_APPLICATION => {
                FunctionApplication::cast(node.clone()).map(Self::FunctionApplication)
            }
            FIELD_ACCESS => FieldAccess::cast(node.clone()).map(Self::FieldAccess),
            IT_KW => ItExp::cast(node.clone()).map(Self::ItExp),
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            PARENTHESIZED_EXP => ParenthesizedExp::cast(node.clone()).map(Self::ParenthesizedExp),
            INTEGER_LITERAL | FLOAT_LITERAL | PHYSICAL_LITERAL | TRUE_KW | FALSE_KW
            | STRING_LITERAL => LiteralExp::cast(node.clone()).map(Self::LiteralExp),
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_ENUM_VALUE_REFERENCE => {
                EnumValueReference::cast(node.clone()).map(Self::EnumValueReference)
            }
            LIST_CONSTRUCTOR => ListConstructor::cast(node.clone()).map(Self::ListConstructor),
            PARENTHESES_RANGE_CONSTRUCTOR | BRACKETS_RANGE_CONSTRUCTOR => {
                RangeConstructor::cast(node.clone()).map(Self::RangeConstructor)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::TernaryOpExp(node) => node.syntax(),
            Self::LogicalOpExp(node) => node.syntax(),
            Self::BinaryOpExp(node) => node.syntax(),
            Self::UnaryOpExp(node) => node.syntax(),
            Self::CastExp(node) => node.syntax(),
            Self::TypeTestExp(node) => node.syntax(),
            Self::ElementAccess(node) => node.syntax(),
            Self::FunctionApplication(node) => node.syntax(),
            Self::FieldAccess(node) => node.syntax(),
            Self::ItExp(node) => node.syntax(),
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::ParenthesizedExp(node) => node.syntax(),
            Self::LiteralExp(node) => node.syntax(),
            Self::EnumValueReference(node) => node.syntax(),
            Self::ListConstructor(node) => node.syntax(),
            Self::RangeConstructor(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingSiFactor<'a>(OscDslNode<'a>);
impl TrailingSiFactor<'_> {
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn factor_token(&self) -> Option<FactorToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for TrailingSiFactor<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TRAILING_SI_FACTOR
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingSiOffset<'a>(OscDslNode<'a>);
impl TrailingSiOffset<'_> {
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn offset_token(&self) -> Option<OffsetToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for TrailingSiOffset<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TRAILING_SI_OFFSET
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDeclList<'a>(OscDslNode<'a>);
impl<'a> EnumMemberDeclList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn enum_member_decl(&self) -> impl Iterator<Item = EnumMemberDecl<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for EnumMemberDeclList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EnumMemberDecl<'a>(OscDslNode<'a>);
impl EnumMemberDecl<'_> {
    pub fn enum_member_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_initializer(&self) -> Option<EnumMemberInitializer> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumMemberDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EnumMemberInitializer<'a>(OscDslNode<'a>);
impl EnumMemberInitializer<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumMemberInitializer<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_MEMBER_INITIALIZER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumValueReference<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    PrefixedEnumValueReference(PrefixedEnumValueReference<'a>),
}
impl EnumValueReference<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_enum_value_reference(&self) -> Option<PrefixedEnumValueReference> {
        match self {
            Self::PrefixedEnumValueReference(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for EnumValueReference<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_ENUM_VALUE_REFERENCE
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            PREFIXED_ENUM_VALUE_REFERENCE => {
                PrefixedEnumValueReference::cast(node.clone()).map(Self::PrefixedEnumValueReference)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::PrefixedEnumValueReference(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedEnumValueReference<'a>(OscDslNode<'a>);
impl PrefixedEnumValueReference<'_> {
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
impl<'a> TypedNode for PrefixedEnumValueReference<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_ENUM_VALUE_REFERENCE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsClause<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, STRUCT_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            STRUCT_BODY => StructBody::cast(node.clone()).map(Self::StructBody),
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct StructInheritsCondition<'a>(OscDslNode<'a>);
impl StructInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn equal_token(&self) -> Option<EqualToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructInheritsCondition<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StructBody<'a>(OscDslNode<'a>);
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
    pub fn struct_member_decl_list(&self) -> Option<StructMemberDeclList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct StructMemberDeclList<'a>(OscDslNode<'a>);
impl<'a> StructMemberDeclList<'a> {
    pub fn struct_member_decl(&self) -> impl Iterator<Item = StructMemberDecl<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for StructMemberDeclList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCT_MEMBER_DECL_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructMemberDecl<'a> {
    EventDeclaration(EventDeclaration<'a>),
    FieldDeclaration(FieldDeclaration<'a>),
    ConstraintDeclaration(ConstraintDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    CoverageDeclaration(CoverageDeclaration<'a>),
}
impl StructMemberDecl<'_> {
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
}
impl<'a> TypedNode for StructMemberDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_DECLARATION => EventDeclaration::cast(node.clone()).map(Self::EventDeclaration),
            PARAMETER_DECLARATION | VARIABLE_DECLARATION => {
                FieldDeclaration::cast(node.clone()).map(Self::FieldDeclaration)
            }
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => {
                ConstraintDeclaration::cast(node.clone()).map(Self::ConstraintDeclaration)
            }
            METHOD_DECLARATION => {
                MethodDeclaration::cast(node.clone()).map(Self::MethodDeclaration)
            }
            COVERAGE_DECLARATION => {
                CoverageDeclaration::cast(node.clone()).map(Self::CoverageDeclaration)
            }
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
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventDeclaration<'a>(OscDslNode<'a>);
impl EventDeclaration<'_> {
    pub fn event_token(&self) -> Option<EventToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn event_argument_list_specification(&self) -> Option<EventArgumentListSpecification> {
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, PARAMETER_DECLARATION | VARIABLE_DECLARATION)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PARAMETER_DECLARATION => {
                ParameterDeclaration::cast(node.clone()).map(Self::ParameterDeclaration)
            }
            VARIABLE_DECLARATION => {
                VariableDeclaration::cast(node.clone()).map(Self::VariableDeclaration)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KEEP_CONSTRAINT_DECLARATION => {
                KeepConstraintDeclaration::cast(node.clone()).map(Self::KeepConstraintDeclaration)
            }
            REMOVE_DEFAULT_DECLARATION => {
                RemoveDefaultDeclaration::cast(node.clone()).map(Self::RemoveDefaultDeclaration)
            }
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
pub struct MethodDeclaration<'a>(OscDslNode<'a>);
impl MethodDeclaration<'_> {
    pub fn def_token(&self) -> Option<DefToken> {
        support::child(&self.0, 0usize)
    }
    pub fn method_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_list_specification(&self) -> Option<ArgumentListSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct CoverageDeclaration<'a>(OscDslNode<'a>);
impl CoverageDeclaration<'_> {
    pub fn coverage_operator(&self) -> Option<CoverageOperator> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for CoverageDeclaration<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActorInheritsClause<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ACTOR_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ACTOR_BODY => ActorBody::cast(node.clone()).map(Self::ActorBody),
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct ActorInheritsCondition<'a>(OscDslNode<'a>);
impl ActorInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn equal_token(&self) -> Option<EqualToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActorInheritsCondition<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActorBody<'a>(OscDslNode<'a>);
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
    pub fn actor_member_decl_list(&self) -> Option<ActorMemberDeclList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActorBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActorMemberDeclList<'a>(OscDslNode<'a>);
impl<'a> ActorMemberDeclList<'a> {
    pub fn actor_member_decl(&self) -> impl Iterator<Item = ActorMemberDecl<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ActorMemberDeclList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTOR_MEMBER_DECL_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorMemberDecl<'a> {
    EventDeclaration(EventDeclaration<'a>),
    FieldDeclaration(FieldDeclaration<'a>),
    ConstraintDeclaration(ConstraintDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    CoverageDeclaration(CoverageDeclaration<'a>),
}
impl ActorMemberDecl<'_> {
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
}
impl<'a> TypedNode for ActorMemberDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_DECLARATION => EventDeclaration::cast(node.clone()).map(Self::EventDeclaration),
            PARAMETER_DECLARATION | VARIABLE_DECLARATION => {
                FieldDeclaration::cast(node.clone()).map(Self::FieldDeclaration)
            }
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => {
                ConstraintDeclaration::cast(node.clone()).map(Self::ConstraintDeclaration)
            }
            METHOD_DECLARATION => {
                MethodDeclaration::cast(node.clone()).map(Self::MethodDeclaration)
            }
            COVERAGE_DECLARATION => {
                CoverageDeclaration::cast(node.clone()).map(Self::CoverageDeclaration)
            }
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
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsClause<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SCENARIO_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SCENARIO_BODY => ScenarioBody::cast(node.clone()).map(Self::ScenarioBody),
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct ScenarioInheritsCondition<'a>(OscDslNode<'a>);
impl ScenarioInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn equal_token(&self) -> Option<EqualToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ScenarioInheritsCondition<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ScenarioBody<'a>(OscDslNode<'a>);
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
    pub fn scenario_member_item_list(&self) -> Option<ScenarioMemberItemList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ScenarioBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ScenarioMemberItemList<'a>(OscDslNode<'a>);
impl<'a> ScenarioMemberItemList<'a> {
    pub fn scenario_member_item(&self) -> impl Iterator<Item = ScenarioMemberItem<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ScenarioMemberItemList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SCENARIO_MEMBER_ITEM_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioMemberItem<'a> {
    ScenarioMemberDecl(ScenarioMemberDecl<'a>),
    BehaviorSpecification(BehaviorSpecification<'a>),
}
impl ScenarioMemberItem<'_> {
    pub fn as_scenario_member_decl(&self) -> Option<ScenarioMemberDecl> {
        match self {
            Self::ScenarioMemberDecl(node) => Some(node.clone()),
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
impl<'a> TypedNode for ScenarioMemberItem<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            EVENT_DECLARATION
            | PARAMETER_DECLARATION
            | VARIABLE_DECLARATION
            | KEEP_CONSTRAINT_DECLARATION
            | REMOVE_DEFAULT_DECLARATION
            | METHOD_DECLARATION
            | COVERAGE_DECLARATION
            | MODIFIER_APPLICATION => {
                ScenarioMemberDecl::cast(node.clone()).map(Self::ScenarioMemberDecl)
            }
            ON_DIRECTIVE | DO_DIRECTIVE => {
                BehaviorSpecification::cast(node.clone()).map(Self::BehaviorSpecification)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioMemberDecl<'a> {
    EventDeclaration(EventDeclaration<'a>),
    FieldDeclaration(FieldDeclaration<'a>),
    ConstraintDeclaration(ConstraintDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    CoverageDeclaration(CoverageDeclaration<'a>),
    ModifierApplication(ModifierApplication<'a>),
}
impl ScenarioMemberDecl<'_> {
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
}
impl<'a> TypedNode for ScenarioMemberDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_DECLARATION => EventDeclaration::cast(node.clone()).map(Self::EventDeclaration),
            PARAMETER_DECLARATION | VARIABLE_DECLARATION => {
                FieldDeclaration::cast(node.clone()).map(Self::FieldDeclaration)
            }
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => {
                ConstraintDeclaration::cast(node.clone()).map(Self::ConstraintDeclaration)
            }
            METHOD_DECLARATION => {
                MethodDeclaration::cast(node.clone()).map(Self::MethodDeclaration)
            }
            COVERAGE_DECLARATION => {
                CoverageDeclaration::cast(node.clone()).map(Self::CoverageDeclaration)
            }
            MODIFIER_APPLICATION => {
                ModifierApplication::cast(node.clone()).map(Self::ModifierApplication)
            }
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
        }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ON_DIRECTIVE | DO_DIRECTIVE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ON_DIRECTIVE => OnDirective::cast(node.clone()).map(Self::OnDirective),
            DO_DIRECTIVE => DoDirective::cast(node.clone()).map(Self::DoDirective),
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
pub struct ModifierApplication<'a>(OscDslNode<'a>);
impl ModifierApplication<'_> {
    pub fn actor_path(&self) -> Option<ActorPath> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierApplication<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct PrefixedBehaviorName<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActionInheritsClause<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ACTION_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ACTION_BODY => ActionBody::cast(node.clone()).map(Self::ActionBody),
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct ActionBody<'a>(OscDslNode<'a>);
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
    pub fn action_member_item_list(&self) -> Option<ActionMemberItemList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActionBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActionInheritsCondition<'a>(OscDslNode<'a>);
impl ActionInheritsCondition<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn equal_token(&self) -> Option<EqualToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ActionInheritsCondition<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ActionMemberItemList<'a>(OscDslNode<'a>);
impl<'a> ActionMemberItemList<'a> {
    pub fn action_member_item(&self) -> impl Iterator<Item = ActionMemberItem<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ActionMemberItemList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ACTION_MEMBER_ITEM_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionMemberItem<'a> {
    ScenarioMemberDecl(ScenarioMemberDecl<'a>),
    BehaviorSpecification(BehaviorSpecification<'a>),
}
impl ActionMemberItem<'_> {
    pub fn as_scenario_member_decl(&self) -> Option<ScenarioMemberDecl> {
        match self {
            Self::ScenarioMemberDecl(node) => Some(node.clone()),
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
impl<'a> TypedNode for ActionMemberItem<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            EVENT_DECLARATION
            | PARAMETER_DECLARATION
            | VARIABLE_DECLARATION
            | KEEP_CONSTRAINT_DECLARATION
            | REMOVE_DEFAULT_DECLARATION
            | METHOD_DECLARATION
            | COVERAGE_DECLARATION
            | MODIFIER_APPLICATION => {
                ScenarioMemberDecl::cast(node.clone()).map(Self::ScenarioMemberDecl)
            }
            ON_DIRECTIVE | DO_DIRECTIVE => {
                BehaviorSpecification::cast(node.clone()).map(Self::BehaviorSpecification)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierOfClause<'a>(OscDslNode<'a>);
impl ModifierOfClause<'_> {
    pub fn of_token(&self) -> Option<OfToken> {
        support::child(&self.0, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> Option<QualifiedBehaviorName> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierOfClause<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, MODIFIER_BODY | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            MODIFIER_BODY => ModifierBody::cast(node.clone()).map(Self::ModifierBody),
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct ModifierBody<'a>(OscDslNode<'a>);
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
    pub fn modifier_member_item_list(&self) -> Option<ModifierMemberItemList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ModifierBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ModifierMemberItemList<'a>(OscDslNode<'a>);
impl<'a> ModifierMemberItemList<'a> {
    pub fn modifier_member_item(&self) -> impl Iterator<Item = ModifierMemberItem<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ModifierMemberItemList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == MODIFIER_MEMBER_ITEM_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierMemberItem<'a> {
    ScenarioMemberDecl(ScenarioMemberDecl<'a>),
    OnDirective(OnDirective<'a>),
}
impl ModifierMemberItem<'_> {
    pub fn as_scenario_member_decl(&self) -> Option<ScenarioMemberDecl> {
        match self {
            Self::ScenarioMemberDecl(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_on_directive(&self) -> Option<OnDirective> {
        match self {
            Self::OnDirective(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ModifierMemberItem<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_DECLARATION
            | PARAMETER_DECLARATION
            | VARIABLE_DECLARATION
            | KEEP_CONSTRAINT_DECLARATION
            | REMOVE_DEFAULT_DECLARATION
            | METHOD_DECLARATION
            | COVERAGE_DECLARATION
            | MODIFIER_APPLICATION => {
                ScenarioMemberDecl::cast(node.clone()).map(Self::ScenarioMemberDecl)
            }
            ON_DIRECTIVE => OnDirective::cast(node.clone()).map(Self::OnDirective),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::OnDirective(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnDirective<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EnumTypeExtension<'a>(OscDslNode<'a>);
impl EnumTypeExtension<'_> {
    pub fn extend_token(&self) -> Option<ExtendToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_bracket_token(&self) -> Option<LeftBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn enum_member_decl_list(&self) -> Option<EnumMemberDeclList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_bracket_token(&self) -> Option<RightBracketToken> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EnumTypeExtension<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ENUM_TYPE_EXTENSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredTypeExtension<'a>(OscDslNode<'a>);
impl StructuredTypeExtension<'_> {
    pub fn extend_token(&self) -> Option<ExtendToken> {
        support::child(&self.0, 0usize)
    }
    pub fn extendable_type_name(&self) -> Option<TypeReference> {
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
    pub fn extension_member_decl_list(&self) -> Option<ExtensionMemberDeclList> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for StructuredTypeExtension<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == STRUCTURED_TYPE_EXTENSION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionMemberDeclList<'a>(OscDslNode<'a>);
impl<'a> ExtensionMemberDeclList<'a> {
    pub fn extension_member_decl(&self) -> impl Iterator<Item = ExtensionMemberDecl<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ExtensionMemberDeclList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EXTENSION_MEMBER_DECL_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionMemberDecl<'a> {
    RelaxedExtensionMemberDecl(RelaxedExtensionMemberDecl<'a>),
    BehaviorSpecification(BehaviorSpecification<'a>),
}
impl ExtensionMemberDecl<'_> {
    pub fn as_relaxed_extension_member_decl(&self) -> Option<RelaxedExtensionMemberDecl> {
        match self {
            Self::RelaxedExtensionMemberDecl(node) => Some(node.clone()),
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
impl<'a> TypedNode for ExtensionMemberDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            EVENT_DECLARATION
            | PARAMETER_DECLARATION
            | VARIABLE_DECLARATION
            | KEEP_CONSTRAINT_DECLARATION
            | REMOVE_DEFAULT_DECLARATION
            | METHOD_DECLARATION
            | COVERAGE_DECLARATION
            | MODIFIER_APPLICATION => {
                RelaxedExtensionMemberDecl::cast(node.clone()).map(Self::RelaxedExtensionMemberDecl)
            }
            ON_DIRECTIVE | DO_DIRECTIVE => {
                BehaviorSpecification::cast(node.clone()).map(Self::BehaviorSpecification)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::RelaxedExtensionMemberDecl(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelaxedExtensionMemberDecl<'a> {
    EventDeclaration(EventDeclaration<'a>),
    ParameterDeclaration(ParameterDeclaration<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    KeepConstraintDeclaration(KeepConstraintDeclaration<'a>),
    RemoveDefaultDeclaration(RemoveDefaultDeclaration<'a>),
    MethodDeclaration(MethodDeclaration<'a>),
    CoverageDeclaration(CoverageDeclaration<'a>),
    ModifierApplication(ModifierApplication<'a>),
}
impl RelaxedExtensionMemberDecl<'_> {
    pub fn as_event_declaration(&self) -> Option<EventDeclaration> {
        match self {
            Self::EventDeclaration(node) => Some(node.clone()),
            _ => None,
        }
    }
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
}
impl<'a> TypedNode for RelaxedExtensionMemberDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            EVENT_DECLARATION => EventDeclaration::cast(node.clone()).map(Self::EventDeclaration),
            PARAMETER_DECLARATION => {
                ParameterDeclaration::cast(node.clone()).map(Self::ParameterDeclaration)
            }
            VARIABLE_DECLARATION => {
                VariableDeclaration::cast(node.clone()).map(Self::VariableDeclaration)
            }
            KEEP_CONSTRAINT_DECLARATION => {
                KeepConstraintDeclaration::cast(node.clone()).map(Self::KeepConstraintDeclaration)
            }
            REMOVE_DEFAULT_DECLARATION => {
                RemoveDefaultDeclaration::cast(node.clone()).map(Self::RemoveDefaultDeclaration)
            }
            METHOD_DECLARATION => {
                MethodDeclaration::cast(node.clone()).map(Self::MethodDeclaration)
            }
            COVERAGE_DECLARATION => {
                CoverageDeclaration::cast(node.clone()).map(Self::CoverageDeclaration)
            }
            MODIFIER_APPLICATION => {
                ModifierApplication::cast(node.clone()).map(Self::ModifierApplication)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::EventDeclaration(node) => node.syntax(),
            Self::ParameterDeclaration(node) => node.syntax(),
            Self::VariableDeclaration(node) => node.syntax(),
            Self::KeepConstraintDeclaration(node) => node.syntax(),
            Self::RemoveDefaultDeclaration(node) => node.syntax(),
            Self::MethodDeclaration(node) => node.syntax(),
            Self::CoverageDeclaration(node) => node.syntax(),
            Self::ModifierApplication(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration<'a>(OscDslNode<'a>);
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
    pub fn parameter_initilizer_clause(&self) -> Option<ParameterInitilizerClause> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_with_declaration_or_newline(
        &self,
    ) -> Option<ParameterWithDeclarationOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParameterDeclaration<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct VariableDeclaration<'a>(OscDslNode<'a>);
impl VariableDeclaration<'_> {
    pub fn var_token(&self) -> Option<VarToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<FieldNameList> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn type_declarator(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
    pub fn variable_initializer(&self) -> Option<VariableInitializer> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for VariableDeclaration<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct KeepConstraintDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct RemoveDefaultDeclaration<'a>(OscDslNode<'a>);
impl RemoveDefaultDeclaration<'_> {
    pub fn remove_default_token(&self) -> Option<RemoveDefaultToken> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn parameter_reference(&self) -> Option<ParameterReference> {
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EventArgumentListSpecification<'a>(OscDslNode<'a>);
impl EventArgumentListSpecification<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_list_specification(&self) -> Option<ArgumentListSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventArgumentListSpecification<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EVENT_ARGUMENT_LIST_SPECIFICATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventIsClause<'a>(OscDslNode<'a>);
impl EventIsClause<'_> {
    pub fn is_token(&self) -> Option<IsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventIsClause<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ArgumentListSpecification<'a>(OscDslNode<'a>);
impl<'a> ArgumentListSpecification<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn argument_specification(&self) -> impl Iterator<Item = ArgumentSpecification<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ArgumentListSpecification<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_LIST_SPECIFICATION
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            EVENT_REFERENCE_SPECIFICATION
                | TERNARY_OP_EXP
                | LOGICAL_OP_EXP
                | BINARY_OP_EXP
                | UNARY_OP_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | FIELD_ACCESS
                | IT_KW
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_ENUM_VALUE_REFERENCE
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
            EVENT_REFERENCE_SPECIFICATION => EventReferenceSpecification::cast(node.clone())
                .map(Self::EventReferenceSpecification),
            TERNARY_OP_EXP
            | LOGICAL_OP_EXP
            | BINARY_OP_EXP
            | UNARY_OP_EXP
            | CAST_EXP
            | TYPE_TEST_EXP
            | ELEMENT_ACCESS
            | FUNCTION_APPLICATION
            | FIELD_ACCESS
            | IT_KW
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PARENTHESIZED_EXP
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | PHYSICAL_LITERAL
            | TRUE_KW
            | FALSE_KW
            | STRING_LITERAL
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PREFIXED_ENUM_VALUE_REFERENCE
            | LIST_CONSTRUCTOR
            | PARENTHESES_RANGE_CONSTRUCTOR
            | BRACKETS_RANGE_CONSTRUCTOR
            | RISE_EXPRESSION
            | FALL_EXPRESSION
            | ELAPSED_EXPRESSION
            | EVERY_EXPRESSION => EventCondition::cast(node.clone()).map(Self::EventCondition),
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
pub struct EventReferenceSpecification<'a>(OscDslNode<'a>);
impl EventReferenceSpecification<'_> {
    pub fn event_reference(&self) -> Option<EventReference> {
        support::child(&self.0, 0usize)
    }
    pub fn event_reference_condition(&self) -> Option<EventReferenceCondition> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventReferenceSpecification<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            TERNARY_OP_EXP
                | LOGICAL_OP_EXP
                | BINARY_OP_EXP
                | UNARY_OP_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | FIELD_ACCESS
                | IT_KW
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_ENUM_VALUE_REFERENCE
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
            TERNARY_OP_EXP
            | LOGICAL_OP_EXP
            | BINARY_OP_EXP
            | UNARY_OP_EXP
            | CAST_EXP
            | TYPE_TEST_EXP
            | ELEMENT_ACCESS
            | FUNCTION_APPLICATION
            | FIELD_ACCESS
            | IT_KW
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PARENTHESIZED_EXP
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | PHYSICAL_LITERAL
            | TRUE_KW
            | FALSE_KW
            | STRING_LITERAL
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PREFIXED_ENUM_VALUE_REFERENCE
            | LIST_CONSTRUCTOR
            | PARENTHESES_RANGE_CONSTRUCTOR
            | BRACKETS_RANGE_CONSTRUCTOR => Expression::cast(node.clone()).map(Self::Expression),
            RISE_EXPRESSION => RiseExpression::cast(node.clone()).map(Self::RiseExpression),
            FALL_EXPRESSION => FallExpression::cast(node.clone()).map(Self::FallExpression),
            ELAPSED_EXPRESSION => {
                ElapsedExpression::cast(node.clone()).map(Self::ElapsedExpression)
            }
            EVERY_EXPRESSION => EveryExpression::cast(node.clone()).map(Self::EveryExpression),
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
pub struct EventReference<'a>(OscDslNode<'a>);
impl EventReference<'_> {
    pub fn at_token(&self) -> Option<AtToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_path(&self) -> Option<EventPath> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventReference<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EventReferenceCondition<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EventFieldDecl<'a>(OscDslNode<'a>);
impl EventFieldDecl<'_> {
    pub fn as_token(&self) -> Option<AsToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EventFieldDecl<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub enum EventPath<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    PrefixedEventPath(PrefixedEventPath<'a>),
}
impl EventPath<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_event_path(&self) -> Option<PrefixedEventPath> {
        match self {
            Self::PrefixedEventPath(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for EventPath<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_EVENT_PATH
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            PREFIXED_EVENT_PATH => {
                PrefixedEventPath::cast(node.clone()).map(Self::PrefixedEventPath)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::PrefixedEventPath(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedEventPath<'a>(OscDslNode<'a>);
impl PrefixedEventPath<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PrefixedEventPath<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_EVENT_PATH
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiseExpression<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FallExpression<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ElapsedExpression<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EveryExpression<'a>(OscDslNode<'a>);
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
    pub fn trailing_every_exp_offset(&self) -> Option<TrailingEveryExpOffset> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EveryExpression<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct TrailingEveryExpOffset<'a>(OscDslNode<'a>);
impl TrailingEveryExpOffset<'_> {
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
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
impl<'a> TypedNode for TrailingEveryExpOffset<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TRAILING_EVERY_EXP_OFFSET
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldNameList<'a>(OscDslNode<'a>);
impl<'a> FieldNameList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn qualified_identifier(&self) -> impl Iterator<Item = QualifiedIdentifier<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for FieldNameList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ParameterInitilizerClause<'a>(OscDslNode<'a>);
impl ParameterInitilizerClause<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn default_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParameterInitilizerClause<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PARAMETER_INITILIZER_CLAUSE
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, PARAMETER_WITH_DECLARATION | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PARAMETER_WITH_DECLARATION => {
                ParameterWithDeclaration::cast(node.clone()).map(Self::ParameterWithDeclaration)
            }
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct VariableInitializer<'a>(OscDslNode<'a>);
impl VariableInitializer<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn variable_initializer_value(&self) -> Option<VariableInitializerValue> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for VariableInitializer<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == VARIABLE_INITIALIZER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableInitializerValue<'a> {
    Expression(Expression<'a>),
    SampleExpression(SampleExpression<'a>),
}
impl VariableInitializerValue<'_> {
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
impl<'a> TypedNode for VariableInitializerValue<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            TERNARY_OP_EXP
                | LOGICAL_OP_EXP
                | BINARY_OP_EXP
                | UNARY_OP_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | FIELD_ACCESS
                | IT_KW
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PARENTHESIZED_EXP
                | INTEGER_LITERAL
                | FLOAT_LITERAL
                | PHYSICAL_LITERAL
                | TRUE_KW
                | FALSE_KW
                | STRING_LITERAL
                | IDENTIFIER
                | PREFIXED_IDENTIFIER
                | PREFIXED_ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | PARENTHESES_RANGE_CONSTRUCTOR
                | BRACKETS_RANGE_CONSTRUCTOR
                | SAMPLE_EXPRESSION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            TERNARY_OP_EXP
            | LOGICAL_OP_EXP
            | BINARY_OP_EXP
            | UNARY_OP_EXP
            | CAST_EXP
            | TYPE_TEST_EXP
            | ELEMENT_ACCESS
            | FUNCTION_APPLICATION
            | FIELD_ACCESS
            | IT_KW
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PARENTHESIZED_EXP
            | INTEGER_LITERAL
            | FLOAT_LITERAL
            | PHYSICAL_LITERAL
            | TRUE_KW
            | FALSE_KW
            | STRING_LITERAL
            | IDENTIFIER
            | PREFIXED_IDENTIFIER
            | PREFIXED_ENUM_VALUE_REFERENCE
            | LIST_CONSTRUCTOR
            | PARENTHESES_RANGE_CONSTRUCTOR
            | BRACKETS_RANGE_CONSTRUCTOR => Expression::cast(node.clone()).map(Self::Expression),
            SAMPLE_EXPRESSION => SampleExpression::cast(node.clone()).map(Self::SampleExpression),
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
pub struct SampleExpression<'a>(OscDslNode<'a>);
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
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_specification(&self) -> Option<EventSpecification> {
        support::child(&self.0, 0usize)
    }
    pub fn sample_default_value(&self) -> Option<SampleDefaultValue> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SampleExpression<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct SampleDefaultValue<'a>(OscDslNode<'a>);
impl SampleDefaultValue<'_> {
    pub fn comma_token(&self) -> Option<CommaToken> {
        support::child(&self.0, 0usize)
    }
    pub fn default_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for SampleDefaultValue<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == SAMPLE_DEFAULT_VALUE
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithDeclaration<'a>(OscDslNode<'a>);
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
    pub fn parameter_with_member(&self) -> Option<ParameterWithMember> {
        support::child(&self.0, 0usize)
    }
    pub fn dedent_token(&self) -> Option<DedentToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ParameterWithDeclaration<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => {
                ConstraintDeclaration::cast(node.clone()).map(Self::ConstraintDeclaration)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, DEFAULT_KW | HARD_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            DEFAULT_KW => DefaultToken::cast(node.clone()).map(Self::DefaultToken),
            HARD_KW => HardToken::cast(node.clone()).map(Self::HardToken),
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
pub enum ParameterReference<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    FieldAccess(FieldAccess<'a>),
}
impl ParameterReference<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<FieldAccess> {
        match self {
            Self::FieldAccess(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ParameterReference<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IDENTIFIER | PREFIXED_IDENTIFIER | FIELD_ACCESS)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            FIELD_ACCESS => FieldAccess::cast(node.clone()).map(Self::FieldAccess),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::FieldAccess(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAccess<'a>(OscDslNode<'a>);
impl FieldAccess<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn field_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for FieldAccess<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == FIELD_ACCESS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodReturnType<'a>(OscDslNode<'a>);
impl MethodReturnType<'_> {
    pub fn arrow_token(&self) -> Option<ArrowToken> {
        support::child(&self.0, 0usize)
    }
    pub fn return_type(&self) -> Option<TypeDeclarator> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodReturnType<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct MethodImplementation<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, ONLY_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            ONLY_KW => OnlyToken::cast(node.clone()).map(Self::OnlyToken),
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            METHOD_EXPRESSION_BODY | UNDEFINED_KW | METHOD_EXTERNAL_BODY
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            METHOD_EXPRESSION_BODY => {
                MethodExpressionBody::cast(node.clone()).map(Self::MethodExpressionBody)
            }
            UNDEFINED_KW => UndefinedToken::cast(node.clone()).map(Self::UndefinedToken),
            METHOD_EXTERNAL_BODY => {
                MethodExternalBody::cast(node.clone()).map(Self::MethodExternalBody)
            }
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
pub struct MethodExpressionBody<'a>(OscDslNode<'a>);
impl MethodExpressionBody<'_> {
    pub fn expression_token(&self) -> Option<ExpressionToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for MethodExpressionBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct MethodExternalBody<'a>(OscDslNode<'a>);
impl MethodExternalBody<'_> {
    pub fn external_token(&self) -> Option<ExternalToken> {
        support::child(&self.0, 0usize)
    }
    pub fn structured_identifier(&self) -> Option<StructuredIdentifier> {
        support::child(&self.0, 0usize)
    }
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
impl<'a> TypedNode for MethodExternalBody<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ArgumentList<'a>(OscDslNode<'a>);
impl ArgumentList<'_> {
    pub fn positional_argument_list(&self) -> Option<PositionalArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn named_argument_list(&self) -> Option<NamedArgumentList> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ArgumentList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, COVER_KW | RECORD_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            COVER_KW => CoverToken::cast(node.clone()).map(Self::CoverToken),
            RECORD_KW => RecordToken::cast(node.clone()).map(Self::RecordToken),
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
pub enum ActorPath<'a> {
    QualifiedIdentifier(QualifiedIdentifier<'a>),
    PrefixedActorPath(PrefixedActorPath<'a>),
}
impl ActorPath<'_> {
    pub fn as_qualified_identifier(&self) -> Option<QualifiedIdentifier> {
        match self {
            Self::QualifiedIdentifier(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_actor_path(&self) -> Option<PrefixedActorPath> {
        match self {
            Self::PrefixedActorPath(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ActorPath<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_ACTOR_PATH
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER => {
                QualifiedIdentifier::cast(node.clone()).map(Self::QualifiedIdentifier)
            }
            PREFIXED_ACTOR_PATH => {
                PrefixedActorPath::cast(node.clone()).map(Self::PrefixedActorPath)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::PrefixedActorPath(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedActorPath<'a>(OscDslNode<'a>);
impl PrefixedActorPath<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn actor_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PrefixedActorPath<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_ACTOR_PATH
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoDirective<'a>(OscDslNode<'a>);
impl DoDirective<'_> {
    pub fn do_token(&self) -> Option<DoToken> {
        support::child(&self.0, 0usize)
    }
    pub fn do_member(&self) -> Option<DoMember> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for DoDirective<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct OnMemberList<'a>(OscDslNode<'a>);
impl<'a> OnMemberList<'a> {
    pub fn on_member(&self) -> impl Iterator<Item = OnMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for OnMemberList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, CALL_DIRECTIVE | EMIT_DIRECTIVE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            CALL_DIRECTIVE => CallDirective::cast(node.clone()).map(Self::CallDirective),
            EMIT_DIRECTIVE => EmitDirective::cast(node.clone()).map(Self::EmitDirective),
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
pub struct CallDirective<'a>(OscDslNode<'a>);
impl CallDirective<'_> {
    pub fn call_token(&self) -> Option<CallToken> {
        support::child(&self.0, 0usize)
    }
    pub fn method_invocation(&self) -> Option<MethodInvocation> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for CallDirective<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EmitDirective<'a>(OscDslNode<'a>);
impl EmitDirective<'_> {
    pub fn emit_token(&self) -> Option<EmitToken> {
        support::child(&self.0, 0usize)
    }
    pub fn event_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn emit_arguments(&self) -> Option<EmitArguments> {
        support::child(&self.0, 0usize)
    }
    pub fn newline_token(&self) -> Option<NewlineToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for EmitDirective<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DoMember<'a>(OscDslNode<'a>);
impl DoMember<'_> {
    pub fn do_member_label(&self) -> Option<DoMemberLabel> {
        support::child(&self.0, 0usize)
    }
    pub fn do_member_body(&self) -> Option<DoMemberBody> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for DoMember<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct DoMemberLabel<'a>(OscDslNode<'a>);
impl DoMemberLabel<'_> {
    pub fn label_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for DoMemberLabel<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == DO_MEMBER_LABEL
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            COMPOSITION | BEHAVIOR_INVOCATION | WAIT_DIRECTIVE | EMIT_DIRECTIVE | CALL_DIRECTIVE
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            COMPOSITION => Composition::cast(node.clone()).map(Self::Composition),
            BEHAVIOR_INVOCATION => {
                BehaviorInvocation::cast(node.clone()).map(Self::BehaviorInvocation)
            }
            WAIT_DIRECTIVE => WaitDirective::cast(node.clone()).map(Self::WaitDirective),
            EMIT_DIRECTIVE => EmitDirective::cast(node.clone()).map(Self::EmitDirective),
            CALL_DIRECTIVE => CallDirective::cast(node.clone()).map(Self::CallDirective),
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
pub struct Composition<'a>(OscDslNode<'a>);
impl Composition<'_> {
    pub fn composition_operator(&self) -> Option<CompositionOperator> {
        support::child(&self.0, 0usize)
    }
    pub fn composition_arguments(&self) -> Option<CompositionArguments> {
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct BehaviorInvocation<'a>(OscDslNode<'a>);
impl BehaviorInvocation<'_> {
    pub fn behavior_path(&self) -> Option<BehaviorPath> {
        support::child(&self.0, 0usize)
    }
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn behavior_with_declaration_or_newline(&self) -> Option<BehaviorWithDeclarationOrNewline> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for BehaviorInvocation<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct WaitDirective<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, SERIAL_KW | ONE_OF_KW | PARALLEL_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            SERIAL_KW => SerialToken::cast(node.clone()).map(Self::SerialToken),
            ONE_OF_KW => OneOfToken::cast(node.clone()).map(Self::OneOfToken),
            PARALLEL_KW => ParallelToken::cast(node.clone()).map(Self::ParallelToken),
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
pub struct CompositionArguments<'a>(OscDslNode<'a>);
impl CompositionArguments<'_> {
    pub fn left_paren_token(&self) -> Option<LeftParenToken> {
        support::child(&self.0, 0usize)
    }
    pub fn unqualified_argument_list(&self) -> Option<UnqualifiedArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn right_paren_token(&self) -> Option<RightParenToken> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for CompositionArguments<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == COMPOSITION_ARGUMENTS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberList<'a>(OscDslNode<'a>);
impl<'a> DoMemberList<'a> {
    pub fn do_member(&self) -> impl Iterator<Item = DoMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for DoMemberList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct BehaviorWithDeclaration<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UnqualifiedArgumentList<'a>(OscDslNode<'a>);
impl UnqualifiedArgumentList<'_> {
    pub fn positional_argument_list(&self) -> Option<PositionalArgumentList> {
        support::child(&self.0, 0usize)
    }
    pub fn unqualified_named_argument_list(&self) -> Option<UnqualifiedNamedArgumentList> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for UnqualifiedArgumentList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNQUALIFIED_ARGUMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorPath<'a> {
    QualifiedBehaviorName(QualifiedBehaviorName<'a>),
    PrefixedBehaviorPath(PrefixedBehaviorPath<'a>),
}
impl BehaviorPath<'_> {
    pub fn as_qualified_behavior_name(&self) -> Option<QualifiedBehaviorName> {
        match self {
            Self::QualifiedBehaviorName(node) => Some(node.clone()),
            _ => None,
        }
    }
    pub fn as_prefixed_behavior_path(&self) -> Option<PrefixedBehaviorPath> {
        match self {
            Self::PrefixedBehaviorPath(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for BehaviorPath<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_BEHAVIOR_NAME | PREFIXED_BEHAVIOR_PATH
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IDENTIFIER | PREFIXED_IDENTIFIER | PREFIXED_BEHAVIOR_NAME => {
                QualifiedBehaviorName::cast(node.clone()).map(Self::QualifiedBehaviorName)
            }
            PREFIXED_BEHAVIOR_PATH => {
                PrefixedBehaviorPath::cast(node.clone()).map(Self::PrefixedBehaviorPath)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::QualifiedBehaviorName(node) => node.syntax(),
            Self::PrefixedBehaviorPath(node) => node.syntax(),
        }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, BEHAVIOR_WITH_DECLARATION | NEWLINE)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            BEHAVIOR_WITH_DECLARATION => {
                BehaviorWithDeclaration::cast(node.clone()).map(Self::BehaviorWithDeclaration)
            }
            NEWLINE => NewlineToken::cast(node.clone()).map(Self::NewlineToken),
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
pub struct PrefixedBehaviorPath<'a>(OscDslNode<'a>);
impl PrefixedBehaviorPath<'_> {
    pub fn actor_expr(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
    pub fn dot_token(&self) -> Option<DotToken> {
        support::child(&self.0, 0usize)
    }
    pub fn behavior_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for PrefixedBehaviorPath<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == PREFIXED_BEHAVIOR_PATH
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithMemberList<'a>(OscDslNode<'a>);
impl<'a> BehaviorWithMemberList<'a> {
    pub fn behavior_with_member(&self) -> impl Iterator<Item = BehaviorWithMember<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for BehaviorWithMemberList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION => {
                ConstraintDeclaration::cast(node.clone()).map(Self::ConstraintDeclaration)
            }
            MODIFIER_APPLICATION => {
                ModifierApplication::cast(node.clone()).map(Self::ModifierApplication)
            }
            UNTIL_DIRECTIVE => UntilDirective::cast(node.clone()).map(Self::UntilDirective),
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
pub struct UntilDirective<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct EmitArguments<'a>(OscDslNode<'a>);
impl EmitArguments<'_> {
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
impl<'a> TypedNode for EmitArguments<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == EMIT_ARGUMENTS
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodInvocation<'a>(OscDslNode<'a>);
impl MethodInvocation<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
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
impl<'a> TypedNode for MethodInvocation<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == METHOD_INVOCATION
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSpecification<'a>(OscDslNode<'a>);
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
    pub fn argument_initializer(&self) -> Option<ArgumentInitializer> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ArgumentSpecification<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ArgumentInitializer<'a>(OscDslNode<'a>);
impl ArgumentInitializer<'_> {
    pub fn assign_token(&self) -> Option<AssignToken> {
        support::child(&self.0, 0usize)
    }
    pub fn default_value(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for ArgumentInitializer<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == ARGUMENT_INITIALIZER
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalArgumentList<'a>(OscDslNode<'a>);
impl<'a> PositionalArgumentList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn expression(&self) -> impl Iterator<Item = Expression<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for PositionalArgumentList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == POSITIONAL_ARGUMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgumentList<'a>(OscDslNode<'a>);
impl<'a> NamedArgumentList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn named_argument(&self) -> impl Iterator<Item = NamedArgument<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for NamedArgumentList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == NAMED_ARGUMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgument<'a>(OscDslNode<'a>);
impl NamedArgument<'_> {
    pub fn argument_name(&self) -> Option<QualifiedIdentifier> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for NamedArgument<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct UnqualifiedNamedArgumentList<'a>(OscDslNode<'a>);
impl<'a> UnqualifiedNamedArgumentList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn unqualified_named_argument(
        &self,
    ) -> impl Iterator<Item = UnqualifiedNamedArgument<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for UnqualifiedNamedArgumentList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNQUALIFIED_NAMED_ARGUMENT_LIST
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedNamedArgument<'a>(OscDslNode<'a>);
impl UnqualifiedNamedArgument<'_> {
    pub fn unqualified_argument_name(&self) -> Option<IdentifierToken> {
        support::child(&self.0, 0usize)
    }
    pub fn colon_token(&self) -> Option<ColonToken> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for UnqualifiedNamedArgument<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNQUALIFIED_NAMED_ARGUMENT
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryOpExp<'a>(OscDslNode<'a>);
impl TernaryOpExp<'_> {
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
impl<'a> TypedNode for TernaryOpExp<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == TERNARY_OP_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOpExp<'a>(OscDslNode<'a>);
impl LogicalOpExp<'_> {
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
impl<'a> TypedNode for LogicalOpExp<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == LOGICAL_OP_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOpExp<'a>(OscDslNode<'a>);
impl BinaryOpExp<'_> {
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
impl<'a> TypedNode for BinaryOpExp<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == BINARY_OP_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOpExp<'a>(OscDslNode<'a>);
impl UnaryOpExp<'_> {
    pub fn unary_op(&self) -> Option<UnaryOp> {
        support::child(&self.0, 0usize)
    }
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
}
impl<'a> TypedNode for UnaryOpExp<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        value == UNARY_OP_EXP
    }
    fn cast(node: Self::Node) -> Option<Self> {
        Self::can_cast(*node.value()).then(|| Self(node))
    }
    fn syntax(&self) -> &Self::Node {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExp<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct TypeTestExp<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ElementAccess<'a>(OscDslNode<'a>);
impl ElementAccess<'_> {
    pub fn object_expr(&self) -> Option<Expression> {
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct FunctionApplication<'a>(OscDslNode<'a>);
impl FunctionApplication<'_> {
    pub fn expression(&self) -> Option<Expression> {
        support::child(&self.0, 0usize)
    }
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
impl<'a> TypedNode for FunctionApplication<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub enum ItExp<'a> {
    ItToken(ItToken<'a>),
}
impl ItExp<'_> {
    pub fn as_it_token(&self) -> Option<ItToken> {
        match self {
            Self::ItToken(node) => Some(node.clone()),
            _ => None,
        }
    }
}
impl<'a> TypedNode for ItExp<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, IT_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            IT_KW => ItToken::cast(node.clone()).map(Self::ItToken),
            _ => None,
        }
    }
    fn syntax(&self) -> &Self::Node {
        match self {
            Self::ItToken(node) => node.syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExp<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            INTEGER_LITERAL => {
                IntegerLiteralToken::cast(node.clone()).map(Self::IntegerLiteralToken)
            }
            FLOAT_LITERAL => FloatLiteralToken::cast(node.clone()).map(Self::FloatLiteralToken),
            PHYSICAL_LITERAL => PhysicalLiteral::cast(node.clone()).map(Self::PhysicalLiteral),
            TRUE_KW | FALSE_KW => BoolLiteral::cast(node.clone()).map(Self::BoolLiteral),
            STRING_LITERAL => StringLiteralToken::cast(node.clone()).map(Self::StringLiteralToken),
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
pub struct ListConstructor<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(
            value,
            PARENTHESES_RANGE_CONSTRUCTOR | BRACKETS_RANGE_CONSTRUCTOR
        )
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            PARENTHESES_RANGE_CONSTRUCTOR => ParenthesesRangeConstructor::cast(node.clone())
                .map(Self::ParenthesesRangeConstructor),
            BRACKETS_RANGE_CONSTRUCTOR => {
                BracketsRangeConstructor::cast(node.clone()).map(Self::BracketsRangeConstructor)
            }
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, FAT_ARROW | OR_KW | AND_KW)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            FAT_ARROW => FatArrowToken::cast(node.clone()).map(Self::FatArrowToken),
            OR_KW => OrToken::cast(node.clone()).map(Self::OrToken),
            AND_KW => AndToken::cast(node.clone()).map(Self::AndToken),
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
            EQUAL => EqualToken::cast(node.clone()).map(Self::EqualToken),
            NOT_EQUAL => NotEqualToken::cast(node.clone()).map(Self::NotEqualToken),
            LESS => LessToken::cast(node.clone()).map(Self::LessToken),
            LESS_EQUAL => LessEqualToken::cast(node.clone()).map(Self::LessEqualToken),
            GREATER => GreaterToken::cast(node.clone()).map(Self::GreaterToken),
            GREATER_EQUAL => GreaterEqualToken::cast(node.clone()).map(Self::GreaterEqualToken),
            IN_KW => InToken::cast(node.clone()).map(Self::InToken),
            PLUS => PlusToken::cast(node.clone()).map(Self::PlusToken),
            MINUS => MinusToken::cast(node.clone()).map(Self::MinusToken),
            STAR => StarToken::cast(node.clone()).map(Self::StarToken),
            SLASH => SlashToken::cast(node.clone()).map(Self::SlashToken),
            PERCENT => PercentToken::cast(node.clone()).map(Self::PercentToken),
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
    fn can_cast(value: Self::Value) -> bool {
        matches!(value, NOT_KW | MINUS)
    }
    fn cast(node: Self::Node) -> Option<Self> {
        match *node.value() {
            NOT_KW => NotToken::cast(node.clone()).map(Self::NotToken),
            MINUS => MinusToken::cast(node.clone()).map(Self::MinusToken),
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
pub struct ExpressionList<'a>(OscDslNode<'a>);
impl<'a> ExpressionList<'a> {
    pub fn comma_token(&self) -> impl Iterator<Item = CommaToken<'a>> + 'a {
        support::children(&self.0)
    }
    pub fn expression(&self) -> impl Iterator<Item = Expression<'a>> + 'a {
        support::children(&self.0)
    }
}
impl<'a> TypedNode for ExpressionList<'a> {
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct ParenthesesRangeConstructor<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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
pub struct BracketsRangeConstructor<'a>(OscDslNode<'a>);
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
    type Value = OscDslSyntaxKind;
    type Node = OscDslNode<'a>;
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