//! Chemistry's figures of formulas and amounts: balanced equations atom by atom, compositions, substance-by-substance
//! and gas-by-gas comparisons (each bar a run of the model), masses before and after a reaction, yields.

use crate::figs_common::{tick, Chart};
use crate::figs_extra::sub;
use crate::figures::{bars, f};
use ax_engine::figkit::{self as k, num_of, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

const SUBSTANCES: &[&str] = &["H2O", "NH3", "CH4", "CO2", "NaCl", "H2SO4", "CaCO3", "C6H12O6"];
const GASES: &[&str] = &["H2", "He", "CH4", "NH3", "N2", "O2", "Ar", "CO2", "SO2"];
const MOLECULES: &[&str] = &["C2H6", "C2H4", "C2H2", "C6H12", "C6H6", "C5H5N", "C10H8", "C2H3Cl"];
/// Compounds offered when comparing the content of one element (those containing it are kept).
const COMPOUNDS: &[&str] = &["H2O", "CO2", "O2", "O3", "H2SO4", "CaCO3", "C6H12O6", "KMnO4", "SiO2", "FeO", "Fe2O3",
                             "Fe3O4", "FeS2", "FeCO3", "FeSO4", "CH4", "NH3", "NaCl", "HCl", "NaOH", "CuSO4", "CuO",
                             "Al2O3", "MgO", "CaO", "ZnO", "KCl", "CaCl2", "N2O", "HNO3", "H3PO4", "C2H5OH"];

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "atoms_of_element" => vec![element_in(s, "by-substance", "N_atoms", 1.0, "atoms")],
        "mass_of_element" => vec![element_in(s, "by-compound", "m_element", 1000.0, "mass")],
        "mass_percent" => vec![composition(s), element_in(s, "oxygen-share", "percent", 1.0, "percent")],
        "balance_equation" => vec![atom_balance(s), coefficients(s)],
        "dalton_mixture" => vec![dalton(s)],
        "degree_of_unsaturation" => vec![by_items(s, "by-molecule", "Rings plus pi bonds from the formula", "formula",
                                                  MOLECULES, "dou", 1.0, "degree of unsaturation (rings + pi bonds)",
                                                  "degree of unsaturation", false)],
        "empirical_formula" => vec![mass_to_atoms(s)],
        "gas_density" => vec![by_items(s, "by-gas", "Gas density, gas by gas", "formula", GASES, "rho", 1.0,
                                       "density (kg/m^3)", "density", false)],
        "graham_rate_ratio" => vec![graham(s)],
        "ideal_gas_mass" => vec![by_items(s, "by-gas", "Mass of the same gas volume, gas by gas", "formula", GASES, "m",
                                          1000.0, "mass (g)", "mass", false)],
        "particles_from_mass" => vec![by_items(s, "by-gas", "Molecules in the same mass of each gas", "formula", GASES,
                                               "N", 1.0, "molecules (logarithmic)", "number of molecules", true)],
        "mass_from_moles" => vec![by_items(s, "by-substance", "Mass of the same amount of each substance", "formula",
                                           SUBSTANCES, "m", 1000.0, "mass (g)", "mass", false)],
        "moles_from_mass" => vec![by_items(s, "by-substance", "Moles in the same mass of each substance", "formula",
                                           SUBSTANCES, "n", 1.0, "amount (mol)", "amount of substance", false)],
        "molar_mass" => vec![by_items(s, "by-substance", "Molar masses of common substances", "formula", SUBSTANCES,
                                      "M", 1000.0, "molar mass (g/mol)", "molar mass", false), molar_composition(s)],
        "limiting_reagent" => {
            let (ma, mb) = (s.get("m_a").unwrap_or(f64::NAN), s.get("m_b").unwrap_or(f64::NAN));
            vec![before_after(s), (ma > 0.0 && mb > 0.0).then_some(()).and_then(|_| crate::figs_common::verdict_map(
                s, "Limiting reagent: where the verdict changes", "limiting", "m_a", (ma / 10.0, ma * 10.0), "m_b",
                (mb / 10.0, mb * 10.0), (false, false)))]
        }
        "molecular_formula" => vec![multiples(s)],
        "partial_pressure" => vec![air(s)],
        "percent_yield" => vec![yield_bars(s)],
        "theoretical_yield" => vec![products(s)],
        _ => vec![],
    }
}

/// The element symbols of a formula in their order (H2SO4 -> H, S, O); None when the text is not a formula.
pub fn elements(formula: &str) -> Option<Vec<String>> {
    let cs: Vec<char> = formula.trim().chars().collect();
    let mut out: Vec<String> = vec![];
    let mut i = 0;
    while i < cs.len() {
        let ch = cs[i];
        if ch.is_ascii_uppercase() {
            let mut e = ch.to_string();
            if i + 1 < cs.len() && cs[i + 1].is_ascii_lowercase() {
                e.push(cs[i + 1]);
                i += 1;
            }
            if !out.contains(&e) {
                out.push(e);
            }
        } else if !(ch.is_ascii_digit() || ch == '(' || ch == ')' || ch == '.') {
            return None;
        }
        i += 1;
    }
    (!out.is_empty()).then_some(out)
}

/// The species of an equation as written ("C3H8 + O2 -> CO2 + H2O"): reactants and products, coefficients dropped.
pub fn species(eq: &str) -> Option<(Vec<String>, Vec<String>)> {
    let e = eq.replace('→', "->").replace('⇌', "->").replace("<->", "->").replace('=', "->");
    let mut sides = e.split("->");
    let (l, r) = (sides.next()?, sides.next()?);
    if sides.next().is_some() {
        return None;
    }
    let side = |t: &str| -> Option<Vec<String>> {
        let v: Vec<String> = t.split('+').map(|x| {
            let x = x.trim();
            let x = x.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' ');
            x.trim().to_string()
        }).collect();
        (v.iter().all(|x| elements(x).is_some())).then_some(v)
    };
    Some((side(l)?, side(r)?))
}

/// One output for each of a list of substances (the visitor's own added and highlighted), one run each.
#[allow(clippy::too_many_arguments)]
fn by_items(s: &Solved, kind: &str, title: &str, vary: &str, items: &[&str], out: &str, scale: f64, ylab: &str,
            word: &str, log: bool) -> Option<Figure> {
    let mine = s.txt(vary)?.trim().to_string();
    let mut list: Vec<String> = items.iter().map(|x| x.to_string()).collect();
    if !list.contains(&mine) {
        list.push(mine.clone());
    }
    let (mut labels, mut vals, mut cls, mut parts) = (vec![], vec![], vec![], vec![]);
    for it in &list {
        let Some(a) = s.with_input(&[(vary, Input::Text(it.clone()))]) else { continue };
        let Some(v) = num_of(&a, out) else { continue };
        let v = v * scale;
        labels.push(sub(it));
        vals.push(v);
        cls.push(if *it == mine { "axp-b2" } else { "axp-b" });
        parts.push(format!("{} {}", sub(it), tick(v)));
    }
    if labels.len() < 2 || !cls.contains(&"axp-b2") {
        return None;
    }
    let body = bars(&labels, &vals, &cls, None, ylab, "", log, None)?;
    let others: Vec<String> = s.given.iter().filter(|g| g.0 != vary).map(|g| format!("{} = {}", g.0, show(&g.1))).collect();
    Some(s.chart(kind, title,
                 &format!("The {word} for {} substances, each returned by the model{}: {}. Highlighted: the formula \
                           given, {}.", labels.len(),
                          if others.is_empty() { String::new() } else { format!(" with {}", others.join(", ")) },
                          parts.join(", "), sub(&mine)), &body))
}

fn show(i: &Input) -> String {
    match i {
        Input::Text(t) => t.clone(),
        Input::Int(n) => n.to_string(),
        Input::Float(x) => tick(*x),
        Input::Json(v) => v.to_string(),
    }
}

/// The content of the asked element in compounds that contain it (the visitor's compound highlighted).
fn element_in(s: &Solved, kind: &str, out: &str, scale: f64, what: &str) -> Option<Figure> {
    let el = s.txt("element")?.trim().to_string();
    let mine = s.txt("formula")?.trim().to_string();
    let mut list: Vec<String> = vec![mine.clone()];
    for c in COMPOUNDS {
        if list.len() >= 8 {
            break;
        }
        // compounds only (an element alone, O2 or O3, is 100 % of itself)
        if *c != mine && elements(c).map(|e| e.contains(&el) && e.len() > 1).unwrap_or(false) {
            list.push(c.to_string());
        }
    }
    let (mut labels, mut vals, mut cls, mut parts) = (vec![], vec![], vec![], vec![]);
    for it in &list {
        let Some(a) = s.with_input(&[("formula", Input::Text(it.clone()))]) else { continue };
        let Some(v) = num_of(&a, out) else { continue };
        let v = v * scale;
        labels.push(sub(it));
        vals.push(v);
        cls.push(if *it == mine { "axp-b2" } else { "axp-b" });
        parts.push(format!("{} {}", sub(it), tick(v)));
    }
    if labels.len() < 2 || cls[0] != "axp-b2" {
        return None;
    }
    let (ylab, title, cap) = match what {
        "atoms" => (format!("{el} atoms"), format!("{el} atoms in the same mass of each substance"),
                    format!("Number of {el} atoms in {} of each substance", k::q(s.get("m")?, "kg"))),
        "mass" => (format!("mass of {el} (g)"), format!("{el} content of compounds containing it"),
                   format!("Mass of {el} contained in {} of each compound", k::q(s.get("m")?, "kg"))),
        _ => (format!("{el} (% by mass)"), format!("Percent {el} by mass, compound by compound"),
              format!("Percent {el} by mass in compounds containing it")),
    };
    let body = bars(&labels, &vals, &cls, None, &ylab, "", false, None)?;
    Some(s.chart(kind, &title, &format!("{cap}, each returned by the model: {}. Highlighted: the formula given, {}.",
                                        parts.join(", "), sub(&mine)), &body))
}

fn composition(s: &Solved) -> Option<Figure> {
    let el = s.txt("element")?.trim().to_string();
    let formula = s.txt("formula")?.trim().to_string();
    let comp = crate::figs_common::out_map(s, "composition")?;
    let labels: Vec<String> = comp.iter().map(|c| c.0.clone()).collect();
    let vals: Vec<f64> = comp.iter().map(|c| c.1).collect();
    let cls: Vec<&str> = comp.iter().map(|c| if c.0 == el { "axp-b2" } else { "axp-b" }).collect();
    let tops: Vec<String> = vals.iter().map(|v| format!("{} %", tick(*v))).collect();
    let body = bars(&labels, &vals, &cls, Some(&tops), "mass percent (%)", "", false, None)?;
    Some(s.chart("composition", &format!("Mass composition of {}", sub(&formula)),
                 &format!("Mass composition of {}: the percent by mass of every element, returned by the model ({}). \
                           Highlighted: the element asked, {el}.", sub(&formula),
                          comp.iter().map(|c| format!("{} {} %", c.0, tick(c.1))).collect::<Vec<_>>().join(", ")),
                 &body))
}

fn molar_composition(s: &Solved) -> Option<Figure> {
    let formula = s.txt("formula")?.trim().to_string();
    let m = s.out("M_g")?;
    let els = elements(&formula)?;
    let a = s.solve_other("mass_percent", &[("formula", Input::Text(formula.clone())), ("element", Input::Text(els[0].clone()))])?;
    let comp: Vec<(String, f64)> = a.values.iter().find(|v| v.0 == "composition")?.1.as_object()?
        .iter().filter_map(|(k, v)| v.as_f64().map(|x| (k.clone(), x))).collect();
    if comp.is_empty() {
        return None;
    }
    let mut labels: Vec<String> = comp.iter().map(|c| c.0.clone()).collect();
    let mut vals: Vec<f64> = comp.iter().map(|c| c.1 / 100.0 * m).collect();
    let mut cls: Vec<&str> = vec!["axp-b"; vals.len()];
    labels.push("total M".into());
    vals.push(m);
    cls.push("axp-b2");
    let tops: Vec<String> = vals.iter().map(|v| format!("{} g/mol", tick(*v))).collect();
    let body = bars(&labels, &vals, &cls, Some(&tops), "contribution to M (g/mol)", &format!("M = {} g/mol", tick(m)),
                    false, None)?;
    Some(s.chart("composition", &format!("Molar mass of {}, element by element", sub(&formula)),
                 &format!("Where the molar mass of {} comes from: each bar is one element's share, its mass percent \
                           (returned by the model) times the molar mass M = {} g/mol (returned by the model); the shares \
                           add up to M: {}.", sub(&formula), tick(m),
                          labels.iter().zip(&vals).take(comp.len()).map(|(l, v)| format!("{l} {} g/mol", tick(*v)))
                              .collect::<Vec<_>>().join(", ")), &body))
}

/// The number of atoms of an element in one formula unit, from the model's own counts in a small mass.
fn subscript(s: &Solved, formula: &str, el: &str) -> Option<f64> {
    let m = 0.001;
    let n = s.solve_other("particles_from_mass", &[("formula", Input::Text(formula.into())), ("m", f(m))])?;
    let units = num_of(&n, "N")?;
    let a = s.solve_other("atoms_of_element", &[("formula", Input::Text(formula.into())),
                                                ("element", Input::Text(el.into())), ("m", f(m))]);
    let atoms = match a {
        Some(a) => num_of(&a, "N_atoms")?,
        None => return elements(formula).map(|e| !e.contains(&el.to_string())).unwrap_or(false).then_some(0.0),
    };
    let r = atoms / units;
    Some(if (r - r.round()).abs() < 1e-6 { r.round() } else { r })
}

fn atom_balance(s: &Solved) -> Option<Figure> {
    let (re, pr) = species(&s.txt("equation")?)?;
    let co = s.out_list("coefficients")?;
    if co.len() != re.len() + pr.len() {
        return None;
    }
    let mut els: Vec<String> = vec![];
    for sp in re.iter().chain(&pr) {
        for e in elements(sp)? {
            if !els.contains(&e) {
                els.push(e);
            }
        }
    }
    if els.len() > 10 {
        return None;
    }
    let (mut left, mut right) = (vec![], vec![]);
    for e in &els {
        let mut l = 0.0;
        let mut r = 0.0;
        for (i, sp) in re.iter().enumerate() {
            l += co[i] * subscript(s, sp, e)?;
        }
        for (j, sp) in pr.iter().enumerate() {
            r += co[re.len() + j] * subscript(s, sp, e)?;
        }
        left.push(Some(l));
        right.push(Some(r));
    }
    let balanced = s.out_text("balanced")?;
    let body = k::bar_groups(&els, &[("reactants".to_string(), left.clone()), ("products".to_string(), right.clone())],
                             "element", "atoms on each side", &sub(&balanced));
    Some(s.chart("atom-balance", "Balanced equation: atoms of every element on both sides",
                 &format!("Atom balance of {}: for each element, the atoms on the reactant and on the product side, each \
                           the returned coefficient times the element's atoms in one formula unit (counted by the model) \
                           : {}.", sub(&balanced),
                          els.iter().zip(left.iter().zip(&right)).map(|(e, (l, r))| format!("{e} {} = {}",
                              tick(l.unwrap_or(0.0)), tick(r.unwrap_or(0.0)))).collect::<Vec<_>>().join("; ")), &body))
}

fn coefficients(s: &Solved) -> Option<Figure> {
    let eq = s.txt("equation")?;
    let (re, pr) = species(&eq)?;
    let co = s.out_list("coefficients")?;
    if co.len() != re.len() + pr.len() || co.len() > 12 {
        return None;
    }
    let labels: Vec<String> = re.iter().chain(&pr).map(|x| sub(x)).collect();
    let cls: Vec<&str> = (0..co.len()).map(|i| if i < re.len() { "axp-b" } else { "axp-b2" }).collect();
    let balanced = s.out_text("balanced")?;
    let body = bars(&labels, &co, &cls, None, "coefficient in the balanced equation", &sub(&balanced), false,
                    Some(re.len()))?;
    Some(s.chart("coefficients", "Balanced equation: the coefficients",
                 &format!("The balanced equation {}: the coefficient of each reactant (left of the dashed line) and \
                           product (right), as returned by the model for the stated skeleton {}.", sub(&balanced), eq.trim()),
                 &body))
}

fn dalton(s: &Solved) -> Option<Figure> {
    let moles = s.vec("moles")?;
    let pp = s.out_list("partial_pressures")?;
    let xf = s.out_list("mole_fractions")?;
    let pt = s.get("P_total")?;
    if pp.len() != moles.len() || xf.len() != pp.len() || pp.len() > 10 {
        return None;
    }
    let mut labels: Vec<String> = moles.iter().enumerate().map(|(i, m)| format!("gas {}\n({} mol)", i + 1, tick(*m))).collect();
    let mut vals: Vec<f64> = pp.iter().map(|p| p / 1000.0).collect();
    let mut cls: Vec<&str> = (0..pp.len()).map(|i| ["axp-b", "axp-b2", "axp-b3"][i % 3]).collect();
    let mut tops: Vec<String> = vals.iter().zip(&xf).map(|(v, x)| format!("{} kPa\nx = {}", tick(*v), crate::figs_extra::short(*x))).collect();
    let sum: f64 = vals.iter().sum();
    labels.push("sum".into());
    vals.push(sum);
    cls.push("axp-b4");
    tops.push(format!("total {} kPa", tick(sum)));
    let body = bars(&labels, &vals, &cls, Some(&tops), "partial pressure (kPa)", "", false, Some(pp.len()))?;
    Some(s.chart("partial-pressures", "Partial pressures of a gas mixture",
                 &format!("Dalton's law: a mixture of {} mol of {} gases at a total pressure of {} kPa; each bar is a \
                           partial pressure returned by the model with its mole fraction ({}), and the last bar adds them \
                           back up to {} kPa.", moles.iter().map(|m| tick(*m)).collect::<Vec<_>>().join(", "), pp.len(),
                          tick(pt / 1000.0), xf.iter().map(|x| tick(*x)).collect::<Vec<_>>().join(", "), tick(sum)), &body))
}

fn mass_to_atoms(s: &Solved) -> Option<Figure> {
    let comp = s.txt("composition")?;
    let counts = s.out_list("counts")?;
    let formula = s.out_text("formula")?;
    // the stated composition as given: "C 40.0, H 6.71, O 53.3"
    let mut stated: Vec<(String, f64)> = vec![];
    for part in comp.split(',') {
        let mut w = part.split_whitespace();
        let (Some(e), Some(v)) = (w.next(), w.next()) else { return None };
        let v: f64 = v.trim_end_matches('%').parse().ok()?;
        stated.push((e.to_string(), v));
    }
    if stated.len() != counts.len() || stated.is_empty() {
        return None;
    }
    let labels: Vec<String> = stated.iter().map(|x| x.0.clone()).collect();
    let pv: Vec<f64> = stated.iter().map(|x| x.1).collect();
    let tops: Vec<String> = pv.iter().map(|v| format!("{} %", tick(*v))).collect();
    let top = bars(&labels, &pv, &vec!["axp-b4"; pv.len()], Some(&tops), "stated mass percent (%)",
                   "Stated composition (by mass)", false, None)?;
    let bottom = bars(&labels, &counts, &vec!["axp-b"; counts.len()], None, "atoms in the formula",
                      &format!("Atom ratio returned: {}", sub(&formula)), false, None)?;
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("mass-to-atoms", "Empirical formula from a mass composition",
                  &format!("Top: the stated composition, {} by mass. Bottom: the whole-number atom ratio the model \
                            returns, {}: the empirical formula {}.",
                           stated.iter().map(|x| format!("{} {} %", x.0, tick(x.1))).collect::<Vec<_>>().join(", "),
                           labels.iter().zip(&counts).map(|(l, c)| format!("{l} {}", tick(*c))).collect::<Vec<_>>().join(", "),
                           sub(&formula)), &body, h))
}

fn graham(s: &Solved) -> Option<Figure> {
    let (g1, g2, r) = (s.txt("formula_1")?.trim().to_string(), s.txt("formula_2")?.trim().to_string(), s.out("ratio")?);
    let mut list: Vec<String> = GASES.iter().map(|x| x.to_string()).chain(["Cl2".to_string()]).collect();
    if !list.contains(&g1) {
        list.insert(0, g1.clone());
    }
    let (mut labels, mut vals, mut cls) = (vec![], vec![], vec![]);
    for g in &list {
        let Some(a) = s.with_input(&[("formula_1", Input::Text(g.clone()))]) else { continue };
        let Some(v) = num_of(&a, "ratio") else { continue };
        labels.push(sub(g));
        vals.push(v);
        cls.push(if *g == g1 { "axp-b2" } else { "axp-b" });
    }
    if labels.len() < 2 || !cls.contains(&"axp-b2") {
        return None;
    }
    let mut body = bars(&labels, &vals, &cls, None, &format!("effusion rate relative to {}", sub(&g2)), "", false, None)?;
    // the line where a gas effuses as fast as the reference
    let (lo, hi) = Chart::range(&vals);
    let ya = crate::figs_common::Axis::new(lo.min(0.0), hi.max(0.0), false, crate::figs_common::CY1,
                                           crate::figs_common::CY0, true);
    let y1 = ya.px(1.0);
    body.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{},{y1:.1} L{},{y1:.1}\"/>",
                           crate::figs_common::CX0, crate::figs_common::CX1));
    Some(s.chart("by-gas", &format!("Effusion rates relative to {}", sub(&g2)),
                 &format!("Graham's law: how many times faster each gas effuses than {} (dashed: as fast), each ratio \
                           returned by the model: {}. Highlighted: the gas given, {} against {}, ratio {}.", sub(&g2),
                          labels.iter().zip(&vals).map(|(l, v)| format!("{l} {}", tick(*v))).collect::<Vec<_>>().join(", "),
                          sub(&g1), sub(&g2), tick(r)), &body))
}

fn before_after(s: &Solved) -> Option<Figure> {
    let (ra, rb, pr) = (s.txt("reactant_a")?.trim().to_string(), s.txt("reactant_b")?.trim().to_string(),
                        s.txt("product")?.trim().to_string());
    let (ma, mb) = (s.get("m_a")?, s.get("m_b")?);
    let (pm, lim, ex) = (s.out("product_mass")?, s.out_text("limiting")?, s.out("excess_left")?);
    let (after_a, after_b) = if lim.trim() == ra { (0.0, ex) } else if lim.trim() == rb { (ex, 0.0) } else { return None };
    let labels = vec![sub(&ra), sub(&rb), sub(&pr)];
    let before = vec![Some(ma * 1000.0), Some(mb * 1000.0), Some(0.0)];
    let after = vec![Some(after_a * 1000.0), Some(after_b * 1000.0), Some(pm * 1000.0)];
    let body = k::bar_groups(&labels, &[("before".to_string(), before), ("after".to_string(), after)], "",
                             "mass (g)", &format!("limiting: {}", sub(&lim)));
    Some(s.chart("before-after", "Limiting reactant: masses before and after",
                 &format!("Masses before and after the reaction {} with {} g of {} and {} g of {}. The model returns the \
                           limiting reactant ({}, used up), the excess left ({} g) and the product formed ({} g of {}).",
                          s.txt("equation")?.trim(), tick(ma * 1000.0), sub(&ra), tick(mb * 1000.0), sub(&rb), sub(&lim),
                          tick(ex * 1000.0), tick(pm * 1000.0), sub(&pr)), &body))
}

fn multiples(s: &Solved) -> Option<Figure> {
    let (emp, m, mult) = (s.txt("empirical")?.trim().to_string(), s.get("M")?, s.out("multiple")?);
    if !(mult >= 1.0) || !(m > 0.0) {
        return None;
    }
    let top = (mult as usize + 1).clamp(6, 8);
    let (mut labels, mut vals, mut tops, mut cls, mut parts) = (vec![], vec![], vec![], vec![], vec![]);
    for kk in 1..=top {
        let mk = m * kk as f64 / mult;
        let Some(a) = s.with(&[("M", mk)]) else { continue };
        let (Some(v), Some(fm)) = (num_of(&a, "multiple"), a.values.iter().find(|x| x.0 == "formula")
            .and_then(|x| x.1.as_str().map(String::from))) else { continue };
        labels.push(tick(mk * 1000.0));
        vals.push(v);
        tops.push(sub(&fm));
        cls.push(if (v - mult).abs() < 1e-9 { "axp-b2" } else { "axp-b" });
        parts.push(format!("{} g/mol -> {}", tick(mk * 1000.0), sub(&fm)));
    }
    if labels.len() < 2 {
        return None;
    }
    let body = bars(&labels, &vals, &cls, Some(&tops), &format!("multiple of {}", sub(&emp)), "", false, None)?;
    let body = body + &format!("<text class=\"axp-l\" x=\"292\" y=\"326\" text-anchor=\"middle\">stated molar mass M (g/mol)</text>");
    Some(s.chart("multiples", "Molecular formulas from one empirical formula",
                 &format!("From the empirical formula {} to the molecular formula: for {} molar masses (whole multiples \
                           of the one given) the model returns the multiple and the formula: {}. The molar mass given ({} \
                           g/mol) gives {}.", sub(&emp), labels.len(), parts.join(", "), tick(m * 1000.0),
                          sub(&s.out_text("formula")?)), &body))
}

fn air(s: &Solved) -> Option<Figure> {
    let (x, pt, p) = (s.get("x")?, s.get("P_total")?, s.get("p")?);
    let gases = [("N2", 0.7808), ("O2", 0.2095), ("Ar", 0.00934), ("CO2", 0.00042)];
    let (mut labels, mut vals, mut cls) = (vec![], vec![], vec![]);
    for (g, xf) in gases {
        let Some(a) = s.with(&[("x", xf)]) else { continue };
        let Some(v) = num_of(&a, "p") else { continue };
        labels.push(format!("{}\nx={}", sub(g), tick(xf)));
        vals.push(v);
        cls.push("axp-b");
    }
    labels.push(format!("given\nx={}", tick(x)));
    vals.push(p);
    cls.push("axp-b2");
    if labels.len() < 2 {
        return None;
    }
    let body = bars(&labels, &vals, &cls, None, "partial pressure p (Pa)", "", true, Some(labels.len() - 1))?;
    Some(s.chart("air", "Partial pressures of the gases in air",
                 &format!("Partial pressures in dry air at a total of {} kPa (logarithmic axis), each from the model with \
                           a stated mole fraction: N2 0.7808, O2 0.2095, Ar 0.00934, CO2 0.00042; last bar: the mole \
                           fraction given, x = {}, p = {} Pa.", tick(pt / 1000.0), tick(x), tick(p)), &body))
}

fn yield_bars(s: &Solved) -> Option<Figure> {
    let (th, act, py) = (s.out("theoretical_mass")?, s.get("actual")?, s.out("percent_yield")?);
    let pr = s.txt("product")?.trim().to_string();
    let diff = th - act;
    let labels = vec!["theoretical".to_string(), "actual".to_string(), "not obtained".to_string()];
    let vals = vec![th * 1000.0, act * 1000.0, diff * 1000.0];
    let tops = vec![format!("{} g", tick(th * 1000.0)), format!("{} g = {} %", tick(act * 1000.0), tick(py)),
                    format!("{} g", tick(diff * 1000.0))];
    let body = bars(&labels, &vals, &["axp-b4", "axp-b", "axp-b3"], Some(&tops), &format!("mass of {} (g)", sub(&pr)),
                    "", false, None)?;
    Some(s.chart("yield-bars", "Percent yield: actual against theoretical",
                 &format!("Percent yield of {} from {} g of {} by {}: the model's theoretical mass ({} g) against the \
                           stated actual mass ({} g); right: the difference not obtained ({} g, derived). Marked: percent \
                           yield {} %.", sub(&pr), tick(s.get("m")? * 1000.0), sub(s.txt("reactant")?.trim()),
                          s.txt("equation")?.trim(), tick(th * 1000.0), tick(act * 1000.0), tick(diff * 1000.0), tick(py)),
                 &body))
}

fn products(s: &Solved) -> Option<Figure> {
    let (re, pr) = species(&s.txt("equation")?)?;
    let reac = s.txt("reactant")?.trim().to_string();
    let m = s.get("m")?;
    let mine = s.txt("product")?.trim().to_string();
    let _ = re;
    let (mut labels, mut vals, mut tops, mut cls) = (vec![format!("{} (given)", sub(&reac))], vec![m * 1000.0],
                                                     vec![format!("{} g", tick(m * 1000.0))], vec!["axp-b4"]);
    for p in &pr {
        let Some(a) = s.with_input(&[("product", Input::Text(p.clone()))]) else { continue };
        let (Some(pm), Some(n)) = (num_of(&a, "product_mass"), num_of(&a, "n_product")) else { continue };
        labels.push(sub(p));
        vals.push(pm * 1000.0);
        tops.push(format!("{} g ({} mol)", tick(pm * 1000.0), tick(n)));
        cls.push(if *p == mine { "axp-b2" } else { "axp-b" });
    }
    if labels.len() < 2 {
        return None;
    }
    let body = bars(&labels, &vals, &cls, Some(&tops), "mass (g)", "", false, Some(1))?;
    Some(s.chart("products", "Theoretical yield of each product",
                 &format!("Theoretical yield of every product of {} from {} g of {} (the other reactants in excess), \
                           each returned by the model: {}. Highlighted: the product asked, {}.", s.txt("equation")?.trim(),
                          tick(m * 1000.0), sub(&reac),
                          labels.iter().zip(&tops).skip(1).map(|(l, t)| format!("{l} {t}")).collect::<Vec<_>>().join(", "),
                          sub(&mine)), &body))
}
