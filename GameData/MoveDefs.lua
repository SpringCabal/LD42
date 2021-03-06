-- Wiki: http://springrts.com/wiki/Movedefs.lua
-- See also; http://springrts.com/wiki/Units-UnitDefs#Tag:movementClass

local moveDefs  =    {
    {
        name            =   "Bot2x2",
        footprintX      =   2,
        footprintZ      =   2,
        maxWaterDepth   =   20,
        maxSlope        =   55,
        crushStrength   =   5,
        heatmapping     =   false,
    },
    {
        name            =   "Eskimo",
        footprintX      =   2,
        footprintZ      =   2,
        maxWaterDepth   =   10,
        maxSlope        =   20,
        crushStrength   =   5,
        heatmapping     =   false,
    }, 
	{
        name            =   "SHIP4",
        footprintX      =   4,
        footprintZ      =   4,
        maxWaterDepth   =   999,
        maxSlope        =   20,
        crushStrength   =   5,
        heatmapping     =   false,
    },
}

return moveDefs
