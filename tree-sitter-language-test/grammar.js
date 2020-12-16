const sepBy1 = (sep, rule) => seq(
    rule,
    repeat(seq(sep, rule)),
    optional(sep),
);

const sepBy = (sep, rule) => optional(sepBy1(sep, rule));

const parens = (rule) => seq('(', rule, ')');
const block = (rule) => seq('{', rule, '}');

module.exports = grammar({
    name: 'language_test',

    extras: $ => [
        $.comment,
        /\s/,
    ],

    rules: {
        source_file: $ => repeat($._item),

        _item: $ => choice(
            $.import_item,
            $.export_item,
            $.struct_item,
            $.function_item,
        ),

        import_item: $ => seq(
            'import',
            block(sepBy(',', $.identifier)),
        ),

        export_item: $ => seq(
            'export',
            block(sepBy(',', $.identifier)),
        ),

        struct_item: $ => seq(
            'struct',
            field('name', $.identifier),
            field('parameters', $.struct_parameter_list),
            ';',
        ),

        struct_parameter_list: $ => parens(sepBy(',', $.struct_parameter)),

        struct_parameter: $ => seq(
            field('name', optional($.identifier)),
            ':',
            field('type', $._type),
        ),

        function_item: $ => seq(
            'func',
            field('name', $.identifier),
            field('parameters', $.func_parameter_list),
            '->',
            field('return_type', $._type),
            field('body', $.block)
        ),

        func_parameter_list: $ => parens(sepBy(',', $.func_parameter)),

        func_parameter: $ => seq(
            field('name', optional($.identifier)),
            field('pattern', $._pattern),
            ':',
            field('type', $._type),
        ),

        _type: $ => choice(
            $.identifier,
            // TODO: other kinds of types
        ),

        block: $ => block(seq(
            repeat($._statement),
            field('trailing', optional($._expression)),
        )),

        _statement: $ => choice(
            $.let_statement,
            seq($._expression, ';'),
            // TODO: other kinds of statements
        ),

        let_statement: $ => seq(
            'let',
            field('name', $._pattern),
            field('type', optional(seq(':', $._type))),
            '=',
            field('value', $._expression),
            ';',
        ),

        _expression: $ => choice(
            $.identifier,
            $.number,
            // TODO: other kinds of expressions
        ),

        _pattern: $ => choice(
            $.identifier,
            $.number,
            // TODO: other kinds of patterns
        ),

        identifier: $ => /[a-zA-Z_]\w*/,
        number: $ => /[+-]?\d+/,

        comment: $ => token(choice(
            seq('//', /.*/),
            seq(
              '/*',
              /[^*]*\*+([^/*][^*]*\*+)*/,
              '/',
            ),
        )),
    },
});
