function widget:GetInfo()
  return {
    name      = "selection_display",
    desc      = "Displays eskimo selection and attributes. Named like this to hide the SB UI",
    author    = "gajop",
    date      = "LD42",
    license   = "GNU GPL, v2 or later",
    layer     = 0,
    enabled   = true,
  }
end

local Chili
local Progressbar
local Label

local selection
local ctrls = {}

function widget:Initialize()
    Chili = WG.Chili


    selection = Spring.GetSelectedUnits()
    GenerateUI(selection)
end

local EL_SIZE = 40
function MakeUnitUI(unitID, index, values)
    idStr = tostring(unitID)

    local ctrl = Chili.Control:New {
        parent = Chili.Screen0,
        name = "ctrl_" .. idStr,
        x = 0,
        y = 50 + EL_SIZE * index,
        width = 200,
        height = 60,
        autosize = false,
        resizeItems = false,
        -- autosize = true,
    }
    local childIndex = 0
    for _, v in ipairs(values) do
        local pb = Chili.Progressbar:New {
            parent = ctrl,
            x = 0,
            y = childIndex * 11,
            height = 10,
            width = 100,
            value = v.value,
            max = v.max,
            color = v.color,
        }
        ctrl["pb" .. v.name] = pb
        childIndex = childIndex + 1
    end
    return ctrl
end

function GenerateUI(unitIDs)
    for _, ctrl in pairs(ctrls) do
        ctrl:Dispose()
    end
    ctrls = {}
    for i, unitID in pairs(unitIDs) do
        local values = GetBarValues(unitID)
        local ctrl = MakeUnitUI(unitID, i, values)
        ctrls[unitID] = ctrl

        UpdateUnitBars(unitID, values)
    end
end

local eskimoDefID = UnitDefNames["eskimo"].id

function GetEskimoBars(unitID)
    local health = Spring.GetUnitRulesParam(unitID, "health")
    local food = Spring.GetUnitRulesParam(unitID, "food")
    local heat = Spring.GetUnitRulesParam(unitID, "heat")
    return {
        {
            value = health,
            max = maxHealth,
            color = { 1, 0, 0, 1 },
            name = "health"
        },
        {
            value = food,
            max = maxFood,
            color = { 0, 1, 0, 1 },
            name = "food"
        },
        {
            value = heat,
            max = maxHeat,
            color = { 0.5, 0.3, 0.1, 1.0 },
            name = "heat",
        }
    }
end

function GetBarValues(unitID)
    local unitDefID = Spring.GetUnitDefID(unitID)
    if unitDefID == eskimoDefID then
        return GetEskimoBars(unitID)
    else
        local value, max = Spring.GetUnitHealth(unitID)
        return {
            {
                value = value,
                max = max,
                color = { 1, 0, 0, 1 },
                name = "health"
            }
        }
    end
end

function UpdateUnitBars(unitID, values)
    local ctrl = ctrls[unitID]

    for _, v in ipairs(values) do
        local pb = ctrl["pb" .. v.name]
        pb.max = v.max
        pb.color = v.color
        pb:SetValue(v.value)
        pb:Invalidate()
    end
end

local function UpdateSelection()
    local oldSelection = selection
    selection = Spring.GetSelectedUnits()
    table.sort(selection)

    if #oldSelection ~= #selection then
        return true
    end

    for i, k in ipairs(oldSelection) do
        if oldSelection[i] ~= selection[i] then
            return true
        end
    end

    return false
end

function widget:Update()
    if UpdateSelection() then
        GenerateUI(selection)
    end

    for _, unitID in pairs(selection) do
        local values = GetBarValues(unitID)
        UpdateUnitBars(unitID, values)
    end
end

function widget:UnitDestroyed(unitID)
    ctrls[unitID]:Dispose()
    ctrls[unitID] = nil
end
