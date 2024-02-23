local visual_utils = {}

function visual_utils.get_selected_text()
  local start_pos = vim.fn.getpos("v")
  local end_pos = vim.fn.getpos(".")

  -- We switch the start and end positions if the start is after the end line or character
  -- This way we can always select from the top down and from left to right
  if start_pos[2] > end_pos[2] or start_pos[3] > end_pos[3] then
    start_pos, end_pos = end_pos, start_pos
  end

  if start_pos[2] == end_pos[2] then
    return vim.api.nvim_buf_get_lines(0, start_pos[2] - 1, start_pos[2], false)[1]:sub(start_pos[3], end_pos[3] - 1)
  else
    -- 如果选中的是多行文本，则需要分别获取每一行的文本
    local selected_text = {}
    for i = start_pos[2], end_pos[2] do
      -- 使用 vim.api.nvim_buf_get_lines() 函数获取选中的文本
      local line_text = vim.api.nvim_buf_get_lines(0, i - 1, i, false)[1]
      -- 如果是选中的第一行，需要从 mark 'v' 的列开始获取
      if i == start_pos[2] then
        line_text = line_text:sub(start_pos[3])
      end
      -- 如果是选中的最后一行，需要获取到当前光标的列
      if i == end_pos[2] then
        line_text = line_text:sub(1, end_pos[3] - 1)
      end
      table.insert(selected_text, line_text)
    end
    -- 输出当前选中的文本
    return table.concat(selected_text, "\n")
  end
end

return visual_utils
