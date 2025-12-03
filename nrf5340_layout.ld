/* Layout for the nRF5340-DK and nRF5340 dongle, used by the
 * app in this repository.
 */

MEMORY {
  /* The application region is 64 bytes (0x40) and we reserve 0x40000 at the end
   * of the flash for the persistent storage.
   * Example from the nrf52840:
   * FLASH (X) : ORIGIN = 0x00040040, LENGTH = 0x0007FFC0
   * RAM (W) : ORIGIN = 0x20020000, LENGTH = 128K
   * Here is updated for the nRF5340
  */
  FLASH (X) : ORIGIN = 0x00080040, LENGTH = 0x000BFFC0 /* 1 MB - 256 KB - 64 bytes */
  RAM (W) : ORIGIN = 0x20020000, LENGTH = 384K /* 512 KB - 128 KB reserved for kernel */
}

TBF_HEADER_SIZE = 0x60;
INCLUDE libtock_layout.ld
