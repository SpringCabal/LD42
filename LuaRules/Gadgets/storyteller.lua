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

local story

local s11n


local lastStepFrame = -math.huge
local waitFrames = 0
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
end

local ourTeam = 0
local enemyTeam = 1
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
            units = {"iglu", "iglu", "iglu", "iglu", "iglu",
					 "iglu", "iglu", "iglu", "iglu", "iglu",
					 -- "iglu", "iglu", "iglu", "iglu", "iglu",
		 		     -- "iglu", "iglu", "iglu", "iglu", "iglu",
					 -- "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",
					 -- "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",
					 "eskimo", "eskimo", "eskimo", "eskimo", "eskimo",
					 "eskimo", "eskimo", "eskimo", "eskimo", "eskimo"},
			team = ourTeam,
        },
        {
            name = "spawn",
            units = {"pirate", "pirate", "pirate"},
			team = enemyTeam,
        },
        {
            name = "spawn",
            units = {"drillship"},
			team = enemyTeam,
        },
        {
            name = "intro",
            about = "food_healing",
        },
        {
            name = "spawn",
            units = {"pirate","drillship","pirate"},
			team = enemyTeam,
        },
        {
            name = "intro",
            about = "selling"
        },
        {
            name = "spawn",
            units = {"pirate","drillship","pirate","pirate","drillship"},
			team = enemyTeam,
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

function SpawnUnits(units, team)
	for _, defName in pairs(units) do
		local x, y, z
		if defName ~= "drillship" then
			x = Game.mapSizeX / 2 + math.random(-Game.mapSizeX / 8, Game.mapSizeX / 8)
			z = Game.mapSizeZ / 2 + math.random(-Game.mapSizeZ / 8, Game.mapSizeZ / 8)
		else
			x = Game.mapSizeX / 2 + math.random(Game.mapSizeX / 4, Game.mapSizeX / 2)
			z = Game.mapSizeZ / 2 + math.random(Game.mapSizeZ / 4, Game.mapSizeZ / 2)
		end
		if defName ~= "pirate" then
			y = Spring.GetGroundHeight(x, z)
		else
			y = Spring.GetGroundHeight(x, z) + 5000
		end
		s11n:Add({
			defName = defName,
			pos = {
				x = x,
				y = y,
				z = z,
			},
			team = team,
		})
	end
end

function DoIntro(about)
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
	if step.name == "spawn" then
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
