module controller (
    input  wire         a_clk,
    input  wire         reset_n,
    input  wire         write,
    input  wire [127:0] writedata,
    input  wire         read,
    output wire [127:0] readdata
);
    reg [127:0] storage;
	 reg [127:0] results;
	 vecUnit u(storage, results[63:0]);

    assign readdata = results;

    always @(posedge a_clk or negedge reset_n) begin
        if (!reset_n) begin
            storage <= 128'd0;
				results <= 128'd0;
		  end
				
        else if (write)
            storage <= writedata;
    end
endmodule