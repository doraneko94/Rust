// This code is quoted from http://suzuzusu.hatenablog.com/entry/2018/03/27/093132

extern crate gnuplot;

fn rk4<F: Fn(f64)->f64>(f: F, x: f64, dt: f64) -> f64 {
    let k1 = dt * f(x);
    let k2 = dt * f(x+0.5*k1);
    let k3 = dt * f(x+0.5*k2);
    let k4 = dt * f(x+k3);
    (k1+2.0*k2+2.0*k3+k4) / 6.0
}

fn main() {
    const A: f64 = 0.7;
    const B: f64 = 0.8;
    const C: f64 = 10.0;

    let mut v = 0.0;
    let mut w = 0.0;
    let mut t = -30.0;
    let dt = 0.001;
    let mut i_ext;

    let mut x = Vec::new();
    let mut y = Vec::new();

    while t <= 100.0 {
        
        if t >= 30.0 && t <= 70.0 {
            i_ext = 0.35;
        } else {
            i_ext = 0.0;
        }

        let d_v = move |y: f64| C * (-(y.powi(3)/3.0) + y - w + i_ext);
        let d_w = move |y: f64| v - B*y + A;
        w += rk4(d_w, w, dt);
        v += rk4(d_v, v, dt);

        if t >= 0.0 {
            y.push(v);
            x.push(t);
        }

        t += dt;
    }

    let mut fg = gnuplot::Figure::new();
    fg.axes2d().lines(x.iter(), y.iter(), &[gnuplot::Color("blue")]);
    fg.echo_to_file("fn.plt");
}
