use locenv::api::LuaState;
use locenv::FunctionEntry;
use locenv_macros::loader;
use std::os::raw::c_int;

const MODULE_FUNCTIONS: [FunctionEntry; 1] = [FunctionEntry {
    name: "setupsh",
    function: Some(setupsh),
}];

extern "C" fn setupsh(lua: *mut LuaState) -> c_int {
    0
}

#[loader]
extern "C" fn loader(lua: *mut LuaState) -> c_int {
    locenv::create_table(lua, 0, 1);
    locenv::set_functions(lua, &MODULE_FUNCTIONS, 0);

    1
}
