use std::fmt;
use std::vec::Vec;
use std::fs::File;
use std::io::Write;
use std::mem;

fn SolveIBVP(X : f64, N : u32, C : f64, T : f64) -> Vec<f64> {

    let v = vec![0 as f64; N as usize];
    let (mut u1, mut u2) = (& mut vec![0 as f64; N as usize],
                            & mut vec![0 as f64; N as usize]);
    let tau = C * X / N as f64;
    u1[0] = 1 as f64;                  // left bc

    let mut n : usize = 0;
    while n as f64 * tau < T {
        u2[0 as usize] = 1 as f64;
        u2[(N - 1) as usize] = 0f64;

        println!("n = {}", n);

        for i in 1..N - 1 {
            let j = i as usize;
            u2[j] = u1[j] - C/2.0 * (u1[j + 1] - u1[j - 1])
				+ C*C/2.0 * (u1[j + 1] - 2.0 * u1[j]
					   + u1[j - 1]);
        }

        mem::swap(u2, u1);
        n += 1;
    }

    u2.clone()
}

fn main() {
    let base_name = "hw3_".to_string();
    let (N, X, T) = (100, 10, 5);
    let h : f64 = X as f64 / N as f64;
    let Cvals = vec![0.9, 1 as f64, 1.1];

    for C in &Cvals {
        let file_name = format!("{}_{}", base_name, C);
        let mut file = match File::create(file_name) {
            Ok(f) => f,
            Err(e) => return (),
        };
        let v = SolveIBVP(X as f64, N, *C as f64, T as f64);
        for n in 0..v.len() {
            writeln!(&mut file, "{}\t{}", n as f64 * h, v[n]);
        }
    }
}
