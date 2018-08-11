unitDef = {
	unitname = [[canonboat]],
	
	name = [[Icedrillship]],
	description = [[Pirates harvesting ice]],
	acceleration = 0.0768,
	activateWhenBuilt = true,
	brakeRate = 0.042,
	buildCostMetal = 600,
	builder = false,
	buildPic = [[]],
	canGuard = true,
	canMove = true,
	canPatrol = true,
	category = [[SHIP]],
	collisionVolumeOffsets = [[0 6 5]],
	collisionVolumeScales = [[55 55 130]],
	collisionVolumeType = [[cylZ]],
	corpse = [[DEAD]],
	--Core_color.dds Core_other.dds
	customParams = {
		modelradius = [[55]],
		turnatfullspeed = [[1]],
		extradrawrange = 800,
	},
	
	explodeAs = [[]],
	floater = true,
	footprintX = 4,
	footprintZ = 4,
	iconType = [[]],
	idleAutoHeal = 5,
	idleTime = 1800,
	losEmitHeight = 25,
	maxDamage = 3750,
	maxVelocity = 2.0,
	minCloakDistance = 75,
	minWaterDepth = 5,
	movementClass = [[BOAT4]],
	noAutoFire = false,
	noChaseCategory = [[]],
	objectName = [[drillship.s3o]],
	script				 = [[shipscript.lua]],
	selfDestructAs = [[]],
	
	sfxtypes = {
		
		explosiongenerators = {
			[[custom:brawlermuzzle]],
			[[custom:emg_shells_l]],
		},
		
	},
	
	sightDistance = 430,
	sonarDistance = 430,
	turninplace = 0,
	turnRate = 360,
	workerTime = 0,
	
	weapons = {
		
		
		{
			def = [[MISSILE]],
			badTargetCategory = [[SWIM LAND SHIP HOVER]],
			onlyTargetCategory = [[SWIM LAND SINK TURRET FLOAT SHIP HOVER]],
		},
		
	},
	
	weaponDefs = {
		
		
		
		MISSILE = {
			name = [[Destroyer Missiles]],
			areaOfEffect = 48,
			cegTag = [[missiletrailyellow]],
			collideFriendly = false,
			craterBoost = 1,
			craterMult = 2,
			
			damage = {
				default = 400,
				subs = 400,
			},
			
			edgeEffectiveness = 0.5,
			fireStarter = 100,
			fixedLauncher			 = true,	 
			flightTime = 4,
			impulseBoost = 0,
			impulseFactor = 0.4,
			interceptedByShieldType = 2,
			model = [[wep_m_hailstorm.s3o]],
			noSelfDamage = true,
			range = 800,
			reloadtime = 16,
			smokeTrail = true,
			soundHit = [[weapon/missile/missile_fire12]],
			soundStart = [[weapon/missile/missile_fire10]],
			startVelocity			 = 100,
			tolerance = 4000,
			turnrate				 = 30000,
			turret				 = true,	 
			--waterWeapon			 = true,
			weaponAcceleration = 300,
			weaponTimer = 1,
			weaponType = [[StarburstLauncher]],
			weaponVelocity = 1800,
		},
		
	},
	
	
}

return lowerkeys({ canonboat = unitDef })