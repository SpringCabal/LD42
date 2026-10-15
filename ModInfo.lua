local modinfo = {
	name = "MyCube WASM",
	shortname = "MyCubeWASM",
	version = "Git",
	modtype = "1",
	-- Single-player only: without this the engine binds a UDP socket and warns
	-- that nobody else can reach it. Nothing here is ever hosted or joined.
	onlyLocal = true,
	depend = {
		"cursors.sdz",
	}
}

return modinfo
