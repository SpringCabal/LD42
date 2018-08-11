unitDef = {
	unitname = [[drillship]],
	
	name = [[Icedrillship]],
	description = [[Pirates harvesting ice]],
	acceleration = 0.0498,
	activateWhenBuilt = true,
	brakeRate = 0.0808,
	
	buildCostMetal = 400,
	builder = false,
	
	buildPic = [[]],
	canGuard = true,
	canMove = true,
	canPatrol = true,
	category = [[SHIP]],
	collisionVolumeOffsets = [[0 4 4]],
	collisionVolumeScales = [[32 32 128]],
	collisionVolumeType = [[CylZ]],
	corpse = [[]],
	
	customParams = {
		
	},
	
	explodeAs = [[]],
	floater = true,
	footprintX = 4,
	footprintZ = 4,
	iconType = [[]],
	idleAutoHeal = 5,
	idleTime = 1800,
	losEmitHeight = 40,
	maxDamage = 1900,
	maxVelocity = 2.84,
	minCloakDistance = 75,
	minWaterDepth = 5,
	movementClass = [[SHIP4]],
	moveState = 0,
	noAutoFire = false,
	noChaseCategory = [[]],
	objectName = [[drillship.s3o]],
	radarDistance = 1000,
	script				 = [[shipscript.lua]],
	selfDestructAs = [[]],
	sightDistance = 660,
	sonarDistance = 660,
	turninplace = 0,
	turnRate = 486,
	waterline = 4,
	workerTime = 0,
	
	weapons = {
		
	},
	
	
	
	
}

return lowerkeys({ drillship = unitDef })