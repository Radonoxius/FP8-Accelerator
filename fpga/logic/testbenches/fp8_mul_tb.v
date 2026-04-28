`timescale 1ns/1ps

module fp8_mul_tb;

    reg [7:0] A, B;
    wire [7:0] P;
    
    fp8_mul dut (.A(A), .B(B), .P(P));
	 
    integer ai, bi,fd;

    initial begin
        fd = $fopen("fpga_mul.csv", "w");
        for (ai = 0; ai < 256; ai = ai + 1) begin
            for (bi = 0; bi < 256; bi = bi + 1) begin
                A = ai[7:0]; B = bi[7:0];
                #1;
                $fwrite(fd, "%b,%b,%b\n", A, B, P);
               
            end
        end
        $fclose(fd);
        $display("Done.");
        $finish;
    end
endmodule