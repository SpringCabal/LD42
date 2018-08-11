local Lantern = Unit:New {
    canMove				= false,
    maxVelocity         = 0.00001,
    movementClass		= "KBOT_Infantry",
    canGuard            = false,
    canPatrol           = false,
    canRepeat           = false,
    category            = "INFANTRY",

    pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'cylY',
    -- corpse				= "",
    footprintX			= 10,
    footprintZ			= 10,
    mass				= 50,
    maxDamage			= 300, -- default only
    repairable			= false,
    sightDistance		= 0,

    upright				= true,

    name                = "Lantern",
    maxDamage           = 150,
    mass                = 60,
    script             = "lantern.lua",

    buildPic = "",
}

return {
    Lantern = Lantern,
}
