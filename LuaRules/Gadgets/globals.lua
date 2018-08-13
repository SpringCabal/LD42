
if not gadgetHandler:IsSyncedCode() then
	return
end
--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
	return {
		name    = "globals",
		desc    = "Global setup",
		author  = "gajop",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1,
		enabled = true,
	}
end

function gadget:Initialize()
    Spring.SetGlobalLos(0, true)
    Spring.SetGlobalLos(1, true)
end

local prevValue
function gadget:GameFrame()
    local newValue = Spring.GetGameRulesParam("sb_gameMode")
    if prevValue ~= newValue then
        prevValue = newValue
    end
    self:Initialize()
end
