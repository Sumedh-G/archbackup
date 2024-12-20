-- Options for lazy.nvim

return {
  defaults = {
    lazy = true,
  },
  spec = {
    -- Catppuccin Theme
    { 
      "catppuccin/nvim", 
      name = "catppuccin",
      lazy = false,
      priority = 1000,
      config = function()
        require("catppuccin").setup(
          require("hogfiles.configs.colorscheme")
        )
        vim.cmd([[colorscheme catppuccin]])
      end,
    },
    -- Syntax highlighting
    {
      "nvim-treesitter/nvim-treesitter",
      build = ":TSUpdate",
      lazy = false,
      config = function()
        local configs = require("nvim-treesitter.configs")
        configs.setup(require("hogfiles.configs.treesitter"))
      end,
    },
    {
      "folke/twilight.nvim",
      event = "VeryLazy",
      opts = require("hogfiles.configs.twilight"),
    },
    -- Auto-pairing
    {
      "utilyre/sentiment.nvim",
      version = "*",
      event = "VeryLazy", -- keep for lazy loading
      opts = require("hogfiles.configs.sentiment"),
      init = function()
        vim.g.loaded_matchparen = 1
      end,
    },
    {
      "kylechui/nvim-surround",
      version = "*",
      event = "VeryLazy",
      opts = require("hogfiles.configs.surround"),
    },
    {
      "windwp/nvim-autopairs",
      event = "InsertEnter",
      opts = require("hogfiles.configs.autopairs"),
    },
    {
      "HiPhish/rainbow-delimiters.nvim",
      event = "VeryLazy",
    },
    {
      "lukas-reineke/indent-blankline.nvim",
      main = "ibl",
      ---@module "ibl"
      ---@type ibl.config
      opts = require("hogfiles.configs.indent_blankline"),
      event = "VeryLazy"
    },
    -- Movement
    {
      "DanilaMihailov/beacon.nvim",
      event = "VeryLazy",
      opts = { speed = 1 },
    },
    {
      "nvim-telescope/telescope.nvim",
      tag = '0.1.8',
      dependencies = {
        'nvim-lua/plenary.nvim',
        {
          "nvim-telescope/telescope-fzf-native.nvim",
          build = 'cmake -S. -Bbuild -DCMAKE_BUILD_TYPE=Release && cmake --build build --config Release',
        },
      },
      opts = require("hogfiles.configs.telescope"),
      lazy = false,
    },
  },
  install = { colorscheme = { "catppuccin" } },
  checker = { enabled = true },
  lockfile = vim.fn.stdpath("config") .. "/lua/hogfiles/configs/.lazy-lock.json",
  ui = {
    border = "rounded",
    backdrop = 40,
  },
}
