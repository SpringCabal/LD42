function widget:GetInfo()
  return {
    name      = "resource-ui",
    desc      = "Displays game resources",
    author    = "gajop",
    date      = "LD42",
    license   = "GNU GPL, v2 or later",
    layer     = 0,
    enabled   = true,
  }
end

local Chili

local ctrl

local maxFood
local maxHeat

function widget:Initialize()
    Chili = WG.Chili
    GenerateUI()

    maxFood = Spring.GetGameRulesParam("maxFood")
    maxHeat = Spring.GetGameRulesParam("maxHeat")
end

local EL_SIZE = 40
function MakeUI(values)
    local ctrl = Chili.Control:New {
        parent = Chili.Screen0,
        x = "40%",
        bottom = 10,
        width = 500,
        height = 60,
        autosize = false,
        resizeItems = false,
        -- autosize = true,
    }
    local childIndex = 0
    for _, v in ipairs(values) do
        local lblCaption = Chili.Label:New {
            parent = ctrl,
            x = childIndex * 110 + 30,
            y = 0,
            height = 30,
            width = 100,
            caption = v.name,
        }
        local pb = Chili.Progressbar:New {
            parent = ctrl,
            x = childIndex * 110,
            y = 20,
            height = 30,
            width = 100,
            value = v.value,
            max = v.max,
            color = v.color,
            caption = tostring(math.ceil(v.value)),
        }
        ctrl["pb" .. v.name] = pb
        ctrl["lb" .. v.name] = lbl
        childIndex = childIndex + 1
    end
    return ctrl
end

function GenerateUI()
    if ctrl ~= nil then
        ctrl:Dispose()
        ctrl = nil
    end

    local values = GetBarValues()
    ctrl = MakeUI(values)
    UpdateBars(values)
end

function GetBarValues()
    local food = Spring.GetGameRulesParam("food")
    local heat = Spring.GetGameRulesParam("heat")
    return {
        {
            value = food,
            max = maxFood,
            color = { 0, 1, 0, 1 },
            name = "Food"
        },
        {
            value = heat,
            max = maxHeat,
            color = { 0.5, 0.3, 0.1, 1.0 },
            name = "Heat",
        }
    }
end

function UpdateBars(values)
    for _, v in ipairs(values) do
        local pb = ctrl["pb" .. v.name]
        pb.max = v.max
        pb.color = v.color
        pb:SetValue(v.value)
        pb:SetCaption(math.ceil(tostring(v.value)))
        pb:Invalidate()
    end
end

function widget:Update()
    UpdateBars(GetBarValues())
end
