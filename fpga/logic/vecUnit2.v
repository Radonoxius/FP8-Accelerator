module vecUnit2 (
    input  wire [127:0] bus,
	 input wire [127:0] opcode,
    output wire [63:0]  result_vec
);
    wire [2:0] global_opcode = opcode[127:125];

    wire [7:0] r0, r1, r2, r3, r4, r5, r6, r7;

    wire [7:0] a0 = bus[8  +: 8], b0 = bus[0  +: 8];
    wire [7:0] a1 = bus[24 +: 8], b1 = bus[16 +: 8];
    wire [7:0] a2 = bus[40 +: 8], b2 = bus[32 +: 8];
    wire [7:0] a3 = bus[56 +: 8], b3 = bus[48 +: 8];
    wire [7:0] a4 = bus[72 +: 8], b4 = bus[64 +: 8];
    wire [7:0] a5 = bus[88 +: 8], b5 = bus[80 +: 8];
    wire [7:0] a6 = bus[104 +: 8], b6 = bus[96 +: 8];
	 wire [7:0] a7 = bus[120 +: 8], b7 = bus[112 +: 8];

    opUnit2 u0(a0, b0, r0, global_opcode);
    opUnit2 u1(a1, b1, r1, global_opcode);
    opUnit2 u2(a2, b2, r2, global_opcode);
    opUnit2 u3(a3, b3, r3, global_opcode);
    opUnit2 u4(a4, b4, r4, global_opcode);
    opUnit2 u5(a5, b5, r5, global_opcode);
    opUnit2 u6(a6, b6, r6, global_opcode);
	 opUnit2 u7(a7, b7, r7, global_opcode);

    assign result_vec = {r7, r6, r5, r4, r3, r2, r1, r0};
endmodule