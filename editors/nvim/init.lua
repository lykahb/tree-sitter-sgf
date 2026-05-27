local sgf_repo = vim.env.TREE_SITTER_SGF_REPO
  or vim.fn.fnamemodify(debug.getinfo(1, "S").source:sub(2), ":p:h:h:h")

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
