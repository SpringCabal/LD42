local size = 300

function gadget:GetInfo()
  return {
    name      = "Trade UI",
    desc      = "",
    author    = "gajop",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = 1001,
    enabled   = true,
  }
end

if gadgetHandler:IsSyncedCode() then
-- SYNCED

function explode(div, str)
  if (div=='') then return false end
  local pos,arr = 0,{}
  -- for each divider found
  for st,sp in function() return string.find(str,div,pos,true) end do
    table.insert(arr,string.sub(str,pos,st-1)) -- Attach chars left of current divider
    pos = sp + 1 -- Jump past current divider
  end
  table.insert(arr,string.sub(str,pos)) -- Attach chars right of last divider
  return arr
end

-- TODO (eventually); we shouldn't be trusting unsynced to tell us of the height
function gadget:RecvLuaMsg(msg, playerID)
    local msgParams = explode('|', msg)
    if msgParams[1] ~= "TRADE" then
        return
    end

    local x, z, value = tonumber(msgParams[2]), tonumber(msgParams[3]), tonumber(msgParams[4])
    Spring.SetGameRulesParam("tradeMode", 0)

    Spring.SetHeightMapFunc(GG.Drill.DecreaseTerrain, x, z, size, 1)
end

function gadget:Initialize()
    Spring.SetGameRulesParam("meltIce", 0)
    Spring.SetGameRulesParam("tradeMode", 1)
end

else
-- UNSYNCED

Math = {}
function Math.RoundInt(x, step)
    x = math.round(x)
    return x - x % step
end

local old_x, old_z = -1, -1
local cachedValue

function CalculateIce(gx, gz, size)
    size = Math.RoundInt(size, Game.squareSize)
    gx   = Math.RoundInt(gx - size / 2, Game.squareSize)
    gz   = Math.RoundInt(gz - size / 2, Game.squareSize)

    local iceAmount = 0
    for x = 0, size, Game.squareSize do
        for z = 0, size, Game.squareSize do
            local gh = Spring.GetGroundHeight(x + gx, z + gz)
            if gh > 0 then
                iceAmount = iceAmount + gh
            end
        end
    end
    return iceAmount
end

function gadget:MousePress()
    if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
        return false
    end
    if Spring.GetGameRulesParam("tradeMode") <= 0 then
        return false
    end
    Spring.SendLuaRulesMsg("TRADE|" .. tostring(old_x) .. "|" ..
                            tostring(old_z) .. "|" .. tostring(cachedValue))
    return true
end

function gadget:MouseRelease()
    return false
end

function gadget:Update()
    if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
        return
    end
    if Spring.GetGameRulesParam("tradeMode") <= 0 then
        return
    end

    local mx, my = Spring.GetMouseState()
    local result, coords = Spring.TraceScreenRay(mx, my, true, false, false, false)
    if result ~= "ground"  then
        return
    end
    local x, z = coords[1], coords[3]

    if x == old_x and z == old_z then
        return
    end
    old_x = x
    old_z = z

    cachedValue = CalculateIce(
        x,
        z,
        size * 2 / 3
    )
end






Log = Log or {}

local LOG_SECTION = "SpringBoard"

-- Whether Log.Debug should use the LOG.INFO level.
-- This can be helpful to set to true as engine is often compiled with LOG.INFO as the minimum
local LOG_DEBUG_WITH_INFO = false
function Log.DebugWithInfo(enabled)
    LOG_DEBUG_WITH_INFO = enabled
end

-- simplified Spring.Log, see https://springrts.com/wiki/Lua_UnsyncedCtrl#Ingame_Console
function Log.Error(...)
    Spring.Log(LOG_SECTION, LOG.ERROR, ...)
end

function Log.Warning(...)
    Spring.Log(LOG_SECTION, LOG.WARNING, ...)
end

-- this should perhaps be the default and replace Spring.Echo
-- tempted to call this Info, but the actual info (LOG.INFO doesn't get printed out by default)
function Log.Notice(...)
    Spring.Log(LOG_SECTION, LOG.NOTICE, ...)
end

function Log.Info(...)
    Spring.Log(LOG_SECTION, LOG.INFO, ...)
end

-- enable debug printout in dev builds? (Spring.SetLogSectionFilterLevel)
function Log.Debug(...)
    if LOG_DEBUG_WITH_INFO then
        Spring.Log(LOG_SECTION, LOG.INFO, ...)
    else
        Spring.Log(LOG_SECTION, LOG.DEBUG, ...)
    end
end









Shaders = Shaders or {}

function Shaders.Compile(shaderCode, shaderName)
    local shader = gl.CreateShader(shaderCode)
    if not shader then
        local shaderLog = gl.GetShaderLog(shader)
        Log.Error("Errors found when compiling shader: " .. tostring(shaderName))
        Log.Error(shaderLog)
        return
    end

    local shaderLog = gl.GetShaderLog(shader)
    if shaderLog ~= "" then
        Log.Warning("Potential problems found when compiling shader: " .. tostring(shaderName))
        Log.Warning(shaderLog)
    end

    return shader
end







function gadget:Initialize()
    self.size = size
    cachedValue = 0
end

function gadget:initShader()
    local shaderFragStr = [[
    uniform sampler2D brushTex;
    void main()
    {
        vec4 brushColor = texture2D(brushTex, gl_TexCoord[0].st);
        gl_FragColor = gl_Color * brushColor.a;
        //gl_FragColor[4] = 0.5f;
        //gl_FragColor = gl_Color;
        //gl_FragColor = vec4(gl_TexCoord[0].st, 0, 1);
    }
    ]]

    local shaderTemplate = {
        fragment = shaderFragStr,
        uniformInt = {
            brushTex = 0,
        },
    }

    local shader = Shaders.Compile(shaderTemplate, "AbstractMapEditingState")
    if shader then
        self.shaderObj = {
            shader = shader,
        }
    end
end

function gadget:DrawScreen()
    if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
        return
    end
    if Spring.GetGameRulesParam("tradeMode") <= 0 then
        return
    end

    local x, y, z = Spring.GetMouseState()
    gl.Text(tostring(math.round(cachedValue)), x - 20, y + 20, 20)
end

-- minX,minY,minZ, maxX,maxY,maxZ
-- 0,   -0.5,  0,     1, 0.5,   1
function DrawRectangle()
    gl.BeginEnd(GL.QUADS, function()
        --                 gl.MultiTexCoord(0, mCoord[1], mCoord[2])
        --                 gl.MultiTexCoord(1, tCoord[1], tCoord[2] )
        gl.MultiTexCoord(0, 0, 0 )
        gl.Vertex(0, 0, 0)

        --                 gl.MultiTexCoord(0, mCoord[3], mCoord[4])
        --                 gl.MultiTexCoord(1, tCoord[3], tCoord[4] )
        gl.MultiTexCoord(0, 1, 0 )
        gl.Vertex(1, 0, 0)

        --                 gl.MultiTexCoord(0, mCoord[5], mCoord[6])
        --                 gl.MultiTexCoord(1, tCoord[5], tCoord[6] )
        gl.MultiTexCoord(0, 1, 1 )
        gl.Vertex(1, 0, 1)

        --                 gl.MultiTexCoord(0, mCoord[7], mCoord[8])
        --                 gl.MultiTexCoord(1, tCoord[7], tCoord[8] )
        gl.MultiTexCoord(0, 0, 1 )
        gl.Vertex(0, 0, 1)
    end)
end

-- local heightMargin = 2000
-- local averageGroundHeight = (minheight + maxheight) / 2
-- local shapeHeight = heightMargin + (maxheight - minheight) + heightMargin

-- Simplified function for brush texture rendering
local function DrawBrushTexture(vol_dlist)
    gl.DepthMask(false)
    if (gl.DepthClamp) then gl.DepthClamp(true) end

    gl.Culling(GL.FRONT)
    gl.DepthTest(false)
    gl.ColorMask(true, true, true, true)

    gl.CallList(vol_dlist)

    if (gl.DepthClamp) then gl.DepthClamp(false) end
    gl.DepthTest(true)
    gl.Culling(false)
end

function DrawTexturedGroundRectangle(x1,z1,x2,z2, dlist)
    if (type(x1) == "table") then
        local rect = x1
        x1,z1,x2,z2 = rect[1],rect[2],rect[3],rect[4]
    end
    gl.PushMatrix()
    local sizeX, sizeZ = x2 - x1, z2 - z1
    local y = Spring.GetGroundHeight((x1+x2)/2, (z1+z2)/2) - 1
    --   gl.Rotate(rot, 0, 1, 0)
    gl.Translate(x1, y, z1)
    gl.Scale(x2-x1, 1, z2-z1)
    DrawBrushTexture(dlist)
    gl.PopMatrix()
end

function gadget:DrawShape(shape, x, z)
    local pushAttribBits = math.bit_or(GL.COLOR_BUFFER_BIT, GL.ENABLE_BIT, GL.CURRENT_BIT)
    gl.PushPopMatrix(function()
        gl.PushAttrib(pushAttribBits)

        local scale = 1/2
        --     local rotRad = math.rad(self.rotation) + math.pi/2

        if not self.shaderObj then
            self:initShader()
            self.dlist = gl.CreateList(DrawRectangle)
        end
        gl.Texture(0, shape)
        gl.UseShader(self.shaderObj.shader)
        gl.Blending("alpha_add")
        gl.Color(0.0, 0.3, 0.9, 1.0)
        DrawTexturedGroundRectangle(x-self.size*scale, z-self.size*scale, x+self.size*scale, z+self.size*scale, self.dlist)
        gl.UseShader(0)
        gl.Texture(0, false)


        gl.PopAttrib(pushAttribBits)
    end)
end

local function Blit(tex1, tex2)
    gl.Blending("disable")
    gl.Texture(tex1)
    gl.RenderToTexture(tex2, function()
        gl.TexRect(-1,-1, 1, 1, 0, 0, 1, 1)
    end)
    gl.Texture(false)
end

function gadget:DrawWorld()
    if Spring.GetGameRulesParam("sb_gameMode") == "dev" then
        return
    end

    if not self.patternTexture then
        local name = "Bitmaps/circle.png"
        local texInfo = gl.TextureInfo(name)
        self.patternTexture = gl.CreateTexture(texInfo.xsize, texInfo.ysize, {
            fbo = true,
        })
        Blit(name, self.patternTexture)
    end
    if Spring.GetGameRulesParam("tradeMode") <= 0 then
        return
    end

    x, y = Spring.GetMouseState()
    local result, coords = Spring.TraceScreenRay(x, y, true)
    if result == "ground" then
        local x, z = coords[1], coords[3]
        self:DrawShape(self.patternTexture, x, z)
    end
end









end
