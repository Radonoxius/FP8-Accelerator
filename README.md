# FP8 Accelerator
Ever since `AI and Machine Learning` have gained popularity, there has been an ever increasing demand for `GPUs` and similar co-processors. The research involved in this area strives to achieve better `Model` performance by using **fewer parameters and smaller quantization data types**.

One of the popular data types used in quantization is the FP8 E4M3 type, the float type which uses 8 bits.

This project aims to create a working model of a simple `8 way, SIMD Floating Point Unit` called `VFP8`, **for the FP8 E4M3 type**.

The FP8 type used here is the one defined by Nvidia and ARM for ML use-cases.

## Basic Details
Basically, we configure the FPGA as the accelerator/peripheral and send data to it
from the Cortex-A9 CPU. The FPGA processes it and returns the processed data back to the CPU
via the 128 bit AXI Bus.

The FP8 accelerator supports addition, subtraction, multiplication, division and
inverse operations.

This project is compatible **ONLY** with the `Altera DE1-SoC` board.

You also need to have Linux 3.x (no GUI) installed on the DE1-SoC for this project to work.
Also make sure to set all the `MSEL` switches to zero.

## Accuracy & Performance
You can expect 78% to 92% accuracy depending on the operation in calculations, while using VFP8.

The accuracy metric is obtained by comparing 32 bit float results on the `FPU` found in consumer laptops,
with our implementations.

As far as performance is concerned, the speed of calculations using VFP8 is on par/slightly faster than the
FP8 soft-arithmetic done on Single Core of Cortex-A9 with maximum optimisations and `LTO` applied

`1 MFLOPS` is the approximate (DMAS) throughput of VFP8.

## Usage
There is a very simple application in the `sample` folder that demonstrates how the VFP8 accelerator
is used and showcases a subset of the driver API.

## Limitations
As said, this is a very simple and naive implementation. These are the current limitations:

+ VFP8 uses a Single Cycle implementation
+ No streaming/pipelining features
+ No proper registers
+ No DMA support in Driver and Hardware

And
+ Rust Driver is a user-mode Driver
+ VFP8 requires CPU to send data, which stalls that CPU core

## Build Requirements
If you want to use this project in your work, check the Releases page instead.

To build Rust code, you require
+ `clang` and `lld`
+ `rustc` (v1.85+, Rust edition 2024) and `cargo`
+ `armv7-unknown-linux-musleabihf` rust target

Bash and Linux is optional, but nice to have.

If you want access to the complete VFP8 (Quartus) project,
please contact the project members personally. We cant provide the
full project on GitHub due to license restrictions.

The Verilog modules we designed are covered under
the project license.

## Project Members
Nimesh R Acharya _aka Radonoxius_ : 241EC137,
[Contact Nimesh via mail](mailto:Radonoxius@protonmail.com)

Adithya Sudhanva Arcot _aka adithyaarcot_ : 241EC102,
[Contact Aditya via mail](mailto:adithyasudhanvaaarcot@gmail.com)