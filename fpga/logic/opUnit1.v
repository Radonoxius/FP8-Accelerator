module opUnit1 (
    input  wire [7:0] ax,
    output reg  [7:0] resultx,
    input  wire [2:0] opcode
);
    wire [7:0] idiv_result;
    fp8_idiv u0(ax, idiv_result);

    always @(*) begin
        case (opcode)
            3'b001:  resultx = idiv_result;
            default: resultx = 8'd0;
        endcase
    end
endmodule