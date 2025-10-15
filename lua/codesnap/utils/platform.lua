local platform_utils = {}

local current_os_name = vim.loop.os_uname().sysname

function platform_utils.match_os(matches_table)
	local fn = matches_table[current_os_name]

	if fn == nil then
		error("codesnap.nvim not supported on " .. current_os_name)
	end

	return fn()
end

-- Check if running on Windows
function platform_utils.is_windows()
	return current_os_name == "Windows_NT"
end

-- Check if running on macOS
function platform_utils.is_macos()
	return current_os_name == "Darwin"
end

-- Check if running on Linux
function platform_utils.is_linux()
	return current_os_name == "Linux"
end

return platform_utils
