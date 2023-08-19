local ___MAR_MODULES___ = {}

function ___MAR_REQUIRE___(name)
    local module = ___MAR_MODULES___[name]
    if not module then
        print('module not found: ', name)  
    end
    return module
end

---- test_main_lua ----
function entrypoint()
    print("Hello World")
end

---- end test_main_lua ----
        
entrypoint()