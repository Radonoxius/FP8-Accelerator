module fp8_mul(
    input [7:0] A,
    input [7:0] B,
    output [7:0] P
);
    // pull out exp and mantissa
    wire [3:0] ea = A[6:3];
    wire [2:0] ma = A[2:0];
    wire [3:0] eb = B[6:3];
    wire [2:0] mb = B[2:0];

    // XOR signs for multiplication
    wire res_sign = A[7] ^ B[7];

    // NaN and zero checks
    wire is_nan_a = (A[6:0] == 7'h7F);
    wire is_nan_b = (B[6:0] == 7'h7F);
    wire is_zero_a = (A[6:0] == 7'h00);
    wire is_zero_b = (B[6:0] == 7'h00);

    // hidden bit for subnormals
    wire [3:0] fa = (ea == 0) ? {1'b0, ma} : {1'b1, ma};
    wire [3:0] fb = (eb == 0) ? {1'b0, mb} : {1'b1, mb};

    // unbiased exponents
    wire signed [6:0] unbiased_a = (ea == 0) ? -7'sd6 : ($signed({3'b0, ea}) - 7'sd7);
    wire signed [6:0] unbiased_b = (eb == 0) ? -7'sd6 : ($signed({3'b0, eb}) - 7'sd7);

    reg signed [6:0] res_exp;
    reg [7:0] raw_prod;
    reg [3:0] norm_count;

    reg gb;
    reg st;
    reg [4:0] trunc;
    reg do_round;
    reg [4:0] mant_out;

    always @(*) begin
        // add exponents and +1 because both mantissas have hidden bit
        res_exp = unbiased_a + unbiased_b + 1;
        raw_prod = fa * fb;

        // normalize - shift left until leading 1 is at bit 7
        if (raw_prod != 0) begin
            repeat(7) begin
                if (raw_prod[7] == 1'b0) begin
                    raw_prod = raw_prod << 1;
                    res_exp = res_exp - 1;
                end
            end
        end

        // grab rounding bits from the product
        gb = raw_prod[3];
        st = |raw_prod[2:0];
        trunc = raw_prod[7:4];

        // round to nearest even
        do_round = (gb == 1'b1) && (st || trunc[0]);
        mant_out = trunc + (do_round ? 1 : 0);

        // if rounding caused overflow shift right and bump exp
        if (mant_out >= 16) begin
            mant_out = mant_out >> 1;
            res_exp = res_exp + 1;
        end

        // normalize subnormals
        if (mant_out < 8 && res_exp > -6) begin
            mant_out = mant_out << 1;
            res_exp = res_exp - 1;
        end
        if (mant_out < 8 && res_exp > -6) begin
            mant_out = mant_out << 1;
            res_exp = res_exp - 1;
        end
        if (mant_out < 8 && res_exp > -6) begin
            mant_out = mant_out << 1;
            res_exp = res_exp - 1;
        end
    end

    // re-bias exponent
    wire [3:0] final_exp = $unsigned(res_exp + 7);
    wire [2:0] final_mant = mant_out[2:0];

    // special cases then normal output
    assign P = (is_nan_a || is_nan_b) ? {res_sign, 7'h7F} :
               (is_zero_a || is_zero_b) ? {res_sign, 7'h00} :
               (res_exp > 8) ? 8'h7F :
               (res_exp < -6) ? 8'h00 :
               (res_exp == -6 && mant_out < 8) ? {res_sign, 4'd0, final_mant} :
               (final_exp == 15 && final_mant == 3'd7) ? {res_sign, 4'd15, 3'd6} :
               {res_sign, final_exp, final_mant};

endmodule