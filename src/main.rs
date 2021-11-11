
extern crate num_complex;

use num_complex::Complex;
use std::io;

fn main()
{
//単純な複素数生成
	let z :Complex<f64> = Complex::new(1.0, 2.0);
	println!("単純な複素数生成");
	println!("  通常出力：{ }", z);
	println!("  デバッグ出力：{:?}", z);

//複素演算
	let z1 = Complex::new(1.0, 2.0);
	let z2 = Complex::new(3.0, 4.0);

	println!("\n複素演算");
	println!("  add { }", z1 + z2);
	println!("  deff { }", z1 - z2);
	println!("  product { }", z1 * z2);
	println!("  div { }", z1 / z2);
	println!("  norm { }, { }", z1.norm(), z2.norm());
	println!("  arg { }, { }", z1.arg(), z2.arg());

//複素数Vec
	let mut n = String::new();
// console input
	print!("input number");
	io::stdin().read_line(&mut n).expect("Failed to read line");
	let n :u16 = match n.trim().parse()
					{
						Ok(num) 	=> num,
						Err(_) 	=> {
											println!("console input is illegal");
											std::process::exit(-1);
										},
					};

    let mut z  = Vec::with_capacity(n.into());

    for i in 0..n
    {
    	z.push(Complex::new(1.0*i as f64, 2.0*i as f64));
    }

    for (i, z) in z.iter().enumerate()
	{
		println!("  { } : { }", i, z);
    }


}
