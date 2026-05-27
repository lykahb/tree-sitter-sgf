local repo = vim.fn.fnamemodify(debug.getinfo(1, "S").source:sub(2), ":p:h:h:h")

vim.treesitter.language.add("sgf", {
  path = repo .. "/build/sgf.so",
})

vim.filetype.add({
  extension = {
    sgf = "sgf",
  },
})

vim.opt.runtimepath:prepend(repo)

local parser = vim.treesitter.get_string_parser("(;FF[4]GM[1];B[pd];W[dd])", "sgf")
local tree = parser:parse()[1]
local root = tree:root()

assert(root:type() == "collection", "expected SGF root node to be collection")
assert(vim.treesitter.query.get("sgf", "highlights"), "expected SGF highlight query")

print("nvim sgf smoke ok")
