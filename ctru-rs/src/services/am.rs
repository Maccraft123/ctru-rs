use crate::error::ResultCode;
use crate::services::fs::FsMediaType;
use std::marker::PhantomData;
use bitflags::bitflags;

bitflags! {
    pub struct TitleFlags: u16 {
        const NORMAL = 0x0;
        const DLP_CHILD = 0x1;
        const DEMO = 0x2;
        const CONTENTS = 0x3;
        const ADDON_CONTENTS = 0x4;
        const PATCH = 0x6;
        const CANNOT_EXECUTION = 0x8;
        const SYSTEM = 0x10;
        const REQUIRE_BATCH_UPDATE = 0x20;
        const NOT_REQUIRE_USER_APPROVAL = 0x40;
        const NOT_REQUIRE_RIGHT_FOR_MOUNT = 0x80;
        const CAN_SKIP_CONVERT_JUMP_ID = 0x100;
        const TWL = 0x8000;

        const SYSTEM_CONTENT = Self::CONTENTS.bits | Self::CANNOT_EXECUTION.bits | Self::SYSTEM.bits;
        const SHARED_CONTENT = Self::CONTENTS.bits | Self::CANNOT_EXECUTION.bits | Self::SYSTEM.bits | Self::NOT_REQUIRE_RIGHT_FOR_MOUNT.bits;
        const AUTO_UPDATE_CONTENT = Self::CONTENTS.bits | Self::CANNOT_EXECUTION.bits | Self::SYSTEM.bits | Self::NOT_REQUIRE_USER_APPROVAL.bits | Self::NOT_REQUIRE_RIGHT_FOR_MOUNT.bits;
        const APPLET = Self::NORMAL.bits | Self::SYSTEM.bits | Self::REQUIRE_BATCH_UPDATE.bits;
        const BASE = Self::NORMAL.bits | Self::SYSTEM.bits | Self::REQUIRE_BATCH_UPDATE.bits | Self::CAN_SKIP_CONVERT_JUMP_ID.bits;
        const FIRMWARE = Self::NORMAL.bits | Self::CANNOT_EXECUTION.bits | Self::SYSTEM.bits | Self::REQUIRE_BATCH_UPDATE.bits | Self::CAN_SKIP_CONVERT_JUMP_ID.bits;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Title<'a> {
    id: u64,
    mediatype: FsMediaType,
    _am: PhantomData<&'a Am>,
}

impl<'a> Title<'a> {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn low_u32(&self) -> u32 {
        (self.id & 0x0000_0000_ffff_ffff) as u32
    }
    pub fn high_u32(&self) -> u32 {
        ((self.id & 0xffff_ffff_0000_0000) >> 32) as u32
    }
    pub fn content_category(&self) -> Option<TitleFlags> {
        let flags_bits = ((self.id & 0x0000_ffff_0000_0000) >> 32) as u16;
        TitleFlags::from_bits(flags_bits)
    }
    pub fn get_product_code(&self) -> crate::Result<String> {
        let mut buf: [u8; 16] = [0; 16];
        unsafe {
            ResultCode(ctru_sys::AM_GetTitleProductCode(
                    self.mediatype as u32,
                    self.id,
                    buf.as_mut_ptr()
                    ))?;
        }
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

#[derive(Debug)]
pub struct Am(());

impl Am {
    pub fn init() -> crate::Result<Am> {
        unsafe {
            ResultCode(ctru_sys::amInit())?;
            Ok(Am(()))
        }
    }

    pub fn get_title_count(&self, mediatype: FsMediaType) -> crate::Result<u32> {
        unsafe {
            let mut count = 0;
            ResultCode(ctru_sys::AM_GetTitleCount(mediatype as u32, &mut count))?;
            Ok(count)
        }
    }

    pub fn get_title_list<'a>(&'a self, mediatype: FsMediaType) -> crate::Result<Vec<Title<'a>>> {
        let count = self.get_title_count(mediatype)?;
        let mut buf = Vec::with_capacity(count as usize);
        let mut read_amount = 0;

        unsafe {
            ResultCode(ctru_sys::AM_GetTitleList(
                &mut read_amount,
                mediatype as u32,
                count,
                buf.as_mut_ptr(),
            ))?;

            buf.set_len(read_amount as usize);
        }
        Ok(buf.into_iter()
            .map(|id| Title {id, mediatype, _am: PhantomData})
            .collect())
    }
}

impl Drop for Am {
    fn drop(&mut self) {
        unsafe { ctru_sys::amExit() };
    }
}
