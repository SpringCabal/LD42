unitDef = {
	unitname = [[transportship]],
	
	name = [[Troop carrier]],
	description = [[Pirates invading troops ]],
  acceleration        = 0.102,
  activateWhenBuilt   = true,
  brakeRate           = 0.115,
  buildCostMetal      = 220,
  builder             = false,
  buildPic            = [[]],
  canMove             = true,
  cantBeTransported   = true,
  category            = [[SHIP UNARMED]],
  collisionVolumeOffsets = [[0 0 -3]],
  collisionVolumeScales  = [[35 20 55]],
  collisionVolumeType    = [[ellipsoid]],  
  corpse              = [[]],

  customParams        = {
	turnatfullspeed = [[1]],
    modelradius    = [[15]],
  },

  explodeAs           = [[]],
  floater             = true,
  footprintX          = 4,
  footprintZ          = 4,
  holdSteady          = true,
  iconType            = [[]],
  idleAutoHeal        = 5,
  idleTime            = 1800,
  isFirePlatform      = true,
  maxDamage           = 1200,
  maxVelocity         = 3.3,
  minCloakDistance    = 75,
  movementClass       = [[SHIP4]],
  noChaseCategory     = [[]],
  objectName          = [[drillship.s3o]],
  releaseHeld         = true,
  script              = [[shipscript.lua]],
  selfDestructAs      = [[]],
  sightDistance       = 325,
  sonarDistance       = 325,
  transportCapacity   = 1,
  transportSize       = 3,
  turnRate            = 590,

  

}

return lowerkeys({ transportship = unitDef })