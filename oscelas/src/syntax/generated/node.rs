use super::OscDslSyntaxKind::{self, *};
use crate::syntax::OscDslLanguage;
use biome_rowan::{
    support, AstNode, AstNodeList, AstSeparatedList, RawSyntaxKind, SyntaxKindSet, SyntaxList,
    SyntaxNode, SyntaxResult, SyntaxToken,
};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxElement {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SyntaxElement {
    pub fn syntax_element(&self) -> SyntaxResult<SyntaxElement> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for SyntaxElement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SYNTAX_ELEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SYNTAX_ELEMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bogus {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl Bogus {
    pub fn syntax_element(&self) -> SyntaxResult<SyntaxElement> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for Bogus {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BOGUS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BOGUS
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Empty {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl Empty {
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for Empty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EMPTY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EMPTY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedIdentifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl QualifiedIdentifier {
    pub fn identifier_prefix(&self) -> Option<IdentifierPrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn identifier_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for QualifiedIdentifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(QUALIFIED_IDENTIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == QUALIFIED_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifierPrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl IdentifierPrefix {
    pub fn namespace_name(&self) -> Option<NamespaceName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn colon_colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for IdentifierPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(IDENTIFIER_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == IDENTIFIER_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceName {
    SimpleNamespaceName(SimpleNamespaceName),
    GlobalNamespaceName(GlobalNamespaceName),
}
impl NamespaceName {
    pub fn as_simple_namespace_name(&self) -> Option<&SimpleNamespaceName> {
        match self {
            NamespaceName::SimpleNamespaceName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_global_namespace_name(&self) -> Option<&GlobalNamespaceName> {
        match self {
            NamespaceName::GlobalNamespaceName(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for NamespaceName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SimpleNamespaceName::KIND_SET.union(GlobalNamespaceName::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, SIMPLE_NAMESPACE_NAME | GLOBAL_NAMESPACE_NAME)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            SIMPLE_NAMESPACE_NAME => Self::SimpleNamespaceName(SimpleNamespaceName::cast(syntax)?),
            GLOBAL_NAMESPACE_NAME => Self::GlobalNamespaceName(GlobalNamespaceName::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::SimpleNamespaceName(node) => node.syntax(),
            Self::GlobalNamespaceName(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::SimpleNamespaceName(node) => node.into_syntax(),
            Self::GlobalNamespaceName(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoolLiteralKind {
    True,
    False,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolLiteral {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BoolLiteral {
    pub fn kind(&self) -> BoolLiteralKind {
        match self.syntax.kind() {
            TRUE_KW => BoolLiteralKind::True,
            FALSE_KW => BoolLiteralKind::False,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for BoolLiteral {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BOOL_LITERAL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BOOL_LITERAL
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLiteral {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PhysicalLiteral {
    pub fn number_literal(&self) -> SyntaxResult<NumberLiteral> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn unit_name(&self) -> SyntaxResult<UnitName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for PhysicalLiteral {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PHYSICAL_LITERAL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PHYSICAL_LITERAL
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum NumberLiteralKind {
    FloatLiteral,
    IntegerLiteral,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl NumberLiteral {
    pub fn kind(&self) -> NumberLiteralKind {
        match self.syntax.kind() {
            FLOAT_LITERAL => NumberLiteralKind::FloatLiteral,
            INTEGER_LITERAL => NumberLiteralKind::IntegerLiteral,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for NumberLiteral {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(NUMBER_LITERAL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == NUMBER_LITERAL
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnitName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for UnitName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNIT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNIT_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OscFile {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl OscFile {
    pub fn prelude_statement_list(&self) -> PreludeStatementList {
        support::list(&self.syntax, 0usize)
    }
    pub fn main_statement_list(&self) -> MainStatementList {
        support::list(&self.syntax, 1usize)
    }
}
impl AstNode for OscFile {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(OSC_FILE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == OSC_FILE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreludeStatementList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for PreludeStatementList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PRELUDE_STATEMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PRELUDE_STATEMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for PreludeStatementList {
    type Language = OscDslLanguage;
    type Node = PreludeStatement;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MainStatementList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for MainStatementList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MAIN_STATEMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MAIN_STATEMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for MainStatementList {
    type Language = OscDslLanguage;
    type Node = MainStatement;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreludeStatement {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PreludeStatement {
    pub fn import_statement(&self) -> SyntaxResult<ImportStatement> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for PreludeStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PRELUDE_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PRELUDE_STATEMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MainStatement {
    NamespaceStatement(NamespaceStatement),
    ExportStatement(ExportStatement),
    OscDeclaration(OscDeclaration),
}
impl MainStatement {
    pub fn as_namespace_statement(&self) -> Option<&NamespaceStatement> {
        match self {
            MainStatement::NamespaceStatement(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_export_statement(&self) -> Option<&ExportStatement> {
        match self {
            MainStatement::ExportStatement(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_osc_declaration(&self) -> Option<&OscDeclaration> {
        match self {
            MainStatement::OscDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for MainStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = NamespaceStatement::KIND_SET
        .union(ExportStatement::KIND_SET)
        .union(OscDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            NAMESPACE_STATEMENT | EXPORT_STATEMENT | OSC_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            NAMESPACE_STATEMENT => Self::NamespaceStatement(NamespaceStatement::cast(syntax)?),
            EXPORT_STATEMENT => Self::ExportStatement(ExportStatement::cast(syntax)?),
            OSC_DECLARATION => Self::OscDeclaration(OscDeclaration::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::NamespaceStatement(node) => node.syntax(),
            Self::ExportStatement(node) => node.syntax(),
            Self::OscDeclaration(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::NamespaceStatement(node) => node.into_syntax(),
            Self::ExportStatement(node) => node.into_syntax(),
            Self::OscDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportStatement {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ImportStatement {
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn import_reference(&self) -> SyntaxResult<ImportReference> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for ImportStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(IMPORT_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == IMPORT_STATEMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportReference {
    ImportReferenceString(ImportReferenceString),
    StructuredIdentifier(StructuredIdentifier),
}
impl ImportReference {
    pub fn as_import_reference_string(&self) -> Option<&ImportReferenceString> {
        match self {
            ImportReference::ImportReferenceString(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_structured_identifier(&self) -> Option<&StructuredIdentifier> {
        match self {
            ImportReference::StructuredIdentifier(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ImportReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ImportReferenceString::KIND_SET.union(StructuredIdentifier::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, IMPORT_REFERENCE_STRING | STRUCTURED_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            IMPORT_REFERENCE_STRING => {
                Self::ImportReferenceString(ImportReferenceString::cast(syntax)?)
            }
            STRUCTURED_IDENTIFIER => {
                Self::StructuredIdentifier(StructuredIdentifier::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ImportReferenceString(node) => node.syntax(),
            Self::StructuredIdentifier(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ImportReferenceString(node) => node.into_syntax(),
            Self::StructuredIdentifier(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReferenceString {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ImportReferenceString {
    pub fn string_literal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for ImportReferenceString {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(IMPORT_REFERENCE_STRING as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == IMPORT_REFERENCE_STRING
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredIdentifier {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for StructuredIdentifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCTURED_IDENTIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCTURED_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for StructuredIdentifier {
    type Language = OscDslLanguage;
    type Node = StructuredIdentifierElement;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredIdentifierElement {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructuredIdentifierElement {
    pub fn identifier_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for StructuredIdentifierElement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCTURED_IDENTIFIER_ELEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCTURED_IDENTIFIER_ELEMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceStatement {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl NamespaceStatement {
    pub fn namespace_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn namespace_name(&self) -> SyntaxResult<NamespaceName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn namespace_use_clause(&self) -> Option<NamespaceUseClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for NamespaceStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(NAMESPACE_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == NAMESPACE_STATEMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportStatement {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ExportStatement {
    pub fn export_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn export_specification_list(&self) -> ExportSpecificationList {
        support::list(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for ExportStatement {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EXPORT_STATEMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EXPORT_STATEMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OscDeclaration {
    PhysicalTypeDeclaration(PhysicalTypeDeclaration),
    UnitDeclaration(UnitDeclaration),
    EnumDeclaration(EnumDeclaration),
    StructDeclaration(StructDeclaration),
    ActorDeclaration(ActorDeclaration),
    ActionDeclaration(ActionDeclaration),
    ScenarioDeclaration(ScenarioDeclaration),
    ModifierDeclaration(ModifierDeclaration),
    TypeExtension(TypeExtension),
    GlobalParameterDeclaration(GlobalParameterDeclaration),
}
impl OscDeclaration {
    pub fn as_physical_type_declaration(&self) -> Option<&PhysicalTypeDeclaration> {
        match self {
            OscDeclaration::PhysicalTypeDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_unit_declaration(&self) -> Option<&UnitDeclaration> {
        match self {
            OscDeclaration::UnitDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_enum_declaration(&self) -> Option<&EnumDeclaration> {
        match self {
            OscDeclaration::EnumDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_struct_declaration(&self) -> Option<&StructDeclaration> {
        match self {
            OscDeclaration::StructDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_actor_declaration(&self) -> Option<&ActorDeclaration> {
        match self {
            OscDeclaration::ActorDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_action_declaration(&self) -> Option<&ActionDeclaration> {
        match self {
            OscDeclaration::ActionDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_scenario_declaration(&self) -> Option<&ScenarioDeclaration> {
        match self {
            OscDeclaration::ScenarioDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_modifier_declaration(&self) -> Option<&ModifierDeclaration> {
        match self {
            OscDeclaration::ModifierDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_type_extension(&self) -> Option<&TypeExtension> {
        match self {
            OscDeclaration::TypeExtension(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_global_parameter_declaration(&self) -> Option<&GlobalParameterDeclaration> {
        match self {
            OscDeclaration::GlobalParameterDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for OscDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = PhysicalTypeDeclaration::KIND_SET
        .union(UnitDeclaration::KIND_SET)
        .union(EnumDeclaration::KIND_SET)
        .union(StructDeclaration::KIND_SET)
        .union(ActorDeclaration::KIND_SET)
        .union(ActionDeclaration::KIND_SET)
        .union(ScenarioDeclaration::KIND_SET)
        .union(ModifierDeclaration::KIND_SET)
        .union(TypeExtension::KIND_SET)
        .union(GlobalParameterDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
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
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            PHYSICAL_TYPE_DECLARATION => {
                Self::PhysicalTypeDeclaration(PhysicalTypeDeclaration::cast(syntax)?)
            }
            UNIT_DECLARATION => Self::UnitDeclaration(UnitDeclaration::cast(syntax)?),
            ENUM_DECLARATION => Self::EnumDeclaration(EnumDeclaration::cast(syntax)?),
            STRUCT_DECLARATION => Self::StructDeclaration(StructDeclaration::cast(syntax)?),
            ACTOR_DECLARATION => Self::ActorDeclaration(ActorDeclaration::cast(syntax)?),
            ACTION_DECLARATION => Self::ActionDeclaration(ActionDeclaration::cast(syntax)?),
            SCENARIO_DECLARATION => Self::ScenarioDeclaration(ScenarioDeclaration::cast(syntax)?),
            MODIFIER_DECLARATION => Self::ModifierDeclaration(ModifierDeclaration::cast(syntax)?),
            TYPE_EXTENSION => Self::TypeExtension(TypeExtension::cast(syntax)?),
            GLOBAL_PARAMETER_DECLARATION => {
                Self::GlobalParameterDeclaration(GlobalParameterDeclaration::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
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
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::PhysicalTypeDeclaration(node) => node.into_syntax(),
            Self::UnitDeclaration(node) => node.into_syntax(),
            Self::EnumDeclaration(node) => node.into_syntax(),
            Self::StructDeclaration(node) => node.into_syntax(),
            Self::ActorDeclaration(node) => node.into_syntax(),
            Self::ActionDeclaration(node) => node.into_syntax(),
            Self::ScenarioDeclaration(node) => node.into_syntax(),
            Self::ModifierDeclaration(node) => node.into_syntax(),
            Self::TypeExtension(node) => node.into_syntax(),
            Self::GlobalParameterDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceUseClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl NamespaceUseClause {
    pub fn use_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn namespace_list(&self) -> NamespaceList {
        support::list(&self.syntax, 1usize)
    }
}
impl AstNode for NamespaceUseClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(NAMESPACE_USE_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == NAMESPACE_USE_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for NamespaceList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(NAMESPACE_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == NAMESPACE_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for NamespaceList {
    type Language = OscDslLanguage;
    type Node = NamespaceName;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleNamespaceName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SimpleNamespaceName {
    pub fn identifier_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for SimpleNamespaceName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SIMPLE_NAMESPACE_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SIMPLE_NAMESPACE_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalNamespaceName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl GlobalNamespaceName {
    pub fn null_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for GlobalNamespaceName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLOBAL_NAMESPACE_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == GLOBAL_NAMESPACE_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSpecificationList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ExportSpecificationList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EXPORT_SPECIFICATION_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EXPORT_SPECIFICATION_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for ExportSpecificationList {
    type Language = OscDslLanguage;
    type Node = ExportSpecification;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSpecification {
    QualifiedIdentifier(QualifiedIdentifier),
    ExportWildcardSpecification(ExportWildcardSpecification),
}
impl ExportSpecification {
    pub fn as_qualified_identifier(&self) -> Option<&QualifiedIdentifier> {
        match self {
            ExportSpecification::QualifiedIdentifier(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_export_wildcard_specification(&self) -> Option<&ExportWildcardSpecification> {
        match self {
            ExportSpecification::ExportWildcardSpecification(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ExportSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        QualifiedIdentifier::KIND_SET.union(ExportWildcardSpecification::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, QUALIFIED_IDENTIFIER | EXPORT_WILDCARD_SPECIFICATION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            QUALIFIED_IDENTIFIER => Self::QualifiedIdentifier(QualifiedIdentifier::cast(syntax)?),
            EXPORT_WILDCARD_SPECIFICATION => {
                Self::ExportWildcardSpecification(ExportWildcardSpecification::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::QualifiedIdentifier(node) => node.syntax(),
            Self::ExportWildcardSpecification(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::QualifiedIdentifier(node) => node.into_syntax(),
            Self::ExportWildcardSpecification(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportWildcardSpecification {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ExportWildcardSpecification {
    pub fn export_wildcard_specification_prefix(
        &self,
    ) -> Option<ExportWildcardSpecificationPrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for ExportWildcardSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EXPORT_WILDCARD_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EXPORT_WILDCARD_SPECIFICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportWildcardSpecificationPrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ExportWildcardSpecificationPrefix {
    pub fn namespace_name(&self) -> Option<NamespaceName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn colon_colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for ExportWildcardSpecificationPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EXPORT_WILDCARD_SPECIFICATION_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EXPORT_WILDCARD_SPECIFICATION_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTypeDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PhysicalTypeDeclaration {
    pub fn type_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn physical_type_name(&self) -> SyntaxResult<PhysicalTypeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn base_unit_specifier(&self) -> SyntaxResult<BaseUnitSpecifier> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for PhysicalTypeDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PHYSICAL_TYPE_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PHYSICAL_TYPE_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnitDeclaration {
    pub fn unit_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn unit_name(&self) -> SyntaxResult<UnitName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn physical_type_name(&self) -> SyntaxResult<PhysicalTypeName> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn unit_specifier(&self) -> SyntaxResult<UnitSpecifier> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl AstNode for UnitDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNIT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNIT_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumDeclaration {
    pub fn enum_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn enum_name(&self) -> SyntaxResult<EnumName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn enum_member_decl_list(&self) -> EnumMemberDeclList {
        support::list(&self.syntax, 4usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl AstNode for EnumDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructDeclaration {
    pub fn struct_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn struct_name(&self) -> SyntaxResult<StructName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn struct_inherits_clause(&self) -> Option<StructInheritsClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn struct_body_or_empty(&self) -> SyntaxResult<StructBodyOrEmpty> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for StructDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCT_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActorDeclaration {
    pub fn actor_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn actor_name(&self) -> SyntaxResult<ActorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn actor_inherits_clause(&self) -> Option<ActorInheritsClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn actor_body_or_empty(&self) -> SyntaxResult<ActorBodyOrEmpty> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for ActorDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActionDeclaration {
    pub fn action_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> SyntaxResult<QualifiedBehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn action_inherits_clause(&self) -> Option<ActionInheritsClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn action_body_or_empty(&self) -> SyntaxResult<ActionBodyOrEmpty> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for ActionDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTION_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTION_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ScenarioDeclaration {
    pub fn scenario_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> SyntaxResult<QualifiedBehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn scenario_inherits_clause(&self) -> Option<ScenarioInheritsClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn scenario_body_or_empty(&self) -> SyntaxResult<ScenarioBodyOrEmpty> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for ScenarioDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SCENARIO_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SCENARIO_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierDeclaration {
    pub fn modifier_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn modifier_name_prefix(&self) -> Option<ModifierNamePrefix> {
        support::node(&self.syntax, 1usize)
    }
    pub fn modifier_name(&self) -> SyntaxResult<ModifierName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn modifier_of_clause(&self) -> Option<ModifierOfClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn modifier_body_or_empty(&self) -> SyntaxResult<ModifierBodyOrEmpty> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl AstNode for ModifierDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExtension {
    EnumTypeExtension(EnumTypeExtension),
    StructuredTypeExtension(StructuredTypeExtension),
}
impl TypeExtension {
    pub fn as_enum_type_extension(&self) -> Option<&EnumTypeExtension> {
        match self {
            TypeExtension::EnumTypeExtension(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_structured_type_extension(&self) -> Option<&StructuredTypeExtension> {
        match self {
            TypeExtension::StructuredTypeExtension(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for TypeExtension {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        EnumTypeExtension::KIND_SET.union(StructuredTypeExtension::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ENUM_TYPE_EXTENSION | STRUCTURED_TYPE_EXTENSION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ENUM_TYPE_EXTENSION => Self::EnumTypeExtension(EnumTypeExtension::cast(syntax)?),
            STRUCTURED_TYPE_EXTENSION => {
                Self::StructuredTypeExtension(StructuredTypeExtension::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EnumTypeExtension(node) => node.syntax(),
            Self::StructuredTypeExtension(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EnumTypeExtension(node) => node.into_syntax(),
            Self::StructuredTypeExtension(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalParameterDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl GlobalParameterDeclaration {
    pub fn global_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn parameter_declaration(&self) -> SyntaxResult<ParameterDeclaration> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for GlobalParameterDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(GLOBAL_PARAMETER_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == GLOBAL_PARAMETER_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDeclarator {
    NonAggregateTypeDeclarator(NonAggregateTypeDeclarator),
    AggregateTypeDeclarator(AggregateTypeDeclarator),
}
impl TypeDeclarator {
    pub fn as_non_aggregate_type_declarator(&self) -> Option<&NonAggregateTypeDeclarator> {
        match self {
            TypeDeclarator::NonAggregateTypeDeclarator(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_aggregate_type_declarator(&self) -> Option<&AggregateTypeDeclarator> {
        match self {
            TypeDeclarator::AggregateTypeDeclarator(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for TypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        NonAggregateTypeDeclarator::KIND_SET.union(AggregateTypeDeclarator::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            NON_AGGREGATE_TYPE_DECLARATOR | AGGREGATE_TYPE_DECLARATOR
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            NON_AGGREGATE_TYPE_DECLARATOR => {
                Self::NonAggregateTypeDeclarator(NonAggregateTypeDeclarator::cast(syntax)?)
            }
            AGGREGATE_TYPE_DECLARATOR => {
                Self::AggregateTypeDeclarator(AggregateTypeDeclarator::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::NonAggregateTypeDeclarator(node) => node.syntax(),
            Self::AggregateTypeDeclarator(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::NonAggregateTypeDeclarator(node) => node.into_syntax(),
            Self::AggregateTypeDeclarator(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonAggregateTypeDeclarator {
    PrimitiveType(PrimitiveType),
    PhysicalTypeName(PhysicalTypeName),
    EnumName(EnumName),
    StructName(StructName),
    ActorName(ActorName),
    QualifiedBehaviorName(QualifiedBehaviorName),
}
impl NonAggregateTypeDeclarator {
    pub fn as_primitive_type(&self) -> Option<&PrimitiveType> {
        match self {
            NonAggregateTypeDeclarator::PrimitiveType(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_physical_type_name(&self) -> Option<&PhysicalTypeName> {
        match self {
            NonAggregateTypeDeclarator::PhysicalTypeName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_enum_name(&self) -> Option<&EnumName> {
        match self {
            NonAggregateTypeDeclarator::EnumName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_struct_name(&self) -> Option<&StructName> {
        match self {
            NonAggregateTypeDeclarator::StructName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_actor_name(&self) -> Option<&ActorName> {
        match self {
            NonAggregateTypeDeclarator::ActorName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_qualified_behavior_name(&self) -> Option<&QualifiedBehaviorName> {
        match self {
            NonAggregateTypeDeclarator::QualifiedBehaviorName(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for NonAggregateTypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = PrimitiveType::KIND_SET
        .union(PhysicalTypeName::KIND_SET)
        .union(EnumName::KIND_SET)
        .union(StructName::KIND_SET)
        .union(ActorName::KIND_SET)
        .union(QualifiedBehaviorName::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            PRIMITIVE_TYPE
                | PHYSICAL_TYPE_NAME
                | ENUM_NAME
                | STRUCT_NAME
                | ACTOR_NAME
                | QUALIFIED_BEHAVIOR_NAME
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            PRIMITIVE_TYPE => Self::PrimitiveType(PrimitiveType::cast(syntax)?),
            PHYSICAL_TYPE_NAME => Self::PhysicalTypeName(PhysicalTypeName::cast(syntax)?),
            ENUM_NAME => Self::EnumName(EnumName::cast(syntax)?),
            STRUCT_NAME => Self::StructName(StructName::cast(syntax)?),
            ACTOR_NAME => Self::ActorName(ActorName::cast(syntax)?),
            QUALIFIED_BEHAVIOR_NAME => {
                Self::QualifiedBehaviorName(QualifiedBehaviorName::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::PrimitiveType(node) => node.syntax(),
            Self::PhysicalTypeName(node) => node.syntax(),
            Self::EnumName(node) => node.syntax(),
            Self::StructName(node) => node.syntax(),
            Self::ActorName(node) => node.syntax(),
            Self::QualifiedBehaviorName(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::PrimitiveType(node) => node.into_syntax(),
            Self::PhysicalTypeName(node) => node.into_syntax(),
            Self::EnumName(node) => node.into_syntax(),
            Self::StructName(node) => node.into_syntax(),
            Self::ActorName(node) => node.into_syntax(),
            Self::QualifiedBehaviorName(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggregateTypeDeclarator {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl AggregateTypeDeclarator {
    pub fn list_type_declarator(&self) -> SyntaxResult<ListTypeDeclarator> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for AggregateTypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(AGGREGATE_TYPE_DECLARATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == AGGREGATE_TYPE_DECLARATOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrimitiveTypeKind {
    Int,
    Uint,
    Float,
    Bool,
    String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveType {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PrimitiveType {
    pub fn kind(&self) -> PrimitiveTypeKind {
        match self.syntax.kind() {
            INT_KW => PrimitiveTypeKind::Int,
            UINT_KW => PrimitiveTypeKind::Uint,
            FLOAT_KW => PrimitiveTypeKind::Float,
            BOOL_KW => PrimitiveTypeKind::Bool,
            STRING_KW => PrimitiveTypeKind::String,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for PrimitiveType {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PRIMITIVE_TYPE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PRIMITIVE_TYPE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalTypeName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PhysicalTypeName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for PhysicalTypeName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PHYSICAL_TYPE_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PHYSICAL_TYPE_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for EnumName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for StructName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCT_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActorName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ActorName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedBehaviorName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl QualifiedBehaviorName {
    pub fn qualified_behavior_name_prefix(&self) -> Option<QualifiedBehaviorNamePrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn behavior_name(&self) -> SyntaxResult<BehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for QualifiedBehaviorName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(QUALIFIED_BEHAVIOR_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == QUALIFIED_BEHAVIOR_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTypeDeclarator {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ListTypeDeclarator {
    pub fn list_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn non_aggregate_type_declarator(&self) -> SyntaxResult<NonAggregateTypeDeclarator> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for ListTypeDeclarator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(LIST_TYPE_DECLARATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == LIST_TYPE_DECLARATOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseUnitSpecifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BaseUnitSpecifier {
    pub fn si_base_unit_specifier(&self) -> SyntaxResult<SiBaseUnitSpecifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for BaseUnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BASE_UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BASE_UNIT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitSpecifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnitSpecifier {
    pub fn si_unit_specifier(&self) -> SyntaxResult<SiUnitSpecifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for UnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNIT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseUnitSpecifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SiBaseUnitSpecifier {
    pub fn si_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn si_base_exponent_list(&self) -> SiBaseExponentList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for SiBaseUnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SI_BASE_UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SI_BASE_UNIT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiUnitSpecifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SiUnitSpecifier {
    pub fn si_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn si_base_exponent_list(&self) -> SiBaseExponentList {
        support::list(&self.syntax, 2usize)
    }
    pub fn trailing_si_factor(&self) -> Option<TrailingSiFactor> {
        support::node(&self.syntax, 3usize)
    }
    pub fn trailing_si_offset(&self) -> Option<TrailingSiOffset> {
        support::node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for SiUnitSpecifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SI_UNIT_SPECIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SI_UNIT_SPECIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseExponentList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for SiBaseExponentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SI_BASE_EXPONENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SI_BASE_EXPONENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for SiBaseExponentList {
    type Language = OscDslLanguage;
    type Node = SiBaseExponent;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseExponent {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SiBaseExponent {
    pub fn si_base_unit_name(&self) -> SyntaxResult<SiBaseUnitName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn integer_literal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for SiBaseExponent {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SI_BASE_EXPONENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SI_BASE_EXPONENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SiBaseUnitNameKind {
    Kg,
    M,
    S,
    A,
    K,
    Mol,
    Cd,
    Rad,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SiBaseUnitName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SiBaseUnitName {
    pub fn kind(&self) -> SiBaseUnitNameKind {
        match self.syntax.kind() {
            KG_KW => SiBaseUnitNameKind::Kg,
            M_KW => SiBaseUnitNameKind::M,
            S_KW => SiBaseUnitNameKind::S,
            A_KW => SiBaseUnitNameKind::A,
            K_KW => SiBaseUnitNameKind::K,
            MOL_KW => SiBaseUnitNameKind::Mol,
            CD_KW => SiBaseUnitNameKind::Cd,
            RAD_KW => SiBaseUnitNameKind::Rad,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for SiBaseUnitName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SI_BASE_UNIT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SI_BASE_UNIT_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingSiFactor {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl TrailingSiFactor {
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn factor_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn number_literal(&self) -> SyntaxResult<NumberLiteral> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for TrailingSiFactor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TRAILING_SI_FACTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == TRAILING_SI_FACTOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingSiOffset {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl TrailingSiOffset {
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn offset_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn number_literal(&self) -> SyntaxResult<NumberLiteral> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for TrailingSiOffset {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TRAILING_SI_OFFSET as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == TRAILING_SI_OFFSET
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDeclList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for EnumMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_MEMBER_DECL_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_MEMBER_DECL_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for EnumMemberDeclList {
    type Language = OscDslLanguage;
    type Node = EnumMemberDecl;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberDecl {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberDecl {
    pub fn enum_member_name(&self) -> SyntaxResult<EnumMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn enum_member_initializer(&self) -> Option<EnumMemberInitializer> {
        support::node(&self.syntax, 1usize)
    }
}
impl AstNode for EnumMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_MEMBER_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_MEMBER_DECL
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for EnumMemberName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_MEMBER_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_MEMBER_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberInitializer {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberInitializer {
    pub fn assign_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn enum_member_value(&self) -> SyntaxResult<EnumMemberValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for EnumMemberInitializer {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_MEMBER_INITIALIZER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_MEMBER_INITIALIZER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumMemberValue {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumMemberValue {
    pub fn integer_literal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for EnumMemberValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_MEMBER_VALUE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_MEMBER_VALUE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueReference {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumValueReference {
    pub fn enum_value_reference_prefix(&self) -> Option<EnumValueReferencePrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn enum_member_name(&self) -> SyntaxResult<EnumMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for EnumValueReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_VALUE_REFERENCE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_VALUE_REFERENCE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumValueReferencePrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumValueReferencePrefix {
    pub fn enum_name(&self) -> SyntaxResult<EnumName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn exclamation_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for EnumValueReferencePrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_VALUE_REFERENCE_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_VALUE_REFERENCE_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructInheritsClause {
    pub fn inherits_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn struct_name(&self) -> SyntaxResult<StructName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn struct_inherits_condition(&self) -> Option<StructInheritsCondition> {
        support::node(&self.syntax, 2usize)
    }
}
impl AstNode for StructInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCT_INHERITS_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCT_INHERITS_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructBodyOrEmpty {
    StructBody(StructBody),
    Empty(Empty),
}
impl StructBodyOrEmpty {
    pub fn as_struct_body(&self) -> Option<&StructBody> {
        match self {
            StructBodyOrEmpty::StructBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            StructBodyOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for StructBodyOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = StructBody::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, STRUCT_BODY | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            STRUCT_BODY => Self::StructBody(StructBody::cast(syntax)?),
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::StructBody(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::StructBody(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructInheritsCondition {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructInheritsCondition {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn field_name(&self) -> SyntaxResult<FieldName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn equal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn struct_inherits_constant(&self) -> SyntaxResult<StructInheritsConstant> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for StructInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCT_INHERITS_CONDITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCT_INHERITS_CONDITION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl FieldName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for FieldName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(FIELD_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == FIELD_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructInheritsConstant {
    EnumValueReference(EnumValueReference),
    BoolLiteral(BoolLiteral),
}
impl StructInheritsConstant {
    pub fn as_enum_value_reference(&self) -> Option<&EnumValueReference> {
        match self {
            StructInheritsConstant::EnumValueReference(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_bool_literal(&self) -> Option<&BoolLiteral> {
        match self {
            StructInheritsConstant::BoolLiteral(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for StructInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        EnumValueReference::KIND_SET.union(BoolLiteral::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ENUM_VALUE_REFERENCE | BOOL_LITERAL)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ENUM_VALUE_REFERENCE => Self::EnumValueReference(EnumValueReference::cast(syntax)?),
            BOOL_LITERAL => Self::BoolLiteral(BoolLiteral::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EnumValueReference(node) => node.syntax(),
            Self::BoolLiteral(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EnumValueReference(node) => node.into_syntax(),
            Self::BoolLiteral(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructBody {
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn struct_member_decl_list(&self) -> StructMemberDeclList {
        support::list(&self.syntax, 3usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for StructBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCT_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCT_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructMemberDeclList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for StructMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCT_MEMBER_DECL_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCT_MEMBER_DECL_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for StructMemberDeclList {
    type Language = OscDslLanguage;
    type Node = StructMemberDecl;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructMemberDecl {
    EventDeclaration(EventDeclaration),
    FieldDeclaration(FieldDeclaration),
    ConstraintDeclaration(ConstraintDeclaration),
    MethodDeclaration(MethodDeclaration),
    CoverageDeclaration(CoverageDeclaration),
}
impl StructMemberDecl {
    pub fn as_event_declaration(&self) -> Option<&EventDeclaration> {
        match self {
            StructMemberDecl::EventDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_field_declaration(&self) -> Option<&FieldDeclaration> {
        match self {
            StructMemberDecl::FieldDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_constraint_declaration(&self) -> Option<&ConstraintDeclaration> {
        match self {
            StructMemberDecl::ConstraintDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_method_declaration(&self) -> Option<&MethodDeclaration> {
        match self {
            StructMemberDecl::MethodDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_coverage_declaration(&self) -> Option<&CoverageDeclaration> {
        match self {
            StructMemberDecl::CoverageDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for StructMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = EventDeclaration::KIND_SET
        .union(FieldDeclaration::KIND_SET)
        .union(ConstraintDeclaration::KIND_SET)
        .union(MethodDeclaration::KIND_SET)
        .union(CoverageDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            EVENT_DECLARATION
                | FIELD_DECLARATION
                | CONSTRAINT_DECLARATION
                | METHOD_DECLARATION
                | COVERAGE_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            EVENT_DECLARATION => Self::EventDeclaration(EventDeclaration::cast(syntax)?),
            FIELD_DECLARATION => Self::FieldDeclaration(FieldDeclaration::cast(syntax)?),
            CONSTRAINT_DECLARATION => {
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(syntax)?)
            }
            METHOD_DECLARATION => Self::MethodDeclaration(MethodDeclaration::cast(syntax)?),
            COVERAGE_DECLARATION => Self::CoverageDeclaration(CoverageDeclaration::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EventDeclaration(node) => node.syntax(),
            Self::FieldDeclaration(node) => node.syntax(),
            Self::ConstraintDeclaration(node) => node.syntax(),
            Self::MethodDeclaration(node) => node.syntax(),
            Self::CoverageDeclaration(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EventDeclaration(node) => node.into_syntax(),
            Self::FieldDeclaration(node) => node.into_syntax(),
            Self::ConstraintDeclaration(node) => node.into_syntax(),
            Self::MethodDeclaration(node) => node.into_syntax(),
            Self::CoverageDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventDeclaration {
    pub fn event_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_name(&self) -> SyntaxResult<EventName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn event_argument_list_specification(&self) -> Option<EventArgumentListSpecification> {
        support::node(&self.syntax, 2usize)
    }
    pub fn event_is_clause(&self) -> Option<EventIsClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for EventDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDeclaration {
    ParameterDeclaration(ParameterDeclaration),
    VariableDeclaration(VariableDeclaration),
}
impl FieldDeclaration {
    pub fn as_parameter_declaration(&self) -> Option<&ParameterDeclaration> {
        match self {
            FieldDeclaration::ParameterDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_variable_declaration(&self) -> Option<&VariableDeclaration> {
        match self {
            FieldDeclaration::VariableDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for FieldDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ParameterDeclaration::KIND_SET.union(VariableDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, PARAMETER_DECLARATION | VARIABLE_DECLARATION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            PARAMETER_DECLARATION => {
                Self::ParameterDeclaration(ParameterDeclaration::cast(syntax)?)
            }
            VARIABLE_DECLARATION => Self::VariableDeclaration(VariableDeclaration::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ParameterDeclaration(node) => node.syntax(),
            Self::VariableDeclaration(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ParameterDeclaration(node) => node.into_syntax(),
            Self::VariableDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintDeclaration {
    KeepConstraintDeclaration(KeepConstraintDeclaration),
    RemoveDefaultDeclaration(RemoveDefaultDeclaration),
}
impl ConstraintDeclaration {
    pub fn as_keep_constraint_declaration(&self) -> Option<&KeepConstraintDeclaration> {
        match self {
            ConstraintDeclaration::KeepConstraintDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_remove_default_declaration(&self) -> Option<&RemoveDefaultDeclaration> {
        match self {
            ConstraintDeclaration::RemoveDefaultDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ConstraintDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        KeepConstraintDeclaration::KIND_SET.union(RemoveDefaultDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            KEEP_CONSTRAINT_DECLARATION | REMOVE_DEFAULT_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            KEEP_CONSTRAINT_DECLARATION => {
                Self::KeepConstraintDeclaration(KeepConstraintDeclaration::cast(syntax)?)
            }
            REMOVE_DEFAULT_DECLARATION => {
                Self::RemoveDefaultDeclaration(RemoveDefaultDeclaration::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::KeepConstraintDeclaration(node) => node.syntax(),
            Self::RemoveDefaultDeclaration(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::KeepConstraintDeclaration(node) => node.into_syntax(),
            Self::RemoveDefaultDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodDeclaration {
    pub fn def_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn method_name(&self) -> SyntaxResult<MethodName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn argument_list_specification(&self) -> ArgumentListSpecification {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn method_return_type(&self) -> Option<MethodReturnType> {
        support::node(&self.syntax, 5usize)
    }
    pub fn method_implementation(&self) -> SyntaxResult<MethodImplementation> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 7usize)
    }
}
impl AstNode for MethodDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl CoverageDeclaration {
    pub fn coverage_operator(&self) -> SyntaxResult<CoverageOperator> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn argument_list(&self) -> SyntaxResult<ArgumentList> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for CoverageDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(COVERAGE_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == COVERAGE_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActorInheritsClause {
    pub fn inherits_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn actor_name(&self) -> SyntaxResult<ActorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn actor_inherits_condition(&self) -> Option<ActorInheritsCondition> {
        support::node(&self.syntax, 2usize)
    }
}
impl AstNode for ActorInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_INHERITS_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_INHERITS_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorBodyOrEmpty {
    ActorBody(ActorBody),
    Empty(Empty),
}
impl ActorBodyOrEmpty {
    pub fn as_actor_body(&self) -> Option<&ActorBody> {
        match self {
            ActorBodyOrEmpty::ActorBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            ActorBodyOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ActorBodyOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = ActorBody::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ACTOR_BODY | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ACTOR_BODY => Self::ActorBody(ActorBody::cast(syntax)?),
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ActorBody(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ActorBody(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorInheritsCondition {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActorInheritsCondition {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn field_name(&self) -> SyntaxResult<FieldName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn equal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn actor_inherits_constant(&self) -> SyntaxResult<ActorInheritsConstant> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ActorInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_INHERITS_CONDITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_INHERITS_CONDITION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorInheritsConstant {
    EnumValueReference(EnumValueReference),
    BoolLiteral(BoolLiteral),
}
impl ActorInheritsConstant {
    pub fn as_enum_value_reference(&self) -> Option<&EnumValueReference> {
        match self {
            ActorInheritsConstant::EnumValueReference(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_bool_literal(&self) -> Option<&BoolLiteral> {
        match self {
            ActorInheritsConstant::BoolLiteral(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ActorInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        EnumValueReference::KIND_SET.union(BoolLiteral::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ENUM_VALUE_REFERENCE | BOOL_LITERAL)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ENUM_VALUE_REFERENCE => Self::EnumValueReference(EnumValueReference::cast(syntax)?),
            BOOL_LITERAL => Self::BoolLiteral(BoolLiteral::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EnumValueReference(node) => node.syntax(),
            Self::BoolLiteral(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EnumValueReference(node) => node.into_syntax(),
            Self::BoolLiteral(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActorBody {
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn actor_member_decl_list(&self) -> ActorMemberDeclList {
        support::list(&self.syntax, 3usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ActorBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorMemberDeclList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ActorMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_MEMBER_DECL_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_MEMBER_DECL_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for ActorMemberDeclList {
    type Language = OscDslLanguage;
    type Node = ActorMemberDecl;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActorMemberDecl {
    EventDeclaration(EventDeclaration),
    FieldDeclaration(FieldDeclaration),
    ConstraintDeclaration(ConstraintDeclaration),
    MethodDeclaration(MethodDeclaration),
    CoverageDeclaration(CoverageDeclaration),
}
impl ActorMemberDecl {
    pub fn as_event_declaration(&self) -> Option<&EventDeclaration> {
        match self {
            ActorMemberDecl::EventDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_field_declaration(&self) -> Option<&FieldDeclaration> {
        match self {
            ActorMemberDecl::FieldDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_constraint_declaration(&self) -> Option<&ConstraintDeclaration> {
        match self {
            ActorMemberDecl::ConstraintDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_method_declaration(&self) -> Option<&MethodDeclaration> {
        match self {
            ActorMemberDecl::MethodDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_coverage_declaration(&self) -> Option<&CoverageDeclaration> {
        match self {
            ActorMemberDecl::CoverageDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ActorMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = EventDeclaration::KIND_SET
        .union(FieldDeclaration::KIND_SET)
        .union(ConstraintDeclaration::KIND_SET)
        .union(MethodDeclaration::KIND_SET)
        .union(CoverageDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            EVENT_DECLARATION
                | FIELD_DECLARATION
                | CONSTRAINT_DECLARATION
                | METHOD_DECLARATION
                | COVERAGE_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            EVENT_DECLARATION => Self::EventDeclaration(EventDeclaration::cast(syntax)?),
            FIELD_DECLARATION => Self::FieldDeclaration(FieldDeclaration::cast(syntax)?),
            CONSTRAINT_DECLARATION => {
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(syntax)?)
            }
            METHOD_DECLARATION => Self::MethodDeclaration(MethodDeclaration::cast(syntax)?),
            COVERAGE_DECLARATION => Self::CoverageDeclaration(CoverageDeclaration::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EventDeclaration(node) => node.syntax(),
            Self::FieldDeclaration(node) => node.syntax(),
            Self::ConstraintDeclaration(node) => node.syntax(),
            Self::MethodDeclaration(node) => node.syntax(),
            Self::CoverageDeclaration(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EventDeclaration(node) => node.into_syntax(),
            Self::FieldDeclaration(node) => node.into_syntax(),
            Self::ConstraintDeclaration(node) => node.into_syntax(),
            Self::MethodDeclaration(node) => node.into_syntax(),
            Self::CoverageDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ScenarioInheritsClause {
    pub fn inherits_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> SyntaxResult<QualifiedBehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn scenario_inherits_condition(&self) -> Option<ScenarioInheritsCondition> {
        support::node(&self.syntax, 2usize)
    }
}
impl AstNode for ScenarioInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SCENARIO_INHERITS_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SCENARIO_INHERITS_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioBodyOrEmpty {
    ScenarioBody(ScenarioBody),
    Empty(Empty),
}
impl ScenarioBodyOrEmpty {
    pub fn as_scenario_body(&self) -> Option<&ScenarioBody> {
        match self {
            ScenarioBodyOrEmpty::ScenarioBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            ScenarioBodyOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ScenarioBodyOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = ScenarioBody::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, SCENARIO_BODY | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            SCENARIO_BODY => Self::ScenarioBody(ScenarioBody::cast(syntax)?),
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ScenarioBody(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ScenarioBody(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioInheritsCondition {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ScenarioInheritsCondition {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn field_name(&self) -> SyntaxResult<FieldName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn equal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn scenario_inherits_constant(&self) -> SyntaxResult<ScenarioInheritsConstant> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ScenarioInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SCENARIO_INHERITS_CONDITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SCENARIO_INHERITS_CONDITION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioInheritsConstant {
    EnumValueReference(EnumValueReference),
    BoolLiteral(BoolLiteral),
}
impl ScenarioInheritsConstant {
    pub fn as_enum_value_reference(&self) -> Option<&EnumValueReference> {
        match self {
            ScenarioInheritsConstant::EnumValueReference(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_bool_literal(&self) -> Option<&BoolLiteral> {
        match self {
            ScenarioInheritsConstant::BoolLiteral(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ScenarioInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        EnumValueReference::KIND_SET.union(BoolLiteral::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ENUM_VALUE_REFERENCE | BOOL_LITERAL)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ENUM_VALUE_REFERENCE => Self::EnumValueReference(EnumValueReference::cast(syntax)?),
            BOOL_LITERAL => Self::BoolLiteral(BoolLiteral::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EnumValueReference(node) => node.syntax(),
            Self::BoolLiteral(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EnumValueReference(node) => node.into_syntax(),
            Self::BoolLiteral(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ScenarioBody {
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn scenario_member_item_list(&self) -> ScenarioMemberItemList {
        support::list(&self.syntax, 3usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ScenarioBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SCENARIO_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SCENARIO_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioMemberItemList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ScenarioMemberItemList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SCENARIO_MEMBER_ITEM_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SCENARIO_MEMBER_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for ScenarioMemberItemList {
    type Language = OscDslLanguage;
    type Node = ScenarioMemberItem;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioMemberItem {
    ScenarioMemberDecl(ScenarioMemberDecl),
    BehaviorSpecification(BehaviorSpecification),
}
impl ScenarioMemberItem {
    pub fn as_scenario_member_decl(&self) -> Option<&ScenarioMemberDecl> {
        match self {
            ScenarioMemberItem::ScenarioMemberDecl(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_behavior_specification(&self) -> Option<&BehaviorSpecification> {
        match self {
            ScenarioMemberItem::BehaviorSpecification(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ScenarioMemberItem {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ScenarioMemberDecl::KIND_SET.union(BehaviorSpecification::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, SCENARIO_MEMBER_DECL | BEHAVIOR_SPECIFICATION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            SCENARIO_MEMBER_DECL => Self::ScenarioMemberDecl(ScenarioMemberDecl::cast(syntax)?),
            BEHAVIOR_SPECIFICATION => {
                Self::BehaviorSpecification(BehaviorSpecification::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ScenarioMemberDecl(node) => node.into_syntax(),
            Self::BehaviorSpecification(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioMemberDecl {
    EventDeclaration(EventDeclaration),
    FieldDeclaration(FieldDeclaration),
    ConstraintDeclaration(ConstraintDeclaration),
    MethodDeclaration(MethodDeclaration),
    CoverageDeclaration(CoverageDeclaration),
}
impl ScenarioMemberDecl {
    pub fn as_event_declaration(&self) -> Option<&EventDeclaration> {
        match self {
            ScenarioMemberDecl::EventDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_field_declaration(&self) -> Option<&FieldDeclaration> {
        match self {
            ScenarioMemberDecl::FieldDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_constraint_declaration(&self) -> Option<&ConstraintDeclaration> {
        match self {
            ScenarioMemberDecl::ConstraintDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_method_declaration(&self) -> Option<&MethodDeclaration> {
        match self {
            ScenarioMemberDecl::MethodDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_coverage_declaration(&self) -> Option<&CoverageDeclaration> {
        match self {
            ScenarioMemberDecl::CoverageDeclaration(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ScenarioMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = EventDeclaration::KIND_SET
        .union(FieldDeclaration::KIND_SET)
        .union(ConstraintDeclaration::KIND_SET)
        .union(MethodDeclaration::KIND_SET)
        .union(CoverageDeclaration::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            EVENT_DECLARATION
                | FIELD_DECLARATION
                | CONSTRAINT_DECLARATION
                | METHOD_DECLARATION
                | COVERAGE_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            EVENT_DECLARATION => Self::EventDeclaration(EventDeclaration::cast(syntax)?),
            FIELD_DECLARATION => Self::FieldDeclaration(FieldDeclaration::cast(syntax)?),
            CONSTRAINT_DECLARATION => {
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(syntax)?)
            }
            METHOD_DECLARATION => Self::MethodDeclaration(MethodDeclaration::cast(syntax)?),
            COVERAGE_DECLARATION => Self::CoverageDeclaration(CoverageDeclaration::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EventDeclaration(node) => node.syntax(),
            Self::FieldDeclaration(node) => node.syntax(),
            Self::ConstraintDeclaration(node) => node.syntax(),
            Self::MethodDeclaration(node) => node.syntax(),
            Self::CoverageDeclaration(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EventDeclaration(node) => node.into_syntax(),
            Self::FieldDeclaration(node) => node.into_syntax(),
            Self::ConstraintDeclaration(node) => node.into_syntax(),
            Self::MethodDeclaration(node) => node.into_syntax(),
            Self::CoverageDeclaration(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorSpecification {
    OnDirective(OnDirective),
    DoDirective(DoDirective),
}
impl BehaviorSpecification {
    pub fn as_on_directive(&self) -> Option<&OnDirective> {
        match self {
            BehaviorSpecification::OnDirective(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_do_directive(&self) -> Option<&DoDirective> {
        match self {
            BehaviorSpecification::DoDirective(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for BehaviorSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        OnDirective::KIND_SET.union(DoDirective::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ON_DIRECTIVE | DO_DIRECTIVE)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ON_DIRECTIVE => Self::OnDirective(OnDirective::cast(syntax)?),
            DO_DIRECTIVE => Self::DoDirective(DoDirective::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::OnDirective(node) => node.syntax(),
            Self::DoDirective(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::OnDirective(node) => node.into_syntax(),
            Self::DoDirective(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedBehaviorNamePrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl QualifiedBehaviorNamePrefix {
    pub fn actor_name(&self) -> SyntaxResult<ActorName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for QualifiedBehaviorNamePrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(QUALIFIED_BEHAVIOR_NAME_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == QUALIFIED_BEHAVIOR_NAME_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BehaviorName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for BehaviorName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BEHAVIOR_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BEHAVIOR_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActionInheritsClause {
    pub fn inherits_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> SyntaxResult<QualifiedBehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn action_inherits_condition(&self) -> Option<ActionInheritsCondition> {
        support::node(&self.syntax, 2usize)
    }
}
impl AstNode for ActionInheritsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTION_INHERITS_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTION_INHERITS_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionBodyOrEmpty {
    ActionBody(ActionBody),
    Empty(Empty),
}
impl ActionBodyOrEmpty {
    pub fn as_action_body(&self) -> Option<&ActionBody> {
        match self {
            ActionBodyOrEmpty::ActionBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            ActionBodyOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ActionBodyOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = ActionBody::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ACTION_BODY | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ACTION_BODY => Self::ActionBody(ActionBody::cast(syntax)?),
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ActionBody(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ActionBody(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActionBody {
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn action_member_item_list(&self) -> ActionMemberItemList {
        support::list(&self.syntax, 3usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ActionBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTION_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTION_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionInheritsCondition {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActionInheritsCondition {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn field_name(&self) -> SyntaxResult<FieldName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn equal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn action_inherits_constant(&self) -> SyntaxResult<ActionInheritsConstant> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ActionInheritsCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTION_INHERITS_CONDITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTION_INHERITS_CONDITION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionInheritsConstant {
    EnumValueReference(EnumValueReference),
    BoolLiteral(BoolLiteral),
}
impl ActionInheritsConstant {
    pub fn as_enum_value_reference(&self) -> Option<&EnumValueReference> {
        match self {
            ActionInheritsConstant::EnumValueReference(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_bool_literal(&self) -> Option<&BoolLiteral> {
        match self {
            ActionInheritsConstant::BoolLiteral(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ActionInheritsConstant {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        EnumValueReference::KIND_SET.union(BoolLiteral::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, ENUM_VALUE_REFERENCE | BOOL_LITERAL)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            ENUM_VALUE_REFERENCE => Self::EnumValueReference(EnumValueReference::cast(syntax)?),
            BOOL_LITERAL => Self::BoolLiteral(BoolLiteral::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EnumValueReference(node) => node.syntax(),
            Self::BoolLiteral(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EnumValueReference(node) => node.into_syntax(),
            Self::BoolLiteral(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMemberItemList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ActionMemberItemList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTION_MEMBER_ITEM_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTION_MEMBER_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for ActionMemberItemList {
    type Language = OscDslLanguage;
    type Node = ActionMemberItem;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionMemberItem {
    ScenarioMemberDecl(ScenarioMemberDecl),
    BehaviorSpecification(BehaviorSpecification),
}
impl ActionMemberItem {
    pub fn as_scenario_member_decl(&self) -> Option<&ScenarioMemberDecl> {
        match self {
            ActionMemberItem::ScenarioMemberDecl(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_behavior_specification(&self) -> Option<&BehaviorSpecification> {
        match self {
            ActionMemberItem::BehaviorSpecification(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ActionMemberItem {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ScenarioMemberDecl::KIND_SET.union(BehaviorSpecification::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, SCENARIO_MEMBER_DECL | BEHAVIOR_SPECIFICATION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            SCENARIO_MEMBER_DECL => Self::ScenarioMemberDecl(ScenarioMemberDecl::cast(syntax)?),
            BEHAVIOR_SPECIFICATION => {
                Self::BehaviorSpecification(BehaviorSpecification::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ScenarioMemberDecl(node) => node.into_syntax(),
            Self::BehaviorSpecification(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierNamePrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierNamePrefix {
    pub fn actor_name(&self) -> SyntaxResult<ActorName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for ModifierNamePrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_NAME_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_NAME_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ModifierName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierOfClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierOfClause {
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn qualified_behavior_name(&self) -> SyntaxResult<QualifiedBehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for ModifierOfClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_OF_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_OF_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierBodyOrEmpty {
    ModifierBody(ModifierBody),
    Empty(Empty),
}
impl ModifierBodyOrEmpty {
    pub fn as_modifier_body(&self) -> Option<&ModifierBody> {
        match self {
            ModifierBodyOrEmpty::ModifierBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            ModifierBodyOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ModifierBodyOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = ModifierBody::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, MODIFIER_BODY | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            MODIFIER_BODY => Self::ModifierBody(ModifierBody::cast(syntax)?),
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ModifierBody(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ModifierBody(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierBody {
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn modifier_member_item_list(&self) -> ModifierMemberItemList {
        support::list(&self.syntax, 3usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for ModifierBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierMemberItemList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ModifierMemberItemList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_MEMBER_ITEM_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_MEMBER_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for ModifierMemberItemList {
    type Language = OscDslLanguage;
    type Node = ModifierMemberItem;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModifierMemberItem {
    ScenarioMemberDecl(ScenarioMemberDecl),
    OnDirective(OnDirective),
}
impl ModifierMemberItem {
    pub fn as_scenario_member_decl(&self) -> Option<&ScenarioMemberDecl> {
        match self {
            ModifierMemberItem::ScenarioMemberDecl(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_on_directive(&self) -> Option<&OnDirective> {
        match self {
            ModifierMemberItem::OnDirective(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ModifierMemberItem {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ScenarioMemberDecl::KIND_SET.union(OnDirective::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, SCENARIO_MEMBER_DECL | ON_DIRECTIVE)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            SCENARIO_MEMBER_DECL => Self::ScenarioMemberDecl(ScenarioMemberDecl::cast(syntax)?),
            ON_DIRECTIVE => Self::OnDirective(OnDirective::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::OnDirective(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ScenarioMemberDecl(node) => node.into_syntax(),
            Self::OnDirective(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnDirective {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl OnDirective {
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_specification(&self) -> SyntaxResult<EventSpecification> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn on_member_list(&self) -> OnMemberList {
        support::list(&self.syntax, 5usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl AstNode for OnDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ON_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ON_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumTypeExtension {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EnumTypeExtension {
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn enum_name(&self) -> SyntaxResult<EnumName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn enum_member_decl_list(&self) -> EnumMemberDeclList {
        support::list(&self.syntax, 4usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl AstNode for EnumTypeExtension {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ENUM_TYPE_EXTENSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ENUM_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuredTypeExtension {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StructuredTypeExtension {
    pub fn extend_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn extendable_type_name(&self) -> SyntaxResult<ExtendableTypeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn extendable_member_decl_list(&self) -> ExtendableMemberDeclList {
        support::list(&self.syntax, 5usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl AstNode for StructuredTypeExtension {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRUCTURED_TYPE_EXTENSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRUCTURED_TYPE_EXTENSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendableTypeName {
    StructName(StructName),
    ActorName(ActorName),
    QualifiedBehaviorName(QualifiedBehaviorName),
}
impl ExtendableTypeName {
    pub fn as_struct_name(&self) -> Option<&StructName> {
        match self {
            ExtendableTypeName::StructName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_actor_name(&self) -> Option<&ActorName> {
        match self {
            ExtendableTypeName::ActorName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_qualified_behavior_name(&self) -> Option<&QualifiedBehaviorName> {
        match self {
            ExtendableTypeName::QualifiedBehaviorName(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ExtendableTypeName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = StructName::KIND_SET
        .union(ActorName::KIND_SET)
        .union(QualifiedBehaviorName::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, STRUCT_NAME | ACTOR_NAME | QUALIFIED_BEHAVIOR_NAME)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            STRUCT_NAME => Self::StructName(StructName::cast(syntax)?),
            ACTOR_NAME => Self::ActorName(ActorName::cast(syntax)?),
            QUALIFIED_BEHAVIOR_NAME => {
                Self::QualifiedBehaviorName(QualifiedBehaviorName::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::StructName(node) => node.syntax(),
            Self::ActorName(node) => node.syntax(),
            Self::QualifiedBehaviorName(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::StructName(node) => node.into_syntax(),
            Self::ActorName(node) => node.into_syntax(),
            Self::QualifiedBehaviorName(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtendableMemberDeclList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ExtendableMemberDeclList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EXTENDABLE_MEMBER_DECL_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EXTENDABLE_MEMBER_DECL_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for ExtendableMemberDeclList {
    type Language = OscDslLanguage;
    type Node = ExtendableMemberDecl;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtendableMemberDecl {
    StructMemberDecl(StructMemberDecl),
    ActorMemberDecl(ActorMemberDecl),
    ScenarioMemberDecl(ScenarioMemberDecl),
    BehaviorSpecification(BehaviorSpecification),
}
impl ExtendableMemberDecl {
    pub fn as_struct_member_decl(&self) -> Option<&StructMemberDecl> {
        match self {
            ExtendableMemberDecl::StructMemberDecl(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_actor_member_decl(&self) -> Option<&ActorMemberDecl> {
        match self {
            ExtendableMemberDecl::ActorMemberDecl(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_scenario_member_decl(&self) -> Option<&ScenarioMemberDecl> {
        match self {
            ExtendableMemberDecl::ScenarioMemberDecl(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_behavior_specification(&self) -> Option<&BehaviorSpecification> {
        match self {
            ExtendableMemberDecl::BehaviorSpecification(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ExtendableMemberDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = StructMemberDecl::KIND_SET
        .union(ActorMemberDecl::KIND_SET)
        .union(ScenarioMemberDecl::KIND_SET)
        .union(BehaviorSpecification::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            STRUCT_MEMBER_DECL | ACTOR_MEMBER_DECL | SCENARIO_MEMBER_DECL | BEHAVIOR_SPECIFICATION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            STRUCT_MEMBER_DECL => Self::StructMemberDecl(StructMemberDecl::cast(syntax)?),
            ACTOR_MEMBER_DECL => Self::ActorMemberDecl(ActorMemberDecl::cast(syntax)?),
            SCENARIO_MEMBER_DECL => Self::ScenarioMemberDecl(ScenarioMemberDecl::cast(syntax)?),
            BEHAVIOR_SPECIFICATION => {
                Self::BehaviorSpecification(BehaviorSpecification::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::StructMemberDecl(node) => node.syntax(),
            Self::ActorMemberDecl(node) => node.syntax(),
            Self::ScenarioMemberDecl(node) => node.syntax(),
            Self::BehaviorSpecification(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::StructMemberDecl(node) => node.into_syntax(),
            Self::ActorMemberDecl(node) => node.into_syntax(),
            Self::ScenarioMemberDecl(node) => node.into_syntax(),
            Self::BehaviorSpecification(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ParameterDeclaration {
    pub fn field_name_list(&self) -> FieldNameList {
        support::list(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn type_declarator(&self) -> SyntaxResult<TypeDeclarator> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn parameter_initilizer_clause(&self) -> Option<ParameterInitilizerClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn parameter_with_declaration_or_empty(
        &self,
    ) -> SyntaxResult<ParameterWithDeclarationOrEmpty> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl AstNode for ParameterDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PARAMETER_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PARAMETER_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for EventName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventArgumentListSpecification {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventArgumentListSpecification {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument_list_specification(&self) -> ArgumentListSpecification {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for EventArgumentListSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_ARGUMENT_LIST_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_ARGUMENT_LIST_SPECIFICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventIsClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventIsClause {
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_specification(&self) -> SyntaxResult<EventSpecification> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for EventIsClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_IS_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_IS_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentListSpecification {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ArgumentListSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ARGUMENT_LIST_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ARGUMENT_LIST_SPECIFICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for ArgumentListSpecification {
    type Language = OscDslLanguage;
    type Node = ArgumentSpecification;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventSpecification {
    EventReferenceSpecification(EventReferenceSpecification),
    EventCondition(EventCondition),
}
impl EventSpecification {
    pub fn as_event_reference_specification(&self) -> Option<&EventReferenceSpecification> {
        match self {
            EventSpecification::EventReferenceSpecification(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_event_condition(&self) -> Option<&EventCondition> {
        match self {
            EventSpecification::EventCondition(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for EventSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        EventReferenceSpecification::KIND_SET.union(EventCondition::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, EVENT_REFERENCE_SPECIFICATION | EVENT_CONDITION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            EVENT_REFERENCE_SPECIFICATION => {
                Self::EventReferenceSpecification(EventReferenceSpecification::cast(syntax)?)
            }
            EVENT_CONDITION => Self::EventCondition(EventCondition::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::EventReferenceSpecification(node) => node.syntax(),
            Self::EventCondition(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::EventReferenceSpecification(node) => node.into_syntax(),
            Self::EventCondition(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReferenceSpecification {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventReferenceSpecification {
    pub fn event_reference(&self) -> SyntaxResult<EventReference> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn event_reference_condition(&self) -> Option<EventReferenceCondition> {
        support::node(&self.syntax, 1usize)
    }
}
impl AstNode for EventReferenceSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_REFERENCE_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_REFERENCE_SPECIFICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventCondition {
    BoolExpression(BoolExpression),
    RiseExpression(RiseExpression),
    FallExpression(FallExpression),
    ElapsedExpression(ElapsedExpression),
    EveryExpression(EveryExpression),
}
impl EventCondition {
    pub fn as_bool_expression(&self) -> Option<&BoolExpression> {
        match self {
            EventCondition::BoolExpression(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_rise_expression(&self) -> Option<&RiseExpression> {
        match self {
            EventCondition::RiseExpression(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_fall_expression(&self) -> Option<&FallExpression> {
        match self {
            EventCondition::FallExpression(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_elapsed_expression(&self) -> Option<&ElapsedExpression> {
        match self {
            EventCondition::ElapsedExpression(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_every_expression(&self) -> Option<&EveryExpression> {
        match self {
            EventCondition::EveryExpression(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for EventCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = BoolExpression::KIND_SET
        .union(RiseExpression::KIND_SET)
        .union(FallExpression::KIND_SET)
        .union(ElapsedExpression::KIND_SET)
        .union(EveryExpression::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            BOOL_EXPRESSION
                | RISE_EXPRESSION
                | FALL_EXPRESSION
                | ELAPSED_EXPRESSION
                | EVERY_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            BOOL_EXPRESSION => Self::BoolExpression(BoolExpression::cast(syntax)?),
            RISE_EXPRESSION => Self::RiseExpression(RiseExpression::cast(syntax)?),
            FALL_EXPRESSION => Self::FallExpression(FallExpression::cast(syntax)?),
            ELAPSED_EXPRESSION => Self::ElapsedExpression(ElapsedExpression::cast(syntax)?),
            EVERY_EXPRESSION => Self::EveryExpression(EveryExpression::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::BoolExpression(node) => node.syntax(),
            Self::RiseExpression(node) => node.syntax(),
            Self::FallExpression(node) => node.syntax(),
            Self::ElapsedExpression(node) => node.syntax(),
            Self::EveryExpression(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::BoolExpression(node) => node.into_syntax(),
            Self::RiseExpression(node) => node.into_syntax(),
            Self::FallExpression(node) => node.into_syntax(),
            Self::ElapsedExpression(node) => node.into_syntax(),
            Self::EveryExpression(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReference {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventReference {
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_path(&self) -> SyntaxResult<EventPath> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for EventReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_REFERENCE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_REFERENCE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventReferenceCondition {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventReferenceCondition {
    pub fn event_field_decl(&self) -> Option<EventFieldDecl> {
        support::node(&self.syntax, 0usize)
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn event_condition(&self) -> SyntaxResult<EventCondition> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for EventReferenceCondition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_REFERENCE_CONDITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_REFERENCE_CONDITION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFieldDecl {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventFieldDecl {
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_field_name(&self) -> SyntaxResult<EventFieldName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for EventFieldDecl {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_FIELD_DECL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_FIELD_DECL
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventPath {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventPath {
    pub fn event_path_prefix(&self) -> Option<EventPathPrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn event_name(&self) -> SyntaxResult<EventName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for EventPath {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_PATH as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_PATH
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventFieldName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventFieldName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for EventFieldName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_FIELD_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_FIELD_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventPathPrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EventPathPrefix {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for EventPathPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVENT_PATH_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVENT_PATH_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    TernaryOpExp(TernaryOpExp),
    LogicalOpExp(LogicalOpExp),
    BinaryOpExp(BinaryOpExp),
    UnaryOpExp(UnaryOpExp),
    CastExp(CastExp),
    TypeTestExp(TypeTestExp),
    ElementAccess(ElementAccess),
    FunctionApplication(FunctionApplication),
    FieldAccess(FieldAccess),
    ItExp(ItExp),
    QualifiedIdentifier(QualifiedIdentifier),
    ParenthesizedExp(ParenthesizedExp),
    LiteralExp(LiteralExp),
    EnumValueReference(EnumValueReference),
    ListConstructor(ListConstructor),
    RangeConstructor(RangeConstructor),
}
impl Expression {
    pub fn as_ternary_op_exp(&self) -> Option<&TernaryOpExp> {
        match self {
            Expression::TernaryOpExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_logical_op_exp(&self) -> Option<&LogicalOpExp> {
        match self {
            Expression::LogicalOpExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_binary_op_exp(&self) -> Option<&BinaryOpExp> {
        match self {
            Expression::BinaryOpExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_unary_op_exp(&self) -> Option<&UnaryOpExp> {
        match self {
            Expression::UnaryOpExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_cast_exp(&self) -> Option<&CastExp> {
        match self {
            Expression::CastExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_type_test_exp(&self) -> Option<&TypeTestExp> {
        match self {
            Expression::TypeTestExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_element_access(&self) -> Option<&ElementAccess> {
        match self {
            Expression::ElementAccess(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_function_application(&self) -> Option<&FunctionApplication> {
        match self {
            Expression::FunctionApplication(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<&FieldAccess> {
        match self {
            Expression::FieldAccess(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_it_exp(&self) -> Option<&ItExp> {
        match self {
            Expression::ItExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_qualified_identifier(&self) -> Option<&QualifiedIdentifier> {
        match self {
            Expression::QualifiedIdentifier(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_parenthesized_exp(&self) -> Option<&ParenthesizedExp> {
        match self {
            Expression::ParenthesizedExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_literal_exp(&self) -> Option<&LiteralExp> {
        match self {
            Expression::LiteralExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_enum_value_reference(&self) -> Option<&EnumValueReference> {
        match self {
            Expression::EnumValueReference(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_list_constructor(&self) -> Option<&ListConstructor> {
        match self {
            Expression::ListConstructor(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_range_constructor(&self) -> Option<&RangeConstructor> {
        match self {
            Expression::RangeConstructor(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for Expression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = TernaryOpExp::KIND_SET
        .union(LogicalOpExp::KIND_SET)
        .union(BinaryOpExp::KIND_SET)
        .union(UnaryOpExp::KIND_SET)
        .union(CastExp::KIND_SET)
        .union(TypeTestExp::KIND_SET)
        .union(ElementAccess::KIND_SET)
        .union(FunctionApplication::KIND_SET)
        .union(FieldAccess::KIND_SET)
        .union(ItExp::KIND_SET)
        .union(QualifiedIdentifier::KIND_SET)
        .union(ParenthesizedExp::KIND_SET)
        .union(LiteralExp::KIND_SET)
        .union(EnumValueReference::KIND_SET)
        .union(ListConstructor::KIND_SET)
        .union(RangeConstructor::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            TERNARY_OP_EXP
                | LOGICAL_OP_EXP
                | BINARY_OP_EXP
                | UNARY_OP_EXP
                | CAST_EXP
                | TYPE_TEST_EXP
                | ELEMENT_ACCESS
                | FUNCTION_APPLICATION
                | FIELD_ACCESS
                | IT_EXP
                | QUALIFIED_IDENTIFIER
                | PARENTHESIZED_EXP
                | LITERAL_EXP
                | ENUM_VALUE_REFERENCE
                | LIST_CONSTRUCTOR
                | RANGE_CONSTRUCTOR
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            TERNARY_OP_EXP => Self::TernaryOpExp(TernaryOpExp::cast(syntax)?),
            LOGICAL_OP_EXP => Self::LogicalOpExp(LogicalOpExp::cast(syntax)?),
            BINARY_OP_EXP => Self::BinaryOpExp(BinaryOpExp::cast(syntax)?),
            UNARY_OP_EXP => Self::UnaryOpExp(UnaryOpExp::cast(syntax)?),
            CAST_EXP => Self::CastExp(CastExp::cast(syntax)?),
            TYPE_TEST_EXP => Self::TypeTestExp(TypeTestExp::cast(syntax)?),
            ELEMENT_ACCESS => Self::ElementAccess(ElementAccess::cast(syntax)?),
            FUNCTION_APPLICATION => Self::FunctionApplication(FunctionApplication::cast(syntax)?),
            FIELD_ACCESS => Self::FieldAccess(FieldAccess::cast(syntax)?),
            IT_EXP => Self::ItExp(ItExp::cast(syntax)?),
            QUALIFIED_IDENTIFIER => Self::QualifiedIdentifier(QualifiedIdentifier::cast(syntax)?),
            PARENTHESIZED_EXP => Self::ParenthesizedExp(ParenthesizedExp::cast(syntax)?),
            LITERAL_EXP => Self::LiteralExp(LiteralExp::cast(syntax)?),
            ENUM_VALUE_REFERENCE => Self::EnumValueReference(EnumValueReference::cast(syntax)?),
            LIST_CONSTRUCTOR => Self::ListConstructor(ListConstructor::cast(syntax)?),
            RANGE_CONSTRUCTOR => Self::RangeConstructor(RangeConstructor::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
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
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::TernaryOpExp(node) => node.into_syntax(),
            Self::LogicalOpExp(node) => node.into_syntax(),
            Self::BinaryOpExp(node) => node.into_syntax(),
            Self::UnaryOpExp(node) => node.into_syntax(),
            Self::CastExp(node) => node.into_syntax(),
            Self::TypeTestExp(node) => node.into_syntax(),
            Self::ElementAccess(node) => node.into_syntax(),
            Self::FunctionApplication(node) => node.into_syntax(),
            Self::FieldAccess(node) => node.into_syntax(),
            Self::ItExp(node) => node.into_syntax(),
            Self::QualifiedIdentifier(node) => node.into_syntax(),
            Self::ParenthesizedExp(node) => node.into_syntax(),
            Self::LiteralExp(node) => node.into_syntax(),
            Self::EnumValueReference(node) => node.into_syntax(),
            Self::ListConstructor(node) => node.into_syntax(),
            Self::RangeConstructor(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BoolExpression {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for BoolExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BOOL_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BOOL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiseExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl RiseExpression {
    pub fn rise_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn bool_expression(&self) -> SyntaxResult<BoolExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for RiseExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(RISE_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == RISE_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl FallExpression {
    pub fn fall_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn bool_expression(&self) -> SyntaxResult<BoolExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for FallExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(FALL_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == FALL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElapsedExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ElapsedExpression {
    pub fn elapsed_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn duration_expression(&self) -> SyntaxResult<DurationExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for ElapsedExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ELAPSED_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ELAPSED_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EveryExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EveryExpression {
    pub fn every_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn duration_expression(&self) -> SyntaxResult<DurationExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn trailing_every_exp_offset(&self) -> Option<TrailingEveryExpOffset> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for EveryExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EVERY_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EVERY_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl DurationExpression {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for DurationExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(DURATION_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == DURATION_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrailingEveryExpOffset {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl TrailingEveryExpOffset {
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn offset_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn duration_expression(&self) -> SyntaxResult<DurationExpression> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl AstNode for TrailingEveryExpOffset {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TRAILING_EVERY_EXP_OFFSET as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == TRAILING_EVERY_EXP_OFFSET
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl VariableDeclaration {
    pub fn var_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn field_name_list(&self) -> FieldNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn type_declarator(&self) -> SyntaxResult<TypeDeclarator> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn variable_initializer(&self) -> Option<VariableInitializer> {
        support::node(&self.syntax, 4usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for VariableDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VARIABLE_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == VARIABLE_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldNameList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for FieldNameList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(FIELD_NAME_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == FIELD_NAME_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for FieldNameList {
    type Language = OscDslLanguage;
    type Node = FieldName;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterInitilizerClause {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ParameterInitilizerClause {
    pub fn assign_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn default_value(&self) -> SyntaxResult<DefaultValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for ParameterInitilizerClause {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PARAMETER_INITILIZER_CLAUSE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PARAMETER_INITILIZER_CLAUSE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterWithDeclarationOrEmpty {
    ParameterWithDeclaration(ParameterWithDeclaration),
    Empty(Empty),
}
impl ParameterWithDeclarationOrEmpty {
    pub fn as_parameter_with_declaration(&self) -> Option<&ParameterWithDeclaration> {
        match self {
            ParameterWithDeclarationOrEmpty::ParameterWithDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            ParameterWithDeclarationOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ParameterWithDeclarationOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ParameterWithDeclaration::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, PARAMETER_WITH_DECLARATION | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            PARAMETER_WITH_DECLARATION => {
                Self::ParameterWithDeclaration(ParameterWithDeclaration::cast(syntax)?)
            }
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ParameterWithDeclaration(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ParameterWithDeclaration(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultValue {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl DefaultValue {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for DefaultValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(DEFAULT_VALUE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == DEFAULT_VALUE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableInitializer {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl VariableInitializer {
    pub fn assign_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn variable_initializer_value(&self) -> SyntaxResult<VariableInitializerValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for VariableInitializer {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(VARIABLE_INITIALIZER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == VARIABLE_INITIALIZER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableInitializerValue {
    DefaultValue(DefaultValue),
    SampleExpression(SampleExpression),
}
impl VariableInitializerValue {
    pub fn as_default_value(&self) -> Option<&DefaultValue> {
        match self {
            VariableInitializerValue::DefaultValue(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_sample_expression(&self) -> Option<&SampleExpression> {
        match self {
            VariableInitializerValue::SampleExpression(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for VariableInitializerValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        DefaultValue::KIND_SET.union(SampleExpression::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, DEFAULT_VALUE | SAMPLE_EXPRESSION)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            DEFAULT_VALUE => Self::DefaultValue(DefaultValue::cast(syntax)?),
            SAMPLE_EXPRESSION => Self::SampleExpression(SampleExpression::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::DefaultValue(node) => node.syntax(),
            Self::SampleExpression(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::DefaultValue(node) => node.into_syntax(),
            Self::SampleExpression(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SampleExpression {
    pub fn sample_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn event_specification(&self) -> SyntaxResult<EventSpecification> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn sample_default_value(&self) -> Option<SampleDefaultValue> {
        support::node(&self.syntax, 5usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl AstNode for SampleExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SAMPLE_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SAMPLE_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleDefaultValue {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl SampleDefaultValue {
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn default_value(&self) -> SyntaxResult<DefaultValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for SampleDefaultValue {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SAMPLE_DEFAULT_VALUE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == SAMPLE_DEFAULT_VALUE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ParameterWithDeclaration {
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn parameter_with_member(&self) -> SyntaxResult<ParameterWithMember> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for ParameterWithDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PARAMETER_WITH_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PARAMETER_WITH_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterWithMember {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ParameterWithMember {
    pub fn constraint_declaration(&self) -> SyntaxResult<ConstraintDeclaration> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ParameterWithMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PARAMETER_WITH_MEMBER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PARAMETER_WITH_MEMBER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeepConstraintDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl KeepConstraintDeclaration {
    pub fn keep_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn constraint_qualifier(&self) -> Option<ConstraintQualifier> {
        support::node(&self.syntax, 2usize)
    }
    pub fn constraint_expression(&self) -> SyntaxResult<ConstraintExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for KeepConstraintDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(KEEP_CONSTRAINT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == KEEP_CONSTRAINT_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDefaultDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl RemoveDefaultDeclaration {
    pub fn remove_default_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn parameter_reference(&self) -> SyntaxResult<ParameterReference> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for RemoveDefaultDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(REMOVE_DEFAULT_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == REMOVE_DEFAULT_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConstraintQualifierKind {
    Default,
    Hard,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintQualifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ConstraintQualifier {
    pub fn kind(&self) -> ConstraintQualifierKind {
        match self.syntax.kind() {
            DEFAULT_KW => ConstraintQualifierKind::Default,
            HARD_KW => ConstraintQualifierKind::Hard,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for ConstraintQualifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CONSTRAINT_QUALIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == CONSTRAINT_QUALIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstraintExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ConstraintExpression {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ConstraintExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CONSTRAINT_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == CONSTRAINT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterReference {
    FieldName(FieldName),
    FieldAccess(FieldAccess),
}
impl ParameterReference {
    pub fn as_field_name(&self) -> Option<&FieldName> {
        match self {
            ParameterReference::FieldName(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_field_access(&self) -> Option<&FieldAccess> {
        match self {
            ParameterReference::FieldAccess(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for ParameterReference {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        FieldName::KIND_SET.union(FieldAccess::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, FIELD_NAME | FIELD_ACCESS)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            FIELD_NAME => Self::FieldName(FieldName::cast(syntax)?),
            FIELD_ACCESS => Self::FieldAccess(FieldAccess::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::FieldName(node) => node.syntax(),
            Self::FieldAccess(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::FieldName(node) => node.into_syntax(),
            Self::FieldAccess(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldAccess {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl FieldAccess {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn field_name(&self) -> SyntaxResult<FieldName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for FieldAccess {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(FIELD_ACCESS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == FIELD_ACCESS
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for MethodName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodReturnType {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodReturnType {
    pub fn arrow_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn return_type(&self) -> SyntaxResult<ReturnType> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for MethodReturnType {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_RETURN_TYPE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_RETURN_TYPE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodImplementation {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodImplementation {
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn method_qualifier(&self) -> Option<MethodQualifier> {
        support::node(&self.syntax, 1usize)
    }
    pub fn method_body(&self) -> SyntaxResult<MethodBody> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for MethodImplementation {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_IMPLEMENTATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_IMPLEMENTATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnType {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ReturnType {
    pub fn type_declarator(&self) -> SyntaxResult<TypeDeclarator> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ReturnType {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(RETURN_TYPE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == RETURN_TYPE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodQualifier {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodQualifier {
    pub fn only_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for MethodQualifier {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_QUALIFIER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_QUALIFIER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodBody {
    MethodExpressionBody(MethodExpressionBody),
    MethodUndefinedBody(MethodUndefinedBody),
    MethodExternalBody(MethodExternalBody),
}
impl MethodBody {
    pub fn as_method_expression_body(&self) -> Option<&MethodExpressionBody> {
        match self {
            MethodBody::MethodExpressionBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_method_undefined_body(&self) -> Option<&MethodUndefinedBody> {
        match self {
            MethodBody::MethodUndefinedBody(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_method_external_body(&self) -> Option<&MethodExternalBody> {
        match self {
            MethodBody::MethodExternalBody(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for MethodBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = MethodExpressionBody::KIND_SET
        .union(MethodUndefinedBody::KIND_SET)
        .union(MethodExternalBody::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            METHOD_EXPRESSION_BODY | METHOD_UNDEFINED_BODY | METHOD_EXTERNAL_BODY
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            METHOD_EXPRESSION_BODY => {
                Self::MethodExpressionBody(MethodExpressionBody::cast(syntax)?)
            }
            METHOD_UNDEFINED_BODY => Self::MethodUndefinedBody(MethodUndefinedBody::cast(syntax)?),
            METHOD_EXTERNAL_BODY => Self::MethodExternalBody(MethodExternalBody::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::MethodExpressionBody(node) => node.syntax(),
            Self::MethodUndefinedBody(node) => node.syntax(),
            Self::MethodExternalBody(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::MethodExpressionBody(node) => node.into_syntax(),
            Self::MethodUndefinedBody(node) => node.into_syntax(),
            Self::MethodExternalBody(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExpressionBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodExpressionBody {
    pub fn expression_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for MethodExpressionBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_EXPRESSION_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_EXPRESSION_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodUndefinedBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodUndefinedBody {
    pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for MethodUndefinedBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_UNDEFINED_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_UNDEFINED_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodExternalBody {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodExternalBody {
    pub fn external_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn structured_identifier(&self) -> StructuredIdentifier {
        support::list(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for MethodExternalBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_EXTERNAL_BODY as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_EXTERNAL_BODY
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentList {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ArgumentList {
    pub fn positional_argument_list(&self) -> PositionalArgumentList {
        support::list(&self.syntax, 0usize)
    }
    pub fn named_argument_list(&self) -> NamedArgumentList {
        support::list(&self.syntax, 1usize)
    }
}
impl AstNode for ArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ARGUMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CoverageOperatorKind {
    Cover,
    Record,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoverageOperator {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl CoverageOperator {
    pub fn kind(&self) -> CoverageOperatorKind {
        match self.syntax.kind() {
            COVER_KW => CoverageOperatorKind::Cover,
            RECORD_KW => CoverageOperatorKind::Record,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for CoverageOperator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(COVERAGE_OPERATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == COVERAGE_OPERATOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierApplication {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierApplication {
    pub fn modifier_application_prefix(&self) -> Option<ModifierApplicationPrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn modifier_name(&self) -> SyntaxResult<ModifierName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for ModifierApplication {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_APPLICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_APPLICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModifierApplicationPrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ModifierApplicationPrefix {
    pub fn actor_expression(&self) -> SyntaxResult<ActorExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for ModifierApplicationPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MODIFIER_APPLICATION_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == MODIFIER_APPLICATION_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorExpression {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ActorExpression {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ActorExpression {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ACTOR_EXPRESSION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ACTOR_EXPRESSION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoDirective {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl DoDirective {
    pub fn do_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn do_member(&self) -> SyntaxResult<DoMember> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for DoDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(DO_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == DO_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnMemberList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for OnMemberList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ON_MEMBER_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ON_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for OnMemberList {
    type Language = OscDslLanguage;
    type Node = OnMember;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OnMember {
    CallDirective(CallDirective),
    EmitDirective(EmitDirective),
}
impl OnMember {
    pub fn as_call_directive(&self) -> Option<&CallDirective> {
        match self {
            OnMember::CallDirective(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_emit_directive(&self) -> Option<&EmitDirective> {
        match self {
            OnMember::EmitDirective(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for OnMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        CallDirective::KIND_SET.union(EmitDirective::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, CALL_DIRECTIVE | EMIT_DIRECTIVE)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            CALL_DIRECTIVE => Self::CallDirective(CallDirective::cast(syntax)?),
            EMIT_DIRECTIVE => Self::EmitDirective(EmitDirective::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::CallDirective(node) => node.syntax(),
            Self::EmitDirective(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::CallDirective(node) => node.into_syntax(),
            Self::EmitDirective(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallDirective {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl CallDirective {
    pub fn call_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn method_invocation(&self) -> SyntaxResult<MethodInvocation> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for CallDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CALL_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == CALL_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitDirective {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EmitDirective {
    pub fn emit_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_name(&self) -> SyntaxResult<EventName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn emit_arguments(&self) -> Option<EmitArguments> {
        support::node(&self.syntax, 2usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for EmitDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EMIT_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EMIT_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMember {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl DoMember {
    pub fn do_member_label(&self) -> Option<DoMemberLabel> {
        support::node(&self.syntax, 0usize)
    }
    pub fn do_member_body(&self) -> SyntaxResult<DoMemberBody> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for DoMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(DO_MEMBER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == DO_MEMBER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberLabel {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl DoMemberLabel {
    pub fn label_name(&self) -> SyntaxResult<LabelName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for DoMemberLabel {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(DO_MEMBER_LABEL as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == DO_MEMBER_LABEL
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoMemberBody {
    Composition(Composition),
    BehaviorInvocation(BehaviorInvocation),
    WaitDirective(WaitDirective),
    EmitDirective(EmitDirective),
    CallDirective(CallDirective),
}
impl DoMemberBody {
    pub fn as_composition(&self) -> Option<&Composition> {
        match self {
            DoMemberBody::Composition(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_behavior_invocation(&self) -> Option<&BehaviorInvocation> {
        match self {
            DoMemberBody::BehaviorInvocation(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_wait_directive(&self) -> Option<&WaitDirective> {
        match self {
            DoMemberBody::WaitDirective(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_emit_directive(&self) -> Option<&EmitDirective> {
        match self {
            DoMemberBody::EmitDirective(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_call_directive(&self) -> Option<&CallDirective> {
        match self {
            DoMemberBody::CallDirective(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for DoMemberBody {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = Composition::KIND_SET
        .union(BehaviorInvocation::KIND_SET)
        .union(WaitDirective::KIND_SET)
        .union(EmitDirective::KIND_SET)
        .union(CallDirective::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            COMPOSITION | BEHAVIOR_INVOCATION | WAIT_DIRECTIVE | EMIT_DIRECTIVE | CALL_DIRECTIVE
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            COMPOSITION => Self::Composition(Composition::cast(syntax)?),
            BEHAVIOR_INVOCATION => Self::BehaviorInvocation(BehaviorInvocation::cast(syntax)?),
            WAIT_DIRECTIVE => Self::WaitDirective(WaitDirective::cast(syntax)?),
            EMIT_DIRECTIVE => Self::EmitDirective(EmitDirective::cast(syntax)?),
            CALL_DIRECTIVE => Self::CallDirective(CallDirective::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::Composition(node) => node.syntax(),
            Self::BehaviorInvocation(node) => node.syntax(),
            Self::WaitDirective(node) => node.syntax(),
            Self::EmitDirective(node) => node.syntax(),
            Self::CallDirective(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::Composition(node) => node.into_syntax(),
            Self::BehaviorInvocation(node) => node.into_syntax(),
            Self::WaitDirective(node) => node.into_syntax(),
            Self::EmitDirective(node) => node.into_syntax(),
            Self::CallDirective(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl LabelName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for LabelName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(LABEL_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == LABEL_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composition {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl Composition {
    pub fn composition_operator(&self) -> SyntaxResult<CompositionOperator> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn composition_arguments(&self) -> Option<CompositionArguments> {
        support::node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn do_member_list(&self) -> DoMemberList {
        support::list(&self.syntax, 5usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn behavior_with_declaration(&self) -> Option<BehaviorWithDeclaration> {
        support::node(&self.syntax, 7usize)
    }
}
impl AstNode for Composition {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(COMPOSITION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == COMPOSITION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorInvocation {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BehaviorInvocation {
    pub fn behavior_invocation_prefix(&self) -> Option<BehaviorInvocationPrefix> {
        support::node(&self.syntax, 0usize)
    }
    pub fn behavior_name(&self) -> SyntaxResult<BehaviorName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn behavior_with_declaration_or_empty(
        &self,
    ) -> SyntaxResult<BehaviorWithDeclarationOrEmpty> {
        support::required_node(&self.syntax, 5usize)
    }
}
impl AstNode for BehaviorInvocation {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BEHAVIOR_INVOCATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BEHAVIOR_INVOCATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitDirective {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl WaitDirective {
    pub fn wait_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_specification(&self) -> SyntaxResult<EventSpecification> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for WaitDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(WAIT_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == WAIT_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompositionOperatorKind {
    Serial,
    OneOf,
    Parallel,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionOperator {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl CompositionOperator {
    pub fn kind(&self) -> CompositionOperatorKind {
        match self.syntax.kind() {
            SERIAL_KW => CompositionOperatorKind::Serial,
            ONE_OF_KW => CompositionOperatorKind::OneOf,
            PARALLEL_KW => CompositionOperatorKind::Parallel,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for CompositionOperator {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(COMPOSITION_OPERATOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == COMPOSITION_OPERATOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositionArguments {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl CompositionArguments {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn unqualified_argument_list(&self) -> SyntaxResult<UnqualifiedArgumentList> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for CompositionArguments {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(COMPOSITION_ARGUMENTS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == COMPOSITION_ARGUMENTS
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoMemberList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for DoMemberList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(DO_MEMBER_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == DO_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for DoMemberList {
    type Language = OscDslLanguage;
    type Node = DoMember;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithDeclaration {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BehaviorWithDeclaration {
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn indent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn behavior_with_member_list(&self) -> BehaviorWithMemberList {
        support::list(&self.syntax, 4usize)
    }
    pub fn dedent_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for BehaviorWithDeclaration {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BEHAVIOR_WITH_DECLARATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BEHAVIOR_WITH_DECLARATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedArgumentList {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedArgumentList {
    pub fn positional_argument_list(&self) -> PositionalArgumentList {
        support::list(&self.syntax, 0usize)
    }
    pub fn unqualified_named_argument_list(&self) -> UnqualifiedNamedArgumentList {
        support::list(&self.syntax, 1usize)
    }
}
impl AstNode for UnqualifiedArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNQUALIFIED_ARGUMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNQUALIFIED_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorInvocationPrefix {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BehaviorInvocationPrefix {
    pub fn actor_expression(&self) -> SyntaxResult<ActorExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl AstNode for BehaviorInvocationPrefix {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BEHAVIOR_INVOCATION_PREFIX as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BEHAVIOR_INVOCATION_PREFIX
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorWithDeclarationOrEmpty {
    BehaviorWithDeclaration(BehaviorWithDeclaration),
    Empty(Empty),
}
impl BehaviorWithDeclarationOrEmpty {
    pub fn as_behavior_with_declaration(&self) -> Option<&BehaviorWithDeclaration> {
        match self {
            BehaviorWithDeclarationOrEmpty::BehaviorWithDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_empty(&self) -> Option<&Empty> {
        match self {
            BehaviorWithDeclarationOrEmpty::Empty(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for BehaviorWithDeclarationOrEmpty {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        BehaviorWithDeclaration::KIND_SET.union(Empty::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(kind, BEHAVIOR_WITH_DECLARATION | EMPTY)
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            BEHAVIOR_WITH_DECLARATION => {
                Self::BehaviorWithDeclaration(BehaviorWithDeclaration::cast(syntax)?)
            }
            EMPTY => Self::Empty(Empty::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::BehaviorWithDeclaration(node) => node.syntax(),
            Self::Empty(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::BehaviorWithDeclaration(node) => node.into_syntax(),
            Self::Empty(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorWithMemberList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for BehaviorWithMemberList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BEHAVIOR_WITH_MEMBER_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BEHAVIOR_WITH_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstNodeList for BehaviorWithMemberList {
    type Language = OscDslLanguage;
    type Node = BehaviorWithMember;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BehaviorWithMember {
    ConstraintDeclaration(ConstraintDeclaration),
    ModifierApplication(ModifierApplication),
    UntilDirective(UntilDirective),
}
impl BehaviorWithMember {
    pub fn as_constraint_declaration(&self) -> Option<&ConstraintDeclaration> {
        match self {
            BehaviorWithMember::ConstraintDeclaration(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_modifier_application(&self) -> Option<&ModifierApplication> {
        match self {
            BehaviorWithMember::ModifierApplication(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_until_directive(&self) -> Option<&UntilDirective> {
        match self {
            BehaviorWithMember::UntilDirective(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for BehaviorWithMember {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = ConstraintDeclaration::KIND_SET
        .union(ModifierApplication::KIND_SET)
        .union(UntilDirective::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            CONSTRAINT_DECLARATION | MODIFIER_APPLICATION | UNTIL_DIRECTIVE
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            CONSTRAINT_DECLARATION => {
                Self::ConstraintDeclaration(ConstraintDeclaration::cast(syntax)?)
            }
            MODIFIER_APPLICATION => Self::ModifierApplication(ModifierApplication::cast(syntax)?),
            UNTIL_DIRECTIVE => Self::UntilDirective(UntilDirective::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ConstraintDeclaration(node) => node.syntax(),
            Self::ModifierApplication(node) => node.syntax(),
            Self::UntilDirective(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ConstraintDeclaration(node) => node.into_syntax(),
            Self::ModifierApplication(node) => node.into_syntax(),
            Self::UntilDirective(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntilDirective {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UntilDirective {
    pub fn until_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn event_specification(&self) -> SyntaxResult<EventSpecification> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn newline_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for UntilDirective {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNTIL_DIRECTIVE as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNTIL_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmitArguments {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl EmitArguments {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument_list(&self) -> SyntaxResult<ArgumentList> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for EmitArguments {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EMIT_ARGUMENTS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EMIT_ARGUMENTS
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodInvocation {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl MethodInvocation {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for MethodInvocation {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(METHOD_INVOCATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == METHOD_INVOCATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentSpecification {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ArgumentSpecification {
    pub fn argument_name(&self) -> SyntaxResult<ArgumentName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn type_declarator(&self) -> SyntaxResult<TypeDeclarator> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn argument_initializer(&self) -> Option<ArgumentInitializer> {
        support::node(&self.syntax, 3usize)
    }
}
impl AstNode for ArgumentSpecification {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ARGUMENT_SPECIFICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ARGUMENT_SPECIFICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ArgumentName {
    pub fn qualified_identifier(&self) -> SyntaxResult<QualifiedIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for ArgumentName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ARGUMENT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ARGUMENT_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentInitializer {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ArgumentInitializer {
    pub fn assign_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn default_value(&self) -> SyntaxResult<DefaultValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for ArgumentInitializer {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ARGUMENT_INITIALIZER as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ARGUMENT_INITIALIZER
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalArgumentList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for PositionalArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(POSITIONAL_ARGUMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == POSITIONAL_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for PositionalArgumentList {
    type Language = OscDslLanguage;
    type Node = PositionalArgument;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgumentList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for NamedArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(NAMED_ARGUMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == NAMED_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for NamedArgumentList {
    type Language = OscDslLanguage;
    type Node = NamedArgument;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositionalArgument {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PositionalArgument {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for PositionalArgument {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(POSITIONAL_ARGUMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == POSITIONAL_ARGUMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedArgument {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl NamedArgument {
    pub fn argument_name(&self) -> SyntaxResult<ArgumentName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for NamedArgument {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(NAMED_ARGUMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == NAMED_ARGUMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedNamedArgumentList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for UnqualifiedNamedArgumentList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNQUALIFIED_NAMED_ARGUMENT_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNQUALIFIED_NAMED_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for UnqualifiedNamedArgumentList {
    type Language = OscDslLanguage;
    type Node = UnqualifiedNamedArgument;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedNamedArgument {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedNamedArgument {
    pub fn unqualified_argument_name(&self) -> SyntaxResult<UnqualifiedArgumentName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for UnqualifiedNamedArgument {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNQUALIFIED_NAMED_ARGUMENT as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNQUALIFIED_NAMED_ARGUMENT
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnqualifiedArgumentName {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnqualifiedArgumentName {
    pub fn identifier_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for UnqualifiedArgumentName {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNQUALIFIED_ARGUMENT_NAME as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNQUALIFIED_ARGUMENT_NAME
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryOpExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl TernaryOpExp {
    pub fn condition(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn question_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn then(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn els(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl AstNode for TernaryOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TERNARY_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == TERNARY_OP_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOpExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl LogicalOpExp {
    pub fn lhs(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn logical_op(&self) -> SyntaxResult<LogicalOp> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn rhs(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for LogicalOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(LOGICAL_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == LOGICAL_OP_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOpExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BinaryOpExp {
    pub fn lhs(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn binary_op(&self) -> SyntaxResult<BinaryOp> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn rhs(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl AstNode for BinaryOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BINARY_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BINARY_OP_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOpExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnaryOpExp {
    pub fn unary_op(&self) -> SyntaxResult<UnaryOp> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl AstNode for UnaryOpExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNARY_OP_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNARY_OP_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl CastExp {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn type_declarator(&self) -> SyntaxResult<TypeDeclarator> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for CastExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CAST_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == CAST_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeTestExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl TypeTestExp {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn type_declarator(&self) -> SyntaxResult<TypeDeclarator> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for TypeTestExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(TYPE_TEST_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == TYPE_TEST_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElementAccess {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ElementAccess {
    pub fn object(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn index(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for ElementAccess {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(ELEMENT_ACCESS as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == ELEMENT_ACCESS
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionApplication {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl FunctionApplication {
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn argument_list(&self) -> Option<ArgumentList> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl AstNode for FunctionApplication {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(FUNCTION_APPLICATION as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == FUNCTION_APPLICATION
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ItExp {
    pub fn it_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for ItExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(IT_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == IT_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ParenthesizedExp {
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for ParenthesizedExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PARENTHESIZED_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PARENTHESIZED_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralExp {
    IntegerLiteralExp(IntegerLiteralExp),
    FloatLiteralExp(FloatLiteralExp),
    PhysicalLiteralExp(PhysicalLiteralExp),
    BoolLiteralExp(BoolLiteralExp),
    StringLiteralExp(StringLiteralExp),
}
impl LiteralExp {
    pub fn as_integer_literal_exp(&self) -> Option<&IntegerLiteralExp> {
        match self {
            LiteralExp::IntegerLiteralExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_float_literal_exp(&self) -> Option<&FloatLiteralExp> {
        match self {
            LiteralExp::FloatLiteralExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_physical_literal_exp(&self) -> Option<&PhysicalLiteralExp> {
        match self {
            LiteralExp::PhysicalLiteralExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_bool_literal_exp(&self) -> Option<&BoolLiteralExp> {
        match self {
            LiteralExp::BoolLiteralExp(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_string_literal_exp(&self) -> Option<&StringLiteralExp> {
        match self {
            LiteralExp::StringLiteralExp(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for LiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> = IntegerLiteralExp::KIND_SET
        .union(FloatLiteralExp::KIND_SET)
        .union(PhysicalLiteralExp::KIND_SET)
        .union(BoolLiteralExp::KIND_SET)
        .union(StringLiteralExp::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            INTEGER_LITERAL_EXP
                | FLOAT_LITERAL_EXP
                | PHYSICAL_LITERAL_EXP
                | BOOL_LITERAL_EXP
                | STRING_LITERAL_EXP
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            INTEGER_LITERAL_EXP => Self::IntegerLiteralExp(IntegerLiteralExp::cast(syntax)?),
            FLOAT_LITERAL_EXP => Self::FloatLiteralExp(FloatLiteralExp::cast(syntax)?),
            PHYSICAL_LITERAL_EXP => Self::PhysicalLiteralExp(PhysicalLiteralExp::cast(syntax)?),
            BOOL_LITERAL_EXP => Self::BoolLiteralExp(BoolLiteralExp::cast(syntax)?),
            STRING_LITERAL_EXP => Self::StringLiteralExp(StringLiteralExp::cast(syntax)?),
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::IntegerLiteralExp(node) => node.syntax(),
            Self::FloatLiteralExp(node) => node.syntax(),
            Self::PhysicalLiteralExp(node) => node.syntax(),
            Self::BoolLiteralExp(node) => node.syntax(),
            Self::StringLiteralExp(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::IntegerLiteralExp(node) => node.into_syntax(),
            Self::FloatLiteralExp(node) => node.into_syntax(),
            Self::PhysicalLiteralExp(node) => node.into_syntax(),
            Self::BoolLiteralExp(node) => node.into_syntax(),
            Self::StringLiteralExp(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListConstructor {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ListConstructor {
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression_list(&self) -> ExpressionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl AstNode for ListConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(LIST_CONSTRUCTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == LIST_CONSTRUCTOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeConstructor {
    ParenthesesRangeConstructor(ParenthesesRangeConstructor),
    BracketsRangeConstructor(BracketsRangeConstructor),
}
impl RangeConstructor {
    pub fn as_parentheses_range_constructor(&self) -> Option<&ParenthesesRangeConstructor> {
        match self {
            RangeConstructor::ParenthesesRangeConstructor(node) => Some(node),
            _ => None,
        }
    }
    pub fn as_brackets_range_constructor(&self) -> Option<&BracketsRangeConstructor> {
        match self {
            RangeConstructor::BracketsRangeConstructor(node) => Some(node),
            _ => None,
        }
    }
}
impl AstNode for RangeConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        ParenthesesRangeConstructor::KIND_SET.union(BracketsRangeConstructor::KIND_SET);
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        matches!(
            kind,
            PARENTHESES_RANGE_CONSTRUCTOR | BRACKETS_RANGE_CONSTRUCTOR
        )
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        let result = match syntax.kind() {
            PARENTHESES_RANGE_CONSTRUCTOR => {
                Self::ParenthesesRangeConstructor(ParenthesesRangeConstructor::cast(syntax)?)
            }
            BRACKETS_RANGE_CONSTRUCTOR => {
                Self::BracketsRangeConstructor(BracketsRangeConstructor::cast(syntax)?)
            }
            _ => return None,
        };
        Some(result)
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        match &self {
            Self::ParenthesesRangeConstructor(node) => node.syntax(),
            Self::BracketsRangeConstructor(node) => node.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        match self {
            Self::ParenthesesRangeConstructor(node) => node.into_syntax(),
            Self::BracketsRangeConstructor(node) => node.into_syntax(),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LogicalOpKind {
    FatArrow,
    Or,
    And,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalOp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl LogicalOp {
    pub fn kind(&self) -> LogicalOpKind {
        match self.syntax.kind() {
            FAT_ARROW => LogicalOpKind::FatArrow,
            OR_KW => LogicalOpKind::Or,
            AND_KW => LogicalOpKind::And,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for LogicalOp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(LOGICAL_OP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == LOGICAL_OP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOpKind {
    Equal,
    NotEqual,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    In,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BinaryOp {
    pub fn kind(&self) -> BinaryOpKind {
        match self.syntax.kind() {
            EQUAL => BinaryOpKind::Equal,
            NOT_EQUAL => BinaryOpKind::NotEqual,
            LESS => BinaryOpKind::Less,
            LESS_EQ => BinaryOpKind::LessEq,
            GREATER => BinaryOpKind::Greater,
            GREATER_EQ => BinaryOpKind::GreaterEq,
            IN_KW => BinaryOpKind::In,
            PLUS => BinaryOpKind::Plus,
            MINUS => BinaryOpKind::Minus,
            STAR => BinaryOpKind::Star,
            SLASH => BinaryOpKind::Slash,
            PERCENT => BinaryOpKind::Percent,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for BinaryOp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BINARY_OP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BINARY_OP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOpKind {
    Not,
    Minus,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl UnaryOp {
    pub fn kind(&self) -> UnaryOpKind {
        match self.syntax.kind() {
            NOT_KW => UnaryOpKind::Not,
            MINUS => UnaryOpKind::Minus,
            _ => unreachable!(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for UnaryOp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(UNARY_OP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == UNARY_OP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerLiteralExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl IntegerLiteralExp {
    pub fn integer_literal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for IntegerLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(INTEGER_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == INTEGER_LITERAL_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatLiteralExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl FloatLiteralExp {
    pub fn float_literal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for FloatLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(FLOAT_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == FLOAT_LITERAL_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalLiteralExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl PhysicalLiteralExp {
    pub fn physical_literal(&self) -> SyntaxResult<PhysicalLiteral> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for PhysicalLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PHYSICAL_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PHYSICAL_LITERAL_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolLiteralExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BoolLiteralExp {
    pub fn bool_literal(&self) -> SyntaxResult<BoolLiteral> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl AstNode for BoolLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BOOL_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BOOL_LITERAL_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteralExp {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl StringLiteralExp {
    pub fn string_literal_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl AstNode for StringLiteralExp {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(STRING_LITERAL_EXP as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == STRING_LITERAL_EXP
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionList {
    syntax_list: SyntaxList<OscDslLanguage>,
}
impl AstNode for ExpressionList {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(EXPRESSION_LIST as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == EXPRESSION_LIST
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self {
            syntax_list: syntax.into_list(),
        })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax_list.into_node()
    }
}
impl AstSeparatedList for ExpressionList {
    type Language = OscDslLanguage;
    type Node = Expression;
    fn syntax_list(&self) -> &SyntaxList<OscDslLanguage> {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList<OscDslLanguage> {
        self.syntax_list
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesesRangeConstructor {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl ParenthesesRangeConstructor {
    pub fn range_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn begin(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn end(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl AstNode for ParenthesesRangeConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PARENTHESES_RANGE_CONSTRUCTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == PARENTHESES_RANGE_CONSTRUCTOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketsRangeConstructor {
    syntax: SyntaxNode<OscDslLanguage>,
}
impl BracketsRangeConstructor {
    pub fn l_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn begin(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn dot_dot_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn end(&self) -> SyntaxResult<Expression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_bracket_token(&self) -> SyntaxResult<SyntaxToken<OscDslLanguage>> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl AstNode for BracketsRangeConstructor {
    type Language = OscDslLanguage;
    const KIND_SET: SyntaxKindSet<Self::Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(BRACKETS_RANGE_CONSTRUCTOR as u16));
    fn can_cast(kind: OscDslSyntaxKind) -> bool {
        kind == BRACKETS_RANGE_CONSTRUCTOR
    }
    fn cast(syntax: SyntaxNode<Self::Language>) -> Option<Self> {
        Self::can_cast(syntax.kind()).then(|| Self { syntax })
    }
    fn syntax(&self) -> &SyntaxNode<Self::Language> {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode<Self::Language> {
        self.syntax
    }
}