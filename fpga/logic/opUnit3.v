module opUnit3 (
    input  wire [7:0] ax,
    input wire [7:0] bx,
    input wire [7:0] cx,
    output reg  [7:0] resultx,
    input  wire [2:0] opcode
);
    wire [7:0] fma_result;
    fp8_fma u0(ax, bx, cx, fma_result);

    always @(*) begin
        case (opcode)
            3'b001:  resultx = fma_result;
            default: resultx = 8'd0;
        endcase
    end
endmodule