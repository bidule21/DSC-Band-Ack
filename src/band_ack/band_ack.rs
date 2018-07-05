use std::ffi::CString;
use std::os::raw::c_char;
use band_ack::error::TickError;



extern {
    fn call_band_ack(address: u8, device: *const c_char, command: u8, ticks: &u16) -> u8;
}



/// Get the ticks counter on the devices with the provided address
///
/// address     Address of the target device, configured by dip switches
/// device      Path to the serial device to use
/// return      Error or ticks
pub fn get_ticks(address: u8, device: String) -> Result<u16, TickError> {
    let ticks = 0_u16;
    let d = CString::new(device).unwrap();
    let error = unsafe { call_band_ack(address, d.as_ptr(), 1_u8, &ticks) };
    if let Some(error) = TickError::from_u8(error) {
        return Err(error)
    }
    return Ok(ticks)
}
