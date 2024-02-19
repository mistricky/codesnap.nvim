local logger = {}

function logger.log(level, message)
  vim.api.nvim_notify("[" .. level .. "] CodeSnap: " .. tostring(vim.inspect(message)), vim.log.levels[level], {})
end

function logger.error(message)
  logger.log("ERROR", message)
end

return logger
