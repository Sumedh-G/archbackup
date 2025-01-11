-- Catppuccin
local catppuccin_options = {
  compile_path = vim.fn.stdpath "cache" .. "/catpuccin",
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
    variables = { "italic" },
    numbers = {},
    booleans = { "italic" },
    properties = {},
    types = { "italic" },
    operators = {},
  },
  custom_highlights = function(colors)
    return {
      NonText = { fg=colors.surface2 },
      MatchParen = { fg=colors.blue },
      TreesitterContextLineNumber = { fg=colors.sapphire }
    }
  end,
  default_integrations = false,
  integrations = {
    markdown = true,
    render_markdown = true,
    treesitter = true,
    treesitter_context = true,
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
