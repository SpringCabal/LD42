--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:GetInfo()
  return {
    name      = "Analytics API",
    desc      = "Optional analytics integration",
    author    = "SpringCabal",
    license   = "GPLv2",
    hidden    = true,
    enabled   = false,
    api       = true,
  }
end

-- ANALYTICS_DIRNAME = "libs/analytics/luaui/widgets/analytics/"
-- VFS.Include(ANALYTICS_DIRNAME .. "init.lua", nil, VFS.DEF_MODE)
