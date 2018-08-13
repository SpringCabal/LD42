if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
	return {
		name    = "Parachute remover",
		desc    = "removes parachutes.",
		author  = "Anarchid",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1,
		enabled = true,
	}
end

local LOG_SECTION = "para"
local parachutes = {}
local paraDefID = UnitDefNames["parachut"].id

function UnitIsPara(unitID)
    return paraDefID == Spring.GetUnitDefID(unitID)
end

function gadget:Initialize()
    for _, unitID in pairs(Spring.GetAllUnits()) do
        self:UnitCreated(unitID)
    end
end

function gadget:UnitCreated(unitID)
    if not UnitIsPara(unitID) then
		return
	end
    parachutes[unitID] = {
        unitID = unitID,
        hasOrder = false,
		checkedFrame = -1,
    }
end


function gadget:UnitDestroyed(unitID)
    parachutes[unitID] = nil
end

function gadget:GameFrame()
    for unitID, parachute in pairs(parachutes) do
        local x,y,z = Spring.GetUnitPosition(unitID);
        local h = Spring.GetGroundHeight(x,z)
        if ( h >= y) then 
            Spring.DestroyUnit(unitID, false, true)
        end
    end
end