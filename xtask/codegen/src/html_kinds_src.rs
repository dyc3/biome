use crate::kind_src::KindsSrc;

pub const HTML_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("/", "SLASH"),
        ("=", "EQ"),
        ("!", "BANG"),
        ("-", "MINUS"),
        // HTML-ish tokens
        (":", "COLON"),
        (".", "PERIOD"),
        ("{{", "L_DOUBLE_CURLY"),
        ("}}", "R_DOUBLE_CURLY"),
        ("@", "AT"),
        ("v-", "V_DASH"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
    ],
    keywords: &["null", "true", "false", "doctype", "html"],
    literals: &["HTML_STRING_LITERAL", "HTML_LITERAL"],
    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "COMMENT",
        "HTML_IDENT",
    ],
    nodes: &[
        "HTML_ROOT",
        "HTML_DIRECTIVE",
        "HTML_SELF_CLOSING_TAG",
        "HTML_ELEMENT",
        "HTML_OPENING_ELEMENT",
        "HTML_CLOSING_ELEMENT",
        "HTML_SELF_CLOSING_ELEMENT",
        "HTML_ATTRIBUTE",
        "HTML_ATTRIBUTE_INITIALIZER_CLAUSE",
        "HTML_STRING",
        "HTML_NAME",
        "HTML_ELEMENT_LIST",
        "HTML_ATTRIBUTE_LIST",
        "HTML_CONTENT",
        // Bogus nodes
        "HTML_BOGUS",
        "HTML_BOGUS_ELEMENT",
        "HTML_BOGUS_ATTRIBUTE",
        // Vue nodes
        "VUE_DIRECTIVE",
        "VUE_DIRECTIVE_ARGUMENT",
        "VUE_DIRECTIVE_ARGUMENT_DYNAMIC",
        "VUE_DIRECTIVE_ARGUMENT_STATIC",
        "VUE_DIRECTIVE_MODIFIER",
        "VUE_DIRECTIVE_MODIFIER_LIST",
        "VUE_DIRECTIVE_VALUE",
        "VUE_V_BIND_SHORTHAND",
        "VUE_V_ON_SHORTHAND",
        "VUE_TEMPLATE_INTERPOLATION",
    ],
};
