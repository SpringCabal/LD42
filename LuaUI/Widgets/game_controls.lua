local controls = VFS.Include("LuaHandler/Utilities/game_controls.lua")

widget.GetInfo = controls.GetInfo
function widget:Initialize()
	controls.Initialize(self, true)
end
widget.KeyPress = controls.KeyPress
widget.Shutdown = controls.Shutdown
