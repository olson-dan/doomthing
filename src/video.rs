use std::ffi::{c_char, c_void};

#[no_mangle]
pub extern "C" fn I_SetPalette(_ptr: *const c_void) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_FinishUpdate() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_UpdateNoBlit() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_StartFrame() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_StartTic() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_InitGraphics() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_ReadScreen(_ptr: *const c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_ShutdownGraphics() {
    todo!()
}
