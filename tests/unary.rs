#![cfg(test)]

#[macro_use]
mod macros;

test!(unary_pos_unquoted_ident, "a {\n  color: +foo;\n}\n");
test!(
    unary_pos_whitespace,
    "a {\n  color: +     foo;\n}\n",
    "a {\n  color: +foo;\n}\n"
);
test!(unary_pos_dblquoted_ident, "a {\n  color: +\"foo\";\n}\n");
test!(
    unary_pos_sglquoted_ident,
    "a {\n  color: +'foo';\n}\n",
    "a {\n  color: +\"foo\";\n}\n"
);
test!(unary_pos_color, "a {\n  color: +\"foo\";\n}\n");
test!(
    unary_pos_number,
    "a {\n  color: +1px;\n}\n",
    "a {\n  color: 1px;\n}\n"
);
test!(
    unary_pos_in_list,
    "a {\n  color: bar,+ \"bar\" - foo;\n}\n",
    "a {\n  color: bar, +\"bar\"-foo;\n}\n"
);
test!(unary_neg_unquoted_ident, "a {\n  color: -foo;\n}\n");
test!(unary_neg_dblquoted_ident, "a {\n  color: -\"foo\";\n}\n");
test!(
    unary_neg_sglquoted_ident,
    "a {\n  color: -'foo';\n}\n",
    "a {\n  color: -\"foo\";\n}\n"
);
test!(unary_neg_color, "a {\n  color: -\"foo\";\n}\n");
test!(unary_neg_number, "a {\n  color: -1px;\n}\n");
test!(
    unary_neg_whitespace,
    "a {\n  color: - 1px;\n}\n",
    "a {\n  color: -1px;\n}\n"
);
test!(
    unary_neg_number_type,
    "a {\n  color: type-of(- 1px);\n}\n",
    "a {\n  color: number;\n}\n"
);