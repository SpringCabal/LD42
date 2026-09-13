if not gadgetHandler:IsSyncedCode() then
	return
end

function gadget:GetInfo()
	return {
		name    = "Health",
		desc    = "Health mechanics.",
		author  = "gajop",
		date    = "LD42",
		license = "GNU GPL, v2 or later",
		layer   = -1,
		enabled = true,
	}
end

local function UpdateAllAttributes(unitID, table)
    for k, v in pairs(table) do
        Spring.SetUnitRulesParam(unitID, k, v)
    end
end

function gadget:UnitCreated(unitID, unitDefID)
    local unitDef = UnitDefs[unitDefID]
    local health = tonumber(unitDef.customParams.health) or 100000
    Spring.SetUnitRulesParam(unitID, "health", health, {public=true})
end

function gadget:UnitPreDamaged(unitID, unitDefID, unitTeam, damage, paralyzer, weaponDefID, projectileID, attackerID, attackerDefID, attackerTeam)
    local hp = Spring.GetUnitRulesParam(unitID, "health")
    if not hp or not attackerDefID then
        return 0
    end
	local damage = tonumber(UnitDefs[attackerDefID].customParams.damage) or 0
    hp = hp - damage
    -- local damage = WeaponDefs[weaponDefID].customParams.damage
    Spring.SetUnitRulesParam(unitID, "health", hp, {public=true})
    if hp < 0 then
		return 10000000000000000 -- die without recursion
        -- Spring.DestroyUnit(unitID)
    end
    return 0
end
