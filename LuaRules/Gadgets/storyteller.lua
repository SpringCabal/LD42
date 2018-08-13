if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
   return {
      name      = "storyteller",
      desc      = "",
      author    = "gajop",
      date      = "LD42",
      license   = "GPL-v2",
      layer     = 0,
      enabled   = true
   }
end

local GAME_FRAME_PER_SEC = 33
local ourTeam = 0
local enemyTeam = 1
local gaiaTeam = Spring.GetGaiaTeamID()
local IGLU_MIN_DISTANCE = 400

local story
local s11n
local lastStepFrame
local waitFrames

function gadget:Initialize()
	lastStepFrame = -math.huge
	waitFrames = 0
    story = GetStory()
	s11n = GG.s11n:GetUnitBridge()
	for _, unitID in pairs(Spring.GetAllUnits()) do
		Spring.DestroyUnit(unitID, false, true)
	end
	if Spring.GetGameRulesParam("sb_gameMode") == nil then
        Spring.RevertHeightMap(0, 0, Game.mapSizeX, Game.mapSizeZ, 1)
    end

	Spring.SetGameRulesParam("introEvent", "")
end

function GetStory()
    -- Spawn a few pirates
    -- Spawn a drilling boat
    -- Introduce food and healing
    -- Spawn pirates and a drilling boat
    -- Introduce selling
    -- Spawn stuff
    -- Introduce heat
    -- Spawn stuff
    -- Introduce global warming ("It's getting warmer")
    -- Spawn stuff
    return {
		{
            name = "spawn",
            units = {"coalburner", "iglu", "iglu", "iglu", "iglu", "iglu",
					 -- "iglu", "iglu", "iglu", "iglu", "iglu",
					 -- "iglu", "iglu", "iglu", "iglu", "iglu",
		 		     -- "iglu", "iglu", "iglu", "iglu", "iglu",
					 -- "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",
					 -- "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",
					 "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",
					 "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",},
			team = ourTeam,
			time = 1,
			-- time = 5,
        },
		{
            name = "intro",
			about = "The last remaining good people, bundled on a block of ice.",
			time = 7,
        },
		{
            name = "intro",
			about = "Survive.",
			time = 5,
        },
        {
            name = "spawn",
            units = {"pirate", "pirate", "pirate"},
			team = enemyTeam,
			time = 30,
        },
		{
			name = "intro",
			-- about = "food_healing",
			about = "Your people are cold. Send them to the stove to warm up.",
		},
		{
			name = "intro",
			-- about = "food_healing",
			about = "They come for Ice.",
			time = 1,
		},
        {
            name = "spawn",
            units = {"drillship"},
			team = enemyTeam,
			time = 50,
        },
		{
            name = "intro",
            about = "You are running out of coal. \nBuy more. (Press 2)",
			time = 30,
        },
		{
            name = "intro",
            about = "They are relentless.",
			time = 3,
        },
        {
            name = "spawn",
            units = {"pirate","drillship","pirate"},
			team = enemyTeam,
        },
		{
			name = "intro",
			-- about = "food_healing",
			about = "Sell the Ice to feed and heat your people.\n\n(Press 1 and Left Mouse Button)",
		},
		{
            name = "intro",
            about = "The ice...",
			time = 3,
        },
        {
            name = "spawn",
            units = {"pirate","drillship","pirate","pirate","drillship"},
			team = enemyTeam,
        },
		{
			name = "intro",
			-- about = "food_healing",
			about = "Your people are hungry. Buy food. (Press 3)",
		},
        {
            name = "spawn",
            units = {"pirate","drillship","pirate","pirate", "pirate"},
			team = enemyTeam,
        },
        {
            name = "outro", -- keep spawning units and global warming
        },
    }
end

local function findPosition(defName, team)
	local x, y, z
	if defName == "drillship" then
		x = Game.mapSizeX / 2 + math.random(Game.mapSizeX / 4, Game.mapSizeX / 2) * math.sgn(math.random() - 0.5)
		z = Game.mapSizeZ / 2 + math.random(Game.mapSizeZ / 4, Game.mapSizeZ / 2) * math.sgn(math.random() - 0.5)
	else
		x = Game.mapSizeX / 2 + math.random(-Game.mapSizeX / 8, Game.mapSizeX / 8)
		z = Game.mapSizeZ / 2 + math.random(-Game.mapSizeZ / 8, Game.mapSizeZ / 8)
	end
	if defName == "pirate" then
		y = Spring.GetGroundHeight(x, z) + 5000
	else
		y = Spring.GetGroundHeight(x, z)
	end
	return x, y, z
end

local igluDefID = UnitDefNames["iglu"].id
local coalBurnerDefID = UnitDefNames["coalburner"].id
function SpawnUnits(units, team)
	for _, defName in pairs(units) do
		local x, y, z
		if defName == "iglu" or defName == "coalburner" then
			local found
			repeat
				x, y, z = findPosition(defName, team)
				found = false
				for _, unitID in pairs(Spring.GetUnitsInCylinder(x, z, IGLU_MIN_DISTANCE)) do
					local defID = Spring.GetUnitDefID(unitID)
					if igluDefID == defID or defID == coalBurnerDefID then
						found = true
					end
				end
			until not found
		else
			x, y, z = findPosition(defName, team)
		end
		local obj = {
			defName = defName,
			pos = {
				x = x,
				y = y,
				z = z,
			},
			team = team,
		}
		if defName == "iglu" or defName == "coalburner" then
			obj.team = gaiaTeam
			obj.neutral = true
			obj.blocking = {
	        	blockEnemyPushing = true,
	        	blockHeightChanges = false,
	        	crushable = false,
	        	isBlocking = true,
	        	isProjectileCollidable = false,
	        	isRaySegmentCollidable = false,
	        	isSolidObjectCollidable = true,
	        }
		end
		s11n:Add(obj)
		if defName == "iglu" then
			-- add an extra lantern nearby
			obj.defName = "lantern"
			obj.pos.x = obj.pos.x + math.random(130, 150) * math.sgn(math.random() - 0.5)
			obj.pos.z = obj.pos.z + math.random(130, 150) * math.sgn(math.random() - 0.5)
			obj.pos.y = Spring.GetGroundHeight(obj.pos.x, obj.pos.z)
			s11n:Add(obj)
		end
		if defName == "pirate" then
			-- spawn a parachute on them nearby
			obj.defName = "parachut"
			obj.pos.y = obj.pos.y + 100
			s11n:Add(obj)
		end
	end
end

function DoIntro(about)
	Spring.SetGameRulesParam("introEvent", about)
end

function DoOutro()
end

function DoStep(step)
    if step.name == "spawn" then
        SpawnUnits(step.units, step.team)
    elseif step.name == "intro" then
        DoIntro(step.about)
    elseif step.name == "outro" then
        DoOutro()
    end
end

function NextStep()
    local step = story[1]
    DoStep(step)
    table.remove(story, 1)
	if step.time then
		waitFrames = step.time * GAME_FRAME_PER_SEC
	elseif step.name == "spawn" then
		waitFrames = 30 * GAME_FRAME_PER_SEC
	else
		waitFrames = 10 * GAME_FRAME_PER_SEC
	end
end

local function IsNextStepTime()
	if #story == 0 then
		return false
	end
	local now = Spring.GetGameFrame()
	if not (now - lastStepFrame >= waitFrames) then
		return false
	end

	lastStepFrame = now
	return true
end

local lastSBMode = nil
function gadget:GameFrame()
	local currentSBMode = Spring.GetGameRulesParam("sb_gameMode")
	if lastSBMode ~= currentSBMode then
		lastSBMode = currentSBMode

		if currentSBMode ~= "dev" then
			self:Initialize()
		end
		return
	end
	if currentSBMode == "dev" then
		return
	end

    local frame = Spring.GetGameFrame()

    if IsNextStepTime() then
        NextStep()
    end
end
