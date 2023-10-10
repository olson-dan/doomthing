use anyhow::{bail, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use std::ffi::c_char;
use std::ffi::c_void;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::str;

pub struct Lump {
    pub name: String,
    pub data: Vec<u8>,
    pub pos: u64,
    pub len: usize,
}

pub struct Wad {
    pub lumps: Vec<Lump>,
}

impl Wad {
    pub fn from_reader<T>(mut data: T) -> Result<Wad>
    where
        T: Read + Seek,
    {
        let mut tag = [0u8; 4];
        data.read_exact(&mut tag)?;
        let tag = str::from_utf8(&tag)?;
        if tag != "IWAD" && tag != "PWAD" {
            bail!(format!("Invalid WAD magic"));
        }
        let num_lumps = data.read_u32::<LittleEndian>()? as usize;
        let offset = data.read_u32::<LittleEndian>()? as u64;

        data.seek(SeekFrom::Start(offset))?;

        let mut lumps: Vec<Lump> = Vec::with_capacity(num_lumps);
        for _ in 0..num_lumps {
            let pos = data.read_u32::<LittleEndian>()? as u64;
            let len = data.read_u32::<LittleEndian>()? as usize;
            let mut name = [0u8; 8];
            data.read_exact(&mut name)?;
            let name = str::from_utf8(&name)?.trim_end_matches('\0').to_string();
            lumps.push(Lump {
                name,
                pos,
                len,
                data: Vec::new(),
            });
        }
        for lump in lumps.iter_mut() {
            data.seek(SeekFrom::Start(lump.pos))?;
            let mut contents = vec![0u8; lump.len];
            data.read_exact(&mut contents)?;
            lump.data = contents;
        }

        Ok(Wad { lumps })
    }

    pub fn get_num_for_name(&self, name: &str) -> Option<usize> {
        for (i, lump) in self.lumps.iter().enumerate() {
            if lump.name == name {
                return Some(i);
            }
        }
        None
    }

    pub fn cache_lump_num(&self, num: usize) -> Option<&[u8]> {
        if num < self.lumps.len() {
            return Some(&self.lumps[num].data);
        }
        None
    }

    pub fn cache_lump_name(&self, name: &str) -> Option<&[u8]> {
        if let Some(lump) = self.lumps.iter().rev().find(|l| l.name == name) {
            Some(&lump.data)
        } else {
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn W_InitMultipleFiles(_filenames: *const *const c_char) {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_Reload() {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_CheckNumForName(_name: *const c_char) -> i32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_GetNumForName(_name: *const c_char) -> i32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_LumpLength(_lump: i32) -> i32 {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_ReadLump(_lump: i32, _dest: *mut c_void) {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_CacheLumpNum(_lump: i32, _tag: i32) -> *const c_void {
    todo!()
}

#[no_mangle]
pub extern "C" fn W_CacheLumpName(_name: *const c_char, _tag: i32) -> *const c_void {
    todo!()
}
