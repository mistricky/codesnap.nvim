local codesnap = require("codesnap")
local snap_client_cwd = codesnap.cwd .. "/snap-client"
local snap_server_cwd = codesnap.cwd .. "/snap-server"

-- Build preview client
os.execute(
  "cd "
    .. snap_client_cwd
    .. " && "
    .. "npm i "
    .. " && "
    .. "npm run build"
    .. " && "
    .. "mv ./build "
    .. snap_server_cwd
    .. "/public"
)

-- Build server
os.execute("cd " .. snap_server_cwd .. " && " .. "cargo build --release")
