#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>

#define LW_BRIDGE_BASE    0xFF200000
#define LW_BRIDGE_SPAN    0x00002000

#define LED_OFFSET        0x00001000   // 0xFF201000 - 0xFF200000

int main() {
    int fd;
    void *virtual_base;
    volatile uint32_t *led;

    // Open /dev/mem
    fd = open("/dev/mem", O_RDWR | O_SYNC);
    if (fd < 0) {
        perror("ERROR: Could not open /dev/mem");
        return 1;
    }

    // Map the Lightweight Bridge into virtual address space
    virtual_base = mmap(NULL, LW_BRIDGE_SPAN,
                        PROT_READ | PROT_WRITE,
                        MAP_SHARED, fd, LW_BRIDGE_BASE);

    if (virtual_base == MAP_FAILED) {
        perror("ERROR: mmap() failed");
        close(fd);
        return 1;
    }

    // Point to LED register
    led = (volatile uint32_t *)(virtual_base + LED_OFFSET);

    printf("LED Blinker Started. Press Ctrl+C to stop.\n");

    // Blink loop
    while (1) {
        *led = 1;
        printf("LED ON\n");
        usleep(500000);  // 500ms

        *led = 0;
        printf("LED OFF\n");
        usleep(500000);
    }

    // Cleanup (unreachable here but good practice)
    munmap(virtual_base, LW_BRIDGE_SPAN);
    close(fd);
    return 0;
}