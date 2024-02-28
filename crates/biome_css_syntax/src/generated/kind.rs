//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum CssSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    TILDE,
    HASH,
    AMP,
    PIPE,
    PIPE2,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    DOT,
    COLON,
    COLON2,
    EQ,
    BANG,
    NEQ,
    MINUS,
    LTEQ,
    GTEQ,
    PLUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AT,
    DOLLAR_EQ,
    TILDE_EQ,
    CDC,
    CDO,
    MEDIA_KW,
    KEYFRAMES_KW,
    NOT_KW,
    AND_KW,
    ONLY_KW,
    OR_KW,
    I_KW,
    IMPORTANT_KW,
    HIGHLIGHT_KW,
    PART_KW,
    DIR_KW,
    LOCAL_KW,
    GLOBAL_KW,
    ANY_KW,
    CURRENT_KW,
    PAST_KW,
    FUTURE_KW,
    HOST_KW,
    HOST_CONTEXT_KW,
    MATCHES_KW,
    IS_KW,
    WHERE_KW,
    HAS_KW,
    LANG_KW,
    NTH_CHILD_KW,
    NTH_LAST_CHILD_KW,
    NTH_OF_TYPE_KW,
    NTH_LAST_OF_TYPE_KW,
    NTH_COL_KW,
    NTH_LAST_COL_KW,
    CHARSET_KW,
    COLOR_PROFILE_KW,
    COUNTER_STYLE_KW,
    PROPERTY_KW,
    CONTAINER_KW,
    STYLE_KW,
    LTR_KW,
    RTL_KW,
    N_KW,
    EVEN_KW,
    ODD_KW,
    OF_KW,
    FROM_KW,
    TO_KW,
    VAR_KW,
    URL_KW,
    SRC_KW,
    FONT_PALETTE_VALUES_KW,
    FONT_FEATURE_VALUES_KW,
    STYLISTIC_KW,
    HISTORICAL_FORMS_KW,
    STYLESET_KW,
    CHARACTER_VARIANT_KW,
    SWASH_KW,
    ORNAMENTS_KW,
    ANNOTATION_KW,
    AUTO_KW,
    THIN_KW,
    MEDIUM_KW,
    THICK_KW,
    NONE_KW,
    HIDDEN_KW,
    DOTTED_KW,
    DASHED_KW,
    SOLID_KW,
    DOUBLE_KW,
    GROOVE_KW,
    RIDGE_KW,
    INSET_KW,
    OUTSET_KW,
    INITIAL_KW,
    INHERIT_KW,
    UNSET_KW,
    REVERT_KW,
    REVERT_LAYER_KW,
    DEFAULT_KW,
    EM_KW,
    REM_KW,
    EX_KW,
    REX_KW,
    CAP_KW,
    RCAP_KW,
    CH_KW,
    RCH_KW,
    IC_KW,
    RIC_KW,
    LH_KW,
    RLH_KW,
    VW_KW,
    SVW_KW,
    LVW_KW,
    DVW_KW,
    VH_KW,
    SVH_KW,
    LVH_KW,
    DVH_KW,
    VI_KW,
    SVI_KW,
    LVI_KW,
    DVI_KW,
    VB_KW,
    SVB_KW,
    LVB_KW,
    DVB_KW,
    VMIN_KW,
    SVMIN_KW,
    LVMIN_KW,
    DVMIN_KW,
    VMAX_KW,
    SVMAX_KW,
    LVMAX_KW,
    DVMAX_KW,
    CM_KW,
    MM_KW,
    Q_KW,
    IN_KW,
    PC_KW,
    PT_KW,
    PX_KW,
    MOZMM_KW,
    RPX_KW,
    CQW_KW,
    CQH_KW,
    CQI_KW,
    CQB_KW,
    CQMIN_KW,
    CQMAX_KW,
    DEG_KW,
    GRAD_KW,
    RAD_KW,
    TURN_KW,
    S_KW,
    MS_KW,
    HZ_KW,
    KHZ_KW,
    DPI_KW,
    DPCM_KW,
    DPPX_KW,
    X_KW,
    FR_KW,
    PAGE_KW,
    LEFT_KW,
    RIGHT_KW,
    FIRST_KW,
    BLANK_KW,
    TOP_LEFT_CORNER_KW,
    TOP_LEFT_KW,
    TOP_CENTER_KW,
    TOP_RIGHT_KW,
    TOP_RIGHT_CORNER_KW,
    BOTTOM_LEFT_CORNER_KW,
    BOTTOM_LEFT_KW,
    BOTTOM_CENTER_KW,
    BOTTOM_RIGHT_KW,
    BOTTOM_RIGHT_CORNER_KW,
    LEFT_TOP_KW,
    LEFT_MIDDLE_KW,
    LEFT_BOTTOM_KW,
    RIGHT_TOP_KW,
    RIGHT_MIDDLE_KW,
    RIGHT_BOTTOM_KW,
    LAYER_KW,
    SCOPE_KW,
    SUPPORTS_KW,
    SELECTOR_KW,
    IMPORT_KW,
    NAMESPACE_KW,
    STARTING_STYLE_KW,
    DOCUMENT_KW,
    URL_PREFIX_KW,
    DOMAIN_KW,
    MEDIA_DOCUMENT_KW,
    REGEXP_KW,
    FONT_FACE_KW,
    CSS_STRING_LITERAL,
    CSS_NUMBER_LITERAL,
    CSS_DASHED_IDENTIFIER,
    CSS_CUSTOM_IDENTIFIER,
    CSS_SPACE_LITERAL,
    CSS_URL_VALUE_RAW_LITERAL,
    CSS_COLOR_LITERAL,
    CSS_DIMENSION_VALUE,
    CSS_PERCENTAGE_VALUE,
    ERROR_TOKEN,
    IDENT,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    MULTILINE_COMMENT,
    CSS_ROOT,
    CSS_RULE_LIST,
    CSS_QUALIFIED_RULE,
    CSS_NESTED_QUALIFIED_RULE,
    CSS_SELECTOR_LIST,
    CSS_ANY_FUNCTION,
    CSS_DECLARATION_LIST_BLOCK,
    CSS_RULE_LIST_BLOCK,
    CSS_DECLARATION_OR_AT_RULE_BLOCK,
    CSS_DECLARATION_OR_RULE_BLOCK,
    CSS_DECLARATION_OR_RULE_LIST,
    CSS_DECLARATION_OR_AT_RULE_LIST,
    CSS_DECLARATION_WITH_SEMICOLON,
    CSS_DECLARATION,
    CSS_IDENTIFIER,
    CSS_NUMBER,
    CSS_PARAMETER,
    CSS_PERCENTAGE,
    CSS_RATIO,
    CSS_FUNCTION,
    CSS_STRING,
    CSS_VAR_FUNCTION,
    CSS_VAR_FUNCTION_VALUE,
    CSS_ATTRIBUTE_LIST,
    CSS_DECLARATION_LIST,
    CSS_COMPONENT_VALUE_LIST,
    CSS_GENERIC_COMPONENT_VALUE_LIST,
    CSS_GENERIC_DELIMITER,
    CSS_GENERIC_PROPERTY,
    CSS_UNKNOWN_PROPERTY_VALUE,
    CSS_PARAMETER_LIST,
    CSS_DECLARATION_IMPORTANT,
    CSS_REGULAR_DIMENSION,
    CSS_UNKNOWN_DIMENSION,
    CSS_NAMESPACE,
    CSS_NAMED_NAMESPACE_PREFIX,
    CSS_UNIVERSAL_NAMESPACE_PREFIX,
    CSS_ANY_SELECTOR_LIST,
    CSS_COMPLEX_SELECTOR,
    CSS_COMPOUND_SELECTOR,
    CSS_SUB_SELECTOR_LIST,
    CSS_ID_SELECTOR,
    CSS_CLASS_SELECTOR,
    CSS_TYPE_SELECTOR,
    CSS_UNIVERSAL_SELECTOR,
    CSS_PSEUDO_CLASS_SELECTOR,
    CSS_PSEUDO_CLASS_SELECTOR_PARAMETERS,
    CSS_PSEUDO_ELEMENT_SELECTOR,
    CSS_PSEUDO_ELEMENT_IDENTIFIER,
    CSS_PSEUDO_ELEMENT_FUNCTION_SELECTOR,
    CSS_PSEUDO_ELEMENT_FUNCTION_IDENTIFIER,
    CSS_PSEUDO_CLASS_IDENTIFIER,
    CSS_PSEUDO_CLASS_FUNCTION_IDENTIFIER,
    CSS_PSEUDO_CLASS_FUNCTION_SELECTOR,
    CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR,
    CSS_COMPOUND_SELECTOR_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST,
    CSS_RELATIVE_SELECTOR_LIST,
    CSS_RELATIVE_SELECTOR,
    CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST,
    CSS_PSEUDO_VALUE_LIST,
    CSS_PSEUDO_CLASS_FUNCTION_NTH,
    CSS_PSEUDO_CLASS_NTH_SELECTOR,
    CSS_PSEUDO_CLASS_NTH,
    CSS_PSEUDO_CLASS_NTH_NUMBER,
    CSS_PSEUDO_CLASS_NTH_IDENTIFIER,
    CSS_NTH_OFFSET,
    CSS_PSEUDO_CLASS_OF_NTH_SELECTOR,
    CSS_ATTRIBUTE_SELECTOR,
    CSS_ATTRIBUTE,
    CSS_ATTRIBUTE_NAME,
    CSS_ATTRIBUTE_MATCHER,
    CSS_ATTRIBUTE_MATCHER_VALUE,
    CSS_PARENTHESIZED_EXPRESSION,
    CSS_LIST_OF_COMPONENT_VALUES_EXPRESSION,
    CSS_BINARY_EXPRESSION,
    CSS_URL_VALUE_RAW,
    CSS_URL_FUNCTION,
    CSS_URL_MODIFIER_LIST,
    CSS_COLOR,
    CSS_BORDER,
    CSS_AT_RULE,
    CSS_CHARSET_AT_RULE,
    CSS_COLOR_PROFILE_AT_RULE,
    CSS_COUNTER_STYLE_AT_RULE,
    CSS_PROPERTY_AT_RULE,
    CSS_CONTAINER_AT_RULE,
    CSS_CONTAINER_NOT_QUERY,
    CSS_CONTAINER_AND_QUERY,
    CSS_CONTAINER_OR_QUERY,
    CSS_CONTAINER_QUERY_IN_PARENS,
    CSS_CONTAINER_STYLE_QUERY_IN_PARENS,
    CSS_CONTAINER_SIZE_FEATURE_IN_PARENS,
    CSS_CONTAINER_STYLE_NOT_QUERY,
    CSS_CONTAINER_STYLE_AND_QUERY,
    CSS_CONTAINER_STYLE_OR_QUERY,
    CSS_CONTAINER_STYLE_IN_PARENS,
    CSS_FONT_FACE_AT_RULE,
    CSS_FONT_FEATURE_VALUES_AT_RULE,
    CSS_FONT_FEATURE_VALUES_BLOCK,
    CSS_FONT_FEATURE_VALUES_ITEM,
    CSS_FONT_FEATURE_VALUES_ITEM_LIST,
    CSS_FONT_FEATURE_VALUES_STYLISTIC,
    CSS_FONT_FEATURE_VALUES_HISTORICAL_FORMS,
    CSS_FONT_FEATURE_VALUES_STYLESET,
    CSS_FONT_FEATURE_VALUES_CHARACTER_VARIANT,
    CSS_FONT_FEATURE_VALUES_SWASH,
    CSS_FONT_FEATURE_VALUES_ORNAMENTS,
    CSS_FONT_FEATURE_VALUES_ANNOTATION,
    CSS_FONT_PALETTE_VALUES_AT_RULE,
    CSS_KEYFRAMES_AT_RULE,
    CSS_KEYFRAMES_BODY,
    CSS_MEDIA_AT_RULE,
    CSS_MEDIA_QUERY_LIST,
    CSS_MEDIA_QUERY,
    CSS_MEDIA_CONDITION_QUERY,
    CSS_MEDIA_TYPE_QUERY,
    CSS_MEDIA_AND_TYPE_QUERY,
    CSS_MEDIA_TYPE,
    CSS_MEDIA_NOT_CONDITION,
    CSS_MEDIA_AND_CONDITION,
    CSS_MEDIA_OR_CONDITION,
    CSS_MEDIA_CONDITION_IN_PARENS,
    CSS_MEDIA_FEATURE_IN_PARENS,
    CSS_QUERY_FEATURE_PLAIN,
    CSS_QUERY_FEATURE_BOOLEAN,
    CSS_QUERY_FEATURE_RANGE,
    CSS_QUERY_FEATURE_REVERSE_RANGE,
    CSS_QUERY_FEATURE_RANGE_INTERVAL,
    CSS_QUERY_FEATURE_RANGE_COMPARISON,
    CSS_KEYFRAMES_BLOCK,
    CSS_KEYFRAMES_ITEM_LIST,
    CSS_KEYFRAMES_ITEM,
    CSS_KEYFRAMES_IDENT_SELECTOR,
    CSS_KEYFRAMES_PERCENTAGE_SELECTOR,
    CSS_KEYFRAMES_SELECTOR_LIST,
    CSS_PAGE_AT_RULE,
    CSS_PAGE_SELECTOR_LIST,
    CSS_PAGE_SELECTOR,
    CSS_PAGE_SELECTOR_PSEUDO_LIST,
    CSS_PAGE_SELECTOR_PSEUDO,
    CSS_PAGE_AT_RULE_BLOCK,
    CSS_PAGE_AT_RULE_ITEM_LIST,
    CSS_MARGIN_AT_RULE,
    CSS_LAYER_AT_RULE,
    CSS_LAYER_REFERENCE,
    CSS_LAYER_REFERENCE_LIST,
    CSS_LAYER_NAME_LIST,
    CSS_LAYER_DECLARATION,
    CSS_SUPPORTS_AT_RULE,
    CSS_SUPPORTS_NOT_CONDITION,
    CSS_SUPPORTS_AND_CONDITION,
    CSS_SUPPORTS_OR_CONDITION,
    CSS_SUPPORTS_CONDITION_IN_PARENS,
    CSS_SUPPORTS_FEATURE_DECLARATION,
    CSS_SUPPORTS_FEATURE_SELECTOR,
    CSS_SCOPE_AT_RULE,
    CSS_SCOPE_RANGE_START,
    CSS_SCOPE_RANGE_END,
    CSS_SCOPE_RANGE_INTERVAL,
    CSS_SCOPE_EDGE,
    CSS_IMPORT_AT_RULE,
    CSS_IMPORT_ANONYMOUS_LAYER,
    CSS_IMPORT_NAMED_LAYER,
    CSS_IMPORT_SUPPORTS,
    CSS_NAMESPACE_AT_RULE,
    CSS_STARTING_STYLE_AT_RULE,
    CSS_DOCUMENT_AT_RULE,
    CSS_DOCUMENT_MATCHER_LIST,
    CSS_DOCUMENT_CUSTOM_MATCHER,
    CSS_BOGUS,
    CSS_BOGUS_BLOCK,
    CSS_BOGUS_KEYFRAMES_ITEM,
    CSS_BOGUS_RULE,
    CSS_BOGUS_SELECTOR,
    CSS_BOGUS_SUB_SELECTOR,
    CSS_BOGUS_PSEUDO_CLASS,
    CSS_BOGUS_PSEUDO_ELEMENT,
    CSS_BOGUS_AT_RULE,
    CSS_BOGUS_LAYER,
    CSS_BOGUS_PAGE_SELECTOR_PSEUDO,
    CSS_BOGUS_DECLARATION_ITEM,
    CSS_BOGUS_COMPONENT_VALUE,
    CSS_BOGUS_PARAMETER,
    CSS_BOGUS_PROPERTY,
    CSS_BOGUS_PROPERTY_VALUE,
    CSS_BOGUS_MEDIA_QUERY,
    CSS_BOGUS_SCOPE_RANGE,
    CSS_BOGUS_URL_MODIFIER,
    CSS_BOGUS_DOCUMENT_MATCHER,
    CSS_BOGUS_FONT_FEATURE_VALUES_ITEM,
    #[doc(hidden)]
    __LAST,
}
use self::CssSyntaxKind::*;
impl CssSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
            | L_ANGLE | R_ANGLE | TILDE | HASH | AMP | PIPE | PIPE2 | PLUS | STAR | SLASH
            | CARET | PERCENT | DOT | COLON | COLON2 | EQ | BANG | NEQ | MINUS | LTEQ | GTEQ
            | PLUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AT | DOLLAR_EQ
            | TILDE_EQ | CDC | CDO => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            CSS_STRING_LITERAL
            | CSS_NUMBER_LITERAL
            | CSS_DASHED_IDENTIFIER
            | CSS_CUSTOM_IDENTIFIER
            | CSS_SPACE_LITERAL
            | CSS_URL_VALUE_RAW_LITERAL
            | CSS_COLOR_LITERAL
            | CSS_DIMENSION_VALUE
            | CSS_PERCENTAGE_VALUE => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            CSS_RULE_LIST
            | CSS_SELECTOR_LIST
            | CSS_DECLARATION_OR_RULE_LIST
            | CSS_DECLARATION_OR_AT_RULE_LIST
            | CSS_ATTRIBUTE_LIST
            | CSS_DECLARATION_LIST
            | CSS_COMPONENT_VALUE_LIST
            | CSS_GENERIC_COMPONENT_VALUE_LIST
            | CSS_PARAMETER_LIST
            | CSS_ANY_SELECTOR_LIST
            | CSS_SUB_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_COMPOUND_SELECTOR_LIST
            | CSS_COMPOUND_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_RELATIVE_SELECTOR_LIST
            | CSS_RELATIVE_SELECTOR_LIST
            | CSS_PSEUDO_CLASS_FUNCTION_VALUE_LIST
            | CSS_PSEUDO_VALUE_LIST
            | CSS_URL_MODIFIER_LIST
            | CSS_FONT_FEATURE_VALUES_ITEM_LIST
            | CSS_MEDIA_QUERY_LIST
            | CSS_KEYFRAMES_ITEM_LIST
            | CSS_KEYFRAMES_SELECTOR_LIST
            | CSS_PAGE_SELECTOR_LIST
            | CSS_PAGE_SELECTOR_PSEUDO_LIST
            | CSS_PAGE_AT_RULE_ITEM_LIST
            | CSS_LAYER_REFERENCE_LIST
            | CSS_LAYER_NAME_LIST
            | CSS_DOCUMENT_MATCHER_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<CssSyntaxKind> {
        let kw = match ident {
            "media" => MEDIA_KW,
            "keyframes" => KEYFRAMES_KW,
            "not" => NOT_KW,
            "and" => AND_KW,
            "only" => ONLY_KW,
            "or" => OR_KW,
            "i" => I_KW,
            "important" => IMPORTANT_KW,
            "highlight" => HIGHLIGHT_KW,
            "part" => PART_KW,
            "dir" => DIR_KW,
            "local" => LOCAL_KW,
            "global" => GLOBAL_KW,
            "any" => ANY_KW,
            "current" => CURRENT_KW,
            "past" => PAST_KW,
            "future" => FUTURE_KW,
            "host" => HOST_KW,
            "host-context" => HOST_CONTEXT_KW,
            "matches" => MATCHES_KW,
            "is" => IS_KW,
            "where" => WHERE_KW,
            "has" => HAS_KW,
            "lang" => LANG_KW,
            "nth-child" => NTH_CHILD_KW,
            "nth-last-child" => NTH_LAST_CHILD_KW,
            "nth-of-type" => NTH_OF_TYPE_KW,
            "nth-last-of-type" => NTH_LAST_OF_TYPE_KW,
            "nth-col" => NTH_COL_KW,
            "nth-last-col" => NTH_LAST_COL_KW,
            "charset" => CHARSET_KW,
            "color-profile" => COLOR_PROFILE_KW,
            "counter-style" => COUNTER_STYLE_KW,
            "property" => PROPERTY_KW,
            "container" => CONTAINER_KW,
            "style" => STYLE_KW,
            "ltr" => LTR_KW,
            "rtl" => RTL_KW,
            "n" => N_KW,
            "even" => EVEN_KW,
            "odd" => ODD_KW,
            "of" => OF_KW,
            "from" => FROM_KW,
            "to" => TO_KW,
            "var" => VAR_KW,
            "url" => URL_KW,
            "src" => SRC_KW,
            "font-palette-values" => FONT_PALETTE_VALUES_KW,
            "font-feature-values" => FONT_FEATURE_VALUES_KW,
            "stylistic" => STYLISTIC_KW,
            "historical-forms" => HISTORICAL_FORMS_KW,
            "styleset" => STYLESET_KW,
            "character-variant" => CHARACTER_VARIANT_KW,
            "swash" => SWASH_KW,
            "ornaments" => ORNAMENTS_KW,
            "annotation" => ANNOTATION_KW,
            "auto" => AUTO_KW,
            "thin" => THIN_KW,
            "medium" => MEDIUM_KW,
            "thick" => THICK_KW,
            "none" => NONE_KW,
            "hidden" => HIDDEN_KW,
            "dotted" => DOTTED_KW,
            "dashed" => DASHED_KW,
            "solid" => SOLID_KW,
            "double" => DOUBLE_KW,
            "groove" => GROOVE_KW,
            "ridge" => RIDGE_KW,
            "inset" => INSET_KW,
            "outset" => OUTSET_KW,
            "initial" => INITIAL_KW,
            "inherit" => INHERIT_KW,
            "unset" => UNSET_KW,
            "revert" => REVERT_KW,
            "revert-layer" => REVERT_LAYER_KW,
            "default" => DEFAULT_KW,
            "em" => EM_KW,
            "rem" => REM_KW,
            "ex" => EX_KW,
            "rex" => REX_KW,
            "cap" => CAP_KW,
            "rcap" => RCAP_KW,
            "ch" => CH_KW,
            "rch" => RCH_KW,
            "ic" => IC_KW,
            "ric" => RIC_KW,
            "lh" => LH_KW,
            "rlh" => RLH_KW,
            "vw" => VW_KW,
            "svw" => SVW_KW,
            "lvw" => LVW_KW,
            "dvw" => DVW_KW,
            "vh" => VH_KW,
            "svh" => SVH_KW,
            "lvh" => LVH_KW,
            "dvh" => DVH_KW,
            "vi" => VI_KW,
            "svi" => SVI_KW,
            "lvi" => LVI_KW,
            "dvi" => DVI_KW,
            "vb" => VB_KW,
            "svb" => SVB_KW,
            "lvb" => LVB_KW,
            "dvb" => DVB_KW,
            "vmin" => VMIN_KW,
            "svmin" => SVMIN_KW,
            "lvmin" => LVMIN_KW,
            "dvmin" => DVMIN_KW,
            "vmax" => VMAX_KW,
            "svmax" => SVMAX_KW,
            "lvmax" => LVMAX_KW,
            "dvmax" => DVMAX_KW,
            "cm" => CM_KW,
            "mm" => MM_KW,
            "q" => Q_KW,
            "in" => IN_KW,
            "pc" => PC_KW,
            "pt" => PT_KW,
            "px" => PX_KW,
            "mozmm" => MOZMM_KW,
            "rpx" => RPX_KW,
            "cqw" => CQW_KW,
            "cqh" => CQH_KW,
            "cqi" => CQI_KW,
            "cqb" => CQB_KW,
            "cqmin" => CQMIN_KW,
            "cqmax" => CQMAX_KW,
            "deg" => DEG_KW,
            "grad" => GRAD_KW,
            "rad" => RAD_KW,
            "turn" => TURN_KW,
            "s" => S_KW,
            "ms" => MS_KW,
            "hz" => HZ_KW,
            "khz" => KHZ_KW,
            "dpi" => DPI_KW,
            "dpcm" => DPCM_KW,
            "dppx" => DPPX_KW,
            "x" => X_KW,
            "fr" => FR_KW,
            "page" => PAGE_KW,
            "left" => LEFT_KW,
            "right" => RIGHT_KW,
            "first" => FIRST_KW,
            "blank" => BLANK_KW,
            "top-left-corner" => TOP_LEFT_CORNER_KW,
            "top-left" => TOP_LEFT_KW,
            "top-center" => TOP_CENTER_KW,
            "top-right" => TOP_RIGHT_KW,
            "top-right-corner" => TOP_RIGHT_CORNER_KW,
            "bottom-left-corner" => BOTTOM_LEFT_CORNER_KW,
            "bottom-left" => BOTTOM_LEFT_KW,
            "bottom-center" => BOTTOM_CENTER_KW,
            "bottom-right" => BOTTOM_RIGHT_KW,
            "bottom-right-corner" => BOTTOM_RIGHT_CORNER_KW,
            "left-top" => LEFT_TOP_KW,
            "left-middle" => LEFT_MIDDLE_KW,
            "left-bottom" => LEFT_BOTTOM_KW,
            "right-top" => RIGHT_TOP_KW,
            "right-middle" => RIGHT_MIDDLE_KW,
            "right-bottom" => RIGHT_BOTTOM_KW,
            "layer" => LAYER_KW,
            "scope" => SCOPE_KW,
            "supports" => SUPPORTS_KW,
            "selector" => SELECTOR_KW,
            "import" => IMPORT_KW,
            "namespace" => NAMESPACE_KW,
            "starting-style" => STARTING_STYLE_KW,
            "document" => DOCUMENT_KW,
            "url-prefix" => URL_PREFIX_KW,
            "domain" => DOMAIN_KW,
            "media-document" => MEDIA_DOCUMENT_KW,
            "regexp" => REGEXP_KW,
            "font-face" => FONT_FACE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SEMICOLON => ";",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            TILDE => "~",
            HASH => "#",
            AMP => "&",
            PIPE => "|",
            PIPE2 => "||",
            PLUS => "+",
            STAR => "*",
            SLASH => "/",
            CARET => "^",
            PERCENT => "%",
            DOT => ".",
            COLON => ":",
            COLON2 => "::",
            EQ => "=",
            BANG => "!",
            NEQ => "!=",
            MINUS => "-",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            PIPEEQ => "|=",
            AMPEQ => "&=",
            CARETEQ => "^=",
            SLASHEQ => "/=",
            STAREQ => "*=",
            PERCENTEQ => "%=",
            AT => "@",
            DOLLAR_EQ => "$=",
            TILDE_EQ => "~=",
            CDC => "-->",
            CDO => "<!--",
            MEDIA_KW => "media",
            KEYFRAMES_KW => "keyframes",
            NOT_KW => "not",
            AND_KW => "and",
            ONLY_KW => "only",
            OR_KW => "or",
            I_KW => "i",
            IMPORTANT_KW => "important",
            HIGHLIGHT_KW => "highlight",
            PART_KW => "part",
            DIR_KW => "dir",
            LOCAL_KW => "local",
            GLOBAL_KW => "global",
            ANY_KW => "any",
            CURRENT_KW => "current",
            PAST_KW => "past",
            FUTURE_KW => "future",
            HOST_KW => "host",
            HOST_CONTEXT_KW => "host-context",
            MATCHES_KW => "matches",
            IS_KW => "is",
            WHERE_KW => "where",
            HAS_KW => "has",
            LANG_KW => "lang",
            NTH_CHILD_KW => "nth-child",
            NTH_LAST_CHILD_KW => "nth-last-child",
            NTH_OF_TYPE_KW => "nth-of-type",
            NTH_LAST_OF_TYPE_KW => "nth-last-of-type",
            NTH_COL_KW => "nth-col",
            NTH_LAST_COL_KW => "nth-last-col",
            CHARSET_KW => "charset",
            COLOR_PROFILE_KW => "color-profile",
            COUNTER_STYLE_KW => "counter-style",
            PROPERTY_KW => "property",
            CONTAINER_KW => "container",
            STYLE_KW => "style",
            LTR_KW => "ltr",
            RTL_KW => "rtl",
            N_KW => "n",
            EVEN_KW => "even",
            ODD_KW => "odd",
            OF_KW => "of",
            FROM_KW => "from",
            TO_KW => "to",
            VAR_KW => "var",
            URL_KW => "url",
            SRC_KW => "src",
            FONT_PALETTE_VALUES_KW => "font-palette-values",
            FONT_FEATURE_VALUES_KW => "font-feature-values",
            STYLISTIC_KW => "stylistic",
            HISTORICAL_FORMS_KW => "historical-forms",
            STYLESET_KW => "styleset",
            CHARACTER_VARIANT_KW => "character-variant",
            SWASH_KW => "swash",
            ORNAMENTS_KW => "ornaments",
            ANNOTATION_KW => "annotation",
            AUTO_KW => "auto",
            THIN_KW => "thin",
            MEDIUM_KW => "medium",
            THICK_KW => "thick",
            NONE_KW => "none",
            HIDDEN_KW => "hidden",
            DOTTED_KW => "dotted",
            DASHED_KW => "dashed",
            SOLID_KW => "solid",
            DOUBLE_KW => "double",
            GROOVE_KW => "groove",
            RIDGE_KW => "ridge",
            INSET_KW => "inset",
            OUTSET_KW => "outset",
            INITIAL_KW => "initial",
            INHERIT_KW => "inherit",
            UNSET_KW => "unset",
            REVERT_KW => "revert",
            REVERT_LAYER_KW => "revert-layer",
            DEFAULT_KW => "default",
            EM_KW => "em",
            REM_KW => "rem",
            EX_KW => "ex",
            REX_KW => "rex",
            CAP_KW => "cap",
            RCAP_KW => "rcap",
            CH_KW => "ch",
            RCH_KW => "rch",
            IC_KW => "ic",
            RIC_KW => "ric",
            LH_KW => "lh",
            RLH_KW => "rlh",
            VW_KW => "vw",
            SVW_KW => "svw",
            LVW_KW => "lvw",
            DVW_KW => "dvw",
            VH_KW => "vh",
            SVH_KW => "svh",
            LVH_KW => "lvh",
            DVH_KW => "dvh",
            VI_KW => "vi",
            SVI_KW => "svi",
            LVI_KW => "lvi",
            DVI_KW => "dvi",
            VB_KW => "vb",
            SVB_KW => "svb",
            LVB_KW => "lvb",
            DVB_KW => "dvb",
            VMIN_KW => "vmin",
            SVMIN_KW => "svmin",
            LVMIN_KW => "lvmin",
            DVMIN_KW => "dvmin",
            VMAX_KW => "vmax",
            SVMAX_KW => "svmax",
            LVMAX_KW => "lvmax",
            DVMAX_KW => "dvmax",
            CM_KW => "cm",
            MM_KW => "mm",
            Q_KW => "q",
            IN_KW => "in",
            PC_KW => "pc",
            PT_KW => "pt",
            PX_KW => "px",
            MOZMM_KW => "mozmm",
            RPX_KW => "rpx",
            CQW_KW => "cqw",
            CQH_KW => "cqh",
            CQI_KW => "cqi",
            CQB_KW => "cqb",
            CQMIN_KW => "cqmin",
            CQMAX_KW => "cqmax",
            DEG_KW => "deg",
            GRAD_KW => "grad",
            RAD_KW => "rad",
            TURN_KW => "turn",
            S_KW => "s",
            MS_KW => "ms",
            HZ_KW => "hz",
            KHZ_KW => "khz",
            DPI_KW => "dpi",
            DPCM_KW => "dpcm",
            DPPX_KW => "dppx",
            X_KW => "x",
            FR_KW => "fr",
            PAGE_KW => "page",
            LEFT_KW => "left",
            RIGHT_KW => "right",
            FIRST_KW => "first",
            BLANK_KW => "blank",
            TOP_LEFT_CORNER_KW => "top-left-corner",
            TOP_LEFT_KW => "top-left",
            TOP_CENTER_KW => "top-center",
            TOP_RIGHT_KW => "top-right",
            TOP_RIGHT_CORNER_KW => "top-right-corner",
            BOTTOM_LEFT_CORNER_KW => "bottom-left-corner",
            BOTTOM_LEFT_KW => "bottom-left",
            BOTTOM_CENTER_KW => "bottom-center",
            BOTTOM_RIGHT_KW => "bottom-right",
            BOTTOM_RIGHT_CORNER_KW => "bottom-right-corner",
            LEFT_TOP_KW => "left-top",
            LEFT_MIDDLE_KW => "left-middle",
            LEFT_BOTTOM_KW => "left-bottom",
            RIGHT_TOP_KW => "right-top",
            RIGHT_MIDDLE_KW => "right-middle",
            RIGHT_BOTTOM_KW => "right-bottom",
            LAYER_KW => "layer",
            SCOPE_KW => "scope",
            SUPPORTS_KW => "supports",
            SELECTOR_KW => "selector",
            IMPORT_KW => "import",
            NAMESPACE_KW => "namespace",
            STARTING_STYLE_KW => "starting-style",
            DOCUMENT_KW => "document",
            URL_PREFIX_KW => "url-prefix",
            DOMAIN_KW => "domain",
            MEDIA_DOCUMENT_KW => "media-document",
            REGEXP_KW => "regexp",
            FONT_FACE_KW => "font-face",
            CSS_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: CssSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: CssSyntaxKind :: COMMA } ; ['('] => { $ crate :: CssSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: CssSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: CssSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: CssSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: CssSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: CssSyntaxKind :: R_BRACK } ; [<] => { $ crate :: CssSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: CssSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: CssSyntaxKind :: TILDE } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; [&] => { $ crate :: CssSyntaxKind :: AMP } ; [|] => { $ crate :: CssSyntaxKind :: PIPE } ; [||] => { $ crate :: CssSyntaxKind :: PIPE2 } ; [+] => { $ crate :: CssSyntaxKind :: PLUS } ; [*] => { $ crate :: CssSyntaxKind :: STAR } ; [/] => { $ crate :: CssSyntaxKind :: SLASH } ; [^] => { $ crate :: CssSyntaxKind :: CARET } ; [%] => { $ crate :: CssSyntaxKind :: PERCENT } ; [.] => { $ crate :: CssSyntaxKind :: DOT } ; [:] => { $ crate :: CssSyntaxKind :: COLON } ; [::] => { $ crate :: CssSyntaxKind :: COLON2 } ; [=] => { $ crate :: CssSyntaxKind :: EQ } ; [!] => { $ crate :: CssSyntaxKind :: BANG } ; [!=] => { $ crate :: CssSyntaxKind :: NEQ } ; [-] => { $ crate :: CssSyntaxKind :: MINUS } ; [<=] => { $ crate :: CssSyntaxKind :: LTEQ } ; [>=] => { $ crate :: CssSyntaxKind :: GTEQ } ; [+=] => { $ crate :: CssSyntaxKind :: PLUSEQ } ; [|=] => { $ crate :: CssSyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: CssSyntaxKind :: AMPEQ } ; [^=] => { $ crate :: CssSyntaxKind :: CARETEQ } ; [/=] => { $ crate :: CssSyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: CssSyntaxKind :: STAREQ } ; [%=] => { $ crate :: CssSyntaxKind :: PERCENTEQ } ; [@] => { $ crate :: CssSyntaxKind :: AT } ; ["$="] => { $ crate :: CssSyntaxKind :: DOLLAR_EQ } ; [~=] => { $ crate :: CssSyntaxKind :: TILDE_EQ } ; [-->] => { $ crate :: CssSyntaxKind :: CDC } ; [<!--] => { $ crate :: CssSyntaxKind :: CDO } ; [media] => { $ crate :: CssSyntaxKind :: MEDIA_KW } ; [keyframes] => { $ crate :: CssSyntaxKind :: KEYFRAMES_KW } ; [not] => { $ crate :: CssSyntaxKind :: NOT_KW } ; [and] => { $ crate :: CssSyntaxKind :: AND_KW } ; [only] => { $ crate :: CssSyntaxKind :: ONLY_KW } ; [or] => { $ crate :: CssSyntaxKind :: OR_KW } ; [i] => { $ crate :: CssSyntaxKind :: I_KW } ; [important] => { $ crate :: CssSyntaxKind :: IMPORTANT_KW } ; [highlight] => { $ crate :: CssSyntaxKind :: HIGHLIGHT_KW } ; [part] => { $ crate :: CssSyntaxKind :: PART_KW } ; [dir] => { $ crate :: CssSyntaxKind :: DIR_KW } ; [local] => { $ crate :: CssSyntaxKind :: LOCAL_KW } ; [global] => { $ crate :: CssSyntaxKind :: GLOBAL_KW } ; [any] => { $ crate :: CssSyntaxKind :: ANY_KW } ; [current] => { $ crate :: CssSyntaxKind :: CURRENT_KW } ; [past] => { $ crate :: CssSyntaxKind :: PAST_KW } ; [future] => { $ crate :: CssSyntaxKind :: FUTURE_KW } ; [host] => { $ crate :: CssSyntaxKind :: HOST_KW } ; [host_context] => { $ crate :: CssSyntaxKind :: HOST_CONTEXT_KW } ; [matches] => { $ crate :: CssSyntaxKind :: MATCHES_KW } ; [is] => { $ crate :: CssSyntaxKind :: IS_KW } ; [where] => { $ crate :: CssSyntaxKind :: WHERE_KW } ; [has] => { $ crate :: CssSyntaxKind :: HAS_KW } ; [lang] => { $ crate :: CssSyntaxKind :: LANG_KW } ; [nth_child] => { $ crate :: CssSyntaxKind :: NTH_CHILD_KW } ; [nth_last_child] => { $ crate :: CssSyntaxKind :: NTH_LAST_CHILD_KW } ; [nth_of_type] => { $ crate :: CssSyntaxKind :: NTH_OF_TYPE_KW } ; [nth_last_of_type] => { $ crate :: CssSyntaxKind :: NTH_LAST_OF_TYPE_KW } ; [nth_col] => { $ crate :: CssSyntaxKind :: NTH_COL_KW } ; [nth_last_col] => { $ crate :: CssSyntaxKind :: NTH_LAST_COL_KW } ; [charset] => { $ crate :: CssSyntaxKind :: CHARSET_KW } ; [color_profile] => { $ crate :: CssSyntaxKind :: COLOR_PROFILE_KW } ; [counter_style] => { $ crate :: CssSyntaxKind :: COUNTER_STYLE_KW } ; [property] => { $ crate :: CssSyntaxKind :: PROPERTY_KW } ; [container] => { $ crate :: CssSyntaxKind :: CONTAINER_KW } ; [style] => { $ crate :: CssSyntaxKind :: STYLE_KW } ; [ltr] => { $ crate :: CssSyntaxKind :: LTR_KW } ; [rtl] => { $ crate :: CssSyntaxKind :: RTL_KW } ; [n] => { $ crate :: CssSyntaxKind :: N_KW } ; [even] => { $ crate :: CssSyntaxKind :: EVEN_KW } ; [odd] => { $ crate :: CssSyntaxKind :: ODD_KW } ; [of] => { $ crate :: CssSyntaxKind :: OF_KW } ; [from] => { $ crate :: CssSyntaxKind :: FROM_KW } ; [to] => { $ crate :: CssSyntaxKind :: TO_KW } ; [var] => { $ crate :: CssSyntaxKind :: VAR_KW } ; [url] => { $ crate :: CssSyntaxKind :: URL_KW } ; [src] => { $ crate :: CssSyntaxKind :: SRC_KW } ; [font_palette_values] => { $ crate :: CssSyntaxKind :: FONT_PALETTE_VALUES_KW } ; [font_feature_values] => { $ crate :: CssSyntaxKind :: FONT_FEATURE_VALUES_KW } ; [stylistic] => { $ crate :: CssSyntaxKind :: STYLISTIC_KW } ; [historical_forms] => { $ crate :: CssSyntaxKind :: HISTORICAL_FORMS_KW } ; [styleset] => { $ crate :: CssSyntaxKind :: STYLESET_KW } ; [character_variant] => { $ crate :: CssSyntaxKind :: CHARACTER_VARIANT_KW } ; [swash] => { $ crate :: CssSyntaxKind :: SWASH_KW } ; [ornaments] => { $ crate :: CssSyntaxKind :: ORNAMENTS_KW } ; [annotation] => { $ crate :: CssSyntaxKind :: ANNOTATION_KW } ; [auto] => { $ crate :: CssSyntaxKind :: AUTO_KW } ; [thin] => { $ crate :: CssSyntaxKind :: THIN_KW } ; [medium] => { $ crate :: CssSyntaxKind :: MEDIUM_KW } ; [thick] => { $ crate :: CssSyntaxKind :: THICK_KW } ; [none] => { $ crate :: CssSyntaxKind :: NONE_KW } ; [hidden] => { $ crate :: CssSyntaxKind :: HIDDEN_KW } ; [dotted] => { $ crate :: CssSyntaxKind :: DOTTED_KW } ; [dashed] => { $ crate :: CssSyntaxKind :: DASHED_KW } ; [solid] => { $ crate :: CssSyntaxKind :: SOLID_KW } ; [double] => { $ crate :: CssSyntaxKind :: DOUBLE_KW } ; [groove] => { $ crate :: CssSyntaxKind :: GROOVE_KW } ; [ridge] => { $ crate :: CssSyntaxKind :: RIDGE_KW } ; [inset] => { $ crate :: CssSyntaxKind :: INSET_KW } ; [outset] => { $ crate :: CssSyntaxKind :: OUTSET_KW } ; [initial] => { $ crate :: CssSyntaxKind :: INITIAL_KW } ; [inherit] => { $ crate :: CssSyntaxKind :: INHERIT_KW } ; [unset] => { $ crate :: CssSyntaxKind :: UNSET_KW } ; [revert] => { $ crate :: CssSyntaxKind :: REVERT_KW } ; [revert_layer] => { $ crate :: CssSyntaxKind :: REVERT_LAYER_KW } ; [default] => { $ crate :: CssSyntaxKind :: DEFAULT_KW } ; [em] => { $ crate :: CssSyntaxKind :: EM_KW } ; [rem] => { $ crate :: CssSyntaxKind :: REM_KW } ; [ex] => { $ crate :: CssSyntaxKind :: EX_KW } ; [rex] => { $ crate :: CssSyntaxKind :: REX_KW } ; [cap] => { $ crate :: CssSyntaxKind :: CAP_KW } ; [rcap] => { $ crate :: CssSyntaxKind :: RCAP_KW } ; [ch] => { $ crate :: CssSyntaxKind :: CH_KW } ; [rch] => { $ crate :: CssSyntaxKind :: RCH_KW } ; [ic] => { $ crate :: CssSyntaxKind :: IC_KW } ; [ric] => { $ crate :: CssSyntaxKind :: RIC_KW } ; [lh] => { $ crate :: CssSyntaxKind :: LH_KW } ; [rlh] => { $ crate :: CssSyntaxKind :: RLH_KW } ; [vw] => { $ crate :: CssSyntaxKind :: VW_KW } ; [svw] => { $ crate :: CssSyntaxKind :: SVW_KW } ; [lvw] => { $ crate :: CssSyntaxKind :: LVW_KW } ; [dvw] => { $ crate :: CssSyntaxKind :: DVW_KW } ; [vh] => { $ crate :: CssSyntaxKind :: VH_KW } ; [svh] => { $ crate :: CssSyntaxKind :: SVH_KW } ; [lvh] => { $ crate :: CssSyntaxKind :: LVH_KW } ; [dvh] => { $ crate :: CssSyntaxKind :: DVH_KW } ; [vi] => { $ crate :: CssSyntaxKind :: VI_KW } ; [svi] => { $ crate :: CssSyntaxKind :: SVI_KW } ; [lvi] => { $ crate :: CssSyntaxKind :: LVI_KW } ; [dvi] => { $ crate :: CssSyntaxKind :: DVI_KW } ; [vb] => { $ crate :: CssSyntaxKind :: VB_KW } ; [svb] => { $ crate :: CssSyntaxKind :: SVB_KW } ; [lvb] => { $ crate :: CssSyntaxKind :: LVB_KW } ; [dvb] => { $ crate :: CssSyntaxKind :: DVB_KW } ; [vmin] => { $ crate :: CssSyntaxKind :: VMIN_KW } ; [svmin] => { $ crate :: CssSyntaxKind :: SVMIN_KW } ; [lvmin] => { $ crate :: CssSyntaxKind :: LVMIN_KW } ; [dvmin] => { $ crate :: CssSyntaxKind :: DVMIN_KW } ; [vmax] => { $ crate :: CssSyntaxKind :: VMAX_KW } ; [svmax] => { $ crate :: CssSyntaxKind :: SVMAX_KW } ; [lvmax] => { $ crate :: CssSyntaxKind :: LVMAX_KW } ; [dvmax] => { $ crate :: CssSyntaxKind :: DVMAX_KW } ; [cm] => { $ crate :: CssSyntaxKind :: CM_KW } ; [mm] => { $ crate :: CssSyntaxKind :: MM_KW } ; [q] => { $ crate :: CssSyntaxKind :: Q_KW } ; [in] => { $ crate :: CssSyntaxKind :: IN_KW } ; [pc] => { $ crate :: CssSyntaxKind :: PC_KW } ; [pt] => { $ crate :: CssSyntaxKind :: PT_KW } ; [px] => { $ crate :: CssSyntaxKind :: PX_KW } ; [mozmm] => { $ crate :: CssSyntaxKind :: MOZMM_KW } ; [rpx] => { $ crate :: CssSyntaxKind :: RPX_KW } ; [cqw] => { $ crate :: CssSyntaxKind :: CQW_KW } ; [cqh] => { $ crate :: CssSyntaxKind :: CQH_KW } ; [cqi] => { $ crate :: CssSyntaxKind :: CQI_KW } ; [cqb] => { $ crate :: CssSyntaxKind :: CQB_KW } ; [cqmin] => { $ crate :: CssSyntaxKind :: CQMIN_KW } ; [cqmax] => { $ crate :: CssSyntaxKind :: CQMAX_KW } ; [deg] => { $ crate :: CssSyntaxKind :: DEG_KW } ; [grad] => { $ crate :: CssSyntaxKind :: GRAD_KW } ; [rad] => { $ crate :: CssSyntaxKind :: RAD_KW } ; [turn] => { $ crate :: CssSyntaxKind :: TURN_KW } ; [s] => { $ crate :: CssSyntaxKind :: S_KW } ; [ms] => { $ crate :: CssSyntaxKind :: MS_KW } ; [hz] => { $ crate :: CssSyntaxKind :: HZ_KW } ; [khz] => { $ crate :: CssSyntaxKind :: KHZ_KW } ; [dpi] => { $ crate :: CssSyntaxKind :: DPI_KW } ; [dpcm] => { $ crate :: CssSyntaxKind :: DPCM_KW } ; [dppx] => { $ crate :: CssSyntaxKind :: DPPX_KW } ; [x] => { $ crate :: CssSyntaxKind :: X_KW } ; [fr] => { $ crate :: CssSyntaxKind :: FR_KW } ; [page] => { $ crate :: CssSyntaxKind :: PAGE_KW } ; [left] => { $ crate :: CssSyntaxKind :: LEFT_KW } ; [right] => { $ crate :: CssSyntaxKind :: RIGHT_KW } ; [first] => { $ crate :: CssSyntaxKind :: FIRST_KW } ; [blank] => { $ crate :: CssSyntaxKind :: BLANK_KW } ; [top_left_corner] => { $ crate :: CssSyntaxKind :: TOP_LEFT_CORNER_KW } ; [top_left] => { $ crate :: CssSyntaxKind :: TOP_LEFT_KW } ; [top_center] => { $ crate :: CssSyntaxKind :: TOP_CENTER_KW } ; [top_right] => { $ crate :: CssSyntaxKind :: TOP_RIGHT_KW } ; [top_right_corner] => { $ crate :: CssSyntaxKind :: TOP_RIGHT_CORNER_KW } ; [bottom_left_corner] => { $ crate :: CssSyntaxKind :: BOTTOM_LEFT_CORNER_KW } ; [bottom_left] => { $ crate :: CssSyntaxKind :: BOTTOM_LEFT_KW } ; [bottom_center] => { $ crate :: CssSyntaxKind :: BOTTOM_CENTER_KW } ; [bottom_right] => { $ crate :: CssSyntaxKind :: BOTTOM_RIGHT_KW } ; [bottom_right_corner] => { $ crate :: CssSyntaxKind :: BOTTOM_RIGHT_CORNER_KW } ; [left_top] => { $ crate :: CssSyntaxKind :: LEFT_TOP_KW } ; [left_middle] => { $ crate :: CssSyntaxKind :: LEFT_MIDDLE_KW } ; [left_bottom] => { $ crate :: CssSyntaxKind :: LEFT_BOTTOM_KW } ; [right_top] => { $ crate :: CssSyntaxKind :: RIGHT_TOP_KW } ; [right_middle] => { $ crate :: CssSyntaxKind :: RIGHT_MIDDLE_KW } ; [right_bottom] => { $ crate :: CssSyntaxKind :: RIGHT_BOTTOM_KW } ; [layer] => { $ crate :: CssSyntaxKind :: LAYER_KW } ; [scope] => { $ crate :: CssSyntaxKind :: SCOPE_KW } ; [supports] => { $ crate :: CssSyntaxKind :: SUPPORTS_KW } ; [selector] => { $ crate :: CssSyntaxKind :: SELECTOR_KW } ; [import] => { $ crate :: CssSyntaxKind :: IMPORT_KW } ; [namespace] => { $ crate :: CssSyntaxKind :: NAMESPACE_KW } ; [starting_style] => { $ crate :: CssSyntaxKind :: STARTING_STYLE_KW } ; [document] => { $ crate :: CssSyntaxKind :: DOCUMENT_KW } ; [url_prefix] => { $ crate :: CssSyntaxKind :: URL_PREFIX_KW } ; [domain] => { $ crate :: CssSyntaxKind :: DOMAIN_KW } ; [media_document] => { $ crate :: CssSyntaxKind :: MEDIA_DOCUMENT_KW } ; [regexp] => { $ crate :: CssSyntaxKind :: REGEXP_KW } ; [font_face] => { $ crate :: CssSyntaxKind :: FONT_FACE_KW } ; [ident] => { $ crate :: CssSyntaxKind :: IDENT } ; [EOF] => { $ crate :: CssSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: CssSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; }
