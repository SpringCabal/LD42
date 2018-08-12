

local Coalburner = Unit:New {

    --buildCostMetal        = 65, -- used only for power XP calcs
    canMove             = false,

    category            = "BUILDING",

    --pushResistant       = true,
    collisionVolumeScales   = '37 40 37',
    collisionVolumeTest     = 1,
    collisionVolumeType     = 'CylY',
    footprintX          = 4,
    footprintZ          = 4,
    mass                = 50,
    minCollisionSpeed   = 1,

    repairable          = false,
    sightDistance       = 800,


    stealth             = true,
    turnRate            = 3000,
    upright             = true,


    name                = "Coalburner",
    activateWhenBuilt   = true,
    customParams = {
    },

    idletime					= 120, --in simframes
    idleautoheal 				= 50,
    autoheal 					= 1,

    maxDamage           = 1600,
    onoffable           = true,
  
    script              = "coalburnerscript.lua",
	objectName 				= "coalburner.s3o",
}

return {
    coalburner    = Coalburner,
}

