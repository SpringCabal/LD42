--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function gadget:GetInfo()
	return {
		name      = "Game End",
		desc      = "Handles team/allyteam deaths and declares gameover",
		author    = "Andrea Piras",
		date      = "June, 2013",
		license   = "GNU GPL, v2 or later",
		layer     = 0,
		enabled   = true  --  loaded by default?
	}
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

--SYNCED
if (not gadgetHandler:IsSyncedCode()) then
   return false
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local initializeFrame

function gadget:Initialize()
	Spring.SetGameRulesParam("gameEnd", "")
end

local eskimoDefID = UnitDefNames["eskimo"].id
function EndCheck()
	local isLoss = true
	for _, unitID in ipairs(Spring.GetAllUnits()) do
		if eskimoDefID == Spring.GetUnitDefID(unitID) then
			isLoss = false
		end
	end
	if isLoss then
		return "loss"
	end

	-- There is no victory, only suffering
end

function gadget:GameFrame(frame)
	if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
		return
	end
	if not initializeFrame then
		initializeFrame = Spring.GetGameFrame()
		Spring.SetGameRulesParam("gameEnd", "")
		Spring.SetGameRulesParam("startTime", initializeFrame)
	end
	if frame > initializeFrame + 5 then
		local endCheck = EndCheck()
		if endCheck == "victory" then
			Spring.SetGameRulesParam("gameEnd", "victory")
		elseif endCheck == "loss" then
			Spring.SetGameRulesParam("gameEnd", "loss")
		end
	end
end
