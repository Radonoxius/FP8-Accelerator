# FP8 Accelerator
Ever since `AI and Machine Learning` have gained popularity, there has been an ever increasing demand for `GPUs` and similar co-processors. The research involved in this area strives to achieve better `Model` performance by using **fewer parameters and smaller quantization data types**.

One of the popular data types used in quantization is the FP8 E4M3 type, the float type which uses 8 bits.

This project aims to create a working model of a `Floating Point Unit` **for the FP8 E4M3 type**.

The FP8 type used here is the one defined by Nvidia and ARM for ML use-cases.

## Technical Details
This project is only compatible with the DE1-SoC board.

Basically, we configure the FPGA as the accelerator and send data to it
from the Cortex-A9 CPU. The FPGA processes it and returns the processed data back to the CPU
via the AXI/Lite Bus.

## Members
Nimesh R Acharya _aka Radonoxius_ : 241EC137,
[Contact Nimesh via mail](mailto:Radonoxius@protonmail.com)

Adithya Sudhanva Arcot _aka adithyaarcot_ : 241EC102,
[Contact Aditya via mail](mailto:adithyasudhanvaaarcot@gmail.com)
