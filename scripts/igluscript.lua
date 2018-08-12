include "lib_UnitScript.lua"
include "lib_Animation.lua"

myDefID= Spring.GetUnitDefID(unitID)



TableOfPieceGroups = getPieceTableByNameGroups(false, true)

function buildUnit(speed)
	hideAll(unitID)
	Show(TableOfPieceGroups["Build"][1])
	for buildIndex=1, #TableOfPieceGroups["Build"],1 do 
		if TableOfPieceGroups["Build"][buildIndex] then
			if math.random(0,1)==1 then 
				Show(TableOfPieceGroups["Build"][buildIndex])
				rval = math.random(-360,360)
				Turn(TableOfPieceGroups["Build"][buildIndex],y_axis,math.rad(rval),0)
				
				xyIndex= "XY"..buildIndex.."Deco"
				yIndex= "Y"..buildIndex.."Deco"
				process(TableOfPieceGroups[xyIndex],
						function(id)
							if id and math.random(0,1) ==1 then
								rval = math.random(-360,360)
								Turn(id,y_axis,math.rad(rval),speed)
								rval =math.random(-80,15)
								Turn(id,x_axis,math.rad(rval),speed)
								Show(id)
							end
						
						end
						)
				
				process(TableOfPieceGroups[yIndex],
						function(id)
							if id and math.random(0,1) == 1 then
							rval = math.random(-360,360)
								Turn(id,y_axis,math.rad(rval),speed)
								Show(id)
							end
						
						end
						)
			
			
			end
		end
	
	end
	
	
	
	
end


function script.HitByWeapon(x, z, weaponDefID, damage)
end


function script.Create()
	resetAll(unitID)
	buildUnit()
end

function script.Killed(recentDamage, _)
	
	return 1
end


function script.Activate()
	
	return 1
end

function script.Deactivate()

	return 0
end
