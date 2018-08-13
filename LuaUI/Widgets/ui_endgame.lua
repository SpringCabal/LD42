--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

function widget:GetInfo()
  return {
    name      = "Chili EndGame Window",
    desc      = "Derived from v0.005 Chili EndGame Window by CarRepairer",
    author    = "Anarchid",
    date      = "April 2015",
    license   = "GNU GPL, v2 or later",
    layer     = 0,
    enabled   = true,
  }
end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
local spSendCommands			= Spring.SendCommands

local echo = Spring.Echo

local Chili
local Image
local Button
local Checkbox
local Window
local Panel
local ScrollPanel
local StackPanel
local Label
local screen0
local color2incolor
local incolor2color

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------

local window_endgame
local frame_delay = 0
local sentGameStart = false
local playerName = ""
local nameBox, restartButton, submitButton, lblUpload
local gameOverTime

local function ShowEndGameWindow()
	screen0:AddChild(window_endgame)
end

local function SetupControls()
	local winSizeX, winSizeY = Spring.GetWindowGeometry()
	local width, height = 400, 400

	window_endgame = Window:New{
		name = "GameOver",
		--caption = "Game Over",
		x = (winSizeX - width)/2,
		y = winSizeY/2 - height*0.65,
		width  = width,
		height = height,
		padding = {8, 8, 8, 8};
		--autosize   = true;
		--parent = screen0,
		draggable = false,
		resizable = false,
	}

    local survialTime = math.floor((Spring.GetGameFrame() - Spring.GetGameRulesParam("startTime")) / 33)

	Chili.Label:New{
 		x = 60,
 		y = 30,
 		width = 100,
 		parent = window_endgame,
 		caption = "Game Over",
 		fontsize = 50,
 		textColor = {1,0,0,1},
 	}

	Chili.Label:New{
 		x = 114,
 		y = 155,
 		width = 100,
 		parent = window_endgame,
 		caption = "Survival time: " .. survialTime .. "s",
 		fontsize = 32,
 		textColor = {1,0,0,1},
 	}

    restartButton = Button:New{
		bottom  = 30;
		width   = 90;
		x       = 150;
		height  = 55;
		caption = "Restart",
 		fontsize = 20,
		OnClick = {
			function()
				Spring.SendCommands("cheat", "luarules reload", "cheat")
                window_endgame:Dispose()
                window_endgame = nil
                frame_delay = Spring.GetGameFrame()
            end
		};
		parent = window_endgame;
	}
	Button:New{
		bottom  = 30;
		width   = 90;
		x       = 260;
		height  = 55;
		caption = "Exit",
 		fontsize = 20,
		OnClick = {
			function()
				nameBox = nil
				restartButton = nil
				Spring.SendCommands("quit","quitforce")
			end
		};
		parent = window_endgame;
	}

	screen0:AddChild(window_endgame)

end

--------------------------------------------------------------------------------
--------------------------------------------------------------------------------
--callins

include('keysym.h.lua')
local RETURN = KEYSYMS.RETURN
function widget:KeyPress(key, mods, isRepeat)
	if key == RETURN and restartButton and restartButton.OnClick and restartButton.OnClick[1] and
			submitButton and submitButton.OnClick and submitButton.OnClick[1] then
        submitButton.OnClick[1]()
		restartButton.OnClick[1]()
        return true
    end
end

function widget:Initialize()
	if (not WG.Chili) then
		widgetHandler:RemoveWidget()
		return
	end

	Chili = WG.Chili
	Image = Chili.Image
	Button = Chili.Button
	Checkbox = Chili.Checkbox
	Window = Chili.Window
	Panel = Chili.Panel
	ScrollPanel = Chili.ScrollPanel
	StackPanel = Chili.StackPanel
	Label = Chili.Label
	screen0 = Chili.Screen0
	color2incolor = Chili.color2incolor
	incolor2color = Chili.incolor2color

end

function widget:GameFrame()
    local gameEnd = Spring.GetGameRulesParam("gameEnd")
    if gameEnd == "victory" or gameEnd == "loss" then
        widget:GameOver({})
    end
end

function widget:GameOver(winningAllyTeams)
    if window_endgame or Spring.GetGameFrame() - frame_delay < 300 then
        return
    end

    local myAllyTeamID = Spring.GetMyAllyTeamID()
    for _, winningAllyTeamID in pairs(winningAllyTeams) do
        if myAllyTeamID == winningAllyTeamID then
            Spring.SendCommands("endgraph 0")
            SetupControls()
            caption:SetCaption("You win!");
            caption.font.color={0,1,0,1};
            ShowEndGameWindow()
            return
        end
    end
    Spring.SendCommands("endgraph 0")
    SetupControls()
    ShowEndGameWindow()
end
