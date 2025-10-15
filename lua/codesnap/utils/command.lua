local command_util = {}

function command_util.exec_command(command, mode)
	local handle = assert(io.popen(command, mode))
	local origin = assert(handle:read("*a"))

	handle:close()

	return origin
end

return command_util
