use crate::error::ResultCode;
use std::time::Duration;
use std::marker::PhantomData;
use ctru_sys::Handle;

pub struct NotificationSem<'a> {
    handle: Handle,
    _srv: PhantomData<&'a Srv>
}

impl<'a> NotificationSem<'a> {
    pub fn subscribe(&self, num: u32) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::srvSubscribe(num))?;
        }
        Ok(())
    }

    pub fn unsubscribe(&self, num: u32) -> crate::Result<()> {
        unsafe {
            ResultCode(ctru_sys::srvUnsubscribe(num))?;
        }
        Ok(())
    }

    pub fn recv_notification_timeout(&self, timeout: Duration) -> crate::Result<Option<u32>> {
        let mut notification = 0;
        unsafe {
            match ctru_sys::svcWaitSynchronization(self.handle, timeout.as_nanos() as i64) {
                0x09401BFE => return Ok(None), // Timed out
                other => ResultCode(other)?,
            }
            ResultCode(ctru_sys::srvReceiveNotification(&mut notification))?;
        }
        Ok(Some(notification))
    }

    pub fn poll_notification(&self) -> crate::Result<Option<u32>> {
        self.recv_notification_timeout(Duration::from_millis(0))
    }
}

pub struct Srv(());

impl Srv {
    pub fn init() -> crate::Result<Srv> {
        unsafe {
            ResultCode(ctru_sys::srvInit())?;
            Ok(Srv(()))
        }
    }

    pub fn set_non_blocking(&self, val: bool) {
        unsafe {
            ctru_sys::srvSetBlockingPolicy(val);
        }
    }

    pub fn enable_notification(&self) -> crate::Result<NotificationSem> {
        let mut handle = 0;
        unsafe {
            ResultCode(ctru_sys::srvEnableNotification(&mut handle))?;
        }
        Ok(NotificationSem{handle, _srv: PhantomData})
    }
}

impl Drop for Srv {
    fn drop(&mut self) {
        unsafe { ctru_sys::srvExit() };
    }
}
