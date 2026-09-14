-- Wiki: http://springrts.com/wiki/Gamedev:Glossary#Unit_script
-- Enables the original Lua unit-script framework. Gameplay is implemented by
-- Core-WASM; the Lua scripts are still authoritative for unit animation.

UNITSCRIPT_DIR = "scripts/"

include("luagadgets/gadgets/unit_script.lua")

-- The Rust rules port owns drilling now, but the original ship script still
-- calls this hook while it starts its mining animation. Keep that animation
-- path alive without loading the old Lua terrain-deformation gadget.
GG.Drill = GG.Drill or {}
GG.Drill.DrillUnit = GG.Drill.DrillUnit or function() end

-- Core exposes the engine-canonical scriptName, including `scripts/`, while
-- the shared framework expects the basename before it prepends
-- UNITSCRIPT_DIR. Keep the compatibility view installed for the entire
-- lifetime of the gadget: the framework looks up UnitDefs again from its
-- UnitCreated callin, after Initialize has returned. Restoring the engine
-- table after Initialize makes those lookups miss every script.
local engine_unit_defs = UnitDefs
local framework_unit_defs = {}
for id, unit_def in ipairs(engine_unit_defs) do
    local copy = {
        scriptName = unit_def.scriptName,
    }
    if type(copy.scriptName) == "string" then
        copy.scriptName = copy.scriptName:gsub("^scripts/", "")
    end
    setmetatable(copy, { __index = unit_def })
    framework_unit_defs[id] = copy
end

local system = System or _G.System
UnitDefs = framework_unit_defs
if system then
    system.UnitDefs = framework_unit_defs
end

local framework_unit_created = gadget.UnitCreated
local scripted_units = {}
gadget.UnitCreated = function(self, unit_id, unit_def_id)
    scripted_units[unit_id] = true
    return framework_unit_created(self, unit_id, unit_def_id)
end

-- Core-Wasm's CreateUnit callout dispatches the unit lifecycle to the Core
-- module, but this engine build does not re-enter LuaRules' UnitCreated
-- callin for that nested creation. Reconcile the same event at the next
-- synced frame; this is only a transport bridge, while the framework and all
-- per-unit animation code remain the original Lua implementation.
local framework_game_frame = gadget.GameFrame
gadget.GameFrame = function(self, frame)
    framework_game_frame(self, frame)
    for _, unit_id in ipairs(Spring.GetAllUnits()) do
        if not scripted_units[unit_id] then
            self:UnitCreated(unit_id, Spring.GetUnitDefID(unit_id))
        end
    end
end
