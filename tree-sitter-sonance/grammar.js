const tokens = {
    // Regex
    identifier: /[a-zA-Z_]\w*/,
    number: /\d+/,

    // Keywords
    func: 'func',
    import: 'import',
    external: 'external',

    // Groups
    open_paren: '(',
    close_paren: ')',
    open_block: '{',
    close_block: '}',

    // Punctuation
    arrow: '->',
    semi: ';',
    colon: ':',
    at: '@',
};

const
    sepBy = (sep, rule) => optional(sepBy1(sep, rule)),
    sepBy1 = (sep, rule) => seq(
        rule,
        repeat(seq(sep, rule)),
        optional(sep),
    ),

    wrap = (start, end) => (rule = '') => seq(start, rule, end),
    parens = wrap('(', ')'),
    block = wrap('{', '}'),
    angles = wrap('<', '>');

module.exports = grammar({
    name: 'sonance',
    extras: $ => [
        $.comment,
        /\s/,
    ],

    rules: {
        source_file: $ => repeat($._item),

        _item: $ => seq(
            repeat($.attribute),
            choice(
                $.import_item,
                $.func_item,
            ),
            tokens.semi,
        ),

        attribute: $ => seq(
            tokens.at,
            field('name', $.identifier),
            parens(),
        ),

        import_item: $ => seq(
           tokens.import,
           block(repeat($._item)),
        ),

        func_item: $ => seq(
            tokens.func,
            field('name', $.identifier),
            field('parameters', $.parameter_list),
            tokens.arrow,
            field('return_ty', $._ty),
            optional(field('body', $.block)),
        ),

        parameter_list: $ => parens(sepBy(',', $.parameter)),

        parameter: $ => seq(
            field('name', $.identifier),
            tokens.colon,
            field('ty', $._ty),
        ),

        _ty: $ => choice(
            $.identifier,
        ),

        block: $ => block(sepBy(';', $._expression)),

        _expression: $ => choice(
            $.number,
            $.call_expression,
        ),

        call_expression: $ => seq(
            optional(field('leading', $._expression)),
            field('name', $.identifier),
            field('arguments', $.argument_list),
        ),

        argument_list: $ => parens(sepBy(',', $._expression)),

        identifier: $ => tokens.identifier,
        number: $ => tokens.number,

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
