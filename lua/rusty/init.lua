-- local function open_floating()
--     -- create the buffer for the window
--     local buff = vim.api.nvim_create_buf(false,true)
--     --find the size of the screen 
--     local width = vim.api.nvim_get_option("columns")
--     local height = vim.api.nvim_get_option("lines")
--     --define the window size
--     local window_width = math.ceil(width*0.4)
--     local window_height = math.ceil(height*0.4)
--     --calculate the window position on the screen
--     local row = math.ceil((height / window_height) / 2) -1
--     local columns = math.ceil((width / window_width) / 2)
--
--     --Opening the window now
--
--     local window = vim.api.nvim_open_win(buf, true, {
--     relative = "editor",
--     width = window_width,
--     height = window_height,
--     row = row,
--     col = col,
--     style = "minimal",
--     border = "rounded",
--     })
--
--     --set some text in the window for now
--     vim.api.nvim_buf_set_lines(buf, 0, -1, false, {"Hi"})
--
--     vim.api.nvim_set_keymap("n", "<Leader>q", "<cmd>bd!<CR>", {noremap = true, silent = true})
-- end
--
-- open_floating()
--

local M = {}

M.open_floating_window = function()
    -- Create a new scratch buffer
    local buf = vim.api.nvim_create_buf(false, true)

    -- Get the editor's dimensions
    local width = vim.api.nvim_get_option("columns")
    local height = vim.api.nvim_get_option("lines")

    -- Define window dimensions
    local win_width = math.ceil(width * 0.4)
    local win_height = math.ceil(height * 0.3)

    -- Calculate the window position (centered)
    local row = math.ceil((height - win_height) / 2) - 1
    local col = math.ceil((width - win_width) / 2)

    -- Open the floating window
    local win = vim.api.nvim_open_win(buf, true, {
        relative = "editor",
        width = win_width,
        height = win_height,
        row = row,
        col = col,
        style = "minimal",
        border = "rounded",
    })

    -- Set some example text
    vim.api.nvim_buf_set_lines(buf, 0, -1, false, {
        "Welcome to the Floating Window!",
        "Press 'q' to close.",
    })

    -- Set keybinding to close the floating window
    vim.api.nvim_buf_set_keymap(buf, "n", "q", "<cmd>bd!<CR>", { noremap = true, silent = true })
end

-- Define a command to open the floating window
vim.api.nvim_create_user_command("OpenFloatingWindow", M.open_floating_window, {})

return M

