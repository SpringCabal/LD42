	--Wiki: http://springrts.com/wiki/Modrules.lua

local modRules = {
	movement = {
		allowPushingEnemyUnits    = true,
		allowCrushingAlliedUnits  = false,
		allowUnitCollisionDamage  = false,
		allowUnitCollisionOverlap = false,
		allowGroundUnitGravity    = true,
		allowDirectionalPathing   = true,
	},
	system = {
		pathFinderSystem = 0, -- legacy
	},
}

return modRules
