if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
	return {
		name    = "Eskimo attributes",
		desc    = "Eskimo attribute mechanics.",
		author  = "gajop",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1,
		enabled = true,
	}
end

local LOG_SECTION = "eskimo-attributes"

local GAME_FRAME_PER_SEC = 33
local MULTI = 1 / GAME_FRAME_PER_SEC

local MAX_HEALTH            = 100
local START_HEALTH          = 50
local HEALTH_REGEN_RATE     = 1.0 * MULTI
local FREEZING_HEALTH_DECAY = 1.0 * MULTI
local STARVING_HEALTH_DECAY = 0.2 * MULTI

local REST_STATES = {
    RESTING = "resting",
    ACTIVE = "active",
}

local MAX_FOOD          = 100
local START_FOOD        = 40
local FOOD_DECAY_RATE   = 1   * MULTI
local FOOD_EATING_RATE  = 10  * MULTI
local FOOD_RES_USAGE    = 1   * MULTI

local EATING_STATES = {
    IDLE = "idle",
    EATING = "eating",
    STARVING = "starving"
}

local MAX_HEAT          = 100
local START_HEAT        = 80
local WARMING_INCREASE  = 4   * MULTI
local COLD_DECREASE     = 1   * MULTI
local HEAT_RES_USAGE    = 1   * MULTI

local WARM_STATES = {
    WARM = "warm",
    COLD = "cold",
    FREEZING = "freezing",
}

local eskimoDefID = UnitDefNames["eskimo"].id

local eskimos = {}

local function UnitIsEskimo(unitID)
    local unitDefID = Spring.GetUnitDefID(unitID)
    return unitDefID == eskimoDefID
end

function gadget:Initialize()
    Spring.SetGameRulesParam("maxHealth", MAX_HEALTH)
    Spring.SetGameRulesParam("maxFood", MAX_FOOD)
    Spring.SetGameRulesParam("maxHeat", MAX_HEAT)

    for _, unitID in pairs(Spring.GetAllUnits()) do
        self:UnitCreated(unitID)
    end
end

local function UpdateAllAttributes(unitID, table)
    for k, v in pairs(table) do
        Spring.SetUnitRulesParam(unitID, k, v)
    end
end

function gadget:UnitCreated(unitID)
    if not UnitIsEskimo(unitID) then
		return
	end

    local function _l(attr)
        return Spring.GetUnitRulesParam(unitID, attr)
    end

    local eskimo = {
        unitID = unitID,
        attrs = {
            health =       _l("health")       or START_HEALTH,
            food =         _l("food")         or START_FOOD,
            heat =         _l("heat")         or START_HEAT,
            rest_state =   _l("rest_state")   or REST_STATES.ACTIVE,
            warm_state =   _l("warm_state")   or WARM_STATES.COLD,
            eating_state = _l("eating_state") or EATING_STATES.IDLE,
			--eating_state = _l("eating_state") or EATING_STATES.EATING,
        }
    }

    eskimos[unitID] = eskimo
    UpdateAllAttributes(unitID, eskimo.attrs)
end

function SetAttribute(unitID, key, value)
    local eskimo = eskimos[unitID]
    eskimo.attrs[key] = value
    Spring.SetUnitRulesParam(unitID, key, value)
end

function gadget:UnitDestroyed(unitID)
    if UnitIsEskimo(unitID) then
        eskimos[unitID] = nil
    end
end

local function DoHeat(eskimo)
    local warm_state = eskimo.attrs.warm_state
    local heat = eskimo.attrs.heat
    if warm_state == WARM_STATES.WARM then
		if not GG.Resources.Consume("food", HEAT_RES_USAGE) then
			SetAttribute(eskimo.unitID, "warm_state", WARM_STATES.COLD)
		else
			heat = heat + WARMING_INCREASE
		end
    elseif warm_state == WARM_STATES.COLD then
        heat = heat - COLD_DECREASE
	elseif not warm_state == WARM_STATES.FREEZING then
		Spring.Log(LOG_SECTION, LOG.ERROR, "Invalid warm_state " ..
			tostring(warm_state) .. " for: " .. tostring(unitID))
    end

    if heat <= 0 then
        SetAttribute(eskimo.unitID, "warm_state", WARM_STATES.FREEZING)
    end
	if heat >= MAX_HEAT then
		SetAttribute(eskimo.unitID, "warm_state", WARM_STATES.COLD)
	end

    heat = math.min(heat, MAX_HEAT)
    heat = math.max(heat, 0)
    SetAttribute(eskimo.unitID, "heat", heat)
end

local function DoFood(eskimo)
    local eating_state = eskimo.attrs.eating_state
    local food = eskimo.attrs.food
    if eating_state == EATING_STATES.IDLE then
        food = food - FOOD_DECAY_RATE
    elseif eating_state == EATING_STATES.EATING then
		if not GG.Resources.Consume("food", FOOD_RES_USAGE) then
			SetAttribute(eskimo.unitID, "eating_state", EATING_STATES.IDLE)
		else
			food = food + FOOD_EATING_RATE
		end
	elseif not eating_state == EATING_STATES.STARVING then
		Spring.Log(LOG_SECTION, LOG.ERROR, "Invalid eating_state " ..
			tostring(eating_state) .. " for: " .. tostring(unitID))
    end

    if food <= 0 then
        SetAttribute(eskimo.unitID, "eating_state", EATING_STATES.STARVING)
    end
	if food >= MAX_FOOD then
		SetAttribute(eskimo.unitID, "eating_state", EATING_STATES.IDLE)
	end

    food = math.min(food, MAX_FOOD)
    food = math.max(food, 0)
    SetAttribute(eskimo.unitID, "food", food)
end

local function DoHealth(eskimo)
    local health = eskimo.attrs.health
    if eskimo.attrs.rest_state == REST_STATES.RESTING then
        health = health + HEALTH_REGEN_RATE
    end
    if eskimo.attrs.warm_state == WARM_STATES.FREEZING then
        health = health - FREEZING_HEALTH_DECAY
    end
    if eskimo.attrs.eating_state == EATING_STATES.STARVING then
        health = health - STARVING_HEALTH_DECAY
    end

    local delayedDestroy = false
    if health <= 0 then
        delayedDestroy = true
    end

    health = math.min(health, MAX_HEALTH)
    health = math.max(health, 0)
    SetAttribute(eskimo.unitID, "health", health)

    if delayedDestroy then
        Spring.DestroyUnit(eskimo.unitID)
    end
end

function gadget:GameFrame()
	if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
        return
    end
    local frame = Spring.GetGameFrame()

    for _, eskimo in pairs(eskimos) do
        DoHeat(eskimo)
        DoFood(eskimo)
        DoHealth(eskimo)
    end
end

GG.Eskimo = {
    SetAttribute = SetAttribute,
}
