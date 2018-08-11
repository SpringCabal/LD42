include "lib_UnitScript.lua"
include "lib_Animation.lua"

myDefID= Spring.GetUnitDefID(unitID)
bIsDrillShip = [UnitDefNames["drillship"].id] ==myDefID
bIsTransport = UnitDefNames["transportship"].id == myDefID
bIsCannonBoat = UnitDefNames["canonboat"].id == myDefID

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

function startIcedrill(speed)
	process(TableOfPieceGroups["icedrill"],
	function (id)
		Turn(id,x_axis, math.rad(0),speed)
		Turn(id,y_axis, math.rad(0),speed)
		Turn(id,z_axis, math.rad(0),speed)
	end
	)
	Spin(TableOfPieceGroups["icedrill"],z_axis, math.rad(42),speed)	
	
	
end

function stopIcedrill(speed)
	Turn(TableOfPieceGroups["icedrill"][1],x_axis,math.rad(75),speed)
	Turn(TableOfPieceGroups["icedrill"][3],x_axis,math.rad(-150),speed)
	Turn(TableOfPieceGroups["icedrill"][6],x_axis,math.rad(-100),speed)
	StopSpin(TableOfPieceGroups["icedrill"],z_axis,speed)
	--TODO Retract Table
end


-- UnitBuilding Functions
function randomShowDecoration()
	process(TableOfPieceGroups["Deco"]
	function(id)
		if math.random(0,1) == 1 then Show(id) end		
	end)
end


showFunctions = {
	[UnitDefNames["drillship"].id] = function ()
		showT(TableOfPieceGroups["Boatbody"])
		showT(TableOfPieceGroups["Turret"][1])
		showT(TableOfPieceGroups["icedrill"])
		setIceDrillLoad(0)
		stopIcedrill(0)
		randomShowDecoration()
		
		
		
	end,
	
	[UnitDefNames["transportship"].id] = function ()
		showT(TableOfPieceGroups["Boatbody"])
		randomShowDecoration()
	end,
	
	[UnitDefNames["canonboat"].id] = function ()
		showT(TableOfPieceGroups["Boatbody"])
		showT(TableOfPieceGroups["Turret"][2])
		randomShowDecoration()
	end
}

TableOfPieceGroups = getPieceTableByNameGroups(false, true)

function showAndTell()
	hideAll(unitID)
	showFunctions[myDefID]()
	
	
	
end


function script.HitByWeapon(x, z, weaponDefID, damage)
end


function script.Create()
	resetAll(unitID)
	showAndTell()
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
	startIcedrill(1.0)
	return 1
end

function script.Deactivate()
	stopIcedrill(1.0)
	return 0
end
