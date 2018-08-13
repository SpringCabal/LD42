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

Animations['idle'] = VFS.Include("Scripts/animations/idle.lua", scriptEnv)
Animations['walk'] = VFS.Include("Scripts/animations/walk.lua", scriptEnv)
Animations['aim_spear'] = VFS.Include("Scripts/animations/aim_spear.lua", scriptEnv)
Animations['aim_gun'] = VFS.Include("Scripts/animations/aim_gun.lua", scriptEnv)
Animations['throw'] = VFS.Include("Scripts/animations/throw.lua", scriptEnv)
Animations['death_shot'] = VFS.Include("Scripts/animations/death_shot.lua", scriptEnv)
Animations['death_exhausted'] = VFS.Include("Scripts/animations/death_exhausted.lua", scriptEnv)
Animations['flounder'] = VFS.Include("Scripts/animations/flounder.lua", scriptEnv)


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
    
    Spring.SetUnitMidAndAimPos(unitID, 0, 30, 0, 0, 30, 0, true)
    Spring.SetUnitRadiusAndHeight(unitID, 20, 40)

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
    Hide(Spear);
    Hide(Gun);
    local unitDefID = Spring.GetUnitDefID(unitID)
    if ( UnitDefs[unitDefID].customParams.has_gun == "1") then
        GetGun()
    end

    deathSound = UnitDefs[unitDefID].customParams.death_sound

    PlayAnimation('idle');
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
local SIG_AIM =   tonumber("00100",2);

local isThrowing = false;
local isDrowning = false;
local hasGun = false;
local deathSound = "sounds/pirate_death.ogg";

local function Walk()
    if(isDrowning) then return end;
	Signal(SIG_WALK)
	SetSignalMask(SIG_WALK)
	PlayAnimation("walk", true);
	while true do
		PlayAnimation("walk", false);
	end
end

local function Idle()
    if(isDrowning) then return end;
	Signal(SIG_WALK)
	SetSignalMask(SIG_WALK)
	PlayAnimation("idle",true)
end

local function AimGun()
    if(isDrowning) then return end;
    Signal(SIG_AIM)
    SetSignalMask(SIG_AIM)
    PlayAnimation("aim_gun")
    return true
end

local function AimSpear()
    if(isDrowning) then return end;
    Signal(SIG_AIM)
    SetSignalMask(SIG_AIM)
    Show(Spear)
    PlayAnimation("aim_spear")
    return true
end

function GetGun()
    hasGun = true;
    Hide(Spear)
    Show(Gun)
end

local function Throw()
    isThrowing = true
    Hide(Spear)
    PlayAnimation("throw")
    isThrowing = false
end

local function Shoot()
    Move(Gun, x_axis, 15,0 )
    WaitForMove(Gun, z_axis)
    Move(Gun, x_axis, 0, 15)
end

function script.StartMoving()
    if(isDrowning) then return end;
	Signal(SIG_WALK);
	StartThread(Walk);
end

function script.StopMoving()
    if(isDrowning) then return end;
	Signal(SIG_WALK);
	StartThread(Idle);
end

function script.QueryWeapon()
    if(hasGun) then
        return GunMuzzle
    else
	    return Head
    end
end

function script.AimFromWeapon()
    if(hasGun) then
        return GunMuzzle
    else
	    return Head
    end
	--return Gun
end

function script.FireWeapon()
	-- return Head
    if (hasGun) then
	    return GunMuzzle
    else
        return Spear
    end
end

function script.AimWeapon(num, heading, pitch)
    if (isDrowning) then return end;
    if (num == 1 and hasGun) then return end
    if (num == 2 and not hasGun) then return end

    local _,isLoaded = Spring.GetUnitWeaponState(unitID, num);
    if (isLoaded and not isThrowing) then
        if(hasGun) then
            return AimGun();
        else
            return AimSpear();
        end
    end
end

function script.Shot(weaponOrSomething)
    if (hasGun) then
        StartThread(Shoot);
    else
        StartThread(Throw);
    end
end

function StartDrowning() 
    if (not isDrowning) then
        isDrowning = true;
        Signal(SIG_AIM)
        Signal(SIG_IDLE)
        Signal(SIG_WALK)
        StartThread(Drown);
    end
end

function Drown()
    while (true) do
        PlayAnimation('flounder');
    end
end

function script.Killed(recentDamage, _)
	Signal(SIG_AIM);
	Signal(SIG_WALK);
	Signal(SIG_IDLE);

    local px,py,pz = Spring.GetUnitPosition(unitID);
    Spring.PlaySoundFile(deathSound, 9, px, py, pz)

    --if(recentDamage > 0) then -- this doesn't really tell apart kills and expirations, fixme
    if(math.random() > 0.5) then
        PlayAnimation('death_shot');
        return 2;
    end
    PlayAnimation('death_exhausted');
    return 1;
end