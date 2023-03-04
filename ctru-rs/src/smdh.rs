use crate::error::ResultCode;
use crate::services::am::Title;
use crate::services::cfgu::Language;
use crate::services::fs::Fs;
use bitflags::bitflags;
use ctru_sys::FS_Path;
use std::mem::{size_of, MaybeUninit};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Smdh {
    magic: [u8; 4],
    version: u16,
    titles: [SmdhTitle; 16],
    age_ratings: [u8; 16],
    region_lock: RegionLock,
    matchmaker_id: u32,
    matchmaker_bit_id: u64,
    flags: SmdhFlags,
    eula_version_major: u8,
    eula_version_minor: u8,
    optimal_banner_anim_frame: f32,
    cec_id: u32,
    _pad: [u8; 8],
    small_icon: [u8; 0x480],
    large_icon: [u8; 0x1200],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SmdhTitle {
    short: [u16; 0x40],
    long: [u16; 0x80],
    publisher: [u16; 0x40],
}

bitflags! {
    pub struct SmdhFlags: u32 {
        const VISIBLE = 0x1;
        const AUTOBOOT_GAMECARD = 0x2;
        const PARENTAL_3D_ALLOW = 0x4;
        const REQUIRE_CTR_EULA = 0x8;
        const AUTOSAVE_ON_EXIT = 0x10;
        const EXTENDED_BANNER = 0x20;
        const REGION_RATING_REQUIRED = 0x40;
        const SAVEDATA_USAGE = 0x80;
        const RECORD_USAGE = 0x100;
        const DISABLE_SD_SAVEDATA_BACKUPS = 0x400;
        const NEW3DS_EXCLUSIVE = 0x1000;
    }
}

bitflags! {
    pub struct RegionLock: u32 {
        const JAPAN = 0x1;
        const NORTH_AMERICA = 0x2;
        const EUROPE = 0x4;
        const AUSTRALIA = 0x8;
        const CHINA = 0x10;
        const KOREA = 0x20;
        const TAIWAN = 0x40;
        const REGION_FREE = 0x7fff_ffff;
    }
}

// make sure types are correct size
const _SMDH_SIZE_CHECKER: [u8; 0x36c0] = [0; size_of::<Smdh>()];
const _TITLE_SIZE_CHECKER: [u8; 0x200] = [0; size_of::<SmdhTitle>()];

impl Smdh {
    pub fn load(id: Title) -> crate::Result<Self> {
        let archive_path_data: [u32; 4] =
            [id.low_u32(), id.high_u32(), id.media_type() as u32, 0x0];
        let smdh_path_data: [u32; 5] = [0x0, 0x0, 0x2, 0x6E6F6369, 0x0];

        let archive_path = FS_Path {
            type_: ctru_sys::PATH_BINARY,
            size: size_of::<[u32; 4]>() as u32,
            data: archive_path_data.as_ptr() as *const libc::c_void,
        };
        let smdh_path = FS_Path {
            type_: ctru_sys::PATH_BINARY,
            size: size_of::<[u32; 5]>() as u32,
            data: smdh_path_data.as_ptr() as *const libc::c_void,
        };

        let _fs = Fs::init();
        let mut smdh_handle = 0;

        let smdh: Smdh = unsafe {
            let mut ret = MaybeUninit::zeroed();
            ResultCode(ctru_sys::FSUSER_OpenFileDirectly(
                &mut smdh_handle,
                ctru_sys::ARCHIVE_SAVEDATA_AND_CONTENT,
                archive_path,
                smdh_path,
                ctru_sys::FS_OPEN_READ,
                0x0,
            ))?;

            ResultCode(ctru_sys::FSFILE_Read(
                smdh_handle,
                std::ptr::null_mut(),
                0x0,
                ret.as_mut_ptr() as *mut libc::c_void,
                size_of::<Smdh>() as u32,
            ))?;

            ResultCode(ctru_sys::FSFILE_Close(smdh_handle))?;

            ret.assume_init()
        };

        assert_eq!(&smdh.magic, b"SMDH");
        Ok(smdh)
    }

    pub fn short_name(&self, lang: Language) -> String {
        String::from_utf16_lossy(&self.titles[lang as usize].short)
    }

    pub fn long_name(&self, lang: Language) -> String {
        String::from_utf16_lossy(&self.titles[lang as usize].long)
    }

    pub fn publisher(&self, lang: Language) -> String {
        String::from_utf16_lossy(&self.titles[lang as usize].publisher)
    }

    pub fn version(&self) -> u16 {
        self.version
    }

    pub fn flags(&self) -> SmdhFlags {
        self.flags
    }

    pub fn region(&self) -> RegionLock {
        self.region_lock
    }
}
