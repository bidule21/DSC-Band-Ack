use band_ack::error::TickError;



extern {
    fn call_band_ack(address: u8, command: u8, ticks: &u16) -> u8;
}



/// Get the ticks counter on the devices with the provided address
///
/// address     Address of the target device, configured by dip switches
/// return      Error or ticks
pub fn get_ticks(address: u8) -> Result<u16, TickError> {
    let ticks = 0_u16;
    let error = unsafe { call_band_ack(address, 1_u8, &ticks) };
    if let Some(error) = TickError::from_u8(error) {
        return Err(error)
    }
    return Ok(ticks)
}
