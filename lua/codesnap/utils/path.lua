local string_utils = require("codesnap.utils.string")
local platform_utils = require("codesnap.utils.platform")
local path_utils = {}

-- Get the appropriate path separator for the current OS
function path_utils.get_separator()
	return package.config:sub(1, 1) -- Returns '\' on Windows, '/' on Unix-like systems
end

function path_utils.join(separator, ...)
	local args = { ... }

	return table.concat(args, separator)
end

function path_utils.dir_name()
	local sep = path_utils.get_separator()
	local pattern = "@?(.*" .. vim.pesc(sep) .. ")"
	return debug.getinfo(1).source:match(pattern)
end

function path_utils.with_dir_name(path)
	return path_utils.dir_name() .. path
end

function path_utils.get_escaped_cwd()
	local cwd = vim.fn.getcwd()

	return string_utils.escape(cwd)
end

function path_utils.back(path)
	local sep = path_utils.get_separator()
	local pattern = vim.pesc(sep) .. "[^" .. vim.pesc(sep) .. "]+" .. vim.pesc(sep) .. "?$"
	local parsed_path, _ = path:gsub(pattern, "")

	return parsed_path
end

function path_utils.get_workspace()
	local cwd = vim.fn.getcwd()
	local sep = path_utils.get_separator()
	local pattern = vim.pesc(sep) .. "([^" .. vim.pesc(sep) .. "]+)$"
	local _, _, workspace = string.find(cwd, pattern)

	return workspace == nil and "" or workspace
end

function path_utils.get_relative_path()
	local full_file_path = vim.fn.expand("%:p")
	local cwd = path_utils.get_escaped_cwd()
	local sep = path_utils.get_separator()

	-- Remove the CWD from the full path
	local relative = full_file_path:gsub("^" .. cwd, "")

	-- Remove leading separator if present
	if relative:sub(1, 1) == sep then
		relative = relative:sub(2)
	end

	return relative
end

-- Get default save path by OS
-- If Linux, use XDG_PICTURE_DIR
-- if mac use ~/Pictures
-- if windows use FOLDERID_Pictures
function path_utils.get_default_save_path()
	local sep = path_utils.get_separator()
	local home = os.getenv("HOME") or os.getenv("USERPROFILE") -- USERPROFILE for Windows
	local home_picture_folder = home .. sep .. "Pictures"

	return platform_utils.match_os({
		Darwin = function()
			return home_picture_folder
		end,
		Linux = function()
			return os.getenv("XDG_PICTURES_DIR") or home_picture_folder
		end,
		Windows_NT = function()
			-- On Windows, use %USERPROFILE%\Pictures
			return home_picture_folder
		end,
	})
end

return path_utils
