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
    "balance_equation",
    "beer_lambert_absorbance",
    "beer_lambert_concentration",
    "boiling_point_elevation",
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
    "hydrogen_transition_wavelength",
    "hydronium_from_ph",
    "ideal_gas_mass",
    "ideal_gas_moles",
    "ideal_gas_pressure",
    "ideal_gas_temperature",
    "ideal_gas_volume",
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
    "raoult_vapor_pressure",
    "reaction_enthalpy_from_formation",
    "reaction_order_from_rates",
    "rms_speed",
    "second_order_concentration",
    "solute_mass_for_solution",
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
        "balance_equation" => {
            let r = f_balance_equation(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_balance(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beer_lambert_absorbance" => {
            let r = f_beer_absorbance(match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_255(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "beer_lambert_concentration" => {
            let r = f_beer_concentration(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_265(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boiling_point_elevation" => {
            let r = f__colligative(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_275(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boyle_final_pressure" => {
            let r = f_boyle_p2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_285(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boyle_final_volume" => {
            let r = f_boyle_v2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_295(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "buffer_ph" => {
            let r = f_buffer_ph(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_base") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_acid") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_305(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "calorimetry_heat" => {
            let r = f_calorimetry_heat(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_312(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "charles_final_volume" => {
            let r = f_charles_v2(match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_322(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clausius_clapeyron_enthalpy" => {
            let r = f_cc_enthalpy(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_334(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clausius_clapeyron_pressure" => {
            let r = f_cc_pressure(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_341(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_gas_final_pressure" => {
            let r = f_combined_p2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_350(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_gas_final_temperature" => {
            let r = f_combined_t2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_360(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "combined_gas_final_volume" => {
            let r = f_combined_v2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_370(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "common_ion_solubility" => {
            let r = f_common_ion(match v.get("Ksp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_common") { Some(x) => py::from_val(x), None => { return None } }, match v.get("common") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_409(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "consecutive_first_order" => {
            let r = f_consecutive(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_424(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crossover_temperature" => {
            let r = f_crossover_t(match v.get("dH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_444(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dalton_mixture" => {
            let r = f_dalton_mixture(match v.get("moles") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_total") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_459(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "degree_of_unsaturation" => {
            let r = f_degree_of_unsaturation(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_497(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_g_from_cell_potential" => {
            let r = f_dg_from_e(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_534(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_g_from_k" => {
            let r = f_dg_from_k(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_544(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "delta_g_nonstandard" => {
            let r = f_dg_nonstandard(match v.get("dG0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_554(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dilution_final_concentration" => {
            let r = f_dilution_c2(match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_563(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dilution_volume" => {
            let r = f_dilution_v1(match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_575(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "electrolysis_mass" => {
            let r = f_electrolysis_mass(match v.get("metal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_589(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "electrolysis_time" => {
            let r = f_electrolysis_time(match v.get("metal") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_603(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "empirical_formula" => {
            let r = f_empirical_formula(match v.get("composition") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_empirical(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "enthalpy_from_calorimetry" => {
            let r = f_calorimetry_enthalpy(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_678(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equilibrium_ice" => {
            let r = f_equilibrium_ice(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_ice(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_concentration" => {
            let r = f_first_order_conc(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_792(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_half_life" => {
            let r = f_first_order_half_life(match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_801(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_order_time" => {
            let r = f_first_order_time(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_813(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "freezing_point_depression" => {
            let r = f__colligative(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_823(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gas_density" => {
            let r = f_gas_density(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_859(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gay_lussac_final_pressure" => {
            let r = f_gay_lussac_p2(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_897(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gibbs_free_energy" => {
            let r = f_gibbs(match v.get("dH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_914(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graham_molar_mass" => {
            let r = f_graham_unknown(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_known") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "graham_rate_ratio" => {
            let r = f_graham_ratio(match v.get("formula_1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1005(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heat_from_reaction_mass" => {
            let r = f_heat_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1072(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydrogen_transition_wavelength" => {
            let r = f_hydrogen_line(match v.get("n_upper") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_lower") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1110(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hydronium_from_ph" => {
            let r = f_h_from_ph(match v.get("pH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1117(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_mass" => {
            let r = f_gas_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1153(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_moles" => {
            let r = f__ig_moles(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1191(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_pressure" => {
            let r = f__ig_pressure(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1201(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_temperature" => {
            let r = f__ig_temperature(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ideal_gas_volume" => {
            let r = f__ig_volume(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1221(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "k_from_cell_potential" => {
            let r = f_k_from_e(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1236(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "k_from_delta_g" => {
            let r = f_k_from_dg(match v.get("dG") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1246(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ka_from_ph" => {
            let r = f_ka_from_ph(match v.get("pH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1264(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kc_from_kp" => {
            let r = f_kc_from_kp(match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1291(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kp_from_kc" => {
            let r = f_kp_from_kc(match v.get("Kc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1304(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ksp_from_solubility" => {
            let r = f_ksp_from_s(match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1315(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ksp_molar_solubility" => {
            let r = f_ksp_solubility(match v.get("Ksp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "limiting_reagent" => {
            let r = f_limiting_reagent(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("product") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_limiting(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mass_from_moles" => {
            let r = f_mass_from_moles(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mass_of_element" => {
            let r = f_mass_of_element(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1586(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mass_percent" => {
            let r = f_mass_percent(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("element") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1652(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mean_speed" => {
            let r = f_mean_speed(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1716(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molality" => {
            let r = f_molality(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1780(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass" => {
            let r = f_molar_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_freezing_point" => {
            let r = f_molar_mass_from_freezing(match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dT") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1876(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_gas_density" => {
            let r = f_molar_mass_from_density(match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1886(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_gas_sample" => {
            let r = f_molar_mass_from_sample(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1896(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molar_mass_from_osmotic_pressure" => {
            let r = f_molar_mass_from_osmotic(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pi") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1906(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molarity_from_mass" => {
            let r = f_molarity_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1942(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mole_fraction" => {
            let r = f_mole_fraction(match v.get("formula_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "molecular_formula" => {
            let r = f_molecular_formula(match v.get("empirical") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moles_from_mass" => {
            let r = f_moles_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2152(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moles_from_particles" => {
            let r = f_moles_from_particles(match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2190(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nernst_potential" => {
            let r = f_nernst(match v.get("E0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2203(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "osmotic_pressure" => {
            let r = f_osmotic_pressure(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2210(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_pressure" => {
            let r = f_partial_pressure(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_total") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2223(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "particles_from_mass" => {
            let r = f_particles_from_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2259(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "particles_from_moles" => {
            let r = f_particles_from_moles(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2297(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "percent_yield" => {
            let r = f_percent_yield(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("product") { Some(x) => py::from_val(x), None => { return None } }, match v.get("actual") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_diprotic_acid" => {
            let r = f_ph_diprotic(match v.get("Ka1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ka2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2462(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_from_hydronium" => {
            let r = f_ph_from_h(match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_from_hydroxide" => {
            let r = f_ph_from_oh(match v.get("oh") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2476(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_salt_of_weak_acid" => {
            let r = f_ph_salt_weak_acid(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2500(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_strong_acid" => {
            let r = f_ph_strong_acid(match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2507(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_strong_base" => {
            let r = f_ph_strong_base(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_oh") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2514(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_weak_acid" => {
            let r = f_ph_weak_acid(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2538(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ph_weak_base" => {
            let r = f_ph_weak_base(match v.get("Kb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2562(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photon_energy" => {
            let r = f_photon_energy(match v.get("lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2569(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "photon_wavelength" => {
            let r = f_photon_wavelength(match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2579(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "raoult_vapor_pressure" => {
            let r = f_raoult(match v.get("formula_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solute") { Some(x) => py::from_val(x), None => { return None } }, match v.get("formula_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_solvent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2616(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_enthalpy_from_formation" => {
            let r = f_formation_sum(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("values") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2734(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reaction_order_from_rates" => {
            let r = f_reaction_order(match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rms_speed" => {
            let r = f_rms_speed(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2782(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_concentration" => {
            let r = f_second_order_conc(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2820(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solute_mass_for_solution" => {
            let r = f_solute_mass(match v.get("formula") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2856(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "theoretical_yield" => {
            let r = f_theoretical_yield(match v.get("equation") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactant") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("product") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2994(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "titration_equivalence_volume" => {
            let r = f_equivalence_volume(match v.get("c_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3032(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "titration_ph_strong_acid_strong_base" => {
            let r = f__strong_strong(match v.get("c_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3046(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "titration_ph_weak_acid_strong_base" => {
            let r = f__weak_titr(match v.get("Ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3073(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "van_der_waals_pressure" => {
            let r = f_vdw_pressure(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3083(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vant_hoff_k2" => {
            let r = f_vant_hoff(match v.get("K1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3096(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vibrational_wavenumber" => {
            let r = f_vib_wavenumber(match v.get("molecule") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3137(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "zero_order_concentration" => {
            let r = f_zero_order_conc(match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3150(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
fn c_W() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("H"), Py::Float(1.008)), (Py::str("He"), Py::Float(4.003)), (Py::str("Li"), Py::Float(6.941)), (Py::str("Be"), Py::Float(9.012)), (Py::str("B"), Py::Float(10.812)), (Py::str("C"), Py::Float(12.011)), (Py::str("N"), Py::Float(14.007)), (Py::str("O"), Py::Float(15.999)), (Py::str("F"), Py::Float(18.998)), (Py::str("Ne"), Py::Float(20.18)), (Py::str("Na"), Py::Float(22.99)), (Py::str("Mg"), Py::Float(24.305)), (Py::str("Al"), Py::Float(26.982)), (Py::str("Si"), Py::Float(28.086)), (Py::str("P"), Py::Float(30.974)), (Py::str("S"), Py::Float(32.067)), (Py::str("Cl"), Py::Float(35.453)), (Py::str("Ar"), Py::Float(39.948)), (Py::str("K"), Py::Float(39.098)), (Py::str("Ca"), Py::Float(40.078)), (Py::str("Sc"), Py::Float(44.956)), (Py::str("Ti"), Py::Float(47.867)), (Py::str("V"), Py::Float(50.944)), (Py::str("Cr"), Py::Float(51.996)), (Py::str("Mn"), Py::Float(54.938)), (Py::str("Fe"), Py::Float(55.845)), (Py::str("Co"), Py::Float(58.933)), (Py::str("Ni"), Py::Float(58.693)), (Py::str("Cu"), Py::Float(63.546)), (Py::str("Zn"), Py::Float(65.39)), (Py::str("Ga"), Py::Float(69.723)), (Py::str("Ge"), Py::Float(72.61)), (Py::str("As"), Py::Float(74.922)), (Py::str("Se"), Py::Float(78.96)), (Py::str("Br"), Py::Float(79.904)), (Py::str("Kr"), Py::Float(83.8)), (Py::str("Rb"), Py::Float(85.468)), (Py::str("Sr"), Py::Float(87.62)), (Py::str("Y"), Py::Float(88.906)), (Py::str("Zr"), Py::Float(91.224)), (Py::str("Nb"), Py::Float(92.906)), (Py::str("Mo"), Py::Float(95.94)), (Py::str("Ru"), Py::Float(101.07)), (Py::str("Rh"), Py::Float(102.906)), (Py::str("Pd"), Py::Float(106.42)), (Py::str("Ag"), Py::Float(107.868)), (Py::str("Cd"), Py::Float(112.412)), (Py::str("In"), Py::Float(114.818)), (Py::str("Sn"), Py::Float(118.711)), (Py::str("Sb"), Py::Float(121.76)), (Py::str("Te"), Py::Float(127.6)), (Py::str("I"), Py::Float(126.904)), (Py::str("Xe"), Py::Float(131.29)), (Py::str("Cs"), Py::Float(132.905)), (Py::str("Ba"), Py::Float(137.328)), (Py::str("La"), Py::Float(138.906)), (Py::str("Ce"), Py::Float(140.116)), (Py::str("Pr"), Py::Float(140.908)), (Py::str("Nd"), Py::Float(144.24)), (Py::str("Sm"), Py::Float(150.36)), (Py::str("Eu"), Py::Float(151.964)), (Py::str("Gd"), Py::Float(157.25)), (Py::str("Tb"), Py::Float(158.925)), (Py::str("Dy"), Py::Float(162.5)), (Py::str("Ho"), Py::Float(164.93)), (Py::str("Er"), Py::Float(167.26)), (Py::str("Tm"), Py::Float(168.934)), (Py::str("Yb"), Py::Float(173.04)), (Py::str("Lu"), Py::Float(174.967)), (Py::str("Hf"), Py::Float(178.49)), (Py::str("Ta"), Py::Float(180.948)), (Py::str("W"), Py::Float(183.84)), (Py::str("Re"), Py::Float(186.207)), (Py::str("Os"), Py::Float(190.23)), (Py::str("Ir"), Py::Float(192.217)), (Py::str("Pt"), Py::Float(195.078)), (Py::str("Au"), Py::Float(196.967)), (Py::str("Hg"), Py::Float(200.59)), (Py::str("Tl"), Py::Float(204.383)), (Py::str("Pb"), Py::Float(207.2)), (Py::str("Bi"), Py::Float(208.98)), (Py::str("Th"), Py::Float(232.038)), (Py::str("Pa"), Py::Float(231.036)), (Py::str("U"), Py::Float(238.029))])) })());
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
fn f__as_written_balanced(mut v_text: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_products: Py = Py::None;
    let mut v_reactants: Py = Py::None;
    let mut v_species: Py = Py::None;
    let t2684 = (f_parse_equation(v_text.clone())?).items()?;
    if t2684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reactants = t2684[0].clone();
    v_products = t2684[1].clone();
    v_coefs = { let mut out = vec![]; for t2685 in py::iter(&(py::add(&v_reactants.clone(), &v_products.clone())?))? { let mut v__f: Py; let mut v_c: Py; 
let t2686 = (t2685).items()?;
if t2686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t2686[0].clone();
v__f = t2686[1].clone();
out.push(v_c.clone()); } Py::list(out) };
    v_species = { let mut out = vec![]; for t2687 in py::iter(&(py::add(&v_reactants.clone(), &v_products.clone())?))? { let mut v__c: Py; let mut v_f: Py; 
let t2688 = (t2687).items()?;
if t2688.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t2688[0].clone();
v_f = t2688[1].clone();
out.push(v_f.clone()); } Py::list(out) };
    if (py::not(&f__conserved(v_coefs.clone(), v_species.clone(), py::len(&v_reactants.clone())?)?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation as written is not balanced; balance it first"))));
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t2725 in py::iter(&(v_coefs.clone()))? { let mut v_c: Py; 
v_c = t2725;
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
    if ({ let t3056 = v_flo.clone(); Py::Bool({ let t3057 = Py::Int(0i64); if py::eq(&t3056, &t3057) { true } else { false } }) }).truthy() {
        return Ok(v_lo.clone());
    }
    for t3058 in py::range_iter(&[(v_iters.clone()).i()?]) {
        v__ = t3058;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t3063 = { let t3059 = v_mid.clone(); Py::Bool({ let t3060 = v_lo.clone(); if py::eq(&t3059, &t3060) { true } else { false } }) }; if t3063.truthy() { t3063 } else { { let t3061 = v_mid.clone(); Py::Bool({ let t3062 = v_hi.clone(); if py::eq(&t3061, &t3062) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_mid.clone()])?;
        if ({ let t3064 = v_fm.clone(); Py::Bool({ let t3065 = Py::Int(0i64); if py::eq(&t3064, &t3065) { true } else { false } }) }).truthy() {
            return Ok(v_mid.clone());
        }
        if ({ let t3066 = { let t3068 = v_fm.clone(); Py::Bool({ let t3069 = Py::Int(0i64); if py::lt(&t3068, &t3069)? { true } else { false } }) }; Py::Bool({ let t3067 = { let t3070 = v_flo.clone(); Py::Bool({ let t3071 = Py::Int(0i64); if py::lt(&t3070, &t3071)? { true } else { false } }) }; if py::eq(&t3066, &t3067) { true } else { false } }) }).truthy() {
            let t3072 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t3072.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t3072[0].clone();
            v_flo = t3072[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
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
    if ({ let t607 = py::len(&v_pairs.clone())?; Py::Bool({ let t608 = Py::Int(2i64); if py::lt(&t607, &t608)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least two elements with their percentages, e.g. 'C 40.0, H 6.71, O 53.3'"))));
    }
    v_comp = Py::dict(vec![]);
    for t609 in py::iter(&(v_pairs.clone()))? {
        let t618 = (t609).items()?;
        if t618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_el = t618[0].clone();
        v_v = t618[1].clone();
        v_el = f__element(v_el.clone())?;
        if ({ let t614 = v_el.clone(); Py::Bool({ let t615 = v_comp.clone(); if py::contains(&t615, &t614)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_el.clone(), "")?, " is given twice".to_string()].concat()))));
        }
        py::setitem(&v_comp.clone(), &v_el.clone(), py::float(&v_v.clone())?)?;
        if ({ let t616 = py::getitem(&v_comp.clone(), &v_el.clone())?; Py::Bool({ let t617 = Py::Int(0i64); if py::le(&t616, &t617)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every percentage must be positive"))));
        }
    }
    return Ok(v_comp.clone());
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
    for t2689 in py::iter(&(py::enumerate(&py::zip(&[v_coefs.clone(), v_species.clone()])?, 0)?))? {
        let t2720 = (t2689).items()?;
        if t2720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_j = t2720[0].clone();
        let t2721 = (t2720[1].clone()).items()?;
        if t2721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t2721[0].clone();
        v_f = t2721[1].clone();
        for t2716 in py::iter(&(py::method(&f__counts_rd(v_f.clone())?, "items", &[])?))? {
            let t2719 = (t2716).items()?;
            if t2719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_el = t2719[0].clone();
            v_n = t2719[1].clone();
            py::setitem(&v_tot.clone(), &v_el.clone(), py::add(&py::method(&v_tot.clone(), "get", &[v_el.clone(), Py::Int(0i64)])?, &(if ({ let t2717 = v_j.clone(); Py::Bool({ let t2718 = v_nr.clone(); if py::lt(&t2717, &t2718)? { true } else { false } }) }).truthy() { py::mul(&v_c.clone(), &v_n.clone())? } else { py::mul(&py::neg(&v_c.clone())?, &v_n.clone())? }))?)?;
        }
    }
    return Ok(Py::Bool(({ let mut out = vec![]; for t2722 in py::iter(&(py::method(&v_tot.clone(), "values", &[])?))? { let mut v_v: Py; 
v_v = t2722;
out.push({ let t2723 = v_v.clone(); Py::Bool({ let t2724 = Py::Int(0i64); if py::eq(&t2723, &t2724) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
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
            while ({ let t3000 = { let t2998 = v_j.clone(); Py::Bool({ let t2999 = py::len(&v_t.clone())?; if py::lt(&t2998, &t2999)? { true } else { false } }) }; if !t3000.truthy() { t3000 } else { py::method(&py::getitem(&v_t.clone(), &v_j.clone())?, "isdigit", &[])? } }).truthy() {
                v_j = py::iadd(&v_j, &Py::Int(1i64))?;
            }
            return Ok(Py::tuple(vec![(if ({ let t3001 = v_j.clone(); Py::Bool({ let t3002 = v_i.clone(); if py::gt(&t3001, &t3002)? { true } else { false } }) }).truthy() { py::int(&py::slice(&v_t.clone(), Some((v_i.clone()).i()?), Some((v_j.clone()).i()?), None)?)? } else { Py::Int(1i64) }), v_j.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let rec3003 = py::rec_cell();
    v_seq = { let v_number = v_number.clone(); let v_out = v_out.clone(); let v_parts = v_parts.clone(); let v_s = v_s.clone(); let v_text = v_text.clone(); let rec3003 = rec3003.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_close: Py = a.get(2).cloned().unwrap_or(Py::None);
            let v_seq: Py = py::rec_get(&rec3003)?;
            let mut v_acc: Py = Py::None;
            let mut v_inner: Py = Py::None;
            let mut v_k: Py = Py::None;
            let mut v_n: Py = Py::None;
            let mut v_sym: Py = Py::None;
            let mut v_v: Py = Py::None;
            v_acc = Py::dict(vec![]);
            while ({ let t3018 = { let t3014 = v_i.clone(); Py::Bool({ let t3015 = py::len(&v_t.clone())?; if py::lt(&t3014, &t3015)? { true } else { false } }) }; if !t3018.truthy() { t3018 } else { { let t3016 = py::getitem(&v_t.clone(), &v_i.clone())?; Py::Bool({ let t3017 = Py::str(")]}"); if !py::contains(&t3017, &t3016)? { true } else { false } }) } } }).truthy() {
                if ({ let t3004 = py::getitem(&v_t.clone(), &v_i.clone())?; Py::Bool({ let t3005 = Py::str("([{"); if py::contains(&t3005, &t3004)? { true } else { false } }) }).truthy() {
                    let t3006 = (py::call(&v_seq.clone(), &[v_t.clone(), py::add(&v_i.clone(), &Py::Int(1i64))?, py::getitem(&c__PAIRS()?, &py::getitem(&v_t.clone(), &v_i.clone())?)?])?).items()?;
                    if t3006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_inner = t3006[0].clone();
                    v_i = t3006[1].clone();
                    v_i = py::iadd(&v_i, &Py::Int(1i64))?;
                    let t3007 = (py::call(&v_number.clone(), &[v_t.clone(), v_i.clone()])?).items()?;
                    if t3007.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_n = t3007[0].clone();
                    v_i = t3007[1].clone();
                    for t3008 in py::iter(&(py::method(&v_inner.clone(), "items", &[])?))? {
                        let t3009 = (t3008).items()?;
                        if t3009.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                        v_k = t3009[0].clone();
                        v_v = t3009[1].clone();
                        py::setitem(&v_acc.clone(), &v_k.clone(), py::add(&py::method(&v_acc.clone(), "get", &[v_k.clone(), Py::Int(0i64)])?, &py::mul(&v_v.clone(), &v_n.clone())?)?)?;
                    }
                } else {
                    v_sym = py::add(&py::getitem(&v_t.clone(), &v_i.clone())?, &(if ({ let t3012 = { let t3010 = py::add(&v_i.clone(), &Py::Int(1i64))?; Py::Bool({ let t3011 = py::len(&v_t.clone())?; if py::lt(&t3010, &t3011)? { true } else { false } }) }; if !t3012.truthy() { t3012 } else { py::method(&py::getitem(&v_t.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, "islower", &[])? } }).truthy() { py::getitem(&v_t.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)? } else { Py::str("") }))?;
                    v_i = py::iadd(&v_i, &py::len(&v_sym.clone())?)?;
                    let t3013 = (py::call(&v_number.clone(), &[v_t.clone(), v_i.clone()])?).items()?;
                    if t3013.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_n = t3013[0].clone();
                    v_i = t3013[1].clone();
                    py::setitem(&v_acc.clone(), &v_sym.clone(), py::add(&py::method(&v_acc.clone(), "get", &[v_sym.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
                }
            }
            return Ok(Py::tuple(vec![v_acc.clone(), v_i.clone()]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    py::rec_set(&rec3003, &v_seq);
    for t3019 in py::iter(&(v_parts.clone()))? {
        v_p = t3019;
        let t3020 = ((if (py::method(&py::slice(&v_p.clone(), None, Some((Py::Int(1i64)).i()?), None)?, "isdigit", &[])?).truthy() { py::call(&v_number.clone(), &[v_p.clone(), Py::Int(0i64)])? } else { Py::tuple(vec![Py::Int(1i64), Py::Int(0i64)]) })).items()?;
        if t3020.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mult = t3020[0].clone();
        v_i = t3020[1].clone();
        let t3021 = (py::call(&v_seq.clone(), &[v_p.clone(), v_i.clone(), Py::None])?).items()?;
        if t3021.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_inner = t3021[0].clone();
        v__ = t3021[1].clone();
        for t3022 in py::iter(&(py::method(&v_inner.clone(), "items", &[])?))? {
            let t3023 = (t3022).items()?;
            if t3023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_k = t3023[0].clone();
            v_v = t3023[1].clone();
            py::setitem(&v_out.clone(), &v_k.clone(), py::add(&py::method(&v_out.clone(), "get", &[v_k.clone(), Py::Int(0i64)])?, &py::mul(&v_v.clone(), &v_mult.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
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
    if ({ let t1618 = v_t.clone(); Py::Bool({ let t1619 = c_W()?; if py::contains(&t1619, &t1618)? { true } else { false } }) }).truthy() {
        return Ok(v_t.clone());
    }
    if ({ let t1620 = py::method(&v_t.clone(), "lower", &[])?; Py::Bool({ let t1621 = c_ELEMENT_NAMES()?; if py::contains(&t1621, &t1620)? { true } else { false } }) }).truthy() {
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
    v_species = py::add(&{ let mut out = vec![]; for t2931 in py::iter(&(v_reactants.clone()))? { let mut v__c: Py; let mut v_f: Py; 
let t2932 = (t2931).items()?;
if t2932.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t2932[0].clone();
v_f = t2932[1].clone();
out.push(v_f.clone()); } Py::list(out) }, &{ let mut out = vec![]; for t2933 in py::iter(&(v_products.clone()))? { let mut v__c: Py; let mut v_f: Py; 
let t2934 = (t2933).items()?;
if t2934.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t2934[0].clone();
v_f = t2934[1].clone();
out.push(v_f.clone()); } Py::list(out) })?;
    v_counts = { let mut out = vec![]; for t2935 in py::iter(&(v_species.clone()))? { let mut v_f: Py; 
v_f = t2935;
out.push(f_formula_counts(v_f.clone())?); } Py::list(out) };
    v_elements = py::sorted(&py::set(&{ let mut out = vec![]; for t2936 in py::iter(&(v_counts.clone()))? { let mut v_c: Py; 
v_c = t2936;
for t2937 in py::iter(&(v_c.clone()))? { let mut v_e: Py; 
v_e = t2937;
out.push(v_e.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?;
    v_nr = py::len(&v_reactants.clone())?;
    v_rows = { let mut out = vec![]; for t2938 in py::iter(&(v_elements.clone()))? { let mut v_e: Py; 
v_e = t2938;
out.push({ let mut out = vec![]; for t2939 in py::iter(&(py::enumerate(&v_counts.clone(), 0)?))? { let mut v_c: Py; let mut v_j: Py; 
let t2940 = (t2939).items()?;
if t2940.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t2940[0].clone();
v_c = t2940[1].clone();
out.push(py::fraction(&[py::mul(&py::method(&v_c.clone(), "get", &[v_e.clone(), Py::Int(0i64)])?, &(if ({ let t2941 = v_j.clone(); Py::Bool({ let t2942 = v_nr.clone(); if py::lt(&t2941, &t2942)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { py::neg(&Py::Int(1i64))? }))?])?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::tuple(vec![v_rows.clone(), v_species.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_kw: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t3093 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t3095 = (t3093).items()?;
        if t3095.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3095[0].clone();
        v_v = t3095[1].clone();
        if (py::not(&{ let t3094 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3094.truthy() { t3094 } else { py::math1("isfinite", &v_v.clone())? } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a finite number".to_string()].concat()))));
        }
    }
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
    let t3106 = (Py::tuple(vec![Py::list(vec![Py::dict(vec![])]), Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t3106.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_stack = t3106[0].clone();
    v_opens = t3106[1].clone();
    v_pos = t3106[2].clone();
    while ({ let t3120 = v_pos.clone(); Py::Bool({ let t3121 = py::len(&v_s.clone())?; if py::lt(&t3120, &t3121)? { true } else { false } }) }).truthy() {
        v_m = py::method(&c__FTOK()?, "match", &[v_s.clone(), v_pos.clone()])?;
        if (py::not(&v_m.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&["cannot read the formula at '".to_string(), py::format(&py::slice(&v_s.clone(), Some((v_pos.clone()).i()?), None, None)?, "")?, "'".to_string()].concat()))));
        }
        if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() {
            v_el = py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?;
            if ({ let t3107 = v_el.clone(); Py::Bool({ let t3108 = c_W()?; if !py::contains(&t3108, &t3107)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["'".to_string(), py::format(&v_el.clone(), "")?, "' is not an element symbol with a standard atomic weight".to_string()].concat()))));
            }
            v_n = (if (py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)? } else { Py::Int(1i64) });
            if ({ let t3109 = v_n.clone(); Py::Bool({ let t3110 = Py::Int(0i64); if py::eq(&t3109, &t3110) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a subscript of 0 is not allowed"))));
            }
            py::setitem(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, &v_el.clone(), py::add(&py::method(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, "get", &[v_el.clone(), Py::Int(0i64)])?, &v_n.clone())?)?;
        } else {
            if (py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?).truthy() {
                let _ = py::method(&v_stack.clone(), "append", &[Py::dict(vec![])])?;
                let _ = py::method(&v_opens.clone(), "append", &[py::method(&v_m.clone(), "group", &[Py::Int(3i64)])?])?;
            } else {
                if ({ let t3115 = { let t3111 = py::len(&v_stack.clone())?; Py::Bool({ let t3112 = Py::Int(1i64); if py::eq(&t3111, &t3112) { true } else { false } }) }; if t3115.truthy() { t3115 } else { { let t3113 = py::getitem(&c__PAIRS()?, &py::getitem(&v_opens.clone(), &py::neg(&Py::Int(1i64))?)?)?; Py::Bool({ let t3114 = py::method(&v_m.clone(), "group", &[Py::Int(4i64)])?; if py::ne(&t3113, &t3114) { true } else { false } }) } } }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("unbalanced brackets in the formula"))));
                }
                let _ = py::method(&v_opens.clone(), "pop", &[])?;
                v_n = (if (py::method(&v_m.clone(), "group", &[Py::Int(5i64)])?).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(5i64)])?)? } else { Py::Int(1i64) });
                if ({ let t3116 = v_n.clone(); Py::Bool({ let t3117 = Py::Int(0i64); if py::eq(&t3116, &t3117) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("a subscript of 0 is not allowed"))));
                }
                v_top = py::method(&v_stack.clone(), "pop", &[])?;
                if (py::not(&v_top.clone())).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("empty brackets in the formula"))));
                }
                for t3118 in py::iter(&(py::method(&v_top.clone(), "items", &[])?))? {
                    let t3119 = (t3118).items()?;
                    if t3119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_el = t3119[0].clone();
                    v_c = t3119[1].clone();
                    py::setitem(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, &v_el.clone(), py::add(&py::method(&py::getitem(&v_stack.clone(), &py::neg(&Py::Int(1i64))?)?, "get", &[v_el.clone(), Py::Int(0i64)])?, &py::mul(&v_c.clone(), &v_n.clone())?)?)?;
                }
            }
        }
        v_pos = py::method(&v_m.clone(), "end", &[])?;
    }
    if ({ let t3124 = { let t3122 = py::len(&v_stack.clone())?; Py::Bool({ let t3123 = Py::Int(1i64); if py::ne(&t3122, &t3123) { true } else { false } }) }; if t3124.truthy() { t3124 } else { py::not(&py::getitem(&v_stack.clone(), &Py::Int(0i64))?) } }).truthy() {
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
    v_lo = py::minmax(&[{ let mut out = vec![]; for t737 in py::iter(&(py::zip(&[v_c0.clone(), v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t738 = (t737).items()?;
if t738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t738[0].clone();
v_n = t738[1].clone();
if ({ let t739 = v_n.clone(); Py::Bool({ let t740 = Py::Int(0i64); if py::gt(&t739, &t740)? { true } else { false } }) }).truthy() { out.push(py::truediv(&py::neg(&v_c.clone())?, &v_n.clone())?); }} Py::list(out) }], true)?;
    v_hi = py::minmax(&[{ let mut out = vec![]; for t741 in py::iter(&(py::zip(&[v_c0.clone(), v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t742 = (t741).items()?;
if t742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t742[0].clone();
v_n = t742[1].clone();
if ({ let t743 = v_n.clone(); Py::Bool({ let t744 = Py::Int(0i64); if py::lt(&t743, &t744)? { true } else { false } }) }).truthy() { out.push(py::truediv(&v_c.clone(), &py::neg(&v_n.clone())?)?); }} Py::list(out) }], false)?;
    if (py::not(&{ let t745 = v_lo.clone(); Py::Bool({ let t746 = v_hi.clone(); if py::lt(&t745, &t746)? { true } else { false } }) })).truthy() {
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
    let t718 = (f_parse_equation(v_equation.clone())?).items()?;
    if t718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t718[0].clone();
    v_p = t718[1].clone();
    return Ok(Py::tuple(vec![py::add(&{ let mut out = vec![]; for t719 in py::iter(&(v_r.clone()))? { let mut v__f: Py; let mut v_c: Py; 
let t720 = (t719).items()?;
if t720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t720[0].clone();
v__f = t720[1].clone();
out.push(py::neg(&py::float(&v_c.clone())?)?); } Py::list(out) }, &{ let mut out = vec![]; for t721 in py::iter(&(v_p.clone()))? { let mut v__f: Py; let mut v_c: Py; 
let t722 = (t721).items()?;
if t722.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t722[0].clone();
v__f = t722[1].clone();
out.push(py::float(&v_c.clone())?); } Py::list(out) })?, { let mut out = vec![]; for t723 in py::iter(&(py::add(&v_r.clone(), &v_p.clone())?))? { let mut v__c: Py; let mut v_f: Py; 
let t724 = (t723).items()?;
if t724.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t724[0].clone();
v_f = t724[1].clone();
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
    for t765 in py::iter(&(py::zip(&[v_c0.clone(), v_nu.clone()])?))? {
        let t770 = (t765).items()?;
        if t770.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t770[0].clone();
        v_n = t770[1].clone();
        v_v = py::add(&v_c.clone(), &py::mul(&v_n.clone(), &v_x.clone())?)?;
        if ({ let t766 = v_v.clone(); Py::Bool({ let t767 = Py::Int(0i64); if py::le(&t766, &t767)? { true } else { false } }) }).truthy() {
            return Ok((if ({ let t768 = v_n.clone(); Py::Bool({ let t769 = Py::Int(0i64); if py::gt(&t768, &t769)? { true } else { false } }) }).truthy() { py::neg(&Py::Float(f64::INFINITY))? } else { Py::Float(f64::INFINITY) }));
        }
        v_s = py::iadd(&v_s, &py::mul(&v_n.clone(), &py::math1("log", &v_v.clone())?)?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mm_rd(mut v_text: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t3024 in py::iter(&(py::method(&f__counts_rd(v_text.clone())?, "items", &[])?))? { let mut v_el: Py; let mut v_n: Py; 
let t3025 = (t3024).items()?;
if t3025.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_el = t3025[0].clone();
v_n = t3025[1].clone();
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
    v_m = { let mut out = vec![]; for t2944 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t2944;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
    let t2945 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t2945.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pivots = t2945[0].clone();
    v_r = t2945[1].clone();
    for t2946 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t2946;
        v_p = (match ('g2947: { let mut out = vec![]; for t2948 in py::range_iter(&[(v_r.clone()).i()?, (py::len(&v_m.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2948;
if ({ let t2949 = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_c.clone())?; Py::Bool({ let t2950 = Py::Int(0i64); if py::ne(&t2949, &t2950) { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g2947 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => Py::None });
        if ({ let t2951 = v_p.clone(); Py::Bool({ let t2952 = Py::None; if py::is(&t2951, &t2952) { true } else { false } }) }).truthy() {
            continue;
        }
        let t2953 = (Py::tuple(vec![py::getitem(&v_m.clone(), &v_p.clone())?, py::getitem(&v_m.clone(), &v_r.clone())?])).items()?;
        if t2953.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_m.clone(), &v_r.clone(), t2953[0].clone())?;
        py::setitem(&v_m.clone(), &v_p.clone(), t2953[1].clone())?;
        v_piv = py::getitem(&py::getitem(&v_m.clone(), &v_r.clone())?, &v_c.clone())?;
        py::setitem(&v_m.clone(), &v_r.clone(), { let mut out = vec![]; for t2954 in py::iter(&(py::getitem(&v_m.clone(), &v_r.clone())?))? { let mut v_x: Py; 
v_x = t2954;
out.push(py::truediv(&v_x.clone(), &v_piv.clone())?); } Py::list(out) })?;
        for t2955 in py::range_iter(&[(py::len(&v_m.clone())?).i()?]) {
            v_i = t2955;
            if ({ let t2960 = { let t2956 = v_i.clone(); Py::Bool({ let t2957 = v_r.clone(); if py::ne(&t2956, &t2957) { true } else { false } }) }; if !t2960.truthy() { t2960 } else { { let t2958 = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_c.clone())?; Py::Bool({ let t2959 = Py::Int(0i64); if py::ne(&t2958, &t2959) { true } else { false } }) } } }).truthy() {
                v_f = py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_c.clone())?;
                py::setitem(&v_m.clone(), &v_i.clone(), { let mut out = vec![]; for t2961 in py::iter(&(py::zip(&[py::getitem(&v_m.clone(), &v_i.clone())?, py::getitem(&v_m.clone(), &v_r.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2962 = (t2961).items()?;
if t2962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2962[0].clone();
v_b = t2962[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
        let _ = py::method(&v_pivots.clone(), "append", &[v_c.clone()])?;
        v_r = py::iadd(&v_r, &Py::Int(1i64))?;
    }
    v_free = { let mut out = vec![]; for t2963 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t2963;
if ({ let t2964 = v_c.clone(); Py::Bool({ let t2965 = v_pivots.clone(); if !py::contains(&t2965, &t2964)? { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) };
    if ({ let t2966 = py::len(&v_free.clone())?; Py::Bool({ let t2967 = Py::Int(1i64); if py::ne(&t2966, &t2967) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation cannot be balanced uniquely (it has no solution or several independent ones)"))));
    }
    v_fc = py::getitem(&v_free.clone(), &Py::Int(0i64))?;
    v_v = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    py::setitem(&v_v.clone(), &v_fc.clone(), py::fraction(&[Py::Int(1i64)])?)?;
    for t2968 in py::iter(&(py::enumerate(&v_pivots.clone(), 0)?))? {
        let t2969 = (t2968).items()?;
        if t2969.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2969[0].clone();
        v_pc = t2969[1].clone();
        py::setitem(&v_v.clone(), &v_pc.clone(), py::neg(&py::getitem(&py::getitem(&v_m.clone(), &v_i.clone())?, &v_fc.clone())?)?)?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t2970 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2970;
out.push({ let t2971 = v_x.clone(); Py::Bool({ let t2972 = Py::Int(0i64); if py::eq(&t2971, &t2972) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation cannot be balanced with every species taking part"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2973 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2973;
out.push({ let t2974 = v_x.clone(); Py::Bool({ let t2975 = Py::Int(0i64); if py::lt(&t2974, &t2975)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        v_v = { let mut out = vec![]; for t2976 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2976;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
    }
    if (Py::Bool(({ let mut out = vec![]; for t2977 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2977;
out.push({ let t2978 = v_x.clone(); Py::Bool({ let t2979 = Py::Int(0i64); if py::lt(&t2978, &t2979)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the equation cannot be balanced with positive coefficients"))));
    }
    v_lcm = Py::Int(1i64);
    for t2980 in py::iter(&(v_v.clone()))? {
        v_x = t2980;
        v_lcm = py::floordiv(&py::mul(&v_lcm.clone(), &py::denominator(&v_x.clone())?)?, &py::gcd(&v_lcm.clone(), &py::denominator(&v_x.clone())?)?)?;
    }
    v_ints = { let mut out = vec![]; for t2981 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t2981;
out.push(py::int(&py::mul(&v_x.clone(), &v_lcm.clone())?)?); } Py::list(out) };
    v_g = Py::Int(0i64);
    for t2982 in py::iter(&(v_ints.clone()))? {
        v_x = t2982;
        v_g = py::gcd(&v_g.clone(), &v_x.clone())?;
    }
    return Ok({ let mut out = vec![]; for t2983 in py::iter(&(v_ints.clone()))? { let mut v_x: Py; 
v_x = t2983;
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
    for t3142 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? {
        let t3147 = (t3142).items()?;
        if t3147.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3147[0].clone();
        v_v = t3147[1].clone();
        if ({ let t3146 = py::not(&{ let t3143 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3143.truthy() { t3143 } else { py::math1("isfinite", &v_v.clone())? } }); if t3146.truthy() { t3146 } else { { let t3144 = v_v.clone(); Py::Bool({ let t3145 = Py::Int(0i64); if py::le(&t3144, &t3145)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_k.clone(), "")?, " must be a positive number".to_string()].concat()))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_s: Py = Py::None;
    let t3151 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
    if t3151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t3151[0].clone();
    v_b = t3151[1].clone();
    v_s = py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?;
    return Ok((if ({ let t3152 = v_s.clone(); Py::Bool({ let t3153 = Py::Int(0i64); if py::eq(&t3152, &t3153) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?, &v_s.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__role(mut v_species: Py, mut v_nr: Py, mut v_formula: Py) -> R {
    let mut v_j: Py = Py::None;
    if ({ let t2985 = v_formula.clone(); Py::Bool({ let t2986 = v_species.clone(); if !py::contains(&t2986, &t2985)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_formula.clone(), "")?, " does not appear in the equation".to_string()].concat()))));
    }
    v_j = py::method(&v_species.clone(), "index", &[v_formula.clone()])?;
    return Ok(Py::tuple(vec![v_j.clone(), { let t2987 = v_j.clone(); Py::Bool({ let t2988 = v_nr.clone(); if py::lt(&t2987, &t2988)? { true } else { false } }) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__strong_strong(mut v_c_a: Py, mut v_V_a: Py, mut v_c_b: Py, mut v_V_b: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("c_a"), v_c_a.clone()), (Py::str("V_a"), v_V_a.clone()), (Py::str("c_b"), v_c_b.clone())]))?;
    if ({ let t3042 = v_V_b.clone(); Py::Bool({ let t3043 = Py::Int(0i64); if py::lt(&t3042, &t3043)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the added volume cannot be negative"))));
    }
    v_D = py::truediv(&py::truediv(&py::sub(&py::mul(&v_c_a.clone(), &v_V_a.clone())?, &py::mul(&v_c_b.clone(), &v_V_b.clone())?)?, &py::add(&v_V_a.clone(), &v_V_b.clone())?)?, &Py::Float(1000.0))?;
    v_h = (if ({ let t3044 = v_D.clone(); Py::Bool({ let t3045 = Py::Int(0i64); if py::ge(&t3044, &t3045)? { true } else { false } }) }).truthy() { py::add(&py::truediv(&v_D.clone(), &Py::Int(2i64))?, &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_D.clone(), &v_D.clone())?, &Py::Int(4i64))?, &c_KW()?)?)?)? } else { py::truediv(&py::mul(&Py::Int(2i64), &c_KW()?)?, &py::add(&py::neg(&v_D.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_D.clone(), &v_D.clone())?, &py::mul(&Py::Int(4i64), &c_KW()?)?)?)?)?)? });
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
    let t196 = (f_parse_equation(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?;
    if t196.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reactants = t196[0].clone();
    v_products = t196[1].clone();
    v_species = { let mut out = vec![]; for t197 in py::iter(&(py::add(&v_reactants.clone(), &v_products.clone())?))? { let mut v__c: Py; let mut v_f: Py; 
let t198 = (t197).items()?;
if t198.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t198[0].clone();
v_f = t198[1].clone();
out.push(v_f.clone()); } Py::list(out) };
    v_c = py::getitem(&v_o.clone(), &Py::str("coefficients"))?;
    if ({ let t207 = { let t199 = py::len(&v_c.clone())?; Py::Bool({ let t200 = py::len(&v_species.clone())?; if py::ne(&t199, &t200) { true } else { false } }) }; if t207.truthy() { t207 } else { Py::Bool(({ let mut out = vec![]; for t201 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t201;
out.push({ let t206 = { let t202 = v_x.clone(); Py::Bool({ let t203 = Py::Int(0i64); if py::le(&t202, &t203)? { true } else { false } }) }; if t206.truthy() { t206 } else { { let t204 = v_x.clone(); Py::Bool({ let t205 = py::int(&v_x.clone())?; if py::ne(&t204, &t205) { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_c = { let mut out = vec![]; for t208 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t208;
out.push(py::int(&v_x.clone())?); } Py::list(out) };
    v_g = Py::Int(0i64);
    for t209 in py::iter(&(v_c.clone()))? {
        v_x = t209;
        v_g = py::gcd(&v_g.clone(), &v_x.clone())?;
    }
    return Ok((if ({ let t248 = { let t210 = v_g.clone(); Py::Bool({ let t211 = Py::Int(1i64); if py::eq(&t210, &t211) { true } else { false } }) }; if !t248.truthy() { t248 } else { f__conserved(v_c.clone(), v_species.clone(), py::len(&v_reactants.clone())?)? } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
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
    if ({ let t663 = py::set(&v_counts.clone())?; Py::Bool({ let t664 = py::set(&v_comp.clone())?; if py::ne(&t663, &t664) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_g = Py::Int(0i64);
    for t665 in py::iter(&(py::method(&v_counts.clone(), "values", &[])?))? {
        v_v = t665;
        v_g = py::gcd(&v_g.clone(), &v_v.clone())?;
    }
    if ({ let t666 = v_g.clone(); Py::Bool({ let t667 = Py::Int(1i64); if py::ne(&t666, &t667) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_tot_given = py::sum(&py::method(&v_comp.clone(), "values", &[])?, None)?;
    v_tot_mass = py::sum(&{ let mut out = vec![]; for t668 in py::iter(&(v_counts.clone()))? { let mut v_e: Py; 
v_e = t668;
out.push(py::mul(&py::getitem(&v_counts.clone(), &v_e.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?); } Py::list(out) }, None)?;
    v_dev = py::minmax(&[{ let mut out = vec![]; for t669 in py::iter(&(v_comp.clone()))? { let mut v_e: Py; 
v_e = t669;
out.push(py::truediv(&py::abs(&py::sub(&py::truediv(&py::mul(&py::getitem(&v_counts.clone(), &v_e.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?, &v_tot_mass.clone())?, &py::truediv(&py::getitem(&v_comp.clone(), &v_e.clone())?, &v_tot_given.clone())?)?)?, &py::truediv(&py::getitem(&v_comp.clone(), &v_e.clone())?, &v_tot_given.clone())?)?); } Py::list(out) }], true)?;
    return Ok((if ({ let t670 = v_dev.clone(); Py::Bool({ let t671 = Py::Float(0.03); if py::le(&t670, &t671)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
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
    let t773 = (f__ice_terms(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?;
    if t773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nu = t773[0].clone();
    v_sp = t773[1].clone();
    v_x = py::getitem(&v_o.clone(), &Py::str("x"))?;
    v_eq = { let mut out = vec![]; for t774 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("initial"))?, v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t775 = (t774).items()?;
if t775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t775[0].clone();
v_n = t775[1].clone();
out.push(py::add(&v_c.clone(), &py::mul(&v_n.clone(), &v_x.clone())?)?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t776 in py::iter(&(v_eq.clone()))? { let mut v_v: Py; 
v_v = t776;
out.push({ let t777 = v_v.clone(); Py::Bool({ let t778 = Py::Int(0i64); if py::le(&t777, &t778)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_given = py::method(&v_o.clone(), "get", &[Py::str("equilibrium"), v_eq.clone()])?;
    return Ok(py::minmax(&[py::abs(&py::sub(&f__lnq(v_nu.clone(), py::getitem(&v_i.clone(), &Py::str("initial"))?, v_x.clone())?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("K"))?)?)?)?, (if ({ let t779 = py::len(&v_given.clone())?; Py::Bool({ let t780 = py::len(&v_eq.clone())?; if py::eq(&t779, &t780) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t781 in py::iter(&(py::zip(&[v_given.clone(), v_eq.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t782 = (t781).items()?;
if t782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t782[0].clone();
v_b = t782[1].clone();
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
    let t1450 = (f_balance(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?;
    if t1450.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t1450[0].clone();
    v_species = t1450[1].clone();
    v_nr = t1450[2].clone();
    let t1451 = (Py::tuple(vec![py::method(&v_species.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("reactant_a"))?])?, py::method(&v_species.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("reactant_b"))?])?, py::method(&v_species.clone(), "index", &[py::getitem(&v_i.clone(), &Py::str("product"))?])?])).items()?;
    if t1451.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ja = t1451[0].clone();
    v_jb = t1451[1].clone();
    v_jp = t1451[2].clone();
    let t1480 = (Py::tuple(vec![py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m_a"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("reactant_a"))?)?)?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m_b"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("reactant_b"))?)?)?])).items()?;
    if t1480.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_na = t1480[0].clone();
    v_nb = t1480[1].clone();
    v_ext = py::minmax(&[py::truediv(&v_na.clone(), &py::getitem(&v_coefs.clone(), &v_ja.clone())?)?, py::truediv(&v_nb.clone(), &py::getitem(&v_coefs.clone(), &v_jb.clone())?)?], false)?;
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("product_mass"))?, py::truediv(&py::mul(&py::mul(&v_ext.clone(), &py::getitem(&v_coefs.clone(), &v_jp.clone())?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("product"))?)?)?, &Py::Float(1000.0))?)?);
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
    if ({ let t3053 = v_V_b.clone(); Py::Bool({ let t3054 = Py::Int(0i64); if py::lt(&t3053, &t3054)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the added volume cannot be negative"))));
    }
    v_Vt = py::add(&v_V_a.clone(), &v_V_b.clone())?;
    let t3055 = (Py::tuple(vec![py::truediv(&py::truediv(&py::mul(&v_c_a.clone(), &v_V_a.clone())?, &v_Vt.clone())?, &Py::Float(1000.0))?, py::truediv(&py::truediv(&py::mul(&v_c_b.clone(), &v_V_b.clone())?, &v_Vt.clone())?, &Py::Float(1000.0))?])).items()?;
    if t3055.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3055[0].clone();
    v_Na = t3055[1].clone();
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
    return Ok(py::method(&Py::str(""), "join", &[{ let mut out = vec![]; for t2076 in py::iter(&(v_order.clone()))? { let mut v_el: Py; 
v_el = t2076;
if (py::method(&v_counts.clone(), "get", &[v_el.clone()])?).truthy() { out.push(py::add(&v_el.clone(), &(if ({ let t2077 = py::getitem(&v_counts.clone(), &v_el.clone())?; Py::Bool({ let t2078 = Py::Int(1i64); if py::ne(&t2077, &t2078) { true } else { false } }) }).truthy() { Py::str(&py::to_str(&py::getitem(&v_counts.clone(), &v_el.clone())?)) } else { Py::str("") }))?); }} Py::list(out) }])?);
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
            return Ok(py::add(&(if ({ let t190 = v_c.clone(); Py::Bool({ let t191 = Py::Int(1i64); if py::ne(&t190, &t191) { true } else { false } }) }).truthy() { Py::str(&[py::format(&v_c.clone(), "")?, " ".to_string()].concat()) } else { Py::str("") }), &v_f.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(py::add(&py::add(&py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t192 in py::iter(&(py::zip(&[py::slice(&v_coefs.clone(), None, Some((v_nr.clone()).i()?), None)?, py::slice(&v_species.clone(), None, Some((v_nr.clone()).i()?), None)?])?))? { let mut v_c: Py; let mut v_f: Py; 
let t193 = (t192).items()?;
if t193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t193[0].clone();
v_f = t193[1].clone();
out.push(py::call(&v_term.clone(), &[v_c.clone(), v_f.clone()])?); } Py::list(out) }])?, &Py::str(" -> "))?, &py::method(&Py::str(" + "), "join", &[{ let mut out = vec![]; for t194 in py::iter(&(py::zip(&[py::slice(&v_coefs.clone(), Some((v_nr.clone()).i()?), None, None)?, py::slice(&v_species.clone(), Some((v_nr.clone()).i()?), None, None)?])?))? { let mut v_c: Py; let mut v_f: Py; 
let t195 = (t194).items()?;
if t195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t195[0].clone();
v_f = t195[1].clone();
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
fn f_balance(mut v_text: Py) -> R {
    let mut v_coefs: Py = Py::None;
    let mut v_products: Py = Py::None;
    let mut v_reactants: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_species: Py = Py::None;
    let t2930 = (f_parse_equation(v_text.clone())?).items()?;
    if t2930.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_reactants = t2930[0].clone();
    v_products = t2930[1].clone();
    let t2943 = (f__element_matrix(v_reactants.clone(), v_products.clone())?).items()?;
    if t2943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t2943[0].clone();
    v_species = t2943[1].clone();
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
    let t188 = (f_balance(v_equation.clone())?).items()?;
    if t188.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t188[0].clone();
    v_species = t188[1].clone();
    v_nr = t188[2].clone();
    return Ok(Py::dict(vec![(Py::str("coefficients"), { let mut out = vec![]; for t189 in py::iter(&(v_coefs.clone()))? { let mut v_c: Py; 
v_c = t189;
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
    if ({ let t332 = v_T1.clone(); Py::Bool({ let t333 = v_T2.clone(); if py::eq(&t332, &t333) { true } else { false } }) }).truthy() {
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
    let t380 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t380[0].clone();
    v_y = t380[1].clone();
    v_c = py::truediv(&v_c_common.clone(), &Py::Float(1000.0))?;
    let t383 = ((if ({ let t381 = v_common.clone(); Py::Bool({ let t382 = Py::str("cation"); if py::eq(&t381, &t382) { true } else { false } }) }).truthy() { Py::tuple(vec![v_c.clone(), Py::Float(0.0)]) } else { Py::tuple(vec![Py::Float(0.0), v_c.clone()]) })).items()?;
    if t383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ca = t383[0].clone();
    v_cb = t383[1].clone();
    v_term = { let v_Ksp = v_Ksp.clone(); let v_c_common = v_c_common.clone(); let v_ca = v_ca.clone(); let v_cb = v_cb.clone(); let v_common = v_common.clone(); let v_x = v_x.clone(); let v_y = v_y.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_u: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t384 = v_c.clone(); Py::Bool({ let t385 = Py::Int(0i64); if py::eq(&t384, &t385) { true } else { false } }) }).truthy() { py::add(&py::math1("log", &v_k.clone())?, &v_u.clone())? } else { py::math1("log", &py::add(&py::mul(&v_k.clone(), &py::math1("exp", &v_u.clone())?)?, &v_c.clone())?)? }));
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
fn f_consecutive(mut v_A0: Py, mut v_k1: Py, mut v_k2: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("A0"), v_A0.clone()), (Py::str("k1"), v_k1.clone()), (Py::str("k2"), v_k2.clone()), (Py::str("t"), v_t.clone())]))?;
    v_A = py::mul(&v_A0.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)?;
    v_d = py::sub(&v_k2.clone(), &v_k1.clone())?;
    v_B = (if ({ let t422 = v_d.clone(); Py::Bool({ let t423 = Py::Int(0i64); if py::eq(&t422, &t423) { true } else { false } }) }).truthy() { py::mul(&py::mul(&py::mul(&v_A0.clone(), &v_k1.clone())?, &v_t.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)? } else { py::truediv(&py::mul(&py::mul(&py::mul(&v_A0.clone(), &v_k1.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k1.clone())?, &v_t.clone())?)?)?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_d.clone())?, &v_t.clone())?)?)?)?, &v_d.clone())? });
    return Ok(Py::dict(vec![(Py::str("B"), v_B.clone()), (Py::str("A"), v_A.clone()), (Py::str("C"), py::sub(&py::sub(&v_A0.clone(), &v_A.clone())?, &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crossover_t(mut v_dH: Py, mut v_dS: Py) -> R {
    let _ = f__finite(Py::dict(vec![(Py::str("dH"), v_dH.clone()), (Py::str("dS"), v_dS.clone())]))?;
    if ({ let t443 = { let t432 = v_dS.clone(); Py::Bool({ let t433 = Py::Int(0i64); if py::eq(&t432, &t433) { true } else { false } }) }; if t443.truthy() { t443 } else { { let t442 = { let t434 = v_dH.clone(); Py::Bool({ let t435 = Py::Int(0i64); if py::eq(&t434, &t435) { true } else { false } }) }; if t442.truthy() { t442 } else { { let t436 = { let t438 = v_dH.clone(); Py::Bool({ let t439 = Py::Int(0i64); if py::gt(&t438, &t439)? { true } else { false } }) }; Py::Bool({ let t437 = { let t440 = v_dS.clone(); Py::Bool({ let t441 = Py::Int(0i64); if py::gt(&t440, &t441)? { true } else { false } }) }; if py::ne(&t436, &t437) { true } else { false } }) } } } } }).truthy() {
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
    if (Py::Bool(({ let mut out = vec![]; for t454 in py::iter(&(v_moles.clone()))? { let mut v_n: Py; 
v_n = t454;
out.push({ let t455 = v_n.clone(); Py::Bool({ let t456 = Py::Int(0i64); if py::le(&t455, &t456)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every amount must be positive"))));
    }
    v_tot = py::fsum(&v_moles.clone())?;
    v_x = { let mut out = vec![]; for t457 in py::iter(&(v_moles.clone()))? { let mut v_n: Py; 
v_n = t457;
out.push(py::truediv(&v_n.clone(), &v_tot.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("partial_pressures"), { let mut out = vec![]; for t458 in py::iter(&(v_x.clone()))? { let mut v_xi: Py; 
v_xi = t458;
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
    v_mono = py::add(&py::method(&v_c.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?, &py::sum(&{ let mut out = vec![]; for t491 in py::iter(&(Py::tuple(vec![Py::str("F"), Py::str("Cl"), Py::str("Br"), Py::str("I")])))? { let mut v_x: Py; 
v_x = t491;
out.push(py::method(&v_c.clone(), "get", &[v_x.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)?;
    v_d = py::fraction(&[py::sub(&py::add(&py::add(&py::mul(&Py::Int(2i64), &v_tet.clone())?, &Py::Int(2i64))?, &v_tri.clone())?, &v_mono.clone())?, Py::Int(2i64)])?;
    if ({ let t496 = { let t492 = v_d.clone(); Py::Bool({ let t493 = Py::Int(0i64); if py::lt(&t492, &t493)? { true } else { false } }) }; if t496.truthy() { t496 } else { { let t494 = py::denominator(&v_d.clone())?; Py::Bool({ let t495 = Py::Int(1i64); if py::ne(&t494, &t495) { true } else { false } }) } } }).truthy() {
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
    if ({ let t561 = v_V2.clone(); Py::Bool({ let t562 = v_V1.clone(); if py::lt(&t561, &t562)? { true } else { false } }) }).truthy() {
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
    if ({ let t573 = v_c2.clone(); Py::Bool({ let t574 = v_c1.clone(); if py::gt(&t573, &t574)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stock solution must be more concentrated than the diluted one"))));
    }
    v_V1 = py::truediv(&py::mul(&v_c2.clone(), &v_V2.clone())?, &v_c1.clone())?;
    return Ok(Py::dict(vec![(Py::str("V1"), v_V1.clone()), (Py::str("V1_mL"), py::mul(&v_V1.clone(), &Py::Float(1000000.0))?)]));
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
    v_moles = { let mut out = vec![]; for t619 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t619;
out.push((v_e.clone(), py::truediv(&py::getitem(&v_comp.clone(), &v_e.clone())?, &py::getitem(&c_W()?, &v_e.clone())?)?)); } Py::dict(out) };
    v_lo = py::minmax(&[py::method(&v_moles.clone(), "values", &[])?], false)?;
    v_ratio = { let mut out = vec![]; for t620 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t620;
out.push((v_e.clone(), py::truediv(&py::getitem(&v_moles.clone(), &v_e.clone())?, &v_lo.clone())?)); } Py::dict(out) };
    for t621 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(13i64)).i()?]) {
        v_k = t621;
        v_ints = { let mut out = vec![]; for t622 in py::iter(&(py::method(&v_ratio.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_r: Py; 
let t623 = (t622).items()?;
if t623.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t623[0].clone();
v_r = t623[1].clone();
out.push((v_e.clone(), py::round(&py::mul(&v_k.clone(), &v_r.clone())?, None)?)); } Py::dict(out) };
        if (Py::Bool(({ let mut out = vec![]; for t624 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t624;
out.push({ let t629 = { let t625 = py::getitem(&v_ints.clone(), &v_e.clone())?; Py::Bool({ let t626 = Py::Int(1i64); if py::ge(&t625, &t626)? { true } else { false } }) }; if !t629.truthy() { t629 } else { { let t627 = py::abs(&py::sub(&py::mul(&v_k.clone(), &py::getitem(&v_ratio.clone(), &v_e.clone())?)?, &py::getitem(&v_ints.clone(), &v_e.clone())?)?)?; Py::Bool({ let t628 = py::mul(&Py::Float(0.015), &py::getitem(&v_ints.clone(), &v_e.clone())?)?; if py::le(&t627, &t628)? { true } else { false } }) } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
            v_g = Py::Int(0i64);
            for t630 in py::iter(&(py::method(&v_ints.clone(), "values", &[])?))? {
                v_v = t630;
                v_g = py::gcd(&v_g.clone(), &v_v.clone())?;
            }
            v_ints = { let mut out = vec![]; for t631 in py::iter(&(py::method(&v_ints.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_v: Py; 
let t632 = (t631).items()?;
if t632.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t632[0].clone();
v_v = t632[1].clone();
out.push((v_e.clone(), py::floordiv(&v_v.clone(), &v_g.clone())?)); } Py::dict(out) };
            return Ok(Py::dict(vec![(Py::str("formula"), f__write_formula(v_order.clone(), v_ints.clone())?), (Py::str("counts"), { let mut out = vec![]; for t636 in py::iter(&(v_order.clone()))? { let mut v_e: Py; 
v_e = t636;
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
    let t725 = (f__ice_terms(v_equation.clone())?).items()?;
    if t725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nu = t725[0].clone();
    v_sp = t725[1].clone();
    if ({ let t726 = py::len(&v_initial.clone())?; Py::Bool({ let t727 = py::len(&v_sp.clone())?; if py::ne(&t726, &t727) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one initial concentration per species (".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_sp.clone()])?, "")?, ")".to_string()].concat()))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t728 in py::iter(&(v_initial.clone()))? { let mut v_c: Py; 
v_c = t728;
out.push({ let t729 = v_c.clone(); Py::Bool({ let t730 = Py::Int(0i64); if py::lt(&t729, &t730)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("concentrations cannot be negative"))));
    }
    let _ = f__pos(Py::dict(vec![(Py::str("K"), v_K.clone())]))?;
    let t747 = (f__ice_bounds(v_nu.clone(), v_initial.clone())?).items()?;
    if t747.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t747[0].clone();
    v_hi = t747[1].clone();
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
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone()), (Py::str("equilibrium"), { let mut out = vec![]; for t771 in py::iter(&(py::zip(&[v_initial.clone(), v_nu.clone()])?))? { let mut v_c: Py; let mut v_n: Py; 
let t772 = (t771).items()?;
if t772.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t772[0].clone();
v_n = t772[1].clone();
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
    if ({ let t811 = v_A.clone(); Py::Bool({ let t812 = v_A0.clone(); if py::ge(&t811, &t812)? { true } else { false } }) }).truthy() {
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
    let t2726 = (f__as_written_balanced(v_equation.clone())?).items()?;
    if t2726.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t2726[0].clone();
    v_species = t2726[1].clone();
    v_nr = t2726[2].clone();
    if ({ let t2727 = py::len(&v_values.clone())?; Py::Bool({ let t2728 = py::len(&v_species.clone())?; if py::ne(&t2727, &t2728) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["give one value per species in the order written (".to_string(), py::format(&py::method(&Py::str(", "), "join", &[v_species.clone()])?, "")?, ")".to_string()].concat()))));
    }
    v_tot = py::fsum(&{ let mut out = vec![]; for t2729 in py::iter(&(py::enumerate(&py::zip(&[v_coefs.clone(), v_values.clone()])?, 0)?))? { let mut v_c: Py; let mut v_j: Py; let mut v_v: Py; 
let t2730 = (t2729).items()?;
if t2730.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t2730[0].clone();
let t2731 = (t2730[1].clone()).items()?;
if t2731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t2731[0].clone();
v_v = t2731[1].clone();
out.push(py::mul(&(if ({ let t2732 = v_j.clone(); Py::Bool({ let t2733 = v_nr.clone(); if py::ge(&t2732, &t2733)? { true } else { false } }) }).truthy() { v_c.clone() } else { py::neg(&v_c.clone())? }), &v_v.clone())?); } Py::list(out) })?;
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
    for t3103 in py::iter(&(py::method(&c__HYDRATE_SPLIT()?, "split", &[v_s.clone()])?))? {
        v_part = t3103;
        if (py::not(&v_part.clone())).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("malformed hydrate formula"))));
        }
        v_m = ax_engine::pyre::module_call("match", &Py::str("(\\d+)(?=[A-Z(\\[{])"), &[v_part.clone()], 0)?;
        v_mult = (if (v_m.clone()).truthy() { py::int(&py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?)? } else { Py::Int(1i64) });
        if (v_m.clone()).truthy() {
            v_part = py::slice(&v_part.clone(), Some((py::method(&v_m.clone(), "end", &[])?).i()?), None, None)?;
        }
        if ({ let t3104 = v_mult.clone(); Py::Bool({ let t3105 = Py::Int(0i64); if py::eq(&t3104, &t3105) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a multiplier of 0 is not allowed"))));
        }
        for t3125 in py::iter(&(py::method(&f__group_counts(v_part.clone())?, "items", &[])?))? {
            let t3126 = (t3125).items()?;
            if t3126.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_el = t3126[0].clone();
            v_n = t3126[1].clone();
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
    return Ok(Py::dict(vec![(Py::str("dG"), v_dG.clone()), (Py::str("dG_kJ"), py::truediv(&v_dG.clone(), &Py::Float(1000.0))?), (Py::str("spontaneity"), (if ({ let t910 = v_dG.clone(); Py::Bool({ let t911 = Py::Int(0i64); if py::lt(&t910, &t911)? { true } else { false } }) }).truthy() { Py::str("spontaneous") } else { (if ({ let t912 = v_dG.clone(); Py::Bool({ let t913 = Py::Int(0i64); if py::gt(&t912, &t913)? { true } else { false } }) }).truthy() { Py::str("non-spontaneous") } else { Py::str("at equilibrium") }) }))]));
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
fn f_hydrogen_line(mut v_n_upper: Py, mut v_n_lower: Py) -> R {
    let mut v_inv: Py = Py::None;
    let t1104 = (Py::tuple(vec![py::int(&v_n_upper.clone())?, py::int(&v_n_lower.clone())?])).items()?;
    if t1104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n_upper = t1104[0].clone();
    v_n_lower = t1104[1].clone();
    if ({ let t1109 = { let t1105 = v_n_lower.clone(); Py::Bool({ let t1106 = Py::Int(1i64); if py::lt(&t1105, &t1106)? { true } else { false } }) }; if t1109.truthy() { t1109 } else { { let t1107 = v_n_upper.clone(); Py::Bool({ let t1108 = v_n_lower.clone(); if py::le(&t1107, &t1108)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the levels must be whole numbers with n_upper > n_lower >= 1"))));
    }
    v_inv = py::mul(&c_R_H()?, &py::sub(&py::truediv(&Py::Float(1.0), &py::pow(&v_n_lower.clone(), &Py::Int(2i64))?)?, &py::truediv(&Py::Float(1.0), &py::pow(&v_n_upper.clone(), &Py::Int(2i64))?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("lam"), py::truediv(&Py::Float(1.0), &v_inv.clone())?), (Py::str("lam_nm"), py::truediv(&Py::Float(1000000000.0), &v_inv.clone())?), (Py::str("E"), py::mul(&py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?, &v_inv.clone())?)]));
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
    return Ok(Py::dict(vec![(Py::str("lnK"), v_lnK.clone()), (Py::str("K"), (if ({ let t1234 = v_lnK.clone(); Py::Bool({ let t1235 = Py::Int(700i64); if py::lt(&t1234, &t1235)? { true } else { false } }) }).truthy() { py::math1("exp", &v_lnK.clone())? } else { Py::Float(f64::INFINITY) }))]));
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
    if ({ let t1263 = { let t1259 = v_a.clone(); Py::Bool({ let t1260 = Py::Int(0i64); if py::le(&t1259, &t1260)? { true } else { false } }) }; if t1263.truthy() { t1263 } else { { let t1261 = v_a.clone(); Py::Bool({ let t1262 = v_C.clone(); if py::ge(&t1261, &t1262)? { true } else { false } }) } } }).truthy() {
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
    let t1314 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t1314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1314[0].clone();
    v_y = t1314[1].clone();
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
    let t1325 = (Py::tuple(vec![py::int(&v_x.clone())?, py::int(&v_y.clone())?])).items()?;
    if t1325.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t1325[0].clone();
    v_y = t1325[1].clone();
    if ({ let t1330 = { let t1326 = v_x.clone(); Py::Bool({ let t1327 = Py::Int(1i64); if py::lt(&t1326, &t1327)? { true } else { false } }) }; if t1330.truthy() { t1330 } else { { let t1328 = v_y.clone(); Py::Bool({ let t1329 = Py::Int(1i64); if py::lt(&t1328, &t1329)? { true } else { false } }) } } }).truthy() {
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
    let t1431 = (f_balance(v_equation.clone())?).items()?;
    if t1431.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t1431[0].clone();
    v_species = t1431[1].clone();
    v_nr = t1431[2].clone();
    let t1436 = (f__role(v_species.clone(), v_nr.clone(), v_reactant_a.clone())?).items()?;
    if t1436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ja = t1436[0].clone();
    v_ra = t1436[1].clone();
    let t1437 = (f__role(v_species.clone(), v_nr.clone(), v_reactant_b.clone())?).items()?;
    if t1437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jb = t1437[0].clone();
    v_rb = t1437[1].clone();
    let t1438 = (f__role(v_species.clone(), v_nr.clone(), v_product.clone())?).items()?;
    if t1438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jp = t1438[0].clone();
    v_rp = t1438[1].clone();
    if ({ let t1442 = py::not(&{ let t1439 = v_ra.clone(); if !t1439.truthy() { t1439 } else { v_rb.clone() } }); if t1442.truthy() { t1442 } else { { let t1440 = v_ja.clone(); Py::Bool({ let t1441 = v_jb.clone(); if py::eq(&t1440, &t1441) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two given substances must be two different reactants"))));
    }
    if (v_rp.clone()).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_product.clone(), "")?, " is a reactant, not a product".to_string()].concat()))));
    }
    let t1445 = (Py::tuple(vec![py::mul(&py::truediv(&v_m_a.clone(), &f_molar_mass_g(v_reactant_a.clone())?)?, &Py::Float(1000.0))?, py::mul(&py::truediv(&v_m_b.clone(), &f_molar_mass_g(v_reactant_b.clone())?)?, &Py::Float(1000.0))?])).items()?;
    if t1445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_na = t1445[0].clone();
    v_nb = t1445[1].clone();
    let t1446 = (Py::tuple(vec![py::truediv(&v_na.clone(), &py::getitem(&v_coefs.clone(), &v_ja.clone())?)?, py::truediv(&v_nb.clone(), &py::getitem(&v_coefs.clone(), &v_jb.clone())?)?])).items()?;
    if t1446.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xa = t1446[0].clone();
    v_xb = t1446[1].clone();
    let t1449 = ((if ({ let t1447 = v_xa.clone(); Py::Bool({ let t1448 = v_xb.clone(); if py::le(&t1447, &t1448)? { true } else { false } }) }).truthy() { Py::tuple(vec![v_xa.clone(), v_reactant_a.clone(), v_reactant_b.clone(), v_nb.clone(), v_jb.clone()]) } else { Py::tuple(vec![v_xb.clone(), v_reactant_b.clone(), v_reactant_a.clone(), v_na.clone(), v_ja.clone()]) })).items()?;
    if t1449.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_ext = t1449[0].clone();
    v_lim = t1449[1].clone();
    v_other = t1449[2].clone();
    v_n_o = t1449[3].clone();
    v_j_o = t1449[4].clone();
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
    if ({ let t1582 = v_el.clone(); Py::Bool({ let t1583 = v_counts.clone(); if !py::contains(&t1583, &t1582)? { true } else { false } }) }).truthy() {
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
    if ({ let t1646 = v_el.clone(); Py::Bool({ let t1647 = v_counts.clone(); if !py::contains(&t1647, &t1646)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_el.clone(), "")?, " does not occur in ".to_string(), py::format(&v_formula.clone(), "")?].concat()))));
    }
    v_total = f_molar_mass_g(v_formula.clone())?;
    v_comp = { let mut out = vec![]; for t1650 in py::iter(&(py::method(&v_counts.clone(), "items", &[])?))? { let mut v_e: Py; let mut v_n: Py; 
let t1651 = (t1650).items()?;
if t1651.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t1651[0].clone();
v_n = t1651[1].clone();
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
    return Ok(py::fsum(&{ let mut out = vec![]; for t2992 in py::iter(&(py::method(&f_formula_counts(v_text.clone())?, "items", &[])?))? { let mut v_el: Py; let mut v_n: Py; 
let t2993 = (t2992).items()?;
if t2993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_el = t2993[0].clone();
v_n = t2993[1].clone();
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
    let t2615 = (Py::tuple(vec![py::truediv(&v_m_solute.clone(), &f_molar_mass_g(v_formula_solute.clone())?)?, py::truediv(&v_m_solvent.clone(), &f_molar_mass_g(v_formula_solvent.clone())?)?])).items()?;
    if t2615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t2615[0].clone();
    v_n2 = t2615[1].clone();
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
    if ({ let t2075 = { let t2071 = v_n.clone(); Py::Bool({ let t2072 = Py::Int(1i64); if py::lt(&t2071, &t2072)? { true } else { false } }) }; if t2075.truthy() { t2075 } else { { let t2073 = py::abs(&py::sub(&v_k.clone(), &v_n.clone())?)?; Py::Bool({ let t2074 = py::mul(&Py::Float(0.02), &v_n.clone())?; if py::gt(&t2073, &t2074)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the molar mass is ".to_string(), py::format(&v_k.clone(), ".3f")?, " times the empirical formula mass, not a whole multiple".to_string()].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("multiple"), py::float(&v_n.clone())?), (Py::str("formula"), f__write_formula(v_order.clone(), { let mut out = vec![]; for t2079 in py::iter(&(py::method(&v_counts.clone(), "items", &[])?))? { let mut v_c: Py; let mut v_e: Py; 
let t2080 = (t2079).items()?;
if t2080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t2080[0].clone();
v_c = t2080[1].clone();
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
    if ({ let t2897 = { let t2894 = py::len(&v_sides.clone())?; Py::Bool({ let t2895 = Py::Int(2i64); if py::ne(&t2894, &t2895) { true } else { false } }) }; if t2897.truthy() { t2897 } else { { let t2896 = py::not(&py::method(&py::getitem(&v_sides.clone(), &Py::Int(0i64))?, "strip", &[])?); if t2896.truthy() { t2896 } else { py::not(&py::method(&py::getitem(&v_sides.clone(), &Py::Int(1i64))?, "strip", &[])?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an equation needs reactants, one arrow and products"))));
    }
    v_out = Py::list(vec![]);
    for t2898 in py::iter(&(v_sides.clone()))? {
        v_side = t2898;
        v_terms = Py::list(vec![]);
        for t2899 in py::iter(&(py::method(&v_side.clone(), "split", &[Py::str("+")])?))? {
            v_term = t2899;
            v_term = py::method(&v_term.clone(), "strip", &[])?;
            v_m = ax_engine::pyre::module_call("fullmatch", &Py::str("(\\d+(?:/\\d+)?)?\\s*(\\S+)"), &[v_term.clone()], 0)?;
            if (py::not(&v_m.clone())).truthy() {
                return Err(py::value_error(py::to_str(&Py::str(&["cannot read the term '".to_string(), py::format(&v_term.clone(), "")?, "'".to_string()].concat()))));
            }
            v_coef = (if (py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?).truthy() { py::fraction(&[py::method(&v_m.clone(), "group", &[Py::Int(1i64)])?])? } else { py::fraction(&[Py::Int(1i64)])? });
            if ({ let t2900 = v_coef.clone(); Py::Bool({ let t2901 = Py::Int(0i64); if py::le(&t2900, &t2901)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("coefficients must be positive"))));
            }
            let _ = f_formula_counts(py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?)?;
            let _ = py::method(&v_terms.clone(), "append", &[Py::tuple(vec![v_coef.clone(), py::method(&v_m.clone(), "group", &[Py::Int(2i64)])?])])?;
        }
        let _ = py::method(&v_out.clone(), "append", &[v_terms.clone()])?;
    }
    v_names = { let mut out = vec![]; for t2926 in py::iter(&(py::add(&py::getitem(&v_out.clone(), &Py::Int(0i64))?, &py::getitem(&v_out.clone(), &Py::Int(1i64))?)?))? { let mut v__c: Py; let mut v_f: Py; 
let t2927 = (t2926).items()?;
if t2927.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__c = t2927[0].clone();
v_f = t2927[1].clone();
out.push(v_f.clone()); } Py::list(out) };
    if ({ let t2928 = py::len(&py::set(&v_names.clone())?)?; Py::Bool({ let t2929 = py::len(&v_names.clone())?; if py::ne(&t2928, &t2929) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a species appears more than once"))));
    }
    return Ok(Py::tuple(vec![py::getitem(&v_out.clone(), &Py::Int(0i64))?, py::getitem(&v_out.clone(), &Py::Int(1i64))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_pressure(mut v_x: Py, mut v_P_total: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("P_total"), v_P_total.clone())]))?;
    if (py::not(&{ let t2220 = Py::Int(0i64); Py::Bool({ let t2221 = v_x.clone(); if py::lt(&t2220, &t2221)? { { let t2222 = Py::Int(1i64); if py::le(&t2221, &t2222)? { true } else { false } } } else { false } }) })).truthy() {
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
fn f_raoult(mut v_formula_solute: Py, mut v_m_solute: Py, mut v_formula_solvent: Py, mut v_m_solvent: Py, mut v_P0: Py) -> R {
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::dict(vec![(Py::str("P0"), v_P0.clone())]))?;
    v_x = py::getitem(&f_mole_fraction(v_formula_solute.clone(), v_m_solute.clone(), v_formula_solvent.clone(), v_m_solvent.clone())?, &Py::str("x_solvent"))?;
    return Ok(Py::dict(vec![(Py::str("P"), py::mul(&v_x.clone(), &v_P0.clone())?), (Py::str("dP"), py::mul(&py::sub(&Py::Int(1i64), &v_x.clone())?, &v_P0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_order(mut v_c1: Py, mut v_r1: Py, mut v_c2: Py, mut v_r2: Py) -> R {
    let _ = f__pos(Py::dict(vec![(Py::str("c1"), v_c1.clone()), (Py::str("r1"), v_r1.clone()), (Py::str("c2"), v_c2.clone()), (Py::str("r2"), v_r2.clone())]))?;
    if ({ let t2747 = v_c1.clone(); Py::Bool({ let t2748 = v_c2.clone(); if py::eq(&t2747, &t2748) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two concentrations must differ"))));
    }
    return Ok(Py::dict(vec![(Py::str("order"), py::truediv(&py::math1("log", &py::truediv(&v_r2.clone(), &v_r1.clone())?)?, &py::math1("log", &py::truediv(&v_c2.clone(), &v_c1.clone())?)?)?)]));
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
    let t2984 = (f_balance(v_equation.clone())?).items()?;
    if t2984.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_coefs = t2984[0].clone();
    v_species = t2984[1].clone();
    v_nr = t2984[2].clone();
    let t2989 = (f__role(v_species.clone(), v_nr.clone(), v_reactant.clone())?).items()?;
    if t2989.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jr = t2989[0].clone();
    v_rr = t2989[1].clone();
    let t2990 = (f__role(v_species.clone(), v_nr.clone(), v_product.clone())?).items()?;
    if t2990.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_jp = t2990[0].clone();
    v_rp = t2990[1].clone();
    if ({ let t2991 = py::not(&v_rr.clone()); if t2991.truthy() { t2991 } else { v_rp.clone() } }).truthy() {
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
    if ({ let t3081 = v_V.clone(); Py::Bool({ let t3082 = py::mul(&v_n.clone(), &v_b.clone())?; if py::le(&t3081, &t3082)? { true } else { false } }) }).truthy() {
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
    v_atoms = { let mut out = vec![]; for t3127 in py::iter(&(py::method(&v_c.clone(), "items", &[])?))? { let mut v_el: Py; let mut v_n: Py; 
let t3128 = (t3127).items()?;
if t3128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_el = t3128[0].clone();
v_n = t3128[1].clone();
for t3129 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3129;
out.push(v_el.clone()); }} Py::list(out) };
    if ({ let t3135 = { let t3130 = py::len(&v_atoms.clone())?; Py::Bool({ let t3131 = Py::Int(2i64); if py::ne(&t3130, &t3131) { true } else { false } }) }; if t3135.truthy() { t3135 } else { Py::Bool(({ let mut out = vec![]; for t3132 in py::iter(&(v_atoms.clone()))? { let mut v_a: Py; 
v_a = t3132;
out.push({ let t3133 = v_a.clone(); Py::Bool({ let t3134 = c_ISO()?; if !py::contains(&t3134, &t3133)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a diatomic molecule of H, C, N, O, F, Cl, Br, I, S ... is expected"))));
    }
    let t3136 = (Py::tuple(vec![py::mul(&py::getitem(&c_ISO()?, &py::getitem(&v_atoms.clone(), &Py::Int(0i64))?)?, &c_U_KG()?)?, py::mul(&py::getitem(&c_ISO()?, &py::getitem(&v_atoms.clone(), &Py::Int(1i64))?)?, &c_U_KG()?)?])).items()?;
    if t3136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m1 = t3136[0].clone();
    v_m2 = t3136[1].clone();
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
    if ({ let t3148 = v_A.clone(); Py::Bool({ let t3149 = Py::Int(0i64); if py::lt(&t3148, &t3149)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the reactant is used up before this time (A0 - k t < 0)"))));
    }
    return Ok(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("A_molL"), py::truediv(&v_A.clone(), &Py::Float(1000.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1005(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("ratio"))?, &Py::Int(2i64))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_1"))?)?)?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1072(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("q"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("dH"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1110(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&Py::Float(1.0), &py::getitem(&v_o.clone(), &Py::str("lam"))?)?, py::mul(&c_R_H()?, &py::sub(&py::truediv(&Py::Float(1.0), &py::pow(&py::getitem(&v_i.clone(), &Py::str("n_lower"))?, &Py::Int(2i64))?)?, &py::truediv(&Py::Float(1.0), &py::pow(&py::getitem(&v_i.clone(), &Py::str("n_upper"))?, &Py::Int(2i64))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1117(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("h"))?, py::mul(&Py::Float(1000.0), &py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_i.clone(), &Py::str("pH"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1153(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("m"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1191(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1201(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1211(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_R()?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1221(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1236(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("lnK"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_F()?)?, &py::getitem(&v_i.clone(), &Py::str("E"))?)?)?)?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1246(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::add(&py::truediv(&py::abs(&py::add(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("lnK"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::getitem(&v_i.clone(), &Py::str("dG"))?)?)?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?, &f__rel(py::method(&v_o.clone(), "get", &[Py::str("K"), py::math1("exp", &py::getitem(&v_o.clone(), &Py::str("lnK"))?)?])?, py::math1("exp", &py::getitem(&v_o.clone(), &Py::str("lnK"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1264(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::neg(&py::math1("log10", &f__weak_h(py::pow(&Py::Float(10.0), &py::neg(&py::getitem(&v_o.clone(), &Py::str("pKa"))?)?)?, py::truediv(&py::getitem(&v_i.clone(), &Py::str("c"))?, &Py::Float(1000.0))?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("pH"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1291(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("Kc"))?, &py::pow(&py::mul(&c_R_LATM()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::getitem(&v_i.clone(), &Py::str("dn"))?)?)?, py::getitem(&v_i.clone(), &Py::str("Kp"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1304(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("Kp"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("Kc"))?, &py::pow(&py::mul(&c_R_LATM()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::getitem(&v_i.clone(), &Py::str("dn"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1315(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::pow(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("Ksp"))?, &py::mul(&py::pow(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("y"))?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?)?, &py::truediv(&Py::Float(1.0), &py::add(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("s"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1331(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::pow(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_o.clone(), &Py::str("s"))?)?, &Py::Float(1000.0))?, &py::getitem(&v_i.clone(), &Py::str("x"))?)?, &py::pow(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("y"))?, &py::getitem(&v_o.clone(), &Py::str("s"))?)?, &Py::Float(1000.0))?, &py::getitem(&v_i.clone(), &Py::str("y"))?)?)?, py::getitem(&v_i.clone(), &Py::str("Ksp"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1516(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("m"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1586(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("m_element"))?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?)?);
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
fn s_1652(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("percent"))?, py::truediv(&py::mul(&py::mul(&Py::Float(100.0), &py::getitem(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("element"))?)?)?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1716(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("u_mean"))?, &Py::Int(2i64))?, &Py::Float(std::f64::consts::PI))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::mul(&py::mul(&Py::Int(8i64), &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1780(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("m_solvent"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("m_solute"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1838(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__rel(py::getitem(&v_o.clone(), &Py::str("M_g"))?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &Py::Float(1000.0))?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1876(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("dT"))?)?, &py::getitem(&v_i.clone(), &Py::str("m_solvent"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, &py::getitem(&v_i.clone(), &Py::str("m_solute"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1886(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("rho"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1896(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("P"))?)?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1906(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &py::getitem(&v_i.clone(), &Py::str("Pi"))?)?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, py::mul(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("m"))?)?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1942(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("c"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("m"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2007(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::getitem(&v_o.clone(), &Py::str("x_solute"))?, &py::sub(&Py::Int(1i64), &py::getitem(&v_o.clone(), &Py::str("x_solute"))?)?)?, py::truediv(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solute"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solute"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solvent"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solvent"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2081(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t2119 = { let t2082 = py::getitem(&v_o.clone(), &Py::str("multiple"))?; Py::Bool({ let t2083 = py::round(&py::getitem(&v_o.clone(), &Py::str("multiple"))?, None)?; if py::eq(&t2082, &t2083) { true } else { false } }) }; if !t2119.truthy() { t2119 } else { { let t2118 = { let t2084 = py::abs(&py::sub(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("M"))?, &py::truediv(&f__mm_rd(py::getitem(&v_i.clone(), &Py::str("empirical"))?)?, &Py::Float(1000.0))?)?, &py::getitem(&v_o.clone(), &Py::str("multiple"))?)?)?; Py::Bool({ let t2085 = py::mul(&Py::Float(0.02), &py::getitem(&v_o.clone(), &Py::str("multiple"))?)?; if py::le(&t2084, &t2085)? { true } else { false } }) }; if !t2118.truthy() { t2118 } else { { let t2114 = f__counts_rd(py::getitem(&v_o.clone(), &Py::str("formula"))?)?; Py::Bool({ let t2115 = { let mut out = vec![]; for t2116 in py::iter(&(py::method(&f__counts_rd(py::getitem(&v_i.clone(), &Py::str("empirical"))?)?, "items", &[])?))? { let mut v_c: Py; let mut v_e: Py; 
let t2117 = (t2116).items()?;
if t2117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_e = t2117[0].clone();
v_c = t2117[1].clone();
out.push((v_e.clone(), py::mul(&v_c.clone(), &py::int(&py::getitem(&v_o.clone(), &Py::str("multiple"))?)?)?)); } Py::dict(out) }; if py::eq(&t2114, &t2115) { true } else { false } }) } } } } }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2152(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::getitem(&v_i.clone(), &Py::str("m"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2190(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("n"))?, &c_NA()?)?, py::getitem(&v_i.clone(), &Py::str("N"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2203(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_i.clone(), &Py::str("E0"))?)?, &py::mul(&py::truediv(&py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_F()?)?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("Q"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("E0"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2210(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("Pi"))?, py::mul(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2223(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("p"))?, py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::getitem(&v_i.clone(), &Py::str("P_total"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2259(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("N"))?, py::mul(&py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &Py::Float(1000.0))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &c_NA()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2297(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::getitem(&v_o.clone(), &Py::str("N"))?, &c_NA()?)?, py::getitem(&v_i.clone(), &Py::str("n"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2407(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2462(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2469(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::getitem(&v_o.clone(), &Py::str("pH"))?, &py::math1("log10", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("h"))?, &Py::Float(1000.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2476(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::sub(&Py::Float(14.0), &py::getitem(&v_o.clone(), &Py::str("pH"))?)?, &py::math1("log10", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("oh"))?, &Py::Float(1000.0))?)?)?)?);
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
fn s_2500(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2507(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2514(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2538(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_255(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("A"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("eps"))?, &py::getitem(&v_i.clone(), &Py::str("l"))?)?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2562(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_2569(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("E"))?, &py::getitem(&v_i.clone(), &Py::str("lam"))?)?, py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2579(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("lam"))?, &py::getitem(&v_i.clone(), &Py::str("E"))?)?, py::mul(&c_H_PLANCK()?, &c_C_LIGHT()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2616(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("P"))?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P0"))?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solvent"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solvent"))?)?)?)?, &py::add(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solvent"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solvent"))?)?)?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("m_solute"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_solute"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_265(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("c"))?, &py::getitem(&v_i.clone(), &Py::str("eps"))?)?, &py::getitem(&v_i.clone(), &Py::str("l"))?)?, py::getitem(&v_i.clone(), &Py::str("A"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2734(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_sp: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_nr: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dH"))?, &Py::Float(1000.0))?, &py::fsum(&{ let mut out = vec![]; for t2735 in py::iter(&(py::enumerate(&py::zip(&[v_c.clone(), py::getitem(&v_i.clone(), &Py::str("values"))?])?, 0)?))? { let mut v_j: Py; let mut v_v: Py; let mut v_x: Py; 
let t2736 = (t2735).items()?;
if t2736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t2736[0].clone();
let t2737 = (t2736[1].clone()).items()?;
if t2737.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t2737[0].clone();
v_v = t2737[1].clone();
out.push(py::mul(&(if ({ let t2738 = v_j.clone(); Py::Bool({ let t2739 = v_nr.clone(); if py::ge(&t2738, &t2739)? { true } else { false } }) }).truthy() { v_x.clone() } else { py::neg(&v_x.clone())? }), &v_v.clone())?); } Py::list(out) })?)?)?, &py::minmax(&[Py::Float(1.0), py::sum(&{ let mut out = vec![]; for t2740 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("values"))?))? { let mut v_v: Py; 
v_v = t2740;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &({ let mut v: Vec<Py> = vec![]; v.extend((f__as_written_balanced(py::getitem(&v_i.clone(), &Py::str("equation"))?)?).items()?); v })[..])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2749(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("order"))?, &py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("c2"))?, &py::getitem(&v_i.clone(), &Py::str("c1"))?)?)?)?, &py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("r2"))?, &py::getitem(&v_i.clone(), &Py::str("r1"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_275(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("dT"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2782(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::truediv(&py::mul(&py::pow(&py::getitem(&v_o.clone(), &Py::str("u_rms"))?, &Py::Int(2i64))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?, py::mul(&py::mul(&Py::Int(3i64), &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2820(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::sub(&py::truediv(&Py::Float(1.0), &py::getitem(&v_o.clone(), &Py::str("A"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?, py::truediv(&Py::Float(1.0), &py::getitem(&v_i.clone(), &Py::str("A0"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_285(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2856(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("m"))?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("c"))?, &py::getitem(&v_i.clone(), &Py::str("V"))?)?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_295(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V2"))?, &py::getitem(&v_i.clone(), &Py::str("P2"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2994(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_3032(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V_eq"))?, &py::getitem(&v_i.clone(), &Py::str("c_t"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("ratio"))?, &py::getitem(&v_i.clone(), &Py::str("c_a"))?)?, &py::getitem(&v_i.clone(), &Py::str("V_a"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3046(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_305(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("pH"))?, &py::math1("log10", &py::getitem(&v_i.clone(), &Py::str("Ka"))?)?)?, &py::math1("log10", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("c_base"))?, &py::getitem(&v_i.clone(), &Py::str("c_acid"))?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3073(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("pH"))?, &py::getitem(&{ let t3074 = v_i.clone(); f__weak_titr(py::getitem(&t3074, &Py::str("Ka"))?, py::getitem(&t3074, &Py::str("c_a"))?, py::getitem(&t3074, &Py::str("V_a"))?, py::getitem(&t3074, &Py::str("c_b"))?, py::getitem(&t3074, &Py::str("V_b"))?)? }, &Py::str("pH"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3083(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::add(&py::getitem(&v_o.clone(), &Py::str("P"))?, &py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("a"))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("n"))?, &Py::Int(2i64))?)?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("V"))?, &Py::Int(2i64))?)?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("V"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("n"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3096(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("lnK2"))?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("K1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("dH"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_312(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("q"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3137(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("wavenumber"))?, py::getitem(&v_r.clone(), &Py::str("wavenumber"))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let t3141 = v_i.clone(); f_vib_wavenumber(py::getitem(&t3141, &Py::str("molecule"))?, py::getitem(&t3141, &Py::str("k"))?)? }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3150(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::add(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?, py::getitem(&v_i.clone(), &Py::str("A0"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_322(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("V1"))?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_334(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::sub(&py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("P1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dH"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_341(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t342 = py::getitem(&v_o.clone(), &Py::str("P2"))?; Py::Bool({ let t343 = Py::Int(0i64); if py::gt(&t342, &t343)? { true } else { false } }) }).truthy() { py::abs(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("P1"))?)?)?, &py::mul(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("dH"))?, &c_R()?)?, &py::sub(&py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T2"))?)?, &py::truediv(&Py::Int(1i64), &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_350(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_360(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("P1"))?)?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_370(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("V2"))?, &py::getitem(&v_i.clone(), &Py::str("P2"))?)?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_409(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_ca: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_cb: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t410 = v_s.clone(); Py::Bool({ let t411 = Py::Int(0i64); if py::gt(&t410, &t411)? { true } else { false } }) }).truthy() { py::abs(&py::sub(&py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &py::math1("log", &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("x"))?, &v_s.clone())?, &v_ca.clone())?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("y"))?, &py::math1("log", &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("y"))?, &v_s.clone())?, &v_cb.clone())?)?)?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("Ksp"))?)?)?)? } else { Py::Float(1.0) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::truediv(&py::getitem(&v_o.clone(), &Py::str("s"))?, &Py::Float(1000.0))?, (if ({ let t412 = py::getitem(&v_i.clone(), &Py::str("common"))?; Py::Bool({ let t413 = Py::str("cation"); if py::eq(&t412, &t413) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_i.clone(), &Py::str("c_common"))?, &Py::Float(1000.0))? } else { Py::Float(0.0) }), (if ({ let t414 = py::getitem(&v_i.clone(), &Py::str("common"))?; Py::Bool({ let t415 = Py::str("anion"); if py::eq(&t414, &t415) { true } else { false } }) }).truthy() { py::truediv(&py::getitem(&v_i.clone(), &Py::str("c_common"))?, &Py::Float(1000.0))? } else { Py::Float(0.0) })])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_424(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("B"))?, py::getitem(&v_r.clone(), &Py::str("B"))?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[{ let t428 = v_i.clone(); f_consecutive(py::getitem(&t428, &Py::str("A0"))?, py::getitem(&t428, &Py::str("k1"))?, py::getitem(&t428, &Py::str("k2"))?, py::getitem(&t428, &Py::str("t"))?)? }])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_444(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("dS"))?)?, py::getitem(&v_i.clone(), &Py::str("dH"))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_459(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t460 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("partial_pressures"))?, py::getitem(&v_i.clone(), &Py::str("moles"))?])?))? { let mut v_n: Py; let mut v_p: Py; 
let t461 = (t460).items()?;
if t461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t461[0].clone();
v_n = t461[1].clone();
out.push(f__rel(v_p.clone(), py::mul(&py::truediv(&v_n.clone(), &py::fsum(&py::getitem(&v_i.clone(), &Py::str("moles"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("P_total"))?)?)?); } Py::list(out) }, &Py::list(vec![(if ({ let t465 = py::len(&py::getitem(&v_o.clone(), &Py::str("partial_pressures"))?)?; Py::Bool({ let t466 = py::len(&py::getitem(&v_i.clone(), &Py::str("moles"))?)?; if py::eq(&t465, &t466) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) })]))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_497(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_c: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dou"))?, &py::truediv(&py::sub(&py::sub(&py::add(&py::add(&py::add(&py::add(&py::mul(&Py::Int(2i64), &py::add(&py::method(&v_c.clone(), "get", &[Py::str("C"), Py::Int(0i64)])?, &py::method(&v_c.clone(), "get", &[Py::str("Si"), Py::Int(0i64)])?)?)?, &Py::Int(2i64))?, &py::method(&v_c.clone(), "get", &[Py::str("N"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("P"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("B"), Py::Int(0i64)])?)?, &py::method(&v_c.clone(), "get", &[Py::str("H"), Py::Int(0i64)])?)?, &py::sum(&{ let mut out = vec![]; for t498 in py::iter(&(Py::tuple(vec![Py::str("F"), Py::str("Cl"), Py::str("Br"), Py::str("I")])))? { let mut v_x: Py; 
v_x = t498;
out.push(py::method(&v_c.clone(), "get", &[v_x.clone(), Py::Int(0i64)])?); } Py::list(out) }, None)?)?, &Py::Int(2i64))?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__counts_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_534(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("dG"))?, py::mul(&py::mul(&py::neg(&py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_F()?)?, &py::getitem(&v_i.clone(), &Py::str("E"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_544(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::abs(&py::add(&py::truediv(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("K"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_554(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &py::getitem(&v_i.clone(), &Py::str("dG0"))?)?, &py::mul(&py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, &py::math1("log", &py::getitem(&v_i.clone(), &Py::str("Q"))?)?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("dG0"))?)?, py::mul(&c_R()?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_563(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("c2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("c1"))?, &py::getitem(&v_i.clone(), &Py::str("V1"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_575(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("V1"))?, &py::getitem(&v_i.clone(), &Py::str("c1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("c2"))?, &py::getitem(&v_i.clone(), &Py::str("V2"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_589(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, &c_F()?)?, py::truediv(&py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("I"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?, &py::getitem(&c_W()?, &f__element(py::getitem(&v_i.clone(), &Py::str("metal"))?)?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_603(mut v_i: Py, mut v_o: Py) -> R {
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
fn s_678(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::neg(&py::getitem(&v_o.clone(), &Py::str("dH"))?)?, &py::getitem(&v_i.clone(), &Py::str("n"))?)?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("m"))?, &py::getitem(&v_i.clone(), &Py::str("c"))?)?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("T2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_792(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t793 = py::getitem(&v_o.clone(), &Py::str("A"))?; Py::Bool({ let t794 = Py::Int(0i64); if py::gt(&t793, &t794)? { true } else { false } }) }).truthy() { py::abs(&py::add(&py::math1("log", &py::truediv(&py::getitem(&v_o.clone(), &Py::str("A"))?, &py::getitem(&v_i.clone(), &Py::str("A0"))?)?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("t"))?)?)?)? } else { Py::Float(1.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_801(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("t_half"))?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?, py::math1("log", &Py::Float(2.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_813(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("t"))?, &py::getitem(&v_i.clone(), &Py::str("k"))?)?, py::math1("log", &py::truediv(&py::getitem(&v_i.clone(), &Py::str("A0"))?, &py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_823(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::getitem(&v_o.clone(), &Py::str("dT"))?, py::mul(&py::mul(&py::getitem(&v_i.clone(), &Py::str("i"))?, &py::getitem(&v_i.clone(), &Py::str("K"))?)?, &py::getitem(&v_i.clone(), &Py::str("b"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_859(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("rho"))?, &c_R()?)?, &py::getitem(&v_i.clone(), &Py::str("T"))?)?, py::truediv(&py::mul(&py::getitem(&v_i.clone(), &Py::str("P"))?, &f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula"))?)?)?, &Py::Float(1000.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_897(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::getitem(&v_i.clone(), &Py::str("T1"))?)?, py::mul(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::getitem(&v_i.clone(), &Py::str("T2"))?)?)?);
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
fn s_914(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::add(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dG"))?, &py::getitem(&v_i.clone(), &Py::str("dH"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("dS"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("dH"))?)?, py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("T"))?, &py::getitem(&v_i.clone(), &Py::str("dS"))?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_947(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__rel(py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("M"))?, &Py::Float(1000.0))?, &py::pow(&py::getitem(&v_i.clone(), &Py::str("ratio"))?, &Py::Int(2i64))?)?, f__mm_rd(py::getitem(&v_i.clone(), &Py::str("formula_known"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
