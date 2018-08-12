

local Parachut = Unit:New {
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


    name                = "Parachut",
    activateWhenBuilt   = true,
    customParams = {
    },

    idletime					= 120, --in simframes
    idleautoheal 				= 50,
    autoheal 					= 1,

    maxDamage           = 1600,
    maxVelocity         = 10,
    onoffable           = true,
    fireState           = 0,
    moveState           = 0,
    script              = "",
	objectName 				= "parachut.s3o",
}

return {
    parachut    = Parachut,
}

