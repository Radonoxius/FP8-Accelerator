module fp8_div(
    input [7:0] A,
    input [7:0] B,
    output [7:0] P
);
    // pull out exp and mantissa
    wire [3:0] ea = A[6:3];
    wire [2:0] ma = A[2:0];
    wire [3:0] eb = B[6:3];
    wire [2:0] mb = B[2:0];

    // XOR signs for division
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

    // shift A mantissa left by 10 bits before dividing to keep precision
    wire [13:0] dvd = {fa, 10'b0};
    wire [13:0] raw_q = dvd / fb;
    wire [13:0] rem = dvd % fb;

    reg signed [6:0] res_exp;
    reg [13:0] raw_mant;
    reg lost;
    reg gb;
    reg st;
    reg [4:0] trunc;
    reg do_round;
    reg [4:0] mant_out;

    always @(*) begin
        // subtract exponents, -4 to account for the shift we did above
        res_exp = unbiased_a - unbiased_b - 4;
        raw_mant = raw_q;
        lost = 0;

        // normalize by checking how big the quotient is and shifting accordingly
        if (raw_mant >= 8192) begin
            lost = |raw_mant[6:0];
            raw_mant = raw_mant >> 7;
            res_exp = res_exp + 7;
        end else if (raw_mant >= 4096) begin
            lost = |raw_mant[5:0];
            raw_mant = raw_mant >> 6;
            res_exp = res_exp + 6;
        end else if (raw_mant >= 2048) begin
            lost = |raw_mant[4:0];
            raw_mant = raw_mant >> 5;
            res_exp = res_exp + 5;
        end else if (raw_mant >= 1024) begin
            lost = |raw_mant[3:0];
            raw_mant = raw_mant >> 4;
            res_exp = res_exp + 4;
        end else if (raw_mant >= 512) begin
            lost = |raw_mant[2:0];
            raw_mant = raw_mant >> 3;
            res_exp = res_exp + 3;
        end else if (raw_mant >= 256) begin
            lost = |raw_mant[1:0];
            raw_mant = raw_mant >> 2;
            res_exp = res_exp + 2;
        end else if (raw_mant >= 128) begin
            lost = raw_mant[0];
            raw_mant = raw_mant >> 1;
            res_exp = res_exp + 1;
        end

        // handle subnormal results by shifting right until exp reaches -6
        repeat(14) begin
            if (res_exp < -6 && raw_mant > 0) begin
                lost = lost | raw_mant[0];
                raw_mant = raw_mant >> 1;
                res_exp = res_exp + 1;
            end
        end

        // grab rounding bits, include remainder in sticky
        gb = raw_mant[2];
        st = (raw_mant[1:0] != 2'b00) | lost | (rem != 0);
        trunc = raw_mant[7:3];

        // round to nearest even
        do_round = (gb == 1'b1) && (st || trunc[0]);
        mant_out = trunc + (do_round ? 1 : 0);

        // rounding overflow
        if (mant_out >= 16) begin
            mant_out = mant_out >> 1;
            res_exp = res_exp + 1;
        end
    end

    // re-bias exponent
    wire [3:0] final_exp = $unsigned(res_exp + 7);

    // special cases - div by zero gives NaN, 0/0 gives NaN
    assign P = (is_nan_a && is_nan_b) ? 8'h7F :
               (is_nan_a || is_nan_b) ? {res_sign, 7'h7F} :
               (is_zero_a && is_zero_b) ? 8'h7F :
               (is_zero_b) ? {res_sign, 7'h7F} :
               (is_zero_a) ? {res_sign, 7'h00} :
               (res_exp > 8) ? 8'h7F :
               (res_exp < -6) ? {res_sign, 7'h00} :
               (res_exp == -6 && mant_out < 8) ? {res_sign, 4'd0, mant_out[2:0]} :
               (final_exp == 15 && mant_out[2:0] == 3'd7) ? {res_sign, 4'd15, 3'd6} :
               {res_sign, final_exp, mant_out[2:0]};

endmodule