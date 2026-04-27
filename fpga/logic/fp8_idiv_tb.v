`timescale 1ns / 1ps

module fp8_idiv_tb();

    reg [7:0] A;
    wire [7:0] P;

    integer file_ptr;
    integer i;


    fp8_idiv uut (
        .A(A), 
        .P(P)
    );

    initial begin
   
        file_ptr = $fopen("fpga_idiv.csv", "w");
        
        if (file_ptr == 0) begin
            $display("Error: Could not open file for writing.");
            $finish;
        end

        $display("Starting Exhaustive Test for fp8_idiv...");

   
        for (i = 0; i < 256; i = i + 1) begin
            A = i;
            
            #5; /
            
        
            $fwrite(file_ptr, "%b,%b\n", A, P);
        end

        $fclose(file_ptr);
        $display("Done. 256 cases saved to 'idiv_results.csv'.");
        $finish;
    end
      
endmodule
