--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:GetInfo()
	return {
		name      = "Lantern Lights",
		desc      = "Shiny",
		author    = "gajop",
		date      = "LD42",
		license   = "GPL V2",
		layer     = 0,
		enabled   = true
	}
end

local lanternDefID = UnitDefNames["lantern"].id
local function GetLights(beamLights, beamLightCount, pointLights, pointLightCount)

	-- GIVE UP. TODO: Try to fix it.. someday
	if true then
		return beamLights, beamLightCount, pointLights, pointLightCount
	end

    for _, unitID in pairs(Spring.GetAllUnits()) do
        local unitDefID = Spring.GetUnitDefID(unitID)

		if unitDefID == lanternDefID then
	        local x, y, z = Spring.GetUnitPosition(unitID)
			pointLightCount = pointLightCount + 1
	        pointLights[pointLightCount] = {
	            px = x,
	            py = y + 100,
	            pz = z,
	            param = {
	                radius = 50,
	                r = 1,
	                g = 0,
	                b = 0,
	            },
	            colMult = 1,
	        }
		end
    end

	return beamLights, beamLightCount, pointLights, pointLightCount
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:Initialize()
	if WG.DeferredLighting_RegisterFunction then
		WG.DeferredLighting_RegisterFunction(GetLights)
	end
end
