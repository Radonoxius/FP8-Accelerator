module opUnit2 (
    input  wire [7:0] ax,
    input  wire [7:0] bx,
    output reg  [7:0] resultx,
    input  wire [2:0] opcode
);
    wire [7:0] add_result;
    fp8_adder u1(ax, bx, add_result);
	 
	wire [7:0] sub_result;
	fp8_sub u2(ax, bx, sub_result);
	 
	wire [7:0] mul_result;
	fp8_mul u3(ax, bx, mul_result);
	 
	wire [7:0] div_result;
	fp8_div u4(ax, bx, div_result);

    always @(*) begin
        case (opcode)
            3'b100:  resultx = add_result;
            3'b101:  resultx = sub_result;
            3'b110:  resultx = mul_result;
            3'b111:  resultx = div_result;
            default: resultx = 8'd0;
        endcase
    end

endmodule