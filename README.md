# tree-sitter-sgf

This grammar implements the [Smart Game Format](https://en.wikipedia.org/wiki/Smart_Game_Format), SGF. It targets `FF[4]` standard, and has limited support for the older versions. It supports Go and other games.

# Development

Generate the parser and run the public corpus tests with:

```sh
npm install
npm run generate
npm test
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
source = { path = "/Users/borys/projects/tree-sitter-sgf" }
```

Then link the Helix query set into Helix's user runtime:

```sh
mkdir -p ~/.config/helix/runtime/queries
ln -sfn /Users/borys/projects/tree-sitter-sgf/queries/helix ~/.config/helix/runtime/queries/sgf
hx -g build
hx --health sgf
```

# Neovim

Neovim uses `queries/sgf/highlights.scm`. The Helix indent and textobject
queries are kept under `queries/helix/` and are not required for Neovim.

Build a shared parser for local testing:

```sh
npm run generate
npx tree-sitter build -o build/sgf.so
```

Register the local parser, filetype, and runtime path with the example config
in `editors/nvim/init.lua`.

Smoke-test the local Neovim setup with:

```sh
nvim --headless --clean -u NONE -S editors/nvim/smoke.lua +qa
```

# References

The [SGF official specification](https://www.red-bean.com/sgf/index.html).
