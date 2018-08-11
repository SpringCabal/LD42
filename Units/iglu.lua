unitDef = {
  unitname                      = [[iglu]],
  name                          = [[Iglu]],
  description                   = [[Citizen Building]],
  acceleration                  = 0,
  activateWhenBuilt             = true,
  brakeRate                     = 0,
  buildCostMetal                = 70,
  builder                       = false,
  buildingGroundDecalDecaySpeed = 30,
  buildingGroundDecalSizeX      = 6,
  buildingGroundDecalSizeY      = 6,
  buildingGroundDecalType       = [[]],
  buildPic                      = [[]],
  category                      = [[BUILDING]],



  damageModifier                = 0.25,
  energyMake                    = 2,
  explodeAs                     = [[SMALL_BUILDINGEX]],
  footprintX                    = 5,
  footprintZ                    = 5,
  iconType                      = [[]],
  idleAutoHeal                  = 5,
  idleTime                      = 1800,
  maxDamage                     = 500,
  maxSlope                      = 18,
  maxVelocity                   = 0,
  maxWaterDepth                 = 0,
  minCloakDistance              = 150,
  noAutoFire                    = false,
  objectName                    = [[iglu.s3o]],
  onoffable                     = true,
  script                        = [[igluscript.lua]],
  selfDestructAs                = [[SMALL_BUILDINGEX]],
  sightDistance                 = 273,
  turnRate                      = 0,
  useBuildingGroundDecal        = true,
  workerTime                    = 0,
  yardMap                       = [[ooooooooooooooooooooooooo]],

  

}

return lowerkeys({ iglu = unitDef })