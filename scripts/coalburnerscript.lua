include "lib_UnitScript.lua"
include "lib_Animation.lua"

myDefID= Spring.GetUnitDefID(unitID)

-- functions intended for external calls by gadgets/widgets

Inards = piece "Inards"
TableOfPieceGroups = getPieceTableByNameGroups(false, true)
function setHeat(factor)
	hideAll(unitID)

	factor= math.max(0,math.min(1,factor)) or 0
	index = (factor) * 4
	index= math.max(1,math.min(4,index + 1)) or 1

	
	if index ==1 then Show(Inards) else Hide(Inards) end
	if TableOfPieceGroups["Heater"][index] then Show (TableOfPieceGroups["Heater"][index] ) end

	if factor > 0.25 then
		
		for i=1,#TableOfPieceGroups["firecycle"],1 do
		Show(TableOfPieceGroups["firecycle"][i])
		val = 0
		if i~= 1 then
		val = 150
		else 
		val = -150 
			
		end
		
		Spin(TableOfPieceGroups["firecycle"][i],x_axis,math.rad(val),0)
		end
	end
end
function buildUnitByType()
	setHeat(0)
	



end


function script.HitByWeapon(x, z, weaponDefID, damage)
end


function script.Create()
	Spring.SetUnitNoSelect(unitID,true);
	Spring.SetUnitNoMinimap(unitID,true);
	Spring.SetUnitNeutral(unitID,true);
	Spring.SetUnitRulesParam(unitID,'untargetable',1)

	resetAll(unitID)
	buildUnitByType()
end

function script.Killed(recentDamage, _)

	return 1
end



function script.Activate()
		setHeat(1)
	return 1
end

function script.Deactivate()
		setHeat(0)
	return 0
end
