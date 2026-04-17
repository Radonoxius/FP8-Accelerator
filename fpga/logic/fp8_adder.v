module fp8_adder(
    input [7:0] A,
    input [7:0] B,
    output [7:0] P
);

    wire a_pos = ~A[7];
    wire b_pos = ~B[7];
    wire [3:0] a_exp = A[6:3];
    wire [2:0] a_mant = A[2:0];
    wire [3:0] b_exp = B[6:3];
    wire [2:0] b_mant = B[2:0];

    wire flag_nan_a = (A[6:0] == 7'h7F);
    wire flag_nan_b = (B[6:0] == 7'h7F);
    wire flag_z_a   = (A[6:0] == 7'h00);
    wire flag_z_b   = (B[6:0] == 7'h00);

    wire [3:0] a_frac = (a_exp == 0) ? {1'b0, a_mant} : {1'b1, a_mant};
    wire [3:0] b_frac = (b_exp == 0) ? {1'b0, b_mant} : {1'b1, b_mant};

    wire signed [6:0] s_exp_a = (a_exp == 0) ? -7'sd6 : ($signed({3'b0, a_exp}) - 7'sd7);
    wire signed [6:0] s_exp_b = (b_exp == 0) ? -7'sd6 : ($signed({3'b0, b_exp}) - 7'sd7);

    wire [7:0] a_val = {1'b0, a_frac, 3'b000};
    wire [7:0] b_val = {1'b0, b_frac, 3'b000};

    reg signed [6:0] result_exp;
    reg result_sign;
    reg [7:0] sum;
    reg [6:0] shift;

    reg guard;
    reg [1:0] sticky;
    reg [4:0] truncated;
    reg round_up;
    reg [5:0] abs_mant;

    always @(*) begin

        result_sign = 0; sum = 0; result_exp = 0; shift = 0;

        if (s_exp_a > s_exp_b) begin
            shift = s_exp_a - s_exp_b;
            result_exp = s_exp_a;
            if (a_pos && b_pos) begin
                result_sign = 0; sum = a_val + (b_val >> shift);
            end else if (a_pos && !b_pos) begin
                result_sign = 0; sum = a_val - (b_val >> shift);
            end else if (!a_pos && b_pos) begin
                result_sign = 1; sum = a_val - (b_val >> shift);
            end else begin
                result_sign = 1; sum = a_val + (b_val >> shift);
            end
        end 
        else if (s_exp_a == s_exp_b) begin
            result_exp = s_exp_a;
            if (a_val >= b_val) begin
                if (a_pos && b_pos) begin
                    result_sign = 0; sum = a_val + b_val;
                end else if (a_pos && !b_pos) begin
                    result_sign = 0; sum = a_val - b_val;
                end else if (!a_pos && b_pos) begin
                    result_sign = 1; sum = a_val - b_val;
                end else begin
                    result_sign = 1; sum = a_val + b_val;
                end
            end else begin
                if (b_pos && a_pos) begin
                    result_sign = 0; sum = a_val + b_val;
                end else if (b_pos && !a_pos) begin
                    result_sign = 0; sum = b_val - a_val;
                end else if (!b_pos && a_pos) begin
                    result_sign = 1; sum = b_val - a_val;
                end else begin
                    result_sign = 1; sum = a_val + b_val;
                end
            end
        end 
        else begin
            shift = s_exp_b - s_exp_a;
            result_exp = s_exp_b;
            if (b_pos && a_pos) begin
                result_sign = 0; sum = b_val + (a_val >> shift);
            end else if (b_pos && !a_pos) begin
                result_sign = 0; sum = b_val - (a_val >> shift);
            end else if (!b_pos && a_pos) begin
                result_sign = 1; sum = b_val - (a_val >> shift);
            end else begin
                result_sign = 1; sum = b_val + (a_val >> shift);
            end
        end

        guard = sum[2];
        sticky = sum[1:0];
        truncated = sum[7:3];

        round_up = (guard == 1'b1) && ((sticky != 0) || (truncated[0] == 1'b1));
        abs_mant = truncated + (round_up ? 1 : 0);

        if (abs_mant >= 16) begin
            abs_mant = abs_mant >> 1;
            result_exp = result_exp + 1;
        end

        if (abs_mant == 0) begin

            result_exp = -7'sd6; 
        end else begin

            if (abs_mant < 8 && result_exp > -6) begin abs_mant = abs_mant << 1; result_exp = result_exp - 1; end
            if (abs_mant < 8 && result_exp > -6) begin abs_mant = abs_mant << 1; result_exp = result_exp - 1; end
            if (abs_mant < 8 && result_exp > -6) begin abs_mant = abs_mant << 1; result_exp = result_exp - 1; end
        end
    end


    wire [3:0] exp_bits = $unsigned(result_exp + 7);
    wire [2:0] mant_bits = abs_mant[2:0];

    assign P = (flag_nan_a && flag_nan_b) ? 8'h7F :                 
               (flag_nan_a) ? A :                                      
               (flag_nan_b) ? B :                                   
               (flag_z_a && flag_z_b) ? 8'h00 :                        
               (flag_z_a) ? B :                                     
               (flag_z_b) ? A :                                 
               (sum == 0) ? 8'h00 :                                   
               (result_exp > 8) ? 8'h7F :                             
               (result_exp < -6) ? 8'h00 :                          
               (result_exp == -6 && abs_mant < 8) ? {result_sign, 4'd0, mant_bits} :    
               (exp_bits == 15 && mant_bits == 3'd7) ? {result_sign, 4'd15, 3'd6} : 
               {result_sign, exp_bits, mant_bits};             

endmodule
