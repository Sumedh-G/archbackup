
local maps = {
  {
    "<leader>ff", "<cmd>Telescope find_files<cr>",
    { "n" }, desc = "Find Files",
  },
  {
    "<leader>fz", "<cmd>Telescope live_grep<cr>",
    { "n" }, desc = "Live Grep",
  },
  {
    "<leader>fb", "<cmd>Telescope buffers<cr>",
    { "n" }, desc = "Find Buffer",
  },
  {
    "<leader>fg", "<cmd>Telescope git_files<cr>",
    { "n" }, desc = "Find Files",
  },
  {
    "<leader>fs", "<cmd>Telescope grep_string<cr>",
    { "n" }, desc = "Find String",
  },
  {
    "<leader>fh", "<cmd>Telescope search_history<cr>",
    { "n" }, desc = "Search history",
  },
  {
    "<leader>fc", "<cmd>Telescope command_history<cr>",
    { "n", "c" }, desc = "Command history",
  },
  {
    "<leader>fm", "<cmd>Telescope man_pages<cr>",
    { "n" }, desc = "Find ManPage"
  },
}

return maps
