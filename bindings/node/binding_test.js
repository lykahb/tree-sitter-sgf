const assert = require("node:assert/strict");
const test = require("node:test");
const Parser = require("tree-sitter");
const SGF = require(".");

test("can load and parse SGF", () => {
  const parser = new Parser();
  parser.setLanguage(SGF);

  const tree = parser.parse("(;GM[1]FF[4]SZ[19];B[pd];W[dd])");
  assert.equal(tree.rootNode.hasError, false);
});
