-- Core-WASM owns the MyCube gameplay callins. Lua unit scripts remain content.
-- The stock gadget loader only sees files in LuaRules/Gadgets, so retain the
-- small bridge that enables the scripts referenced by the unit definitions.
return VFS.Include("luagadgets/gadgets.lua", nil, VFS.BASE)
