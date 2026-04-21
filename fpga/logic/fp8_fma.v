module fp8_fma(
	input [7:0]A,
	input [7:0]B,
	input [7:0]C,
	output [7:0]P
);
	wire [7:0]P1;
	fp8_mul M1 (A, B, P1);
	fp8_adder A1 (C, P1, P);
endmodule