-- Syntax highlighting

local treesitter_options = {
  ensure_installed = {
    "c", 
    "lua", 
    "vim", 
    "vimdoc", 
    "query", 
    "elixir", 
    "heex", 
    "javascript", 
    "html", 
    "markdown",
    "markdown_inline",
    "bash",
    "latex",
  },
  highlight = {
    enable = true,
    additional_vim_regex_highlighting = false,
  },
  incremental_selection = {
    enable = true,
    keymaps = {
      init_selection = "gnn",
      node_incremental = "grn",
      node_decremental = "grm",
      scope_incremental = "grc",
    },
  },
  indent = {
    enable = true,
  },
}

return treesitter_options
