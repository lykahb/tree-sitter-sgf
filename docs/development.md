# Development

## Project structure

- corpus/ contains optional local SGF files. Do not commit this folder. Use it for local testing and as source of patterns for the public test suite.
- grammar.js contains the Tree-sitter syntax grammar. It models the official SGF collection/game-tree/node/property/value structure and keeps property-specific validation out of the parser.
- src/ contains generated Tree-sitter parser artifacts. Commit these files so downstream users can build the grammar without regenerating it.
- test/corpus/ contains small public fixtures. Prefer synthetic examples derived from the SGF specification over copied external games.
- queries/sgf contains shared editor queries. queries/helix contains Helix-specific queries and duplicates highlights to avoid self-inheritance when linked into Helix as runtime/queries/sgf.

## Syntax and semantics

The parser accepts SGF syntax. It does not enforce semantic constraints from the specification, including property types, legal point ranges, move/setup conflicts, game-info inheritance rules, or game-specific move notation.

This split keeps all official SGF games parseable through one syntax tree. Semantic checks can be added later as a separate linter or library over the parsed tree.

## Local corpus checks

The local corpus is ignored by git. Use `npm run parse:corpus` to parse every
local `corpus/**/*.sgf` file when the corpus is present. Public regression tests
should stay in `test/corpus/` and use synthetic fixtures derived from the SGF
specification.

## Backwards compatibility with older SGF versions

See the 
https://www.red-bean.com/sgf/converting.html
and the source code for the converter https://www.red-bean.com/sgf/sgfc/index.html.
