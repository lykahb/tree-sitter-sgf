local sgf_repo = "/Users/borys/projects/tree-sitter-sgf"

vim.treesitter.language.add("sgf", {
  path = sgf_repo .. "/build/sgf.so",
})

vim.filetype.add({
  extension = {
    sgf = "sgf",
  },
})

vim.opt.runtimepath:prepend(sgf_repo)

vim.api.nvim_create_autocmd("FileType", {
  pattern = "sgf",
  callback = function()
    vim.treesitter.start()
  end,
})

