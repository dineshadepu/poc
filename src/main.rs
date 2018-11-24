extern crate rayon; // 1.0.3
use rayon::prelude::*;

struct DEM {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub fx: Vec<f32>,
    pub fy: Vec<f32>,
}

// Original version, just rewritten to take slices instead
// of references to vectors, since we don't need to add/remove
// elements from the vectors
fn contact_force(
    d_x: &[f32],
    d_y: &[f32],
    d_fx: &mut [f32],
    d_fy: &mut [f32],
    s_x: &[f32],
    s_y: &[f32],
) {
    for i in 0..d_x.len() {
        for j in 0..s_x.len() {
            let dx = d_x[i] - s_x[j];
            let dy = d_y[i] - s_y[j];
            d_fx[i] += 1e5 * dx;
            d_fy[i] += 1e5 * dy;
        }
    }
}

// Version using iterators instead of looping through indices,
// the zip-chains might look a bit hairy, but they can be simplified by
// using the `izip` macro from the itertools crate. I chose not to do that
// here since we want to switch to using rayon instead.
fn contact_force_iter(
    d_x: &[f32],
    d_y: &[f32],
    d_fx: &mut [f32],
    d_fy: &mut [f32],
    s_x: &[f32],
    s_y: &[f32],
) {
    for (d_fxi, (d_fyi, (d_xi, d_yi))) in d_fx
        .iter_mut()
        .zip(d_fy.iter_mut().zip(d_x.iter().zip(d_y.iter())))
    {
        for (s_xj, s_yj) in s_x.iter().zip(s_y.iter()) {
            let dx = d_xi - s_xj;
            let dy = d_yi - s_yj;
            *d_fxi += 1e5 * dx;
            *d_fyi += 1e5 * dy;
        }
    }
}

// Now use the `for_each` method on the iterators instead of using an explicit for-loop
// this should be equivalent, but using the `for_each` method makes this code much more
// similar to the parallel version.
fn contact_force_iter_for_each(
    d_x: &[f32],
    d_y: &[f32],
    d_fx: &mut [f32],
    d_fy: &mut [f32],
    s_x: &[f32],
    s_y: &[f32],
) {
    d_fx.iter_mut()
        .zip(d_fy.iter_mut().zip(d_x.iter().zip(d_y.iter())))
        .for_each(|(d_fxi, (d_fyi, (d_xi, d_yi)))| {
            s_x.iter().zip(s_y.iter()).for_each(|(s_xj, s_yj)| {
                let dx = d_xi - s_xj;
                let dy = d_yi - s_yj;
                *d_fxi += 1e5 * dx;
                *d_fyi += 1e5 * dy;
            });
        });
}

// And now the parallel version, the only thing that has changed is that we use `par_iter_mut()`
// and `par_iter()` instead of `iter_mut()` and `iter()` in the outer loop.
fn contact_force_par(
    d_x: &[f32],
    d_y: &[f32],
    d_fx: &mut [f32],
    d_fy: &mut [f32],
    s_x: &[f32],
    s_y: &[f32],
) {
    d_fx.par_iter_mut()
        .zip(d_fy.par_iter_mut().zip(d_x.par_iter().zip(d_y.par_iter())))
        .for_each(|(d_fxi, (d_fyi, (d_xi, d_yi)))| {
            s_x.iter().zip(s_y.iter()).for_each(|(s_xj, s_yj)| {
                let dx = d_xi - s_xj;
                let dy = d_yi - s_yj;
                *d_fxi += 1e5 * dx;
                *d_fyi += 1e5 * dy;
            });
        });
}

fn main() {
    let mut dem = DEM {
        x: vec![0.; 4000],
        y: vec![0.; 4000],
        fx: vec![0.; 4000],
        fy: vec![0.; 4000],
    };
    // contact_force_iter(&dem.x, &dem.y, &mut dem.fx, &mut dem.fy, &dem.x, &dem.y);
    let mut tf = 1.;
    let dt = 1e-3;
    while tf > 0.{
        contact_force_par(&dem.x, &dem.y, &mut dem.fx, &mut dem.fy, &dem.x, &dem.y);
        // contact_force(&dem.x, &dem.y, &mut dem.fx, &mut dem.fy, &dem.x, &dem.y);
        tf = tf - dt;
        // println!("tf {}", tf);
    }
}
