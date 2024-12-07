local list_utils = require("codesnap.utils.list")
local table_utils = {}

function table_utils.assign(t, props)
  local parsed_t = t or {}

  for k, v in pairs(props) do
    if type(v) == "table" then
      parsed_t[k] = table_utils.assign(parsed_t[k], v)
    else
      parsed_t[k] = v
    end
  end

  return parsed_t
end

function table_utils.merge(t1, t2)
  for k, v in pairs(t2) do
    t1[k] = v
  end

  return t1
end

-- Merge two tables, if the value of the key in t2 is "none", it will be removed from t1
-- which is useful for removing a key from the config, in CodeSnap, set border = None to remove the border
function table_utils.merge_config(t1, t2)
  for k, v in pairs(t1) do
    if type(v) == "table" and type(t2[k]) == "table" then
      t1[k] = table_utils.merge_config(v, t2[k])
    elseif t2[k] == "none" then
      t1[k] = nil
    elseif t2[k] ~= nil then
      t1[k] = t2[k]
    end
  end

  return t1
end

function table_utils.is_array(t)
  return type(t[1]) == "number"
end

function table_utils.typeof(value)
  if type(value) == "table" then
    if table_utils.is_array(value) then
      return "array"
    else
      return "table"
    end
  end

  return type(value)
end

function table_utils.serialize_array(t)
  local result = list_utils.map(t, function(ele)
    table_utils.to_string(ele)
  end)

  return "[" .. result.concat(t, ",") .. "]"
end

function table_utils.serialize_table(t)
  local result = {}

  for key, value in pairs(t) do
    table.insert(result, string.format("%s = %s", key, table_utils.to_string(value)))
  end

  return "{" .. table.concat(result, ",") .. "}"
end

function table_utils.serialize_string(value)
  return "[[" .. value .. "]]"
end

function table_utils.to_string(t)
  local complex_type_parser = {
    array = table_utils.serialize_array,
    table = table_utils.serialize_table,
    string = table_utils.serialize_string,
  }

  local parse = complex_type_parser[table_utils.typeof(t)] or function(v)
    return v
  end

  return parse(t)
end

return table_utils
