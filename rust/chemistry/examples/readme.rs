//! Chemistry: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p chemistry-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&chemistry_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "buffer_ph", "inputs": {"Ka": "1.737800829e-05", "c_base": "150 mol/m^3", "c_acid": "100 mol/m^3"}}));
    println!();
    show(json!({"key": "isotope_abundance", "inputs": {"m1": "34.97", "m2": "36.97", "A_r": "35.45"}}));
    println!();
    show(json!({"key": "k_from_cell_potential", "inputs": {"n": "2", "E": "0.46 V", "T": "298 K"}}));
    println!();
    show(json!({"key": "equilibrium_ice", "inputs": {"equation": "N2O4 ⇌ 2 NO2", "initial": [0.1, 0], "K": "0.0046"}}));
    println!();
    show(json!({"key": "boyle_final_pressure", "inputs": {"V2": "0.001 m^3", "P1": "101325 Pa", "V1": "0.004 m^3"}}));
    println!();
    show(json!({"key": "activation_energy", "inputs": {"k1": "0.001", "T1": "300 K", "k2": "0.005", "T2": "320 K"}}));
    println!();
    show(json!({"key": "degree_of_unsaturation", "inputs": {"formula": "C6H6"}}));
    println!();
    show(json!({"key": "radioactive_dating_age", "inputs": {"fraction": "0.25", "t_half": "1.80825048e+11 s"}}));
    println!();
    show(json!({"key": "enantiomeric_excess_from_rotation", "inputs": {"alpha_obs": "12", "alpha_pure": "24"}}));
    println!();
    show(json!({"key": "boiling_point_elevation", "inputs": {"K": "0.512 K*kg/mol", "b": "0.5 mol/kg", "i": "1"}}));
    println!();
    show(json!({"key": "beer_lambert_absorbance", "inputs": {"l": "0.01 m", "c": "0.02 mol/m^3", "eps": "1500 m^2/mol"}}));
    println!();
    show(json!({"key": "atoms_of_element", "inputs": {"element": "O", "formula": "CO2", "m": "0.01 kg"}}));
    println!();
    show(json!({"key": "clausius_clapeyron_enthalpy", "inputs": {"P1": "13332.23684 Pa", "T1": "300 K", "P2": "53328.94737 Pa", "T2": "340 K"}}));
    println!();
    show(json!({"key": "conjugate_dissociation_constant", "inputs": {"K": "1.8e-05", "known": "Ka"}}));
    println!();
    show(json!({"key": "average_atomic_mass", "inputs": {"masses": [34.97, 36.97], "abundances": [0.7578, 0.2422]}}));
}
