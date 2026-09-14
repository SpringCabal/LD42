if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
	return {
		name    = "Drill",
		desc    = "Drill mechanics.",
		author  = "gajop",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1,
		enabled = true,
	}
end

local LOG_SECTION = "drill"

local GAME_FRAME_PER_SEC = 33

local DRILL_SIZE = 250
local NEARBY_SIZE = 100
local DRILL_AMOUNT = 2.0

local ICE_HEIGHT = -1

Math = {}
function Math.RoundInt(x, step)
    x = math.round(x)
    return x - x % step
end

local drillShips = {}
local drillShipDefID = UnitDefNames["drillship"].id

function UnitIsDrillShip(unitID)
    return drillShipDefID == Spring.GetUnitDefID(unitID)
end

function gadget:Initialize()
    for _, unitID in pairs(Spring.GetAllUnits()) do
        self:UnitCreated(unitID)
    end
end

function gadget:UnitCreated(unitID)
    if not UnitIsDrillShip(unitID) then
		return
	end
    drillShips[unitID] = {
        unitID = unitID,
        hasOrder = false,
		checkedFrame = -1,
    }
end

function gadget:UnitDestroyed(unitID)
    drillShips[unitID] = nil
end

function DecreaseTerrain(x, z, size, amount)
    local startX = Math.RoundInt(x - size / 2, Game.squareSize)
    local startZ = Math.RoundInt(z - size / 2, Game.squareSize)

    size = Math.RoundInt(size, Game.squareSize)
    local sh = size / 2
	local max = sh -- math.sqrt(sh * sh + sh * sh)
    for x = 0, size, Game.squareSize do
        local dx = (x - sh) * (x - sh)
        for z = 0, size, Game.squareSize do
            local dz = (z - sh) * (z - sh)
            local d = max - math.sqrt(dx + dz)
			d = math.max(0, d)

            local gx = x + startX
            local gz = z + startZ
            local gh = Spring.GetGroundHeight(gx, gz)

            gh = gh - amount * d -- / math.log(2 + d) -- * (max / (d + 0.001))

			gh = math.max(gh, -500)
            Spring.SetHeightMap(gx, gz, gh)
        end
    end
end

function DrillUnit(unitID)
    local x, y, z = Spring.GetUnitPosition(unitID)

	local unitDefID = Spring.GetUnitDefID(unitID)
	if unitDefID == drillShipDefID then
		local drill = Spring.GetUnitPieceMap(unitID)["icemine"]
		x, y, z = Spring.GetUnitPiecePosDir(unitID, drill)
	end
    -- Spring.Echo("DRILL", Spring.GetGameFrame(), unitID, x, y, z)

    Spring.SetHeightMapFunc(DecreaseTerrain, x, z, DRILL_SIZE, DRILL_AMOUNT)
end

function NearbyIce(unitID)
    local size = NEARBY_SIZE
    local sx, _, sz = Spring.GetUnitPosition(unitID)
    local startX = Math.RoundInt(sx - size / 2, Game.squareSize)
    local startZ = Math.RoundInt(sz - size / 2, Game.squareSize)

    size = Math.RoundInt(size, Game.squareSize)
    for x = 0, size, Game.squareSize do
        for z = 0, size, Game.squareSize do
            local gh = Spring.GetGroundHeight(x + startX, z + startZ)

            -- Spring.Echo(gh, ICE_HEIGHT)
            if gh > ICE_HEIGHT then
                return true
            end
        end
    end

    return false
end

function FindClosestIce(unitID)
    local sx, _, sz = Spring.GetUnitPosition(unitID)
    sx = Math.RoundInt(sx, Game.squareSize)
    sz = Math.RoundInt(sz, Game.squareSize)

    for size = Game.squareSize, math.max(Game.mapSizeX, Game.mapSizeZ) do
        local x = sx
        local z = sz

        for _, z in pairs({sz - size, sz + size}) do
	        for x = sx - size, sx + size, Game.squareSize do
	            if Spring.GetGroundHeight(x, z) > ICE_HEIGHT then
	                return x, z
	            end
	        end
        end
        for _, x in pairs({sx - size, sx + size}) do
	        for z = sz - size, sz + size, Game.squareSize do
	            if Spring.GetGroundHeight(x, z) > ICE_HEIGHT then
	                return x, z
	            end
	        end
		end
    end

    return nil
end

function OrderMove(unitID, x, z)
    -- Spring.Echo("OrderMove(unitID, x, z)", unitID, x, z)
    Spring.GiveOrderToUnit(unitID, CMD.MOVE, {x, Spring.GetGroundHeight(x, z), z}, {})
end

local function distance(x1, z1, x2, z2)
    local dx = x1 - x2
    local dz = z1 - z2
    return math.sqrt(dx * dx + dz * dz)
end

local DISABLE_GADGET = false
local CHECK_FRAME_INTERVAL = 30
function gadget:GameFrame()
    if DISABLE_GADGET then
        return
    end
	if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
        return
    end
	local frame = Spring.GetGameFrame()
    for unitID, drillShip in pairs(drillShips) do
        -- Spring.Echo("NoNearbyIce(unitID)", NearbyIce(unitID))
        if frame - drillShip.checkedFrame > CHECK_FRAME_INTERVAL and not NearbyIce(unitID) then
			drillShip.checkedFrame = frame
            local moveClose = false
            if drillShip.hasOrder then
                local x, _, z = Spring.GetUnitPosition(unitID)
                if distance(drillShip.orderX, drillShip.orderZ, x, z) > NEARBY_SIZE then
                    OrderMove(unitID, drillShip.orderX, drillShip.orderZ)
                    moveClose = true
                end
            end
            if not moveClose then
                local x, z = FindClosestIce(unitID)
                if not x then
                    drillShip.hasOrder = false
                    Spring.Echo("no more ice")
                    DISABLE_GADGET = true
                    return
                else
                    OrderMove(unitID, x, z)
                    drillShip.hasOrder = true
                    drillShip.orderX = x
                    drillShip.orderZ = z
                end
            end
        end
    end
end

GG.Drill = {
    DrillUnit = DrillUnit,
	DecreaseTerrain = DecreaseTerrain
}
