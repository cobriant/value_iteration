use extendr_api::prelude::*;
use ndarray::{Array, Array2, Array3, Axis};
use std::convert::TryFrom;

#[extendr]
fn value_iteration(
    n: i32, // Changed to i32 for compatibility with R
    reward_guess: Vec<f64>,
    transition_flat: Vec<f64>, // Use a flat vector for transition matrix
    rows: i32, // Individual dimension parameters
    cols: i32,
    depth: i32,
    beta: f64,
    out: &str,
) -> Robj { // Return Robj for compatibility with R
    let n_usize = usize::try_from(n).unwrap();
    let transition_array = Array::from_shape_vec((rows as usize, cols as usize, depth as usize), transition_flat)
        .unwrap_or_else(|_| panic!("Invalid transition matrix shape"));
    let reward = Array::from_shape_vec((n_usize, 1), reward_guess).unwrap();
    let mut value = Array2::zeros((n_usize, 1));
    let mut tol: f64;

    let m = transition_array.shape()[2]; // Number of actions determined from the transition array shape

    for _ in 0..10000 {
        let mut value_action = Array2::zeros((n_usize, m));

        for action in 0..m {
            let transition_slice = transition_array.index_axis(Axis(2), action);
            let value_temp = reward.clone() + beta * transition_slice.dot(&value);
            value_action.column_mut(action).assign(&value_temp.column(0).to_owned());
        }

        let value_next = value_action.map_axis(Axis(1), |row| *row.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap());
        let value_next_2d = value_next.insert_axis(Axis(1));
        tol = (&value - &value_next_2d).mapv(f64::abs).iter().fold(0.0, |a, &b| a.max(b));

        if tol < 0.01 {
            let result = if out == "value_action" {
                value_action
            } else {
                value_next_2d
            };
            return Robj::from(result.iter().cloned().collect::<Vec<f64>>()); // Convert to vector and then to Robj
        }

        value = value_next_2d;
    }

    Robj::from("no convergence")
}

// Macro to generate exports
extendr_module! {
    mod valueiteration;
    fn value_iteration;
}
