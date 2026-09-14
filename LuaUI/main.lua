-- The Core UI module owns the normal LuaUI drawing/input callins. This small
-- raw LuaUI entrypoint only supplies the layout hook that the original Lua
-- archive gets from its layout.lua widget.
function LayoutButtons(x_icons, y_icons, _command_count, _commands)
    return "", x_icons, y_icons, {}, {}, {}, {}, {}, {}, {}, {}
end
