const
    sepBy = (sep, rule) => optional(sepBy1(sep, rule)),
    sepBy1 = (sep, rule) => seq(
        rule,
        repeat(seq(sep, rule)),
        optional(sep),
    ),

    wrap = (start, end) => (rule) => seq(start, rule, end),
    parens = wrap('(', ')'),
    block = wrap('{', '}'),
    angles = wrap('<', '>');

module.exports = grammar({
    name: 'language_test',

    extras: $ => [
        $.comment,
        /\s/,
    ],

    rules: {
        source_file: $ => repeat($._item),

        _item: $ => seq(
            choice(
                $.module_item,
                $.import_item,
                $.export_item,
                $.struct_item,
                $.union_item,
                $.function_item,
            ),
            ';',
        ),

        module_item: $ => seq(
            'module',
            field('name', $.identifier),
            optional(field('generics', $.generic_parameter_list)),
            optional(seq(
                '->',
                field('return_type', $._type),
            )),
            block(repeat($._item)),
        ),

        simple_path: $ => seq(
            $.identifier,
            optional(field('args', $.generic_argument_list)),
            optional(seq('::', $.simple_path)),
        ),

        compound_path: $ => seq(
            $.identifier,
            optional(field('args', $.generic_argument_list)),
            optional(choice(
                seq('::', choice(
                    $.compound_path,
                    block(sepBy(',', $.compound_path)),
                )),
                seq('as', $.identifier),
            )),
        ),

        import_item: $ => seq(
            'import',
            block(sepBy(',', $.compound_path)),
        ),

        export_item: $ => seq(
            'export',
            block(sepBy(',', $.simple_path)),
        ),

        struct_item: $ => seq(
            'struct',
            field('name', $.identifier),
            optional(field('generics', $.generic_parameter_list)),
            field('parameters', $.struct_parameter_list),
        ),

        struct_parameter_list: $ => parens(sepBy(',', $.struct_parameter)),

        struct_parameter: $ => seq(
            optional(seq(
                field('name', $.identifier),
                ':',
            )),
            field('type', $._type),
        ),

        generic_argument_list: $ => angles(sepBy(',', $._type)),

        generic_parameter_list: $ => angles(sepBy(',', $.generic_parameter)),

        generic_parameter: $ => seq(
            field('name', $.identifier),
            optional(seq(
                ':',
                field('bounds', $.simple_path),
            )),
        ),

        union_item: $ => seq(
            'union',
            field('name', $.identifier),
            optional(field('generics', $.generic_parameter_list)),
            field('variants', $.union_variant_list),
        ),

        union_variant_list: $ => block(sepBy(',', $.union_variant)),

        union_variant: $ => seq(
            field('name', $.identifier),
            optional(field('parameters', $.struct_parameter_list)),
        ),

        function_item: $ => seq(
            'func',
            field('name', $.identifier),
            optional(field('generics', $.generic_parameter_list)),
            field('parameters', $.func_parameter_list),
            optional(seq(
                '->',
                field('return_type', $._type),
            )),
            field('body', $.block),
        ),

        func_parameter_list: $ => parens(sepBy(',', $.func_parameter)),

        func_parameter: $ => seq(
            field('name', optional($.identifier)),
            field('pattern', $._pattern),
            ':',
            field('type', $._type),
        ),

        _type: $ => choice(
            $.simple_path,
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
            $.simple_path,
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
