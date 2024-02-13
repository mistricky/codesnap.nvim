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

return string_util
