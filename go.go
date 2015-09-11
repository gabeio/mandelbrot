/*
mandelbrot
2015 Gabriel De Luca
MIT Licensed
*/

package main

import (
	"fmt"
	"math"
)

func main() {
	var nx, nt int   // number of points, number of timesteps
	// var i, it int    // index i (x) and it (t)
	var x []float64   // position along wave
	var y []float64   // elevation of wave
	var v []float64   // speed of wave (y direction)
	var dvdt []float64// acceleration of wave (y direction)
	var dx,dt float64 // spacing in x, t
	var xmin, xmax, tmin, tmax float64 // range of x, t

	// define x array
	nx = 500
	xmin = 0.0
	xmax = 10.0
	dx = (xmax-xmin)/(float64(nx)-1.0) // range divided by # intervals
	x = make([]float64,nx)
	x[0] = xmin;
	for i := 1; i<nx-1; i++ {
		x[i]=xmin+float64(i)*dx; // min + i * dx
	}
	x[nx-1] = xmax;

	// define t spacing
	nt = 100000
	tmin = 0.0
	tmax = 10.0
	dt = (tmax-tmin)/(float64(nt)-1.0)

	// instantiate y, x, dvdt arrays
	y = make([]float64, nx)
	dvdt = make([]float64, nx)
	v = make([]float64, nx)

	// initialize arrays
	//     y is a peak in the middle of the wave
	for i := 0; i < nx; i++ {
		y[i] = math.Exp( -(x[i] - (xmax - xmin) / 2.0) * (x[i] - (xmax - xmin) / 2.0))
		v[i] = 0.0
	}

	// iterative loop
	for it := 0; it < nt-1; it++ {
		// calculation dvdt at interior positions
		for i := 1; i < nx-1; i++ {
			dvdt[i] = (y[i+1] + y[i-1] - 2.0 * y[i]) / (dx * dx)
		}
		// update v and y
		for i := 0; i < nx; i++ {
			v[i] = v[i] + dt * dvdt[i]
			y[i] = y[i] + dt * v[i]
		}
	}

	// output
	for i := 0; i < nx; i++ {
		fmt.Printf("%v\t%v\n",x[i],y[i])
	}
}
