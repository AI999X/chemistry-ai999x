//! chemistry: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/chemistry/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "activation_energy",
    "arrhenius_k2",
    "arrhenius_rate_constant",
    "atoms_of_element",
    "average_atomic_mass",
    "balance_equation",
    "beer_lambert_absorbance",
    "beer_lambert_concentration",
    "boiling_point_elevation",
    "bond_enthalpy_reaction",
    "boyle_final_pressure",
    "boyle_final_volume",
    "buffer_ph",
    "calorimetry_heat",
    "charles_final_volume",
    "clausius_clapeyron_enthalpy",
    "clausius_clapeyron_pressure",
    "combined_gas_final_pressure",
    "combined_gas_final_temperature",
    "combined_gas_final_volume",
    "common_ion_solubility",
    "conjugate_dissociation_constant",
    "consecutive_first_order",
    "crossover_temperature",
    "dalton_mixture",
    "degree_of_unsaturation",
    "delta_g_from_cell_potential",
    "delta_g_from_k",
    "delta_g_nonstandard",
    "dilution_final_concentration",
    "dilution_volume",
    "electrolysis_mass",
    "electrolysis_time",
    "empirical_formula",
    "enantiomeric_excess_from_composition",
    "enantiomeric_excess_from_rotation",
    "enthalpy_from_calorimetry",
    "equilibrium_ice",
    "first_order_concentration",
    "first_order_half_life",
    "first_order_time",
    "freezing_point_depression",
    "gas_density",
    "gay_lussac_final_pressure",
    "gibbs_free_energy",
    "graham_molar_mass",
    "graham_rate_ratio",
    "heat_from_reaction_mass",
    "henry_law_solubility",
    "hydrogen_transition_wavelength",
    "hydronium_from_ph",
    "ideal_gas_mass",
    "ideal_gas_moles",
    "ideal_gas_pressure",
    "ideal_gas_temperature",
    "ideal_gas_volume",
    "isotope_abundance",
    "k_from_cell_potential",
    "k_from_delta_g",
    "ka_from_ph",
    "kc_from_kp",
    "kp_from_kc",
    "ksp_from_solubility",
    "ksp_molar_solubility",
    "limiting_reagent",
    "mass_from_moles",
    "mass_of_element",
    "mass_percent",
    "mean_speed",
    "molality",
    "molar_mass",
    "molar_mass_from_freezing_point",
    "molar_mass_from_gas_density",
    "molar_mass_from_gas_sample",
    "molar_mass_from_osmotic_pressure",
    "molarity_from_mass",
    "mole_fraction",
    "molecular_formula",
    "moles_from_mass",
    "moles_from_particles",
    "nernst_potential",
    "nuclear_binding_energy",
    "osmotic_pressure",
    "partial_pressure",
    "particles_from_mass",
    "particles_from_moles",
    "percent_yield",
    "ph_diprotic_acid",
    "ph_from_hydronium",
    "ph_from_hydroxide",
    "ph_salt_of_weak_acid",
    "ph_strong_acid",
    "ph_strong_base",
    "ph_weak_acid",
    "ph_weak_base",
    "photon_energy",
    "photon_wavelength",
    "precipitation_test",
    "radioactive_dating_age",
    "radioactive_decay_remaining",
    "raoult_vapor_pressure",
    "reaction_enthalpy_from_formation",
    "reaction_entropy_from_standard_entropies",
    "reaction_gibbs_from_formation",
    "reaction_order_from_rates",
    "reaction_quotient_direction",
    "rms_speed",
    "second_order_concentration",
    "solute_mass_for_solution",
    "specific_rotation",
    "standard_cell_potential",
    "theoretical_yield",
    "titration_equivalence_volume",
    "titration_ph_strong_acid_strong_base",
    "titration_ph_weak_acid_strong_base",
    "van_der_waals_pressure",
    "vant_hoff_k2",
    "vibrational_wavenumber",
    "zero_order_concentration",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "activation_energy" => {
            let r = f_activation_energy(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arrhenius_k2" => {
            let r = f_arrhenius_k2(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ea") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_16(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "arrhenius_rate_constant" => {
            let r = f_arrhenius_k(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ea") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_25(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "atoms_of_element" => {
            let r = f_atoms_of_element(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_66(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "average_atomic_mass" => {
            let r = f_average_atomic_mass(match v.get("masses") { Some(x) => py::from_val(x), None => { return None } }, match v.get("abundances") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "balance_equation" => {
            let r = f_balance_equation(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_balance(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beer_lambert_absorbance" => {
            let r = f_beer_absorbance(match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_276(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beer_lambert_concentration" => {
            let r = f_beer_concentration(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boiling_point_elevation" => {
            let r = f__colligative(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_296(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bond_enthalpy_reaction" => {
            let r = f_bond_enthalpy_reaction(match v.get("broken_n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("broken_D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formed_n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formed_D") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_bonds(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boyle_final_pressure" => {
            let r = f_boyle_p2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_317(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boyle_final_volume" => {
            let r = f_boyle_v2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_327(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "buffer_ph" => {
            let r = f_buffer_ph(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_acid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_337(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "calorimetry_heat" => {
            let r = f_calorimetry_heat(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_344(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "charles_final_volume" => {
            let r = f_charles_v2(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_354(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clausius_clapeyron_enthalpy" => {
            let r = f_cc_enthalpy(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_366(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clausius_clapeyron_pressure" => {
            let r = f_cc_pressure(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_373(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_gas_final_pressure" => {
            let r = f_combined_p2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_382(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_gas_final_temperature" => {
            let r = f_combined_t2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_392(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_gas_final_volume" => {
            let r = f_combined_v2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_402(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "common_ion_solubility" => {
            let r = f_common_ion(match v.get("Ksp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_common") { Some(x) => py::from_val(x), None => { return None } }, match v.get("common") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_441(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "conjugate_dissociation_constant" => {
            let r = f_conjugate_constant(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("known") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_461(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "consecutive_first_order" => {
            let r = f_consecutive(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crossover_temperature" => {
            let r = f_crossover_t(match v.get("dH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_495(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dalton_mixture" => {
            let r = f_dalton_mixture(match v.get("moles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_total") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_510(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "degree_of_unsaturation" => {
            let r = f_degree_of_unsaturation(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_548(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_g_from_cell_potential" => {
            let r = f_dg_from_e(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_585(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_g_from_k" => {
            let r = f_dg_from_k(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_595(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_g_nonstandard" => {
            let r = f_dg_nonstandard(match v.get("dG0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_605(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dilution_final_concentration" => {
            let r = f_dilution_c2(match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_614(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dilution_volume" => {
            let r = f_dilution_v1(match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_626(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "electrolysis_mass" => {
            let r = f_electrolysis_mass(match v.get("metal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_640(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "electrolysis_time" => {
            let r = f_electrolysis_time(match v.get("metal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_654(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "empirical_formula" => {
            let r = f_empirical_formula(match v.get("composition") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_empirical(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enantiomeric_excess_from_composition" => {
            let r = f_ee_from_composition(match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ee_comp(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enantiomeric_excess_from_rotation" => {
            let r = f_ee_from_rotation(match v.get("alpha_obs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha_pure") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ee_rot(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enthalpy_from_calorimetry" => {
            let r = f_calorimetry_enthalpy(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_758(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equilibrium_ice" => {
            let r = f_equilibrium_ice(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ice(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_concentration" => {
            let r = f_first_order_conc(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_872(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_half_life" => {
            let r = f_first_order_half_life(match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_881(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_time" => {
            let r = f_first_order_time(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "freezing_point_depression" => {
            let r = f__colligative(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_903(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gas_density" => {
            let r = f_gas_density(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_939(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gay_lussac_final_pressure" => {
            let r = f_gay_lussac_p2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_977(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gibbs_free_energy" => {
            let r = f_gibbs(match v.get("dH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_994(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graham_molar_mass" => {
            let r = f_graham_unknown(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_known") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1027(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graham_rate_ratio" => {
            let r = f_graham_ratio(match v.get("formula_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1085(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heat_from_reaction_mass" => {
            let r = f_heat_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1152(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "henry_law_solubility" => {
            let r = f_henry_solubility(match v.get("kH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1190(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydrogen_transition_wavelength" => {
            let r = f_hydrogen_line(match v.get("n_upper") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_lower") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1200(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydronium_from_ph" => {
            let r = f_h_from_ph(match v.get("pH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_mass" => {
            let r = f_gas_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1243(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_moles" => {
            let r = f__ig_moles(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1281(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_pressure" => {
            let r = f__ig_pressure(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_temperature" => {
            let r = f__ig_temperature(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1301(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_volume" => {
            let r = f__ig_volume(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1311(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "isotope_abundance" => {
            let r = f_isotope_abundance(match v.get("m1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A_r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1326(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "k_from_cell_potential" => {
            let r = f_k_from_e(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1338(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "k_from_delta_g" => {
            let r = f_k_from_dg(match v.get("dG") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1348(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ka_from_ph" => {
            let r = f_ka_from_ph(match v.get("pH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1366(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kc_from_kp" => {
            let r = f_kc_from_kp(match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1393(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kp_from_kc" => {
            let r = f_kp_from_kc(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1406(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ksp_from_solubility" => {
            let r = f_ksp_from_s(match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1417(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ksp_molar_solubility" => {
            let r = f_ksp_solubility(match v.get("Ksp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1433(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "limiting_reagent" => {
            let r = f_limiting_reagent(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("product") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_limiting(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mass_from_moles" => {
            let r = f_mass_from_moles(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1618(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mass_of_element" => {
            let r = f_mass_of_element(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1688(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mass_percent" => {
            let r = f_mass_percent(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1754(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_speed" => {
            let r = f_mean_speed(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1818(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molality" => {
            let r = f_molality(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1882(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass" => {
            let r = f_molar_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1940(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_freezing_point" => {
            let r = f_molar_mass_from_freezing(match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1978(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_gas_density" => {
            let r = f_molar_mass_from_density(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1988(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_gas_sample" => {
            let r = f_molar_mass_from_sample(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1998(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_osmotic_pressure" => {
            let r = f_molar_mass_from_osmotic(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2008(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molarity_from_mass" => {
            let r = f_molarity_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2044(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mole_fraction" => {
            let r = f_mole_fraction(match v.get("formula_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molecular_formula" => {
            let r = f_molecular_formula(match v.get("empirical") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2183(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moles_from_mass" => {
            let r = f_moles_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2254(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moles_from_particles" => {
            let r = f_moles_from_particles(match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2292(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nernst_potential" => {
            let r = f_nernst(match v.get("E0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2305(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nuclear_binding_energy" => {
            let r = f_nuclear_binding_energy(match v.get("Z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mass") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_binding(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "osmotic_pressure" => {
            let r = f_osmotic_pressure(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2334(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_pressure" => {
            let r = f_partial_pressure(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_total") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2347(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "particles_from_mass" => {
            let r = f_particles_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2383(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "particles_from_moles" => {
            let r = f_particles_from_moles(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2421(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percent_yield" => {
            let r = f_percent_yield(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("product") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2531(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_diprotic_acid" => {
            let r = f_ph_diprotic(match v.get("Ka1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ka2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2586(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_from_hydronium" => {
            let r = f_ph_from_h(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_from_hydroxide" => {
            let r = f_ph_from_oh(match v.get("oh") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2600(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_salt_of_weak_acid" => {
            let r = f_ph_salt_weak_acid(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2624(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_strong_acid" => {
            let r = f_ph_strong_acid(match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2631(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_strong_base" => {
            let r = f_ph_strong_base(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_oh") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2638(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_weak_acid" => {
            let r = f_ph_weak_acid(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2662(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_weak_base" => {
            let r = f_ph_weak_base(match v.get("Kb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2686(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photon_energy" => {
            let r = f_photon_energy(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2693(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photon_wavelength" => {
            let r = f_photon_wavelength(match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2703(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "precipitation_test" => {
            let r = f_precipitation_test(match v.get("Ksp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_cation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_anion") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_precip(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radioactive_dating_age" => {
            let r = f_radioactive_age(match v.get("fraction") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_half") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2737(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radioactive_decay_remaining" => {
            let r = f_radioactive_remaining(match v.get("m0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_half") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2752(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "raoult_vapor_pressure" => {
            let r = f_raoult(match v.get("formula_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2789(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_enthalpy_from_formation" => {
            let r = f_formation_sum(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("values") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2907(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_entropy_from_standard_entropies" => {
            let r = f_reaction_entropy(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("values") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2998(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_gibbs_from_formation" => {
            let r = f_reaction_gibbs_formation(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("values") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_gibbs_formation(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_order_from_rates" => {
            let r = f_reaction_order(match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3099(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_quotient_direction" => {
            let r = f_reaction_quotient(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("conc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_quotient(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rms_speed" => {
            let r = f_rms_speed(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3198(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_concentration" => {
            let r = f_second_order_conc(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3236(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solute_mass_for_solution" => {
            let r = f_solute_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "specific_rotation" => {
            let r = f_specific_rotation(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3313(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "standard_cell_potential" => {
            let r = f_standard_cell_potential(match v.get("E_cathode") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_anode") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3329(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "theoretical_yield" => {
            let r = f_theoretical_yield(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("product") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3438(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "titration_equivalence_volume" => {
            let r = f_equivalence_volume(match v.get("c_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3476(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "titration_ph_strong_acid_strong_base" => {
            let r = f__strong_strong(match v.get("c_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3490(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "titration_ph_weak_acid_strong_base" => {
            let r = f__weak_titr(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3517(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "van_der_waals_pressure" => {
            let r = f_vdw_pressure(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3527(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vant_hoff_k2" => {
            let r = f_vant_hoff(match v.get("K1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vibrational_wavenumber" => {
            let r = f_vib_wavenumber(match v.get("molecule") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3581(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zero_order_concentration" => {
            let r = f_zero_order_conc(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_ATM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(101325.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_C_LIGHT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(299792458.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ELEMENT_NAMES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("hydrogen"), Py::str("H")), (Py::str("helium"), Py::str("He")), (Py::str("lithium"), Py::str("Li")), (Py::str("beryllium"), Py::str("Be")), (Py::str("boron"), Py::str("B")), (Py::str("carbon"), Py::str("C")), (Py::str("nitrogen"), Py::str("N")), (Py::str("oxygen"), Py::str("O")), (Py::str("fluorine"), Py::str("F")), (Py::str("neon"), Py::str("Ne")), (Py::str("sodium"), Py::str("Na")), (Py::str("magnesium"), Py::str("Mg")), (Py::str("aluminium"), Py::str("Al")), (Py::str("aluminum"), Py::str("Al")), (Py::str("silicon"), Py::str("Si")), (Py::str("phosphorus"), Py::str("P")), (Py::str("sulfur"), Py::str("S")), (Py::str("sulphur"), Py::str("S")), (Py::str("chlorine"), Py::str("Cl")), (Py::str("argon"), Py::str("Ar")), (Py::str("potassium"), Py::str("K")), (Py::str("calcium"), Py::str("Ca")), (Py::str("titanium"), Py::str("Ti")), (Py::str("chromium"), Py::str("Cr")), (Py::str("manganese"), Py::str("Mn")), (Py::str("iron"), Py::str("Fe")), (Py::str("cobalt"), Py::str("Co")), (Py::str("nickel"), Py::str("Ni")), (Py::str("copper"), Py::str("Cu")), (Py::str("zinc"), Py::str("Zn")), (Py::str("bromine"), Py::str("Br")), (Py::str("silver"), Py::str("Ag")), (Py::str("tin"), Py::str("Sn")), (Py::str("iodine"), Py::str("I")), (Py::str("barium"), Py::str("Ba")), (Py::str("platinum"), Py::str("Pt")), (Py::str("gold"), Py::str("Au")), (Py::str("mercury"), Py::str("Hg")), (Py::str("lead"), Py::str("Pb")), (Py::str("uranium"), Py::str("U")), (Py::str("cadmium"), Py::str("Cd")), (Py::str("strontium"), Py::str("Sr"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-19)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_F() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(96485.33212331001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_H_PLANCK() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.62607015e-34)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_ISO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("H"), Py::Float(1.007825032)), (Py::str("C"), Py::Float(12.0)), (Py::str("N"), Py::Float(14.003074)), (Py::str("O"), Py::Float(15.99491462)), (Py::str("F"), Py::Float(18.99840322)), (Py::str("Na"), Py::Float(22.98976928)), (Py::str("Si"), Py::Float(27.97692653)), (Py::str("P"), Py::Float(30.97376163)), (Py::str("S"), Py::Float(31.972071)), (Py::str("Cl"), Py::Float(34.96885268)), (Py::str("Br"), Py::Float(78.9183371)), (Py::str("I"), Py::Float(126.904473)), (Py::str("B"), Py::Float(11.0093054)), (Py::str("Li"), Py::Float(7.01600455)), (Py::str("K"), Py::Float(38.96370668)), (Py::str("Mg"), Py::Float(23.9850417)), (Py::str("Al"), Py::Float(26.98153863)), (Py::str("Ca"), Py::Float(39.96259098))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1e-14)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6931471805599453)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MEV_J() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.602176634e-13)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_H1_U() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.007825031898)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_M_N_U() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.00866491606)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_NA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.02214076e+23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_H() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(10967758.340277294)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_LATM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.08205736608095968)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_U_KG() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.66053906892e-27)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_U_MEV() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(931.49410372)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_W() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("H"), Py::Float(1.008)), (Py::str("He"), Py::Float(4.003)), (Py::str("Li"), Py::Float(6.941)), (Py::str("Be"), Py::Float(9.012)), (Py::str("B"), Py::Float(10.812)), (Py::str("C"), Py::Float(12.011)), (Py::str("N"), Py::Float(14.007)), (Py::str("O"), Py::Float(15.999)), (Py::str("F"), Py::Float(18.998)), (Py::str("Ne"), Py::Float(20.18)), (Py::str("Na"), Py::Float(22.99)), (Py::str("Mg"), Py::Float(24.305)), (Py::str("Al"), Py::Float(26.982)), (Py::str("Si"), Py::Float(28.086)), (Py::str("P"), Py::Float(30.974)), (Py::str("S"), Py::Float(32.067)), (Py::str("Cl"), Py::Float(35.453)), (Py::str("Ar"), Py::Float(39.948)), (Py::str("K"), Py::Float(39.098)), (Py::str("Ca"), Py::Float(40.078)), (Py::str("Sc"), Py::Float(44.956)), (Py::str("Ti"), Py::Float(47.867)), (Py::str("V"), Py::Float(50.944)), (Py::str("Cr"), Py::Float(51.996)), (Py::str("Mn"), Py::Float(54.938)), (Py::str("Fe"), Py::Float(55.845)), (Py::str("Co"), Py::Float(58.933)), (Py::str("Ni"), Py::Float(58.693)), (Py::str("Cu"), Py::Float(63.546)), (Py::str("Zn"), Py::Float(65.39)), (Py::str("Ga"), Py::Float(69.723)), (Py::str("Ge"), Py::Float(72.61)), (Py::str("As"), Py::Float(74.922)), (Py::str("Se"), Py::Float(78.96)), (Py::str("Br"), Py::Float(79.904)), (Py::str("Kr"), Py::Float(83.8)), (Py::str("Rb"), Py::Float(85.468)), (Py::str("Sr"), Py::Float(87.62)), (Py::str("Y"), Py::Float(88.906)), (Py::str("Zr"), Py::Float(91.224)), (Py::str("Nb"), Py::Float(92.906)), (Py::str("Mo"), Py::Float(95.94)), (Py::str("Ru"), Py::Float(101.07)), (Py::str("Rh"), Py::Float(102.906)), (Py::str("Pd"), Py::Float(106.42)), (Py::str("Ag"), Py::Float(107.868)), (Py::str("Cd"), Py::Float(112.412)), (Py::str("In"), Py::Float(114.818)), (Py::str("Sn"), Py::Float(118.711)), (Py::str("Sb"), Py::Float(121.76)), (Py::str("Te"), Py::Float(127.6)), (Py::str("I"), Py::Float(126.904)), (Py::str("Xe"), Py::Float(131.29)), (Py::str("Cs"), Py::Float(132.905)), (Py::str("Ba"), Py::Float(137.328)), (Py::str("La"), Py::Float(138.906)), (Py::str("Ce"), Py::Float(140.116)), (Py::str("Pr"), Py::Float(140.908)), (Py::str("Nd"), Py::Float(144.24)), (Py::str("Sm"), Py::Float(150.36)), (Py::str("Eu"), Py::Float(151.964)), (Py::str("Gd"), Py::Float(157.25)), (Py::str("Tb"), Py::Float(158.925)), (Py::str("Dy"), Py::Float(162.5)), (Py::str("Ho"), Py::Float(164.93)), (Py::str("Er"), Py::Float(167.26)), (Py::str("Tm"), Py::Float(168.934)), (Py::str("Yb"), Py::Float(173.04)), (Py::str("Lu"), Py::Float(174.967)), (Py::str("Hf"), Py::Float(178.49)), (Py::str("Ta"), Py::Float(180.948)), (Py::str("W"), Py::Float(183.84)), (Py::str("Re"), Py::Float(186.207)), (Py::str("Os"), Py::Float(190.23)), (Py::str("Ir"), Py::Float(192.217)), (Py::str("Pt"), Py::Float(195.078)), (Py::str("Au"), Py::Float(196.967)), (Py::str("Hg"), Py::Float(200.59)), (Py::str("Tl"), Py::Float(204.383)), (Py::str("Pb"), Py::Float(207.2)), (Py::str("Bi"), Py::Float(208.98)), (Py::str("Th"), Py::Float(232.038)), (Py::str("Pa"), Py::Float(231.036)), (Py::str("U"), Py::Float(238.029))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YEAR_S() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31557600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__ARROW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\s*(?:<=>|<->|⇌|⇄|⟷|->|-->|→|⟶|=>|=)\\s*"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__DOU_EL() -> R {
    thread_local!(static V: R = (|| -> R { Ok(py::set(&Py::list(vec![Py::str("C"), Py::str("H"), Py::str("N"), Py::str("O"), Py::str("S"), Py::str("P"), Py::str("F"), Py::str("Cl"), Py::str("Br"), Py::str("I"), Py::str("Si"), Py::str("B")]))?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FTOK() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("([A-Z][a-z]?)(\\d*)|([(\\[{])|([)\\]}])(\\d*)"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__HYDRATE_SPLIT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("[·•⋅∙*]|\\.(?=\\d|[A-Z(\\[])"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PAIRS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("("), Py::str(")")), (Py::str("["), Py::str("]")), (Py::str("{"), Py::str("}"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__STATE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pyre::module_call("compile", &Py::str("\\((?:s|l|g|aq)\\)"), &[], 0)?) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__SUBSCRIPT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::Int(8320i64), Py::Int(48i64)), (Py::Int(8321i64), Py::Int(49i64)), (Py::Int(8322i64), Py::Int(50i64)), (Py::Int(8323i64), Py::Int(51i64)), (Py::Int(8324i64), Py::Int(52i64)), (Py::Int(8325i64), Py::Int(53i64)), (Py::Int(8326i64), Py::Int(54i64)), (Py::Int(8327i64), Py::Int(55i64)), (Py::Int(8328i64), Py::Int(56i64)), (Py::Int(8329i64), Py::Int(57i64))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__abs_sum(mut v_values: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_s = Py::Float(0.0);
    for t3088 in py::iter(&(v_values.clone()))? {
        v_v = t3088;
        v_s = py::iadd(&v_s, &py::abs(&v_v.clone())?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__abundance_scale(mut v_abundances: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = Py::Float(0.0);
    for t108 in py::iter(&(v_abundances.clone()))? {
        v_f = t108;
        v_s = py::iadd(&v_s, &v_f.clone())?;
    }
    if ({ let t109 = py::abs(&py::sub(&v_s.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t110 = Py::Float(0.002); if py::le(&t109, &t110)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    if ({ let t111 = py::abs(&py::sub(&v_s.clone(), &Py::Float(100.0))?)?; Py::Bool({ let t112 = Py::Float(0.2); if py::le(&t111, &t112)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.01));
    }
    return Err(py::value_error(py::to_str(&Py::str("the abundances must add up to 1 (fractions) or to 100 (percent)"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__as_written_balanced(mut v_text: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_products: Py = Py::None;
    let mut v_reactants: Py = Py::None;
    let mut v_species: Py = Py::None;
    let t3036 = (f_parse_equation(v_text.clone())?).items()?;
    if t3036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reactants = t3036[0].clone();
    v_products = t3036[1].clone();
    v_coefs = { let mut out = vec![]; for t3037 in py::iter(&(py::add(&v_reactants.clone(), &v_products.clone())?))? { let mut v__f: Py; let mut v_c: Py; 
let t3038 = (t3037).items()?;
if t3038.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t3038[0].clone();
v__f = t3038[1].clone();
out.push(v_c.clone()); } Py::list(out) };
    v_species = { let mut out = vec![]; for t3039 in py::iter(&(py::add(&v_reactants.clone(), &v_products.clone())?))? { let mut v__c: Py; let mut v_f: Py; 
let t3040 = (t3039).items()?;
if t3040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t3040[0].clone();
v_f = t3040[1].clone();
out.push(v_f.clone()); } Py::list(out) };
    if (py::not(&f__conserved(v_coefs.clone(), v_species.clone(), py::len(&v_reactants.clone())?)?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation as written is not balanced; balance it first"))));
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t3077 in py::iter(&(v_coefs.clone()))? { let mut v_c: Py; 
v_c = t3077;
out.push(py::float(&v_c.clone())?); } Py::list(out) }, v_species.clone(), py::len(&v_reactants.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_lo: Py, mut v_hi: Py, mut v_iters: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mid: Py = Py::None;
    v_flo = py::call(&v_f.clone(), &[v_lo.clone()])?;
    if ({ let t3500 = v_flo.clone(); Py::Bool({ let t3501 = Py::Int(0i64); if py::eq(&t3500, &t3501) { true } else { false } }) }).truthy() {
        return Ok(v_lo.clone());
    }
    for t3502 in py::range_iter(&[(v_iters.clone()).i()?]) {
        v__ = t3502;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t3507 = { let t3503 = v_mid.clone(); Py::Bool({ let t3504 = v_lo.clone(); if py::eq(&t3503, &t3504) { true } else { false } }) }; if t3507.truthy() { t3507 } else { { let t3505 = v_mid.clone(); Py::Bool({ let t3506 = v_hi.clone(); if py::eq(&t3505, &t3506) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_mid.clone()])?;
        if ({ let t3508 = v_fm.clone(); Py::Bool({ let t3509 = Py::Int(0i64); if py::eq(&t3508, &t3509) { true } else { false } }) }).truthy() {
            return Ok(v_mid.clone());
        }
        if ({ let t3510 = { let t3512 = v_fm.clone(); Py::Bool({ let t3513 = Py::Int(0i64); if py::lt(&t3512, &t3513)? { true } else { false } }) }; Py::Bool({ let t3511 = { let t3514 = v_flo.clone(); Py::Bool({ let t3515 = Py::Int(0i64); if py::lt(&t3514, &t3515)? { true } else { false } }) }; if py::eq(&t3510, &t3511) { true } else { false } }) }).truthy() {
            let t3516 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t3516.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t3516[0].clone();
            v_flo = t3516[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bond_sum(mut v_ns: Py, mut v_Ds: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = Py::Float(0.0);
    for t305 in py::iter(&(py::zip(&[v_ns.clone(), v_Ds.clone()])?))? {
        let t310 = (t305).items()?;
        if t310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n = t310[0].clone();
        v_D = t310[1].clone();
        if (py::not(&{ let t306 = v_n.clone(); Py::Bool({ let t307 = Py::Int(0i64); if py::gt(&t306, &t307)? { true } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("bond counts must be positive"))));
        }
        if (py::not(&{ let t308 = v_D.clone(); Py::Bool({ let t309 = Py::Int(0i64); if py::gt(&t308, &t309)? { true } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("bond enthalpies are positive (the energy needed to break a bond)"))));
        }
        v_s = py::iadd(&v_s, &py::mul(&v_n.clone(), &v_D.clone())?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cell_word(mut v_E: Py) -> R {
    if ({ let t3325 = v_E.clone(); Py::Bool({ let t3326 = Py::Int(0i64); if py::gt(&t3325, &t3326)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("spontaneous (galvanic)"));
    }
    if ({ let t3327 = v_E.clone(); Py::Bool({ let t3328 = Py::Int(0i64); if py::lt(&t3327, &t3328)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("non-spontaneous (electrolytic)"));
    }
    return Ok(Py::str("at equilibrium"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__coef_sum(mut v_equation: Py, mut v_values: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_coefs: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_species: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let t3078 = (f__as_written_balanced(v_equation.clone())?).items()?;
    if t3078.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t3078[0].clone();
    v_species = t3078[1].clone();
    v_nr = t3078[2].clone();
    if ({ let t3079 = py::len(&v_values.clone())?; Py::Bool({ let t3080 = py::len(&v_species.clone())?; if py::ne(&t3079, &t3080) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one value per species in the order written (".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_species.clone()])?, "")?, ")".to_string()].concat()))));
    }
    v_terms = Py::list(vec![]);
    for t3081 in py::range_iter(&[(py::len(&v_species.clone())?).i()?]) {
        v_j = t3081;
        v_c = (if ({ let t3082 = v_j.clone(); Py::Bool({ let t3083 = v_nr.clone(); if py::ge(&t3082, &t3083)? { true } else { false } }) }).truthy() { py::getitem(&v_coefs.clone(), &v_j.clone())? } else { py::neg(&py::getitem(&v_coefs.clone(), &v_j.clone())?)? });
        let _ = py::method(&v_terms.clone(), "append", &[py::mul(&v_c.clone(), &py::getitem(&v_values.clone(), &v_j.clone())?)?])?;
    }
    return Ok(py::fsum(&v_terms.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__colligative(mut v_i: Py, mut v_K: Py, mut v_b: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("K"), v_K.clone()), (Py::str("b"), v_b.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("dT"), py::mul(&py::mul(&v_i.clone(), &v_K.clone())?, &v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__composition(mut v_text: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_pairs = ax_engine::pyre::module_call("findall", &Py::str("([A-Z][a-z]?)\\s*[:=]?\\s*(\\d+(?:\\.\\d+)?)"), &[v_text.clone()], 0)?;
    if ({ let t658 = py::len(&v_pairs.clone())?; Py::Bool({ let t659 = Py::Int(2i64); if py::lt(&t658, &t659)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least two elements with their percentages, e.g. 'C 40.0, H 6.71, O 53.3'"))));
    }
    v_comp = Py::dict(vec![]);
    for t660 in py::iter(&(v_pairs.clone()))? {
        let t669 = (t660).items()?;
        if t669.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_el = t669[0].clone();
        v_v = t669[1].clone();
        v_el = f__element(v_el.clone())?;
        if ({ let t665 = v_el.clone(); Py::Bool({ let t666 = v_comp.clone(); if py::contains(&t666, &t665)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_el.clone(), "")?, " is given twice".to_string()].concat()))));
        }
        py::setitem(&v_comp.clone(), &v_el.clone(), py::float(&v_v.clone())?)?;
        if ({ let t667 = py::getitem(&v_comp.clone(), &v_el.clone())?; Py::Bool({ let t668 = Py::Int(0i64); if py::le(&t667, &t668)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every percentage must be positive"))));
        }
    }
    return Ok(v_comp.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__conj_word(mut v_known: Py) -> R {
    return Ok((if ({ let t459 = v_known.clone(); Py::Bool({ let t460 = Py::str("Ka"); if py::eq(&t459, &t460) { true } else { false } }) }).truthy() { Py::str("Kb of the conjugate base") } else { Py::str("Ka of the conjugate acid") }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__conserved(mut v_coefs: Py, mut v_species: Py, mut v_nr: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_tot: Py = Py::None;
    v_tot = Py::dict(vec![]);
    for t3041 in py::iter(&(py::enumerate(&py::zip(&[v_coefs.clone(), v_species.clone()])?, 0)?))? {
        let t3072 = (t3041).items()?;
        if t3072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t3072[0].clone();
        let t3073 = (t3072[1].clone()).items()?;
        if t3073.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t3073[0].clone();
        v_f = t3073[1].clone();
        for t3068 in py::iter(&(py::method(&f__counts_rd(v_f.clone())?, "items", &[])?))? {
            let t3071 = (t3068).items()?;
            if t3071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_el = t3071[0].clone();
            v_n = t3071[1].clone();
            py::setitem(&v_tot.clone(), &v_el.clone(), py::add(&py::method(&v_tot.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &(if ({ let t3069 = v_j.clone(); Py::Bool({ let t3070 = v_nr.clone(); if py::lt(&t3069, &t3070)? { true } else { false } }) }).truthy() { py::mul(&v_c.clone(), &v_n.clone())? } else { py::mul(&py::neg(&v_c.clone())?, &v_n.clone())? }))?)?;
        }
    }
    return Ok(Py::Bool(({ let mut out = vec![]; for t3074 in py::iter(&(py::method(&v_tot.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t3074;
out.push({ let t3075 = v_v.clone(); Py::Bool({ let t3076 = Py::Int(0i64); if py::eq(&t3075, &t3076) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__counts_rd(mut v_text: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mult: Py = Py::None;
    let mut v_number: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_seq: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_s = py::method(&py::method(&v_text.clone(), "translate", &[c__SUBSCRIPT()?])?, "replace", &[Py::str(" "), Py::str("")])?;
    v_parts = ax_engine::pyre::module_call("split", &Py::str("[·•⋅∙*]|\\.(?=\\d|[A-Z(\\[])"), &[v_s.clone(), Py::Int(0)], 0)?;
    v_out = Py::dict(vec![]);
    v_number = { let v_number = v_number.clone(); let v_out = v_out.clone(); let v_parts = v_parts.clone(); let v_s = v_s.clone(); let v_text = v_text.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_j: Py = Py::None;
            v_j = v_i.clone();
            while ({ let t3444 = { let t3442 = v_j.clone(); Py::Bool({ let t3443 = py::len(&v_t.clone())?; if py::lt(&t3442, &t3443)? { true } else { false } }) }; if !t3444.truthy() { t3444 } else { py::method(&py::getitem(&v_t.clone(), &v_j.clone())?, "isdigit", &[])? } }).truthy() {
                v_j = py::iadd(&v_j, &Py::Int(1i64))?;
            }
            return Ok(Py::tuple(vec![(if ({ let t3445 = v_j.clone(); Py::Bool({ let t3446 = v_i.clone(); if py::gt(&t3445, &t3446)? { true } else { false } }) }).truthy() { py::int(&py::slice(&v_t.clone(), Some((v_i.clone()).i()?), Some((v_j.clone()).i()?), None)?)? } else { Py::Int(1i64) }), v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec3447 = py::rec_cell();
    v_seq = { let v_number = v_number.clone(); let v_out = v_out.clone(); let v_parts = v_parts.clone(); let v_s = v_s.clone(); let v_text = v_text.clone(); let rec3447 = rec3447.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_close: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_seq: Py = py::rec_get(&rec3447)?;
            let mut v_acc: Py = Py::None;
            let mut v_inner: Py = Py::None;
            let mut v_k: Py = Py::None;
            let mut v_n: Py = Py::None;
            let mut v_sym: Py = Py::None;
            let mut v_v: Py = Py::None;
            v_acc = Py::dict(vec![]);
            while ({ let t3462 = { let t3458 = v_i.clone(); Py::Bool({ let t3459 = py::len(&v_t.clone())?; if py::lt(&t3458, &t3459)? { true } else { false } }) }; if !t3462.truthy() { t3462 } else { { let t3460 = py::getitem(&v_t.clone(), &v_i.clone())?; Py::Bool({ let t3461 = Py::str(")]}"); if !py::contains(&t3461, &t3460)? { true } else { false } }) } } }).truthy() {
                if ({ let t3448 = py::getitem(&v_t.clone(), &v_i.clone())?; Py::Bool({ let t3449 = Py::str("([{"); if py::contains(&t3449, &t3448)? { true } else { false } }) }).truthy() {
                    let t3450 = (py::call(&v_seq.clone(), &[v_t.clone(), py::add(&v_i.clone(), &Py::Int(1i64))?, py::getitem(&c__PAIRS()?, &py::getitem(&v_t.clone(), &v_i.clone())?)?])?).items()?;
                    if t3450.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_inner = t3450[0].clone();
                    v_i = t3450[1].clone();
                    v_i = py::iadd(&v_i, &Py::Int(1i64))?;
                    let t3451 = (py::call(&v_number.clone(), &[v_t.clone(), v_i.clone()])?).items()?;
                    if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_n = t3451[0].clone();
                    v_i = t3451[1].clone();
                    for t3452 in py::iter(&(py::method(&v_inner.clone(), "items", &[])?))? {
                        let t3453 = (t3452).items()?;
                        if t3453.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_k = t3453[0].clone();
                        v_v = t3453[1].clone();
                        py::setitem(&v_acc.clone(), &v_k.clone(), py::add(&py::method(&v_acc.clone(), "get", &[v_k.clone(), Py::Int(0i64)])?, &py::mul(&v_v.clone(), &v_n.clone())?)?)?;
                    }
                } else {
                    v_sym = py::add(&py::getitem(&v_t.clone(), &v_i.clone())?, &(if ({ let t3456 = { let t3454 = py::add(&v_i.clone(), &Py::Int(1i64))?; Py::Bool({ let t3455 = py::len(&v_t.clone())?; if py::lt(&t3454, &t3455)? { true } else { false } }) }; if !t3456.truthy() { t3456 } else { py::method(&py::getitem(&v_t.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, "islower", &[])? } }).truthy() { py::getitem(&v_t.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)? } else { Py::str("") }))?;
                    v_i = py::iadd(&v_i, &py::len(&v_sym.clone())?)?;
                    let t3457 = (py::call(&v_number.clone(), &[v_t.clone(), v_i.clone()])?).items()?;
                    if t3457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_n = t3457[0].clone();
                    v_i = t3457[1].clone();
                    py::setitem(&v_acc.clone(), &v_sym.clone(), py::add(&py::method(&v_acc.clone(), "get", &[v_sym.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
                }
            }
            return Ok(Py::tuple(vec![v_acc.clone(), v_i.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3447, &v_seq);
    for t3463 in py::iter(&(v_parts.clone()))? {
        v_p = t3463;
        let t3464 = ((if (py::method(&py::slice(&v_p.clone(), None, Some((Py::Int(1i64)).i()?), None)?, "isdigit", &[])?).truthy() { py::call(&v_number.clone(), &[v_p.clone(), Py::Int(0i64)])? } else { Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]) })).items()?;
        if t3464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mult = t3464[0].clone();
        v_i = t3464[1].clone();
        let t3465 = (py::call(&v_seq.clone(), &[v_p.clone(), v_i.clone(), Py::None])?).items()?;
        if t3465.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_inner = t3465[0].clone();
        v__ = t3465[1].clone();
        for t3466 in py::iter(&(py::method(&v_inner.clone(), "items", &[])?))? {
            let t3467 = (t3466).items()?;
            if t3467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_k = t3467[0].clone();
            v_v = t3467[1].clone();
            py::setitem(&v_out.clone(), &v_k.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_k.clone(), Py::Int(0i64)])?, &py::mul(&v_v.clone(), &v_mult.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ee_comp_word(mut v_R: Py, mut v_S: Py) -> R {
    if ({ let t733 = v_R.clone(); Py::Bool({ let t734 = v_S.clone(); if py::gt(&t733, &t734)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("R"));
    }
    if ({ let t735 = v_S.clone(); Py::Bool({ let t736 = v_R.clone(); if py::gt(&t735, &t736)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("S"));
    }
    return Ok(Py::str("neither (racemic)"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ee_word(mut v_r: Py) -> R {
    if ({ let t746 = v_r.clone(); Py::Bool({ let t747 = Py::Int(0i64); if py::gt(&t746, &t747)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("the same enantiomer as the pure sample"));
    }
    if ({ let t748 = v_r.clone(); Py::Bool({ let t749 = Py::Int(0i64); if py::lt(&t748, &t749)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("the opposite enantiomer"));
    }
    return Ok(Py::str("neither (racemic)"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__element(mut v_text: Py) -> R {
    let mut v_t: Py = Py::None;
    if (py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_))))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an element is text, such as 'O' or 'oxygen'"))));
    }
    v_t = py::method(&v_text.clone(), "strip", &[])?;
    if ({ let t1720 = v_t.clone(); Py::Bool({ let t1721 = c_W()?; if py::contains(&t1721, &t1720)? { true } else { false } }) }).truthy() {
        return Ok(v_t.clone());
    }
    if ({ let t1722 = py::method(&v_t.clone(), "lower", &[])?; Py::Bool({ let t1723 = c_ELEMENT_NAMES()?; if py::contains(&t1723, &t1722)? { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&c_ELEMENT_NAMES()?, &py::method(&v_t.clone(), "lower", &[])?)?);
    }
    return Err(py::value_error(py::to_str(&Py::str(&["'".to_string(), py::format(&v_text.clone(), "")?, "' is not an element symbol or name".to_string()].concat()))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__element_matrix(mut v_reactants: Py, mut v_products: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_elements: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_species: Py = Py::None;
    v_species = py::add(&{ let mut out = vec![]; for t3375 in py::iter(&(v_reactants.clone()))? { let mut v__c: Py; let mut v_f: Py; 
let t3376 = (t3375).items()?;
if t3376.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t3376[0].clone();
v_f = t3376[1].clone();
out.push(v_f.clone()); } Py::list(out) }, &{ let mut out = vec![]; for t3377 in py::iter(&(v_products.clone()))? { let mut v__c: Py; let mut v_f: Py; 
let t3378 = (t3377).items()?;
if t3378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t3378[0].clone();
v_f = t3378[1].clone();
out.push(v_f.clone()); } Py::list(out) })?;
    v_counts = { let mut out = vec![]; for t3379 in py::iter(&(v_species.clone()))? { let mut v_f: Py; 
v_f = t3379;
out.push(f_formula_counts(v_f.clone())?); } Py::list(out) };
    v_elements = py::sorted(&py::set(&{ let mut out = vec![]; for t3380 in py::iter(&(v_counts.clone()))? { let mut v_c: Py; 
v_c = t3380;
for t3381 in py::iter(&(v_c.clone()))? { let mut v_e: Py; 
v_e = t3381;
out.push(v_e.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?;
    v_nr = py::len(&v_reactants.clone())?;
    v_rows = { let mut out = vec![]; for t3382 in py::iter(&(v_elements.clone()))? { let mut v_e: Py; 
v_e = t3382;
out.push({ let mut out = vec![]; for t3383 in py::iter(&(py::enumerate(&v_counts.clone(), 0)?))? { let mut v_c: Py; let mut v_j: Py; 
let t3384 = (t3383).items()?;
if t3384.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t3384[0].clone();
v_c = t3384[1].clone();
out.push(py::fraction(&[py::mul(&py::method(&v_c.clone(), "get", &[v_e.clone(), Py::Int(0i64)])?, &(if ({ let t3385 = v_j.clone(); Py::Bool({ let t3386 = v_nr.clone(); if py::lt(&t3385, &t3386)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?])?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::tuple(vec![v_rows.clone(), v_species.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3537 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t3539 = (t3537).items()?;
        if t3539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3539[0].clone();
        v_v = t3539[1].clone();
        if (py::not(&{ let t3538 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3538.truthy() { t3538 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gibbs_word(mut v_g: Py) -> R {
    if ({ let t3084 = v_g.clone(); Py::Bool({ let t3085 = Py::Int(0i64); if py::lt(&t3084, &t3085)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("spontaneous"));
    }
    if ({ let t3086 = v_g.clone(); Py::Bool({ let t3087 = Py::Int(0i64); if py::gt(&t3086, &t3087)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("non-spontaneous"));
    }
    return Ok(Py::str("at equilibrium"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__group_counts(mut v_s: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_opens: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_stack: Py = Py::None;
    let mut v_top: Py = Py::None;
    let t3550 = (Py::tuple(vec![Py::list(vec![Py::dict(vec![])]), Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t3550.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_stack = t3550[0].clone();
    v_opens = t3550[1].clone();
    v_pos = t3550[2].clone();
    while ({ let t3564 = v_pos.clone(); Py::Bool({ let t3565 = py::len(&v_s.clone())?; if py::lt(&t3564, &t3565)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__FTOK()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if (py::not(&v_m.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the formula at '".to_string(), py::format(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), None, None)?, "")?, "'".to_string()].concat()))));
        }
        if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() {
            v_el = py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?;
            if ({ let t3551 = v_el.clone(); Py::Bool({ let t3552 = c_W()?; if !py::contains(&t3552, &t3551)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["'".to_string(), py::format(&v_el.clone(), "")?, "' is not an element symbol with a standard atomic weight".to_string()].concat()))));
            }
            v_n = (if (py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)? } else { Py::Int(1i64) });
            if ({ let t3553 = v_n.clone(); Py::Bool({ let t3554 = Py::Int(0i64); if py::eq(&t3553, &t3554) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a subscript of 0 is not allowed"))));
            }
            py::setitem(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, &v_el.clone(), py::add(&py::method(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, "get", &[v_el.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
        } else {
            if (py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?).truthy() {
                let _ = py::method(&v_stack.clone(), "append", &[Py::dict(vec![])])?;
                let _ = py::method(&v_opens.clone(), "append", &[py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?])?;
            } else {
                if ({ let t3559 = { let t3555 = py::len(&v_stack.clone())?; Py::Bool({ let t3556 = Py::Int(1i64); if py::eq(&t3555, &t3556) { true } else { false } }) }; if t3559.truthy() { t3559 } else { { let t3557 = py::getitem(&c__PAIRS()?, &py::getitem(&v_opens.clone(), &py::neg(&Py::Int(1i64))?)?)?; Py::Bool({ let t3558 = py::method(&v_m.clone(), "group", &[Py::Int(4i64)])?; if py::ne(&t3557, &t3558) { true } else { false } }) } } }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced brackets in the formula"))));
                }
                let _ = py::method(&v_opens.clone(), "pop", &[])?;
                v_n = (if (py::method(&v_m.clone(), "group", &[Py::Int(5i64)])?).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(5i64)])?)? } else { Py::Int(1i64) });
                if ({ let t3560 = v_n.clone(); Py::Bool({ let t3561 = Py::Int(0i64); if py::eq(&t3560, &t3561) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("a subscript of 0 is not allowed"))));
                }
                v_top = py::method(&v_stack.clone(), "pop", &[])?;
                if (py::not(&v_top.clone())).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("empty brackets in the formula"))));
                }
                for t3562 in py::iter(&(py::method(&v_top.clone(), "items", &[])?))? {
                    let t3563 = (t3562).items()?;
                    if t3563.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_el = t3563[0].clone();
                    v_c = t3563[1].clone();
                    py::setitem(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, &v_el.clone(), py::add(&py::method(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, "get", &[v_el.clone(), Py::Int(0i64)])?, &py::mul(&v_c.clone(), &v_n.clone())?)?)?;
                }
            }
        }
        v_pos = py::method(&v_m.clone(), "end", &[])?;
    }
    if ({ let t3568 = { let t3566 = py::len(&v_stack.clone())?; Py::Bool({ let t3567 = Py::Int(1i64); if py::ne(&t3566, &t3567) { true } else { false } }) }; if t3568.truthy() { t3568 } else { py::not(&py::getitem(&v_stack.clone(), &Py::Int(0i64))?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("unbalanced brackets or an empty formula"))));
    }
    return Ok(py::getitem(&v_stack.clone(), &Py::Int(0i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ice_bounds(mut v_nu: Py, mut v_c0: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    v_lo = py::minmax(&[{ let mut out = vec![]; for t817 in py::iter(&(py::zip(&[v_c0.clone(), v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t818 = (t817).items()?;
if t818.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t818[0].clone();
v_n = t818[1].clone();
if ({ let t819 = v_n.clone(); Py::Bool({ let t820 = Py::Int(0i64); if py::gt(&t819, &t820)? { true } else { false } }) }).truthy() { out.push(py::truediv(&py::neg(&v_c.clone())?, &v_n.clone())?); }} Py::list(out) }], true)?;
    v_hi = py::minmax(&[{ let mut out = vec![]; for t821 in py::iter(&(py::zip(&[v_c0.clone(), v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t822 = (t821).items()?;
if t822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t822[0].clone();
v_n = t822[1].clone();
if ({ let t823 = v_n.clone(); Py::Bool({ let t824 = Py::Int(0i64); if py::lt(&t823, &t824)? { true } else { false } }) }).truthy() { out.push(py::truediv(&v_c.clone(), &py::neg(&v_n.clone())?)?); }} Py::list(out) }], false)?;
    if (py::not(&{ let t825 = v_lo.clone(); Py::Bool({ let t826 = v_hi.clone(); if py::lt(&t825, &t826)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("no reaction can take place from these initial concentrations"))));
    }
    return Ok(Py::tuple(vec![v_lo.clone(), v_hi.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ice_terms(mut v_equation: Py) -> R {
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    if (ax_engine::pyre::module_call("search", &Py::str("\\((?:s|l)\\)"), &[v_equation.clone()], 0)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("pure solids and liquids do not enter K: write the equation without them"))));
    }
    let t3136 = (f_parse_equation(v_equation.clone())?).items()?;
    if t3136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t3136[0].clone();
    v_p = t3136[1].clone();
    return Ok(Py::tuple(vec![py::add(&{ let mut out = vec![]; for t3137 in py::iter(&(v_r.clone()))? { let mut v__f: Py; let mut v_c: Py; 
let t3138 = (t3137).items()?;
if t3138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t3138[0].clone();
v__f = t3138[1].clone();
out.push(py::neg(&py::float(&v_c.clone())?)?); } Py::list(out) }, &{ let mut out = vec![]; for t3139 in py::iter(&(v_p.clone()))? { let mut v__f: Py; let mut v_c: Py; 
let t3140 = (t3139).items()?;
if t3140.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t3140[0].clone();
v__f = t3140[1].clone();
out.push(py::float(&v_c.clone())?); } Py::list(out) })?, { let mut out = vec![]; for t3141 in py::iter(&(py::add(&v_r.clone(), &v_p.clone())?))? { let mut v__c: Py; let mut v_f: Py; 
let t3142 = (t3141).items()?;
if t3142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t3142[0].clone();
v_f = t3142[1].clone();
out.push(v_f.clone()); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ig_moles(mut v_P: Py, mut v_V: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("V"), v_V.clone()), (Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("n"), py::truediv(&py::mul(&v_P.clone(), &v_V.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ig_pressure(mut v_n: Py, mut v_V: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("V"), v_V.clone()), (Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("P"), py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R()?)?, &v_T.clone())?, &v_V.clone())?), (Py::str("P_atm"), py::truediv(&py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R()?)?, &v_T.clone())?, &v_V.clone())?, &c_ATM()?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ig_temperature(mut v_P: Py, mut v_V: Py, mut v_n: Py) -> R {
    let mut v_T: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("V"), v_V.clone()), (Py::str("n"), v_n.clone())]))?;
    v_T = py::truediv(&py::mul(&v_P.clone(), &v_V.clone())?, &py::mul(&v_n.clone(), &c_R()?)?)?;
    return Ok(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("T_C"), py::sub(&v_T.clone(), &Py::Float(273.15))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ig_volume(mut v_n: Py, mut v_P: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("P"), v_P.clone()), (Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("V"), py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R()?)?, &v_T.clone())?, &v_P.clone())?), (Py::str("V_L"), py::mul(&py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R()?)?, &v_T.clone())?, &v_P.clone())?, &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lnq(mut v_nu: Py, mut v_c0: Py, mut v_x: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_s = Py::Float(0.0);
    for t845 in py::iter(&(py::zip(&[v_c0.clone(), v_nu.clone()])?))? {
        let t850 = (t845).items()?;
        if t850.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t850[0].clone();
        v_n = t850[1].clone();
        v_v = py::add(&v_c.clone(), &py::mul(&v_n.clone(), &v_x.clone())?)?;
        if ({ let t846 = v_v.clone(); Py::Bool({ let t847 = Py::Int(0i64); if py::le(&t846, &t847)? { true } else { false } }) }).truthy() {
            return Ok((if ({ let t848 = v_n.clone(); Py::Bool({ let t849 = Py::Int(0i64); if py::gt(&t848, &t849)? { true } else { false } }) }).truthy() { py::neg(&Py::Float(f64::INFINITY))? } else { Py::Float(f64::INFINITY) }));
        }
        v_s = py::iadd(&v_s, &py::mul(&v_n.clone(), &py::math1("log", &v_v.clone())?)?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mm_rd(mut v_text: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t3468 in py::iter(&(py::method(&f__counts_rd(v_text.clone())?, "items", &[])?))? { let mut v_el: Py; let mut v_n: Py; 
let t3469 = (t3468).items()?;
if t3469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_el = t3469[0].clone();
v_n = t3469[1].clone();
out.push(py::mul(&v_n.clone(), &py::getitem(&c_W()?, &v_el.clone())?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nullspace_1(mut v_rows: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_fc: Py = Py::None;
    let mut v_free: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_ints: Py = Py::None;
    let mut v_lcm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_pivots: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_m = { let mut out = vec![]; for t3388 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t3388;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
    let t3389 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t3389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pivots = t3389[0].clone();
    v_r = t3389[1].clone();
    for t3390 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t3390;
        v_p = (match ('g3391: { let mut out = vec![]; for t3392 in py::range_iter(&[(v_r.clone()).i()?, (py::len(&v_m.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3392;
if ({ let t3393 = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_c.clone())?; Py::Bool({ let t3394 = Py::Int(0i64); if py::ne(&t3393, &t3394) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g3391 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t3395 = v_p.clone(); Py::Bool({ let t3396 = Py::None; if py::is(&t3395, &t3396) { true } else { false } }) }).truthy() {
            continue;
        }
        let t3397 = (Py::tuple(vec![py::getitem(&v_m.clone(), &v_p.clone())?, py::getitem(&v_m.clone(), &v_r.clone())?])).items()?;
        if t3397.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_m.clone(), &v_r.clone(), t3397[0].clone())?;
        py::setitem(&v_m.clone(), &v_p.clone(), t3397[1].clone())?;
        v_piv = py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_c.clone())?;
        py::setitem(&v_m.clone(), &v_r.clone(), { let mut out = vec![]; for t3398 in py::iter(&(py::getitem(&v_m.clone(), &v_r.clone())?))? { let mut v_x: Py; 
v_x = t3398;
out.push(py::truediv(&v_x.clone(), &v_piv.clone())?); } Py::list(out) })?;
        for t3399 in py::range_iter(&[(py::len(&v_m.clone())?).i()?]) {
            v_i = t3399;
            if ({ let t3404 = { let t3400 = v_i.clone(); Py::Bool({ let t3401 = v_r.clone(); if py::ne(&t3400, &t3401) { true } else { false } }) }; if !t3404.truthy() { t3404 } else { { let t3402 = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_c.clone())?; Py::Bool({ let t3403 = Py::Int(0i64); if py::ne(&t3402, &t3403) { true } else { false } }) } } }).truthy() {
                v_f = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_c.clone())?;
                py::setitem(&v_m.clone(), &v_i.clone(), { let mut out = vec![]; for t3405 in py::iter(&(py::zip(&[py::getitem(&v_m.clone(), &v_i.clone())?, py::getitem(&v_m.clone(), &v_r.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3406 = (t3405).items()?;
if t3406.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3406[0].clone();
v_b = t3406[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
        let _ = py::method(&v_pivots.clone(), "append", &[v_c.clone()])?;
        v_r = py::iadd(&v_r, &Py::Int(1i64))?;
    }
    v_free = { let mut out = vec![]; for t3407 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t3407;
if ({ let t3408 = v_c.clone(); Py::Bool({ let t3409 = v_pivots.clone(); if !py::contains(&t3409, &t3408)? { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) };
    if ({ let t3410 = py::len(&v_free.clone())?; Py::Bool({ let t3411 = Py::Int(1i64); if py::ne(&t3410, &t3411) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation cannot be balanced uniquely (it has no solution or several independent ones)"))));
    }
    v_fc = py::getitem(&v_free.clone(), &Py::Int(0i64))?;
    v_v = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    py::setitem(&v_v.clone(), &v_fc.clone(), py::fraction(&[Py::Int(1i64)])?)?;
    for t3412 in py::iter(&(py::enumerate(&v_pivots.clone(), 0)?))? {
        let t3413 = (t3412).items()?;
        if t3413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3413[0].clone();
        v_pc = t3413[1].clone();
        py::setitem(&v_v.clone(), &v_pc.clone(), py::neg(&py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_fc.clone())?)?)?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t3414 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3414;
out.push({ let t3415 = v_x.clone(); Py::Bool({ let t3416 = Py::Int(0i64); if py::eq(&t3415, &t3416) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation cannot be balanced with every species taking part"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t3417 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3417;
out.push({ let t3418 = v_x.clone(); Py::Bool({ let t3419 = Py::Int(0i64); if py::lt(&t3418, &t3419)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        v_v = { let mut out = vec![]; for t3420 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3420;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
    }
    if (Py::Bool(({ let mut out = vec![]; for t3421 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3421;
out.push({ let t3422 = v_x.clone(); Py::Bool({ let t3423 = Py::Int(0i64); if py::lt(&t3422, &t3423)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation cannot be balanced with positive coefficients"))));
    }
    v_lcm = Py::Int(1i64);
    for t3424 in py::iter(&(v_v.clone()))? {
        v_x = t3424;
        v_lcm = py::floordiv(&py::mul(&v_lcm.clone(), &py::denominator(&v_x.clone())?)?, &py::gcd(&v_lcm.clone(), &py::denominator(&v_x.clone())?)?)?;
    }
    v_ints = { let mut out = vec![]; for t3425 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3425;
out.push(py::int(&py::mul(&v_x.clone(), &v_lcm.clone())?)?); } Py::list(out) };
    v_g = Py::Int(0i64);
    for t3426 in py::iter(&(v_ints.clone()))? {
        v_x = t3426;
        v_g = py::gcd(&v_g.clone(), &v_x.clone())?;
    }
    return Ok({ let mut out = vec![]; for t3427 in py::iter(&(v_ints.clone()))? { let mut v_x: Py; 
v_x = t3427;
out.push(py::floordiv(&v_x.clone(), &v_g.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ph(mut v_h_molL: Py) -> R {
    return Ok(py::neg(&py::math1("log10", &v_h_molL.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3586 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t3591 = (t3586).items()?;
        if t3591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3591[0].clone();
        v_v = t3591[1].clone();
        if ({ let t3590 = py::not(&{ let t3587 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3587.truthy() { t3587 } else { py::math1("isfinite", &v_v.clone())? } }); if t3590.truthy() { t3590 } else { { let t3588 = v_v.clone(); Py::Bool({ let t3589 = Py::Int(0i64); if py::le(&t3588, &t3589)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a positive number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__precip_word(mut v_lgQ: Py, mut v_lgK: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = py::sub(&v_lgQ.clone(), &v_lgK.clone())?;
    if ({ let t2719 = py::abs(&v_d.clone())?; Py::Bool({ let t2720 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_lgK.clone())?], true)?)?; if py::le(&t2719, &t2720)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("saturated (Q = Ksp)"));
    }
    if ({ let t2721 = v_d.clone(); Py::Bool({ let t2722 = Py::Int(0i64); if py::gt(&t2721, &t2722)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("precipitate forms (Q > Ksp)"));
    }
    return Ok(Py::str("no precipitate (Q < Ksp)"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_s: Py = Py::None;
    let t3595 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
    if t3595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3595[0].clone();
    v_b = t3595[1].clone();
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t3596 = v_s.clone(); Py::Bool({ let t3597 = Py::Int(0i64); if py::eq(&t3596, &t3597) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &v_s.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__role(mut v_species: Py, mut v_nr: Py, mut v_formula: Py) -> R {
    let mut v_j: Py = Py::None;
    if ({ let t3429 = v_formula.clone(); Py::Bool({ let t3430 = v_species.clone(); if !py::contains(&t3430, &t3429)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_formula.clone(), "")?, " does not appear in the equation".to_string()].concat()))));
    }
    v_j = py::method(&v_species.clone(), "index", &[v_formula.clone()])?;
    return Ok(Py::tuple(vec![v_j.clone(), { let t3431 = v_j.clone(); Py::Bool({ let t3432 = v_nr.clone(); if py::lt(&t3431, &t3432)? { true } else { false } }) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shift_word(mut v_lgQ: Py, mut v_lgK: Py) -> R {
    if ({ let t3157 = py::abs(&py::sub(&v_lgQ.clone(), &v_lgK.clone())?)?; Py::Bool({ let t3158 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_lgK.clone())?], true)?)?; if py::le(&t3157, &t3158)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("at equilibrium"));
    }
    if ({ let t3159 = v_lgQ.clone(); Py::Bool({ let t3160 = v_lgK.clone(); if py::lt(&t3159, &t3160)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("forward (toward products)"));
    }
    return Ok(Py::str("reverse (toward reactants)"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__strong_strong(mut v_c_a: Py, mut v_V_a: Py, mut v_c_b: Py, mut v_V_b: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c_a"), v_c_a.clone()), (Py::str("V_a"), v_V_a.clone()), (Py::str("c_b"), v_c_b.clone())]))?;
    if ({ let t3486 = v_V_b.clone(); Py::Bool({ let t3487 = Py::Int(0i64); if py::lt(&t3486, &t3487)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the added volume cannot be negative"))));
    }
    v_D = py::truediv(&py::truediv(&py::sub(&py::mul(&v_c_a.clone(), &v_V_a.clone())?, &py::mul(&v_c_b.clone(), &v_V_b.clone())?)?, &py::add(&v_V_a.clone(), &v_V_b.clone())?)?, &Py::Float(1000.0))?;
    v_h = (if ({ let t3488 = v_D.clone(); Py::Bool({ let t3489 = Py::Int(0i64); if py::ge(&t3488, &t3489)? { true } else { false } }) }).truthy() { py::add(&py::truediv(&v_D.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_D.clone(), &v_D.clone())?, &Py::Int(4i64))?, &c_KW()?)?)?)? } else { py::truediv(&py::mul(&Py::Int(2i64), &c_KW()?)?, &py::add(&py::neg(&v_D.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&Py::Int(4i64), &c_KW()?)?)?)?)?)? });
    return Ok(Py::dict(vec![(Py::str("pH"), f__ph(v_h.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_balance(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_products: Py = Py::None;
    let mut v_reactants: Py = Py::None;
    let mut v_species: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t217 = (f_parse_equation(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?;
    if t217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reactants = t217[0].clone();
    v_products = t217[1].clone();
    v_species = { let mut out = vec![]; for t218 in py::iter(&(py::add(&v_reactants.clone(), &v_products.clone())?))? { let mut v__c: Py; let mut v_f: Py; 
let t219 = (t218).items()?;
if t219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t219[0].clone();
v_f = t219[1].clone();
out.push(v_f.clone()); } Py::list(out) };
    v_c = py::getitem(&v_o.clone(), &Py::str("coefficients"))?;
    if ({ let t228 = { let t220 = py::len(&v_c.clone())?; Py::Bool({ let t221 = py::len(&v_species.clone())?; if py::ne(&t220, &t221) { true } else { false } }) }; if t228.truthy() { t228 } else { Py::Bool(({ let mut out = vec![]; for t222 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t222;
out.push({ let t227 = { let t223 = v_x.clone(); Py::Bool({ let t224 = Py::Int(0i64); if py::le(&t223, &t224)? { true } else { false } }) }; if t227.truthy() { t227 } else { { let t225 = v_x.clone(); Py::Bool({ let t226 = py::int(&v_x.clone())?; if py::ne(&t225, &t226) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_c = { let mut out = vec![]; for t229 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t229;
out.push(py::int(&v_x.clone())?); } Py::list(out) };
    v_g = Py::Int(0i64);
    for t230 in py::iter(&(v_c.clone()))? {
        v_x = t230;
        v_g = py::gcd(&v_g.clone(), &v_x.clone())?;
    }
    return Ok((if ({ let t269 = { let t231 = v_g.clone(); Py::Bool({ let t232 = Py::Int(1i64); if py::eq(&t231, &t232) { true } else { false } }) }; if !t269.truthy() { t269 } else { f__conserved(v_c.clone(), v_species.clone(), py::len(&v_reactants.clone())?)? } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_binding(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f_nuclear_binding_energy(py::getitem(&v_i.clone(), &Py::str("Z"))?, py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("mass"))?)?;
    return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("E_b_MeV"))?, py::getitem(&v_r.clone(), &Py::str("E_b_MeV"))?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("E_b"))?, py::getitem(&v_r.clone(), &Py::str("E_b"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("E_per_nucleon_MeV"))?, py::getitem(&v_r.clone(), &Py::str("E_per_nucleon_MeV"))?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("dm"))?, py::getitem(&v_r.clone(), &Py::str("dm"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_bonds(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_Fm: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_B = f__bond_sum(py::getitem(&v_i.clone(), &Py::str("broken_n"))?, py::getitem(&v_i.clone(), &Py::str("broken_D"))?)?;
    v_Fm = f__bond_sum(py::getitem(&v_i.clone(), &Py::str("formed_n"))?, py::getitem(&v_i.clone(), &Py::str("formed_D"))?)?;
    v_s = py::minmax(&[Py::Float(1.0), py::abs(&py::sub(&v_B.clone(), &v_Fm.clone())?)?], true)?;
    return Ok(py::truediv(&py::add(&py::add(&py::add(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dH_kJ"))?, &py::sub(&v_B.clone(), &v_Fm.clone())?)?)?, &py::abs(&py::sub(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dH"))?, &Py::Float(1000.0))?, &py::sub(&v_B.clone(), &v_Fm.clone())?)?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("broken_kJ"))?, &v_B.clone())?)?)?, &py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("formed_kJ"))?, &v_Fm.clone())?)?)?, &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ee_comp(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_tot: Py = Py::None;
    v_tot = py::add(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("S"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("ee"))?, &py::truediv(&py::mul(&Py::Float(100.0), &py::abs(&py::sub(&py::getitem(&v_i.clone(), &Py::str("R"))?, &py::getitem(&v_i.clone(), &Py::str("S"))?)?)?)?, &v_tot.clone())?)?)?, &Py::Float(100.0))?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("percent_R"))?, &py::truediv(&py::mul(&Py::Float(100.0), &py::getitem(&v_i.clone(), &Py::str("R"))?)?, &v_tot.clone())?)?)?, &Py::Float(100.0))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("percent_S"))?, &py::truediv(&py::mul(&Py::Float(100.0), &py::getitem(&v_i.clone(), &Py::str("S"))?)?, &v_tot.clone())?)?)?, &Py::Float(100.0))?)?, &(if ({ let t737 = py::getitem(&v_o.clone(), &Py::str("predominant"))?; Py::Bool({ let t738 = f__ee_comp_word(py::getitem(&v_i.clone(), &Py::str("R"))?, py::getitem(&v_i.clone(), &Py::str("S"))?)?; if py::eq(&t737, &t738) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ee_rot(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = py::truediv(&py::getitem(&v_i.clone(), &Py::str("alpha_obs"))?, &py::getitem(&v_i.clone(), &Py::str("alpha_pure"))?)?;
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("ee"))?, &py::mul(&Py::Float(100.0), &py::abs(&v_r.clone())?)?)?)?, &Py::Float(100.0))?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("major_percent"))?, &Py::Float(50.0))?, &py::mul(&Py::Float(50.0), &py::abs(&v_r.clone())?)?)?)?, &Py::Float(100.0))?)?, &py::truediv(&py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("minor_percent"))?, &Py::Float(50.0))?, &py::mul(&Py::Float(50.0), &py::abs(&v_r.clone())?)?)?)?, &Py::Float(100.0))?)?, &(if ({ let t750 = py::getitem(&v_o.clone(), &Py::str("predominant"))?; Py::Bool({ let t751 = f__ee_word(v_r.clone())?; if py::eq(&t750, &t751) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_empirical(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_dev: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_tot_given: Py = Py::None;
    let mut v_tot_mass: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_comp = f__composition(py::getitem(&v_i.clone(), &Py::str("composition"))?)?;
    v_counts = f__counts_rd(py::getitem(&v_o.clone(), &Py::str("formula"))?)?;
    if ({ let t714 = py::set(&v_counts.clone())?; Py::Bool({ let t715 = py::set(&v_comp.clone())?; if py::ne(&t714, &t715) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_g = Py::Int(0i64);
    for t716 in py::iter(&(py::method(&v_counts.clone(), "values", &[])?))? {
        v_v = t716;
        v_g = py::gcd(&v_g.clone(), &v_v.clone())?;
    }
    if ({ let t717 = v_g.clone(); Py::Bool({ let t718 = Py::Int(1i64); if py::ne(&t717, &t718) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_tot_given = py::sum(&py::method(&v_comp.clone(), "values", &[])?, None)?;
    v_tot_mass = py::sum(&{ let mut out = vec![]; for t719 in py::iter(&(v_counts.clone()))? { let mut v_e: Py; 
v_e = t719;
out.push(py::mul(&py::getitem(&v_counts.clone(), &v_e.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?); } Py::list(out) }, None)?;
    v_dev = py::minmax(&[{ let mut out = vec![]; for t720 in py::iter(&(v_comp.clone()))? { let mut v_e: Py; 
v_e = t720;
out.push(py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_counts.clone(), &v_e.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?, &v_tot_mass.clone())?, &py::truediv(&py::getitem(&v_comp.clone(), &v_e.clone())?, &v_tot_given.clone())?)?)?, &py::truediv(&py::getitem(&v_comp.clone(), &v_e.clone())?, &v_tot_given.clone())?)?); } Py::list(out) }], true)?;
    return Ok((if ({ let t721 = v_dev.clone(); Py::Bool({ let t722 = Py::Float(0.03); if py::le(&t721, &t722)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gibbs_formation(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_g = f__coef_sum(py::getitem(&v_i.clone(), &Py::str("equation"))?, py::getitem(&v_i.clone(), &Py::str("values"))?)?;
    v_s = py::minmax(&[Py::Float(1.0), f__abs_sum(py::getitem(&v_i.clone(), &Py::str("values"))?)?], true)?;
    return Ok(py::add(&py::truediv(&py::add(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dG_kJ"))?, &v_g.clone())?)?, &py::abs(&py::sub(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &Py::Float(1000.0))?, &v_g.clone())?)?)?, &v_s.clone())?, &(if ({ let t3089 = py::getitem(&v_o.clone(), &Py::str("spontaneity"))?; Py::Bool({ let t3090 = f__gibbs_word(v_g.clone())?; if py::eq(&t3089, &t3090) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_ice(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_eq: Py = Py::None;
    let mut v_given: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t853 = (f__ice_terms(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?;
    if t853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nu = t853[0].clone();
    v_sp = t853[1].clone();
    v_x = py::getitem(&v_o.clone(), &Py::str("x"))?;
    v_eq = { let mut out = vec![]; for t854 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("initial"))?, v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t855 = (t854).items()?;
if t855.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t855[0].clone();
v_n = t855[1].clone();
out.push(py::add(&v_c.clone(), &py::mul(&v_n.clone(), &v_x.clone())?)?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t856 in py::iter(&(v_eq.clone()))? { let mut v_v: Py; 
v_v = t856;
out.push({ let t857 = v_v.clone(); Py::Bool({ let t858 = Py::Int(0i64); if py::le(&t857, &t858)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_given = py::method(&v_o.clone(), "get", &[Py::str("equilibrium"), v_eq.clone()])?;
    return Ok(py::minmax(&[py::abs(&py::sub(&f__lnq(v_nu.clone(), py::getitem(&v_i.clone(), &Py::str("initial"))?, v_x.clone())?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("K"))?)?)?)?, (if ({ let t859 = py::len(&v_given.clone())?; Py::Bool({ let t860 = py::len(&v_eq.clone())?; if py::eq(&t859, &t860) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t861 in py::iter(&(py::zip(&[v_given.clone(), v_eq.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t862 = (t861).items()?;
if t862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t862[0].clone();
v_b = t862[1].clone();
out.push(f__rel(v_a.clone(), v_b.clone())?); } Py::list(out) }], true)? } else { Py::Float(1.0) })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_limiting(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_ext: Py = Py::None;
    let mut v_ja: Py = Py::None;
    let mut v_jb: Py = Py::None;
    let mut v_jp: Py = Py::None;
    let mut v_na: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_species: Py = Py::None;
    let t1552 = (f_balance(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?;
    if t1552.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t1552[0].clone();
    v_species = t1552[1].clone();
    v_nr = t1552[2].clone();
    let t1553 = (Py::tuple(vec![py::method(&v_species.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("reactant_a"))?])?, py::method(&v_species.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("reactant_b"))?])?, py::method(&v_species.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("product"))?])?])).items()?;
    if t1553.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ja = t1553[0].clone();
    v_jb = t1553[1].clone();
    v_jp = t1553[2].clone();
    let t1582 = (Py::tuple(vec![py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m_a"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("reactant_a"))?)?)?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m_b"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("reactant_b"))?)?)?])).items()?;
    if t1582.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_na = t1582[0].clone();
    v_nb = t1582[1].clone();
    v_ext = py::minmax(&[py::truediv(&v_na.clone(), &py::getitem(&v_coefs.clone(), &v_ja.clone())?)?, py::truediv(&v_nb.clone(), &py::getitem(&v_coefs.clone(), &v_jb.clone())?)?], false)?;
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("product_mass"))?, py::truediv(&py::mul(&py::mul(&v_ext.clone(), &py::getitem(&v_coefs.clone(), &v_jp.clone())?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("product"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_precip(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f_precipitation_test(py::getitem(&v_i.clone(), &Py::str("Ksp"))?, py::getitem(&v_i.clone(), &Py::str("x"))?, py::getitem(&v_i.clone(), &Py::str("y"))?, py::getitem(&v_i.clone(), &Py::str("c_cation"))?, py::getitem(&v_i.clone(), &Py::str("c_anion"))?)?;
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::getitem(&v_r.clone(), &Py::str("Q"))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("log10_Q"))?, &py::getitem(&v_r.clone(), &Py::str("log10_Q"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_r.clone(), &Py::str("log10_Q"))?)?], true)?)?)?, &(if ({ let t2726 = py::getitem(&v_o.clone(), &Py::str("result"))?; Py::Bool({ let t2727 = py::getitem(&v_r.clone(), &Py::str("result"))?; if py::eq(&t2726, &t2727) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_quotient(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_r: Py = Py::None;
    v_r = f_reaction_quotient(py::getitem(&v_i.clone(), &Py::str("equation"))?, py::getitem(&v_i.clone(), &Py::str("conc"))?, py::getitem(&v_i.clone(), &Py::str("K"))?)?;
    return Ok(py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("Q"))?, py::getitem(&v_r.clone(), &Py::str("Q"))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("log10_Q"))?, &py::getitem(&v_r.clone(), &Py::str("log10_Q"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_r.clone(), &Py::str("log10_Q"))?)?], true)?)?)?, &(if ({ let t3164 = py::getitem(&v_o.clone(), &Py::str("direction"))?; Py::Bool({ let t3165 = py::getitem(&v_r.clone(), &Py::str("direction"))?; if py::eq(&t3164, &t3165) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__weak_h(mut v_Ka: Py, mut v_C: Py) -> R {
    let mut v_f: Py = Py::None;
    v_f = { let v_C = v_C.clone(); let v_Ka = v_Ka.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&{ let v_C = v_C.clone(); let v_Ka = v_Ka.clone(); let v_u = v_u.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&v_h.clone(), &py::truediv(&py::mul(&v_Ka.clone(), &v_C.clone())?, &py::add(&v_Ka.clone(), &v_h.clone())?)?)?, &py::truediv(&c_KW()?, &v_h.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::math1("exp", &v_u.clone())?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(py::math1("exp", &f__bisect(v_f.clone(), py::sub(&py::math1("log", &Py::Float(1e-07))?, &Py::Float(1e-09))?, py::math1("log", &py::add(&v_C.clone(), &Py::Float(1e-06))?)?, Py::Int(400i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__weak_titr(mut v_Ka: Py, mut v_c_a: Py, mut v_V_a: Py, mut v_c_b: Py, mut v_V_b: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Na: Py = Py::None;
    let mut v_Vt: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ka"), v_Ka.clone()), (Py::str("c_a"), v_c_a.clone()), (Py::str("V_a"), v_V_a.clone()), (Py::str("c_b"), v_c_b.clone())]))?;
    if ({ let t3497 = v_V_b.clone(); Py::Bool({ let t3498 = Py::Int(0i64); if py::lt(&t3497, &t3498)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the added volume cannot be negative"))));
    }
    v_Vt = py::add(&v_V_a.clone(), &v_V_b.clone())?;
    let t3499 = (Py::tuple(vec![py::truediv(&py::truediv(&py::mul(&v_c_a.clone(), &v_V_a.clone())?, &v_Vt.clone())?, &Py::Float(1000.0))?, py::truediv(&py::truediv(&py::mul(&v_c_b.clone(), &v_V_b.clone())?, &v_Vt.clone())?, &Py::Float(1000.0))?])).items()?;
    if t3499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3499[0].clone();
    v_Na = t3499[1].clone();
    v_f = { let v_C = v_C.clone(); let v_Ka = v_Ka.clone(); let v_Na = v_Na.clone(); let v_V_a = v_V_a.clone(); let v_V_b = v_V_b.clone(); let v_Vt = v_Vt.clone(); let v_c_a = v_c_a.clone(); let v_c_b = v_c_b.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&{ let v_C = v_C.clone(); let v_Ka = v_Ka.clone(); let v_Na = v_Na.clone(); let v_V_a = v_V_a.clone(); let v_V_b = v_V_b.clone(); let v_Vt = v_Vt.clone(); let v_c_a = v_c_a.clone(); let v_c_b = v_c_b.clone(); let v_u = v_u.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&py::add(&v_h.clone(), &v_Na.clone())?, &py::truediv(&c_KW()?, &v_h.clone())?)?, &py::truediv(&py::mul(&v_Ka.clone(), &v_C.clone())?, &py::add(&v_Ka.clone(), &v_h.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::math1("exp", &v_u.clone())?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("pH"), f__ph(py::math1("exp", &f__bisect(v_f.clone(), py::math1("log", &Py::Float(1e-20))?, py::math1("log", &Py::Float(10.0))?, Py::Int(400i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__write_formula(mut v_order: Py, mut v_counts: Py) -> R {
    return Ok(py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t2178 in py::iter(&(v_order.clone()))? { let mut v_el: Py; 
v_el = t2178;
if (py::method(&v_counts.clone(), "get", &[v_el.clone()])?).truthy() { out.push(py::add(&v_el.clone(), &(if ({ let t2179 = py::getitem(&v_counts.clone(), &v_el.clone())?; Py::Bool({ let t2180 = Py::Int(1i64); if py::ne(&t2179, &t2180) { true } else { false } }) }).truthy() { Py::str(&py::to_str(&py::getitem(&v_counts.clone(), &v_el.clone())?)) } else { Py::str("") }))?); }} Py::list(out) }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__written(mut v_coefs: Py, mut v_species: Py, mut v_nr: Py) -> R {
    let mut v_term: Py = Py::None;
    v_term = { let v_coefs = v_coefs.clone(); let v_nr = v_nr.clone(); let v_species = v_species.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_f: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::add(&(if ({ let t211 = v_c.clone(); Py::Bool({ let t212 = Py::Int(1i64); if py::ne(&t211, &t212) { true } else { false } }) }).truthy() { Py::str(&[py::format(&v_c.clone(), "")?, " ".to_string()].concat()) } else { Py::str("") }), &v_f.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(py::add(&py::add(&py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t213 in py::iter(&(py::zip(&[py::slice(&v_coefs.clone(), None, Some((v_nr.clone()).i()?), None)?, py::slice(&v_species.clone(), None, Some((v_nr.clone()).i()?), None)?])?))? { let mut v_c: Py; let mut v_f: Py; 
let t214 = (t213).items()?;
if t214.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t214[0].clone();
v_f = t214[1].clone();
out.push(py::call(&v_term.clone(), &[v_c.clone(), v_f.clone()])?); } Py::list(out) }])?, &Py::str(" -> "))?, &py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t215 in py::iter(&(py::zip(&[py::slice(&v_coefs.clone(), Some((v_nr.clone()).i()?), None, None)?, py::slice(&v_species.clone(), Some((v_nr.clone()).i()?), None, None)?])?))? { let mut v_c: Py; let mut v_f: Py; 
let t216 = (t215).items()?;
if t216.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t216[0].clone();
v_f = t216[1].clone();
out.push(py::call(&v_term.clone(), &[v_c.clone(), v_f.clone()])?); } Py::list(out) }])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_activation_energy(mut v_k1: Py, mut v_T1: Py, mut v_k2: Py, mut v_T2: Py) -> R {
    let mut v_Ea: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("k1"), v_k1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("k2"), v_k2.clone()), (Py::str("T2"), v_T2.clone())]))?;
    if ({ let t7 = v_T1.clone(); Py::Bool({ let t8 = v_T2.clone(); if py::eq(&t7, &t8) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two temperatures must differ"))));
    }
    v_Ea = py::truediv(&py::mul(&c_R()?, &py::math1("log", &py::truediv(&v_k2.clone(), &v_k1.clone())?)?)?, &py::sub(&py::truediv(&Py::Int(1i64), &v_T1.clone())?, &py::truediv(&Py::Int(1i64), &v_T2.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Ea"), v_Ea.clone()), (Py::str("Ea_kJ"), py::truediv(&v_Ea.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arrhenius_k(mut v_A: Py, mut v_Ea: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("Ea"), v_Ea.clone()), (Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("k"), py::mul(&v_A.clone(), &py::math1("exp", &py::truediv(&py::neg(&v_Ea.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arrhenius_k2(mut v_k1: Py, mut v_T1: Py, mut v_T2: Py, mut v_Ea: Py) -> R {
    let mut v_ln: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("k1"), v_k1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone()), (Py::str("Ea"), v_Ea.clone())]))?;
    v_ln = py::sub(&py::math1("log", &v_k1.clone())?, &py::mul(&py::truediv(&v_Ea.clone(), &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &v_T2.clone())?, &py::truediv(&Py::Int(1i64), &v_T1.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("k2"), py::math1("exp", &v_ln.clone())?), (Py::str("ratio"), py::truediv(&py::math1("exp", &v_ln.clone())?, &v_k1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_atoms_of_element(mut v_formula: Py, mut v_element: Py, mut v_m: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone())]))?;
    v_el = f__element(v_element.clone())?;
    v_counts = f_formula_counts(v_formula.clone())?;
    if ({ let t62 = v_el.clone(); Py::Bool({ let t63 = v_counts.clone(); if !py::contains(&t63, &t62)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_el.clone(), "")?, " does not occur in ".to_string(), py::format(&v_formula.clone(), "")?].concat()))));
    }
    v_n = py::truediv(&v_m.clone(), &py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("N_atoms"), py::mul(&py::mul(&v_n.clone(), &py::getitem(&v_counts.clone(), &v_el.clone())?)?, &c_NA()?)?), (Py::str("n_element"), py::mul(&v_n.clone(), &py::getitem(&v_counts.clone(), &v_el.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_average_atomic_mass(mut v_masses: Py, mut v_abundances: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    if ({ let t98 = py::len(&v_masses.clone())?; Py::Bool({ let t99 = py::len(&v_abundances.clone())?; if py::ne(&t98, &t99) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one abundance per isotope mass"))));
    }
    if ({ let t100 = py::len(&v_masses.clone())?; Py::Bool({ let t101 = Py::Int(2i64); if py::lt(&t100, &t101)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at least two isotopes are needed"))));
    }
    for t102 in py::iter(&(v_masses.clone()))? {
        v_m = t102;
        if (py::not(&{ let t103 = v_m.clone(); Py::Bool({ let t104 = Py::Int(0i64); if py::gt(&t103, &t104)? { true } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("isotope masses must be positive"))));
        }
    }
    for t105 in py::iter(&(v_abundances.clone()))? {
        v_f = t105;
        if ({ let t106 = v_f.clone(); Py::Bool({ let t107 = Py::Int(0i64); if py::lt(&t106, &t107)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("abundances cannot be negative"))));
        }
    }
    v_scale = f__abundance_scale(v_abundances.clone())?;
    v_s = Py::Float(0.0);
    for t113 in py::iter(&(py::zip(&[v_masses.clone(), v_abundances.clone()])?))? {
        let t114 = (t113).items()?;
        if t114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_m = t114[0].clone();
        v_f = t114[1].clone();
        v_s = py::iadd(&v_s, &py::mul(&v_m.clone(), &v_f.clone())?)?;
    }
    v_A = py::mul(&v_s.clone(), &v_scale.clone())?;
    return Ok(Py::dict(vec![(Py::str("A_r"), v_A.clone()), (Py::str("M"), py::truediv(&v_A.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_balance(mut v_text: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_products: Py = Py::None;
    let mut v_reactants: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_species: Py = Py::None;
    let t3374 = (f_parse_equation(v_text.clone())?).items()?;
    if t3374.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reactants = t3374[0].clone();
    v_products = t3374[1].clone();
    let t3387 = (f__element_matrix(v_reactants.clone(), v_products.clone())?).items()?;
    if t3387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t3387[0].clone();
    v_species = t3387[1].clone();
    v_coefs = f__nullspace_1(v_rows.clone(), py::len(&v_species.clone())?)?;
    return Ok(Py::tuple(vec![v_coefs.clone(), v_species.clone(), py::len(&v_reactants.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_balance_equation(mut v_equation: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_species: Py = Py::None;
    let t209 = (f_balance(v_equation.clone())?).items()?;
    if t209.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t209[0].clone();
    v_species = t209[1].clone();
    v_nr = t209[2].clone();
    return Ok(Py::dict(vec![(Py::str("coefficients"), { let mut out = vec![]; for t210 in py::iter(&(v_coefs.clone()))? { let mut v_c: Py; 
v_c = t210;
out.push(py::float(&v_c.clone())?); } Py::list(out) }), (Py::str("balanced"), f__written(v_coefs.clone(), v_species.clone(), v_nr.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beer_absorbance(mut v_eps: Py, mut v_l: Py, mut v_c: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("eps"), v_eps.clone()), (Py::str("l"), v_l.clone()), (Py::str("c"), v_c.clone())]))?;
    v_A = py::mul(&py::mul(&v_eps.clone(), &v_l.clone())?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("T_percent"), py::mul(&Py::Float(100.0), &py::pow(&Py::Float(10.0), &py::neg(&v_A.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_beer_concentration(mut v_A: Py, mut v_eps: Py, mut v_l: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("eps"), v_eps.clone()), (Py::str("l"), v_l.clone())]))?;
    v_c = py::truediv(&v_A.clone(), &py::mul(&v_eps.clone(), &v_l.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("c_molL"), py::truediv(&v_c.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bond_enthalpy_reaction(mut v_broken_n: Py, mut v_broken_D: Py, mut v_formed_n: Py, mut v_formed_D: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_Fm: Py = Py::None;
    let mut v_dH: Py = Py::None;
    if ({ let t304 = { let t300 = py::len(&v_broken_n.clone())?; Py::Bool({ let t301 = py::len(&v_broken_D.clone())?; if py::ne(&t300, &t301) { true } else { false } }) }; if t304.truthy() { t304 } else { { let t302 = py::len(&v_formed_n.clone())?; Py::Bool({ let t303 = py::len(&v_formed_D.clone())?; if py::ne(&t302, &t303) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one count per bond enthalpy on each side"))));
    }
    v_B = f__bond_sum(v_broken_n.clone(), v_broken_D.clone())?;
    v_Fm = f__bond_sum(v_formed_n.clone(), v_formed_D.clone())?;
    v_dH = py::sub(&v_B.clone(), &v_Fm.clone())?;
    return Ok(Py::dict(vec![(Py::str("dH"), py::mul(&v_dH.clone(), &Py::Float(1000.0))?), (Py::str("dH_kJ"), v_dH.clone()), (Py::str("broken_kJ"), v_B.clone()), (Py::str("formed_kJ"), v_Fm.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boyle_p2(mut v_P1: Py, mut v_V1: Py, mut v_V2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("V1"), v_V1.clone()), (Py::str("V2"), v_V2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("P2"), py::truediv(&py::mul(&v_P1.clone(), &v_V1.clone())?, &v_V2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_boyle_v2(mut v_P1: Py, mut v_V1: Py, mut v_P2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("V1"), v_V1.clone()), (Py::str("P2"), v_P2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("V2"), py::truediv(&py::mul(&v_P1.clone(), &v_V1.clone())?, &v_P2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buffer_ph(mut v_Ka: Py, mut v_c_base: Py, mut v_c_acid: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("Ka"), v_Ka.clone()), (Py::str("c_base"), v_c_base.clone()), (Py::str("c_acid"), v_c_acid.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("pH"), py::add(&py::neg(&py::math1("log10", &v_Ka.clone())?)?, &py::math1("log10", &py::truediv(&v_c_base.clone(), &v_c_acid.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_calorimetry_enthalpy(mut v_m: Py, mut v_c: Py, mut v_T1: Py, mut v_T2: Py, mut v_n: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("c"), v_c.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone()), (Py::str("n"), v_n.clone())]))?;
    v_q = py::mul(&py::mul(&v_m.clone(), &v_c.clone())?, &py::sub(&v_T2.clone(), &v_T1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("dH"), py::truediv(&py::neg(&v_q.clone())?, &v_n.clone())?), (Py::str("dH_kJ"), py::truediv(&py::truediv(&py::neg(&v_q.clone())?, &v_n.clone())?, &Py::Float(1000.0))?), (Py::str("q_solution"), v_q.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_calorimetry_heat(mut v_m: Py, mut v_c: Py, mut v_T1: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("c"), v_c.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("q"), py::mul(&py::mul(&v_m.clone(), &v_c.clone())?, &py::sub(&v_T2.clone(), &v_T1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cc_enthalpy(mut v_P1: Py, mut v_T1: Py, mut v_P2: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("P2"), v_P2.clone()), (Py::str("T2"), v_T2.clone())]))?;
    if ({ let t364 = v_T1.clone(); Py::Bool({ let t365 = v_T2.clone(); if py::eq(&t364, &t365) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two temperatures must differ"))));
    }
    return Ok(Py::dict(vec![(Py::str("dH"), py::truediv(&py::mul(&c_R()?, &py::math1("log", &py::truediv(&v_P2.clone(), &v_P1.clone())?)?)?, &py::sub(&py::truediv(&Py::Int(1i64), &v_T1.clone())?, &py::truediv(&Py::Int(1i64), &v_T2.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cc_pressure(mut v_P1: Py, mut v_T1: Py, mut v_T2: Py, mut v_dH: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone()), (Py::str("dH"), v_dH.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("P2"), py::mul(&v_P1.clone(), &py::math1("exp", &py::mul(&py::truediv(&py::neg(&v_dH.clone())?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &v_T2.clone())?, &py::truediv(&Py::Int(1i64), &v_T1.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_charles_v2(mut v_V1: Py, mut v_T1: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("V1"), v_V1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("V2"), py::truediv(&py::mul(&v_V1.clone(), &v_T2.clone())?, &v_T1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combined_p2(mut v_P1: Py, mut v_V1: Py, mut v_T1: Py, mut v_V2: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("V1"), v_V1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("V2"), v_V2.clone()), (Py::str("T2"), v_T2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("P2"), py::truediv(&py::mul(&py::mul(&v_P1.clone(), &v_V1.clone())?, &v_T2.clone())?, &py::mul(&v_T1.clone(), &v_V2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combined_t2(mut v_P1: Py, mut v_V1: Py, mut v_T1: Py, mut v_P2: Py, mut v_V2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("V1"), v_V1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("P2"), v_P2.clone()), (Py::str("V2"), v_V2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("T2"), py::truediv(&py::mul(&py::mul(&v_P2.clone(), &v_V2.clone())?, &v_T1.clone())?, &py::mul(&v_P1.clone(), &v_V1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_combined_v2(mut v_P1: Py, mut v_V1: Py, mut v_T1: Py, mut v_P2: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("V1"), v_V1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("P2"), v_P2.clone()), (Py::str("T2"), v_T2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("V2"), py::truediv(&py::mul(&py::mul(&v_P1.clone(), &v_V1.clone())?, &v_T2.clone())?, &py::mul(&v_T1.clone(), &v_P2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_common_ion(mut v_Ksp: Py, mut v_x: Py, mut v_y: Py, mut v_c_common: Py, mut v_common: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_ca: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_term: Py = Py::None;
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ksp"), v_Ksp.clone()), (Py::str("c_common"), v_c_common.clone())]))?;
    let t412 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t412[0].clone();
    v_y = t412[1].clone();
    v_c = py::truediv(&v_c_common.clone(), &Py::Float(1000.0))?;
    let t415 = ((if ({ let t413 = v_common.clone(); Py::Bool({ let t414 = Py::str("cation"); if py::eq(&t413, &t414) { true } else { false } }) }).truthy() { Py::tuple(vec![v_c.clone(), Py::Float(0.0)]) } else { Py::tuple(vec![Py::Float(0.0), v_c.clone()]) })).items()?;
    if t415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t415[0].clone();
    v_cb = t415[1].clone();
    v_term = { let v_Ksp = v_Ksp.clone(); let v_c_common = v_c_common.clone(); let v_ca = v_ca.clone(); let v_cb = v_cb.clone(); let v_common = v_common.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_u: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t416 = v_c.clone(); Py::Bool({ let t417 = Py::Int(0i64); if py::eq(&t416, &t417) { true } else { false } }) }).truthy() { py::add(&py::math1("log", &v_k.clone())?, &v_u.clone())? } else { py::math1("log", &py::add(&py::mul(&v_k.clone(), &py::math1("exp", &v_u.clone())?)?, &v_c.clone())?)? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_hi = py::getitem(&f_ksp_solubility(v_Ksp.clone(), v_x.clone(), v_y.clone())?, &Py::str("s_molL"))?;
    v_u = f__bisect({ let v_Ksp = v_Ksp.clone(); let v_c = v_c.clone(); let v_c_common = v_c_common.clone(); let v_ca = v_ca.clone(); let v_cb = v_cb.clone(); let v_common = v_common.clone(); let v_hi = v_hi.clone(); let v_term = v_term.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::add(&py::mul(&v_x.clone(), &py::call(&v_term.clone(), &[v_x.clone(), v_ca.clone(), v_u.clone()])?)?, &py::mul(&v_y.clone(), &py::call(&v_term.clone(), &[v_y.clone(), v_cb.clone(), v_u.clone()])?)?)?, &py::math1("log", &v_Ksp.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::sub(&py::math1("log", &v_hi.clone())?, &Py::Float(600.0))?, py::math1("log", &v_hi.clone())?, Py::Int(400i64))?;
    v_s = py::math1("exp", &v_u.clone())?;
    return Ok(Py::dict(vec![(Py::str("s"), py::mul(&v_s.clone(), &Py::Float(1000.0))?), (Py::str("s_molL"), v_s.clone()), (Py::str("log10_s"), py::truediv(&v_u.clone(), &py::math1("log", &Py::Float(10.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conjugate_constant(mut v_K: Py, mut v_known: Py) -> R {
    let mut v_Kc: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone())]))?;
    if ({ let t458 = { let t454 = v_known.clone(); Py::Bool({ let t455 = Py::str("Ka"); if py::ne(&t454, &t455) { true } else { false } }) }; if !t458.truthy() { t458 } else { { let t456 = v_known.clone(); Py::Bool({ let t457 = Py::str("Kb"); if py::ne(&t456, &t457) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the given constant is Ka (of an acid) or Kb (of a base)"))));
    }
    v_Kc = py::truediv(&c_KW()?, &v_K.clone())?;
    return Ok(Py::dict(vec![(Py::str("K_conj"), v_Kc.clone()), (Py::str("pK"), py::neg(&py::math1("log10", &v_K.clone())?)?), (Py::str("pK_conj"), py::neg(&py::math1("log10", &v_Kc.clone())?)?), (Py::str("conjugate"), f__conj_word(v_known.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_consecutive(mut v_A0: Py, mut v_k1: Py, mut v_k2: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("A0"), v_A0.clone()), (Py::str("k1"), v_k1.clone()), (Py::str("k2"), v_k2.clone()), (Py::str("t"), v_t.clone())]))?;
    v_A = py::mul(&v_A0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)?;
    v_d = py::sub(&v_k2.clone(), &v_k1.clone())?;
    v_B = (if ({ let t473 = v_d.clone(); Py::Bool({ let t474 = Py::Int(0i64); if py::eq(&t473, &t474) { true } else { false } }) }).truthy() { py::mul(&py::mul(&py::mul(&v_A0.clone(), &v_k1.clone())?, &v_t.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)? } else { py::truediv(&py::mul(&py::mul(&py::mul(&v_A0.clone(), &v_k1.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_d.clone())?, &v_t.clone())?)?)?)?, &v_d.clone())? });
    return Ok(Py::dict(vec![(Py::str("B"), v_B.clone()), (Py::str("A"), v_A.clone()), (Py::str("C"), py::sub(&py::sub(&v_A0.clone(), &v_A.clone())?, &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crossover_t(mut v_dH: Py, mut v_dS: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("dH"), v_dH.clone()), (Py::str("dS"), v_dS.clone())]))?;
    if ({ let t494 = { let t483 = v_dS.clone(); Py::Bool({ let t484 = Py::Int(0i64); if py::eq(&t483, &t484) { true } else { false } }) }; if t494.truthy() { t494 } else { { let t493 = { let t485 = v_dH.clone(); Py::Bool({ let t486 = Py::Int(0i64); if py::eq(&t485, &t486) { true } else { false } }) }; if t493.truthy() { t493 } else { { let t487 = { let t489 = v_dH.clone(); Py::Bool({ let t490 = Py::Int(0i64); if py::gt(&t489, &t490)? { true } else { false } }) }; Py::Bool({ let t488 = { let t491 = v_dS.clone(); Py::Bool({ let t492 = Py::Int(0i64); if py::gt(&t491, &t492)? { true } else { false } }) }; if py::ne(&t487, &t488) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("dH and dS must have the same sign for a temperature where dG = 0 to exist"))));
    }
    return Ok(Py::dict(vec![(Py::str("T"), py::truediv(&v_dH.clone(), &v_dS.clone())?), (Py::str("T_C"), py::sub(&py::truediv(&v_dH.clone(), &v_dS.clone())?, &Py::Float(273.15))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dalton_mixture(mut v_moles: Py, mut v_P_total: Py) -> R {
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("P_total"), v_P_total.clone())]))?;
    if (Py::Bool(({ let mut out = vec![]; for t505 in py::iter(&(v_moles.clone()))? { let mut v_n: Py; 
v_n = t505;
out.push({ let t506 = v_n.clone(); Py::Bool({ let t507 = Py::Int(0i64); if py::le(&t506, &t507)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every amount must be positive"))));
    }
    v_tot = py::fsum(&v_moles.clone())?;
    v_x = { let mut out = vec![]; for t508 in py::iter(&(v_moles.clone()))? { let mut v_n: Py; 
v_n = t508;
out.push(py::truediv(&v_n.clone(), &v_tot.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("partial_pressures"), { let mut out = vec![]; for t509 in py::iter(&(v_x.clone()))? { let mut v_xi: Py; 
v_xi = t509;
out.push(py::mul(&v_xi.clone(), &v_P_total.clone())?); } Py::list(out) }), (Py::str("mole_fractions"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_degree_of_unsaturation(mut v_formula: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_mono: Py = Py::None;
    let mut v_tet: Py = Py::None;
    let mut v_tri: Py = Py::None;
    v_c = f_formula_counts(v_formula.clone())?;
    if (py::sub(&py::set(&v_c.clone())?, &c__DOU_EL()?)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("only C, H, N, O, S, P, Si, B and halogens are supported"))));
    }
    if (py::not(&py::method(&v_c.clone(), "get", &[Py::str("C")])?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an organic formula (with carbon) is expected"))));
    }
    v_tet = py::add(&py::method(&v_c.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, &py::method(&v_c.clone(), "get", &[Py::str("Si"), Py::Int(0i64)])?)?;
    v_tri = py::add(&py::add(&py::method(&v_c.clone(), "get", &[Py::str("N"), Py::Int(0i64)])?, &py::method(&v_c.clone(), "get", &[Py::str("P"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("B"), Py::Int(0i64)])?)?;
    v_mono = py::add(&py::method(&v_c.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?, &py::sum(&{ let mut out = vec![]; for t542 in py::iter(&(Py::tuple(vec![Py::str("F"), Py::str("Cl"), Py::str("Br"), Py::str("I")])))? { let mut v_x: Py; 
v_x = t542;
out.push(py::method(&v_c.clone(), "get", &[v_x.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)?;
    v_d = py::fraction(&[py::sub(&py::add(&py::add(&py::mul(&Py::Int(2i64), &v_tet.clone())?, &Py::Int(2i64))?, &v_tri.clone())?, &v_mono.clone())?, Py::Int(2i64)])?;
    if ({ let t547 = { let t543 = v_d.clone(); Py::Bool({ let t544 = Py::Int(0i64); if py::lt(&t543, &t544)? { true } else { false } }) }; if t547.truthy() { t547 } else { { let t545 = py::denominator(&v_d.clone())?; Py::Bool({ let t546 = Py::Int(1i64); if py::ne(&t545, &t546) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this formula has no valid structure (negative or half-integer unsaturation)"))));
    }
    return Ok(Py::dict(vec![(Py::str("dou"), py::float(&v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dg_from_e(mut v_n: Py, mut v_E: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("E"), v_E.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("dG"), py::mul(&py::mul(&py::neg(&v_n.clone())?, &c_F()?)?, &v_E.clone())?), (Py::str("dG_kJ"), py::truediv(&py::mul(&py::mul(&py::neg(&v_n.clone())?, &c_F()?)?, &v_E.clone())?, &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dg_from_k(mut v_K: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone()), (Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("dG"), py::mul(&py::mul(&py::neg(&c_R()?)?, &v_T.clone())?, &py::math1("log", &v_K.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dg_nonstandard(mut v_dG0: Py, mut v_T: Py, mut v_Q: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("dG0"), v_dG0.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("Q"), v_Q.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("dG"), py::add(&v_dG0.clone(), &py::mul(&py::mul(&c_R()?, &v_T.clone())?, &py::math1("log", &v_Q.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dilution_c2(mut v_c1: Py, mut v_V1: Py, mut v_V2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("c1"), v_c1.clone()), (Py::str("V1"), v_V1.clone()), (Py::str("V2"), v_V2.clone())]))?;
    if ({ let t612 = v_V2.clone(); Py::Bool({ let t613 = v_V1.clone(); if py::lt(&t612, &t613)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a dilution needs a final volume at least the initial volume"))));
    }
    return Ok(Py::dict(vec![(Py::str("c2"), py::truediv(&py::mul(&v_c1.clone(), &v_V1.clone())?, &v_V2.clone())?), (Py::str("c2_molL"), py::truediv(&py::truediv(&py::mul(&v_c1.clone(), &v_V1.clone())?, &v_V2.clone())?, &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dilution_v1(mut v_c1: Py, mut v_c2: Py, mut v_V2: Py) -> R {
    let mut v_V1: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c1"), v_c1.clone()), (Py::str("c2"), v_c2.clone()), (Py::str("V2"), v_V2.clone())]))?;
    if ({ let t624 = v_c2.clone(); Py::Bool({ let t625 = v_c1.clone(); if py::gt(&t624, &t625)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stock solution must be more concentrated than the diluted one"))));
    }
    v_V1 = py::truediv(&py::mul(&v_c2.clone(), &v_V2.clone())?, &v_c1.clone())?;
    return Ok(Py::dict(vec![(Py::str("V1"), v_V1.clone()), (Py::str("V1_mL"), py::mul(&v_V1.clone(), &Py::Float(1000000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ee_from_composition(mut v_R: Py, mut v_S: Py) -> R {
    let mut v_tot: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("R"), v_R.clone()), (Py::str("S"), v_S.clone())]))?;
    if ({ let t730 = { let t726 = v_R.clone(); Py::Bool({ let t727 = Py::Int(0i64); if py::lt(&t726, &t727)? { true } else { false } }) }; if t730.truthy() { t730 } else { { let t728 = v_S.clone(); Py::Bool({ let t729 = Py::Int(0i64); if py::lt(&t728, &t729)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("amounts cannot be negative"))));
    }
    v_tot = py::add(&v_R.clone(), &v_S.clone())?;
    if (py::not(&{ let t731 = v_tot.clone(); Py::Bool({ let t732 = Py::Int(0i64); if py::gt(&t731, &t732)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the mixture must contain some of either enantiomer"))));
    }
    return Ok(Py::dict(vec![(Py::str("ee"), py::truediv(&py::mul(&Py::Float(100.0), &py::abs(&py::sub(&v_R.clone(), &v_S.clone())?)?)?, &v_tot.clone())?), (Py::str("percent_R"), py::truediv(&py::mul(&Py::Float(100.0), &v_R.clone())?, &v_tot.clone())?), (Py::str("percent_S"), py::truediv(&py::mul(&Py::Float(100.0), &v_S.clone())?, &v_tot.clone())?), (Py::str("predominant"), f__ee_comp_word(v_R.clone(), v_S.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ee_from_rotation(mut v_alpha_obs: Py, mut v_alpha_pure: Py) -> R {
    let mut v_ee: Py = Py::None;
    let mut v_major: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("alpha_obs"), v_alpha_obs.clone()), (Py::str("alpha_pure"), v_alpha_pure.clone())]))?;
    if ({ let t742 = v_alpha_pure.clone(); Py::Bool({ let t743 = Py::Int(0i64); if py::eq(&t742, &t743) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pure enantiomer's specific rotation cannot be zero"))));
    }
    v_r = py::truediv(&v_alpha_obs.clone(), &v_alpha_pure.clone())?;
    if ({ let t744 = py::abs(&v_r.clone())?; Py::Bool({ let t745 = Py::Float(1.0); if py::gt(&t744, &t745)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the observed rotation cannot be larger in size than the pure enantiomer's"))));
    }
    v_ee = py::mul(&Py::Float(100.0), &py::abs(&v_r.clone())?)?;
    v_major = py::add(&Py::Float(50.0), &py::truediv(&v_ee.clone(), &Py::Float(2.0))?)?;
    return Ok(Py::dict(vec![(Py::str("ee"), v_ee.clone()), (Py::str("major_percent"), v_major.clone()), (Py::str("minor_percent"), py::sub(&Py::Float(100.0), &v_major.clone())?), (Py::str("predominant"), f__ee_word(v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_electrolysis_mass(mut v_metal: Py, mut v_I: Py, mut v_t: Py, mut v_n: Py) -> R {
    let mut v_el: Py = Py::None;
    let mut v_m: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("I"), v_I.clone()), (Py::str("t"), v_t.clone()), (Py::str("n"), v_n.clone())]))?;
    v_el = f__element(v_metal.clone())?;
    v_m = py::truediv(&py::mul(&py::truediv(&py::mul(&v_I.clone(), &v_t.clone())?, &py::mul(&v_n.clone(), &c_F()?)?)?, &py::getitem(&c_W()?, &v_el.clone())?)?, &Py::Float(1000.0))?;
    return Ok(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("m_g"), py::mul(&v_m.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_electrolysis_time(mut v_metal: Py, mut v_m: Py, mut v_I: Py, mut v_n: Py) -> R {
    let mut v_el: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("I"), v_I.clone()), (Py::str("n"), v_n.clone())]))?;
    v_el = f__element(v_metal.clone())?;
    return Ok(Py::dict(vec![(Py::str("t"), py::truediv(&py::mul(&py::mul(&py::truediv(&py::mul(&v_m.clone(), &Py::Float(1000.0))?, &py::getitem(&c_W()?, &v_el.clone())?)?, &v_n.clone())?, &c_F()?)?, &v_I.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_empirical_formula(mut v_composition: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_ints: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_moles: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_comp = f__composition(v_composition.clone())?;
    v_order = Py::list((v_comp.clone()).items()?);
    v_moles = { let mut out = vec![]; for t670 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t670;
out.push((v_e.clone(), py::truediv(&py::getitem(&v_comp.clone(), &v_e.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?)); } Py::dict(out) };
    v_lo = py::minmax(&[py::method(&v_moles.clone(), "values", &[])?], false)?;
    v_ratio = { let mut out = vec![]; for t671 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t671;
out.push((v_e.clone(), py::truediv(&py::getitem(&v_moles.clone(), &v_e.clone())?, &v_lo.clone())?)); } Py::dict(out) };
    for t672 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(13i64)).i()?]) {
        v_k = t672;
        v_ints = { let mut out = vec![]; for t673 in py::iter(&(py::method(&v_ratio.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_r: Py; 
let t674 = (t673).items()?;
if t674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t674[0].clone();
v_r = t674[1].clone();
out.push((v_e.clone(), py::round(&py::mul(&v_k.clone(), &v_r.clone())?, None)?)); } Py::dict(out) };
        if (Py::Bool(({ let mut out = vec![]; for t675 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t675;
out.push({ let t680 = { let t676 = py::getitem(&v_ints.clone(), &v_e.clone())?; Py::Bool({ let t677 = Py::Int(1i64); if py::ge(&t676, &t677)? { true } else { false } }) }; if !t680.truthy() { t680 } else { { let t678 = py::abs(&py::sub(&py::mul(&v_k.clone(), &py::getitem(&v_ratio.clone(), &v_e.clone())?)?, &py::getitem(&v_ints.clone(), &v_e.clone())?)?)?; Py::Bool({ let t679 = py::mul(&Py::Float(0.015), &py::getitem(&v_ints.clone(), &v_e.clone())?)?; if py::le(&t678, &t679)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
            v_g = Py::Int(0i64);
            for t681 in py::iter(&(py::method(&v_ints.clone(), "values", &[])?))? {
                v_v = t681;
                v_g = py::gcd(&v_g.clone(), &v_v.clone())?;
            }
            v_ints = { let mut out = vec![]; for t682 in py::iter(&(py::method(&v_ints.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_v: Py; 
let t683 = (t682).items()?;
if t683.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t683[0].clone();
v_v = t683[1].clone();
out.push((v_e.clone(), py::floordiv(&v_v.clone(), &v_g.clone())?)); } Py::dict(out) };
            return Ok(Py::dict(vec![(Py::str("formula"), f__write_formula(v_order.clone(), v_ints.clone())?), (Py::str("counts"), { let mut out = vec![]; for t687 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t687;
out.push(py::float(&py::getitem(&v_ints.clone(), &v_e.clone())?)?); } Py::list(out) })]));
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("the composition does not fit a small whole-number ratio (up to 12)"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equilibrium_ice(mut v_equation: Py, mut v_initial: Py, mut v_K: Py) -> R {
    let mut v_hi: Py = Py::None;
    let mut v_lnK: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t805 = (f__ice_terms(v_equation.clone())?).items()?;
    if t805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nu = t805[0].clone();
    v_sp = t805[1].clone();
    if ({ let t806 = py::len(&v_initial.clone())?; Py::Bool({ let t807 = py::len(&v_sp.clone())?; if py::ne(&t806, &t807) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one initial concentration per species (".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_sp.clone()])?, "")?, ")".to_string()].concat()))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t808 in py::iter(&(v_initial.clone()))? { let mut v_c: Py; 
v_c = t808;
out.push({ let t809 = v_c.clone(); Py::Bool({ let t810 = Py::Int(0i64); if py::lt(&t809, &t810)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("concentrations cannot be negative"))));
    }
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone())]))?;
    let t827 = (f__ice_bounds(v_nu.clone(), v_initial.clone())?).items()?;
    if t827.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t827[0].clone();
    v_hi = t827[1].clone();
    v_lnK = py::math1("log", &v_K.clone())?;
    v_x = f__bisect({ let v_K = v_K.clone(); let v_equation = v_equation.clone(); let v_hi = v_hi.clone(); let v_initial = v_initial.clone(); let v_lnK = v_lnK.clone(); let v_lo = v_lo.clone(); let v_nu = v_nu.clone(); let v_sp = v_sp.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__lnq(v_nu.clone(), v_initial.clone(), v_x.clone())?, &v_lnK.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_lo.clone(), v_hi.clone(), Py::Int(400i64))?;
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("equilibrium"), { let mut out = vec![]; for t851 in py::iter(&(py::zip(&[v_initial.clone(), v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t852 = (t851).items()?;
if t852.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t852[0].clone();
v_n = t852[1].clone();
out.push(py::add(&v_c.clone(), &py::mul(&v_n.clone(), &v_x.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equivalence_volume(mut v_c_t: Py, mut v_V_a: Py, mut v_c_a: Py, mut v_ratio: Py) -> R {
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c_t"), v_c_t.clone()), (Py::str("V_a"), v_V_a.clone()), (Py::str("c_a"), v_c_a.clone()), (Py::str("ratio"), v_ratio.clone())]))?;
    v_V = py::truediv(&py::mul(&py::mul(&v_ratio.clone(), &v_c_a.clone())?, &v_V_a.clone())?, &v_c_t.clone())?;
    return Ok(Py::dict(vec![(Py::str("V_eq"), v_V.clone()), (Py::str("V_eq_mL"), py::mul(&v_V.clone(), &Py::Float(1000000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_conc(mut v_A0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("A0"), v_A0.clone()), (Py::str("k"), v_k.clone()), (Py::str("t"), v_t.clone())]))?;
    v_A = py::mul(&v_A0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A_molL"), py::truediv(&v_A.clone(), &Py::Float(1000.0))?), (Py::str("fraction_left"), py::truediv(&v_A.clone(), &v_A0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_half_life(mut v_k: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("k"), v_k.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("t_half"), py::truediv(&py::math1("log", &Py::Float(2.0))?, &v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_time(mut v_A0: Py, mut v_A: Py, mut v_k: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("A0"), v_A0.clone()), (Py::str("A"), v_A.clone()), (Py::str("k"), v_k.clone())]))?;
    if ({ let t891 = v_A.clone(); Py::Bool({ let t892 = v_A0.clone(); if py::ge(&t891, &t892)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the final concentration must be below the initial one"))));
    }
    return Ok(Py::dict(vec![(Py::str("t"), py::truediv(&py::math1("log", &py::truediv(&v_A0.clone(), &v_A.clone())?)?, &v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_formation_sum(mut v_equation: Py, mut v_values: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_species: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let t2899 = (f__as_written_balanced(v_equation.clone())?).items()?;
    if t2899.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t2899[0].clone();
    v_species = t2899[1].clone();
    v_nr = t2899[2].clone();
    if ({ let t2900 = py::len(&v_values.clone())?; Py::Bool({ let t2901 = py::len(&v_species.clone())?; if py::ne(&t2900, &t2901) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one value per species in the order written (".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_species.clone()])?, "")?, ")".to_string()].concat()))));
    }
    v_tot = py::fsum(&{ let mut out = vec![]; for t2902 in py::iter(&(py::enumerate(&py::zip(&[v_coefs.clone(), v_values.clone()])?, 0)?))? { let mut v_c: Py; let mut v_j: Py; let mut v_v: Py; 
let t2903 = (t2902).items()?;
if t2903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t2903[0].clone();
let t2904 = (t2903[1].clone()).items()?;
if t2904.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t2904[0].clone();
v_v = t2904[1].clone();
out.push(py::mul(&(if ({ let t2905 = v_j.clone(); Py::Bool({ let t2906 = v_nr.clone(); if py::ge(&t2905, &t2906)? { true } else { false } }) }).truthy() { v_c.clone() } else { py::neg(&v_c.clone())? }), &v_v.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("dH"), py::mul(&v_tot.clone(), &Py::Float(1000.0))?), (Py::str("dH_kJ"), v_tot.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_formula_counts(mut v_text: Py) -> R {
    let mut v_el: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mult: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_total: Py = Py::None;
    if (py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_))))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a formula is text, such as 'H2O'"))));
    }
    v_s = py::method(&py::method(&v_text.clone(), "translate", &[c__SUBSCRIPT()?])?, "replace", &[Py::str(" "), Py::str("")])?;
    if (py::not(&v_s.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("empty formula"))));
    }
    if (ax_engine::pyre::module_call("search", &Py::str("[+\\-^]"), &[v_s.clone()], 0)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("ions are not supported: give a neutral formula"))));
    }
    v_total = Py::dict(vec![]);
    for t3547 in py::iter(&(py::method(&c__HYDRATE_SPLIT()?, "split", &[v_s.clone()])?))? {
        v_part = t3547;
        if (py::not(&v_part.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("malformed hydrate formula"))));
        }
        v_m = ax_engine::pyre::module_call("match", &Py::str("(\\d+)(?=[A-Z(\\[{])"), &[v_part.clone()], 0)?;
        v_mult = (if (v_m.clone()).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?)? } else { Py::Int(1i64) });
        if (v_m.clone()).truthy() {
            v_part = py::slice(&v_part.clone(), Some((py::method(&v_m.clone(), "end", &[])?).i()?), None, None)?;
        }
        if ({ let t3548 = v_mult.clone(); Py::Bool({ let t3549 = Py::Int(0i64); if py::eq(&t3548, &t3549) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a multiplier of 0 is not allowed"))));
        }
        for t3569 in py::iter(&(py::method(&f__group_counts(v_part.clone())?, "items", &[])?))? {
            let t3570 = (t3569).items()?;
            if t3570.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_el = t3570[0].clone();
            v_n = t3570[1].clone();
            py::setitem(&v_total.clone(), &v_el.clone(), py::add(&py::method(&v_total.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &py::mul(&v_n.clone(), &v_mult.clone())?)?)?;
        }
    }
    return Ok(v_total.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_density(mut v_formula: Py, mut v_P: Py, mut v_T: Py) -> R {
    let mut v_M: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("T"), v_T.clone())]))?;
    v_M = py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?;
    return Ok(Py::dict(vec![(Py::str("rho"), py::truediv(&py::mul(&v_P.clone(), &v_M.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gas_mass(mut v_formula: Py, mut v_P: Py, mut v_V: Py, mut v_T: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("V"), v_V.clone()), (Py::str("T"), v_T.clone())]))?;
    v_n = py::truediv(&py::mul(&v_P.clone(), &v_V.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("m"), py::truediv(&py::mul(&v_n.clone(), &f_molar_mass_g(v_formula.clone())?)?, &Py::Float(1000.0))?), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gay_lussac_p2(mut v_P1: Py, mut v_T1: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P1"), v_P1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("P2"), py::truediv(&py::mul(&v_P1.clone(), &v_T2.clone())?, &v_T1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gibbs(mut v_dH: Py, mut v_dS: Py, mut v_T: Py) -> R {
    let mut v_dG: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("dH"), v_dH.clone()), (Py::str("dS"), v_dS.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    v_dG = py::sub(&v_dH.clone(), &py::mul(&v_T.clone(), &v_dS.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("dG"), v_dG.clone()), (Py::str("dG_kJ"), py::truediv(&v_dG.clone(), &Py::Float(1000.0))?), (Py::str("spontaneity"), (if ({ let t990 = v_dG.clone(); Py::Bool({ let t991 = Py::Int(0i64); if py::lt(&t990, &t991)? { true } else { false } }) }).truthy() { Py::str("spontaneous") } else { (if ({ let t992 = v_dG.clone(); Py::Bool({ let t993 = Py::Int(0i64); if py::gt(&t992, &t993)? { true } else { false } }) }).truthy() { Py::str("non-spontaneous") } else { Py::str("at equilibrium") }) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graham_ratio(mut v_formula_1: Py, mut v_formula_2: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("ratio"), py::math1("sqrt", &py::truediv(&f_molar_mass_g(v_formula_2.clone())?, &f_molar_mass_g(v_formula_1.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_graham_unknown(mut v_ratio: Py, mut v_formula_known: Py) -> R {
    let mut v_M: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("ratio"), v_ratio.clone())]))?;
    v_M = py::truediv(&f_molar_mass_g(v_formula_known.clone())?, &py::pow(&v_ratio.clone(), &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("M"), py::truediv(&v_M.clone(), &Py::Float(1000.0))?), (Py::str("M_g"), v_M.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_h_from_ph(mut v_pH: Py) -> R {
    let mut v_h: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("pH"), v_pH.clone())]))?;
    v_h = py::pow(&Py::Float(10.0), &py::neg(&v_pH.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("h"), py::mul(&v_h.clone(), &Py::Float(1000.0))?), (Py::str("h_molL"), v_h.clone()), (Py::str("pOH"), py::sub(&Py::Float(14.0), &v_pH.clone())?), (Py::str("oh"), py::mul(&py::truediv(&c_KW()?, &v_h.clone())?, &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heat_from_mass(mut v_formula: Py, mut v_m: Py, mut v_dH: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dH"), v_dH.clone())]))?;
    v_q = py::mul(&py::truediv(&v_m.clone(), &py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?)?, &v_dH.clone())?;
    return Ok(Py::dict(vec![(Py::str("q"), v_q.clone()), (Py::str("heat_released"), py::neg(&v_q.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henry_solubility(mut v_kH: Py, mut v_P: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("kH"), v_kH.clone()), (Py::str("P"), v_P.clone())]))?;
    v_c = py::mul(&v_kH.clone(), &v_P.clone())?;
    return Ok(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("c_molL"), py::truediv(&v_c.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydrogen_line(mut v_n_upper: Py, mut v_n_lower: Py) -> R {
    let mut v_inv: Py = Py::None;
    let t1194 = (Py::tuple(vec![py::int(&v_n_upper.clone())?, py::int(&v_n_lower.clone())?])).items()?;
    if t1194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_upper = t1194[0].clone();
    v_n_lower = t1194[1].clone();
    if ({ let t1199 = { let t1195 = v_n_lower.clone(); Py::Bool({ let t1196 = Py::Int(1i64); if py::lt(&t1195, &t1196)? { true } else { false } }) }; if t1199.truthy() { t1199 } else { { let t1197 = v_n_upper.clone(); Py::Bool({ let t1198 = v_n_lower.clone(); if py::le(&t1197, &t1198)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the levels must be whole numbers with n_upper > n_lower >= 1"))));
    }
    v_inv = py::mul(&c_R_H()?, &py::sub(&py::truediv(&Py::Float(1.0), &py::pow(&v_n_lower.clone(), &Py::Int(2i64))?)?, &py::truediv(&Py::Float(1.0), &py::pow(&v_n_upper.clone(), &Py::Int(2i64))?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("lam"), py::truediv(&Py::Float(1.0), &v_inv.clone())?), (Py::str("lam_nm"), py::truediv(&Py::Float(1000000000.0), &v_inv.clone())?), (Py::str("E"), py::mul(&py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?, &v_inv.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isotope_abundance(mut v_m1: Py, mut v_m2: Py, mut v_A_r: Py) -> R {
    let mut v_x1: Py = Py::None;
    let mut v_x2: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m1"), v_m1.clone()), (Py::str("m2"), v_m2.clone()), (Py::str("A_r"), v_A_r.clone())]))?;
    if ({ let t1321 = v_m1.clone(); Py::Bool({ let t1322 = v_m2.clone(); if py::eq(&t1321, &t1322) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two isotope masses must differ"))));
    }
    if (py::not(&{ let t1323 = py::minmax(&[v_m1.clone(), v_m2.clone()], false)?; Py::Bool({ let t1324 = v_A_r.clone(); if py::lt(&t1323, &t1324)? { { let t1325 = py::minmax(&[v_m1.clone(), v_m2.clone()], true)?; if py::lt(&t1324, &t1325)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the average atomic mass must lie between the two isotope masses"))));
    }
    v_x1 = py::truediv(&py::sub(&v_A_r.clone(), &v_m2.clone())?, &py::sub(&v_m1.clone(), &v_m2.clone())?)?;
    v_x2 = py::truediv(&py::sub(&v_m1.clone(), &v_A_r.clone())?, &py::sub(&v_m1.clone(), &v_m2.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("x1"), v_x1.clone()), (Py::str("x2"), v_x2.clone()), (Py::str("percent1"), py::mul(&Py::Float(100.0), &v_x1.clone())?), (Py::str("percent2"), py::mul(&Py::Float(100.0), &v_x2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_k_from_dg(mut v_dG: Py, mut v_T: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("dG"), v_dG.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("K"), py::math1("exp", &py::truediv(&py::neg(&v_dG.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?)?), (Py::str("lnK"), py::truediv(&py::neg(&v_dG.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_k_from_e(mut v_n: Py, mut v_E: Py, mut v_T: Py) -> R {
    let mut v_lnK: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("T"), v_T.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("E"), v_E.clone())]))?;
    v_lnK = py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_F()?)?, &v_E.clone())?, &py::mul(&c_R()?, &v_T.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("lnK"), v_lnK.clone()), (Py::str("K"), (if ({ let t1336 = v_lnK.clone(); Py::Bool({ let t1337 = Py::Int(700i64); if py::lt(&t1336, &t1337)? { true } else { false } }) }).truthy() { py::math1("exp", &v_lnK.clone())? } else { Py::Float(f64::INFINITY) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ka_from_ph(mut v_pH: Py, mut v_c: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Ka: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("pH"), v_pH.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("c"), v_c.clone())]))?;
    v_C = py::truediv(&v_c.clone(), &Py::Float(1000.0))?;
    v_h = py::pow(&Py::Float(10.0), &py::neg(&v_pH.clone())?)?;
    v_a = py::sub(&v_h.clone(), &py::truediv(&c_KW()?, &v_h.clone())?)?;
    if ({ let t1365 = { let t1361 = v_a.clone(); Py::Bool({ let t1362 = Py::Int(0i64); if py::le(&t1361, &t1362)? { true } else { false } }) }; if t1365.truthy() { t1365 } else { { let t1363 = v_a.clone(); Py::Bool({ let t1364 = v_C.clone(); if py::ge(&t1363, &t1364)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this pH is not possible for a weak acid of that concentration"))));
    }
    v_Ka = py::truediv(&py::mul(&v_h.clone(), &v_a.clone())?, &py::sub(&v_C.clone(), &v_a.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Ka"), v_Ka.clone()), (Py::str("pKa"), py::neg(&py::math1("log10", &v_Ka.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kc_from_kp(mut v_Kp: Py, mut v_T: Py, mut v_dn: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("Kp"), v_Kp.clone()), (Py::str("T"), v_T.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dn"), v_dn.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("Kc"), py::truediv(&v_Kp.clone(), &py::pow(&py::mul(&c_R_LATM()?, &v_T.clone())?, &v_dn.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kp_from_kc(mut v_Kc: Py, mut v_T: Py, mut v_dn: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("Kc"), v_Kc.clone()), (Py::str("T"), v_T.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dn"), v_dn.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("Kp"), py::mul(&v_Kc.clone(), &py::pow(&py::mul(&c_R_LATM()?, &v_T.clone())?, &v_dn.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ksp_from_s(mut v_s: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_Ksp: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("s"), v_s.clone())]))?;
    let t1416 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t1416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1416[0].clone();
    v_y = t1416[1].clone();
    v_sl = py::truediv(&v_s.clone(), &Py::Float(1000.0))?;
    v_Ksp = py::mul(&py::pow(&py::mul(&v_x.clone(), &v_sl.clone())?, &v_x.clone())?, &py::pow(&py::mul(&v_y.clone(), &v_sl.clone())?, &v_y.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Ksp"), v_Ksp.clone()), (Py::str("pKsp"), py::neg(&py::math1("log10", &v_Ksp.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ksp_solubility(mut v_Ksp: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ksp"), v_Ksp.clone())]))?;
    let t1427 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t1427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1427[0].clone();
    v_y = t1427[1].clone();
    if ({ let t1432 = { let t1428 = v_x.clone(); Py::Bool({ let t1429 = Py::Int(1i64); if py::lt(&t1428, &t1429)? { true } else { false } }) }; if t1432.truthy() { t1432 } else { { let t1430 = v_y.clone(); Py::Bool({ let t1431 = Py::Int(1i64); if py::lt(&t1430, &t1431)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and y are whole numbers >= 1"))));
    }
    v_s = py::pow(&py::truediv(&v_Ksp.clone(), &py::mul(&py::pow(&v_x.clone(), &v_x.clone())?, &py::pow(&v_y.clone(), &v_y.clone())?)?)?, &py::truediv(&Py::Float(1.0), &py::add(&v_x.clone(), &v_y.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("s"), py::mul(&v_s.clone(), &Py::Float(1000.0))?), (Py::str("s_molL"), v_s.clone()), (Py::str("log10_s"), py::math1("log10", &v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_limiting_reagent(mut v_equation: Py, mut v_reactant_a: Py, mut v_m_a: Py, mut v_reactant_b: Py, mut v_m_b: Py, mut v_product: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_ext: Py = Py::None;
    let mut v_j_o: Py = Py::None;
    let mut v_ja: Py = Py::None;
    let mut v_jb: Py = Py::None;
    let mut v_jp: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_lim: Py = Py::None;
    let mut v_m_p: Py = Py::None;
    let mut v_n_o: Py = Py::None;
    let mut v_na: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_other: Py = Py::None;
    let mut v_ra: Py = Py::None;
    let mut v_rb: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let mut v_species: Py = Py::None;
    let mut v_xa: Py = Py::None;
    let mut v_xb: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m_a"), v_m_a.clone()), (Py::str("m_b"), v_m_b.clone())]))?;
    let t1533 = (f_balance(v_equation.clone())?).items()?;
    if t1533.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t1533[0].clone();
    v_species = t1533[1].clone();
    v_nr = t1533[2].clone();
    let t1538 = (f__role(v_species.clone(), v_nr.clone(), v_reactant_a.clone())?).items()?;
    if t1538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ja = t1538[0].clone();
    v_ra = t1538[1].clone();
    let t1539 = (f__role(v_species.clone(), v_nr.clone(), v_reactant_b.clone())?).items()?;
    if t1539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jb = t1539[0].clone();
    v_rb = t1539[1].clone();
    let t1540 = (f__role(v_species.clone(), v_nr.clone(), v_product.clone())?).items()?;
    if t1540.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jp = t1540[0].clone();
    v_rp = t1540[1].clone();
    if ({ let t1544 = py::not(&{ let t1541 = v_ra.clone(); if !t1541.truthy() { t1541 } else { v_rb.clone() } }); if t1544.truthy() { t1544 } else { { let t1542 = v_ja.clone(); Py::Bool({ let t1543 = v_jb.clone(); if py::eq(&t1542, &t1543) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two given substances must be two different reactants"))));
    }
    if (v_rp.clone()).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_product.clone(), "")?, " is a reactant, not a product".to_string()].concat()))));
    }
    let t1547 = (Py::tuple(vec![py::mul(&py::truediv(&v_m_a.clone(), &f_molar_mass_g(v_reactant_a.clone())?)?, &Py::Float(1000.0))?, py::mul(&py::truediv(&v_m_b.clone(), &f_molar_mass_g(v_reactant_b.clone())?)?, &Py::Float(1000.0))?])).items()?;
    if t1547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_na = t1547[0].clone();
    v_nb = t1547[1].clone();
    let t1548 = (Py::tuple(vec![py::truediv(&v_na.clone(), &py::getitem(&v_coefs.clone(), &v_ja.clone())?)?, py::truediv(&v_nb.clone(), &py::getitem(&v_coefs.clone(), &v_jb.clone())?)?])).items()?;
    if t1548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xa = t1548[0].clone();
    v_xb = t1548[1].clone();
    let t1551 = ((if ({ let t1549 = v_xa.clone(); Py::Bool({ let t1550 = v_xb.clone(); if py::le(&t1549, &t1550)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_xa.clone(), v_reactant_a.clone(), v_reactant_b.clone(), v_nb.clone(), v_jb.clone()]) } else { Py::tuple(vec![v_xb.clone(), v_reactant_b.clone(), v_reactant_a.clone(), v_na.clone(), v_ja.clone()]) })).items()?;
    if t1551.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_ext = t1551[0].clone();
    v_lim = t1551[1].clone();
    v_other = t1551[2].clone();
    v_n_o = t1551[3].clone();
    v_j_o = t1551[4].clone();
    v_m_p = py::truediv(&py::mul(&py::mul(&v_ext.clone(), &py::getitem(&v_coefs.clone(), &v_jp.clone())?)?, &f_molar_mass_g(v_product.clone())?)?, &Py::Float(1000.0))?;
    v_left = py::truediv(&py::mul(&py::sub(&v_n_o.clone(), &py::mul(&v_ext.clone(), &py::getitem(&v_coefs.clone(), &v_j_o.clone())?)?)?, &f_molar_mass_g(v_other.clone())?)?, &Py::Float(1000.0))?;
    return Ok(Py::dict(vec![(Py::str("product_mass"), v_m_p.clone()), (Py::str("limiting"), v_lim.clone()), (Py::str("excess_left"), v_left.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_from_moles(mut v_formula: Py, mut v_n: Py) -> R {
    let mut v_mg: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone())]))?;
    v_mg = f_molar_mass_g(v_formula.clone())?;
    return Ok(Py::dict(vec![(Py::str("m"), py::truediv(&py::mul(&v_n.clone(), &v_mg.clone())?, &Py::Float(1000.0))?), (Py::str("m_g"), py::mul(&v_n.clone(), &v_mg.clone())?), (Py::str("N"), py::mul(&v_n.clone(), &c_NA()?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_of_element(mut v_formula: Py, mut v_element: Py, mut v_m: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_frac: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone())]))?;
    v_el = f__element(v_element.clone())?;
    v_counts = f_formula_counts(v_formula.clone())?;
    if ({ let t1684 = v_el.clone(); Py::Bool({ let t1685 = v_counts.clone(); if !py::contains(&t1685, &t1684)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_el.clone(), "")?, " does not occur in ".to_string(), py::format(&v_formula.clone(), "")?].concat()))));
    }
    v_frac = py::truediv(&py::mul(&py::getitem(&v_counts.clone(), &v_el.clone())?, &py::getitem(&c_W()?, &v_el.clone())?)?, &f_molar_mass_g(v_formula.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("m_element"), py::mul(&v_m.clone(), &v_frac.clone())?), (Py::str("fraction"), v_frac.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_percent(mut v_formula: Py, mut v_element: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_counts: Py = Py::None;
    let mut v_el: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_el = f__element(v_element.clone())?;
    v_counts = f_formula_counts(v_formula.clone())?;
    if ({ let t1748 = v_el.clone(); Py::Bool({ let t1749 = v_counts.clone(); if !py::contains(&t1749, &t1748)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_el.clone(), "")?, " does not occur in ".to_string(), py::format(&v_formula.clone(), "")?].concat()))));
    }
    v_total = f_molar_mass_g(v_formula.clone())?;
    v_comp = { let mut out = vec![]; for t1752 in py::iter(&(py::method(&v_counts.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_n: Py; 
let t1753 = (t1752).items()?;
if t1753.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t1753[0].clone();
v_n = t1753[1].clone();
out.push((v_e.clone(), py::truediv(&py::mul(&py::mul(&Py::Float(100.0), &v_n.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?, &v_total.clone())?)); } Py::dict(out) };
    return Ok(Py::dict(vec![(Py::str("percent"), py::getitem(&v_comp.clone(), &v_el.clone())?), (Py::str("composition"), v_comp.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mean_speed(mut v_formula: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("u_mean"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(8i64), &c_R()?)?, &v_T.clone())?, &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &f_molar_mass_g(v_formula.clone())?)?, &Py::Float(1000.0))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molality(mut v_formula: Py, mut v_m_solute: Py, mut v_m_solvent: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("m_solute"), v_m_solute.clone()), (Py::str("m_solvent"), v_m_solvent.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("b"), py::truediv(&py::truediv(&v_m_solute.clone(), &py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?)?, &v_m_solvent.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molar_mass(mut v_formula: Py) -> R {
    let mut v_mg: Py = Py::None;
    v_mg = f_molar_mass_g(v_formula.clone())?;
    return Ok(Py::dict(vec![(Py::str("M"), py::truediv(&v_mg.clone(), &Py::Float(1000.0))?), (Py::str("M_g"), v_mg.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molar_mass_from_density(mut v_rho: Py, mut v_P: Py, mut v_T: Py) -> R {
    let mut v_M: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("rho"), v_rho.clone()), (Py::str("P"), v_P.clone()), (Py::str("T"), v_T.clone())]))?;
    v_M = py::truediv(&py::mul(&py::mul(&v_rho.clone(), &c_R()?)?, &v_T.clone())?, &v_P.clone())?;
    return Ok(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("M_g"), py::mul(&v_M.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molar_mass_from_freezing(mut v_m_solute: Py, mut v_m_solvent: Py, mut v_K: Py, mut v_dT: Py, mut v_i: Py) -> R {
    let mut v_M: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m_solute"), v_m_solute.clone()), (Py::str("m_solvent"), v_m_solvent.clone()), (Py::str("K"), v_K.clone()), (Py::str("dT"), v_dT.clone()), (Py::str("i"), v_i.clone())]))?;
    v_M = py::truediv(&py::mul(&py::mul(&v_i.clone(), &v_K.clone())?, &v_m_solute.clone())?, &py::mul(&v_dT.clone(), &v_m_solvent.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("M_g"), py::mul(&v_M.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molar_mass_from_osmotic(mut v_m: Py, mut v_V: Py, mut v_Pi: Py, mut v_T: Py, mut v_i: Py) -> R {
    let mut v_M: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("V"), v_V.clone()), (Py::str("Pi"), v_Pi.clone()), (Py::str("T"), v_T.clone()), (Py::str("i"), v_i.clone())]))?;
    v_M = py::truediv(&py::mul(&py::mul(&py::mul(&v_i.clone(), &v_m.clone())?, &c_R()?)?, &v_T.clone())?, &py::mul(&v_Pi.clone(), &v_V.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("M_g"), py::mul(&v_M.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molar_mass_from_sample(mut v_m: Py, mut v_V: Py, mut v_P: Py, mut v_T: Py) -> R {
    let mut v_M: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("V"), v_V.clone()), (Py::str("P"), v_P.clone()), (Py::str("T"), v_T.clone())]))?;
    v_M = py::truediv(&py::mul(&py::mul(&v_m.clone(), &c_R()?)?, &v_T.clone())?, &py::mul(&v_P.clone(), &v_V.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("M_g"), py::mul(&v_M.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molar_mass_g(mut v_text: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t3436 in py::iter(&(py::method(&f_formula_counts(v_text.clone())?, "items", &[])?))? { let mut v_el: Py; let mut v_n: Py; 
let t3437 = (t3436).items()?;
if t3437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_el = t3437[0].clone();
v_n = t3437[1].clone();
out.push(py::mul(&v_n.clone(), &py::getitem(&c_W()?, &v_el.clone())?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molarity_from_mass(mut v_formula: Py, mut v_m: Py, mut v_V: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("V"), v_V.clone())]))?;
    v_c = py::truediv(&py::truediv(&v_m.clone(), &py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?)?, &v_V.clone())?;
    return Ok(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("c_molL"), py::truediv(&v_c.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mole_fraction(mut v_formula_solute: Py, mut v_m_solute: Py, mut v_formula_solvent: Py, mut v_m_solvent: Py) -> R {
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m_solute"), v_m_solute.clone()), (Py::str("m_solvent"), v_m_solvent.clone())]))?;
    let t2788 = (Py::tuple(vec![py::truediv(&v_m_solute.clone(), &f_molar_mass_g(v_formula_solute.clone())?)?, py::truediv(&v_m_solvent.clone(), &f_molar_mass_g(v_formula_solvent.clone())?)?])).items()?;
    if t2788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t2788[0].clone();
    v_n2 = t2788[1].clone();
    return Ok(Py::dict(vec![(Py::str("x_solute"), py::truediv(&v_n1.clone(), &py::add(&v_n1.clone(), &v_n2.clone())?)?), (Py::str("x_solvent"), py::truediv(&v_n2.clone(), &py::add(&v_n1.clone(), &v_n2.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molecular_formula(mut v_empirical: Py, mut v_M: Py) -> R {
    let mut v_counts: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_me: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("M"), v_M.clone())]))?;
    v_counts = f_formula_counts(v_empirical.clone())?;
    v_order = Py::list((py::dict_fromkeys(&ax_engine::pyre::module_call("findall", &Py::str("[A-Z][a-z]?"), &[v_empirical.clone()], 0)?, Py::None)?).items()?);
    v_me = py::truediv(&f_molar_mass_g(v_empirical.clone())?, &Py::Float(1000.0))?;
    v_k = py::truediv(&v_M.clone(), &v_me.clone())?;
    v_n = py::round(&v_k.clone(), None)?;
    if ({ let t2177 = { let t2173 = v_n.clone(); Py::Bool({ let t2174 = Py::Int(1i64); if py::lt(&t2173, &t2174)? { true } else { false } }) }; if t2177.truthy() { t2177 } else { { let t2175 = py::abs(&py::sub(&v_k.clone(), &v_n.clone())?)?; Py::Bool({ let t2176 = py::mul(&Py::Float(0.02), &v_n.clone())?; if py::gt(&t2175, &t2176)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the molar mass is ".to_string(), py::format(&v_k.clone(), ".3f")?, " times the empirical formula mass, not a whole multiple".to_string()].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("multiple"), py::float(&v_n.clone())?), (Py::str("formula"), f__write_formula(v_order.clone(), { let mut out = vec![]; for t2181 in py::iter(&(py::method(&v_counts.clone(), "items", &[])?))? { let mut v_c: Py; let mut v_e: Py; 
let t2182 = (t2181).items()?;
if t2182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t2182[0].clone();
v_c = t2182[1].clone();
out.push((v_e.clone(), py::mul(&v_c.clone(), &v_n.clone())?)); } Py::dict(out) })?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moles_from_mass(mut v_formula: Py, mut v_m: Py) -> R {
    let mut v_mg: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone())]))?;
    v_mg = f_molar_mass_g(v_formula.clone())?;
    v_n = py::truediv(&v_m.clone(), &py::truediv(&v_mg.clone(), &Py::Float(1000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("N"), py::mul(&v_n.clone(), &c_NA()?)?), (Py::str("M"), py::truediv(&v_mg.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moles_from_particles(mut v_N: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("N"), v_N.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("n"), py::truediv(&v_N.clone(), &c_NA()?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nernst(mut v_E0: Py, mut v_n: Py, mut v_Q: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("Q"), v_Q.clone()), (Py::str("T"), v_T.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("E0"), v_E0.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("E"), py::sub(&v_E0.clone(), &py::mul(&py::truediv(&py::mul(&c_R()?, &v_T.clone())?, &py::mul(&v_n.clone(), &c_F()?)?)?, &py::math1("log", &v_Q.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nuclear_binding_energy(mut v_Z: Py, mut v_A: Py, mut v_mass: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_dm: Py = Py::None;
    let t2306 = (Py::tuple(vec![py::int(&v_Z.clone())?, py::int(&v_A.clone())?])).items()?;
    if t2306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Z = t2306[0].clone();
    v_A = t2306[1].clone();
    if ({ let t2314 = { let t2307 = v_A.clone(); Py::Bool({ let t2308 = Py::Int(2i64); if py::lt(&t2307, &t2308)? { true } else { false } }) }; if t2314.truthy() { t2314 } else { { let t2313 = { let t2309 = v_Z.clone(); Py::Bool({ let t2310 = Py::Int(1i64); if py::lt(&t2309, &t2310)? { true } else { false } }) }; if t2313.truthy() { t2313 } else { { let t2311 = v_Z.clone(); Py::Bool({ let t2312 = v_A.clone(); if py::gt(&t2311, &t2312)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("need 1 <= Z <= A and A >= 2 (a nucleus of at least two nucleons)"))));
    }
    let _ = f__pos(Py::dict(vec![(Py::str("mass"), v_mass.clone())]))?;
    if ({ let t2321 = py::abs(&py::sub(&v_mass.clone(), &v_A.clone())?)?; Py::Bool({ let t2322 = Py::Float(0.5); if py::ge(&t2321, &t2322)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the atomic mass (in u) must lie within 0.5 u of the mass number A"))));
    }
    v_dm = py::sub(&py::add(&py::mul(&v_Z.clone(), &c_M_H1_U()?)?, &py::mul(&py::sub(&v_A.clone(), &v_Z.clone())?, &c_M_N_U()?)?)?, &v_mass.clone())?;
    if (py::not(&{ let t2323 = v_dm.clone(); Py::Bool({ let t2324 = Py::Int(0i64); if py::gt(&t2323, &t2324)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a bound nucleus is lighter than its separate nucleons (the mass defect must be positive)"))));
    }
    v_E = py::mul(&v_dm.clone(), &c_U_MEV()?)?;
    return Ok(Py::dict(vec![(Py::str("dm"), v_dm.clone()), (Py::str("E_b"), py::mul(&v_E.clone(), &c_MEV_J()?)?), (Py::str("E_b_MeV"), v_E.clone()), (Py::str("E_per_nucleon_MeV"), py::truediv(&v_E.clone(), &v_A.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_i: Py, mut v_c: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("c"), v_c.clone()), (Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("Pi"), py::mul(&py::mul(&py::mul(&v_i.clone(), &v_c.clone())?, &c_R()?)?, &v_T.clone())?), (Py::str("Pi_atm"), py::truediv(&py::mul(&py::mul(&py::mul(&v_i.clone(), &v_c.clone())?, &c_R()?)?, &v_T.clone())?, &c_ATM()?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parse_equation(mut v_text: Py) -> R {
    let mut v_coef: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_names: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_sides: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_term: Py = Py::None;
    let mut v_terms: Py = Py::None;
    if (py::not(&Py::Bool(matches!(v_text.clone(), Py::Str(_))))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an equation is text, such as 'H2 + O2 -> H2O'"))));
    }
    v_t = py::method(&c__STATE()?, "sub", &[Py::str(""), py::method(&v_text.clone(), "translate", &[c__SUBSCRIPT()?])?])?;
    v_sides = py::method(&c__ARROW()?, "split", &[py::method(&v_t.clone(), "strip", &[])?])?;
    if ({ let t3341 = { let t3338 = py::len(&v_sides.clone())?; Py::Bool({ let t3339 = Py::Int(2i64); if py::ne(&t3338, &t3339) { true } else { false } }) }; if t3341.truthy() { t3341 } else { { let t3340 = py::not(&py::method(&py::getitem(&v_sides.clone(), &Py::Int(0i64))?, "strip", &[])?); if t3340.truthy() { t3340 } else { py::not(&py::method(&py::getitem(&v_sides.clone(), &Py::Int(1i64))?, "strip", &[])?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an equation needs reactants, one arrow and products"))));
    }
    v_out = Py::list(vec![]);
    for t3342 in py::iter(&(v_sides.clone()))? {
        v_side = t3342;
        v_terms = Py::list(vec![]);
        for t3343 in py::iter(&(py::method(&v_side.clone(), "split", &[Py::str("+")])?))? {
            v_term = t3343;
            v_term = py::method(&v_term.clone(), "strip", &[])?;
            v_m = ax_engine::pyre::module_call("fullmatch", &Py::str("(\\d+(?:/\\d+)?)?\\s*(\\S+)"), &[v_term.clone()], 0)?;
            if (py::not(&v_m.clone())).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["cannot read the term '".to_string(), py::format(&v_term.clone(), "")?, "'".to_string()].concat()))));
            }
            v_coef = (if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { py::fraction(&[py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?])? } else { py::fraction(&[Py::Int(1i64)])? });
            if ({ let t3344 = v_coef.clone(); Py::Bool({ let t3345 = Py::Int(0i64); if py::le(&t3344, &t3345)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("coefficients must be positive"))));
            }
            let _ = f_formula_counts(py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)?;
            let _ = py::method(&v_terms.clone(), "append", &[Py::tuple(vec![v_coef.clone(), py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?])])?;
        }
        let _ = py::method(&v_out.clone(), "append", &[v_terms.clone()])?;
    }
    v_names = { let mut out = vec![]; for t3370 in py::iter(&(py::add(&py::getitem(&v_out.clone(), &Py::Int(0i64))?, &py::getitem(&v_out.clone(), &Py::Int(1i64))?)?))? { let mut v__c: Py; let mut v_f: Py; 
let t3371 = (t3370).items()?;
if t3371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t3371[0].clone();
v_f = t3371[1].clone();
out.push(v_f.clone()); } Py::list(out) };
    if ({ let t3372 = py::len(&py::set(&v_names.clone())?)?; Py::Bool({ let t3373 = py::len(&v_names.clone())?; if py::ne(&t3372, &t3373) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a species appears more than once"))));
    }
    return Ok(Py::tuple(vec![py::getitem(&v_out.clone(), &Py::Int(0i64))?, py::getitem(&v_out.clone(), &Py::Int(1i64))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_pressure(mut v_x: Py, mut v_P_total: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P_total"), v_P_total.clone())]))?;
    if (py::not(&{ let t2344 = Py::Int(0i64); Py::Bool({ let t2345 = v_x.clone(); if py::lt(&t2344, &t2345)? { { let t2346 = Py::Int(1i64); if py::le(&t2345, &t2346)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a mole fraction lies between 0 and 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("p"), py::mul(&v_x.clone(), &v_P_total.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_particles_from_mass(mut v_formula: Py, mut v_m: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone())]))?;
    v_n = py::truediv(&v_m.clone(), &py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("N"), py::mul(&v_n.clone(), &c_NA()?)?), (Py::str("n"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_particles_from_moles(mut v_n: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("N"), py::mul(&v_n.clone(), &c_NA()?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_percent_yield(mut v_equation: Py, mut v_reactant: Py, mut v_m: Py, mut v_product: Py, mut v_actual: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("actual"), v_actual.clone())]))?;
    v_t = py::getitem(&f_theoretical_yield(v_equation.clone(), v_reactant.clone(), v_m.clone(), v_product.clone())?, &Py::str("product_mass"))?;
    return Ok(Py::dict(vec![(Py::str("percent_yield"), py::truediv(&py::mul(&Py::Float(100.0), &v_actual.clone())?, &v_t.clone())?), (Py::str("theoretical_mass"), v_t.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_diprotic(mut v_Ka1: Py, mut v_Ka2: Py, mut v_c: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ka1"), v_Ka1.clone()), (Py::str("Ka2"), v_Ka2.clone()), (Py::str("c"), v_c.clone())]))?;
    v_C = py::truediv(&v_c.clone(), &Py::Float(1000.0))?;
    v_f = { let v_C = v_C.clone(); let v_Ka1 = v_Ka1.clone(); let v_Ka2 = v_Ka2.clone(); let v_c = v_c.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&{ let v_C = v_C.clone(); let v_Ka1 = v_Ka1.clone(); let v_Ka2 = v_Ka2.clone(); let v_c = v_c.clone(); let v_u = v_u.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&v_h.clone(), &py::truediv(&py::mul(&v_C.clone(), &py::add(&py::mul(&v_Ka1.clone(), &v_h.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_Ka1.clone())?, &v_Ka2.clone())?)?)?, &py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&v_Ka1.clone(), &v_h.clone())?)?, &py::mul(&v_Ka1.clone(), &v_Ka2.clone())?)?)?)?, &py::truediv(&c_KW()?, &v_h.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::math1("exp", &v_u.clone())?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_h = py::math1("exp", &f__bisect(v_f.clone(), py::sub(&py::math1("log", &Py::Float(1e-07))?, &Py::Float(1e-09))?, py::math1("log", &py::add(&py::mul(&Py::Int(2i64), &v_C.clone())?, &Py::Float(1e-06))?)?, Py::Int(400i64))?)?;
    return Ok(Py::dict(vec![(Py::str("pH"), f__ph(v_h.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_from_h(mut v_h: Py) -> R {
    let mut v_pH: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("h"), v_h.clone())]))?;
    v_pH = f__ph(py::truediv(&v_h.clone(), &Py::Float(1000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("pH"), v_pH.clone()), (Py::str("pOH"), py::sub(&Py::Float(14.0), &v_pH.clone())?), (Py::str("oh"), py::mul(&py::truediv(&c_KW()?, &py::truediv(&v_h.clone(), &Py::Float(1000.0))?)?, &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_from_oh(mut v_oh: Py) -> R {
    let mut v_pOH: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("oh"), v_oh.clone())]))?;
    v_pOH = f__ph(py::truediv(&v_oh.clone(), &Py::Float(1000.0))?)?;
    return Ok(Py::dict(vec![(Py::str("pH"), py::sub(&Py::Float(14.0), &v_pOH.clone())?), (Py::str("pOH"), v_pOH.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_salt_weak_acid(mut v_Ka: Py, mut v_c: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ka"), v_Ka.clone()), (Py::str("c"), v_c.clone())]))?;
    v_r = f_ph_weak_base(py::truediv(&c_KW()?, &v_Ka.clone())?, v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("pH"), py::getitem(&v_r.clone(), &Py::str("pH"))?), (Py::str("Kb"), py::truediv(&c_KW()?, &v_Ka.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_strong_acid(mut v_c: Py) -> R {
    let mut v_cl: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c"), v_c.clone())]))?;
    v_cl = py::truediv(&v_c.clone(), &Py::Float(1000.0))?;
    v_h = py::add(&py::truediv(&v_cl.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_cl.clone(), &v_cl.clone())?, &Py::Int(4i64))?, &c_KW()?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("pH"), f__ph(v_h.clone())?), (Py::str("h"), py::mul(&v_h.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_strong_base(mut v_c: Py, mut v_n_oh: Py) -> R {
    let mut v_oh: Py = Py::None;
    let mut v_oh0: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c"), v_c.clone())]))?;
    v_oh0 = py::mul(&py::truediv(&v_c.clone(), &Py::Float(1000.0))?, &py::int(&v_n_oh.clone())?)?;
    v_oh = py::add(&py::truediv(&v_oh0.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_oh0.clone(), &v_oh0.clone())?, &Py::Int(4i64))?, &c_KW()?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("pH"), py::sub(&Py::Float(14.0), &f__ph(v_oh.clone())?)?), (Py::str("pOH"), f__ph(v_oh.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_weak_acid(mut v_Ka: Py, mut v_c: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ka"), v_Ka.clone()), (Py::str("c"), v_c.clone())]))?;
    v_C = py::truediv(&v_c.clone(), &Py::Float(1000.0))?;
    v_h = f__weak_h(v_Ka.clone(), v_C.clone())?;
    v_a = py::truediv(&py::mul(&v_Ka.clone(), &v_C.clone())?, &py::add(&v_Ka.clone(), &v_h.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("pH"), f__ph(v_h.clone())?), (Py::str("h"), py::mul(&v_h.clone(), &Py::Float(1000.0))?), (Py::str("percent_ionized"), py::truediv(&py::mul(&Py::Float(100.0), &v_a.clone())?, &v_C.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ph_weak_base(mut v_Kb: Py, mut v_c: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_oh: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Kb"), v_Kb.clone()), (Py::str("c"), v_c.clone())]))?;
    v_C = py::truediv(&v_c.clone(), &Py::Float(1000.0))?;
    v_oh = f__weak_h(v_Kb.clone(), v_C.clone())?;
    return Ok(Py::dict(vec![(Py::str("pH"), py::sub(&Py::Float(14.0), &f__ph(v_oh.clone())?)?), (Py::str("pOH"), f__ph(v_oh.clone())?), (Py::str("percent_ionized"), py::truediv(&py::mul(&Py::Float(100.0), &v_Kb.clone())?, &py::add(&v_Kb.clone(), &v_oh.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_photon_energy(mut v_lam: Py) -> R {
    let mut v_E: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("lam"), v_lam.clone())]))?;
    v_E = py::truediv(&py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?, &v_lam.clone())?;
    return Ok(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_eV"), py::truediv(&v_E.clone(), &c_EV()?)?), (Py::str("E_molar"), py::mul(&v_E.clone(), &c_NA()?)?), (Py::str("nu"), py::truediv(&c_C_LIGHT()?, &v_lam.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_photon_wavelength(mut v_E: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("E"), v_E.clone())]))?;
    v_lam = py::truediv(&py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?, &v_E.clone())?;
    return Ok(Py::dict(vec![(Py::str("lam"), v_lam.clone()), (Py::str("lam_nm"), py::mul(&v_lam.clone(), &Py::Float(1000000000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_precipitation_test(mut v_Ksp: Py, mut v_x: Py, mut v_y: Py, mut v_c_cation: Py, mut v_c_anion: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("Ksp"), v_Ksp.clone()), (Py::str("c_cation"), v_c_cation.clone()), (Py::str("c_anion"), v_c_anion.clone())]))?;
    let t2713 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t2713.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2713[0].clone();
    v_y = t2713[1].clone();
    if ({ let t2718 = { let t2714 = v_x.clone(); Py::Bool({ let t2715 = Py::Int(1i64); if py::lt(&t2714, &t2715)? { true } else { false } }) }; if t2718.truthy() { t2718 } else { { let t2716 = v_y.clone(); Py::Bool({ let t2717 = Py::Int(1i64); if py::lt(&t2716, &t2717)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and y are whole numbers >= 1"))));
    }
    v_a = py::truediv(&v_c_cation.clone(), &Py::Float(1000.0))?;
    v_b = py::truediv(&v_c_anion.clone(), &Py::Float(1000.0))?;
    v_lg = py::add(&py::mul(&v_x.clone(), &py::math1("log10", &v_a.clone())?)?, &py::mul(&v_y.clone(), &py::math1("log10", &v_b.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), py::mul(&py::pow(&v_a.clone(), &v_x.clone())?, &py::pow(&v_b.clone(), &v_y.clone())?)?), (Py::str("log10_Q"), v_lg.clone()), (Py::str("result"), f__precip_word(v_lg.clone(), py::math1("log10", &v_Ksp.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radioactive_age(mut v_fraction: Py, mut v_t_half: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("t_half"), v_t_half.clone())]))?;
    if (py::not(&{ let t2734 = Py::Int(0i64); Py::Bool({ let t2735 = v_fraction.clone(); if py::lt(&t2734, &t2735)? { { let t2736 = Py::Int(1i64); if py::le(&t2735, &t2736)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the fraction remaining lies between 0 and 1"))));
    }
    v_n = py::add(&py::truediv(&py::neg(&py::math1("log", &v_fraction.clone())?)?, &c_LN2()?)?, &Py::Float(0.0))?;
    v_t = py::mul(&v_n.clone(), &v_t_half.clone())?;
    return Ok(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_years"), py::truediv(&v_t.clone(), &c_YEAR_S()?)?), (Py::str("half_lives"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radioactive_remaining(mut v_m0: Py, mut v_t_half: Py, mut v_t: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m0"), v_m0.clone()), (Py::str("t_half"), v_t_half.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("t"), v_t.clone())]))?;
    if ({ let t2750 = v_t.clone(); Py::Bool({ let t2751 = Py::Int(0i64); if py::lt(&t2750, &t2751)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the elapsed time cannot be negative"))));
    }
    v_n = py::truediv(&v_t.clone(), &v_t_half.clone())?;
    v_f = py::pow(&Py::Float(0.5), &v_n.clone())?;
    v_m = py::mul(&v_m0.clone(), &v_f.clone())?;
    return Ok(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("m_g"), py::mul(&v_m.clone(), &Py::Float(1000.0))?), (Py::str("fraction"), v_f.clone()), (Py::str("half_lives"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_raoult(mut v_formula_solute: Py, mut v_m_solute: Py, mut v_formula_solvent: Py, mut v_m_solvent: Py, mut v_P0: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("P0"), v_P0.clone())]))?;
    v_x = py::getitem(&f_mole_fraction(v_formula_solute.clone(), v_m_solute.clone(), v_formula_solvent.clone(), v_m_solvent.clone())?, &Py::str("x_solvent"))?;
    return Ok(Py::dict(vec![(Py::str("P"), py::mul(&v_x.clone(), &v_P0.clone())?), (Py::str("dP"), py::mul(&py::sub(&Py::Int(1i64), &v_x.clone())?, &v_P0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_entropy(mut v_equation: Py, mut v_values: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("dS"), f__coef_sum(v_equation.clone(), v_values.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_gibbs_formation(mut v_equation: Py, mut v_values: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = f__coef_sum(v_equation.clone(), v_values.clone())?;
    return Ok(Py::dict(vec![(Py::str("dG"), py::mul(&v_g.clone(), &Py::Float(1000.0))?), (Py::str("dG_kJ"), v_g.clone()), (Py::str("spontaneity"), f__gibbs_word(v_g.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_order(mut v_c1: Py, mut v_r1: Py, mut v_c2: Py, mut v_r2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("c1"), v_c1.clone()), (Py::str("r1"), v_r1.clone()), (Py::str("c2"), v_c2.clone()), (Py::str("r2"), v_r2.clone())]))?;
    if ({ let t3097 = v_c1.clone(); Py::Bool({ let t3098 = v_c2.clone(); if py::eq(&t3097, &t3098) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two concentrations must differ"))));
    }
    return Ok(Py::dict(vec![(Py::str("order"), py::truediv(&py::math1("log", &py::truediv(&v_r2.clone(), &v_r1.clone())?)?, &py::math1("log", &py::truediv(&v_c2.clone(), &v_c1.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_quotient(mut v_equation: Py, mut v_conc: Py, mut v_K: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_lg: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_sp: Py = Py::None;
    let t3143 = (f__ice_terms(v_equation.clone())?).items()?;
    if t3143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nu = t3143[0].clone();
    v_sp = t3143[1].clone();
    if ({ let t3144 = py::len(&v_conc.clone())?; Py::Bool({ let t3145 = py::len(&v_sp.clone())?; if py::ne(&t3144, &t3145) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one concentration per species (".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_sp.clone()])?, "")?, ")".to_string()].concat()))));
    }
    for t3146 in py::iter(&(v_conc.clone()))? {
        v_c = t3146;
        if (py::not(&{ let t3147 = v_c.clone(); Py::Bool({ let t3148 = Py::Int(0i64); if py::gt(&t3147, &t3148)? { true } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every concentration must be above zero"))));
        }
    }
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone())]))?;
    v_Q = Py::Float(1.0);
    v_lg = Py::Float(0.0);
    for t3155 in py::iter(&(py::zip(&[v_conc.clone(), v_nu.clone()])?))? {
        let t3156 = (t3155).items()?;
        if t3156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t3156[0].clone();
        v_n = t3156[1].clone();
        v_Q = py::mul(&v_Q, &py::pow(&v_c.clone(), &v_n.clone())?)?;
        v_lg = py::iadd(&v_lg, &py::mul(&v_n.clone(), &py::math1("log10", &v_c.clone())?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("log10_Q"), v_lg.clone()), (Py::str("direction"), f__shift_word(v_lg.clone(), py::math1("log10", &v_K.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rms_speed(mut v_formula: Py, mut v_T: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("T"), v_T.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("u_rms"), py::math1("sqrt", &py::truediv(&py::mul(&py::mul(&Py::Int(3i64), &c_R()?)?, &v_T.clone())?, &py::truediv(&f_molar_mass_g(v_formula.clone())?, &Py::Float(1000.0))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_conc(mut v_A0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("A0"), v_A0.clone()), (Py::str("k"), v_k.clone()), (Py::str("t"), v_t.clone())]))?;
    v_A = py::truediv(&Py::Float(1.0), &py::add(&py::truediv(&Py::Float(1.0), &v_A0.clone())?, &py::mul(&v_k.clone(), &v_t.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A_molL"), py::truediv(&v_A.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solute_mass(mut v_formula: Py, mut v_c: Py, mut v_V: Py) -> R {
    let mut v_m: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("V"), v_V.clone())]))?;
    v_m = py::truediv(&py::mul(&py::mul(&v_c.clone(), &v_V.clone())?, &f_molar_mass_g(v_formula.clone())?)?, &Py::Float(1000.0))?;
    return Ok(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("m_g"), py::mul(&v_m.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_specific_rotation(mut v_alpha: Py, mut v_l: Py, mut v_c: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("alpha"), v_alpha.clone())]))?;
    let _ = f__pos(Py::dict(vec![(Py::str("l"), v_l.clone()), (Py::str("c"), v_c.clone())]))?;
    return Ok(Py::dict(vec![(Py::str("specific_rotation"), py::truediv(&py::mul(&Py::Float(100.0), &v_alpha.clone())?, &py::mul(&v_l.clone(), &v_c.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_standard_cell_potential(mut v_E_cathode: Py, mut v_E_anode: Py) -> R {
    let mut v_E: Py = Py::None;
    let _ = f__finite(Py::dict(vec![(Py::str("E_cathode"), v_E_cathode.clone()), (Py::str("E_anode"), v_E_anode.clone())]))?;
    if ({ let t3324 = { let t3320 = py::abs(&v_E_cathode.clone())?; Py::Bool({ let t3321 = Py::Float(3.5); if py::gt(&t3320, &t3321)? { true } else { false } }) }; if t3324.truthy() { t3324 } else { { let t3322 = py::abs(&v_E_anode.clone())?; Py::Bool({ let t3323 = Py::Float(3.5); if py::gt(&t3322, &t3323)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("standard reduction potentials lie between about -3.1 V and +3.0 V (|E°| <= 3.5 V accepted)"))));
    }
    v_E = py::sub(&v_E_cathode.clone(), &v_E_anode.clone())?;
    return Ok(Py::dict(vec![(Py::str("E_cell"), v_E.clone()), (Py::str("spontaneity"), f__cell_word(v_E.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_theoretical_yield(mut v_equation: Py, mut v_reactant: Py, mut v_m: Py, mut v_product: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_jp: Py = Py::None;
    let mut v_jr: Py = Py::None;
    let mut v_n_p: Py = Py::None;
    let mut v_nr: Py = Py::None;
    let mut v_rp: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let mut v_species: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("m"), v_m.clone())]))?;
    let t3428 = (f_balance(v_equation.clone())?).items()?;
    if t3428.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t3428[0].clone();
    v_species = t3428[1].clone();
    v_nr = t3428[2].clone();
    let t3433 = (f__role(v_species.clone(), v_nr.clone(), v_reactant.clone())?).items()?;
    if t3433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jr = t3433[0].clone();
    v_rr = t3433[1].clone();
    let t3434 = (f__role(v_species.clone(), v_nr.clone(), v_product.clone())?).items()?;
    if t3434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jp = t3434[0].clone();
    v_rp = t3434[1].clone();
    if ({ let t3435 = py::not(&v_rr.clone()); if t3435.truthy() { t3435 } else { v_rp.clone() } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give a reactant and a product of the equation"))));
    }
    v_n_p = py::truediv(&py::mul(&py::truediv(&py::mul(&v_m.clone(), &Py::Float(1000.0))?, &f_molar_mass_g(v_reactant.clone())?)?, &py::getitem(&v_coefs.clone(), &v_jp.clone())?)?, &py::getitem(&v_coefs.clone(), &v_jr.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("product_mass"), py::truediv(&py::mul(&v_n_p.clone(), &f_molar_mass_g(v_product.clone())?)?, &Py::Float(1000.0))?), (Py::str("n_product"), v_n_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vant_hoff(mut v_K1: Py, mut v_T1: Py, mut v_T2: Py, mut v_dH: Py) -> R {
    let mut v_lnK2: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("K1"), v_K1.clone()), (Py::str("T1"), v_T1.clone()), (Py::str("T2"), v_T2.clone())]))?;
    let _ = f__finite(Py::dict(vec![(Py::str("dH"), v_dH.clone())]))?;
    v_lnK2 = py::sub(&py::math1("log", &v_K1.clone())?, &py::mul(&py::truediv(&v_dH.clone(), &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &v_T2.clone())?, &py::truediv(&Py::Int(1i64), &v_T1.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("K2"), py::math1("exp", &v_lnK2.clone())?), (Py::str("lnK2"), v_lnK2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vdw_pressure(mut v_n: Py, mut v_V: Py, mut v_T: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_P: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("V"), v_V.clone()), (Py::str("T"), v_T.clone()), (Py::str("a"), v_a.clone()), (Py::str("b"), v_b.clone())]))?;
    if ({ let t3525 = v_V.clone(); Py::Bool({ let t3526 = py::mul(&v_n.clone(), &v_b.clone())?; if py::le(&t3525, &t3526)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the volume must exceed n*b (the excluded volume)"))));
    }
    v_P = py::sub(&py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R()?)?, &v_T.clone())?, &py::sub(&v_V.clone(), &py::mul(&v_n.clone(), &v_b.clone())?)?)?, &py::truediv(&py::mul(&py::mul(&v_a.clone(), &v_n.clone())?, &v_n.clone())?, &py::mul(&v_V.clone(), &v_V.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("P_ideal"), py::truediv(&py::mul(&py::mul(&v_n.clone(), &c_R()?)?, &v_T.clone())?, &v_V.clone())?), (Py::str("P_atm"), py::truediv(&v_P.clone(), &c_ATM()?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vib_wavenumber(mut v_molecule: Py, mut v_k: Py) -> R {
    let mut v_atoms: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_m1: Py = Py::None;
    let mut v_m2: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("k"), v_k.clone())]))?;
    v_c = f_formula_counts(v_molecule.clone())?;
    v_atoms = { let mut out = vec![]; for t3571 in py::iter(&(py::method(&v_c.clone(), "items", &[])?))? { let mut v_el: Py; let mut v_n: Py; 
let t3572 = (t3571).items()?;
if t3572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_el = t3572[0].clone();
v_n = t3572[1].clone();
for t3573 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3573;
out.push(v_el.clone()); }} Py::list(out) };
    if ({ let t3579 = { let t3574 = py::len(&v_atoms.clone())?; Py::Bool({ let t3575 = Py::Int(2i64); if py::ne(&t3574, &t3575) { true } else { false } }) }; if t3579.truthy() { t3579 } else { Py::Bool(({ let mut out = vec![]; for t3576 in py::iter(&(v_atoms.clone()))? { let mut v_a: Py; 
v_a = t3576;
out.push({ let t3577 = v_a.clone(); Py::Bool({ let t3578 = c_ISO()?; if !py::contains(&t3578, &t3577)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a diatomic molecule of H, C, N, O, F, Cl, Br, I, S ... is expected"))));
    }
    let t3580 = (Py::tuple(vec![py::mul(&py::getitem(&c_ISO()?, &py::getitem(&v_atoms.clone(), &Py::Int(0i64))?)?, &c_U_KG()?)?, py::mul(&py::getitem(&c_ISO()?, &py::getitem(&v_atoms.clone(), &Py::Int(1i64))?)?, &c_U_KG()?)?])).items()?;
    if t3580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m1 = t3580[0].clone();
    v_m2 = t3580[1].clone();
    v_mu = py::truediv(&py::mul(&v_m1.clone(), &v_m2.clone())?, &py::add(&v_m1.clone(), &v_m2.clone())?)?;
    v_nu = py::truediv(&py::math1("sqrt", &py::truediv(&v_k.clone(), &v_mu.clone())?)?, &py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?)?;
    return Ok(Py::dict(vec![(Py::str("wavenumber"), py::truediv(&v_nu.clone(), &c_C_LIGHT()?)?), (Py::str("wavenumber_cm"), py::truediv(&py::truediv(&v_nu.clone(), &c_C_LIGHT()?)?, &Py::Float(100.0))?), (Py::str("nu"), v_nu.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zero_order_conc(mut v_A0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("A0"), v_A0.clone()), (Py::str("k"), v_k.clone()), (Py::str("t"), v_t.clone())]))?;
    v_A = py::sub(&v_A0.clone(), &py::mul(&v_k.clone(), &v_t.clone())?)?;
    if ({ let t3592 = v_A.clone(); Py::Bool({ let t3593 = Py::Int(0i64); if py::lt(&t3592, &t3593)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reactant is used up before this time (A0 - k t < 0)"))));
    }
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A_molL"), py::truediv(&v_A.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1027(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &Py::Float(1000.0))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("ratio"))?, &Py::Int(2i64))?)?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_known"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1085(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("ratio"))?, &Py::Int(2i64))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_1"))?)?)?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_115(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_A: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("A_r"))?, v_A.clone())?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &Py::Float(1000.0))?, v_A.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&f_average_atomic_mass(py::getitem(&v_i.clone(), &Py::str("masses"))?, py::getitem(&v_i.clone(), &Py::str("abundances"))?)?, &Py::str("A_r"))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1152(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("q"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("dH"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1190(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("c"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("kH"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("c_molL"))?, &Py::Float(1000.0))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("kH"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1200(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&Py::Float(1.0), &py::getitem(&v_o.clone(), &Py::str("lam"))?)?, py::mul(&c_R_H()?, &py::sub(&py::truediv(&Py::Float(1.0), &py::pow(&py::getitem(&v_i.clone(), &Py::str("n_lower"))?, &Py::Int(2i64))?)?, &py::truediv(&Py::Float(1.0), &py::pow(&py::getitem(&v_i.clone(), &Py::str("n_upper"))?, &Py::Int(2i64))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1207(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("h"))?, py::mul(&Py::Float(1000.0), &py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_i.clone(), &Py::str("pH"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1243(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("m"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1281(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1291(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1301(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_R()?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1311(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1326(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("x1"))?, &py::getitem(&v_i.clone(), &Py::str("m1"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("x2"))?, &py::getitem(&v_i.clone(), &Py::str("m2"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("A_r"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("A_r"))?)?, &py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("x1"))?, &py::getitem(&v_o.clone(), &Py::str("x2"))?)?, &Py::Float(1.0))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("percent1"))?, &py::mul(&Py::Float(100.0), &py::getitem(&v_o.clone(), &Py::str("x1"))?)?)?)?, &Py::Float(100.0))?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("percent2"))?, &py::mul(&Py::Float(100.0), &py::getitem(&v_o.clone(), &Py::str("x2"))?)?)?)?, &Py::Float(100.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1338(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("lnK"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_F()?)?, &py::getitem(&v_i.clone(), &Py::str("E"))?)?)?)?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1348(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::add(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("lnK"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::getitem(&v_i.clone(), &Py::str("dG"))?)?)?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?, &f__rel(py::method(&v_o.clone(), "get", &[Py::str("K"), py::math1("exp", &py::getitem(&v_o.clone(), &Py::str("lnK"))?)?])?, py::math1("exp", &py::getitem(&v_o.clone(), &Py::str("lnK"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1366(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::neg(&py::math1("log10", &f__weak_h(py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_o.clone(), &Py::str("pKa"))?)?)?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("pH"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1393(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Kc"))?, &py::pow(&py::mul(&c_R_LATM()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::getitem(&v_i.clone(), &Py::str("dn"))?)?)?, py::getitem(&v_i.clone(), &Py::str("Kp"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1406(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("Kp"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("Kc"))?, &py::pow(&py::mul(&c_R_LATM()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::getitem(&v_i.clone(), &Py::str("dn"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1417(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::pow(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("Ksp"))?, &py::mul(&py::pow(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("y"))?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?)?, &py::truediv(&Py::Float(1.0), &py::add(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("s"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1433(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::pow(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_o.clone(), &Py::str("s"))?)?, &Py::Float(1000.0))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?, &py::pow(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("y"))?, &py::getitem(&v_o.clone(), &Py::str("s"))?)?, &Py::Float(1000.0))?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?, py::getitem(&v_i.clone(), &Py::str("Ksp"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_16(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t17 = py::getitem(&v_o.clone(), &Py::str("k2"))?; Py::Bool({ let t18 = Py::Int(0i64); if py::gt(&t17, &t18)? { true } else { false } }) }).truthy() { py::abs(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("k2"))?, &py::getitem(&v_i.clone(), &Py::str("k1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("Ea"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1618(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("m"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1688(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("m_element"))?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1754(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("percent"))?, py::truediv(&py::mul(&py::mul(&Py::Float(100.0), &py::getitem(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1818(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("u_mean"))?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::mul(&py::mul(&Py::Int(8i64), &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1882(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("m_solvent"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("m_solute"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1940(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("M_g"))?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &Py::Float(1000.0))?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1978(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("dT"))?)?, &py::getitem(&v_i.clone(), &Py::str("m_solvent"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, &py::getitem(&v_i.clone(), &Py::str("m_solute"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1988(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("rho"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1998(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2008(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("Pi"))?)?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, py::mul(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("m"))?)?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2044(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("c"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("m"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2109(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::getitem(&v_o.clone(), &Py::str("x_solute"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("x_solute"))?)?)?, py::truediv(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solute"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solute"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solvent"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solvent"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2183(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t2221 = { let t2184 = py::getitem(&v_o.clone(), &Py::str("multiple"))?; Py::Bool({ let t2185 = py::round(&py::getitem(&v_o.clone(), &Py::str("multiple"))?, None)?; if py::eq(&t2184, &t2185) { true } else { false } }) }; if !t2221.truthy() { t2221 } else { { let t2220 = { let t2186 = py::abs(&py::sub(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("M"))?, &py::truediv(&f__mm_rd(py::getitem(&v_i.clone(), &Py::str("empirical"))?)?, &Py::Float(1000.0))?)?, &py::getitem(&v_o.clone(), &Py::str("multiple"))?)?)?; Py::Bool({ let t2187 = py::mul(&Py::Float(0.02), &py::getitem(&v_o.clone(), &Py::str("multiple"))?)?; if py::le(&t2186, &t2187)? { true } else { false } }) }; if !t2220.truthy() { t2220 } else { { let t2216 = f__counts_rd(py::getitem(&v_o.clone(), &Py::str("formula"))?)?; Py::Bool({ let t2217 = { let mut out = vec![]; for t2218 in py::iter(&(py::method(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("empirical"))?)?, "items", &[])?))? { let mut v_c: Py; let mut v_e: Py; 
let t2219 = (t2218).items()?;
if t2219.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t2219[0].clone();
v_c = t2219[1].clone();
out.push((v_e.clone(), py::mul(&v_c.clone(), &py::int(&py::getitem(&v_o.clone(), &Py::str("multiple"))?)?)?)); } Py::dict(out) }; if py::eq(&t2216, &t2217) { true } else { false } }) } } } } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2254(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("m"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2292(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &c_NA()?)?, py::getitem(&v_i.clone(), &Py::str("N"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2305(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_i.clone(), &Py::str("E0"))?)?, &py::mul(&py::truediv(&py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_F()?)?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("Q"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("E0"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2334(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("Pi"))?, py::mul(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2347(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("p"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("P_total"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2383(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("N"))?, py::mul(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &c_NA()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2421(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::getitem(&v_o.clone(), &Py::str("N"))?, &c_NA()?)?, py::getitem(&v_i.clone(), &Py::str("n"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_25(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t26 = py::getitem(&v_o.clone(), &Py::str("k"))?; Py::Bool({ let t27 = Py::Int(0i64); if py::gt(&t26, &t27)? { true } else { false } }) }).truthy() { py::abs(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("Ea"))?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2531(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_nr: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__rel(py::mul(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("percent_yield"))?, &Py::Float(100.0))?, &py::truediv(&py::mul(&py::truediv(&py::mul(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("reactant"))?)?)?, &py::getitem(&v_c.clone(), &py::method(&v_s.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("product"))?])?)?)?, &py::getitem(&v_c.clone(), &py::method(&v_s.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("reactant"))?])?)?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("product"))?)?)?, &Py::Float(1000.0))?)?, py::getitem(&v_i.clone(), &Py::str("actual"))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f_balance(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?); v })[..])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2586(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_C: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_h.clone(), &py::truediv(&py::mul(&v_C.clone(), &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Ka1"))?, &v_h.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &py::getitem(&v_i.clone(), &Py::str("Ka1"))?)?, &py::getitem(&v_i.clone(), &Py::str("Ka2"))?)?)?)?, &py::add(&py::add(&py::mul(&v_h.clone(), &v_h.clone())?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Ka1"))?, &v_h.clone())?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("Ka1"))?, &py::getitem(&v_i.clone(), &Py::str("Ka2"))?)?)?)?)?, &py::truediv(&c_KW()?, &v_h.clone())?)?)?, &v_h.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2593(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::getitem(&v_o.clone(), &Py::str("pH"))?, &py::math1("log10", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("h"))?, &Py::Float(1000.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2600(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::sub(&Py::Float(14.0), &py::getitem(&v_o.clone(), &Py::str("pH"))?)?, &py::math1("log10", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("oh"))?, &Py::Float(1000.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2624(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_C: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_Kb: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_x.clone(), &py::truediv(&py::mul(&v_Kb.clone(), &v_C.clone())?, &py::add(&v_Kb.clone(), &v_x.clone())?)?)?, &py::truediv(&c_KW()?, &v_x.clone())?)?)?, &v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::sub(&Py::Float(14.0), &py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?)?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?, py::truediv(&c_KW()?, &py::getitem(&v_i.clone(), &Py::str("Ka"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2631(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_h.clone(), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?)?, &py::truediv(&c_KW()?, &v_h.clone())?)?)?, &v_h.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2638(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_oh: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_oh.clone(), &py::mul(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?, &py::getitem(&v_i.clone(), &Py::str("n_oh"))?)?)?, &py::truediv(&c_KW()?, &v_oh.clone())?)?)?, &v_oh.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::sub(&Py::Float(14.0), &py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2662(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_C: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_h.clone(), &py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Ka"))?, &v_C.clone())?, &py::add(&py::getitem(&v_i.clone(), &Py::str("Ka"))?, &v_h.clone())?)?)?, &py::truediv(&c_KW()?, &v_h.clone())?)?)?, &v_h.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2686(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_C: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_x.clone(), &py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("Kb"))?, &v_C.clone())?, &py::add(&py::getitem(&v_i.clone(), &Py::str("Kb"))?, &v_x.clone())?)?)?, &py::truediv(&c_KW()?, &v_x.clone())?)?)?, &v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::sub(&Py::Float(14.0), &py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?)?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2693(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_i.clone(), &Py::str("lam"))?)?, py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2703(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("lam"))?, &py::getitem(&v_i.clone(), &Py::str("E"))?)?, py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2737(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&Py::Float(0.5), &py::truediv(&py::getitem(&v_o.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("t_half"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("fraction"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("fraction"))?)?, &f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("t_years"))?, &c_YEAR_S()?)?, py::getitem(&v_o.clone(), &Py::str("t"))?)?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("half_lives"))?, &py::truediv(&py::getitem(&v_o.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("t_half"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("half_lives"))?)?], true)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2752(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_n: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_f: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::add(&py::add(&f__rel(py::getitem(&v_o.clone(), &Py::str("m"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("m0"))?, &v_f.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("m_g"))?, py::mul(&py::mul(&Py::Float(1000.0), &py::getitem(&v_i.clone(), &Py::str("m0"))?)?, &v_f.clone())?)?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("fraction"))?, v_f.clone())?)?, &f__rel(py::getitem(&v_o.clone(), &Py::str("half_lives"))?, v_n.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::truediv(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("t_half"))?)?, py::pow(&Py::Float(0.5), &py::truediv(&py::getitem(&v_i.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("t_half"))?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_276(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("A"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("eps"))?, &py::getitem(&v_i.clone(), &Py::str("l"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2789(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("P"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P0"))?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solvent"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solvent"))?)?)?)?, &py::add(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solvent"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solvent"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solute"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solute"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_286(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("c"))?, &py::getitem(&v_i.clone(), &Py::str("eps"))?)?, &py::getitem(&v_i.clone(), &Py::str("l"))?)?, py::getitem(&v_i.clone(), &Py::str("A"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2907(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_sp: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_nr: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dH"))?, &Py::Float(1000.0))?, &py::fsum(&{ let mut out = vec![]; for t2908 in py::iter(&(py::enumerate(&py::zip(&[v_c.clone(), py::getitem(&v_i.clone(), &Py::str("values"))?])?, 0)?))? { let mut v_j: Py; let mut v_v: Py; let mut v_x: Py; 
let t2909 = (t2908).items()?;
if t2909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t2909[0].clone();
let t2910 = (t2909[1].clone()).items()?;
if t2910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2910[0].clone();
v_v = t2910[1].clone();
out.push(py::mul(&(if ({ let t2911 = v_j.clone(); Py::Bool({ let t2912 = v_nr.clone(); if py::ge(&t2911, &t2912)? { true } else { false } }) }).truthy() { v_x.clone() } else { py::neg(&v_x.clone())? }), &v_v.clone())?); } Py::list(out) })?)?)?, &py::minmax(&[Py::Float(1.0), py::sum(&{ let mut out = vec![]; for t2913 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("values"))?))? { let mut v_v: Py; 
v_v = t2913;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f__as_written_balanced(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?); v })[..])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_296(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("dT"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2998(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dS"))?, &f__coef_sum(py::getitem(&v_i.clone(), &Py::str("equation"))?, py::getitem(&v_i.clone(), &Py::str("values"))?)?)?)?, &py::minmax(&[Py::Float(1.0), f__abs_sum(py::getitem(&v_i.clone(), &Py::str("values"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3099(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("order"))?, &py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("c2"))?, &py::getitem(&v_i.clone(), &Py::str("c1"))?)?)?)?, &py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("r2"))?, &py::getitem(&v_i.clone(), &Py::str("r1"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_317(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3198(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("u_rms"))?, &Py::Int(2i64))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::mul(&py::mul(&Py::Int(3i64), &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3236(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::sub(&py::truediv(&Py::Float(1.0), &py::getitem(&v_o.clone(), &Py::str("A"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?, py::truediv(&Py::Float(1.0), &py::getitem(&v_i.clone(), &Py::str("A0"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_327(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V2"))?, &py::getitem(&v_i.clone(), &Py::str("P2"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3272(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("m"))?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3313(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("specific_rotation"))?, &py::getitem(&v_i.clone(), &Py::str("l"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &Py::Float(100.0))?, py::getitem(&v_i.clone(), &Py::str("alpha"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3329(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("E_cell"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("E_cathode"))?, &py::getitem(&v_i.clone(), &Py::str("E_anode"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("E_cathode"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("E_anode"))?)?], true)?)?, &(if ({ let t3330 = py::getitem(&v_o.clone(), &Py::str("spontaneity"))?; Py::Bool({ let t3331 = f__cell_word(py::sub(&py::getitem(&v_i.clone(), &Py::str("E_cathode"))?, &py::getitem(&v_i.clone(), &Py::str("E_anode"))?)?)?; if py::eq(&t3330, &t3331) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_337(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("pH"))?, &py::math1("log10", &py::getitem(&v_i.clone(), &Py::str("Ka"))?)?)?, &py::math1("log10", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("c_base"))?, &py::getitem(&v_i.clone(), &Py::str("c_acid"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3438(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_nr: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("product_mass"))?, py::truediv(&py::mul(&py::truediv(&py::mul(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("reactant"))?)?)?, &py::getitem(&v_c.clone(), &py::method(&v_s.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("product"))?])?)?)?, &py::getitem(&v_c.clone(), &py::method(&v_s.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("reactant"))?])?)?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("product"))?)?)?, &Py::Float(1000.0))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f_balance(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?); v })[..])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_344(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("q"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3476(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V_eq"))?, &py::getitem(&v_i.clone(), &Py::str("c_t"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("ratio"))?, &py::getitem(&v_i.clone(), &Py::str("c_a"))?)?, &py::getitem(&v_i.clone(), &Py::str("V_a"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3490(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_h: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&v_h.clone(), &py::truediv(&c_KW()?, &v_h.clone())?)?, &py::truediv(&py::truediv(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c_a"))?, &py::getitem(&v_i.clone(), &Py::str("V_a"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("c_b"))?, &py::getitem(&v_i.clone(), &Py::str("V_b"))?)?)?, &py::add(&py::getitem(&v_i.clone(), &Py::str("V_a"))?, &py::getitem(&v_i.clone(), &Py::str("V_b"))?)?)?, &Py::Float(1000.0))?)?)?, &py::add(&v_h.clone(), &py::truediv(&c_KW()?, &v_h.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_o.clone(), &Py::str("pH"))?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3517(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("pH"))?, &py::getitem(&{ let t3518 = v_i.clone(); f__weak_titr(py::getitem(&t3518, &Py::str("Ka"))?, py::getitem(&t3518, &Py::str("c_a"))?, py::getitem(&t3518, &Py::str("V_a"))?, py::getitem(&t3518, &Py::str("c_b"))?, py::getitem(&t3518, &Py::str("V_b"))?)? }, &Py::str("pH"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3527(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::add(&py::getitem(&v_o.clone(), &Py::str("P"))?, &py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("n"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("V"))?, &Py::Int(2i64))?)?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_354(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("V1"))?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3540(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("lnK2"))?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("K1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("dH"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3581(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("wavenumber"))?, py::getitem(&v_r.clone(), &Py::str("wavenumber"))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let t3585 = v_i.clone(); f_vib_wavenumber(py::getitem(&t3585, &Py::str("molecule"))?, py::getitem(&t3585, &Py::str("k"))?)? }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3594(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?, py::getitem(&v_i.clone(), &Py::str("A0"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_366(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("P1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dH"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_373(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t374 = py::getitem(&v_o.clone(), &Py::str("P2"))?; Py::Bool({ let t375 = Py::Int(0i64); if py::gt(&t374, &t375)? { true } else { false } }) }).truthy() { py::abs(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("P1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("dH"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_382(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_392(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("P1"))?)?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_402(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("V2"))?, &py::getitem(&v_i.clone(), &Py::str("P2"))?)?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_441(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_ca: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_cb: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t442 = v_s.clone(); Py::Bool({ let t443 = Py::Int(0i64); if py::gt(&t442, &t443)? { true } else { false } }) }).truthy() { py::abs(&py::sub(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::math1("log", &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &v_s.clone())?, &v_ca.clone())?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("y"))?, &py::math1("log", &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("y"))?, &v_s.clone())?, &v_cb.clone())?)?)?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("Ksp"))?)?)?)? } else { Py::Float(1.0) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::truediv(&py::getitem(&v_o.clone(), &Py::str("s"))?, &Py::Float(1000.0))?, (if ({ let t444 = py::getitem(&v_i.clone(), &Py::str("common"))?; Py::Bool({ let t445 = Py::str("cation"); if py::eq(&t444, &t445) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_i.clone(), &Py::str("c_common"))?, &Py::Float(1000.0))? } else { Py::Float(0.0) }), (if ({ let t446 = py::getitem(&v_i.clone(), &Py::str("common"))?; Py::Bool({ let t447 = Py::str("anion"); if py::eq(&t446, &t447) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_i.clone(), &Py::str("c_common"))?, &Py::Float(1000.0))? } else { Py::Float(0.0) })])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_461(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::add(&py::add(&f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("K_conj"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, c_KW()?)?, &py::abs(&py::add(&py::getitem(&v_o.clone(), &Py::str("pK"))?, &py::math1("log10", &py::getitem(&v_i.clone(), &Py::str("K"))?)?)?)?)?, &py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("pK_conj"))?, &Py::Float(14.0))?, &py::math1("log10", &py::getitem(&v_i.clone(), &Py::str("K"))?)?)?)?)?, &(if ({ let t465 = py::getitem(&v_o.clone(), &Py::str("conjugate"))?; Py::Bool({ let t466 = f__conj_word(py::getitem(&v_i.clone(), &Py::str("known"))?)?; if py::eq(&t465, &t466) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_475(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("B"))?, py::getitem(&v_r.clone(), &Py::str("B"))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let t479 = v_i.clone(); f_consecutive(py::getitem(&t479, &Py::str("A0"))?, py::getitem(&t479, &Py::str("k1"))?, py::getitem(&t479, &Py::str("k2"))?, py::getitem(&t479, &Py::str("t"))?)? }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_495(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("dS"))?)?, py::getitem(&v_i.clone(), &Py::str("dH"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_510(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t511 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("partial_pressures"))?, py::getitem(&v_i.clone(), &Py::str("moles"))?])?))? { let mut v_n: Py; let mut v_p: Py; 
let t512 = (t511).items()?;
if t512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t512[0].clone();
v_n = t512[1].clone();
out.push(f__rel(v_p.clone(), py::mul(&py::truediv(&v_n.clone(), &py::fsum(&py::getitem(&v_i.clone(), &Py::str("moles"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("P_total"))?)?)?); } Py::list(out) }, &Py::list(vec![(if ({ let t516 = py::len(&py::getitem(&v_o.clone(), &Py::str("partial_pressures"))?)?; Py::Bool({ let t517 = py::len(&py::getitem(&v_i.clone(), &Py::str("moles"))?)?; if py::eq(&t516, &t517) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_548(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dou"))?, &py::truediv(&py::sub(&py::sub(&py::add(&py::add(&py::add(&py::add(&py::mul(&Py::Int(2i64), &py::add(&py::method(&v_c.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, &py::method(&v_c.clone(), "get", &[Py::str("Si"), Py::Int(0i64)])?)?)?, &Py::Int(2i64))?, &py::method(&v_c.clone(), "get", &[Py::str("N"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("P"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("B"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?)?, &py::sum(&{ let mut out = vec![]; for t549 in py::iter(&(Py::tuple(vec![Py::str("F"), Py::str("Cl"), Py::str("Br"), Py::str("I")])))? { let mut v_x: Py; 
v_x = t549;
out.push(py::method(&v_c.clone(), "get", &[v_x.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)?, &Py::Int(2i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_585(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("dG"))?, py::mul(&py::mul(&py::neg(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_F()?)?, &py::getitem(&v_i.clone(), &Py::str("E"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_595(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("K"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_605(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &py::getitem(&v_i.clone(), &Py::str("dG0"))?)?, &py::mul(&py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("Q"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("dG0"))?)?, py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_614(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("c2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("c1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_626(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V1"))?, &py::getitem(&v_i.clone(), &Py::str("c1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("c2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_640(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_F()?)?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("metal"))?)?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_654(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("I"))?)?, py::mul(&py::mul(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &Py::Float(1000.0))?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("metal"))?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_F()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_66(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("N_atoms"))?, py::mul(&py::mul(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &py::getitem(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &c_NA()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_758(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::neg(&py::getitem(&v_o.clone(), &Py::str("dH"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_872(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t873 = py::getitem(&v_o.clone(), &Py::str("A"))?; Py::Bool({ let t874 = Py::Int(0i64); if py::gt(&t873, &t874)? { true } else { false } }) }).truthy() { py::abs(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_i.clone(), &Py::str("A0"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_881(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("t_half"))?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?, py::math1("log", &Py::Float(2.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_893(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?, py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("A0"))?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("k2"))?, &py::getitem(&v_i.clone(), &Py::str("k1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("Ea"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_903(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("dT"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_939(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("rho"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_977(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_994(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &py::getitem(&v_i.clone(), &Py::str("dH"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("dS"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("dH"))?)?, py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("dS"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
