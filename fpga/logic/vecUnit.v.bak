module vecUnit (
    input wire [127:0] bus,
    output wire [63:0] result_vec
);
    wire [2:0] global_opcode = bus[127:125];

    wire [7:0] a0 = bus[8 +: 8];
    wire [7:0] b0 = bus[0 +: 8];
    wire [7:0] r0 = result_vec[0 +: 8];
    opUnit u0(a0, b0, r0, global_opcode);

    wire [7:0] a1 = bus[24 +: 8];
    wire [7:0] b1 = bus[16 +: 8];
    wire [7:0] r1 = result_vec[8 +: 8];
    opUnit u1(a1, b1, r1, global_opcode);

    wire [7:0] a2 = bus[40 +: 8];
    wire [7:0] b2 = bus[32 +: 8];
    wire [7:0] r2 = result_vec[16 +: 8];
    opUnit u2(a2, b2, r2, global_opcode);

    wire [7:0] a3 = bus[56 +: 8];
    wire [7:0] b3 = bus[48 +: 8];
    wire [7:0] r3 = result_vec[24 +: 8];
    opUnit u3(a3, b3, r3, global_opcode);
endmodule
