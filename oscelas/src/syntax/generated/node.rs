use super::OscDslSyntaxKind;
use crate::syntax::OscDslLanguage;
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxNode};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedIdentifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl QualifiedIdentifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for QualifiedIdentifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::QUALIFIED_IDENTIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::QUALIFIED_IDENTIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierPrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl IdentifierPrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for IdentifierPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::IDENTIFIER_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::IDENTIFIER_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceName {
    node: SyntaxNode<OscDslLanguage>,
}
impl NamespaceName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NamespaceName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NAMESPACE_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NAMESPACE_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolLiteral {
    node: SyntaxNode<OscDslLanguage>,
}
impl BoolLiteral {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BoolLiteral {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BOOL_LITERAL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BOOL_LITERAL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLiteral {
    node: SyntaxNode<OscDslLanguage>,
}
impl PhysicalLiteral {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PhysicalLiteral {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PHYSICAL_LITERAL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PHYSICAL_LITERAL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral {
    node: SyntaxNode<OscDslLanguage>,
}
impl NumberLiteral {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NumberLiteral {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NUMBER_LITERAL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NUMBER_LITERAL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitName {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnitName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnitName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::UNIT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNIT_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscFile {
    node: SyntaxNode<OscDslLanguage>,
}
impl OscFile {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for OscFile {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::OSC_FILE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::OSC_FILE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreludeStatementList {
    node: SyntaxNode<OscDslLanguage>,
}
impl PreludeStatementList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PreludeStatementList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PRELUDE_STATEMENT_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PRELUDE_STATEMENT_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainStatementList {
    node: SyntaxNode<OscDslLanguage>,
}
impl MainStatementList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MainStatementList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MAIN_STATEMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MAIN_STATEMENT_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreludeStatement {
    node: SyntaxNode<OscDslLanguage>,
}
impl PreludeStatement {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PreludeStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PRELUDE_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PRELUDE_STATEMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainStatement {
    node: SyntaxNode<OscDslLanguage>,
}
impl MainStatement {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MainStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MAIN_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MAIN_STATEMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportStatement {
    node: SyntaxNode<OscDslLanguage>,
}
impl ImportStatement {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ImportStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::IMPORT_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::IMPORT_STATEMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReference {
    node: SyntaxNode<OscDslLanguage>,
}
impl ImportReference {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ImportReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::IMPORT_REFERENCE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::IMPORT_REFERENCE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReferenceString {
    node: SyntaxNode<OscDslLanguage>,
}
impl ImportReferenceString {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ImportReferenceString {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::IMPORT_REFERENCE_STRING as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::IMPORT_REFERENCE_STRING
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredIdentifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructuredIdentifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructuredIdentifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::STRUCTURED_IDENTIFIER as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCTURED_IDENTIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceStatement {
    node: SyntaxNode<OscDslLanguage>,
}
impl NamespaceStatement {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NamespaceStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NAMESPACE_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NAMESPACE_STATEMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportStatement {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExportStatement {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExportStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EXPORT_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPORT_STATEMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl OscDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for OscDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::OSC_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::OSC_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceUseClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl NamespaceUseClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NamespaceUseClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NAMESPACE_USE_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NAMESPACE_USE_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceList {
    node: SyntaxNode<OscDslLanguage>,
}
impl NamespaceList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NamespaceList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NAMESPACE_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NAMESPACE_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleNamespaceName {
    node: SyntaxNode<OscDslLanguage>,
}
impl SimpleNamespaceName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SimpleNamespaceName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SIMPLE_NAMESPACE_NAME as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SIMPLE_NAMESPACE_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalNamespaceName {
    node: SyntaxNode<OscDslLanguage>,
}
impl GlobalNamespaceName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for GlobalNamespaceName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::GLOBAL_NAMESPACE_NAME as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::GLOBAL_NAMESPACE_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSpecificationList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExportSpecificationList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExportSpecificationList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EXPORT_SPECIFICATION_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPORT_SPECIFICATION_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExportSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExportSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EXPORT_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPORT_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportWildcardSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExportWildcardSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExportWildcardSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EXPORT_WILDCARD_SPECIFICATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPORT_WILDCARD_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportWildcardSpecificationPrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExportWildcardSpecificationPrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExportWildcardSpecificationPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EXPORT_WILDCARD_SPECIFICATION_PREFIX as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPORT_WILDCARD_SPECIFICATION_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTypeDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl PhysicalTypeDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PhysicalTypeDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PHYSICAL_TYPE_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PHYSICAL_TYPE_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnitDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnitDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::UNIT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNIT_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::STRUCT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTOR_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTION_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SCENARIO_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeExtension {
    node: SyntaxNode<OscDslLanguage>,
}
impl TypeExtension {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TypeExtension {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::TYPE_EXTENSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TYPE_EXTENSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalParameterDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl GlobalParameterDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for GlobalParameterDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::GLOBAL_PARAMETER_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::GLOBAL_PARAMETER_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDeclarator {
    node: SyntaxNode<OscDslLanguage>,
}
impl TypeDeclarator {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::TYPE_DECLARATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TYPE_DECLARATOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonAggregateTypeDeclarator {
    node: SyntaxNode<OscDslLanguage>,
}
impl NonAggregateTypeDeclarator {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NonAggregateTypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::NON_AGGREGATE_TYPE_DECLARATOR as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NON_AGGREGATE_TYPE_DECLARATOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateTypeDeclarator {
    node: SyntaxNode<OscDslLanguage>,
}
impl AggregateTypeDeclarator {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for AggregateTypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::AGGREGATE_TYPE_DECLARATOR as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::AGGREGATE_TYPE_DECLARATOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveType {
    node: SyntaxNode<OscDslLanguage>,
}
impl PrimitiveType {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PrimitiveType {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PRIMITIVE_TYPE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PRIMITIVE_TYPE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTypeName {
    node: SyntaxNode<OscDslLanguage>,
}
impl PhysicalTypeName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PhysicalTypeName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PHYSICAL_TYPE_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PHYSICAL_TYPE_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumName {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructName {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::STRUCT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorName {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTOR_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedBehaviorName {
    node: SyntaxNode<OscDslLanguage>,
}
impl QualifiedBehaviorName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for QualifiedBehaviorName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::QUALIFIED_BEHAVIOR_NAME as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::QUALIFIED_BEHAVIOR_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTypeDeclarator {
    node: SyntaxNode<OscDslLanguage>,
}
impl ListTypeDeclarator {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ListTypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::LIST_TYPE_DECLARATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::LIST_TYPE_DECLARATOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseUnitSpecifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl BaseUnitSpecifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BaseUnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BASE_UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BASE_UNIT_SPECIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitSpecifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnitSpecifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNIT_SPECIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseUnitSpecifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl SiBaseUnitSpecifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SiBaseUnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SI_BASE_UNIT_SPECIFIER as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SI_BASE_UNIT_SPECIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiUnitSpecifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl SiUnitSpecifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SiUnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SI_UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SI_UNIT_SPECIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseExponentList {
    node: SyntaxNode<OscDslLanguage>,
}
impl SiBaseExponentList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SiBaseExponentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SI_BASE_EXPONENT_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SI_BASE_EXPONENT_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseExponent {
    node: SyntaxNode<OscDslLanguage>,
}
impl SiBaseExponent {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SiBaseExponent {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SI_BASE_EXPONENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SI_BASE_EXPONENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseUnitName {
    node: SyntaxNode<OscDslLanguage>,
}
impl SiBaseUnitName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SiBaseUnitName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SI_BASE_UNIT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SI_BASE_UNIT_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingSiFactor {
    node: SyntaxNode<OscDslLanguage>,
}
impl TrailingSiFactor {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TrailingSiFactor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::TRAILING_SI_FACTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TRAILING_SI_FACTOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingSiOffset {
    node: SyntaxNode<OscDslLanguage>,
}
impl TrailingSiOffset {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TrailingSiOffset {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::TRAILING_SI_OFFSET as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TRAILING_SI_OFFSET
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDeclList {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberDeclList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ENUM_MEMBER_DECL_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_MEMBER_DECL_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDecl {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberDecl {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_MEMBER_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_MEMBER_DECL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberName {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumMemberName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_MEMBER_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_MEMBER_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberInitializer {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberInitializer {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumMemberInitializer {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ENUM_MEMBER_INITIALIZER as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_MEMBER_INITIALIZER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberValue {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberValue {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumMemberValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_MEMBER_VALUE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_MEMBER_VALUE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueReference {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumValueReference {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumValueReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_VALUE_REFERENCE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_VALUE_REFERENCE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueReferencePrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumValueReferencePrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumValueReferencePrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ENUM_VALUE_REFERENCE_PREFIX as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_VALUE_REFERENCE_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructInheritsClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::STRUCT_INHERITS_CLAUSE as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_INHERITS_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::STRUCT_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsCondition {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructInheritsCondition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::STRUCT_INHERITS_CONDITION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_INHERITS_CONDITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldName {
    node: SyntaxNode<OscDslLanguage>,
}
impl FieldName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FieldName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FIELD_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FIELD_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsConstant {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructInheritsConstant {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::STRUCT_INHERITS_CONSTANT as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_INHERITS_CONSTANT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructMemberDeclList {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructMemberDeclList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::STRUCT_MEMBER_DECL_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_MEMBER_DECL_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructMemberDecl {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructMemberDecl {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::STRUCT_MEMBER_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCT_MEMBER_DECL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl FieldDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FieldDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FIELD_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FIELD_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ConstraintDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ConstraintDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::CONSTRAINT_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::CONSTRAINT_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl CoverageDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for CoverageDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::COVERAGE_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::COVERAGE_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorInheritsClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTOR_INHERITS_CLAUSE as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_INHERITS_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTOR_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsCondition {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorInheritsCondition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTOR_INHERITS_CONDITION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_INHERITS_CONDITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsConstant {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorInheritsConstant {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTOR_INHERITS_CONSTANT as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_INHERITS_CONSTANT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMemberDeclList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorMemberDeclList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTOR_MEMBER_DECL_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_MEMBER_DECL_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMemberDecl {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorMemberDecl {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTOR_MEMBER_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_MEMBER_DECL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioInheritsClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SCENARIO_INHERITS_CLAUSE as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_INHERITS_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SCENARIO_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsCondition {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioInheritsCondition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SCENARIO_INHERITS_CONDITION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_INHERITS_CONDITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsConstant {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioInheritsConstant {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SCENARIO_INHERITS_CONSTANT as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_INHERITS_CONSTANT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioMemberItemList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioMemberItemList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioMemberItemList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::SCENARIO_MEMBER_ITEM_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_MEMBER_ITEM_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioMemberItem {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioMemberItem {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioMemberItem {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SCENARIO_MEMBER_ITEM as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_MEMBER_ITEM
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioMemberDecl {
    node: SyntaxNode<OscDslLanguage>,
}
impl ScenarioMemberDecl {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ScenarioMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SCENARIO_MEMBER_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SCENARIO_MEMBER_DECL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::BEHAVIOR_SPECIFICATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedBehaviorNamePrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl QualifiedBehaviorNamePrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for QualifiedBehaviorNamePrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::QUALIFIED_BEHAVIOR_NAME_PREFIX as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::QUALIFIED_BEHAVIOR_NAME_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorName {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BEHAVIOR_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionInheritsClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTION_INHERITS_CLAUSE as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_INHERITS_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTION_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsCondition {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionInheritsCondition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTION_INHERITS_CONDITION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_INHERITS_CONDITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsConstant {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionInheritsConstant {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTION_INHERITS_CONSTANT as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_INHERITS_CONSTANT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMemberItemList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionMemberItemList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionMemberItemList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ACTION_MEMBER_ITEM_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_MEMBER_ITEM_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMemberItem {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActionMemberItem {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActionMemberItem {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTION_MEMBER_ITEM as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTION_MEMBER_ITEM
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierNamePrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierNamePrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierNamePrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_NAME_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_NAME_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierName {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierOfClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierOfClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierOfClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_OF_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_OF_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierMemberItemList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierMemberItemList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierMemberItemList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::MODIFIER_MEMBER_ITEM_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_MEMBER_ITEM_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierMemberItem {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierMemberItem {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierMemberItem {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_MEMBER_ITEM as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_MEMBER_ITEM
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnDirective {
    node: SyntaxNode<OscDslLanguage>,
}
impl OnDirective {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for OnDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ON_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ON_DIRECTIVE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumTypeExtension {
    node: SyntaxNode<OscDslLanguage>,
}
impl EnumTypeExtension {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EnumTypeExtension {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ENUM_TYPE_EXTENSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ENUM_TYPE_EXTENSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredTypeExtension {
    node: SyntaxNode<OscDslLanguage>,
}
impl StructuredTypeExtension {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StructuredTypeExtension {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::STRUCTURED_TYPE_EXTENSION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRUCTURED_TYPE_EXTENSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendableTypeName {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExtendableTypeName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExtendableTypeName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EXTENDABLE_TYPE_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXTENDABLE_TYPE_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendableMemberDeclList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExtendableMemberDeclList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExtendableMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EXTENDABLE_MEMBER_DECL_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXTENDABLE_MEMBER_DECL_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendableMemberDecl {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExtendableMemberDecl {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExtendableMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EXTENDABLE_MEMBER_DECL as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXTENDABLE_MEMBER_DECL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParameterDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParameterDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PARAMETER_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARAMETER_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventName {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventArgumentListSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventArgumentListSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventArgumentListSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EVENT_ARGUMENT_LIST_SPECIFICATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_ARGUMENT_LIST_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventIsClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventIsClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventIsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_IS_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_IS_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentListSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl ArgumentListSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ArgumentListSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ARGUMENT_LIST_SPECIFICATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ARGUMENT_LIST_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReferenceSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventReferenceSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventReferenceSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EVENT_REFERENCE_SPECIFICATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_REFERENCE_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventCondition {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventCondition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_CONDITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_CONDITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReference {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventReference {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_REFERENCE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_REFERENCE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReferenceCondition {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventReferenceCondition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventReferenceCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::EVENT_REFERENCE_CONDITION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_REFERENCE_CONDITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFieldDecl {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventFieldDecl {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventFieldDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_FIELD_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_FIELD_DECL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventPath {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventPath {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventPath {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_PATH as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_PATH
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFieldName {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventFieldName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventFieldName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_FIELD_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_FIELD_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventPathPrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl EventPathPrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EventPathPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVENT_PATH_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVENT_PATH_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    node: SyntaxNode<OscDslLanguage>,
}
impl Expression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for Expression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl BoolExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BoolExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BOOL_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BOOL_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiseExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl RiseExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for RiseExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::RISE_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::RISE_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl FallExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FallExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FALL_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FALL_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElapsedExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl ElapsedExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ElapsedExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ELAPSED_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ELAPSED_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl EveryExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EveryExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EVERY_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EVERY_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl DurationExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DurationExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DURATION_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DURATION_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingEveryExpOffset {
    node: SyntaxNode<OscDslLanguage>,
}
impl TrailingEveryExpOffset {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TrailingEveryExpOffset {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::TRAILING_EVERY_EXP_OFFSET as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TRAILING_EVERY_EXP_OFFSET
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl VariableDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for VariableDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::VARIABLE_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::VARIABLE_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldNameList {
    node: SyntaxNode<OscDslLanguage>,
}
impl FieldNameList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FieldNameList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FIELD_NAME_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FIELD_NAME_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterInitilizerClause {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParameterInitilizerClause {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParameterInitilizerClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PARAMETER_INITILIZER_CLAUSE as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARAMETER_INITILIZER_CLAUSE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParameterWithDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParameterWithDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PARAMETER_WITH_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARAMETER_WITH_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultValue {
    node: SyntaxNode<OscDslLanguage>,
}
impl DefaultValue {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DefaultValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DEFAULT_VALUE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DEFAULT_VALUE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableInitializer {
    node: SyntaxNode<OscDslLanguage>,
}
impl VariableInitializer {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for VariableInitializer {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::VARIABLE_INITIALIZER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::VARIABLE_INITIALIZER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableInitializerValue {
    node: SyntaxNode<OscDslLanguage>,
}
impl VariableInitializerValue {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for VariableInitializerValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::VARIABLE_INITIALIZER_VALUE as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::VARIABLE_INITIALIZER_VALUE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl SampleExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SampleExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SAMPLE_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SAMPLE_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleDefaultValue {
    node: SyntaxNode<OscDslLanguage>,
}
impl SampleDefaultValue {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SampleDefaultValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SAMPLE_DEFAULT_VALUE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SAMPLE_DEFAULT_VALUE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithMember {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParameterWithMember {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParameterWithMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PARAMETER_WITH_MEMBER as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARAMETER_WITH_MEMBER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepConstraintDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl KeepConstraintDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for KeepConstraintDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::KEEP_CONSTRAINT_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::KEEP_CONSTRAINT_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDefaultDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl RemoveDefaultDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for RemoveDefaultDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::REMOVE_DEFAULT_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::REMOVE_DEFAULT_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintQualifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl ConstraintQualifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ConstraintQualifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::CONSTRAINT_QUALIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::CONSTRAINT_QUALIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl ConstraintExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ConstraintExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::CONSTRAINT_EXPRESSION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::CONSTRAINT_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterReference {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParameterReference {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParameterReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PARAMETER_REFERENCE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARAMETER_REFERENCE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAccess {
    node: SyntaxNode<OscDslLanguage>,
}
impl FieldAccess {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FieldAccess {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FIELD_ACCESS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FIELD_ACCESS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodName {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodReturnType {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodReturnType {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodReturnType {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_RETURN_TYPE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_RETURN_TYPE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodImplementation {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodImplementation {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodImplementation {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::METHOD_IMPLEMENTATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_IMPLEMENTATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnType {
    node: SyntaxNode<OscDslLanguage>,
}
impl ReturnType {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ReturnType {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::RETURN_TYPE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::RETURN_TYPE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodQualifier {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodQualifier {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodQualifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_QUALIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_QUALIFIER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExpressionBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodExpressionBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodExpressionBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::METHOD_EXPRESSION_BODY as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_EXPRESSION_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodUndefinedBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodUndefinedBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodUndefinedBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::METHOD_UNDEFINED_BODY as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_UNDEFINED_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExternalBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodExternalBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodExternalBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_EXTERNAL_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_EXTERNAL_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments {
    node: SyntaxNode<OscDslLanguage>,
}
impl Arguments {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for Arguments {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ARGUMENTS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ARGUMENTS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierApplication {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierApplication {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierApplication {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::MODIFIER_APPLICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_APPLICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierApplicationPrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl ModifierApplicationPrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ModifierApplicationPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::MODIFIER_APPLICATION_PREFIX as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::MODIFIER_APPLICATION_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorExpression {
    node: SyntaxNode<OscDslLanguage>,
}
impl ActorExpression {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ActorExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ACTOR_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ACTOR_EXPRESSION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoDirective {
    node: SyntaxNode<OscDslLanguage>,
}
impl DoDirective {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DoDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DO_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DO_DIRECTIVE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnMemberList {
    node: SyntaxNode<OscDslLanguage>,
}
impl OnMemberList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for OnMemberList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ON_MEMBER_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ON_MEMBER_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnMember {
    node: SyntaxNode<OscDslLanguage>,
}
impl OnMember {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for OnMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ON_MEMBER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ON_MEMBER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallDirective {
    node: SyntaxNode<OscDslLanguage>,
}
impl CallDirective {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for CallDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::CALL_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::CALL_DIRECTIVE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitDirective {
    node: SyntaxNode<OscDslLanguage>,
}
impl EmitDirective {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EmitDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EMIT_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EMIT_DIRECTIVE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMember {
    node: SyntaxNode<OscDslLanguage>,
}
impl DoMember {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DoMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DO_MEMBER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DO_MEMBER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberLabel {
    node: SyntaxNode<OscDslLanguage>,
}
impl DoMemberLabel {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DoMemberLabel {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DO_MEMBER_LABEL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DO_MEMBER_LABEL
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberBody {
    node: SyntaxNode<OscDslLanguage>,
}
impl DoMemberBody {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DoMemberBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DO_MEMBER_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DO_MEMBER_BODY
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelName {
    node: SyntaxNode<OscDslLanguage>,
}
impl LabelName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for LabelName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::LABEL_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::LABEL_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composition {
    node: SyntaxNode<OscDslLanguage>,
}
impl Composition {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for Composition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::COMPOSITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::COMPOSITION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorInvocation {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorInvocation {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorInvocation {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BEHAVIOR_INVOCATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_INVOCATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitDirective {
    node: SyntaxNode<OscDslLanguage>,
}
impl WaitDirective {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for WaitDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::WAIT_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::WAIT_DIRECTIVE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionOperator {
    node: SyntaxNode<OscDslLanguage>,
}
impl CompositionOperator {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for CompositionOperator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::COMPOSITION_OPERATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::COMPOSITION_OPERATOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionArguments {
    node: SyntaxNode<OscDslLanguage>,
}
impl CompositionArguments {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for CompositionArguments {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::COMPOSITION_ARGUMENTS as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::COMPOSITION_ARGUMENTS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberList {
    node: SyntaxNode<OscDslLanguage>,
}
impl DoMemberList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for DoMemberList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::DO_MEMBER_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::DO_MEMBER_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithDeclaration {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorWithDeclaration {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorWithDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::BEHAVIOR_WITH_DECLARATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_WITH_DECLARATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedArguments {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedArguments {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnqualifiedArguments {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::UNQUALIFIED_ARGUMENTS as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNQUALIFIED_ARGUMENTS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorInvocationPrefix {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorInvocationPrefix {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorInvocationPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::BEHAVIOR_INVOCATION_PREFIX as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_INVOCATION_PREFIX
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithMemberList {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorWithMemberList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorWithMemberList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::BEHAVIOR_WITH_MEMBER_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_WITH_MEMBER_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithMember {
    node: SyntaxNode<OscDslLanguage>,
}
impl BehaviorWithMember {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BehaviorWithMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BEHAVIOR_WITH_MEMBER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BEHAVIOR_WITH_MEMBER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntilDirective {
    node: SyntaxNode<OscDslLanguage>,
}
impl UntilDirective {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UntilDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::UNTIL_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNTIL_DIRECTIVE
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitArguments {
    node: SyntaxNode<OscDslLanguage>,
}
impl EmitArguments {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for EmitArguments {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EMIT_ARGUMENTS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EMIT_ARGUMENTS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodInvocation {
    node: SyntaxNode<OscDslLanguage>,
}
impl MethodInvocation {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for MethodInvocation {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::METHOD_INVOCATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::METHOD_INVOCATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSpecification {
    node: SyntaxNode<OscDslLanguage>,
}
impl ArgumentSpecification {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ArgumentSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::ARGUMENT_SPECIFICATION as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ARGUMENT_SPECIFICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentName {
    node: SyntaxNode<OscDslLanguage>,
}
impl ArgumentName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ArgumentName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ARGUMENT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ARGUMENT_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentInitializer {
    node: SyntaxNode<OscDslLanguage>,
}
impl ArgumentInitializer {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ArgumentInitializer {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ARGUMENT_INITIALIZER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ARGUMENT_INITIALIZER
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalArgumentList {
    node: SyntaxNode<OscDslLanguage>,
}
impl PositionalArgumentList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PositionalArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::POSITIONAL_ARGUMENT_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::POSITIONAL_ARGUMENT_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgumentList {
    node: SyntaxNode<OscDslLanguage>,
}
impl NamedArgumentList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NamedArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NAMED_ARGUMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NAMED_ARGUMENT_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalArgument {
    node: SyntaxNode<OscDslLanguage>,
}
impl PositionalArgument {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PositionalArgument {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::POSITIONAL_ARGUMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::POSITIONAL_ARGUMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgument {
    node: SyntaxNode<OscDslLanguage>,
}
impl NamedArgument {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for NamedArgument {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::NAMED_ARGUMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::NAMED_ARGUMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedNamedArgumentList {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedNamedArgumentList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnqualifiedNamedArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::UNQUALIFIED_NAMED_ARGUMENT_LIST as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNQUALIFIED_NAMED_ARGUMENT_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedNamedArgument {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedNamedArgument {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnqualifiedNamedArgument {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::UNQUALIFIED_NAMED_ARGUMENT as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNQUALIFIED_NAMED_ARGUMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedArgumentName {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedArgumentName {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnqualifiedArgumentName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::UNQUALIFIED_ARGUMENT_NAME as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNQUALIFIED_ARGUMENT_NAME
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryOpExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl TernaryOpExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TernaryOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::TERNARY_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TERNARY_OP_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOpExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl LogicalOpExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for LogicalOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::LOGICAL_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::LOGICAL_OP_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOpExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl BinaryOpExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BinaryOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BINARY_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BINARY_OP_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOpExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnaryOpExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnaryOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::UNARY_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNARY_OP_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl CastExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for CastExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::CAST_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::CAST_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTestExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl TypeTestExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for TypeTestExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::TYPE_TEST_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::TYPE_TEST_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementAccess {
    node: SyntaxNode<OscDslLanguage>,
}
impl ElementAccess {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ElementAccess {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::ELEMENT_ACCESS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::ELEMENT_ACCESS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionApplication {
    node: SyntaxNode<OscDslLanguage>,
}
impl FunctionApplication {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FunctionApplication {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FUNCTION_APPLICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FUNCTION_APPLICATION
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl ItExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ItExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::IT_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::IT_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParenthesizedExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParenthesizedExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PARENTHESIZED_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARENTHESIZED_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl LiteralExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for LiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::LITERAL_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListConstructor {
    node: SyntaxNode<OscDslLanguage>,
}
impl ListConstructor {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ListConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::LIST_CONSTRUCTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::LIST_CONSTRUCTOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeConstructor {
    node: SyntaxNode<OscDslLanguage>,
}
impl RangeConstructor {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for RangeConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::RANGE_CONSTRUCTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::RANGE_CONSTRUCTOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOp {
    node: SyntaxNode<OscDslLanguage>,
}
impl LogicalOp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for LogicalOp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::LOGICAL_OP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::LOGICAL_OP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    node: SyntaxNode<OscDslLanguage>,
}
impl BinaryOp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BinaryOp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BINARY_OP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BINARY_OP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOp {
    node: SyntaxNode<OscDslLanguage>,
}
impl UnaryOp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for UnaryOp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::UNARY_OP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::UNARY_OP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteralExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl IntegerLiteralExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for IntegerLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::INTEGER_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::INTEGER_LITERAL_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatLiteralExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl FloatLiteralExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for FloatLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::FLOAT_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::FLOAT_LITERAL_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLiteralExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl PhysicalLiteralExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for PhysicalLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::PHYSICAL_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PHYSICAL_LITERAL_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolLiteralExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl BoolLiteralExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BoolLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BOOL_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BOOL_LITERAL_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteralExp {
    node: SyntaxNode<OscDslLanguage>,
}
impl StringLiteralExp {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for StringLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::STRING_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::STRING_LITERAL_EXP
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionList {
    node: SyntaxNode<OscDslLanguage>,
}
impl ExpressionList {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ExpressionList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::EXPRESSION_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::EXPRESSION_LIST
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesRangeConstructor {
    node: SyntaxNode<OscDslLanguage>,
}
impl ParenthesesRangeConstructor {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for ParenthesesRangeConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::PARENTHESES_RANGE_CONSTRUCTOR as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::PARENTHESES_RANGE_CONSTRUCTOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketsRangeConstructor {
    node: SyntaxNode<OscDslLanguage>,
}
impl BracketsRangeConstructor {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for BracketsRangeConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        OscDslSyntaxKind::BRACKETS_RANGE_CONSTRUCTOR as u16,
    ));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BRACKETS_RANGE_CONSTRUCTOR
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxElement {
    node: SyntaxNode<OscDslLanguage>,
}
impl SyntaxElement {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for SyntaxElement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::SYNTAX_ELEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::SYNTAX_ELEMENT
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bogus {
    node: SyntaxNode<OscDslLanguage>,
}
impl Bogus {
    pub const unsafe fn new_unchecked(node: SyntaxNode<OscDslLanguage>) -> Self {
        Self { node }
    }
}
impl AstNode for Bogus {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OscDslSyntaxKind::BOGUS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OscDslSyntaxKind::BOGUS
    }
    fn cast(node: SyntaxNode<OscDslLanguage>) -> Option<Self> {
        Self::can_cast(node.kind()).then(|| unsafe { Self::new_unchecked(node) })
    }
    fn syntax(&self) -> &SyntaxNode<OscDslLanguage> {
        &self.node
    }
    fn into_syntax(self) -> SyntaxNode<OscDslLanguage> {
        self.node
    }
}