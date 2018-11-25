

local Iglu = Unit:New {
	buildPic            = "",

	acceleration        = 0.5,
	brakeRate           = 0.4,
    --buildCostMetal        = 65, -- used only for power XP calcs
    canMove             = false,
--     canGuard            = false,
--     canPatrol           = false,
--     canRepeat           = false,

    --pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'CylY',
    footprintX          = 6,
    footprintZ          = 6,
    mass                = 50,
    minCollisionSpeed   = 1,

    repairable          = false,
    sightDistance       = 800,


    stealth             = true,
    turnRate            = 3000,
    upright             = true,


    name                = "Iglu",
    activateWhenBuilt   = true,
    customParams = {
    },

    idletime					= 120, --in simframes
    idleautoheal 				= 50,
    autoheal 					= 1,

    maxDamage           = 1600,
    maxVelocity         = 10,
    onoffable           = true,
    script              = "igluscript.lua",
	objectName 				= "iglu.s3o",
}

local Iglu2 = Unit:New {
	buildPic            = "",

	acceleration        = 0.5,
	brakeRate           = 0.4,
    --buildCostMetal        = 65, -- used only for power XP calcs
    canMove             = false,
--     canGuard            = false,
--     canPatrol           = false,
--     canRepeat           = false,

    --pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'CylY',
    footprintX          = 6,
    footprintZ          = 6,
    mass                = 50,
    minCollisionSpeed   = 1,

    repairable          = false,
    sightDistance       = 800,


    stealth             = true,
    turnRate            = 3000,
    upright             = true,


    name                = "Iglu2",
    activateWhenBuilt   = true,
    customParams = {
    },

    idletime					= 120, --in simframes
    idleautoheal 				= 50,
    autoheal 					= 1,

    maxDamage           = 1600,
    maxVelocity         = 10,
    onoffable           = true,
    script              = "igluscript2.lua",
	objectName 				= "iglu2.dae",
}

return {
    iglu = Iglu,
    iglu2 = Iglu2
}
