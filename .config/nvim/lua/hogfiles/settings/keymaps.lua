local map = vim.keymap.set

-- Ease of life
map("i", "jk", "<ESC>", { desc = "Normal Mode" })

-- Moving between windows
map("n", "<C-h>", "<C-w>h", { desc = "move left" })
map("n", "<C-l>", "<C-w>l", { desc = "move right" })
map("n", "<C-j>", "<C-w>j", { desc = "move down" })
map("n", "<C-k>", "<C-w>k", { desc = "move up" })
