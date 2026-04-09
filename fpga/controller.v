module controller (
    input  wire         a_clk,
    input  wire         reset_n,
    input  wire         write,
    input  wire [127:0] writedata,
    input  wire         read,
    output wire [127:0] readdata
);
    reg [127:0] waste;

    assign readdata = waste;

    always @(posedge a_clk or negedge reset_n) begin
        if (!reset_n)
            waste <= 128'd0;
				
        else if (write)
            waste <= writedata << 1;
    end
endmodule