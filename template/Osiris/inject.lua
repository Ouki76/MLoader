while true do
    module = Injector.getModule("cs2.exe", "rsaenh.dll")

    message = json.decode(module).message

    if message > 0 then
        a = Injector.inject("cs2.exe", Utils.Path .. "\\Osiris.dll")
        print(json.decode(a).message)
        break
    end
end