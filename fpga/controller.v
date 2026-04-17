module controller (
    input  wire         a_clk,
    input  wire         reset_n,
    input  wire         write,
    input  wire [127:0] writedata,
    input  wire         read,
    output reg  [127:0] readdata,
    output reg          waitrequest
);

    reg [127:0] storage;
    wire [63:0] vec_results;

    vecUnit u(storage, vec_results);

    // Tracks whether the current bus transaction has already been serviced.
    // Needed because the Avalon master holds write/read HIGH for one extra cycle
    // after it observes waitrequest LOW, and we must not double-capture it.
    reg serviced;

    always @(posedge a_clk or negedge reset_n) begin
        if (!reset_n) begin
            waitrequest <= 1'b1;
            serviced    <= 1'b0;
            storage     <= 128'd0;
            readdata    <= 128'd0;

        end else begin

            // ----------------------------------------------------------------
            // WRITE path
            // Avalon handshake: waitrequest HIGH stalls the master.
            // We drop it LOW for exactly one registered cycle, which makes the
            // transaction visible to the master on the *next* rising edge.
            // 'serviced' prevents re-capturing while the master is still
            // deasserting its write strobe.
            // ----------------------------------------------------------------
            if (write && !serviced) begin
                storage     <= writedata;   // latch payload into the datapath
                waitrequest <= 1'b0;        // signal: ready to accept this beat
                serviced    <= 1'b1;        // mark transaction as done

            // ----------------------------------------------------------------
            // READ path
            // vec_results is combinational from storage, so it is already
            // stable.  We register it into readdata and drop waitrequest
            // together; both are valid to the master on the next edge.
            // ----------------------------------------------------------------
            end else if (read && !serviced) begin
                readdata    <= {64'd0, vec_results}; // widen 64-bit result to bus width
                waitrequest <= 1'b0;
                serviced    <= 1'b1;

            // ----------------------------------------------------------------
            // IDLE — no active transaction.
            // Re-arm for the next transaction and keep the bus stalled so a
            // new request will always see at least one wait cycle.
            // ----------------------------------------------------------------
            end else if (!write && !read) begin
                waitrequest <= 1'b1;
                serviced    <= 1'b0;

            // ----------------------------------------------------------------
            // HOLD — master is still asserting write/read the cycle after we
            // completed (it hasn't de-asserted yet per Avalon timing).
            // Simply stall until it releases.
            // ----------------------------------------------------------------
            end else begin
                waitrequest <= 1'b1;
            end

        end
    end

endmodule
