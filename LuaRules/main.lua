-- Core-WASM owns the MyCube gameplay and unit-script callins.
-- Keep the synced LuaRules state alive for the ordinary gadget loader.
return VFS.Include("luagadgets/gadgets.lua", nil, VFS.BASE)
