use env_logger::{Builder, Env};
use log::debug;
use num::{complex::ComplexFloat, Complex};
use clap::Parser;

// defaults
const PHASE_OFFSET: f64 = 0.00; // carrier phase offset
const FREQUENCY_OFFSET: f64 = 0.30; // carrier frequency offset
const WN: f64 = 0.01; // pll bandwidth
const ZETA: f64 = 0.707; // pll damping factor
const K: f64 = 1000.0; // pll loop gain
const N: usize = 400; // number of samples

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long="loglevel", default_value_t=String::from("info"))]
    pub loglevel: String,

    #[arg(long = "phaseOffset", default_value_t = PHASE_OFFSET)]
    phase_offset: f64,

    #[arg(long = "frequencyOffset", default_value_t = FREQUENCY_OFFSET)]
    frequency_offset: f64,

    #[arg(long = "pll.bandwidth", default_value_t = WN)]
    pll_bandwidth: f64,

    #[arg(long = "pll.damping", default_value_t = ZETA)]
    pll_damping: f64,

    #[arg(long = "pll.loopGain", default_value_t = K)]
    pll_loop_gain: f64,

    #[arg(long = "samples", default_value_t = N)]
    num_samples: usize,

}

fn main() {
    let settings = Cli::parse();

    Builder::from_env(Env::default().default_filter_or(&settings.loglevel))
    .filter_module("paho_mqtt", log::LevelFilter::Warn)
    .init();

    // generate loop filter parameters (active PI design)
    let t1 = settings.pll_loop_gain / (settings.pll_bandwidth * settings.pll_bandwidth); // tau_1
    let t2 = 2.0 * settings.pll_damping / settings.pll_bandwidth; // tau_2

    // feed-forward coefficients (numerator)
    let b0 = (4.0 * settings.pll_loop_gain / t1) * (1. + t2 / 2.0);
    let b1 = 8.0 * settings.pll_loop_gain / t1;
    let b2 = (4.0 * settings.pll_loop_gain / t1) * (1. - t2 / 2.0);

    // feed-back coefficients (denominator)
    //    a0 =  1.0  is implied
    let a1 = -2.0;
    let a2 = 1.0;

    // print filter coefficients (as comments)
    debug!("#  b = [b0:{:12.8}, b1:{:12.8}, b2:{:12.8}]", b0, b1, b2);
    debug!("#  a = [a0:{:12.8}, a1:{:12.8}, a2:{:12.8}]", 1., a1, a2);

    // filter buffer
    let mut v0 = 0.0;
    let mut v1 = 0.0;
    let mut v2: f64;

    // initialize states
    let mut phi: f64 = settings.phase_offset; // input signal's initial phase
    let mut phi_hat: f64 = 0.0; // PLL's initial phase

    let mut ref_input: Complex<f64>;
    //  = Complex::new(0., 0.);
    let mut sig_output: Complex<f64>;

    println!("# {:6} {:12.8} {:12.8} {:12.8} {:12.8} {:12.8} ",
    "index", "real(x)", "imag(x)", "real(y)", "imag(y)", "error");


    for i in 0..settings.num_samples {
        // compute input sinusoid and update phase
        ref_input = Complex::new(phi.cos(), phi.sin());
        phi += settings.frequency_offset;

        // compute PLL output from phase estimate
        sig_output = Complex::new(phi_hat.cos(), phi_hat.sin());

        // compute error estimate
        let delta_phi: f64 = (ref_input * sig_output.conj()).arg();

        // print results to standard output
        println!(
            "{:6} {:12.8} {:12.8} {:12.8} {:12.8} {:12.8}",
            i, ref_input.re, ref_input.im, sig_output.re, sig_output.im, delta_phi
        );

        // push result through loop filter, updating phase estimate
        v2 = v1; // shift center register to upper register
        v1 = v0; // shift lower register to center register
        v0 = delta_phi - v1 * a1 - v2 * a2; // compute new lower register

        // compute new output
        phi_hat = v0 * b0 + v1 * b1 + v2 * b2;
    }
}
