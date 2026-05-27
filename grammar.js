/**
 * @file Tree-sitter grammar for Smart Game Format (SGF) FF[4].
 * @author Borys
 * @license MIT
 */

module.exports = grammar({
  name: 'sgf',

  extras: $ => [
    /[\s\uFEFF\u2060\u200B]/,
  ],

  rules: {
    collection: $ => repeat1($.game_tree),

    game_tree: $ => seq(
      '(',
      $.sequence,
      repeat($.game_tree),
      ')',
    ),

    sequence: $ => repeat1($.node),

    node: $ => seq(
      ';',
      repeat($.property),
    ),

    property: $ => seq(
      field('name', $.property_identifier),
      repeat1(field('value', $.property_value)),
    ),

    // FF[4] standard properties are uppercase ASCII. Lowercase is accepted
    // here so older and non-standard real-world files remain structurally parseable.
    property_identifier: $ => /[A-Za-z]+/,

    property_value: $ => seq(
      '[',
      optional($.value_content),
      ']',
    ),

    value_content: $ => token.immediate(repeat1(choice(
      /[^\\\]]+/,
      /\\[\s\S]/,
    ))),
  },
});
