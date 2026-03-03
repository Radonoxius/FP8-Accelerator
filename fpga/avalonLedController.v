module avalonLedController (
    input wire a_clk,
    input wire reset_n,
    input wire write,
    input wire [7:0] writedata,

    output reg led_line
);
    always @(posedge a_clk or negedge reset_n) begin
        if(!reset_n)
            led_line <= 0;
        
        if(write)
            led_line <= writedata[0];
    end
endmodule
