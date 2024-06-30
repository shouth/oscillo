#![allow(bad_style, missing_docs, unreachable_pub)]
use enumset::{EnumSet, EnumSetType};
use OscSyntaxKind::*;
#[derive(Debug, EnumSetType, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum OscSyntaxKind {
    EOF,
    DOT,
    DOT_DOT,
    COMMA,
    COLON,
    COLON_COLON,
    ASSIGN,
    AT,
    ARROW,
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    QUESTION,
    EXCLAMATION,
    FAT_ARROW,
    EQUAL,
    NOT_EQUAL,
    LESS,
    LESS_EQUAL,
    GREATER,
    GREATER_EQUAL,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENT,
    A_KW,
    ACTION_KW,
    ACTOR_KW,
    AND_KW,
    AS_KW,
    BOOL_KW,
    CALL_KW,
    CD_KW,
    COVER_KW,
    DEF_KW,
    DEFAULT_KW,
    DO_KW,
    ELAPSED_KW,
    EMIT_KW,
    ENUM_KW,
    EVENT_KW,
    EVERY_KW,
    EXPORT_KW,
    EXPRESSION_KW,
    EXTEND_KW,
    EXTERNAL_KW,
    FACTOR_KW,
    FALL_KW,
    FALSE_KW,
    FLOAT_KW,
    GLOBAL_KW,
    HARD_KW,
    IF_KW,
    IMPORT_KW,
    IN_KW,
    INF_KW,
    INHERITS_KW,
    INT_KW,
    IS_KW,
    IT_KW,
    K_KW,
    KEEP_KW,
    KG_KW,
    LIST_KW,
    M_KW,
    MODIFIER_KW,
    MOL_KW,
    NAMESPACE_KW,
    NAN_KW,
    NOT_KW,
    NULL_KW,
    OF_KW,
    OFFSET_KW,
    ON_KW,
    ONE_OF_KW,
    ONLY_KW,
    OR_KW,
    PARALLEL_KW,
    RAD_KW,
    RANGE_KW,
    RECORD_KW,
    REMOVE_DEFAULT_KW,
    RISE_KW,
    S_KW,
    SAMPLE_KW,
    SCENARIO_KW,
    SERIAL_KW,
    SI_KW,
    STRING_KW,
    STRUCT_KW,
    TRUE_KW,
    TYPE_KW,
    UINT_KW,
    UNDEFINED_KW,
    UNIT_KW,
    UNTIL_KW,
    USE_KW,
    VAR_KW,
    WAIT_KW,
    WITH_KW,
    INTEGER_LITERAL,
    FLOAT_LITERAL,
    STRING_LITERAL,
    NEWLINE,
    INDENT,
    DEDENT,
    IDENTIFIER,
    ERROR,
    WHITESPACE,
    COMMENT,
    TRIVIAL_NEWLINE,
    PREFIXED_IDENTIFIER,
    PHYSICAL_LITERAL,
    OSC_FILE,
    IMPORT_STATEMENT,
    PREFIXED_STRUCTURED_IDENTIFIER,
    NAMESPACE_STATEMENT,
    EXPORT_STATEMENT,
    NAMESPACE_USE_CLAUSE,
    NAMESPACE_LIST,
    NAMESPACE_LIST_ELEMENT,
    EXPORT_SPECIFICATION_LIST,
    EXPORT_SPECIFICATION_LIST_ELEMENT,
    EXPORT_WILDCARD_SPECIFICATION,
    PHYSICAL_TYPE_DECLARATION,
    UNIT_DECLARATION,
    ENUM_DECLARATION,
    STRUCT_DECLARATION,
    ACTOR_DECLARATION,
    ACTION_DECLARATION,
    SCENARIO_DECLARATION,
    MODIFIER_DECLARATION,
    TYPE_EXTENSION,
    GLOBAL_PARAMETER_DECLARATION,
    LIST_TYPE_DECLARATOR,
    SI_BASE_UNIT_SPECIFIER,
    SI_UNIT_SPECIFIER,
    SI_BASE_EXPONENT_LIST,
    SI_BASE_EXPONENT,
    SI_UNIT_ARGUMENT_LIST,
    SI_UNIT_ARGUMENT,
    ENUM_MEMBER_DECLS,
    ENUM_MEMBER_DECL_LIST,
    ENUM_MEMBER_DECL,
    ENUM_INITIALIZER_CLAUSE,
    STRUCT_INHERITS_CLAUSE,
    STRUCT_INHERITS_CONDITION,
    STRUCT_BODY,
    STRUCTURED_TYPE_MEMBER_LIST,
    ACTOR_INHERITS_CLAUSE,
    ACTOR_INHERITS_CONDITION,
    ACTOR_BODY,
    SCENARIO_INHERITS_CLAUSE,
    SCENARIO_INHERITS_CONDITION,
    SCENARIO_BODY,
    PREFIXED_BEHAVIOR_NAME,
    ACTION_INHERITS_CLAUSE,
    ACTION_BODY,
    ACTION_INHERITS_CONDITION,
    MODIFIER_OF_CLAUSE,
    MODIFIER_BODY,
    ENUM_TYPE_EXTENSION_BODY,
    STRUCTURED_TYPE_EXTENSION_BODY,
    PARAMETER_DECLARATION,
    EVENT_DECLARATION,
    METHOD_DECLARATION,
    COVERAGE_DECLARATION,
    MODIFIER_APPLICATION,
    ARGUMENT_SPECIFICATIONS,
    EVENT_IS_CLAUSE,
    EVENT_REFERENCE_SPECIFICATION,
    EVENT_REFERENCE,
    EVENT_REFERENCE_CONDITION,
    EVENT_FIELD_DECL,
    RISE_EXPRESSION,
    FALL_EXPRESSION,
    ELAPSED_EXPRESSION,
    EVERY_EXPRESSION,
    EVERY_EXP_OFFSET,
    VARIABLE_DECLARATION,
    FIELD_NAME_LIST,
    PARAMETER_INITIALIZER_CLAUSE,
    FIELD_NAME_LIST_ELEMENT,
    VARIABLE_INITIALIZER_CLAUSE,
    SAMPLE_EXPRESSION,
    PARAMETER_WITH_DECLARATION,
    PARAMETER_WITH_MEMBER_LIST,
    KEEP_CONSTRAINT_DECLARATION,
    REMOVE_DEFAULT_DECLARATION,
    METHOD_RETURN_TYPE,
    METHOD_IMPLEMENTATION,
    METHOD_EXPRESSION_BODY,
    METHOD_EXTERNAL_BODY,
    ARGUMENTS,
    ON_DIRECTIVE,
    DO_DIRECTIVE,
    ON_MEMBER_LIST,
    CALL_DIRECTIVE,
    EMIT_DIRECTIVE,
    DO_MEMBER,
    COMPOSITION,
    BEHAVIOR_INVOCATION,
    WAIT_DIRECTIVE,
    DO_MEMBER_LIST,
    BEHAVIOR_WITH_DECLARATION,
    BEHAVIOR_WITH_MEMBER_LIST,
    UNTIL_DIRECTIVE,
    ARGUMENT_SPECIFICATION_LIST,
    ARGUMENT_SPECIFICATION,
    ARGUMENT_INITIALIZER_CLAUSE,
    ARGUMENT_LIST,
    POSITIONAL_ARGUMENT,
    NAMED_ARGUMENT,
    TERNARY_EXP,
    LOGICAL_EXP,
    BINARY_EXP,
    UNARY_EXP,
    CAST_EXP,
    TYPE_TEST_EXP,
    ELEMENT_ACCESS,
    FUNCTION_APPLICATION,
    MEMBER_REFERENCE,
    PARENTHESIZED_EXP,
    ENUM_VALUE_REFERENCE,
    LIST_CONSTRUCTOR,
    EXPRESSION_LIST,
    EXPRESSION_LIST_ELEMENT,
    PARENTHESES_RANGE_CONSTRUCTOR,
    BRACKETS_RANGE_CONSTRUCTOR,
}
pub type OscSyntaxKindSet = EnumSet<OscSyntaxKind>;
impl OscSyntaxKind {
    pub fn static_token(&self) -> Option<&'static str> {
        match self {
            DOT => Some("."),
            DOT_DOT => Some(".."),
            COMMA => Some(","),
            COLON => Some(":"),
            COLON_COLON => Some("::"),
            ASSIGN => Some("="),
            AT => Some("@"),
            ARROW => Some("->"),
            LEFT_PAREN => Some("("),
            RIGHT_PAREN => Some(")"),
            LEFT_BRACKET => Some("["),
            RIGHT_BRACKET => Some("]"),
            QUESTION => Some("?"),
            EXCLAMATION => Some("!"),
            FAT_ARROW => Some("=>"),
            EQUAL => Some("=="),
            NOT_EQUAL => Some("!="),
            LESS => Some("<"),
            LESS_EQUAL => Some("<="),
            GREATER => Some(">"),
            GREATER_EQUAL => Some(">="),
            PLUS => Some("+"),
            MINUS => Some("-"),
            STAR => Some("*"),
            SLASH => Some("/"),
            PERCENT => Some("%"),
            A_KW => Some("A"),
            ACTION_KW => Some("action"),
            ACTOR_KW => Some("actor"),
            AND_KW => Some("and"),
            AS_KW => Some("as"),
            BOOL_KW => Some("bool"),
            CALL_KW => Some("call"),
            CD_KW => Some("cd"),
            COVER_KW => Some("cover"),
            DEF_KW => Some("def"),
            DEFAULT_KW => Some("default"),
            DO_KW => Some("do"),
            ELAPSED_KW => Some("elapsed"),
            EMIT_KW => Some("emit"),
            ENUM_KW => Some("enum"),
            EVENT_KW => Some("event"),
            EVERY_KW => Some("every"),
            EXPORT_KW => Some("export"),
            EXPRESSION_KW => Some("expression"),
            EXTEND_KW => Some("extend"),
            EXTERNAL_KW => Some("external"),
            FACTOR_KW => Some("factor"),
            FALL_KW => Some("fall"),
            FALSE_KW => Some("false"),
            FLOAT_KW => Some("float"),
            GLOBAL_KW => Some("global"),
            HARD_KW => Some("hard"),
            IF_KW => Some("if"),
            IMPORT_KW => Some("import"),
            IN_KW => Some("in"),
            INF_KW => Some("inf"),
            INHERITS_KW => Some("inherits"),
            INT_KW => Some("int"),
            IS_KW => Some("is"),
            IT_KW => Some("it"),
            K_KW => Some("K"),
            KEEP_KW => Some("keep"),
            KG_KW => Some("kg"),
            LIST_KW => Some("list"),
            M_KW => Some("m"),
            MODIFIER_KW => Some("modifier"),
            MOL_KW => Some("mol"),
            NAMESPACE_KW => Some("namespace"),
            NAN_KW => Some("nan"),
            NOT_KW => Some("not"),
            NULL_KW => Some("null"),
            OF_KW => Some("of"),
            OFFSET_KW => Some("offset"),
            ON_KW => Some("on"),
            ONE_OF_KW => Some("one_of"),
            ONLY_KW => Some("only"),
            OR_KW => Some("or"),
            PARALLEL_KW => Some("parallel"),
            RAD_KW => Some("rad"),
            RANGE_KW => Some("range"),
            RECORD_KW => Some("record"),
            REMOVE_DEFAULT_KW => Some("remove_default"),
            RISE_KW => Some("rise"),
            S_KW => Some("s"),
            SAMPLE_KW => Some("sample"),
            SCENARIO_KW => Some("scenario"),
            SERIAL_KW => Some("serial"),
            SI_KW => Some("SI"),
            STRING_KW => Some("string"),
            STRUCT_KW => Some("struct"),
            TRUE_KW => Some("true"),
            TYPE_KW => Some("type"),
            UINT_KW => Some("uint"),
            UNDEFINED_KW => Some("undefined"),
            UNIT_KW => Some("unit"),
            UNTIL_KW => Some("until"),
            USE_KW => Some("use"),
            VAR_KW => Some("var"),
            WAIT_KW => Some("wait"),
            WITH_KW => Some("with"),
            _ => None,
        }
    }
}