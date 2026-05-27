# tree-sitter-sgf

This grammar implements the [Smart Game Format](https://en.wikipedia.org/wiki/Smart_Game_Format), SGF. It targets `FF[4]` standard, and has limited support for the older versions. It supports Go and other games.

# Development

Install dependencies, generate the parser, and run the public corpus tests with:

```sh
npm install
npm run generate
npm run check
```

# Installation

Install from npm:

```sh
npm install tree-sitter-sgf
```

```js
const Parser = require("tree-sitter");
const SGF = require("tree-sitter-sgf");

const parser = new Parser();
parser.setLanguage(SGF);
const tree = parser.parse("(;GM[1]FF[4]SZ[19];B[pd];W[dd])");
```

Install from crates.io:

```sh
cargo add tree-sitter-sgf
```

```rust
let mut parser = tree_sitter::Parser::new();
let language = tree_sitter_sgf::LANGUAGE;
parser.set_language(&language.into())?;
let tree = parser.parse("(;GM[1]FF[4])", None).unwrap();
```

Install from PyPI:

```sh
pip install tree-sitter-sgf
```

```python
import tree_sitter_sgf
from tree_sitter import Language, Parser

language = Language(tree_sitter_sgf.language())
parser = Parser(language)
tree = parser.parse(b"(;GM[1]FF[4])")
```

The grammar follows the SGF FF[4] syntax definition: a source file is a
collection of one or more game trees; game trees contain sequences, nodes,
properties, and bracketed property values. It intentionally does not validate
SGF semantics such as property placement, point coordinates, move legality,
duplicate properties, or game-specific value formats.

# Helix

For local Helix testing, point Helix at this repository in
`~/.config/helix/languages.toml`:

```toml
[[language]]
name = "sgf"
scope = "source.sgf"
auto-format = false
injection-regex = "sgf"
file-types = ["sgf"]

[[grammar]]
name = "sgf"
source = { path = "/absolute/path/to/tree-sitter-sgf" }
```

Then link the Helix query set into Helix's user runtime:

```sh
REPO="$(pwd)"
mkdir -p ~/.config/helix/runtime/queries
ln -sfn "$REPO/queries/helix" ~/.config/helix/runtime/queries/sgf
hx -g build
hx --health sgf
```

`queries/helix/highlights.scm` intentionally duplicates the shared highlight
query instead of using `; inherits: sgf`. When the Helix runtime path is a
symlink named `sgf`, inheriting from `sgf` would inherit from itself.

# Neovim

Neovim uses `queries/sgf/highlights.scm`. The Helix indent and textobject
queries are kept under `queries/helix/` and are not required for Neovim.

Build a shared parser for local testing:

```sh
npm run generate
npm run build:parser
```

Register the local parser, filetype, and runtime path with the example config
in `editors/nvim/init.lua`. When copying that file outside this repository, set
`TREE_SITTER_SGF_REPO` to the repository path or replace the `sgf_repo` value.

Smoke-test the local Neovim setup with:

```sh
npm run smoke:nvim
```

If you have a local untracked SGF corpus under `corpus/`, parse it with:

```sh
npm run parse:corpus
```

# References

The [SGF official specification](https://www.red-bean.com/sgf/index.html).
