local Abs = piece('Abs');
local ArmLeft = piece('ArmLeft');
local ArmRight = piece('ArmRight');
local Gun = piece('Gun');
local GunMuzzle = piece('GunMuzzle');
local HandLeft = piece('HandLeft');
local HandRight = piece('HandRight');
local Head = piece('Head');
local KneeLeft = piece('KneeLeft');
local KneeRight = piece('KneeRight');
local Spear = piece('Spear');
local ThighLeft = piece('ThighLeft');
local ThighRight = piece('ThighRight');
local Torso = piece('Torso');

local scriptEnv = {	Abs = Abs,
	ArmLeft = ArmLeft,
	ArmRight = ArmRight,
	Gun = Gun,
	GunMuzzle = GunMuzzle,
	HandLeft = HandLeft,
	HandRight = HandRight,
	Head = Head,
	KneeLeft = KneeLeft,
	KneeRight = KneeRight,
	Spear = Spear,
	ThighLeft = ThighLeft,
	ThighRight = ThighRight,
	Torso = Torso,
	x_axis = x_axis,
	y_axis = y_axis,
	z_axis = z_axis,
}

local Animations = {};
-- you can include externally saved animations like this:
-- Animations['importedAnimation'] = VFS.Include("Scripts/animations/animationscript.lua", scriptEnv)

Animations['guy'] = {
}

function constructSkeleton(unit, piece, offset)
    if (offset == nil) then
        offset = {0,0,0};
    end

    local bones = {};
    local info = Spring.GetUnitPieceInfo(unit,piece);

    for i=1,3 do
        info.offset[i] = offset[i]+info.offset[i];
    end 

    bones[piece] = info.offset;
    local map = Spring.GetUnitPieceMap(unit);
    local children = info.children;

    if (children) then
        for i, childName in pairs(children) do
            local childId = map[childName];
            local childBones = constructSkeleton(unit, childId, info.offset);
            for cid, cinfo in pairs(childBones) do
                bones[cid] = cinfo;
            end
        end
    end        
    return bones;
end

function script.Create()
    local map = Spring.GetUnitPieceMap(unitID);
    local offsets = constructSkeleton(unitID,map.Scene, {0,0,0});
    
    for a,anim in pairs(Animations) do
        for i,keyframe in pairs(anim) do
            local commands = keyframe.commands;
            for k,command in pairs(commands) do
                -- commands are described in (c)ommand,(p)iece,(a)xis,(t)arget,(s)peed format
                -- the t attribute needs to be adjusted for move commands from blender's absolute values
                if (command.c == "move") then
                    local adjusted =  command.t - (offsets[command.p][command.a]);
                    Animations[a][i]['commands'][k].t = command.t - (offsets[command.p][command.a]);
                end
            end
        end
    end
end
            
local animCmd = {['turn']=Turn,['move']=Move};
function PlayAnimation(animname)
    local anim = Animations[animname];
    for i = 1, #anim do
        local commands = anim[i].commands;
        for j = 1,#commands do
            local cmd = commands[j];
            animCmd[cmd.c](cmd.p,cmd.a,cmd.t,cmd.s);
        end
        if(i < #anim) then
            local t = anim[i+1]['time'] - anim[i]['time'];
            Sleep(t*33); -- sleep works on milliseconds
        end
    end
end

local SIG_WALK =  tonumber("00001",2);
local SIG_IDLE =  tonumber("00010",2);


            
