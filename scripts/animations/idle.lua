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
	{
		['time'] = 1,
		['commands'] = {
			{['c']='turn',['p']=KneeLeft, ['a']=x_axis, ['t']=-0.034709, ['s']=0.115696},
			{['c']='turn',['p']=KneeLeft, ['a']=y_axis, ['t']=0.347905, ['s']=1.159683},
			{['c']='turn',['p']=KneeLeft, ['a']=z_axis, ['t']=0.051263, ['s']=0.170877},
			{['c']='turn',['p']=HandRight, ['a']=x_axis, ['t']=0.210026, ['s']=0.700086},
			{['c']='turn',['p']=HandRight, ['a']=y_axis, ['t']=-1.134245, ['s']=3.780815},
			{['c']='turn',['p']=HandRight, ['a']=z_axis, ['t']=0.080467, ['s']=0.268223},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=-0.030462, ['s']=0.101539},
			{['c']='turn',['p']=ThighRight, ['a']=y_axis, ['t']=0.313193, ['s']=1.043978},
			{['c']='turn',['p']=ThighRight, ['a']=z_axis, ['t']=0.046717, ['s']=0.155724},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.010293, ['s']=0.034309},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.726437, ['s']=2.421457},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.160553, ['s']=0.535178},
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.096552, ['s']=0.321842},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.747650, ['s']=2.492168},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.025568, ['s']=0.085228},
			{['c']='turn',['p']=HandLeft, ['a']=x_axis, ['t']=0.096586, ['s']=0.321954},
			{['c']='turn',['p']=HandLeft, ['a']=y_axis, ['t']=1.248623, ['s']=4.162075},
			{['c']='turn',['p']=HandLeft, ['a']=z_axis, ['t']=0.140398, ['s']=0.467994},
			{['c']='turn',['p']=KneeRight, ['a']=x_axis, ['t']=0.015395, ['s']=0.051316},
			{['c']='turn',['p']=KneeRight, ['a']=y_axis, ['t']=-0.376637, ['s']=1.255455},
			{['c']='turn',['p']=KneeRight, ['a']=z_axis, ['t']=-0.064895, ['s']=0.216316},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=0.014677, ['s']=0.048922},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=-0.320854, ['s']=1.069515},
			{['c']='turn',['p']=ThighLeft, ['a']=z_axis, ['t']=-0.054995, ['s']=0.183315},
		}
	},
	{
		['time'] = 10,
		['commands'] = {
		}
	},
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
            