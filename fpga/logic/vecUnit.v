module vecUnit (
    input  wire [127:0] bus,
	 input wire [127:0] opcode,
    output wire [127:0]  result_vec
);
    wire [2:0] global_opcode = opcode[127:125];
	reg [127:0] resx;
	assign result_vec = resx;
	 
	wire [7:0] r10, r11, r12, r13, r14, r15, r16, r17, r18, r19, r110, r111, r112, r113, r114, r115;

    wire [7:0] a10 = bus[0 +: 8], a11 = bus[8 +: 8], a12 = bus[16 +: 8], a13 = bus[24 +: 8];
    wire [7:0] a14 = bus[32 +: 8], a15 = bus[40 +: 8], a16 = bus[48 +: 8], a17 = bus[56 +: 8];
    wire [7:0] a18 = bus[64 +: 8], a19 = bus[72 +: 8], a110 = bus[80 +: 8], a111 = bus[88 +: 8];
    wire [7:0] a112 = bus[96 +: 8], a113 = bus[104 +: 8], a114 = bus[112 +: 8], a115 = bus[120 +: 8];

    opUnit1 u10(a10, r10, global_opcode);
    opUnit1 u11(a11, r11, global_opcode);
    opUnit1 u12(a12, r12, global_opcode);
    opUnit1 u13(a13, r13, global_opcode);
    opUnit1 u14(a14, r14, global_opcode);
    opUnit1 u15(a15, r15, global_opcode);
    opUnit1 u16(a16, r16, global_opcode);
    opUnit1 u17(a17, r17, global_opcode);
    opUnit1 u18(a18, r18, global_opcode);
    opUnit1 u19(a19, r19, global_opcode);
    opUnit1 u110(a110, r110, global_opcode);
    opUnit1 u111(a111, r111, global_opcode);
    opUnit1 u112(a112, r112, global_opcode);
    opUnit1 u113(a113, r113, global_opcode);
    opUnit1 u114(a114, r114, global_opcode);
    opUnit1 u115(a115, r115, global_opcode);


    wire [7:0] r20, r21, r22, r23, r24, r25, r26, r27;

    wire [7:0] a20 = bus[8  +: 8], b20 = bus[0  +: 8];
    wire [7:0] a21 = bus[24 +: 8], b21 = bus[16 +: 8];
    wire [7:0] a22 = bus[40 +: 8], b22 = bus[32 +: 8];
    wire [7:0] a23 = bus[56 +: 8], b23 = bus[48 +: 8];
    wire [7:0] a24 = bus[72 +: 8], b24 = bus[64 +: 8];
    wire [7:0] a25 = bus[88 +: 8], b25 = bus[80 +: 8];
    wire [7:0] a26 = bus[104 +: 8], b26 = bus[96 +: 8];
	wire [7:0] a27 = bus[120 +: 8], b27 = bus[112 +: 8];

    opUnit2 u20(a20, b20, r20, global_opcode);
    opUnit2 u21(a21, b21, r21, global_opcode);
    opUnit2 u22(a22, b22, r22, global_opcode);
    opUnit2 u23(a23, b23, r23, global_opcode);
    opUnit2 u24(a24, b24, r24, global_opcode);
    opUnit2 u25(a25, b25, r25, global_opcode);
    opUnit2 u26(a26, b26, r26, global_opcode);
	opUnit2 u27(a27, b27, r27, global_opcode);


    wire [7:0] r30, r31, r32, r33, r34;

    wire [7:0] a30 = bus[16 +: 8], b30 = bus[8 +: 8], c30 = bus[0 +: 8];
    wire [7:0] a31 = bus[40 +: 8], b31 = bus[32 +: 8], c31 = bus[24 +: 8];
    wire [7:0] a32 = bus[64 +: 8], b33 = bus[56 +: 8], c32 = bus[48 +: 8];
    wire [7:0] a33 = bus[88 +: 8], b34 = bus[80 +: 8], c33 = bus[72 +: 8];
    wire [7:0] a34 = bus[112 +: 8], b35 = bus[104 +: 8], c34 = bus[96 +: 8];

    opUnit3 u30(a30, b30, c30, r30, global_opcode);
    opUnit3 u31(a31, b31, c31, r31, global_opcode);
    opUnit3 u32(a32, b32, c32, r32, global_opcode);
    opUnit3 u33(a33, b33, c33, r33, global_opcode);
    opUnit3 u34(a34, b34, c34, r34, global_opcode);

	always@(*) begin
		casex(global_opcode)
			3'b1xx: resx = {64'd0, r27, r26, r25, r24, r23, r22, r21, r20};
			3'b001: resx = {r115, r114, r113, r112, r111, r110, r19, r18, r17, r16, r15, r14, r13, r12, r11, r10};
			3'b010: resx = {88'd0, r34, r33, r32, r31, r30};
			default: resx = 128'd0;
		endcase
	end
endmodule