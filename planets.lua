function readAll(file)
    local f = io.open(file, "rb")
    local content = f:read("*all")
    f:close()
    return content
end

wrk.method = "POST"
wrk.body   = readAll("planets.json")
wrk.headers["Content-Type"] = "application/json"