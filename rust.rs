/*
mandelbrot
2015 Gabriel De Luca
MIT Licensed
*/

const NX: usize = 500;    // number of points
const NT: i32 = 100000; // number of timesteps

fn main() {
	let mut x: [f64; NX] = [0.0_f64;NX];       // position along wave
	let mut y: [f64; NX] = [0.0_f64;NX];       // elevation of wave
	let mut v: [f64; NX] = [0.0_f64;NX];       // speed of wave (y direction)
	let mut dvdt: [f64; NX] = [0.0_f64;NX];    // acceleration of wave (y direction)
	let mut dx: f64;                    // spacing in x
	let mut dt: f64;                    // spacing in t
	let xmin: f64;
	let xmax: f64;
	let tmin: f64;
	let tmax: f64;

	// define x array
	xmin = 0.0;
	xmax = 10.0;
	dx = (xmax-xmin)/((NX as f64)-1.0); // range divided by # intervals
	// x = [f64;NX];
	x[0 as usize] = xmin;
	for i in 1..(NX-1) {
		x[i as usize]=xmin+(i as f64)*dx; // min + i * dx
	}
	x[(NX-1) as usize] = xmax;

	// define t spacing
	tmin = 0.0;
	tmax = 10.0;
	dt = (tmax-tmin)/((NT as f64)-1.0);

	// instantiate y, x, dvdt arrays

	// initialize arrays
	//     y is a peak in the middle of the wave
	for i in 0..NX {
		y[i as usize] = ( -(x[i as usize] - (xmax - xmin) / 2.0) * (x[i as usize] - (xmax - xmin) / 2.0)).exp();
		v[i as usize] = 0.0;
	}

	// iterative loop
	for it in 0..(NT-1) {
		// calculation dvdt at interior positions
		for i in 1..(NX-1) {
			dvdt[i as usize] = (y[(i+1) as usize] + y[(i-1) as usize] - 2.0 * y[i as usize]) / (dx * dx);
		}
		// update v and y
		for i in 0..NX {
			v[i as usize] = v[i as usize] + dt * dvdt[i as usize];
			y[i as usize] = y[i as usize] + dt * v[i as usize];
		}
	}

	// output
	for i in 0..NX {
		println!("{:?}\t{:?}",x[i as usize],y[i as usize]);
	}
}
