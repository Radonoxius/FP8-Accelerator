`timescale 1ns/1ps

module fp8_sub_tb;

    reg [7:0] A, B;
    wire [7:0] P;
    
    
    fp8_sub dut (
        .A(A),
        .B(B),
        .P(P)
    );



    integer i, j, total, errors, fd;

    initial begin
        total = 0;
        errors = 0;

        fd = $fopen("fpga_sub.csv", "w");
        if (fd == 0) begin
            $display("Error: Could not open fpga_add.csv for writing.");
            $finish;
        end

 
        
        $display("Starting FP8 Adder Exhaustive Test...");
        
        for (i = 0; i < 256; i = i + 1) begin
            for (j = 0; j < 256; j = j + 1) begin
                A = i[7:0];
                B = j[7:0];
                #5; 
                $fwrite(fd, "%b,%b,%b\n", A, B, P);
                if (P !== fp8_ref(A, B)) begin
                    errors = errors + 1;
                end
                total = total + 1;
            end
        end
        
        $fclose(fd);
        $display("CSV file generated: fpga_add.csv");
        $finish;
    end

endmodule