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

# References

The [SGF official specification](https://www.red-bean.com/sgf/index.html).
