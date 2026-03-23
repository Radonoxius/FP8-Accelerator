use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use soft_fp8::Fp8;
use soft_fp8::state::State;

fn bencher(
    a: u8,
    b: u8,
    i_res: u8,
    f: fn(f32, f32) -> f32,
    fail_counter95: &mut u32,
    fail_counter90: &mut u32,
    fail_counter85: &mut u32,
    fail_counter75: &mut u32
) {
    let a = Into::<f32>::into(Fp8::from(a));
    let b = Into::<f32>::into(Fp8::from(b));
    let fpga_r = Into::<f32>::into(Fp8::from(i_res));

    let r_state = State::get(&Fp8::from(i_res));

    let fpu_r = f(a, b);

    let e95 = 5.0 * fpu_r / 100.0;
    let e90 = 10.0 * fpu_r / 100.0;
    let e85 = 15.0 * fpu_r / 100.0;
    let e75 = 25.0 * fpu_r / 100.0;

    let in_range = |err: f32| -> bool {
        f32::abs(fpu_r) - f32::abs(err) < f32::abs(fpga_r) &&
        f32::abs(fpga_r) < f32::abs(fpu_r) + f32::abs(err)
    };

    if !in_range(e95) && r_state != State::NaN {
        *fail_counter95 += 1;
    }
    if !in_range(e90) && r_state != State::NaN {
        *fail_counter90 += 1;
    }
    if !in_range(e85) && r_state != State::NaN {
        *fail_counter85 += 1;
    }
    if !in_range(e75) && r_state != State::NaN {
        *fail_counter75 += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: cargo accu-check <mode> <operator>\n\n \
            <mode> is either soft or fpga\n \
            <operator> is add or sub or mul or div\n\n\
            The arguments MUST STRICTLY follow this order!"
        );
        std::process::exit(1);
    }

    let file = File::open(
        format!("dumps/{}_{}.csv", &args[1], &args[2])
    ).unwrap_or_else(|_| {
        if &args[2] != "add" && &args[2] != "sub" &&
            &args[2] != "mul" && &args[2] != "div" {
            eprintln!(
                "Unsupported operator: `{}`!",
                &args[2]
            )
        } else {
            eprintln!(
                "Make sure that the corresponding csv file exists!"
            );
        }
        std::process::exit(1);
    });

    let mut fail_counter95: u32 = 0;
    let mut fail_counter90: u32 = 0;
    let mut fail_counter85: u32 = 0;
    let mut fail_counter75: u32 = 0;

    for (line_num, line) in
        BufReader::new(file).lines().enumerate() {
        let line = line.unwrap_or_else(|e| {
            eprintln!("Error reading line {}: {}", line_num + 1, e);
            std::process::exit(1);
        });

        if line.trim().is_empty() {
            continue;
        }

        let numbers: Vec<u8> = line
            .split(',')
            .map(|s| u8::from_str_radix(s.trim(), 2)
            .unwrap_or_else(|e| {
                eprintln!("Invalid binary number '{}' on line {}: {}", s.trim(), line_num + 1, e);
                std::process::exit(1);
            }))
            .collect();

        if args[2] == "add" {
            bencher(
                numbers[0],
                numbers[1],
                numbers[2],
                |a_f, b_f| a_f + b_f,
                &mut fail_counter95,
                &mut fail_counter90,
                &mut fail_counter85,
                &mut fail_counter75
            );
        } else if args[2] == "sub" {
            bencher(
                numbers[0],
                numbers[1],
                numbers[2],
                |a_f, b_f| a_f - b_f,
                &mut fail_counter95,
                &mut fail_counter90,
                &mut fail_counter85,
                &mut fail_counter75
            );
        } else if args[2] == "mul" {
            bencher(
                numbers[0],
                numbers[1],
                numbers[2],
                |a_f, b_f| a_f * b_f,
                &mut fail_counter95,
                &mut fail_counter90,
                &mut fail_counter85,
                &mut fail_counter75
            );
        } else if args[2] == "div" {
            bencher(
                numbers[0],
                numbers[1],
                numbers[2],
                |a_f, b_f| a_f / b_f,
                &mut fail_counter95,
                &mut fail_counter90,
                &mut fail_counter85,
                &mut fail_counter75
            );
        } else {
            panic!("Unsupported operator: {}", &args[2]);
        }
    }

    println!(
        "The test results exclude cases where NaN is produced as the result."
    );
    println!(
        "Its assumed that NaN cases are handled correctly.\n\n-----"
    );

    println!("Test results for the {} operation (mode = {}):\n", &args[2], &args[1]);
    println!(
        "Atleast 95% Accuracy: {:.2}% test cases.",
        (65536.0 - fail_counter95 as f32) / 655.360
    );
    println!(
        "Atleast 90% Accuracy: {:.2}% test cases.",
        (65536.0 - fail_counter90 as f32) / 655.360
    );
    println!(
        "Atleast 85% Accuracy: {:.2}% test cases.",
        (65536.0 - fail_counter85 as f32) / 655.360
    );
    println!(
        "Atleast 75% Accuracy: {:.2}% test cases.",
        (65536.0 - fail_counter75 as f32) / 655.360
    );
    println!("-----")
}