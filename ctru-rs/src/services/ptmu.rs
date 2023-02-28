use crate::error::ResultCode;

#[derive(Copy, Clone, Debug)]
pub enum ShellState {
    Closed,
    Open,
}

pub struct Ptmu(());

impl Ptmu {
    pub fn init() -> crate::Result<Ptmu> {
        unsafe {
            ResultCode(ctru_sys::ptmuInit())?;
            Ok(Ptmu(()))
        }
    }

    pub fn get_shell_state(&self) -> crate::Result<ShellState> {
        let mut state_u8 = 0;
        ResultCode(unsafe {ctru_sys::PTMU_GetShellState(&mut state_u8) })?;
        match state_u8 {
            0 => Ok(ShellState::Closed),
            1 => Ok(ShellState::Open),
            _ => unreachable!("Shell state returned by PTMU_GetShellState should never be anything but 0 or 1"),
        }
    }

    pub fn get_battery_level(&self) -> crate::Result<u8> {
        let mut out = 0;
        ResultCode(unsafe {ctru_sys::PTMU_GetBatteryLevel(&mut out)})?;
        Ok(out)
    }

    pub fn is_battery_charging(&self) -> crate::Result<bool> {
        let mut charge_u8 = 0;
        ResultCode(unsafe {ctru_sys::PTMU_GetBatteryChargeState(&mut charge_u8)})?;
        match charge_u8 {
            0 => Ok(false),
            1 => Ok(true),
            _ => unreachable!("PTMU_GetBatteryChargeState should never return charge status that is neither 0 nor 1"),
        }
    }

    pub fn is_adapter_plugged_in(&self) -> crate::Result<bool> {
        let mut status = false;
        ResultCode(unsafe {ctru_sys::PTMU_GetAdapterState(&mut status)})?;
        Ok(status)
    }

    pub fn is_pedometer_counting(&self) -> crate::Result<bool> {
        let mut status_u8 = 0;
        ResultCode(unsafe {ctru_sys::PTMU_GetPedometerState(&mut status_u8)})?;
        match status_u8 {
            0 => Ok(false),
            1 => Ok(true),
            _ => unreachable!("PTMU_GetPedometerState should never return status that is neither 0 nor 1"),
        }
    }

    pub fn get_total_step_count(&self) -> crate::Result<u32> {
        let mut count = 0;
        ResultCode(unsafe {ctru_sys::PTMU_GetTotalStepCount(&mut count)})?;
        Ok(count)
    }
}

impl Drop for Ptmu {
    fn drop(&mut self) {
        unsafe { ctru_sys::ptmuExit() };
    }
}
