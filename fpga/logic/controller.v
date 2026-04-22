module controller (
    input  wire         a_clk,
    input  wire         reset_n,
    input  wire         write,
    input  wire [127:0] writedata,
    input  wire [7:0]   address,
    input  wire         read,
    output reg  [127:0] readdata
);
    reg [127:0] in_storage;
    reg [127:0] opcode;

    wire [63:0] vec_results;
    reg  [63:0] vec_results_r;

    vecUnit2 u(in_storage, opcode, vec_results);

    always @(posedge a_clk or negedge reset_n) begin
        if (!reset_n) begin
            in_storage    <= 128'd0;
            opcode        <= 128'd0;
            readdata      <= 128'd0;
            vec_results_r <= 64'd0;
        end

        else begin
            vec_results_r <= vec_results;

            if (write && address == 8'h00)
                in_storage <= writedata;

            if (write && address == 8'h10)
                opcode <= { writedata[127:125], 125'd0 };

            if (read && address == 8'h20)
                readdata <= vec_results_r;
        end
    end
endmodule