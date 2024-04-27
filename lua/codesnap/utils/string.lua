local string_util = {}

function string_util.trim(str)
  return str:gsub("%s+", "")
end

function string_util.escape(str)
  return str:gsub("[%(%)%.%%%+%-%*%?%[%^%$%]]", "%%%1")
end

function string_util.ends_with(str, suffix)
  return str:sub(-#suffix) == suffix
end

function string_util.is_str_empty(target)
  return target == nil or target == ""
end

function string_util.convert_empty_to_nil(target)
  if target == "" then
    return nil
  else
    return target
  end
end

function string_util.split(str, delimiter)
  local result = {}

  for token in string.gmatch(str, "[^" .. delimiter .. "]+") do
    table.insert(result, token)
  end

  return result
end

return string_util
