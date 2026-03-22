#pragma once

#include <stdint.h>

typedef struct Vfp8Accelerator {
    uint32_t *base_addr;
    int32_t mem_fd;
} Vfp8Accelerator;

Vfp8Accelerator init(void);

void destroy(Vfp8Accelerator device);