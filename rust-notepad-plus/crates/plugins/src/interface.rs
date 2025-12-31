//! Plugin interface definitions
//!
//! Corresponds to PluginInterface.h from the C++ version

#[repr(C)]
pub struct PluginInfo {
    pub name: [u16; 64],
    pub version: u32,
}

#[repr(C)]
pub struct FuncItem {
    pub item_name: [u16; 64],
    pub func_ptr: usize,
    pub cmd_id: u32,
    pub init_check: bool,
}

// Plugin callback types
pub type SetInfoFn = unsafe extern "C" fn(*const PluginInfo);
pub type GetNameFn = unsafe extern "C" fn() -> *const u16;
pub type GetFuncsArrayFn = unsafe extern "C" fn(*mut i32) -> *const FuncItem;
