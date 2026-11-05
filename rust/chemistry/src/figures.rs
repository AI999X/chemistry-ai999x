//! Chemistry's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model computes:
//! gas laws on P-V and V-T diagrams, kinetics, titration curves, spectra, compositions and substance-by-substance
//! comparisons, value maps over two inputs (docs/FIGURE_PARITY.md). Every value drawn is an answer of the model or a
//! further solve of it; a figure the inputs do not allow is left out.

use crate::figs_common::{self as c, tick, Chart, FieldSpec};
use crate::figs_extra::{bars_full, sub};
use ax_engine::figkit::{self as k, num_of, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

pub use crate::figures_stoich as st;

/// The field maps: capability, output shown, input across, input up, logarithmic (across, up), title, unit of z.
const FIELDS: &[(&str, &str, &str, &str, (bool, bool), &str, &str)] = &[
    ("crossover_temperature", "T", "dH", "dS", (false, false),
     "Temperature at which dG = 0 over enthalpy and entropy change", "K"),
    ("delta_g_from_cell_potential", "dG", "n", "E", (false, false),
     "Gibbs energy from a cell potential over electrons transferred and potential", "J/mol"),
    ("delta_g_from_k", "dG", "T", "K", (false, false),
     "Standard Gibbs energy over temperature and equilibrium constant", "J/mol"),
    ("heat_from_reaction_mass", "q", "m", "dH", (false, false),
     "Heat of reaction over the mass reacting and the enthalpy change", "J"),
    ("k_from_delta_g", "lnK", "T", "dG", (false, false), "ln K over temperature and standard Gibbs energy", ""),
    ("ka_from_ph", "pKa", "pH", "c", (false, true), "pKa of a weak acid over measured pH and concentration", ""),
    ("molality", "b", "m_solute", "m_solvent", (false, false), "Molality over mass of solute and mass of solvent",
     "mol/kg"),
    ("molarity_from_mass", "c", "m", "V", (false, false), "Molarity over mass of solute and volume of solution",
     "mol/m^3"),
    ("mole_fraction", "x_solute", "m_solute", "m_solvent", (false, false),
     "Mole fraction of solute over the two masses", ""),
    ("ph_salt_of_weak_acid", "pH", "c", "Ka", (false, false),
     "pH of a salt of a weak acid over concentration and Ka", ""),
    ("ph_weak_acid", "pH", "c", "Ka", (true, true), "pH of a weak acid over concentration and Ka", ""),
    ("ph_weak_base", "pH", "c", "Kb", (false, false), "pH of a weak base over concentration and Kb", ""),
    ("solute_mass_for_solution", "m", "c", "V", (false, false),
     "Mass of solute for a solution over concentration and volume", "kg"),
];

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    if let Some(f) = FIELDS.iter().find(|f| f.0 == s.key) {
        return c::field_map(s, &FieldSpec { z: f.1, x: f.2, y: f.3, log: f.4, title: f.5, zunit: f.6 })
            .into_iter().collect();
    }
    let v: Vec<Option<Figure>> = match s.key {
        "arrhenius_rate_constant" => vec![arrhenius(s)],
        "boyle_final_pressure" => vec![boyle(s, true)],
        "boyle_final_volume" => vec![boyle(s, false)],
        "charles_final_volume" => vec![state_line(s, "V", "volume", "L", 1000.0)],
        "gay_lussac_final_pressure" => vec![state_line(s, "P", "pressure", "kPa", 0.001)],
        "combined_gas_final_pressure" | "combined_gas_final_temperature" | "combined_gas_final_volume" => {
            vec![pv_states(s)]
        }
        "consecutive_first_order" => vec![scheme(s), shares(s)],
        "first_order_concentration" => vec![kinetics(s)],
        "hydrogen_transition_wavelength" => vec![spectrum(s)],
        "ideal_gas_pressure" => vec![isotherms_p(s)],
        "ideal_gas_temperature" => vec![isotherms_t(s)],
        "ideal_gas_volume" => vec![isobars(s)],
        "mean_speed" => vec![speeds(s, "u_mean", "Mean molecular speed against temperature", "mean speed")],
        "rms_speed" => vec![speeds(s, "u_rms", "Root-mean-square speed against temperature", "rms speed")],
        "titration_ph_strong_acid_strong_base" => vec![titration_strong(s)],
        "titration_ph_weak_acid_strong_base" => vec![titration_weak(s)],
        "van_der_waals_pressure" => vec![real_vs_ideal(s)],
        "equilibrium_ice" => vec![ice_vectors(s)],
        "gibbs_free_energy" => {
            let (dh, t) = (s.get("dH").unwrap_or(f64::NAN), s.get("T").unwrap_or(f64::NAN));
            vec![c::around(dh, false).and_then(|xr| c::verdict_map(
                s, "Gibbs energy change (dG = dH - T dS): where the verdict changes", "spontaneity", "dH", xr, "T",
                (t / 10.0, t * 10.0), (false, false)))]
        }
        "reaction_enthalpy_from_formation" => vec![formation_sensitivity(s)],
        _ => st::live(s),
    };
    v.into_iter().flatten().collect()
}

/// A number input as the model reads it again (SI).
pub fn f(x: f64) -> Input {
    Input::Float(x)
}

// ------------------------------------------------------------------------------------------------ kinetics
fn arrhenius(s: &Solved) -> Option<Figure> {
    let (t0, k0) = (s.get("T")?, s.get("k")?);
    if !(t0 > 0.0) || !(k0 > 0.0) {
        return None;
    }
    let ts = k::span(t0 * 0.7, t0 * 1.3, 61, false);
    let pts: Vec<(f64, f64)> = c::curve(s, "T", &ts, "k").into_iter().filter(|p| p.1 > 0.0)
        .map(|(t, kk)| (1000.0 / t, kk)).collect();
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([k0]).collect();
    let mut ch = Chart::new(Chart::range(&pts.iter().map(|p| p.0).collect::<Vec<_>>()), Chart::range_pos(&ys),
                            (false, true), (false, true), "1000/T (inverse temperature, 1/K x 1000)",
                            "rate constant k", &format!("k = {} at T = {} K", tick(k0), tick(t0)));
    ch.line(&pts, "axp-c", "", false);
    ch.mark((1000.0 / t0, k0), &format!("k = {}", tick(k0)));
    let body = ch.body();
    Some(s.chart("arrhenius", "Arrhenius plot: rate constant against inverse temperature",
                 &format!("Arrhenius plot: the rate constant (logarithmic axis) against 1000/T, each point a run of the \
                           model at 61 temperatures from {} K to {} K, the other inputs as given: a straight line of \
                           slope -Ea/R. Marked: T = {} K, k = {}.", tick(ts[0]), tick(ts[60]), tick(t0), tick(k0)), &body))
}

fn kinetics(s: &Solved) -> Option<Figure> {
    let (a0, kk, t0, a) = (s.get("A0")?, s.get("k")?, s.get("t")?, s.get("A")?);
    if !(a0 > 0.0 && kk > 0.0 && t0 >= 0.0) {
        return None;
    }
    let tmax = (1.5 * t0).max(1.2 / kk);
    let ts = k::span(0.0, tmax, 61, false);
    let first = c::curve(s, "t", &ts, "A");
    // the same initial rate k A0 under a zero- and a second-order law, from the model's own capabilities
    let other = |key: &str, rate: f64| -> Vec<(f64, f64)> {
        ts.iter().filter_map(|t| {
            let a = s.solve_other(key, &[("A0", f(a0)), ("k", f(rate)), ("t", f(*t))])?;
            num_of(&a, "A").map(|v| (*t, v))
        }).collect()
    };
    let zero = other("zero_order_concentration", kk * a0);
    let second = other("second_order_concentration", kk / a0);
    if first.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = first.iter().chain(&zero).chain(&second).map(|p| p.1).chain([0.0, a0]).collect();
    let mut ch = Chart::new((0.0, tmax), Chart::range(&ys), (false, false), (false, true), "time t (s)",
                            "concentration [A] (mol/m^3)", &format!("[A] = {} mol/m^3 at t = {} s", tick(a), tick(t0)));
    ch.line(&zero, "axp-c4", "zero order", false);
    ch.line(&first, "axp-c", "first order", false);
    ch.line(&second, "axp-c3", "second order", false);
    ch.mark((t0, a), &tick(a));
    let body = ch.body();
    Some(s.chart("kinetics", "Concentration against time: zero, first and second order",
                 &format!("Reactant concentration against time from A0 = {} mol/m^3 for zero-, first- and second-order \
                           rate laws with the same initial rate ({} mol/(m^3 s); k = {} mol/(m^3 s), {} 1/s and {} \
                           m^3/(mol s)), each computed by the model at 61 times up to {} s. Marked: t = {} s, [A] = {} \
                           mol/m^3 (first order).", tick(a0), tick(kk * a0), tick(kk * a0), tick(kk), tick(kk / a0),
                          tick(tmax), tick(t0), tick(a)), &body))
}

fn scheme(s: &Solved) -> Option<Figure> {
    let (k1, k2, a0, t) = (s.get("k1")?, s.get("k2")?, s.get("A0")?, s.get("t")?);
    let (a, b, cc) = (s.out("A")?, s.out("B")?, s.out("C")?);
    let mut cv = k::Canvas::new(260.0);
    let xs = [70.0, 220.0, 370.0];
    let fills = ["axp-b", "axp-b2", "axp-b3"];
    for (i, (name, v)) in [("A", a), ("B", b), ("C", cc)].iter().enumerate() {
        cv.raw(&format!("<rect class=\"{}\" x=\"{}\" y=\"90\" width=\"80\" height=\"50\" rx=\"6\"/>", fills[i], xs[i]));
        cv.bold(xs[i] + 40.0, 121.0, "middle", name);
        cv.value(xs[i] + 40.0, 162.0, "middle", &format!("{} mol/m^3", tick(*v)));
    }
    cv.arrow((154.0, 115.0), (216.0, 115.0));
    cv.arrow((304.0, 115.0), (366.0, 115.0));
    cv.name(185.0, 80.0, "middle", &format!("k1 = {} 1/s", tick(k1)));
    cv.name(335.0, 80.0, "middle", &format!("k2 = {} 1/s", tick(k2)));
    cv.bold(260.0, 40.0, "middle", "A -> B -> C, two first-order steps");
    cv.name(260.0, 222.0, "middle", &format!("after t = {} s, from A0 = {} mol/m^3", tick(t), tick(a0)));
    Some(s.figure("scheme", "Reaction scheme A -> B -> C with its rate constants",
                  &format!("The consecutive first-order reaction A -> B -> C with k1 = {} 1/s and k2 = {} 1/s, starting \
                            from A0 = {} mol/m^3: the concentrations the model returns at t = {} s are A = {}, B = {} and \
                            C = {} mol/m^3.", tick(k1), tick(k2), tick(a0), tick(t), tick(a), tick(b), tick(cc)),
                  cv.body(), cv.h))
}

fn shares(s: &Solved) -> Option<Figure> {
    let (k1, k2, a0, t0) = (s.get("k1")?, s.get("k2")?, s.get("A0")?, s.get("t")?);
    if !(k1 > 0.0 && k2 > 0.0 && a0 > 0.0) {
        return None;
    }
    let tmax = (4.0 / k1.min(k2)).max(1.2 * t0);
    let ts = k::span(0.0, tmax, 80, false);
    let mut rows = vec![];
    for t in &ts {
        let Some(a) = s.with(&[("t", *t)]) else { continue };
        let (Some(fa), Some(fb), Some(fc)) = (num_of(&a, "A"), num_of(&a, "B"), num_of(&a, "C")) else { continue };
        rows.push((*t, fa / a0, fb / a0, fc / a0));
    }
    if rows.len() < 2 {
        return None;
    }
    let mut ch = Chart::new((0.0, tmax), (0.0, 1.0), (false, false), (false, false), "time t (s)",
                            "share of the starting amount", "");
    let z: Vec<(f64, f64)> = rows.iter().map(|r| (r.0, 0.0)).collect();
    let la: Vec<(f64, f64)> = rows.iter().map(|r| (r.0, r.1)).collect();
    let lb: Vec<(f64, f64)> = rows.iter().map(|r| (r.0, r.1 + r.2)).collect();
    let lc: Vec<(f64, f64)> = rows.iter().map(|r| (r.0, r.1 + r.2 + r.3)).collect();
    ch.area(&z, &la, "axp-b", "");
    ch.area(&la, &lb, "axp-b2", "");
    ch.area(&lb, &lc, "axp-b3", "");
    ch.vguide(t0, &format!("t = {} s", tick(t0)));
    let mut body = ch.body();
    body.push_str(&crate::figs_extra::fill_legend(&[("axp-b", "A"), ("axp-b2", "B"), ("axp-b3", "C")]));
    let peak = rows.iter().cloned().fold((0.0, 0.0), |m, r| if r.2 > m.1 { (r.0, r.2) } else { m });
    let last = rows[rows.len() - 1];
    Some(s.chart("shares", "Species shares over time, stacked",
                 &format!("How the starting {} mol/m^3 of A divides between A, B and C over time (stacked shares, k1 = {} \
                           1/s, k2 = {} 1/s), each concentration computed by the model at {} times. B is largest near t = \
                           {} s (a share of {}); at t = {} s the shares are {}, {} and {}.", tick(a0), tick(k1), tick(k2),
                          rows.len(), tick(peak.0), tick(peak.1), tick(last.0), tick(last.1), tick(last.2), tick(last.3)),
                 &body))
}

// ------------------------------------------------------------------------------------------------ gas laws
fn boyle(s: &Solved, pressure: bool) -> Option<Figure> {
    let (p1, v1, p2, v2) = (s.get("P1")?, s.get("V1")?, s.get("P2")?, s.get("V2")?);
    if !(p1 > 0.0 && v1 > 0.0 && p2 > 0.0 && v2 > 0.0) {
        return None;
    }
    // the isotherm through state 1: the model's own answer for many final states
    let pts: Vec<(f64, f64)> = if pressure {
        let vs = k::span(v1.min(v2) * 0.6, v1.max(v2) * 1.5, 61, false);
        c::curve(s, "V2", &vs, "P2")
    } else {
        let ps = k::span(p1.min(p2) * 0.6, p1.max(p2) * 1.6, 61, false);
        c::curve(s, "P2", &ps, "V2").into_iter().map(|(p, v)| (v, p)).collect()
    };
    let pts: Vec<(f64, f64)> = pts.into_iter().map(|(v, p)| (v * 1000.0, p / 1000.0)).collect();
    if pts.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = pts.iter().map(|p| p.0).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut ch = Chart::new(Chart::range(&xs), Chart::range(&ys), (false, false), (true, true), "volume V (L)",
                            "pressure P (kPa)", "");
    ch.line(&pts, "axp-c", "isotherm through state 1 (P V constant)", false);
    let (a, b) = ((v1 * 1000.0, p1 / 1000.0), (v2 * 1000.0, p2 / 1000.0));
    ch.arrow(a, b);
    ch.dots(&[a], "axp-d3", 4.5);
    ch.text(ch.xa.px(a.0) + 8.0, ch.ya.px(a.1) - 8.0, "start", "axp-lt", &format!("1: {} kPa, {} L", tick(a.1), tick(a.0)));
    let lab = if pressure { format!("P2 = {} kPa", tick(b.1)) } else { format!("V2 = {} L", tick(b.0)) };
    ch.mark(b, &lab);
    let body = ch.body();
    let (title, what) = if pressure {
        ("Boyle's law: compression along an isotherm", format!("the final pressure for 61 final volumes; arrow: the change to V2 = {} L. Marked: P2 = {} kPa", tick(b.0), tick(b.1)))
    } else {
        ("Boyle's law: the new volume on the isotherm", format!("the final volume for 61 final pressures; arrow: the change to P2 = {} kPa. Marked: V2 = {} L", tick(b.1), tick(b.0)))
    };
    Some(s.chart("isotherm", title,
                 &format!("Boyle's law on a P-V diagram: the isotherm through state 1 (P1 = {} kPa, V1 = {} L), each \
                           point the model's {what}.", tick(a.1), tick(a.0)), &body))
}

/// Charles's or Gay-Lussac's law: the state variable q (V or P) against absolute temperature through state 1.
fn state_line(s: &Solved, q: &str, word: &str, unit: &str, scale: f64) -> Option<Figure> {
    let (q1s, q2s) = (format!("{q}1"), format!("{q}2"));
    let (q1, t1, t2, q2) = (s.get(&q1s)?, s.get("T1")?, s.get("T2")?, s.get(&q2s)?);
    if !(t1 > 0.0 && t2 > 0.0) {
        return None;
    }
    let (lo, hi) = (t1.min(t2) * 0.65, t1.max(t2) * 1.25);
    let main: Vec<(f64, f64)> = c::curve(s, "T2", &k::span(lo, hi, 41, false), &q2s).into_iter()
        .map(|(t, v)| (t, v * scale)).collect();
    let toward: Vec<(f64, f64)> = c::curve(s, "T2", &k::span(lo * 0.02, lo, 12, false), &q2s).into_iter()
        .map(|(t, v)| (t, v * scale)).collect();
    if main.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = main.iter().chain(&toward).map(|p| p.1).chain([0.0]).collect();
    let mut ch = Chart::new((0.0, hi), Chart::range(&ys), (false, false), (false, true), "absolute temperature T (K)",
                            &format!("{word} {q} ({unit})"), "");
    ch.line(&toward, "axp-c4", "toward 0 K (the same law)", true);
    ch.line(&main, "axp-c", &format!("through {q}1 = {} {unit} at {} K", tick(q1 * scale), tick(t1)), false);
    let (a, b) = ((t1, q1 * scale), (t2, q2 * scale));
    ch.arrow(a, b);
    ch.dots(&[a], "axp-d3", 4.5);
    ch.mark(b, &format!("{q}2 = {} {unit}", tick(b.1)));
    let body = ch.body();
    let (title, law) = if q == "V" {
        ("Charles's law: volume proportional to absolute temperature", "Charles's law: volume against absolute temperature at constant pressure")
    } else {
        ("Gay-Lussac's law: pressure proportional to absolute temperature", "Gay-Lussac's law: pressure against absolute temperature at constant volume")
    };
    Some(s.chart("state-line", title,
                 &format!("{law} through state 1 ({q}1 = {} {unit}, T1 = {} K), the model's final {word} at 41 \
                           temperatures from {} K to {} K; dashed: the same law toward 0 K. Arrow: the change to T2 = {} \
                           K. Marked: {q}2 = {} {unit}.", tick(a.1), tick(t1), tick(lo), tick(hi), tick(t2), tick(b.1)),
                 &body))
}

fn pv_states(s: &Solved) -> Option<Figure> {
    let (p1, v1, t1) = (s.get("P1")?, s.get("V1")?, s.get("T1")?);
    let (p2, v2, t2) = (s.get("P2")?, s.get("V2")?, s.get("T2")?);
    if ![p1, v1, p2, v2].iter().all(|x| *x > 0.0) {
        return None;
    }
    let vs = k::span(v1.min(v2) * 0.55, v1.max(v2) * 1.6, 61, false);
    // each isotherm is P V constant through its state: the model's Boyle relation
    let iso = |p: f64, v: f64| -> Vec<(f64, f64)> {
        vs.iter().filter_map(|x| {
            let a = s.solve_other("boyle_final_pressure", &[("P1", f(p)), ("V1", f(v)), ("V2", f(*x))])?;
            num_of(&a, "P2").map(|y| (x * 1000.0, y / 1000.0))
        }).collect()
    };
    let (c1, c2) = (iso(p1, v1), iso(p2, v2));
    if c1.len() < 2 || c2.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = c1.iter().chain(&c2).map(|p| p.1).collect();
    let mut ch = Chart::new((vs[0] * 1000.0, vs[60] * 1000.0), Chart::range(&ys), (false, false), (false, true),
                            "volume V (L)", "pressure P (kPa)", "");
    ch.line(&c1, "axp-c", &format!("isotherm T1 = {} K", tick(t1)), false);
    ch.line(&c2, "axp-c2", &format!("isotherm T2 = {} K", tick(t2)), false);
    let (a, b) = ((v1 * 1000.0, p1 / 1000.0), (v2 * 1000.0, p2 / 1000.0));
    ch.arrow(a, b);
    ch.dots(&[a], "axp-d3", 4.5);
    ch.text(ch.xa.px(a.0) + 8.0, ch.ya.px(a.1) + 16.0, "start", "axp-lt", "1");
    let (lab, ret) = match s.key {
        "combined_gas_final_pressure" => (format!("P2 = {} kPa", tick(b.1)), format!("P2 = {} kPa", tick(b.1))),
        "combined_gas_final_temperature" => (format!("T2 = {} K", tick(t2)), format!("T2 = {} K", tick(t2))),
        _ => (format!("V2 = {} L", tick(b.0)), format!("V2 = {} L", tick(b.0))),
    };
    ch.mark(b, &lab);
    let body = ch.body();
    Some(s.chart("pv-states", "Combined gas law: two states, two isotherms",
                 &format!("The combined gas law on a P-V diagram: state 1 (P1 = {} kPa, V1 = {} L, T1 = {} K) lies on \
                           the isotherm T1 and state 2 (P2 = {} kPa, V2 = {} L) on the isotherm T2 = {} K; each isotherm \
                           is P V constant through its state, drawn with the model's Boyle relation at 61 volumes. \
                           Marked: the returned {ret}.", tick(a.1), tick(a.0), tick(t1), tick(b.1), tick(b.0), tick(t2)),
                 &body))
}

fn to_lkpa(v: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    v.into_iter().map(|(x, y)| (x * 1000.0, y / 1000.0)).collect()
}

fn isotherms_p(s: &Solved) -> Option<Figure> {
    let (n, v, t, p) = (s.get("n")?, s.get("V")?, s.get("T")?, s.get("P")?);
    if !(v > 0.0 && t > 0.0) {
        return None;
    }
    let vs = k::span(v * 0.4, v * 4.0, 61, false);
    let temps = [t * 2.0 / 3.0, t, t * 4.0 / 3.0];
    let fam = s.family("V", &vs, "P", "T", &temps, "K");
    if fam.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = fam.iter().flat_map(|f| f.1.iter().map(|p| p.1 / 1000.0)).collect();
    let mut ch = Chart::new((vs[0] * 1000.0, vs[60] * 1000.0), Chart::range(&ys), (false, false), (false, true),
                            "volume V (L)", "pressure P (kPa)", "");
    for (i, (name, pts)) in fam.iter().enumerate() {
        ch.line(&to_lkpa(pts.clone()), ["axp-c3", "axp-c", "axp-c2"][i % 3], name, false);
    }
    ch.mark((v * 1000.0, p / 1000.0), &format!("{} kPa", tick(p / 1000.0)));
    let body = ch.body();
    Some(s.chart("isotherms", "Ideal-gas isotherms on a P-V diagram",
                 &format!("Isotherms of {} mol of ideal gas: pressure against volume at T = {} K, each computed by the \
                           model at 61 volumes from {} L to {} L. Marked: V = {} L at {} K, P = {} kPa.", tick(n),
                          temps.iter().map(|x| tick(*x)).collect::<Vec<_>>().join(", "), tick(vs[0] * 1000.0),
                          tick(vs[60] * 1000.0), tick(v * 1000.0), tick(t), tick(p / 1000.0)), &body))
}

fn isotherms_t(s: &Solved) -> Option<Figure> {
    let (p, v, n, t) = (s.get("P")?, s.get("V")?, s.get("n")?, s.get("T")?);
    if !(v > 0.0 && t > 0.0) {
        return None;
    }
    let vs = k::span(v * 0.2, v * 4.0, 61, false);
    let mut series = vec![];
    for (i, tt) in [t / 2.0, t, 2.0 * t].iter().enumerate() {
        let pts: Vec<(f64, f64)> = vs.iter().filter_map(|x| {
            let a = s.solve_other("ideal_gas_pressure", &[("n", f(n)), ("V", f(*x)), ("T", f(*tt))])?;
            num_of(&a, "P").map(|y| (x * 1000.0, y / 1000.0))
        }).collect();
        if pts.len() >= 2 {
            let name = if i == 1 { format!("isotherm {} K (returned T)", tick(*tt)) } else { format!("isotherm {} K", tick(*tt)) };
            series.push((name, pts, ["axp-c3", "axp-c", "axp-c2"][i]));
        }
    }
    if series.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = series.iter().flat_map(|f| f.1.iter().map(|p| p.1)).collect();
    let mut ch = Chart::new((vs[0] * 1000.0, vs[60] * 1000.0), Chart::range_pos(&ys), (false, true), (false, true),
                            "volume V (L)", "pressure P (kPa)", "");
    for (name, pts, cl) in &series {
        ch.line(pts, cl, name, false);
    }
    ch.mark((v * 1000.0, p / 1000.0), &format!("T = {} K", tick(t)));
    let body = ch.body();
    Some(s.chart("isotherms", "Ideal-gas temperature: the isotherm through a state",
                 &format!("Which isotherm passes through the state P = {} kPa, V = {} L of {} mol? The model returns T = \
                           {} K; drawn (logarithmic pressure axis): that isotherm and those at half and twice the \
                           temperature, each from the model's ideal-gas pressure at 61 volumes from {} L to {} L. Marked: \
                           the state, T = {} K.", tick(p / 1000.0), tick(v * 1000.0), tick(n), tick(t), tick(vs[0] * 1000.0),
                          tick(vs[60] * 1000.0), tick(t)), &body))
}

fn isobars(s: &Solved) -> Option<Figure> {
    let (n, p, t, v) = (s.get("n")?, s.get("P")?, s.get("T")?, s.get("V")?);
    if !(p > 0.0 && t > 0.0) {
        return None;
    }
    let tmax = (1.8 * t).max(500.0);
    let ts = k::span(tmax / 100.0, tmax, 51, false);
    let fam = s.family("T", &ts, "V", "P", &[p / 2.0, p, 2.0 * p], "Pa");
    if fam.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = fam.iter().flat_map(|f| f.1.iter().map(|q| q.1 * 1000.0)).chain([0.0]).collect();
    let mut ch = Chart::new((0.0, tmax), Chart::range(&ys), (false, false), (false, true), "temperature T (K)",
                            "volume V (L)", "");
    for (i, (name, pts)) in fam.iter().enumerate() {
        let pts: Vec<(f64, f64)> = pts.iter().map(|q| (q.0, q.1 * 1000.0)).collect();
        ch.line(&pts, ["axp-c2", "axp-c", "axp-c3"][i % 3], name, false);
    }
    ch.mark((t, v * 1000.0), &format!("{} L", tick(v * 1000.0)));
    let body = ch.body();
    Some(s.chart("isobars", "Ideal-gas isobars: volume against temperature",
                 &format!("Volume of {} mol of ideal gas against temperature (isobars) at half, once and twice P = {} \
                           kPa, computed by the model at 51 temperatures up to {} K: straight lines through 0 K. Marked: \
                           T = {} K, P = {} kPa, V = {} L.", tick(n), tick(p / 1000.0), tick(tmax), tick(t),
                          tick(p / 1000.0), tick(v * 1000.0)), &body))
}

fn speeds(s: &Solved, out: &str, title: &str, word: &str) -> Option<Figure> {
    let (gas, t, u) = (s.txt("formula")?, s.get("T")?, s.get(out)?);
    let tmax = (1.5 * t).max(1000.0);
    let ts = k::span(tmax / 10.0, tmax, 46, false);
    let mut gases: Vec<String> = ["H2", "He", "N2", "CO2"].iter().map(|x| x.to_string()).collect();
    if !gases.iter().any(|g| *g == gas.trim()) {
        gases.push(gas.trim().to_string());
    }
    let mut ch_series = vec![];
    let mut at = vec![];
    for g in &gases {
        let pts: Vec<(f64, f64)> = ts.iter().filter_map(|x| {
            let a = s.with_input(&[("formula", Input::Text(g.clone())), ("T", f(*x))])?;
            num_of(&a, out).map(|y| (*x, y))
        }).collect();
        if pts.len() >= 2 {
            if let Some(a) = s.with_input(&[("formula", Input::Text(g.clone()))]) {
                if let Some(y) = num_of(&a, out) {
                    at.push(format!("{} {}", sub(g), tick(y)));
                }
            }
            ch_series.push((g.clone(), pts));
        }
    }
    if ch_series.is_empty() {
        return None;
    }
    let ys: Vec<f64> = ch_series.iter().flat_map(|x| x.1.iter().map(|p| p.1)).chain([0.0, u]).collect();
    let mut ch = Chart::new((0.0, tmax), Chart::range(&ys), (false, false), (false, true), "temperature T (K)",
                            &format!("{word} (m/s)"), "");
    for (i, (g, pts)) in ch_series.iter().enumerate() {
        ch.line(pts, ["axp-c", "axp-c2", "axp-c3", "axp-c4"][i % 4], &sub(g), false);
    }
    ch.mark((t, u), &format!("{} {} m/s", sub(gas.trim()), tick(u)));
    let body = ch.body();
    Some(s.chart("by-gas", title,
                 &format!("The {word} of gas molecules against temperature for {} gases, each computed by the model at 46 \
                           temperatures from {} K to {} K (it grows as the square root of T/M). At {} K: {} m/s. Marked: \
                           {} at {} K, {} m/s.", ch_series.len(), tick(ts[0]), tick(tmax), tick(t), at.join(", "),
                          sub(gas.trim()), tick(t), tick(u)), &body))
}

fn real_vs_ideal(s: &Solved) -> Option<Figure> {
    let (n, v, t, p, pi) = (s.get("n")?, s.get("V")?, s.get("T")?, s.get("P")?, s.get("P_ideal")?);
    if !(v > 0.0) {
        return None;
    }
    let vs = k::span(v / 40.0, v * 4.0, 71, true);
    let (mut real, mut ideal) = (vec![], vec![]);
    for x in &vs {
        let Some(a) = s.with(&[("V", *x)]) else { continue };
        if let Some(y) = num_of(&a, "P") {
            real.push((x * 1000.0, y / 1e5));
        }
        if let Some(y) = num_of(&a, "P_ideal") {
            ideal.push((x * 1000.0, y / 1e5));
        }
    }
    if real.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = real.iter().chain(&ideal).map(|q| q.1).collect();
    let mut ch = Chart::new((vs[0] * 1000.0, vs[70] * 1000.0), Chart::range_pos(&ys), (true, true), (false, true),
                            "volume V (L)", "pressure P (bar)", "");
    ch.line(&ideal, "axp-c4", "ideal gas", true);
    ch.line(&real, "axp-c", "van der Waals", false);
    ch.mark((v * 1000.0, p / 1e5), &format!("{} bar", tick(p / 1e5)));
    let body = ch.body();
    Some(s.chart("real-vs-ideal", "Real gas against ideal gas: van der Waals isotherm",
                 &format!("Pressure of {} mol of gas at {} K against volume, van der Waals and ideal, both returned by \
                           the model at 71 volumes from {} L to {} L (logarithmic axes; a non-positive pressure is left \
                           out). At large volume the two agree. Marked: V = {} L, P = {} bar (ideal {} bar).", tick(n),
                          tick(t), tick(vs[0] * 1000.0), tick(vs[70] * 1000.0), tick(v * 1000.0), tick(p / 1e5),
                          tick(pi / 1e5)), &body))
}

// ------------------------------------------------------------------------------------------------ acids and spectra
fn titration_strong(s: &Solved) -> Option<Figure> {
    let (ca, va, cb, vb, ph) = (s.get("c_a")?, s.get("V_a")?, s.get("c_b")?, s.get("V_b")?, s.get("pH")?);
    let eq = s.solve_other("titration_equivalence_volume",
                           &[("c_t", f(cb)), ("V_a", f(va)), ("c_a", f(ca)), ("ratio", Input::Int(1))])?;
    let veq = num_of(&eq, "V_eq")?;
    if !(veq > 0.0) {
        return None;
    }
    let vmax = (2.0 * veq).max(1.2 * vb);
    let pts: Vec<(f64, f64)> = c::curve(s, "V_b", &k::span(0.0, vmax, 121, false), "pH").into_iter()
        .map(|(x, y)| (x * 1e6, y)).collect();
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([0.0, 14.0]).collect();
    let mut ch = Chart::new((0.0, vmax * 1e6), Chart::range(&ys), (false, false), (false, false),
                            "base added V_b (mL)", "pH", "");
    ch.vguide(veq * 1e6, &format!("equivalence {} mL", tick(veq * 1e6)));
    ch.hguide(7.0, "pH 7");
    ch.line(&pts, "axp-c", "", false);
    ch.mark((vb * 1e6, ph), &format!("pH {}", tick(ph)));
    let body = ch.body();
    Some(s.chart("titration", "Titration curve with its equivalence point",
                 &format!("Titration curve of a strong acid ({} mol/m^3, {} mL) with a strong base ({} mol/m^3): the pH \
                           the model returns for 121 added volumes from 0 to {} mL; the equivalence volume {} mL is the \
                           model's own equivalence-volume capability. Marked: V_b = {} mL, pH = {}.", tick(ca),
                          tick(va * 1e6), tick(cb), tick(vmax * 1e6), tick(veq * 1e6), tick(vb * 1e6), tick(ph)), &body))
}

fn titration_weak(s: &Solved) -> Option<Figure> {
    let (ka, ca, va, cb, vb, ph) = (s.get("Ka")?, s.get("c_a")?, s.get("V_a")?, s.get("c_b")?, s.get("V_b")?,
                                    s.get("pH")?);
    let eq = s.solve_other("titration_equivalence_volume",
                           &[("c_t", f(cb)), ("V_a", f(va)), ("c_a", f(ca)), ("ratio", Input::Int(1))])?;
    let veq = num_of(&eq, "V_eq")?;
    if !(veq > 0.0) {
        return None;
    }
    let vmax = (2.0 * veq).max(1.2 * vb);
    let xs = k::span(0.0, vmax, 101, false);
    let weak: Vec<(f64, f64)> = c::curve(s, "V_b", &xs, "pH").into_iter().map(|(x, y)| (x * 1e6, y)).collect();
    let strong: Vec<(f64, f64)> = xs.iter().filter_map(|x| {
        let a = s.solve_other("titration_ph_strong_acid_strong_base",
                              &[("c_a", f(ca)), ("V_a", f(va)), ("c_b", f(cb)), ("V_b", f(*x))])?;
        num_of(&a, "pH").map(|y| (x * 1e6, y))
    }).collect();
    if weak.len() < 2 {
        return None;
    }
    let at = |x: f64| s.with(&[("V_b", x)]).and_then(|a| num_of(&a, "pH"));
    let (p10, p90, peq) = (at(0.1 * veq), at(0.9 * veq), at(veq));
    let ys: Vec<f64> = weak.iter().chain(&strong).map(|p| p.1).chain([0.0, 14.0]).collect();
    let mut ch = Chart::new((0.0, vmax * 1e6), Chart::range(&ys), (false, false), (false, false), "base added V_b (mL)",
                            "pH", "");
    ch.xband(0.1 * veq * 1e6, 0.9 * veq * 1e6, "axp-s2");
    ch.vguide(veq * 1e6, &format!("equivalence {} mL", tick(veq * 1e6)));
    ch.line(&strong, "axp-c3", "strong acid", false);
    ch.line(&weak, "axp-c", &format!("weak acid, Ka = {}", tick(ka)), false);
    if let Some(pe) = peq {
        ch.dots(&[(veq * 1e6, pe)], "axp-d2", 4.5);
    }
    ch.mark((vb * 1e6, ph), &format!("pH {}", tick(ph)));
    let body = ch.body();
    let band = match (p10, p90) {
        (Some(a), Some(b)) => format!(" Shaded: the buffer region, 10 % to 90 % neutralised (pH {} to {}).", tick(a), tick(b)),
        _ => String::new(),
    };
    let eqs = peq.map(|p| format!(" At equivalence ({} mL) pH = {}.", tick(veq * 1e6), tick(p))).unwrap_or_default();
    Some(s.chart("titration", "Titration curve with the buffer region",
                 &format!("pH against volume of {} mol/m^3 base added to {} mL of {} mol/m^3 acid, computed by the model \
                           at 101 volumes for the weak acid (Ka = {}) and for a strong acid of the same concentration.{band}\
                           {eqs} Marked: V_b = {} mL, pH = {}.", tick(cb), tick(va * 1e6), tick(ca), tick(ka),
                          tick(vb * 1e6), tick(ph)), &body))
}

fn spectrum(s: &Solved) -> Option<Figure> {
    let (nu, nl) = (s.val("n_upper").and_then(as_int)?, s.val("n_lower").and_then(as_int)?);
    let lam0 = s.out("lam_nm")?;
    let e0 = s.out("E")? / 1.602176634e-19;
    let mut series: Vec<i64> = vec![1, 2, 3];
    if !series.contains(&nl) {
        series.push(nl);
    }
    let names = ["Lyman", "Balmer", "Paschen", "Brackett", "Pfund"];
    let mut lines: Vec<(i64, Vec<(f64, f64)>)> = vec![];
    for l in &series {
        let mut pts = vec![];
        for u in (l + 1)..=(l + 6) {
            let Some(a) = s.solve_other(s.key, &[("n_upper", Input::Int(u)), ("n_lower", Input::Int(*l))]) else { continue };
            if let (Some(lam), Some(e)) = (num_of(&a, "lam_nm"), num_of(&a, "E")) {
                pts.push((lam, e / 1.602176634e-19));
            }
        }
        if !pts.is_empty() {
            lines.push((*l, pts));
        }
    }
    let xs: Vec<f64> = lines.iter().flat_map(|l| l.1.iter().map(|p| p.0)).chain([lam0]).collect();
    let ys: Vec<f64> = lines.iter().flat_map(|l| l.1.iter().map(|p| p.1)).chain([0.0, e0]).collect();
    let (_, xhi) = Chart::range(&xs);
    let mut ch = Chart::new((0.0, xhi * 1.05), Chart::range(&ys), (false, false), (false, true), "wavelength (nm)",
                            "photon energy (eV)", "");
    let cls = [("axp-c3", "axp-d3"), ("axp-c", "axp-d"), ("axp-c2", "axp-d2"), ("axp-c4", "axp-d4")];
    let mut ranges = vec![];
    for (i, (l, pts)) in lines.iter().enumerate() {
        let (lc, dc) = cls[i % 4];
        for p in pts {
            ch.stem(p.0, p.1, lc, dc);
        }
        let nm = names.get((*l - 1) as usize).map(|x| x.to_string()).unwrap_or_else(|| format!("series"));
        ch.legend.push((lc.to_string(), format!("{nm} (to n = {l})")));
        ranges.push(format!("{nm} {} to {} nm", tick(pts[0].0), tick(pts[pts.len() - 1].0)));
    }
    ch.mark((lam0, e0), &format!("{} nm", tick(lam0)));
    let body = ch.body();
    Some(s.chart("spectrum", "Hydrogen line spectrum: the series of lines",
                 &format!("The hydrogen line spectrum: the wavelength of the first six lines of each series, computed by \
                           the model, each drawn at its photon energy (also computed by the model, in eV). Ranges: {}. \
                           Marked: n = {nu} -> {nl}, wavelength {} nm, E = {} eV.", ranges.join("; "), tick(lam0),
                          tick(e0)), &body))
}

fn as_int(v: ax_engine::Val) -> Option<i64> {
    match v {
        ax_engine::Val::Int(i) => Some(i),
        ax_engine::Val::Num(x) if x.fract() == 0.0 => Some(x as i64),
        _ => None,
    }
}

fn ice_vectors(s: &Solved) -> Option<Figure> {
    let init = s.vec("initial")?;
    let eqv = s.out_list("equilibrium")?;
    let x = s.out("x")?;
    if init.len() != 2 || eqv.len() != 2 {
        return None;
    }
    let body = k::arrows(&[("initial".to_string(), (init[0], init[1]), false),
                           ("equilibrium".to_string(), (eqv[0], eqv[1]), true)],
                         &format!("x = {}", tick(x)));
    Some(s.chart("vectors", "Equilibrium concentrations (ICE table): the vectors",
                 &format!("The concentration vectors as arrows from the origin: the initial concentrations ({}, {}) as \
                           given, and the equilibrium concentrations the model returns ({}, {}), with the extent x = {} \
                           mol/L.", tick(init[0]), tick(init[1]), tick(eqv[0]), tick(eqv[1]), tick(x)), &body))
}

/// The reaction enthalpy as the last enthalpy of formation moves, the others as given (41 runs of the model).
fn formation_sensitivity(s: &Solved) -> Option<Figure> {
    let vals = s.vec("values")?;
    let dh = s.out("dH_kJ")?;
    let i = vals.len().checked_sub(1)?;
    let v0 = vals[i];
    let d = v0.abs().max(100.0);
    let xs = k::span(v0 - d, v0 + d, 41, false);
    let pts: Vec<(f64, f64)> = xs.iter().filter_map(|x| {
        let mut v = vals.clone();
        v[i] = *x;
        let a = s.with_input(&[("values", Input::Json(serde_json::json!(v)))])?;
        num_of(&a, "dH_kJ").map(|y| (*x, y))
    }).collect();
    if pts.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain([dh]).collect();
    let mut ch = Chart::new((xs[0], xs[40]), Chart::range(&ys), (false, false), (false, true),
                            &format!("value {} of values (kJ/mol)", i + 1), "reaction enthalpy dH (kJ/mol)",
                            &format!("dH = {} kJ/mol", tick(dh)));
    ch.line(&pts, "axp-c", "", false);
    ch.mark((v0, dh), &format!("values[{}] = {}", i + 1, tick(v0)));
    let body = ch.body();
    Some(s.chart("sensitivity", "Reaction enthalpy from enthalpies of formation: one value moved",
                 &format!("dH (standard reaction enthalpy, kJ/mol) as value {} of values (enthalpy of formation of each \
                           species in the order written) moves from {} to {}, the other values as given ({}); 41 runs of \
                           the model. Equation: {}. Marked: the values given, dH = {} kJ/mol.", i + 1, tick(xs[0]),
                          tick(xs[40]), vals.iter().map(|v| tick(*v)).collect::<Vec<_>>().join(", "),
                          s.txt("equation")?.trim(), tick(dh)), &body))
}

/// Bars helper re-exported for the other figure file.
pub fn bars(labels: &[String], vals: &[f64], classes: &[&str], tops: Option<&[String]>, ylab: &str, header: &str,
            log: bool, split: Option<usize>) -> Option<String> {
    bars_full(labels, vals, classes, tops, ylab, header, log, split)
}
