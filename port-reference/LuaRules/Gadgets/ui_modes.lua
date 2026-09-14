function gadget:GetInfo()
  return {
    name      = "ui_modes",
    desc      = "",
    author    = "gajop",
    date      = "in the future",
    license   = "GPL-v2",
    layer     = 1001,
    enabled   = true,
  }
end

if gadgetHandler:IsSyncedCode() then

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

function gadget:RecvLuaMsg(msg, playerID)
    local msgParams = explode('|', msg)
    if msgParams[1] ~= "MODE" then
        return
    end

    local uiMode = tonumber(msgParams[2])
    if Spring.GetGameRulesParam('uiMode') == uiMode then
        uiMode = -1
    end
    Spring.SetGameRulesParam("uiMode", uiMode)



    -- Custom handling (TODO: Move elsewhere later)
    if uiMode == 2 then
        if GG.Resources.Consume("money", 50) then
            GG.Resources.Add("heat", 100)
        end
        Spring.SetGameRulesParam("uiMode", -1)
    elseif uiMode == 3 then
        if GG.Resources.Consume("money", 50) then
            GG.Resources.Add("food", 100)
        end
        Spring.SetGameRulesParam("uiMode", -1)
    end

end

function gadget:Initialize()
    Spring.SetGameRulesParam("uiMode", -1)
end

else

function gadget:KeyPress(key)
    local keyNum = -1
    for i = 1, 3 do
        if Spring.GetKeyCode(tostring(i)) == key then
            keyNum = i
            break
        end
    end
    if keyNum == -1 then
        return
    end

    Spring.SendLuaRulesMsg("MODE|" .. tostring(keyNum))
end

end
