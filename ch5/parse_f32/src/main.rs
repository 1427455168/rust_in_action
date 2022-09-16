const BIAS: i32 = 127;
const RADIX: f32 = 2.0;


fn main() {
    let n = 42.42_f32;

    let (sign, exponent, fraction) = to_parts(n);
    let (sign_, exponent_, fraction_) = decode(sign, exponent, fraction);

    let n_ = from_parts(sign_, exponent_, fraction_);

    println!("{} -> {}", n, n_);
    println!("sign: {:01b} -> {}", sign, sign_);
    println!("exponent: {:08b} -> {}", exponent, exponent_);
    println!("fraction: {:023b} -> {}", fraction, fraction_);
}


fn to_parts(n: f32) -> (u32, u32, u32) {
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let exponent = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign, exponent, fraction)
}

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
    let sign = (-1.0_f32).powf(sign as f32);

    let exponent = (exponent as i32) - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut num = 1.0_f32;
    for i in 0..23 {
        let mask = 1 << i;
        let one_ar_bit_i = fraction & mask;

        if one_ar_bit_i != 0 {
            let weight = RADIX.powf(i as f32 - 23.0);
            num += weight;
        }
    }

    (sign, exponent, num)
}

fn from_parts(sign: f32, exponent: f32, fraction: f32) -> f32 {
    sign * exponent * fraction
}






