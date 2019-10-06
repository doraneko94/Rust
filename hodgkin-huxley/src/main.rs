extern crate gnuplot;

fn alpha_m(v: f64) -> f64 {
    0.1 * (v+40.0) / (1.0 - (-(v+40.0) / 10.0).exp())
}

fn alpha_h(v: f64) -> f64 {
    0.07 * (-(v+65.0) / 20.0).exp()
}

fn alpha_n(v: f64) -> f64 {
    0.01 * (v+55.0) / (1.0 - (-(v+55.0) / 10.0).exp())
}

fn beta_m(v: f64) -> f64 {
    4.0 * (-(v+65.0) / 18.0).exp()
}

fn beta_h(v: f64) -> f64 {
    1.0 / ((-(v+35.0) / 10.0).exp() + 1.0)
}

fn beta_n(v: f64) -> f64 {
    0.125 * (-(v+65.0) / 80.0).exp()
}

fn rk4<F: Fn(f64)->f64>(f: F, v: f64, dt: f64) -> f64 {
    let k1 = dt * f(v);
    let k2 = dt * f(v + 0.5 * k1);
    let k3 = dt * f(v + 0.5 * k2);
    let k4 = dt * f(v + k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn main() {

    const C_M: f64 = 1.0;
    const G_NA: f64 = 120.0;
    const G_K: f64 = 36.0;
    const G_L: f64 = 0.3;
    const E_NA: f64 = 50.0;
    const E_K: f64 = -77.0;
    const E_L: f64 = -54.387;
    let dt = 0.001;

    let mut i_m: f64;

    let mut v = -65.0;
    let mut m = 0.05;
    let mut h = 0.6;
    let mut n = 0.32;
    let mut t = 0.0;

    let mut x = Vec::new();
    let mut y = Vec::new();

    while t < 250.0 {

        i_m = 0.0;
        if t > 50.0 {
            i_m -= 10.0;
            if t > 100.0 {
                i_m += 10.0;
                if t > 150.0 {
                    i_m -= 20.0;
                    if t > 200.0 {
                        i_m += 20.0;
                    }
                }
            }
        }

        let a_m = alpha_m(v);
        let a_h = alpha_h(v);
        let a_n = alpha_n(v);
        let b_m = beta_m(v);
        let b_h = beta_h(v);
        let b_n = beta_n(v);

        let d_m = |m: f64| a_m * (1.0 - m) - b_m * m;
        let d_h = |h: f64| a_h * (1.0 - h) - b_h * h;
        let d_n = |n: f64| a_n * (1.0 - n) - b_n * n;

        m += rk4(d_m, m, dt);
        h += rk4(d_h, h, dt);
        n += rk4(d_n, n, dt);

        let d_v = |v: f64| (i_m - G_NA * m.powi(3) * h * (v - E_NA) - G_K * n.powi(4) * (v - E_K) - G_L * (v - E_L)) / C_M;

        v += rk4(d_v, v, dt);

        x.push(t);
        y.push(v);

        t += dt;
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(),
                      &[gnuplot::Color("blue")]);
    fg.echo_to_file("hh.plt");
}
