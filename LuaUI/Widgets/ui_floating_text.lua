function widget:GetInfo()
	return {
		name      = 'Flaoting text',
		desc      = 'Displays floating text on events',
		author    = 'gajop',
		date      = 'LD42',
		license   = 'GNU GPL v2',
		layer     = 0,
		enabled   = true,
		handler   = true,
	}
end

local events = {}
local majorEvent

local vsx, vsy
local GAME_FRAME_PER_SEC = 33

function WG.AddEvent(opts)
	opts.str 	    = opts.str or ""
	opts.fontSize 	= opts.fontSize or 12
	opts.time     	= (opts.time or 1) * GAME_FRAME_PER_SEC
	opts.totalTime  = opts.time
	opts.color 		= opts.color or {1, 1, 1, 1}
	opts.speed 		= opts.speed or 10
    table.insert(events, opts)
end

function widget:Initialize()
    vsx, vsy = Spring.GetViewGeometry()
end

function widget:DrawScreen()
    local startPos = vsy / 4
    gl.PushMatrix()
    for i = 1, #events do
        local event = events[i]
        local str		= event.str
		local fontSize  = event.fontSize
		local timeout   = event.timeout
		local color 	= event.color
		local speed     = event.speed

		local progress = (event.totalTime - event.time) / event.totalTime

        local pos = startPos + progress * speed
        -- local size = fontSize - math.abs(event.totalTime - event.time) / GAME_FRAME_PER_SEC * 12
		local size = fontSize
        local fw = gl.GetTextWidth(str) * size
        gl.Color(color[1], color[2], color[3], color[4])
        gl.Text(str, (vsx - fw) / 2, pos, size, 's')
    end
    gl.PopMatrix()
end

local oldIntroEvent
function widget:GameFrame()
    for i = #events, 1, -1 do
        local event = events[i]
        event.time = event.time - 1
        if event.time <= 0 then
            table.remove(events, i)
        end
    end

	local introEvent = Spring.GetGameRulesParam("introEvent")
	if oldIntroEvent ~= introEvent then
		oldIntroEvent = introEvent
	else
		return
	end
	if type(introEvent) == "string" and introEvent ~= "" then
		WG.AddEvent({
			str         = introEvent,
			fontSize 	= 30,
			color 		= {0.9, 0.9, 0.9, 1},
			time 		= 5,
			speed       = -50, -- how much we want to float (ugly name)
		})
	end
end

function widget:Update()
end
