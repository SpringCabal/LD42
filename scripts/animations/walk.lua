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

Animations['guy_walk'] = {
	{
		['time'] = 0,
		['commands'] = {
			{['c']='turn',['p']=KneeLeft, ['a']=x_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=KneeLeft, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=KneeLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='move',['p']=KneeLeft, ['a']=x_axis, ['t']=14.097874, ['s']=0.000000},
			{['c']='move',['p']=KneeLeft, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=KneeLeft, ['a']=z_axis, ['t']=28.207924, ['s']=0.000000},
			{['c']='turn',['p']=HandRight, ['a']=x_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=HandRight, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=HandRight, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='move',['p']=HandRight, ['a']=x_axis, ['t']=-36.961777, ['s']=0.000000},
			{['c']='move',['p']=HandRight, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=HandRight, ['a']=z_axis, ['t']=86.867134, ['s']=0.000000},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=0.695626, ['s']=2.005914},
			{['c']='turn',['p']=ThighRight, ['a']=y_axis, ['t']=0.000000, ['s']=0.000001},
			{['c']='turn',['p']=ThighRight, ['a']=z_axis, ['t']=0.000000, ['s']=0.000001},
			{['c']='move',['p']=ThighRight, ['a']=x_axis, ['t']=-9.532055, ['s']=0.000000},
			{['c']='move',['p']=ThighRight, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=ThighRight, ['a']=z_axis, ['t']=50.056683, ['s']=0.000000},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=-0.222396, ['s']=1.075160},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.860697, ['s']=1.092810},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=0.417736, ['s']=1.775244},
			{['c']='move',['p']=ArmRight, ['a']=x_axis, ['t']=-12.975609, ['s']=0.000000},
			{['c']='move',['p']=ArmRight, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=ArmRight, ['a']=z_axis, ['t']=87.342117, ['s']=0.000000},
			{['c']='turn',['p']=Head, ['a']=x_axis, ['t']=0.063648, ['s']=0.000000},
			{['c']='turn',['p']=Head, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=Head, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='move',['p']=Head, ['a']=x_axis, ['t']=0.798615, ['s']=0.000000},
			{['c']='move',['p']=Head, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=Head, ['a']=z_axis, ['t']=95.416656, ['s']=0.000000},
			{['c']='turn',['p']=Torso, ['a']=x_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=Torso, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=Torso, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='move',['p']=Torso, ['a']=x_axis, ['t']=-0.626301, ['s']=0.000000},
			{['c']='move',['p']=Torso, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=Torso, ['a']=z_axis, ['t']=64.305908, ['s']=0.000000},
			{['c']='turn',['p']=Abs, ['a']=x_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=Abs, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=Abs, ['a']=z_axis, ['t']=3.141593, ['s']=0.000000},
			{['c']='move',['p']=Abs, ['a']=x_axis, ['t']=-0.505653, ['s']=0.000000},
			{['c']='move',['p']=Abs, ['a']=y_axis, ['t']=0.041779, ['s']=0.000000},
			{['c']='move',['p']=Abs, ['a']=z_axis, ['t']=44.409409, ['s']=25.079689},
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=0.376785, ['s']=1.719803},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.902951, ['s']=1.056749},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=0.451559, ['s']=1.148918},
			{['c']='move',['p']=ArmLeft, ['a']=x_axis, ['t']=13.385418, ['s']=0.000000},
			{['c']='move',['p']=ArmLeft, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=ArmLeft, ['a']=z_axis, ['t']=86.867134, ['s']=0.000000},
			{['c']='turn',['p']=HandLeft, ['a']=x_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=HandLeft, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=HandLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='move',['p']=HandLeft, ['a']=x_axis, ['t']=37.371571, ['s']=0.000000},
			{['c']='move',['p']=HandLeft, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=HandLeft, ['a']=z_axis, ['t']=86.867134, ['s']=0.000000},
			{['c']='turn',['p']=KneeRight, ['a']=x_axis, ['t']=-0.093623, ['s']=5.735203},
			{['c']='turn',['p']=KneeRight, ['a']=y_axis, ['t']=0.000000, ['s']=0.000002},
			{['c']='turn',['p']=KneeRight, ['a']=z_axis, ['t']=0.000001, ['s']=0.000003},
			{['c']='move',['p']=KneeRight, ['a']=x_axis, ['t']=-15.825455, ['s']=0.000000},
			{['c']='move',['p']=KneeRight, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=KneeRight, ['a']=z_axis, ['t']=27.732956, ['s']=0.000000},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=-0.786544, ['s']=2.435916},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=-0.000000, ['s']=0.044968},
			{['c']='turn',['p']=ThighLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.006709},
			{['c']='move',['p']=ThighLeft, ['a']=x_axis, ['t']=11.485523, ['s']=0.000000},
			{['c']='move',['p']=ThighLeft, ['a']=y_axis, ['t']=2.182724, ['s']=0.000000},
			{['c']='move',['p']=ThighLeft, ['a']=z_axis, ['t']=47.444344, ['s']=0.000000},
		}
	},
	{
		['time'] = 6,
		['commands'] = {
			{['c']='move',['p']=Abs, ['a']=z_axis, ['t']=48.822319, ['s']=33.096828},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=-0.106387, ['s']=5.101182},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=0.000000, ['s']=0.000001},
			{['c']='turn',['p']=ThighLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=0.328425, ['s']=2.754006},
			{['c']='turn',['p']=ThighRight, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=ThighRight, ['a']=z_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=KneeLeft, ['a']=x_axis, ['t']=-0.943812, ['s']=7.078587},
			{['c']='turn',['p']=KneeLeft, ['a']=y_axis, ['t']=-0.000000, ['s']=0.000001},
			{['c']='turn',['p']=KneeLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000001},
		}
	},
	{
		['time'] = 8,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=0.542268, ['s']=1.241127},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.739866, ['s']=1.223138},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=0.478245, ['s']=0.200147},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=-0.272244, ['s']=0.373858},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.968648, ['s']=0.809632},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=0.582692, ['s']=1.237172},
		}
	},
	{
		['time'] = 10,
		['commands'] = {
			{['c']='move',['p']=Abs, ['a']=z_axis, ['t']=51.242229, ['s']=14.519463},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=0.532960, ['s']=3.836076},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=-0.294788, ['s']=3.739280},
			{['c']='turn',['p']=ThighRight, ['a']=y_axis, ['t']=-0.000000, ['s']=0.000001},
		}
	},
	{
		['time'] = 12,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.340849, ['s']=4.415588},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.659496, ['s']=0.401847},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.215610, ['s']=3.469273},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.307359, ['s']=2.898012},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.984140, ['s']=0.077462},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.217354, ['s']=4.000232},
		}
	},
	{
		['time'] = 15,
		['commands'] = {
			{['c']='move',['p']=Abs, ['a']=z_axis, ['t']=46.608597, ['s']=27.801796},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=1.309487, ['s']=4.659165},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=0.000001, ['s']=0.000003},
			{['c']='turn',['p']=ThighLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000003},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=-0.567114, ['s']=1.633953},
			{['c']='turn',['p']=KneeLeft, ['a']=x_axis, ['t']=-1.067677, ['s']=0.743194},
			{['c']='turn',['p']=KneeLeft, ['a']=y_axis, ['t']=-0.000000, ['s']=0.000000},
			{['c']='turn',['p']=KneeLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000001},
		}
	},
	{
		['time'] = 18,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.591030, ['s']=1.250904},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.769031, ['s']=0.547673},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.456584, ['s']=1.204870},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.497921, ['s']=0.952809},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-1.127796, ['s']=0.718280},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.578940, ['s']=1.807929},
		}
	},
	{
		['time'] = 20,
		['commands'] = {
			{['c']='move',['p']=Abs, ['a']=z_axis, ['t']=49.425346, ['s']=8.450249},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=0.675307, ['s']=3.805081},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=0.000000, ['s']=0.000002},
			{['c']='turn',['p']=ThighLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000002},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=-0.167801, ['s']=2.395877},
			{['c']='turn',['p']=ThighRight, ['a']=y_axis, ['t']=0.000000, ['s']=0.000000},
			{['c']='turn',['p']=KneeRight, ['a']=x_axis, ['t']=-1.089983, ['s']=5.978161},
			{['c']='turn',['p']=KneeRight, ['a']=y_axis, ['t']=-0.000000, ['s']=0.000002},
			{['c']='turn',['p']=KneeRight, ['a']=z_axis, ['t']=0.000001, ['s']=0.000005},
			{['c']='turn',['p']=KneeLeft, ['a']=x_axis, ['t']=-0.208444, ['s']=5.155402},
			{['c']='turn',['p']=KneeLeft, ['a']=y_axis, ['t']=0.000000, ['s']=0.000002},
			{['c']='turn',['p']=KneeLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000001},
		}
	},
	{
		['time'] = 24,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.775553, ['s']=0.922617},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=1.054217, ['s']=1.425931},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.893708, ['s']=2.185622},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.545779, ['s']=0.239292},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-1.425121, ['s']=1.486626},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-1.039714, ['s']=2.303870},
		}
	},
	{
		['time'] = 25,
		['commands'] = {
			{['c']='turn',['p']=KneeLeft, ['a']=x_axis, ['t']=0.000000, ['s']=1.250662},
			{['c']='turn',['p']=KneeLeft, ['a']=z_axis, ['t']=0.000000, ['s']=0.000001},
			{['c']='turn',['p']=ThighLeft, ['a']=x_axis, ['t']=-0.299361, ['s']=5.848007},
			{['c']='turn',['p']=ThighLeft, ['a']=y_axis, ['t']=0.008994, ['s']=0.053961},
			{['c']='turn',['p']=ThighLeft, ['a']=z_axis, ['t']=0.001342, ['s']=0.008051},
			{['c']='turn',['p']=ThighRight, ['a']=x_axis, ['t']=1.096809, ['s']=7.587657},
			{['c']='turn',['p']=ThighRight, ['a']=y_axis, ['t']=0.000000, ['s']=0.000002},
			{['c']='turn',['p']=ThighRight, ['a']=z_axis, ['t']=0.000000, ['s']=0.000001},
			{['c']='turn',['p']=KneeRight, ['a']=x_axis, ['t']=-1.240663, ['s']=0.904082},
			{['c']='turn',['p']=KneeRight, ['a']=y_axis, ['t']=-0.000000, ['s']=0.000001},
			{['c']='turn',['p']=KneeRight, ['a']=z_axis, ['t']=0.000001, ['s']=0.000001},
		}
	},
	{
		['time'] = 30,
		['commands'] = {
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.064313, ['s']=2.888794},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-1.152113, ['s']=1.638051},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.055663, ['s']=5.904309},
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.081830, ['s']=4.162343},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.621151, ['s']=2.598396},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=0.145181, ['s']=6.233332},
		}
	},
	{
		['time'] = 35,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=0.376785, ['s']=1.719803},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.902951, ['s']=1.056749},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=0.451559, ['s']=1.148918},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=-0.222396, ['s']=1.075160},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.860697, ['s']=1.092810},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=0.417736, ['s']=1.775244},
		}
	},
	{
		['time'] = 43,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=0.542268, ['s']=1.241127},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.739866, ['s']=1.223138},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=0.478245, ['s']=0.200147},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=-0.272244, ['s']=0.373858},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.968648, ['s']=0.809632},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=0.582692, ['s']=1.237172},
		}
	},
	{
		['time'] = 47,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.340849, ['s']=4.415588},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.659496, ['s']=0.401847},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.215610, ['s']=3.469273},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.307359, ['s']=2.898012},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-0.984140, ['s']=0.077462},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.217354, ['s']=4.000232},
		}
	},
	{
		['time'] = 53,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.591030, ['s']=1.250904},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.769031, ['s']=0.547673},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.456584, ['s']=1.204870},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.497921, ['s']=0.952809},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-1.127796, ['s']=0.718280},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.578940, ['s']=1.807929},
		}
	},
	{
		['time'] = 59,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.775553, ['s']=0.922617},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=1.054217, ['s']=1.425931},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=-0.893708, ['s']=2.185622},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.545779, ['s']=0.239292},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-1.425121, ['s']=1.486626},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-1.039714, ['s']=2.303870},
		}
	},
	{
		['time'] = 65,
		['commands'] = {
			{['c']='turn',['p']=ArmLeft, ['a']=x_axis, ['t']=-0.081830, ['s']=4.162343},
			{['c']='turn',['p']=ArmLeft, ['a']=y_axis, ['t']=0.621151, ['s']=2.598396},
			{['c']='turn',['p']=ArmLeft, ['a']=z_axis, ['t']=0.145181, ['s']=6.233332},
			{['c']='turn',['p']=ArmRight, ['a']=x_axis, ['t']=0.064313, ['s']=2.888794},
			{['c']='turn',['p']=ArmRight, ['a']=y_axis, ['t']=-1.152113, ['s']=1.638051},
			{['c']='turn',['p']=ArmRight, ['a']=z_axis, ['t']=-0.055663, ['s']=5.904309},
		}
	},
	{
		['time'] = 70,
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
            