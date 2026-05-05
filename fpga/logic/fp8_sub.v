module fp8_sub(
input [7:0] A,
input [7:0] B,
output [7:0] P
);
// check if A is positive and flip B's sign for subtraction
wire a_pos = ~A[7];     
wire b_pos = B[7]; // B's sign is flipped since we're doing A - B

// pull out exp and mantissa from both inputs
wire [3:0] ea = A[6:3];
wire [2:0] ma = A[2:0];
wire [3:0] eb = B[6:3];
wire [2:0] mb = B[2:0];

// NaN is 0x7F in E4M3, zero is when lower 7 bits are 0
wire is_nan_a = (A[6:0] == 7'h7F);
wire is_nan_b = (B[6:0] == 7'h7F);
wire is_zero_a = (A[6:0] == 7'h00);
wire is_zero_b = (B[6:0] == 7'h00);

// hidden bit - subnormals dont have the leading 1
wire [3:0] fa = (ea == 0) ? {1'b0, ma} : {1'b1, ma};
wire [3:0] fb = (eb == 0) ? {1'b0, mb} : {1'b1, mb};

// unbiased exponents, subnormals are fixed at -6
wire signed [6:0] unbiased_a = (ea == 0) ? -7'sd6 : ($signed({3'b0, ea}) - 7'sd7);
wire signed [6:0] unbiased_b = (eb == 0) ? -7'sd6 : ($signed({3'b0, eb}) - 7'sd7);

// shift mantissas left by 3 to give room for guard/sticky bits later
wire [7:0] av = {1'b0, fa, 3'b000};
wire [7:0] bv = {1'b0, fb, 3'b000};

reg signed [6:0] res_exp;
reg res_sign;
reg [7:0] raw_sum;
reg [6:0] sh; // how much to shift the smaller one

// rounding bits - guard is the first bit we cut off, sticky is OR of the rest
reg gb; // guard bit
reg [1:0] st; // sticky bits
reg [4:0] trunc; // the bits we actually keep before rounding
reg do_round;
reg [5:0] mant_out;

always @(*) begin

    res_sign = 0; raw_sum = 0; res_exp = 0; sh = 0;

    // figure out which exponent is bigger and align the smaller mantissa
    if (unbiased_a > unbiased_b) begin
        sh = unbiased_a - unbiased_b;
        res_exp = unbiased_a;
        // now add or subtract depending on signs of both inputs
        if (a_pos && b_pos) begin
            res_sign = 0; raw_sum = av + (bv >> sh);
        end else if (a_pos && !b_pos) begin
            res_sign = 0; raw_sum = av - (bv >> sh);
        end else if (!a_pos && b_pos) begin
            res_sign = 1; raw_sum = av - (bv >> sh);
        end else begin
            res_sign = 1; raw_sum = av + (bv >> sh);
        end
    end 
    else if (unbiased_a == unbiased_b) begin
        res_exp = unbiased_a;
        // same exponent, check which mantissa is bigger to avoid negative raw_sum
        if (av >= bv) begin
            if (a_pos && b_pos) begin
                res_sign = 0; raw_sum = av + bv;
            end else if (a_pos && !b_pos) begin
                res_sign = 0; raw_sum = av - bv;
            end else if (!a_pos && b_pos) begin
                res_sign = 1; raw_sum = av - bv;
            end else begin
                res_sign = 1; raw_sum = av + bv;
            end
        end else begin
            // B is bigger so swap the order
            if (b_pos && a_pos) begin
                res_sign = 0; raw_sum = av + bv;
            end else if (b_pos && !a_pos) begin
                res_sign = 0; raw_sum = bv - av;
            end else if (!b_pos && a_pos) begin
                res_sign = 1; raw_sum = bv - av;
            end else begin
                res_sign = 1; raw_sum = av + bv;
            end
        end
    end 
    else begin
        // B has bigger exponent, shift A instead
        sh = unbiased_b - unbiased_a;
        res_exp = unbiased_b;
        if (b_pos && a_pos) begin
            res_sign = 0; raw_sum = bv + (av >> sh);
        end else if (b_pos && !a_pos) begin
            res_sign = 0; raw_sum = bv - (av >> sh);
        end else if (!b_pos && a_pos) begin
            res_sign = 1; raw_sum = bv - (av >> sh);
        end else begin
            res_sign = 1; raw_sum = bv + (av >> sh);
        end
    end
    
    // grab the bits we need for rounding
    gb = raw_sum[2];
    st = raw_sum[1:0];
    trunc = raw_sum[7:3];

    // round up only if guard is 1 AND (sticky != 0 OR last kept bit is 1)
    // this is the round to nearest even rule
    do_round = (gb == 1'b1) && ((st != 0) || (trunc[0] == 1'b1));
    mant_out = trunc + (do_round ? 1 : 0);

    // if we overflowed into bit 4, shift right and bump exp
    if (mant_out >= 16) begin
        mant_out = mant_out >> 1;
        res_exp = res_exp + 1;
    end

    // handle zero and subnormal cases
    if (mant_out == 0) begin
        res_exp = -7'sd6; // treat as subnormal zero
    end else begin
        // normalize subnormals by shifting left until leading 1 appears
        if (mant_out < 8 && res_exp > -6) begin mant_out = mant_out << 1; res_exp = res_exp - 1; end
        if (mant_out < 8 && res_exp > -6) begin mant_out = mant_out << 1; res_exp = res_exp - 1; end
        if (mant_out < 8 && res_exp > -6) begin mant_out = mant_out << 1; res_exp = res_exp - 1; end
    end
end

// re-bias the exponent back for E4M3 (add 7)
wire [3:0] final_exp = $unsigned(res_exp + 7);
wire [2:0] final_mant = mant_out[2:0];

// output logic - handle all the special cases first before normal output
assign P = (is_nan_a && is_nan_b) ? 8'h7F :          // both NaN -> NaN          
           (is_nan_a) ? A :                            // NaN propagates
           (is_nan_b) ? {~B[7], B[6:0]} :             // B is NaN, flip sign since subtraction                      
           (is_zero_a && is_zero_b) ? 8'h00 :         // 0 - 0 = 0
           (is_zero_a) ? {~B[7], B[6:0]} :            // 0 - B = -B                    
           (is_zero_b) ? A :                           // A - 0 = A         
           (raw_sum == 0) ? 8'h00 :                    // result cancelled out
           (res_exp > 8) ? 8'h7F :                     // overflow -> max value (no inf in E4M3)
           (res_exp < -6) ? 8'h00 :                    // underflow -> zero         
           (res_exp == -6 && mant_out < 8) ? {res_sign, 4'd0, final_mant} :  // subnormal
           (final_exp == 15 && final_mant == 3'd7) ? {res_sign, 4'd15, 3'd6} : // clamp below NaN
           {res_sign, final_exp, final_mant};          // normal result
endmodule