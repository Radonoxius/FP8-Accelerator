module fp8_div(
    input [7:0] A,
    input [7:0] B,
    output [7:0] P
);

    wire [3:0] a_exp = A[6:3];
    wire [2:0] a_mant = A[2:0];
    wire [3:0] b_exp = B[6:3];
    wire [2:0] b_mant = B[2:0];
    wire result_sign = A[7] ^ B[7];

    wire flag_nan_A = (A[6:0] == 7'h7F);
    wire flag_nan_B = (B[6:0] == 7'h7F);
    wire flag_z_A   = (A[6:0] == 7'h00);
    wire flag_z_B   = (B[6:0] == 7'h00);

    wire [3:0] a_mant_full = (a_exp == 0) ? {1'b0, a_mant} : {1'b1, a_mant};
    wire [3:0] b_mant_full = (b_exp == 0) ? {1'b0, b_mant} : {1'b1, b_mant};

    wire signed [6:0] a_eff_exp = (a_exp == 0) ? -7'sd6 : ($signed({3'b0, a_exp}) - 7'sd7);
    wire signed [6:0] b_eff_exp = (b_exp == 0) ? -7'sd6 : ($signed({3'b0, b_exp}) - 7'sd7);

    wire [13:0] dividend = {a_mant_full, 10'b0};
    wire [13:0] raw_quot = dividend / b_mant_full;
    wire [13:0] rem      = dividend % b_mant_full;

    reg signed [6:0] result_exp;
    reg [13:0] full_mant;
    reg lost_bit;
    reg guard;
    reg sticky;
    reg [4:0] truncated;
    reg round_up;
    reg [4:0] abs_mant;

    always @(*) begin
        result_exp = a_eff_exp - b_eff_exp - 4;
        full_mant = raw_quot;
        lost_bit = 0;
        if (full_mant >= 8192) begin         
            lost_bit = |full_mant[6:0];
            full_mant = full_mant >> 7;
            result_exp = result_exp + 7;
        end else if (full_mant >= 4096) begin 
            lost_bit = |full_mant[5:0];
            full_mant = full_mant >> 6;
            result_exp = result_exp + 6;
        end else if (full_mant >= 2048) begin 
            lost_bit = |full_mant[4:0];
            full_mant = full_mant >> 5;
            result_exp = result_exp + 5;
        end else if (full_mant >= 1024) begin
            lost_bit = |full_mant[3:0];
            full_mant = full_mant >> 4;
            result_exp = result_exp + 4;
        end else if (full_mant >= 512) begin  
            lost_bit = |full_mant[2:0];
            full_mant = full_mant >> 3;
            result_exp = result_exp + 3;
        end else if (full_mant >= 256) begin 
            lost_bit = |full_mant[1:0];
            full_mant = full_mant >> 2;
            result_exp = result_exp + 2;
        end else if (full_mant >= 128) begin  
            lost_bit = full_mant[0];
            full_mant = full_mant >> 1;
            result_exp = result_exp + 1;
        end

        repeat(14) begin
            if (result_exp < -6 && full_mant > 0) begin
                lost_bit = lost_bit | full_mant[0];
                full_mant = full_mant >> 1;
                result_exp = result_exp + 1;
            end
        end

        guard = full_mant[2];
        sticky = (full_mant[1:0] != 2'b00) | lost_bit | (rem != 0);
        truncated = full_mant[7:3];

        round_up = (guard == 1'b1) && (sticky || truncated[0]);
        abs_mant = truncated + (round_up ? 1 : 0);

        if (abs_mant >= 16) begin
            abs_mant = abs_mant >> 1;
            result_exp = result_exp + 1;
        end
    end

    wire [3:0] exp_bits = $unsigned(result_exp + 7);

    assign P = (flag_nan_A && flag_nan_B) ? 8'h7F :
					(flag_nan_A || flag_nan_B) ? {result_sign, 7'h7F} :                  
               (flag_z_A && flag_z_B)     ?  8'h7F :                 
               (flag_z_B)                 ? {result_sign, 7'h7F} :   
               (flag_z_A)                 ? {result_sign, 7'h00} : 
               (result_exp > 8)           ? 8'h7F :                  
               (result_exp < -6)          ? {result_sign, 7'h00} :   
               (result_exp == -6 && abs_mant < 8) ? {result_sign, 4'd0, abs_mant[2:0]} : 
               (exp_bits == 15 && abs_mant[2:0] == 3'd7) ? {result_sign, 4'd15, 3'd6} :  
               {result_sign, exp_bits, abs_mant[2:0]};                   

endmodule