module controller (
    input  wire         a_clk,
    input  wire         reset_n,
    input  wire         write,
    input  wire [127:0] writedata,
    input  wire         read,
    output reg  [127:0] readdata // Changed back to 'reg' for a registered output
);
    reg [127:0] storage;
    wire [63:0] vec_results; // Combinational output from the vector unit

    // The vector unit calculates continuously based on 'storage'
    vecUnit u(storage, vec_results);

    always @(posedge a_clk or negedge reset_n) begin
        if (!reset_n) begin
            storage  <= 128'd0;
            readdata <= 128'd0;
        end
        else begin
            // Handle Writes
            if (write) begin
                storage <= writedata;
            end
            
            // Handle Reads: Register the combinational math output
            // This isolates the bus timing from the math timing
            if (read) begin
                readdata <= {64'd0, vec_results};
            end
        end
    end
endmodule
