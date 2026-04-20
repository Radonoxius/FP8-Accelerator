module fp8_idiv(
	input [7:0]A,
	output [7:0]P
);

	fp8_div div_1(8'h38, A, P);
endmodule	