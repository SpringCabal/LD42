local Ship = Unit:New {
	buildPic			= "",

	--pushResistant       = true,
	collisionVolumeScales   = '37 40 37',
	collisionVolumeTest     = 1,
	collisionVolumeType     = 'CylY',
	mass                = 50,
	minCollisionSpeed   = 1,
	movementClass       = "SHIP4",

	-- Sensors
	sightDistance       = 800,


	-- Movement & Placement
	footprintX          = 6,
	footprintZ          = 6,
	upright				= true,
	maxVelocity         = 3,

	acceleration        = 0.5,
	brakeRate           = 0.4,
	--buildCostMetal        = 65, -- used only for power XP calcs
	canMove             = true,
	--     canGuard            = false,
	--     canPatrol           = false,
	--     canRepeat           = false,

	turnRate            = 30 / 0.16,

	maxDamage           = 1600,
	activateWhenBuilt   = true,
	onoffable           = true,
	fireState           = 0,
	moveState           = 0,

	customParams = {
		health = 500,
	}
}

local DrillShip = Ship:New {
	name                = "DrillShip",
	category            = "SHIP",
	script              = "shipscript.lua",
	objectName 			= "drillship.s3o",
}

return {
    drillship    = DrillShip,
}
