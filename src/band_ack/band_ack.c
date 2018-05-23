#include <stdio.h>



/// Get the ticks counter on the devices with the provided address
///
/// address     Address of the target device, configured by dip switches
/// command     Specifies the action we perform
///             0 ask for the ticks
///             1 ping
/// ticks       Pointer to a 16 bit var which will contain the current tick counter
///
/// return      0 No error
///             1 No answer from device
///             2 Invalid answer
///             3 ...
unsigned char call_band_ack(unsigned char address, unsigned char command, unsigned short *ticks) {
  printf("get_ticks address: %i, command: %i\n", address, command);
  *ticks = 2;
  return 0;
}
