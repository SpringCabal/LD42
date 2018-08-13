-- Comments on the right are my grievances with the format

local Spear = Weapon:New {
    -- general
    weaponType            = "Cannon", -- there's a default, but honestly, why?
    name                  = "Spear",
    impactOnly            = true,
    noSelfDamage          = true,
    range                 = 1000,     -- bad defaults (only 10.0)
    weaponVelocity        = 1000,     -- default velocity is 0, wtf?
    reloadTime            = 3.0,
    tolerance             = 6000,

    -- collision & avoidance
    avoidFriendly         = false,
    avoidFeature          = false,

    collideFriendly       = false,
    collideFeature        = false,

    -- targeting & accuracy
    accuracy              = 0.1,
    model                 = 'spear.dae',

    soundStart            = [[SpearThrow]],
    soundHit              = [[Hit]],
}

local Gun = Weapon:New {
    -- general
    weaponType            = "LaserCannon", 
    name                  = "Gun",
    impactOnly            = true,
    noSelfDamage          = true,
    range                 = 1000,     -- bad defaults (only 10.0)
    weaponVelocity        = 2000,     -- default velocity is 0, wtf?
    reloadTime            = 3.0,
    burst                 = 4,
    tolerance             = 6000,

    -- collision & avoidance
    avoidFriendly         = false,
    avoidFeature          = false,

    collideFriendly       = false,
    collideFeature        = false,

    -- targeting & accuracy
    accuracy              = 0.1,

    -- sounds
    soundTrigger          = false,
    soundStart            = [[GunShot]],
    soundHit              = [[Hit]],
}

return {
	Spear = Spear,
    Gun = Gun,
}
