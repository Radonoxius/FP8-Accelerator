#pragma once

#include <stdint.h>

//Represents an 8 bit floating point number
//
//Uses the FP8-E4M3 format defined by Nvidia and ARM for
//AI inference.
typedef struct Fp8 {
    uint8_t byte;
} Fp8;

//Represents the FP8 Accelerator device instance
//
//You can ONLY have 1 instance of this in your program.
//Its undefined behaviour otherwise
//
//### NOT MT SAFE
typedef struct Vfp8Accelerator {
    uint32_t *base_addr;
    int32_t mem_fd;
} Vfp8Accelerator;

//Represents the arithmetic operators supported by vfp8 accelerator
typedef enum Vfp8Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Inverse,
    Fma,

    Idle
} Vfp8Operator;


//Initializes the Accelerator
//
//Call this ONCE in your program.
static Vfp8Accelerator init(void);

//Deactivates the Accelerator. Use this at the end,
//once all the work is done
//
//Call this ONCE in your program.
static void destroy(Vfp8Accelerator device);