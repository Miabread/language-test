module.exports = grammar({
    name: 'sonance',
    extras: $ => [
        $.comment,
        /\s/,
    ],

    rules: {
        source_file: $ => seq('func', $.identifier, '(', ')', '->', 'U8', '{', $.number, '}', ';'),

        identifier: $ => /[a-zA-Z_]\w*/,
        number: $ => /\d+/,

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
