local visual_utils = {}

-- Get all the lines from "from" to "to" and return them as a single string
-- If "from" and "to" are the same, return the line at "from"
local function get_whole_lines(from, to)
  local lines = {}
  if from == to then
    table.insert(lines, vim.api.nvim_buf_get_lines(0, from - 1, from, false)[1])
  else
    for i = from, to do
      table.insert(lines, vim.api.nvim_buf_get_lines(0, i - 1, i, false)[1])
    end
  end
  return table.concat(lines, "\n")
end

function visual_utils.get_start_line_number()
  return vim.fn.line("'<")
end

function visual_utils.get_end_line_number()
  return vim.fn.line("'>")
end

function visual_utils.get_selected_lines()
  return vim.fn.getline("'<", "'>")
end

function visual_utils.get_selected_text()
  return table.concat(visual_utils.get_selected_lines(), "\n")
end

function visual_utils.get_selected_text_realtime()
  local start_pos = vim.fn.getpos("v")
  local end_pos = vim.fn.getpos(".")

  -- We switch the start and end positions if the start is after the end line or character
  -- This way we can always select from the top down and from left to right
  if start_pos[2] > end_pos[2] or start_pos[3] > end_pos[3] then
    start_pos, end_pos = end_pos, start_pos
  end

  if vim.api.nvim_get_mode().mode == "V" then
    return get_whole_lines(start_pos[2], end_pos[2])
  end

  if start_pos[2] == end_pos[2] then
    return vim.api.nvim_buf_get_lines(0, start_pos[2] - 1, start_pos[2], false)[1]:sub(start_pos[3], end_pos[3] - 1)
  end

  local selected_text = {}
  for i = start_pos[2], end_pos[2] do
    local line_text = vim.api.nvim_buf_get_lines(0, i - 1, i, false)[1]
    if i == start_pos[2] then
      line_text = line_text:sub(start_pos[3])
    end
    -- If select last line, there need to get column of current cursor
    if i == end_pos[2] then
      line_text = line_text:sub(1, end_pos[3] - 1)
    end
    table.insert(selected_text, line_text)
  end

  return table.concat(selected_text, "\n")
end

return visual_utils
