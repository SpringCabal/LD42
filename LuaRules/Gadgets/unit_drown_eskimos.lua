if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
	return {
		name    = "De-eskimoer",
		desc    = "drowns eskimos.",
		author  = "gajop",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1,
		enabled = true,
	}
end

local LOG_SECTION = "eski"
local eskimos = {}
local eskimoDefID = UnitDefNames["eskimo"].id

function UnitIsEskimo(unitID)
    return eskimoDefID == Spring.GetUnitDefID(unitID)
end

function gadget:Initialize()
    for _, unitID in pairs(Spring.GetAllUnits()) do
        self:UnitCreated(unitID)
    end
end

function gadget:UnitCreated(unitID)
    if not UnitIsEskimo(unitID) then
		return
	end
    eskimos[unitID] = {
        unitID = unitID,
    }
end


function gadget:UnitDestroyed(unitID)
    eskimos[unitID] = nil
end

function gadget:GameFrame()
    for unitID, _ in pairs(eskimos) do
        local x,y,z = Spring.GetUnitPosition(unitID)
        local h = Spring.GetGroundHeight(x,z)
        if h <= 0 then -- time to drown
            Spring.DestroyUnit(unitID)
        end
    end
end
