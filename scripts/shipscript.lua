include "lib_UnitScript.lua"
include "lib_Animation.lua"

myDefID= Spring.GetUnitDefID(unitID)

bIsDrillShip =  false
bIsTransport =  false
bIsCannonBoat = false

if  UnitDefNames["drillship"] then bIsDrillShip = UnitDefNames["drillship"].id == myDefID end
if UnitDefNames["transportship"] then bIsTransport = UnitDefNames["transportship"].id == myDefID end
if UnitDefNames["canonboat"] then bIsCannonBoat = UnitDefNames["canonboat"].id == myDefID end

-- functions intended for external calls by gadgets/widgets

function setIceDrillLoad(totalIceLoad)

	totalIcecubes, IcecubesSoFar= #TableOfPieceGroups["icecube"], 0
	process(TableOfPieceGroups["icecube"],
	function (id)
		IcecubesSoFar = IcecubesSoFar + 1
		if id and (IcecubesSoFar/totalIcecubes < totalIceLoad) then
			Show(id)
			turnPieceRandDir(id, math.random(0.5,5))
			spinRand(id,-3, 3, 2)
		end
	end
	)

end

function startIcedrill(speed, angle)
	speed = speed or 0
	angle = angle or 0
	process(TableOfPieceGroups["icedrill"],
	function (id)
		Turn(id,x_axis, math.rad(0),speed)
		Turn(id,y_axis, math.rad(0),speed)
		Turn(id,z_axis, math.rad(0),speed)
	end
	)
	Turn(TableOfPieceGroups["icedrill"][6],x_axis,math.rad(angle),speed)
	Spin(TableOfPieceGroups["icedrill"][7],z_axis, math.rad(-142),speed)
	bIceMiningActive = true

end

function sfxIceMining()
icemine= piece"icemine"
	while true do

	Sleep(500)
		if bIceMiningActive== true and math.random(1,50) > 25 then
			Explode(icemine, SFX.SHATTER+ SFX.FALL +SFX.NO_HEATCLOUD)
		end
	end

end

function stopIcedrill(speed)
	speed = speed or 0

	Turn(TableOfPieceGroups["icedrill"][2],x_axis,math.rad(70),speed)
	Turn(TableOfPieceGroups["icedrill"][4],x_axis,math.rad(-160),speed)
	Turn(TableOfPieceGroups["icedrill"][6],x_axis,math.rad(-80),speed)
	StopSpin(TableOfPieceGroups["icedrill"][7],z_axis,speed)
	bIceMiningActive = false
	--TODO Retract Table
end


-- UnitBuilding Functions
function randomShowDecoration()
	process(TableOfPieceGroups["Deco"],
	function(id)
		if math.random(0,1) == 1 then Show(id) end
	end)
end


showFunctions = {}

if UnitDefNames["drillship"] then
	showFunctions[UnitDefNames["drillship"].id] = function ()
		showT(TableOfPieceGroups["Boatbody"])
		Show(TableOfPieceGroups["Turret"][1])
		showT(TableOfPieceGroups["icedrill"])
		setIceDrillLoad(0)
		stopIcedrill(0)
		randomShowDecoration()
		bIceMiningActive=false
		StartThread(sfxIceMining)



	end
end

if UnitDefNames["transportship"] then
showFunctions[UnitDefNames["transportship"].id] = function ()
		showT(TableOfPieceGroups["Boatbody"])
		randomShowDecoration()
	end
end

if UnitDefNames["canonboat"] then
showFunctions[UnitDefNames["canonboat"].id] = function ()
		showT(TableOfPieceGroups["Boatbody"])
		Show(TableOfPieceGroups["Turret"][2])
		randomShowDecoration()
	end
end

TableOfPieceGroups = getPieceTableByNameGroups(false, true)

function buildUnitByType()
	hideAll(unitID)
	if showFunctions[myDefID] then
		showFunctions[myDefID]()
	end
	StartThread(startIcedrill)


end


function script.HitByWeapon(x, z, weaponDefID, damage)
end


function script.Create()
	resetAll(unitID)
	buildUnitByType()
end

function script.Killed(recentDamage, _)

	return 1
end


--- -aimining & fire weapon
function script.AimFromWeapon1()
	return TableOfPieceGroups["Turret"][2]
end



function script.QueryWeapon1()
	return TableOfPieceGroups["Turret"][2]
end

function script.AimWeapon1(Heading, pitch)
	--aiming animation: instantly turn the gun towards the enemy
	WTurn(TableOfPieceGroups["Turret"][2],y_axis,-Heading, math.pi)
	return bIsCannonBoat
end


function script.FireWeapon1()

	return true
end



function script.StartMoving()
end

function script.StopMoving()
end

function script.Activate()
	startIcedrill(1.0,math.random(10,30))
	return 1
end

function script.Deactivate()
	stopIcedrill(1.0)
	return 0
end
