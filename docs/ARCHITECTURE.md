# Project structure

- corpus/ contains sgf files. Some are from the private collections. Do not commit this folder. Use it for local testing and as source of patterns for the test suite.
- grammar.js contains the Tree-sitter syntax grammar. It models the official SGF collection/game-tree/node/property/value structure and keeps property-specific validation out of the parser.
- test/corpus/ contains small public fixtures. Prefer synthetic examples derived from the SGF specification over copied private games.

# Syntax and semantics

The parser accepts SGF syntax. It does not enforce semantic constraints from the specification, including property types, legal point ranges, move/setup conflicts, game-info inheritance rules, or game-specific move notation.

This split keeps all official SGF games parseable through one syntax tree. Semantic checks can be added later as a separate linter or library over the parsed tree.

# Backwards compatibility with older SGF versions

See the 
https://www.red-bean.com/sgf/converting.html
and the source code for the converter https://www.red-bean.com/sgf/sgfc/index.html.
