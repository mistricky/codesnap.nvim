local string_utils = require("codesnap.utils.string")
local table_utils = require("codesnap.utils.table")
local highlight_module = {}

function highlight_module.call_cb_with_parsed_config(cb_name, highlight_start_line_number, highlight_end_line_number)
  vim.api.nvim_buf_delete(0, {})
  vim.schedule(function()
    local main = require("codesnap")
    local config = table_utils.merge(main.highlight_mode_config, {
      highlight_start_line_number = highlight_start_line_number,
      highlight_end_line_number = highlight_end_line_number,
    })

    main[cb_name](config)
  end)
end

function highlight_module.create_highlight_selector_window(cb_name, code)
  local width = 100
  local height = #code + 2
  local row = vim.fn.winheight(0) / 2 - height / 2
  local col = vim.fn.winwidth(0) / 2 - width / 2
  local bufnr = vim.api.nvim_create_buf(false, true)

  vim.api.nvim_buf_set_lines(bufnr, 0, -1, false, code)

  local window_id = vim.api.nvim_open_win(bufnr, false, {
    relative = "editor",
    width = width,
    height = height,
    col = col,
    row = row,
    style = "minimal",
    border = "rounded",
    title = "Select highlight lines",
    title_pos = "center",
  })

  vim.api.nvim_buf_set_option(bufnr, "modifiable", false)
  vim.api.nvim_buf_set_option(bufnr, "filetype", vim.bo.filetype)
  vim.api.nvim_buf_set_keymap(bufnr, "n", "q", ":q<CR>", {})
  vim.api.nvim_buf_set_keymap(bufnr, "", "<ESC>", ":q<CR>", {})
  vim.api.nvim_buf_set_keymap(
    bufnr,
    "v",
    "<CR>",
    ":lua require('codesnap.highlight').call_cb_with_parsed_config('"
      .. cb_name
      .. "', require('codesnap.utils.visual').get_start_line_number(), require('codesnap.utils.visual').get_end_line_number())<CR>",
    { silent = true }
  )
  vim.api.nvim_set_current_win(window_id)
end

return highlight_module
