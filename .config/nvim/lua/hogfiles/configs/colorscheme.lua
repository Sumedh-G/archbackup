-- Catppuccin
local catppuccin_options = {
  flavour = "mocha", -- latte, frappe, macchiato, mocha
  transparent_background = true,
  show_end_of_buffer = true,
  dim_inactive = {
    enabled = false,
    shade = "dark",
    percentage = 0.15,
  },
  styles = {
    comments = { "italic", "bold" },
    conditionals = { "italic" },
    loops = {},
    functions = { "bold" },
    keywords = {},
    strings = {},
    variables = {},
    numbers = {},
    booleans = {},
    properties = {},
    types = {},
    operators = {},
  },
  default_integrations = false,
  integrations = {
    markdown = true,
    treesitter = true,
    semantic_tokens = true,
    telescope = {
      enabled = true,
    },
    rainbow_delimiters = true,
    beacon = true,
    indent_blankline = {
      enabled = true,
      colored_indent_levels = true,
      scope_color = "lavender",
    },
    nvim_surround = true,
    gitsigns = true,
    overseer = true,
    fidget = true,
  },
}

return catppuccin_options
