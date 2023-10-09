use std::ffi::c_void;

#[export_name = "sndserver_filename"] // or #[no_mangle]
pub static SNDSERVER_FILENAME: &[u8; 13] = b"./sndserver \0";

#[no_mangle]
pub extern "C" fn I_SubmitSound() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_InitSound() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_ShutdownMusic() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_ShutdownSound() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_PauseSong(_song: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_ResumeSong(_song: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_SetChannels() {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_StartSound(_id: i32, _vol: i32, _sep: i32, _pitch: i32, _priority: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_GetSfxLumpNum(_: *const c_void) -> i32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_SoundIsPlaying(_: i32) -> i32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_UpdateSoundParams(_handle: i32, _vol: i32, _sep: i32, _pitch: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_SetMusicVolume(_: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_RegisterSong(_: *const c_void) -> i32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_PlaySong(_handle: i32, _looping: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_StopSong(_handle: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_UnRegisterSong(_handle: i32) {
    todo!()
}

#[no_mangle]
pub extern "C" fn I_StopSound(_handle: i32) {
    todo!()
}
