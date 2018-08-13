local Parachut = Unit:New {
	acceleration        = 0.5,
	brakeRate           = 0.4,
	buildPic            = "",
    --buildCostMetal        = 65, -- used only for power XP calcs
    canMove             = true,
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

	category            = "INFANTRY",
	movementClass       = "Eskimo",


    stealth             = true,
    turnRate            = 3000,
    upright             = true,


    name                = "Parachut",
    activateWhenBuilt   = true,
    customParams = { },

    maxDamage           = 1,
    maxVelocity         = 10,
    script              = "",
	objectName 				= "parachut.s3o",
}

return {
    parachut    = Parachut,
}
