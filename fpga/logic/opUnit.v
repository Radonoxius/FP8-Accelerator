module opUnit (
    input  wire [7:0] ax,
    input  wire [7:0] bx,
    output reg  [7:0] resultx,
    input  wire [2:0] opcode
);
    wire [7:0] add_result;
    fp8_add adder (ax, bx, add_result);

    always @(*) begin
        case (opcode)
            3'b100:  resultx = add_result;
            3'b101:  resultx = 8'd0;         // sub placeholder
            3'b110:  resultx = 8'd0;         // mul placeholder
            3'b111:  resultx = 8'd0;         // div placeholder
            default: resultx = 8'd0;
        endcase
    end

endmodule