local M = {}

local win -- Store the floating window handle globally
local buf -- Store the buffer handle globally

M.open_floating_window = function()
    -- Create a buffer
    buf = vim.api.nvim_create_buf(false, true)

    -- Set window size based on the editor's dimensions
    local width = math.floor(vim.o.columns * 0.6)
    local height = math.floor(vim.o.lines * 0.4)
    local row = math.floor((vim.o.lines - height) / 2)
    local col = math.floor((vim.o.columns - width) / 2)

    -- Define window options
    local opts = {
        style = "minimal",  -- Minimal window (no line numbers, etc.)
        relative = "editor", -- Position relative to the entire editor
        width = width,
        height = height,
        row = row,
        col = col,
        border = "rounded",  -- Rounded border around the window
    }

    -- Open the window with the buffer and options, store window handle
    win = vim.api.nvim_open_win(buf, true, opts)

    -- Set some content in the floating window
    vim.api.nvim_buf_set_lines(buf, 0, -1, false, { "Hello, Neovim Floating Window!" })

end 
M.close_floating_window = function()
    if win and vim.api.nvim_win_is_valid(win) then
        vim.api.nvim_win_close(win, true)
        win = nil -- Clear the reference
    end
    if buf and vim.api.nvim_buf_is_valid(buf) then
        vim.api.nvim_buf_delete(buf, { force = true })
        buf = nil
    end
end

return M

