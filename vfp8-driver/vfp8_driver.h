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
//### NOT MT-SAFE
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

//Represents the results of a computation on the accelerator
typedef struct ComputeResult {
    Fp8 res0;
    Fp8 res1;
    Fp8 res2;
    Fp8 res3;
    Fp8 res4;
    Fp8 res5;
    Fp8 res6;
    Fp8 res7;
    Fp8 res8;
    Fp8 res9;
    Fp8 res10;
    Fp8 res11;
    Fp8 res12;
    Fp8 res13;
    Fp8 res14;
    Fp8 res15;
} ComputeResult;

//Dispatches a 2 operand operation to the accelerator and returns the result.
//Expression is of the form `a op b`, where op is +, -, * or /
//
//Inverse is not supported by this function, use `compute_inverse` function instead
static ComputeResult compute(
    Vfp8Accelerator *device,
    Vfp8Operator op,

    Fp8 a0, Fp8 b0,
    Fp8 a1, Fp8 b1,
    Fp8 a2, Fp8 b2,
    Fp8 a3, Fp8 b3,
    Fp8 a4, Fp8 b4,
    Fp8 a5, Fp8 b5,
    Fp8 a6, Fp8 b6,
    Fp8 a7, Fp8 b7
);

//Dispatches the inverse operation to the accelerator and returns the result.
//
//The inverse operation returns 1.0 / a for each operand a.
static ComputeResult compute_inverse(
    Vfp8Accelerator *device,

    Fp8 a0,
    Fp8 a1,
    Fp8 a2,
    Fp8 a3,
    Fp8 a4,
    Fp8 a5,
    Fp8 a6,
    Fp8 a7,
    Fp8 a8,
    Fp8 a9,
    Fp8 a10,
    Fp8 a11,
    Fp8 a12,
    Fp8 a13,
    Fp8 a14,
    Fp8 a15
);