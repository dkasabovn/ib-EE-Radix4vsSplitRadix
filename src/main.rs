use rustfft::{Fft, FftPlanner, FftDirection, num_complex::Complex, algorithm::MixedRadix, algorithm::Radix4};

use std::time::{UNIX_EPOCH, SystemTime};

fn main() {
    let vec = vec![64, 256, 1024, 65536];
    println!("Mixed Radix");
    for test_case in vec.iter() {
        println!("{0}: {1}", test_case, test_mixed_radix(1000, *test_case as usize));
    }
    println!("Radix-4");
    for test_case in vec.iter() {
        println!("{0}: {1}", test_case, test_radix_four(1000, *test_case as usize));
    }
}

fn test_mixed_radix(n: usize, case: usize) -> f64 {
    let mut sum : f64 = 0_f64;
    for _i in 0..n {
        let now : u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        mixed_radix(case);
        let end : u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let difference = end - now;
        sum += difference as f64
    }
    return sum / n as f64;
}

fn test_radix_four(n: usize, case: usize) -> f64 {
    let mut sum : f64 = 0_f64;
    for _i in 0..n {
        let now : u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        radix_four(case);
        let end : u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let difference = end - now;
        sum += difference as f64
    }
    return sum / n as f64;
}

fn mixed_radix(n: usize) {
    let mut buffer = vec![Complex{ re: 0.0f32, im: 0.0f32 }; n];
    let width = (n as f64).sqrt() as usize;
    let mut planner = FftPlanner::new();
    let inner_fft_n1 = planner.plan_fft_forward(width);
    let inner_fft_n2 = planner.plan_fft_forward(width);
    let fft = MixedRadix::new(inner_fft_n1, inner_fft_n2);
    fft.process(&mut buffer);
}

fn radix_four(n:  usize) {
    let mut buffer = vec![Complex{re: 0.0f32, im: 0.0f32}; n];
    let fft = Radix4::<f32>::new(n, FftDirection::Forward);
    fft.process(&mut buffer);
}
