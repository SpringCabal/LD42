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
local START_HEAT    = 80

local MAX_MONEY     = 100
local START_MONEY	= 0

function SetResource(name, value)
    Spring.SetGameRulesParam(name, value)
end

function gadget:Initialize()
	SetResource("maxFood", MAX_FOOD)
	SetResource("maxHeat", MAX_HEAT)
	SetResource("maxMoney", MAX_MONEY)

    SetResource("food", START_FOOD)
    SetResource("heat", START_HEAT)
	SetResource("money", START_MONEY)
end


GG.Resources = {
    SetResource = SetResource,
    Consume = function(name, amount)
		local current = Spring.GetGameRulesParam(name)
		if current - amount < 0 then
			return false
		end
		local newAmount = current - amount
		newAmount = math.max(newAmount, 0)
		SetResource(name, newAmount)
		return true
    end,
	Add = function(name, amount)
		local newAmount = Spring.GetGameRulesParam(name) + amount
		newAmount = math.min(newAmount, 100) -- HARDCODED
		SetResource(name, newAmount)
	end
}
