local list_utils = {}

function list_utils.find(list, predicate)
  for _, value in ipairs(list) do
    if predicate(value) then
      return value
    end
  end

  return nil
end

function list_utils.some(list, predicate)
  return list_utils.find(list, predicate) ~= nil
end

function list_utils.includes(list, value)
  return list_utils.find(list, function(item)
    return item == value
  end) ~= nil
end

function list_utils.map(list, fn)
  local result = {}

  for i, value in ipairs(list) do
    table.insert(result, fn(value, i))
  end

  return result
end

return list_utils
