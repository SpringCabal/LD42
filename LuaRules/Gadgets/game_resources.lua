if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
	return {
		name    = "Game resources",
		desc    = "Game resource mechanics",
		author  = "gajop",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1000,
		enabled = true,
	}
end

local MAX_FOOD      = 100
local START_FOOD    = 80

local MAX_HEAT      = 100
local START_HEAT    = 20


local resources = {}

function SetResource(name, value)
    resources[name] = value
    Spring.SetGameRulesParam(name, value)
end

function gadget:Initialize()
	SetResource("max_food", MAX_FOOD)
	SetResource("max_heat", MAX_HEAT)

    SetResource("food", START_FOOD)
    SetResource("heat", START_HEAT)
end


GG.Resources = {
    SetResource = SetResource,
    Consume = function(name, amount)
		local newAmount = resources[name] - amount
		newAmount = math.max(newAmount, 0)
		SetResource(name, newAmount)
		return newAmount > 0
    end
}
