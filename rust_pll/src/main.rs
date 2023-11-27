use num::{complex::ComplexFloat, Complex};

// parameters
const PHASE_OFFSET: f64 = 0.00; // carrier phase offset
const FREQUENCY_OFFSET: f64 = 0.30; // carrier frequency offset
const WN: f64 = 0.01; // pll bandwidth
const ZETA: f64 = 0.707; // pll damping factor
const K: f64 = 1000.0; // pll loop gain
const N: usize = 400; // number of samples

fn main() {
    // generate loop filter parameters (active PI design)
    let t1 = K / (WN * WN); // tau_1
    let t2 = 2.0 * ZETA / WN; // tau_2

    // feed-forward coefficients (numerator)
    let b0 = (4.0 * K / t1) * (1. + t2 / 2.0);
    let b1 = 8.0 * K / t1;
    let b2 = (4.0 * K / t1) * (1. - t2 / 2.0);

    // feed-back coefficients (denominator)
    //    a0 =  1.0  is implied
    let a1 = -2.0;
    let a2 = 1.0;

    // print filter coefficients (as comments)
    println!("#  b = [b0:{:12.8}, b1:{:12.8}, b2:{:12.8}]", b0, b1, b2);
    println!("#  a = [a0:{:12.8}, a1:{:12.8}, a2:{:12.8}]", 1., a1, a2);

    // filter buffer
    let mut v0 = 0.0;
    let mut v1 = 0.0;
    let mut v2 = 0.0;

    // initialize states
    let mut phi: f64 = PHASE_OFFSET; // input signal's initial phase
    let mut phi_hat: f64 = 0.0; // PLL's initial phase

    let mut x = Complex::new(0., 0.);
    let mut y = Complex::new(0., 0.);

    println!("# {:6} {:12.8} {:12.8} {:12.8} {:12.8} {:12.8} ",
    "index", "real(x)", "imag(x)", "real(y)", "imag(y)", "error");


    for i in 0..N {
        // compute input sinusoid and update phase
        x = Complex::new(phi.cos(), phi.sin());
        phi += FREQUENCY_OFFSET;

        // compute PLL output from phase estimate
        y = Complex::new(phi_hat.cos(), phi_hat.sin());

        // compute error estimate
        let delta_phi: f64 = (x * y.conj()).arg();

        // print results to standard output
        println!(
            "{:6} {:12.8} {:12.8} {:12.8} {:12.8} {:12.8}",
            i, x.re, x.im, y.re, y.im, delta_phi
        );

        // push result through loop filter, updating phase estimate
        v2 = v1; // shift center register to upper register
        v1 = v0; // shift lower register to center register
        v0 = delta_phi - v1 * a1 - v2 * a2; // compute new lower register

        // compute new output
        phi_hat = v0 * b0 + v1 * b1 + v2 * b2;
    }
}
