# chemistry-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `chemistry_ai999x.about()` for the same answer.

Every capability chemistry-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import chemistry_ai999x

result = chemistry_ai999x.solve('molar_mass', formula='H2O')
result.show()
# M = 0.018015 kg/mol, M_g = 18.015
```

## By topic

| topic | capabilities |
|---|---|
| Acids and bases / buffers | [Buffer pH (Henderson-Hasselbalch)](#buffer_ph) |
| Acids and bases / pH | [PH from the hydronium concentration](#ph_from_hydronium), [Hydronium concentration from pH](#hydronium_from_ph), [PH from the hydroxide concentration](#ph_from_hydroxide) |
| Acids and bases / polyprotic acids | [PH of a diprotic acid](#ph_diprotic_acid) |
| Acids and bases / salt hydrolysis | [PH of a salt of a weak acid](#ph_salt_of_weak_acid) |
| Acids and bases / strong acids | [PH of a strong monoprotic acid](#ph_strong_acid) |
| Acids and bases / strong bases | [PH of a strong base](#ph_strong_base) |
| Acids and bases / titrations | [Titration: equivalence-point volume](#titration_equivalence_volume), [PH during a strong acid - strong base titration](#titration_ph_strong_acid_strong_base), [PH during a weak acid - strong base titration](#titration_ph_weak_acid_strong_base) |
| Acids and bases / weak acids | [PH of a weak acid](#ph_weak_acid), [Ka from the pH of a weak acid](#ka_from_ph) |
| Acids and bases / weak bases | [PH of a weak base](#ph_weak_base) |
| Electrochemistry / cell potentials | [Nernst equation](#nernst_potential) |
| Electrochemistry / cell thermodynamics | [Gibbs energy from a cell potential](#delta_g_from_cell_potential), [Equilibrium constant from a standard cell potential](#k_from_cell_potential) |
| Electrochemistry / electrolysis | [Mass deposited in electrolysis (Faraday's law)](#electrolysis_mass), [Time for electrolysis (Faraday's law)](#electrolysis_time) |
| Equilibrium / equilibrium constants | [Kp from Kc](#kp_from_kc), [Kc from Kp](#kc_from_kp) |
| Equilibrium / ICE tables | [Equilibrium concentrations (ICE table)](#equilibrium_ice) |
| Equilibrium / solubility | [Molar solubility from Ksp](#ksp_molar_solubility), [Ksp from molar solubility](#ksp_from_solubility), [Molar solubility with a common ion](#common_ion_solubility) |
| Equilibrium / temperature dependence | [Van 't Hoff equation: K at another temperature](#vant_hoff_k2) |
| Equilibrium / thermodynamics of equilibrium | [Equilibrium constant from the standard Gibbs energy](#k_from_delta_g), [Standard Gibbs energy from the equilibrium constant](#delta_g_from_k), [Gibbs energy change at non-standard conditions](#delta_g_nonstandard) |
| Gases / gas laws | [Boyle's law: final volume](#boyle_final_volume), [Boyle's law: final pressure](#boyle_final_pressure), [Charles's law: final volume](#charles_final_volume), [Gay-Lussac's law: final pressure](#gay_lussac_final_pressure), [Combined gas law: final volume](#combined_gas_final_volume), [Combined gas law: final pressure](#combined_gas_final_pressure), [Combined gas law: final temperature](#combined_gas_final_temperature) |
| Gases / gas mixtures | [Partial pressure (Dalton's law)](#partial_pressure), [Partial pressures in a gas mixture](#dalton_mixture) |
| Gases / ideal gas law | [Ideal gas law: pressure](#ideal_gas_pressure), [Ideal gas law: volume](#ideal_gas_volume), [Ideal gas law: amount](#ideal_gas_moles), [Ideal gas law: temperature](#ideal_gas_temperature), [Mass of a gas sample (ideal gas)](#ideal_gas_mass), [Density of an ideal gas](#gas_density), [Molar mass from gas density](#molar_mass_from_gas_density), [Molar mass from a gas sample](#molar_mass_from_gas_sample) |
| Gases / kinetic theory | [Graham's law: ratio of effusion rates](#graham_rate_ratio), [Graham's law: molar mass of an unknown gas](#graham_molar_mass), [Root-mean-square speed of gas molecules](#rms_speed), [Mean speed of gas molecules](#mean_speed) |
| Gases / real gases | [Van der Waals equation: pressure](#van_der_waals_pressure) |
| Kinetics / complex reactions | [Consecutive first-order reactions (A -> B -> C)](#consecutive_first_order) |
| Kinetics / half-life | [Half-life of a first-order reaction](#first_order_half_life) |
| Kinetics / integrated rate laws | [Zero-order integrated rate law](#zero_order_concentration), [First-order integrated rate law](#first_order_concentration), [Second-order integrated rate law](#second_order_concentration), [Time to reach a concentration (first order)](#first_order_time) |
| Kinetics / rate laws | [Reaction order from initial rates](#reaction_order_from_rates) |
| Kinetics / temperature dependence | [Arrhenius equation: rate constant at another temperature](#arrhenius_k2), [Activation energy from two rate constants](#activation_energy), [Arrhenius equation: rate constant](#arrhenius_rate_constant) |
| Molecular properties / structure | [Degree of unsaturation](#degree_of_unsaturation) |
| Solutions / colligative properties | [Vapor pressure of a solution (Raoult's law)](#raoult_vapor_pressure), [Freezing-point depression](#freezing_point_depression), [Boiling-point elevation](#boiling_point_elevation), [Molar mass from freezing-point depression](#molar_mass_from_freezing_point), [Osmotic pressure](#osmotic_pressure), [Molar mass from osmotic pressure](#molar_mass_from_osmotic_pressure) |
| Solutions / concentration | [Molarity from a dissolved mass](#molarity_from_mass), [Mass of solute for a solution](#solute_mass_for_solution), [Molality of a solution](#molality), [Mole fraction in a solution](#mole_fraction) |
| Solutions / dilution | [Dilution: final concentration](#dilution_final_concentration), [Dilution: volume of stock needed](#dilution_volume) |
| Spectroscopy / absorption | [Beer-Lambert law: absorbance](#beer_lambert_absorbance), [Beer-Lambert law: concentration](#beer_lambert_concentration) |
| Spectroscopy / atomic spectra | [Hydrogen atom transition wavelength (Rydberg)](#hydrogen_transition_wavelength) |
| Spectroscopy / infrared | [Vibrational wavenumber of a diatomic molecule](#vibrational_wavenumber) |
| Spectroscopy / light | [Energy of a photon from its wavelength](#photon_energy), [Wavelength of a photon from its energy](#photon_wavelength) |
| Stoichiometry / chemical equations | [Balancing a chemical equation](#balance_equation) |
| Stoichiometry / composition | [Mass percent of an element](#mass_percent), [Mass of an element in a sample](#mass_of_element), [Empirical formula from percent composition](#empirical_formula), [Molecular formula from the empirical formula](#molecular_formula) |
| Stoichiometry / molar mass | [Molar mass of a formula](#molar_mass) |
| Stoichiometry / reaction yields | [Limiting reagent and theoretical yield](#limiting_reagent), [Theoretical yield from one reactant](#theoretical_yield), [Percent yield of a reaction](#percent_yield) |
| Stoichiometry / the mole | [Moles from a mass](#moles_from_mass), [Mass from moles](#mass_from_moles), [Number of molecules in a mass](#particles_from_mass), [Number of particles in an amount](#particles_from_moles), [Moles from a number of particles](#moles_from_particles), [Atoms of an element in a mass of compound](#atoms_of_element) |
| Thermochemistry / calorimetry | [Heat from a temperature change](#calorimetry_heat), [Reaction enthalpy from coffee-cup calorimetry](#enthalpy_from_calorimetry) |
| Thermochemistry / Hess's law | [Reaction enthalpy from enthalpies of formation (Hess's law)](#reaction_enthalpy_from_formation) |
| Thermochemistry / phase equilibria | [Clausius-Clapeyron: vapor pressure at another temperature](#clausius_clapeyron_pressure), [Clausius-Clapeyron: enthalpy of vaporization](#clausius_clapeyron_enthalpy) |
| Thermochemistry / reaction enthalpy | [Heat of reaction for a given mass](#heat_from_reaction_mass) |
| Thermochemistry / spontaneity | [Gibbs energy change (dG = dH - T dS)](#gibbs_free_energy), [Temperature at which dG = 0](#crossover_temperature) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| molar_mass | Molar mass of a formula | formula | M, M_g |
| moles_from_mass | Moles from a mass | formula, m | n, N, M |
| mass_from_moles | Mass from moles | formula, n | m, m_g, N |
| particles_from_mass | Number of molecules in a mass | formula, m | N, n |
| particles_from_moles | Number of particles in an amount | n | N |
| moles_from_particles | Moles from a number of particles | N | n |
| atoms_of_element | Atoms of an element in a mass of compound | formula, element, m | N_atoms, n_element |
| mass_percent | Mass percent of an element | formula, element | percent, composition |
| mass_of_element | Mass of an element in a sample | formula, element, m | m_element, fraction |
| empirical_formula | Empirical formula from percent composition | composition | formula, counts |
| molecular_formula | Molecular formula from the empirical formula | empirical, M | multiple, formula |
| balance_equation | Balancing a chemical equation | equation | balanced, coefficients |
| limiting_reagent | Limiting reagent and theoretical yield | equation, reactant_a, m_a, reactant_b, m_b, product | product_mass, limiting, excess_left |
| theoretical_yield | Theoretical yield from one reactant | equation, reactant, m, product | product_mass, n_product |
| percent_yield | Percent yield of a reaction | equation, reactant, m, product, actual | percent_yield, theoretical_mass |
| ideal_gas_pressure | Ideal gas law: pressure | n, V, T | P, P_atm |
| ideal_gas_volume | Ideal gas law: volume | n, P, T | V, V_L |
| ideal_gas_moles | Ideal gas law: amount | P, V, T | n |
| ideal_gas_temperature | Ideal gas law: temperature | P, V, n | T, T_C |
| ideal_gas_mass | Mass of a gas sample (ideal gas) | formula, P, V, T | m, n |
| gas_density | Density of an ideal gas | formula, P, T | rho |
| molar_mass_from_gas_density | Molar mass from gas density | rho, P, T | M, M_g |
| molar_mass_from_gas_sample | Molar mass from a gas sample | m, V, P, T | M, M_g |
| boyle_final_volume | Boyle's law: final volume | P1, V1, P2 | V2 |
| boyle_final_pressure | Boyle's law: final pressure | P1, V1, V2 | P2 |
| charles_final_volume | Charles's law: final volume | V1, T1, T2 | V2 |
| gay_lussac_final_pressure | Gay-Lussac's law: final pressure | P1, T1, T2 | P2 |
| combined_gas_final_volume | Combined gas law: final volume | P1, V1, T1, P2, T2 | V2 |
| combined_gas_final_pressure | Combined gas law: final pressure | P1, V1, T1, V2, T2 | P2 |
| combined_gas_final_temperature | Combined gas law: final temperature | P1, V1, T1, P2, V2 | T2 |
| partial_pressure | Partial pressure (Dalton's law) | x, P_total | p |
| dalton_mixture | Partial pressures in a gas mixture | moles, P_total | partial_pressures, mole_fractions |
| graham_rate_ratio | Graham's law: ratio of effusion rates | formula_1, formula_2 | ratio |
| graham_molar_mass | Graham's law: molar mass of an unknown gas | ratio, formula_known | M, M_g |
| van_der_waals_pressure | Van der Waals equation: pressure | n, V, T, a, b | P, P_ideal, P_atm |
| rms_speed | Root-mean-square speed of gas molecules | formula, T | u_rms |
| mean_speed | Mean speed of gas molecules | formula, T | u_mean |
| molarity_from_mass | Molarity from a dissolved mass | formula, m, V | c, c_molL |
| solute_mass_for_solution | Mass of solute for a solution | formula, c, V | m, m_g |
| dilution_final_concentration | Dilution: final concentration | c1, V1, V2 | c2, c2_molL |
| dilution_volume | Dilution: volume of stock needed | c1, c2, V2 | V1, V1_mL |
| molality | Molality of a solution | formula, m_solute, m_solvent | b |
| mole_fraction | Mole fraction in a solution | formula_solute, m_solute, formula_solvent, m_solvent | x_solute, x_solvent |
| raoult_vapor_pressure | Vapor pressure of a solution (Raoult's law) | formula_solute, m_solute, formula_solvent, m_solvent, P0 | P, dP |
| freezing_point_depression | Freezing-point depression | i, K, b | dT |
| boiling_point_elevation | Boiling-point elevation | i, K, b | dT |
| molar_mass_from_freezing_point | Molar mass from freezing-point depression | m_solute, m_solvent, K, dT, i | M, M_g |
| osmotic_pressure | Osmotic pressure | i, c, T | Pi, Pi_atm |
| molar_mass_from_osmotic_pressure | Molar mass from osmotic pressure | m, V, Pi, T, i | M, M_g |
| kp_from_kc | Kp from Kc | Kc, T, dn | Kp |
| kc_from_kp | Kc from Kp | Kp, T, dn | Kc |
| equilibrium_ice | Equilibrium concentrations (ICE table) | equation, initial, K | x, equilibrium |
| k_from_delta_g | Equilibrium constant from the standard Gibbs energy | dG, T | K, lnK |
| delta_g_from_k | Standard Gibbs energy from the equilibrium constant | K, T | dG |
| vant_hoff_k2 | Van 't Hoff equation: K at another temperature | K1, T1, T2, dH | K2, lnK2 |
| delta_g_nonstandard | Gibbs energy change at non-standard conditions | dG0, T, Q | dG |
| ksp_molar_solubility | Molar solubility from Ksp | Ksp, x, y | s, s_molL |
| ksp_from_solubility | Ksp from molar solubility | s, x, y | Ksp |
| common_ion_solubility | Molar solubility with a common ion | Ksp, x, y, c_common, common | s, s_molL |
| ph_from_hydronium | pH from the hydronium concentration | h | pH, pOH, oh |
| hydronium_from_ph | Hydronium concentration from pH | pH | h, h_molL, pOH, oh |
| ph_from_hydroxide | pH from the hydroxide concentration | oh | pH, pOH |
| ph_strong_acid | pH of a strong monoprotic acid | c | pH, h |
| ph_strong_base | pH of a strong base | c, n_oh | pH, pOH |
| ph_weak_acid | pH of a weak acid | Ka, c | pH, h, percent_ionized |
| ph_weak_base | pH of a weak base | Kb, c | pH, pOH, percent_ionized |
| ka_from_ph | Ka from the pH of a weak acid | pH, c | pKa, Ka |
| buffer_ph | Buffer pH (Henderson-Hasselbalch) | Ka, c_base, c_acid | pH |
| titration_equivalence_volume | Titration: equivalence-point volume | c_t, V_a, c_a, ratio | V_eq, V_eq_mL |
| titration_ph_strong_acid_strong_base | pH during a strong acid - strong base titration | c_a, V_a, c_b, V_b | pH |
| titration_ph_weak_acid_strong_base | pH during a weak acid - strong base titration | Ka, c_a, V_a, c_b, V_b | pH |
| ph_diprotic_acid | pH of a diprotic acid | Ka1, Ka2, c | pH |
| ph_salt_of_weak_acid | pH of a salt of a weak acid | Ka, c | pH, Kb |
| zero_order_concentration | Zero-order integrated rate law | A0, k, t | A, A_molL |
| first_order_concentration | First-order integrated rate law | A0, k, t | A, A_molL, fraction_left |
| second_order_concentration | Second-order integrated rate law | A0, k, t | A, A_molL |
| first_order_half_life | Half-life of a first-order reaction | k | t_half |
| first_order_time | Time to reach a concentration (first order) | A0, A, k | t |
| arrhenius_k2 | Arrhenius equation: rate constant at another temperature | k1, T1, T2, Ea | k2, ratio |
| activation_energy | Activation energy from two rate constants | k1, T1, k2, T2 | Ea, Ea_kJ |
| arrhenius_rate_constant | Arrhenius equation: rate constant | A, Ea, T | k |
| reaction_order_from_rates | Reaction order from initial rates | c1, r1, c2, r2 | order |
| consecutive_first_order | Consecutive first-order reactions (A -> B -> C) | A0, k1, k2, t | B, A, C |
| calorimetry_heat | Heat from a temperature change | m, c, T1, T2 | q |
| enthalpy_from_calorimetry | Reaction enthalpy from coffee-cup calorimetry | m, c, T1, T2, n | dH, dH_kJ, q_solution |
| reaction_enthalpy_from_formation | Reaction enthalpy from enthalpies of formation (Hess's law) | equation, values | dH, dH_kJ |
| gibbs_free_energy | Gibbs energy change (dG = dH - T dS) | dH, dS, T | dG, dG_kJ, spontaneity |
| crossover_temperature | Temperature at which dG = 0 | dH, dS | T, T_C |
| heat_from_reaction_mass | Heat of reaction for a given mass | formula, m, dH | q, heat_released |
| clausius_clapeyron_pressure | Clausius-Clapeyron: vapor pressure at another temperature | P1, T1, T2, dH | P2 |
| clausius_clapeyron_enthalpy | Clausius-Clapeyron: enthalpy of vaporization | P1, T1, P2, T2 | dH |
| delta_g_from_cell_potential | Gibbs energy from a cell potential | n, E | dG, dG_kJ |
| k_from_cell_potential | Equilibrium constant from a standard cell potential | n, E, T | lnK, K |
| nernst_potential | Nernst equation | E0, n, Q, T | E |
| electrolysis_mass | Mass deposited in electrolysis (Faraday's law) | metal, I, t, n | m, m_g |
| electrolysis_time | Time for electrolysis (Faraday's law) | metal, m, I, n | t |
| photon_energy | Energy of a photon from its wavelength | lam | E, E_eV, E_molar, nu |
| photon_wavelength | Wavelength of a photon from its energy | E | lam, lam_nm |
| beer_lambert_absorbance | Beer-Lambert law: absorbance | eps, l, c | A, T_percent |
| beer_lambert_concentration | Beer-Lambert law: concentration | A, eps, l | c, c_molL |
| hydrogen_transition_wavelength | Hydrogen atom transition wavelength (Rydberg) | n_upper, n_lower | lam, lam_nm, E |
| degree_of_unsaturation | Degree of unsaturation | formula | dou |
| vibrational_wavenumber | Vibrational wavenumber of a diatomic molecule | molecule, k | wavenumber, wavenumber_cm, nu |

<a id="molar_mass"></a>

### Molar mass of a formula — `molar_mass`

The molar mass of a chemical formula (parentheses, brackets and hydrates are read).

$$
M = \sum_i n_i A_r(i)\ \mathrm{g\,mol^{-1}}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula, e.g. Ca(OH)2 or CuSO4·5H2O |  |

| output | meaning |
|---|---|
| `M` | molar mass (kg/mol) |
| `M_g` | molar mass in g/mol |

- **Method:** atom counts read from the formula (nested groups multiply, hydrate parts add), times standard atomic weights
- **Assumes:** a neutral formula of element symbols with standard atomic weights
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Where the molar mass of glucose, C₆H₁₂O₆, comes from: each bar is one element's share, its mass percent (returned by the package) times the molar mass M = 180](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-composition)

*Where the molar mass of glucose, C₆H₁₂O₆, comes from: each bar is one element's share, its mass percent (returned by the package) times the molar mass M = 180.156 g/mol (returned by the package) — the shares add up to M. Marked: M = 180.156 g/mol; carbon 72.066, oxygen 95.994 g/mol.*

![Molar masses of eight common substances, each returned by the package: H₂O 18](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-by-substance)

*Molar masses of eight common substances, each returned by the package: H₂O 18.02, NH₃ 17.03, CH₄ 16.04, CO₂ 44.01, NaCl 58.44, H₂SO₄ 98.08, CaCO₃ 100.1, C₆H₁₂O₆ 180.2 g/mol. Highlighted: the worked example H₂O.*

```python
chemistry_ai999x.solve('molar_mass', formula='H2O')
# -> M = 0.018015 kg/mol, M_g = 18.015   (SOLVED)
```

```python
chemistry_ai999x.solve('molar_mass', formula='Ca(OH)2')
# -> M = 0.074092 kg/mol, M_g = 74.092   (SOLVED)
```

```python
chemistry_ai999x.solve('molar_mass', formula='CuSO4·5H2O')
# -> M = 0.249684 kg/mol, M_g = 249.684   (SOLVED)
```

```python
chemistry_ai999x.solve('molar_mass', formula='K4[Fe(CN)6]')
# -> M = 0.368345 kg/mol, M_g = 368.345   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the molar mass of Ca(OH)2? | SOLVED | M = 0.074092 kg/mol, M_g = 74.092 |
| Find the molar mass of CuSO4·5H2O. | SOLVED | M = 0.249684 kg/mol, M_g = 249.684 |
| Calculate the molecular weight of C6H12O6. | SOLVED | M = 0.180156 kg/mol, M_g = 180.156 |
| What is the molar mass of Xy2O? | NEEDS_INPUT | molar_mass needs formula (chemical formula, e.g. Ca(OH)2 or CuSO4·5H2O). Example: solve('molar_mass', formula='H2O'). |

<a id="moles_from_mass"></a>

### Moles from a mass — `moles_from_mass`

The amount of substance (and number of formula units) in a given mass of a compound.

$$
n = \frac{m}{M}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula |  |
| `m` | mass of the sample |  |

| output | meaning |
|---|---|
| `n` | amount of substance |
| `N` | number of formula units |
| `M` | molar mass |

- **Method:** $n = m / M$ with M from the formula
- **Assumes:** $m > 0$; a neutral formula
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Amount of substance in the same mass, 36 g, of eight substances, each returned by the package: H₂O 2, NH₃ 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/moles-from-mass-by-substance)

*Amount of substance in the same mass, 36 g, of eight substances, each returned by the package: H₂O 2, NH₃ 2.11, CH₄ 2.24, CO₂ 0.818, NaCl 0.616, H₂SO₄ 0.367, CaCO₃ 0.36, C₆H₁₂O₆ 0.2 mol. Highlighted: the worked example H₂O.*

```python
chemistry_ai999x.solve('moles_from_mass', formula='H2O', m=0.036)
# -> n = 1.99833 mol, N = 1.20343e+24, M = 0.018015 kg/mol   (SOLVED)
```

```python
chemistry_ai999x.solve('moles_from_mass', formula='NaCl', m=0.01)
# -> n = 0.171107 mol, N = 1.03043e+23, M = 0.058443 kg/mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many moles are in 36.0 g of H2O? | SOLVED | n = 1.99833 mol, N = 1.20343e+24, M = 0.018015 kg/mol |
| How many moles are there in 10.0 g of NaCl? | SOLVED | n = 0.171107 mol, N = 1.03043e+23, M = 0.058443 kg/mol |
| How many moles are in 250 mg of CaCO3? | SOLVED | n = 0.00249785 mol, N = 1.50424e+21, M = 0.100086 kg/mol |
| How many moles are in -5 g of H2O? | INVALID_INPUT | Moles from a mass: m must be a positive number. |

<a id="mass_from_moles"></a>

### Mass from moles — `mass_from_moles`

The mass of a given amount (moles) of a compound.

$$
m = n M
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula |  |
| `n` | amount of substance |  |

| output | meaning |
|---|---|
| `m` | mass |
| `m_g` | mass in grams |
| `N` | number of formula units |

- **Method:** $m = n$ M with M from the formula
- **Assumes:** $n > 0$; a neutral formula
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Mass of the same amount, 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-from-moles-by-substance)

*Mass of the same amount, 2.5 mol, of eight substances, each returned by the package: H₂O 45.04, NH₃ 42.58, CH₄ 40.11, CO₂ 110, NaCl 146.1, H₂SO₄ 245.2, CaCO₃ 250.2, C₆H₁₂O₆ 450.4 g. Highlighted: the worked example NaCl.*

```python
chemistry_ai999x.solve('mass_from_moles', formula='NaCl', n=2.5)
# -> m = 0.146108 kg, m_g = 146.107, N = 1.50554e+24   (SOLVED)
```

```python
chemistry_ai999x.solve('mass_from_moles', formula='H2SO4', n=0.15)
# -> m = 0.0147119 kg, m_g = 14.7119, N = 9.03321e+22   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the mass of 2.50 mol of NaCl? | SOLVED | m = 0.146108 kg, m_g = 146.107, N = 1.50554e+24 |
| How many grams are in 0.150 mol of H2SO4? | SOLVED | m = 0.0147119 kg, m_g = 14.7119, N = 9.03321e+22 |
| What is the mass of 3 mmol of C6H12O6? | SOLVED | m = 0.000540468 kg, m_g = 0.540468, N = 1.80664e+21 |

<a id="particles_from_mass"></a>

### Number of molecules in a mass — `particles_from_mass`

The number of molecules (formula units) in a given mass of a compound.

$$
N = \frac{m}{M} N_A
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula |  |
| `m` | mass of the sample |  |

| output | meaning |
|---|---|
| `N` | number of molecules / formula units |
| `n` | amount of substance |

- **Method:** $N = (m / M)$ N_A
- **Assumes:** $m > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Number of molecules in 10 g of nine gases, each returned by the package — light molecules are many per gram](https://www.ai999x.com/?ax_fig=chemistry-ai999x/particles-from-mass-by-gas)

*Number of molecules in 10 g of nine gases, each returned by the package — light molecules are many per gram. Highlighted: the worked example CO₂, 1.3684e+23 molecules.*

```python
chemistry_ai999x.solve('particles_from_mass', formula='CO2', m=0.01)
# -> N = 1.36839e+23, n = 0.227226 mol   (SOLVED)
```

```python
chemistry_ai999x.solve('particles_from_mass', formula='NaCl', m=0.005)
# -> N = 5.15215e+22, n = 0.0855534 mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many molecules are in 10.0 g of CO2? | SOLVED | N = 1.36839e+23, n = 0.227226 mol |
| How many formula units are in 5.00 g of NaCl? | SOLVED | N = 5.15215e+22, n = 0.0855534 mol |

<a id="particles_from_moles"></a>

### Number of particles in an amount — `particles_from_moles`

The number of particles in a given amount of substance (Avogadro's number).

$$
N = n N_A
$$

| input | meaning | condition |
|---|---|---|
| `n` | amount of substance |  |

| output | meaning |
|---|---|
| `N` | number of particles |

- **Method:** $N = n$ N_A
- **Assumes:** $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![N (number of particles) against n (amount of substance) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/particles-from-moles-curve)

*N (number of particles) against n (amount of substance) from 0.125 mol to 0.375 mol. 31 points computed by chemistry-ai999x; marked: the worked example, n = 0.25 mol, N = 1.5055e+23.*

```python
chemistry_ai999x.solve('particles_from_moles', n=0.25)
# -> N = 1.50554e+23   (SOLVED)
```

```python
chemistry_ai999x.solve('particles_from_moles', n=2.0)
# -> N = 1.20443e+24   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many molecules are in 0.250 mol of water? | SOLVED | N = 1.50554e+23 |
| How many atoms are in 2.0 mol of helium? | SOLVED | N = 1.20443e+24 |

<a id="moles_from_particles"></a>

### Moles from a number of particles — `moles_from_particles`

The amount of substance that contains a given number of particles.

$$
n = \frac{N}{N_A}
$$

| input | meaning | condition |
|---|---|---|
| `N` | number of particles |  |

| output | meaning |
|---|---|
| `n` | amount of substance |

- **Method:** $n = N / N_A$
- **Assumes:** $N > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![n (amount of substance) against N (number of particles) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/moles-from-particles-curve)

*n (amount of substance) against N (number of particles) from 1.5050e+23 to 4.5150e+23. 31 points computed by chemistry-ai999x; marked: the worked example, N = 3.0100e+23, n = 0.499822 mol.*

```python
chemistry_ai999x.solve('moles_from_particles', N=3.01e+23)
# -> n = 0.499822 mol   (SOLVED)
```

```python
chemistry_ai999x.solve('moles_from_particles', N=1.204e+24)
# -> n = 1.99929 mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many moles are 3.01e23 molecules of CO2? | SOLVED | n = 0.499822 mol |
| How many moles is 1.204 × 10^24 atoms of iron? | SOLVED | n = 1.99929 mol |

<a id="atoms_of_element"></a>

### Atoms of an element in a mass of compound — `atoms_of_element`

How many atoms of one element a given mass of a compound contains.

$$
N_X = \frac{m}{M} \, \nu_X \, N_A
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula of the compound |  |
| `element` | the element (symbol or name) |  |
| `m` | mass of the compound |  |

| output | meaning |
|---|---|
| `N_atoms` | number of atoms of the element |
| `n_element` | amount of the element |

- **Method:** moles of compound times the number of X atoms per formula, times N_A
- **Assumes:** $m > 0$; the element occurs in the formula
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Number of oxygen atoms in 10 g of seven substances, each returned by the package: H₂O 3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/atoms-of-element-by-substance)

*Number of oxygen atoms in 10 g of seven substances, each returned by the package: H₂O 3.34e23, CO₂ 2.74e23, O₂ 3.76e23, O₃ 3.76e23, H₂SO₄ 2.46e23, CaCO₃ 1.81e23, C₆H₁₂O₆ 2.01e23. Highlighted: the worked example CO₂.*

```python
chemistry_ai999x.solve('atoms_of_element', formula='CO2', element='O', m=0.01)
# -> N_atoms = 2.73678e+23, n_element = 0.454452 mol   (SOLVED)
```

```python
chemistry_ai999x.solve('atoms_of_element', formula='CH4', element='H', m=0.005)
# -> N_atoms = 7.5075e+23, n_element = 1.24665 mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many atoms of oxygen are in 10.0 g of CO2? | SOLVED | N_atoms = 2.73678e+23, n_element = 0.454452 mol |
| How many atoms of H are in 5.00 g of CH4? | SOLVED | N_atoms = 7.5075e+23, n_element = 1.24665 mol |
| How many atoms of N are in 10.0 g of CO2? | INVALID_INPUT | Atoms of an element in a mass of compound: N does not occur in CO2. |

<a id="mass_percent"></a>

### Mass percent of an element — `mass_percent`

The percentage by mass of an element in a compound (and the full percent composition).

$$
w_X = \frac{\nu_X A_r(X)}{M} \times 100\%
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula |  |
| `element` | the element (symbol or name) |  |

| output | meaning |
|---|---|
| `percent` | mass percent of the element (%) |
| `composition` | mass percent of every element (%) |

- **Method:** mass of the element per formula over the molar mass
- **Assumes:** the element occurs in the formula
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Mass composition of H₂SO₄: the percent by mass of every element, returned by the package (H 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-percent-composition)

*Mass composition of H₂SO₄: the percent by mass of every element, returned by the package (H 2.055 %, S 32.7 %, O 65.25 %). Highlighted: the element asked in the worked example, O.*

![Percent oxygen by mass in eight oxygen compounds, each returned by the package: H₂O 88](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-percent-oxygen-share)

*Percent oxygen by mass in eight oxygen compounds, each returned by the package: H₂O 88.81 %, CO₂ 72.71 %, H₂SO₄ 65.25 %, CaCO₃ 47.96 %, Fe₂O₃ 30.06 %, C₆H₁₂O₆ 53.28 %, KMnO₄ 40.5 %, SiO₂ 53.26 %.*

```python
chemistry_ai999x.solve('mass_percent', formula='H2SO4', element='O')
# -> percent = 65.2494, composition = {'H': 2.055485883828342, 'S': 32.69507233964457, 'O': 65.24944177652708}   (SOLVED)
```

```python
chemistry_ai999x.solve('mass_percent', formula='NH4NO3', element='N')
# -> percent = 34.9987, composition = {'N': 34.99868820508976, 'H': 5.0372924553052725, 'O': 59.96401933960495}   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the mass percent of O in H2SO4? | SOLVED | percent = 65.2494, composition = {'H': 2.055485883828342, 'S': 32.69507233964457, 'O': 65.24944177652708} |
| Find the percent by mass of nitrogen in NH4NO3. | SOLVED | percent = 34.9987, composition = {'N': 34.99868820508976, 'H': 5.0372924553052725, 'O': 59.96401933960495} |
| What is the mass percent of Fe in Fe2O3? | SOLVED | percent = 69.9431, composition = {'Fe': 69.94307614270416, 'O': 30.056923857295832} |
| What is the mass percent of Cl in H2SO4? | INVALID_INPUT | Mass percent of an element: Cl does not occur in H2SO4. |

<a id="mass_of_element"></a>

### Mass of an element in a sample — `mass_of_element`

The mass of one element contained in a given mass of a compound.

$$
m_X = m \, \frac{\nu_X A_r(X)}{M}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | chemical formula |  |
| `element` | the element (symbol or name) |  |
| `m` | mass of the compound |  |

| output | meaning |
|---|---|
| `m_element` | mass of the element |
| `fraction` | mass fraction |

- **Method:** the sample mass times the element's mass fraction
- **Assumes:** $m > 0$; the element occurs in the formula
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Iron contained in 50 g of six iron compounds (ores and salts), each mass returned by the package with its mass fraction: FeO 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-of-element-by-compound)

*Iron contained in 50 g of six iron compounds (ores and salts), each mass returned by the package with its mass fraction: FeO 0.777, Fe₂O₃ 0.699, Fe₃O₄ 0.724, FeS₂ 0.465, FeCO₃ 0.482, FeSO₄ 0.368. Highlighted: the worked example, Fe₂O₃, 34.9715 g.*

```python
chemistry_ai999x.solve('mass_of_element', formula='Fe2O3', element='Fe', m=0.05)
# -> m_element = 0.0349715 kg, fraction = 0.699431   (SOLVED)
```

```python
chemistry_ai999x.solve('mass_of_element', formula='NH4NO3', element='N', m=10.0)
# -> m_element = 3.49987 kg, fraction = 0.349987   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What mass of iron is in 50.0 g of Fe2O3? | SOLVED | m_element = 0.0349715 kg, fraction = 0.699431 |
| What mass of N is contained in 10.0 kg of NH4NO3? | SOLVED | m_element = 3.49987 kg, fraction = 0.349987 |

<a id="empirical_formula"></a>

### Empirical formula from percent composition — `empirical_formula`

The simplest whole-number formula of a compound from its mass percentages.

$$
n_i = \frac{w_i}{A_r(i)},\quad \text{subscripts} \propto \frac{n_i}{\min_j n_j}
$$

| input | meaning | condition |
|---|---|---|
| `composition` | element percentages (or masses), e.g. 'C 40.0, H 6.71, O 53.3' |  |

| output | meaning |
|---|---|
| `formula` | empirical formula |
| `counts` | subscripts in the order given |

- **Method:** moles per 100 g, divided by the smallest, multiplied by the smallest k (1..12) giving whole numbers within 1.5%
- **Assumes:** at least two elements; a whole-number ratio with subscripts up to 12 times the smallest
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Top: the stated composition, C 40 %, H 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/empirical-formula-mass-to-atoms)

*Top: the stated composition, C 40 %, H 6.71 %, O 53.3 % by mass. Bottom: the whole-number atom ratio the package returns, C 1, H 2, O 1 — the empirical formula CH₂O. Hydrogen is a small mass share but two atoms.*

```python
chemistry_ai999x.solve('empirical_formula', composition='C 40.0, H 6.71, O 53.3')
# -> formula = CH2O, counts = [1.0, 2.0, 1.0]   (SOLVED)
```

```python
chemistry_ai999x.solve('empirical_formula', composition='C 85.6, H 14.4')
# -> formula = CH2, counts = [1.0, 2.0]   (SOLVED)
```

```python
chemistry_ai999x.solve('empirical_formula', composition='Fe 69.94, O 30.06')
# -> formula = Fe2O3, counts = [2.0, 3.0]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the empirical formula of a compound with 40.0% C, 6.71% H and 53.3% O. | SOLVED | formula = CH2O, counts = [1.0, 2.0, 1.0] |
| What is the empirical formula of a compound that is 85.6% C and 14.4% H? | SOLVED | formula = CH2, counts = [1.0, 2.0] |
| Find the empirical formula of a compound containing 69.94% Fe and 30.06% O. | SOLVED | formula = Fe2O3, counts = [2.0, 3.0] |

<a id="molecular_formula"></a>

### Molecular formula from the empirical formula — `molecular_formula`

The molecular formula from an empirical formula and the molar mass.

$$
n = \frac{M}{M_{\text{empirical}}}
$$

| input | meaning | condition |
|---|---|---|
| `empirical` | empirical formula |  |
| `M` | molar mass of the compound |  |

| output | meaning |
|---|---|
| `multiple` | whole-number multiple n |
| `formula` | molecular formula |

- **Method:** $n = M / M$(empirical), rounded (must be within 2% of a whole number)
- **Assumes:** $M > 0$; M is a whole multiple of the empirical formula mass (within 2%)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![From the empirical formula CH₂O to the molecular formula: for six stated molar masses the package returns the multiple and the formula — 30](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molecular-formula-multiples)

*From the empirical formula CH₂O to the molecular formula: for six stated molar masses the package returns the multiple and the formula — 30.03 g/mol → CH₂O, 60.05 g/mol → C₂H₄O₂, 90.08 g/mol → C₃H₆O₃, 120.1 g/mol → C₄H₈O₄, 150.1 g/mol → C₅H₁₀O₅, 180.2 g/mol → C₆H₁₂O₆. The worked example (M = 180.16 g/mol) is C₆H₁₂O₆, glucose.*

```python
chemistry_ai999x.solve('molecular_formula', empirical='CH2O', M=0.18016)
# -> multiple = 6, formula = C6H12O6   (SOLVED)
```

```python
chemistry_ai999x.solve('molecular_formula', empirical='CH', M=0.07811)
# -> multiple = 6, formula = C6H6   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A compound has the empirical formula CH2O and a molar mass of 180.16 g/mol. What is its molecular formula? | SOLVED | multiple = 6, formula = C6H12O6 |
| The empirical formula is CH and the molar mass is 78.11 g/mol. Find the molecular formula. | SOLVED | multiple = 6, formula = C6H6 |
| The empirical formula is CH2O and the molar mass is 100 g/mol. Find the molecular formula. | INVALID_INPUT | Molecular formula from the empirical formula: the molar mass is 3.330 times the empirical formula mass, not a whole mult |

<a id="balance_equation"></a>

### Balancing a chemical equation — `balance_equation`

The smallest whole-number coefficients that balance a chemical equation.

$$
A\,\boldsymbol{\nu} = \mathbf{0},\ \boldsymbol{\nu} \in \mathbb{Z}_{>0}^{n}
$$

| input | meaning | condition |
|---|---|---|
| `equation` | the equation, e.g. '$C3H8 + O_{2}$ -> $CO2 + H2O$' |  |

| output | meaning |
|---|---|
| `balanced` | the balanced equation |
| `coefficients` | coefficients in the order the species are written |

- **Method:** exact rational null space of the element-conservation matrix, scaled to the smallest whole numbers
- **Assumes:** one independent way to balance (a one-dimensional null space); neutral species
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The balanced equation C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: the coefficient of each reactant (left) and product (right), as returned by the package for the stated skeleton C3H8 + O2 -> CO2 + H2O](https://www.ai999x.com/?ax_fig=chemistry-ai999x/balance-equation-coefficients)

*The balanced equation C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: the coefficient of each reactant (left) and product (right), as returned by the package for the stated skeleton C3H8 + O2 -> CO2 + H2O.*

![Atom balance of C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: for each element, the atoms on the reactant and on the product side — each the returned coefficient times the subscript read from the formula (derived) — C 3 = 3; H 8 = 8; O 10 = 10](https://www.ai999x.com/?ax_fig=chemistry-ai999x/balance-equation-atom-balance)

*Atom balance of C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: for each element, the atoms on the reactant and on the product side — each the returned coefficient times the subscript read from the formula (derived) — C 3 = 3; H 8 = 8; O 10 = 10.*

```python
chemistry_ai999x.solve('balance_equation', equation='C3H8 + O2 -> CO2 + H2O')
# -> coefficients = [1.0, 5.0, 3.0, 4.0], balanced = C3H8 + 5 O2 -> 3 CO2 + 4 H2O   (SOLVED)
```

```python
chemistry_ai999x.solve('balance_equation', equation='Fe + O2 -> Fe2O3')
# -> coefficients = [4.0, 3.0, 2.0], balanced = 4 Fe + 3 O2 -> 2 Fe2O3   (SOLVED)
```

```python
chemistry_ai999x.solve('balance_equation', equation='Cu + HNO3 -> Cu(NO3)2 + NO + H2O')
# -> coefficients = [3.0, 8.0, 3.0, 2.0, 4.0], balanced = 3 Cu + 8 HNO3 -> 3 Cu(NO3)2 + 2 NO + 4 H2O   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Balance the equation C3H8 + O2 -> CO2 + H2O. | SOLVED | coefficients = [1.0, 5.0, 3.0, 4.0], balanced = C3H8 + 5 O2 -> 3 CO2 + 4 H2O |
| Balance: Fe + O2 -> Fe2O3 | SOLVED | coefficients = [4.0, 3.0, 2.0], balanced = 4 Fe + 3 O2 -> 2 Fe2O3 |
| Balance KMnO4 + HCl → KCl + MnCl2 + H2O + Cl2. | SOLVED | coefficients = [2.0, 16.0, 2.0, 2.0, 8.0, 5.0], balanced = 2 KMnO4 + 16 HCl -> 2 KCl + 2 MnCl2 + 8 H2O + 5 Cl2 |
| Balance the equation H2 + O2 -> NaCl. | INVALID_INPUT | Balancing a chemical equation: the equation cannot be balanced uniquely (it has no solution or several independent ones) |

<a id="limiting_reagent"></a>

### Limiting reagent and theoretical yield — `limiting_reagent`

Which of two reactants limits a reaction, the mass of a product formed and the excess left over.

$$
\xi = \min_i \frac{n_i}{\nu_i},\quad m_P = \xi\,\nu_P M_P
$$

| input | meaning | condition |
|---|---|---|
| `equation` | the reaction (balanced or not) |  |
| `reactant_a` | first reactant |  |
| `m_a` | mass of the first reactant |  |
| `reactant_b` | second reactant |  |
| `m_b` | mass of the second reactant |  |
| `product` | the product asked for |  |

| output | meaning |
|---|---|
| `product_mass` | mass of product formed |
| `limiting` | limiting reagent |
| `excess_left` | mass of the excess reagent left over |

- **Method:** balance the equation, moles of each reactant over its coefficient; the smallest is the extent
- **Assumes:** the equation balances uniquely; two different reactants and one product of the equation
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Masses before and after the reaction H2 + O2 -> H2O with 4 g of H₂ and 16 g of O₂ (the worked example)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-before-after)

*Masses before and after the reaction H2 + O2 -> H2O with 4 g of H₂ and 16 g of O₂ (the worked example). The package returns the limiting reactant (O₂, used up), the excess left (1.98387 g) and the product formed (18.0161 g of H₂O).*

![limiting reagent (limiting) over m_a (mass of the first reactant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-decision)

*limiting reagent (limiting) over m_a (mass of the first reactant) from 0.0004 kg to 0.04 kg and m_b (mass of the second reactant) from 0.0016 kg to 0.16 kg (logarithmic axes): each of the 31 × 21 cells is one run of chemistry-ai999x, coloured by the verdict it returned — H2: 243 cells, O2: 408 cells. Other inputs as in the worked example: equation = H2 + O2 -> H2O, reactant_a = H2, reactant_b = O2, product = H2O. Marked: the worked example, m_a = 0.004 kg, m_b = 0.016 kg → O2.*

```python
chemistry_ai999x.solve('limiting_reagent', equation='H2 + O2 -> H2O', reactant_a='H2', m_a=0.004, reactant_b='O2', m_b=0.016, product='H2O')
# -> product_mass = 0.0180161 kg, limiting = O2, excess_left = 0.00198387 kg   (SOLVED)
```

```python
chemistry_ai999x.solve('limiting_reagent', equation='N2 + H2 -> NH3', reactant_a='N2', m_a=0.028, reactant_b='H2', m_b=0.01, product='NH3')
# -> product_mass = 0.034045 kg, limiting = N2, excess_left = 0.00395502 kg   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For H2 + O2 -> H2O, 4.0 g of H2 react with 16.0 g of O2. Which is the limiting reagent and what mass of H2O forms? | SOLVED | product_mass = 0.0180161 kg, limiting = O2, excess_left = 0.00198387 kg |
| In N2 + 3H2 -> 2NH3, 28.0 g of N2 and 10.0 g of H2 are mixed. Find the limiting reactant and the mass of NH3 produced. | SOLVED | product_mass = 0.034045 kg, limiting = N2, excess_left = 0.00395502 kg |
| For H2 + O2 -> H2O, 4.0 g of H2 react with oxygen. Which is the limiting reagent and what mass of H2O forms? | NEEDS_INPUT | limiting_reagent needs reactant_b, m_b (second reactant; mass of the second reactant). Example: solve('limiting_reagent' |

<a id="theoretical_yield"></a>

### Theoretical yield from one reactant — `theoretical_yield`

The mass of a product formed from a given mass of one reactant (in excess of the others).

$$
m_P = \frac{m_R}{M_R}\,\frac{\nu_P}{\nu_R}\,M_P
$$

| input | meaning | condition |
|---|---|---|
| `equation` | the reaction (balanced or not) |  |
| `reactant` | the reactant whose mass is given |  |
| `m` | mass of the reactant |  |
| `product` | the product asked for |  |

| output | meaning |
|---|---|
| `product_mass` | theoretical mass of product |
| `n_product` | moles of product |

- **Method:** balance the equation; moles of reactant times the coefficient ratio times the product's molar mass
- **Assumes:** the equation balances uniquely; the other reactants are in excess
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Theoretical yield of every product of CH4 + O2 -> CO2 + H2O from 16 g of CH₄ (the other reactant in excess), each returned by the package: CO₂ 43](https://www.ai999x.com/?ax_fig=chemistry-ai999x/theoretical-yield-products)

*Theoretical yield of every product of CH4 + O2 -> CO2 + H2O from 16 g of CH₄ (the other reactant in excess), each returned by the package: CO₂ 43.89 g, H₂O 35.93 g.*

![product_mass (theoretical mass of product) against m (mass of the reactant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/theoretical-yield-curve)

*product_mass (theoretical mass of product) against m (mass of the reactant) from 0.008 kg to 0.024 kg. Other inputs as in the worked example: equation = CH4 + O2 -> CO2 + H2O, reactant = CH4, product = CO2. 31 points computed by chemistry-ai999x; marked: the worked example, m = 0.016 kg, product_mass = 0.043891 kg.*

```python
chemistry_ai999x.solve('theoretical_yield', equation='CH4 + O2 -> CO2 + H2O', reactant='CH4', m=0.016, product='CO2')
# -> product_mass = 0.043891 kg, n_product = 0.99732 mol   (SOLVED)
```

```python
chemistry_ai999x.solve('theoretical_yield', equation='Fe2O3 + CO -> Fe + CO2', reactant='Fe2O3', m=0.1, product='Fe')
# -> product_mass = 0.0699431 kg, n_product = 1.25245 mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For CH4 + O2 -> CO2 + H2O, what mass of CO2 is produced from 16.0 g of CH4? | SOLVED | product_mass = 0.043891 kg, n_product = 0.99732 mol |
| What is the theoretical yield of Fe from 100 g of Fe2O3 in Fe2O3 + CO -> Fe + CO2? | SOLVED | product_mass = 0.0699431 kg, n_product = 1.25245 mol |
| For CH4 + O2 -> CO2 + H2O, what mass of CO2 is produced from methane? | NEEDS_INPUT | theoretical_yield needs reactant, m (the reactant whose mass is given; mass of the reactant). Example: solve('theoretica |

<a id="percent_yield"></a>

### Percent yield of a reaction — `percent_yield`

The percent yield from the mass of reactant used and the mass of product obtained.

$$
\%\,\text{yield} = \frac{m_{\text{actual}}}{m_{\text{theoretical}}} \times 100
$$

| input | meaning | condition |
|---|---|---|
| `equation` | the reaction (balanced or not) |  |
| `reactant` | the reactant whose mass is given |  |
| `m` | mass of the reactant |  |
| `product` | the product obtained |  |
| `actual` | actual mass of product obtained |  |

| output | meaning |
|---|---|
| `percent_yield` | percent yield (%) |
| `theoretical_mass` | theoretical mass of product |

- **Method:** theoretical yield from the balanced equation, then actual / theoretical x 100
- **Assumes:** the equation balances uniquely; the other reactants are in excess
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Percent yield of CaO from 100 g of CaCO₃ by CaCO3 -> CaO + CO2: the package's theoretical mass (56](https://www.ai999x.com/?ax_fig=chemistry-ai999x/percent-yield-yield-bars)

*Percent yield of CaO from 100 g of CaCO₃ by CaCO3 -> CaO + CO2: the package's theoretical mass (56.0288 g) against the stated actual mass (50 g); right: the difference not obtained (6.02882 g, derived). Marked: percent yield 89.2398 %.*

```python
chemistry_ai999x.solve('percent_yield', equation='CaCO3 -> CaO + CO2', reactant='CaCO3', m=0.1, product='CaO', actual=0.05)
# -> percent_yield = 89.2398, theoretical_mass = 0.0560288 kg   (SOLVED)
```

```python
chemistry_ai999x.solve('percent_yield', equation='N2 + H2 -> NH3', reactant='N2', m=0.028, product='NH3', actual=0.0255)
# -> percent_yield = 74.9009, theoretical_mass = 0.034045 kg   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For CaCO3 -> CaO + CO2, 100.0 g of CaCO3 gave 50.0 g of CaO. What is the percent yield? | SOLVED | percent_yield = 89.2398, theoretical_mass = 0.0560288 kg |
| In N2 + H2 -> NH3, 28.0 g of N2 produced 25.5 g of NH3. Calculate the percentage yield. | SOLVED | percent_yield = 74.9009, theoretical_mass = 0.034045 kg |

<a id="ideal_gas_pressure"></a>

### Ideal gas law: pressure — `ideal_gas_pressure`

The pressure of an ideal gas from its amount, volume and temperature (PV = nRT).

$$
P = \frac{nRT}{V}
$$

| input | meaning | condition |
|---|---|---|
| `n` | amount of gas |  |
| `V` | volume |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `P` | pressure |
| `P_atm` | pressure in atm |

- **Method:** P = nRT / V
- **Assumes:** n, V, $T > 0$ (T in kelvin)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Isotherms of 2 mol of ideal gas: pressure against volume at T = 200, 300 and 400 K, each computed by the package at 61 volumes from 4 L to 40 L](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-pressure-isotherms)

*Isotherms of 2 mol of ideal gas: pressure against volume at T = 200, 300 and 400 K, each computed by the package at 61 volumes from 4 L to 40 L. Marked: the worked example, V = 10 L at 300 K, P = 498.868 kPa.*

```python
chemistry_ai999x.solve('ideal_gas_pressure', n=2.0, V=0.01, T=300.0)
# -> P = 498868 Pa, P_atm = 4.92344   (SOLVED)
```

```python
chemistry_ai999x.solve('ideal_gas_pressure', n=0.5, V=0.005, T=298.15)
# -> P = 247896 Pa, P_atm = 2.44654   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pressure of 2.0 mol of gas in 10.0 L at 300 K? | SOLVED | P = 498868 Pa, P_atm = 4.92344 |
| Find the pressure exerted by 0.50 mol of an ideal gas in a 5.0 L container at 25 °C. | SOLVED | P = 247896 Pa, P_atm = 2.44654 |
| What is the pressure of 2.0 mol of gas in 10.0 L? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="ideal_gas_volume"></a>

### Ideal gas law: volume — `ideal_gas_volume`

The volume of an ideal gas from its amount, pressure and temperature.

$$
V = \frac{nRT}{P}
$$

| input | meaning | condition |
|---|---|---|
| `n` | amount of gas |  |
| `P` | pressure |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `V` | volume |
| `V_L` | volume in litres |

- **Method:** V = nRT / P
- **Assumes:** n, P, $T > 0$ (T in kelvin)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Volume of 1 mol of ideal gas against temperature (isobars) at 50](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-volume-isobars)

*Volume of 1 mol of ideal gas against temperature (isobars) at 50.66, 101.325 and 202.65 kPa, computed by the package at 51 temperatures from 5 K to 500 K: straight lines through 0 K. Marked: the worked example, 273.15 K and 101.325 kPa, V = 22.414 L (the molar volume at 0 °C and 1 atm).*

```python
chemistry_ai999x.solve('ideal_gas_volume', n=1.0, P=101325.0, T=273.15)
# -> V = 0.022414 m^3, V_L = 22.414   (SOLVED)
```

```python
chemistry_ai999x.solve('ideal_gas_volume', n=0.25, P=202650.0, T=350.0)
# -> V = 0.00359001 m^3, V_L = 3.59001   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What volume does 1.00 mol of gas occupy at 1.00 atm and 273.15 K? | SOLVED | V = 0.022414 m^3, V_L = 22.414 |
| Calculate the volume of 0.250 mol of an ideal gas at 2.00 atm and 350 K. | SOLVED | V = 0.00359001 m^3, V_L = 3.59001 |

<a id="ideal_gas_moles"></a>

### Ideal gas law: amount — `ideal_gas_moles`

The amount (moles) of an ideal gas from its pressure, volume and temperature.

$$
n = \frac{PV}{RT}
$$

| input | meaning | condition |
|---|---|---|
| `P` | pressure |  |
| `V` | volume |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `n` | amount of gas |

- **Method:** $n = PV / RT$
- **Assumes:** P, V, $T > 0$ (T in kelvin)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![n (amount of gas) against T (temperature) from 150 K to 450 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-moles-curve)

*n (amount of gas) against T (temperature) from 150 K to 450 K. One curve for each value of P (pressure): 202650 Pa, 101325 Pa, 405300 Pa. Other inputs as in the worked example: V = 0.005 m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 300 K, n = 0.40622 mol.*

```python
chemistry_ai999x.solve('ideal_gas_moles', P=202650.0, V=0.005, T=300.0)
# -> n = 0.40622 mol   (SOLVED)
```

```python
chemistry_ai999x.solve('ideal_gas_moles', P=101325.0, V=0.0224, T=273.15)
# -> n = 0.999377 mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many moles of gas are in a 5.00 L flask at 2.00 atm and 300 K? | SOLVED | n = 0.40622 mol |
| How many moles of gas occupy 22.4 L at 1.00 atm and 0 °C? | SOLVED | n = 0.999377 mol |

<a id="ideal_gas_temperature"></a>

### Ideal gas law: temperature — `ideal_gas_temperature`

The temperature of an ideal gas from its pressure, volume and amount.

$$
T = \frac{PV}{nR}
$$

| input | meaning | condition |
|---|---|---|
| `P` | pressure |  |
| `V` | volume |  |
| `n` | amount of gas |  |

| output | meaning |
|---|---|
| `T` | temperature |
| `T_C` | temperature in °C |

- **Method:** $T = PV / nR$
- **Assumes:** P, V, $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Which isotherm passes through the state P = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-temperature-isotherms)

*Which isotherm passes through the state P = 101.325 kPa, V = 50 L of 2 mol? The package returns T = 304.665 K; drawn: that isotherm and those at half and twice the temperature, each from the package's ideal-gas pressure at 61 volumes from 10 L to 200 L. Marked: the state, T = 304.665 K.*

```python
chemistry_ai999x.solve('ideal_gas_temperature', P=101325.0, V=0.05, n=2.0)
# -> T = 304.665 K, T_C = 31.5149   (SOLVED)
```

```python
chemistry_ai999x.solve('ideal_gas_temperature', P=150000.0, V=0.002, n=0.1)
# -> T = 360.817 K, T_C = 87.6671   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| At what temperature does 2.00 mol of gas occupy 50.0 L at 1.00 atm? | SOLVED | T = 304.665 K, T_C = 31.5149 |
| Find the temperature of 0.100 mol of gas in a 2.00 L vessel at 150 kPa. | SOLVED | T = 360.817 K, T_C = 87.6671 |

<a id="ideal_gas_mass"></a>

### Mass of a gas sample (ideal gas) — `ideal_gas_mass`

The mass of a named gas that fills a volume at a given pressure and temperature.

$$
m = \frac{PV}{RT}\,M
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the gas |  |
| `P` | pressure |  |
| `V` | volume |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `m` | mass of gas |
| `n` | amount of gas |

- **Method:** $n = PV/RT$, then $m = nM$
- **Assumes:** P, V, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Mass of 5 L of different gases at 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-mass-by-gas)

*Mass of 5 L of different gases at 101.325 kPa and 300 K, each from the package (the same amount of gas, so the mass follows the molar mass). Highlighted: the worked example, CO₂, 8.93867 g.*

![m (mass of gas) against T (temperature) from 150 K to 450 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-mass-curve)

*m (mass of gas) against T (temperature) from 150 K to 450 K. One curve for each value of P (pressure): 101325 Pa, 50662.5 Pa, 202650 Pa. Other inputs as in the worked example: formula = CO2, V = 0.005 m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 300 K, m = 0.00893867 kg.*

```python
chemistry_ai999x.solve('ideal_gas_mass', formula='CO2', P=101325.0, V=0.005, T=300.0)
# -> m = 0.00893867 kg, n = 0.20311 mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What mass of CO2 occupies 5.00 L at 1.00 atm and 300 K? | SOLVED | m = 0.00893867 kg, n = 0.20311 mol |
| How many grams of N2 fill a 10.0 L tank at 2.00 atm and 25 °C? | SOLVED | m = 0.0229009 kg, n = 0.817481 mol |

<a id="gas_density"></a>

### Density of an ideal gas — `gas_density`

The density of a named gas at a given pressure and temperature (ideal gas).

$$
\rho = \frac{PM}{RT}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the gas |  |
| `P` | pressure |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `rho` | density |

- **Method:** $\rho = PM / RT$
- **Assumes:** P, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Density of different gases at 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gas-density-by-gas)

*Density of different gases at 101.325 kPa and 298 K, each returned by the package (ideal gas: density proportional to molar mass). Highlighted: the worked example CO₂, 1.79973 kg/m³; air is about 1.2 kg/m³ at these conditions.*

![rho (density) against T (temperature) from 149 K to 447 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gas-density-curve)

*rho (density) against T (temperature) from 149 K to 447 K. One curve for each value of P (pressure): 101325 Pa, 50662.5 Pa, 202650 Pa. Other inputs as in the worked example: formula = CO2. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 298 K, rho = 1.79973 kg/m^3.*

```python
chemistry_ai999x.solve('gas_density', formula='CO2', P=101325.0, T=298.0)
# -> rho = 1.79973 kg/m^3   (SOLVED)
```

```python
chemistry_ai999x.solve('gas_density', formula='N2', P=200000.0, T=350.0)
# -> rho = 1.92532 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the density of CO2 at 1.00 atm and 298 K? | SOLVED | rho = 1.79973 kg/m^3 |
| Calculate the density of N2 at 200 kPa and 350 K. | SOLVED | rho = 1.92532 kg/m^3 |

<a id="molar_mass_from_gas_density"></a>

### Molar mass from gas density — `molar_mass_from_gas_density`

The molar mass of a gas from its density at a known pressure and temperature.

$$
M = \frac{\rho R T}{P}
$$

| input | meaning | condition |
|---|---|---|
| `rho` | gas density |  |
| `P` | pressure |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `M` | molar mass |
| `M_g` | molar mass in g/mol |

- **Method:** $M = \rho$ R T / P
- **Assumes:** rho, P, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (molar mass) against P (pressure) from 50662](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-gas-density-curve)

*M (molar mass) against P (pressure) from 50662.5 Pa to 151988 Pa. One curve for each value of rho (gas density): 1.25 kg/m^3, 0.625 kg/m^3, 2.5 kg/m^3. Other inputs as in the worked example: T = 273.0 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, P = 101325 Pa, M = 0.0280021 kg/mol.*

```python
chemistry_ai999x.solve('molar_mass_from_gas_density', rho=1.25, P=101325.0, T=273.0)
# -> M = 0.0280021 kg/mol, M_g = 28.0021   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A gas has a density of 1.25 g/L at 1.00 atm and 273 K. What is its molar mass? | SOLVED | M = 0.0280021 kg/mol, M_g = 28.0021 |
| Find the molar mass of a gas whose density is 2.0 g/L at 0.95 atm and 300 K. | SOLVED | M = 0.0518257 kg/mol, M_g = 51.8257 |

<a id="molar_mass_from_gas_sample"></a>

### Molar mass from a gas sample — `molar_mass_from_gas_sample`

The molar mass of a gas from a sample's mass, volume, pressure and temperature.

$$
M = \frac{mRT}{PV}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass of the sample |  |
| `V` | volume |  |
| `P` | pressure |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `M` | molar mass |
| `M_g` | molar mass in g/mol |

- **Method:** M = mRT / PV
- **Assumes:** m, V, P, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (molar mass) against V (volume) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-gas-sample-curve)

*M (molar mass) against V (volume) from 0.00025 m^3 to 0.00075 m^3. One curve for each value of m (mass of the sample): 0.001 kg, 0.0005 kg, 0.002 kg. Other inputs as in the worked example: P = 101325.0 Pa, T = 300.0 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, V = 0.0005 m^3, M = 0.0492344 kg/mol.*

```python
chemistry_ai999x.solve('molar_mass_from_gas_sample', m=0.001, V=0.0005, P=101325.0, T=300.0)
# -> M = 0.0492344 kg/mol, M_g = 49.2344   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A 1.00 g sample of a gas occupies 0.500 L at 1.00 atm and 300 K. What is its molar mass? | SOLVED | M = 0.0492344 kg/mol, M_g = 49.2344 |
| A 2.50 g sample of gas fills 1.20 L at 750 torr and 25 °C. Find its molar mass. | SOLVED | M = 0.0516492 kg/mol, M_g = 51.6492 |

<a id="boyle_final_volume"></a>

### Boyle's law: final volume — `boyle_final_volume`

The volume of a gas after a pressure change at constant temperature $(P1V1 = P2V2)$.

$$
V_2 = \frac{P_1 V_1}{P_2}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | initial pressure |  |
| `V1` | initial volume |  |
| `P2` | final pressure |  |

| output | meaning |
|---|---|
| `V2` | final volume |

- **Method:** $V_{2} = P_{1}$ $V_{1} / P_{2}$
- **Assumes:** P1, V1, $P_{2} > 0$; constant temperature and amount
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Boyle's law on a P–V diagram: the isotherm through state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-volume-isotherm)

*Boyle's law on a P–V diagram: the isotherm through state 1 (P1 = 101.325 kPa, V1 = 2 L), computed by the package at 61 volumes; arrow: raising the pressure to P2 = 303.975 kPa (the worked example). Marked: the returned V2 = 0.666667 L.*

```python
chemistry_ai999x.solve('boyle_final_volume', P1=101325.0, V1=0.002, P2=303975.0)
# -> V2 = 0.000666667 m^3   (SOLVED)
```

```python
chemistry_ai999x.solve('boyle_final_volume', P1=100000.0, V1=0.005, P2=250000.0)
# -> V2 = 0.002 m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A gas occupies 2.00 L at 1.00 atm. What is its volume when compressed to 3.00 atm at constant temperature? | SOLVED | V2 = 0.000666667 m^3 |
| P1 = 100 kPa, V1 = 5.0 L, P2 = 250 kPa. Find the final volume (constant temperature). | SOLVED | V2 = 0.002 m^3 |
| A gas at 1.00 atm and 2.00 atm occupies 2.00 L. What is its volume? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="boyle_final_pressure"></a>

### Boyle's law: final pressure — `boyle_final_pressure`

The pressure of a gas after a volume change at constant temperature.

$$
P_2 = \frac{P_1 V_1}{V_2}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | initial pressure |  |
| `V1` | initial volume |  |
| `V2` | final volume |  |

| output | meaning |
|---|---|
| `P2` | final pressure |

- **Method:** $P_{2} = P_{1}$ $V_{1} / V_{2}$
- **Assumes:** P1, V1, $V_{2} > 0$; constant temperature and amount
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Boyle's law on a P–V diagram: the isotherm through state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-pressure-isotherm)

*Boyle's law on a P–V diagram: the isotherm through state 1 (P1 = 101.325 kPa, V1 = 4 L), each point the package's final pressure at 61 volumes from 0.6 to 6 L; arrow: the compression to V2 = 1 L (the worked example). Marked: P2 = 405.3 kPa.*

```python
chemistry_ai999x.solve('boyle_final_pressure', P1=101325.0, V1=0.004, V2=0.001)
# -> P2 = 405300 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A gas at 1.00 atm occupies 4.00 L. What is the new pressure if it is compressed to 1.00 L at constant temperature? | SOLVED | P2 = 405300 Pa |
| P1 = 2.0 atm, V1 = 3.0 L, V2 = 6.0 L. Find the final pressure. | SOLVED | P2 = 101325 Pa |

<a id="charles_final_volume"></a>

### Charles's law: final volume — `charles_final_volume`

The volume of a gas after a temperature change at constant pressure $(V_{1}/T_{1} = V_{2}/T_{2})$.

$$
V_2 = V_1 \frac{T_2}{T_1}
$$

| input | meaning | condition |
|---|---|---|
| `V1` | initial volume |  |
| `T1` | initial temperature |  |
| `T2` | final temperature |  |

| output | meaning |
|---|---|
| `V2` | final volume |

- **Method:** $V_{2} = V_{1}$ $T_{2} / T_{1}$ (kelvin)
- **Assumes:** $V_{1} > 0$, T1, $T_{2} > 0$ K; constant pressure and amount
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Charles's law: volume against absolute temperature at constant pressure through state 1 (V1 = 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/charles-final-volume-state-line)

*Charles's law: volume against absolute temperature at constant pressure through state 1 (V1 = 2.5 L, T1 = 298.15 K), the package's final volume at 41 temperatures from 200 K to 400 K; dashed: the same law toward 0 K, where the volume would vanish. Arrow: heating to T2 = 348.15 K. Marked: V2 = 2.91925 L.*

```python
chemistry_ai999x.solve('charles_final_volume', V1=0.0025, T1=298.15, T2=348.15)
# -> V2 = 0.00291925 m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A balloon holds 2.50 L at 25 °C. What is its volume when heated to 75 °C at constant pressure? | SOLVED | V2 = 0.00291925 m^3 |
| V1 = 1.0 L, T1 = 300 K, T2 = 600 K. Find the final volume. | SOLVED | V2 = 0.002 m^3 |

<a id="gay_lussac_final_pressure"></a>

### Gay-Lussac's law: final pressure — `gay_lussac_final_pressure`

The pressure of a gas after a temperature change at constant volume $(P_{1}/T_{1} = P_{2}/T_{2})$.

$$
P_2 = P_1 \frac{T_2}{T_1}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | initial pressure |  |
| `T1` | initial temperature |  |
| `T2` | final temperature |  |

| output | meaning |
|---|---|
| `P2` | final pressure |

- **Method:** $P_{2} = P_{1}$ $T_{2} / T_{1}$ (kelvin)
- **Assumes:** $P_{1} > 0$, T1, $T_{2} > 0$ K; constant volume and amount
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Gay-Lussac's law: pressure against absolute temperature at constant volume through state 1 (P1 = 222](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gay-lussac-final-pressure-state-line)

*Gay-Lussac's law: pressure against absolute temperature at constant volume through state 1 (P1 = 222.915 kPa, T1 = 293.15 K, a tyre-like example), the package's final pressure at 41 temperatures from 200 K to 400 K; dashed: toward 0 K. Arrow: warming to T2 = 318.15 K. Marked: P2 = 241.925 kPa.*

```python
chemistry_ai999x.solve('gay_lussac_final_pressure', P1=222915.00000000003, T1=293.15, T2=318.15)
# -> P2 = 241925 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A tyre is at 2.20 atm at 20 °C. What is the pressure when it warms to 45 °C at constant volume? | SOLVED | P2 = 241925 Pa |
| P1 = 100 kPa, T1 = 300 K, T2 = 450 K. Find the final pressure. | SOLVED | P2 = 150000 Pa |

<a id="combined_gas_final_volume"></a>

### Combined gas law: final volume — `combined_gas_final_volume`

The volume of a fixed amount of gas after its pressure and temperature change.

$$
V_2 = \frac{P_1 V_1 T_2}{T_1 P_2}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | initial pressure |  |
| `V1` | initial volume |  |
| `T1` | initial temperature |  |
| `P2` | final pressure |  |
| `T2` | final temperature |  |

| output | meaning |
|---|---|
| `V2` | final volume |

- **Method:** $P1V1/T_{1} = P2V2/T_{2}$ solved for V2
- **Assumes:** all values > 0 (temperatures in kelvin); fixed amount of gas
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The combined gas law on a P–V diagram: state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-volume-pv-states)

*The combined gas law on a P–V diagram: state 1 (P1 = 101.325 kPa, V1 = 4 L, T1 = 300 K) lies on the isotherm T1 and state 2 on the isotherm T2; each isotherm is P·V constant through its state, drawn with the package's Boyle relation at 61 volumes. Stated: P2 = 202.65 kPa, T2 = 400 K. Marked: the returned V2 = 2.667 L.*

![V2 (final volume) against T1 (initial temperature) from 150 K to 450 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-volume-curve)

*V2 (final volume) against T1 (initial temperature) from 150 K to 450 K. One curve for each value of P1 (initial pressure): 101325 Pa, 50662.5 Pa, 202650 Pa. Other inputs as in the worked example: V1 = 0.004 m^3, P2 = 202650.0 Pa, T2 = 400.0 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T1 = 300 K, V2 = 0.00266667 m^3.*

```python
chemistry_ai999x.solve('combined_gas_final_volume', P1=101325.0, V1=0.004, T1=300.0, P2=202650.0, T2=400.0)
# -> V2 = 0.00266667 m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A gas occupies 4.00 L at 1.00 atm and 300 K. What is its volume at 2.00 atm and 400 K? | SOLVED | V2 = 0.00266667 m^3 |
| P1 = 1.0 atm, V1 = 2.0 L, T1 = 273 K, P2 = 0.5 atm, T2 = 546 K. Find the final volume. | SOLVED | V2 = 0.008 m^3 |

<a id="combined_gas_final_pressure"></a>

### Combined gas law: final pressure — `combined_gas_final_pressure`

The pressure of a fixed amount of gas after its volume and temperature change.

$$
P_2 = \frac{P_1 V_1 T_2}{T_1 V_2}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | initial pressure |  |
| `V1` | initial volume |  |
| `T1` | initial temperature |  |
| `V2` | final volume |  |
| `T2` | final temperature |  |

| output | meaning |
|---|---|
| `P2` | final pressure |

- **Method:** $P1V1/T_{1} = P2V2/T_{2}$ solved for P2
- **Assumes:** all values > 0 (temperatures in kelvin); fixed amount of gas
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The combined gas law on a P–V diagram: state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-pressure-pv-states)

*The combined gas law on a P–V diagram: state 1 (P1 = 101.325 kPa, V1 = 3 L, T1 = 300 K) lies on the isotherm T1 and state 2 on the isotherm T2; each isotherm is P·V constant through its state, drawn with the package's Boyle relation at 61 volumes. Stated: V2 = 1.5 L, T2 = 450 K. Marked: the returned P2 = 304 kPa.*

![P2 (final pressure) against T1 (initial temperature) from 150 K to 450 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-pressure-curve)

*P2 (final pressure) against T1 (initial temperature) from 150 K to 450 K. One curve for each value of P1 (initial pressure): 101325 Pa, 50662.5 Pa, 202650 Pa. Other inputs as in the worked example: V1 = 0.003 m^3, V2 = 0.0015 m^3, T2 = 450.0 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T1 = 300 K, P2 = 303975 Pa.*

```python
chemistry_ai999x.solve('combined_gas_final_pressure', P1=101325.0, V1=0.003, T1=300.0, V2=0.0015, T2=450.0)
# -> P2 = 303975 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A gas at 1.00 atm and 300 K occupies 3.00 L. Find the final pressure if V2 = 1.50 L and T2 = 450 K. | SOLVED | P2 = 303975 Pa |
| P1 = 100 kPa, V1 = 10 L, T1 = 300 K, V2 = 5 L, T2 = 330 K. What is the final pressure? | SOLVED | P2 = 220000 Pa |

<a id="combined_gas_final_temperature"></a>

### Combined gas law: final temperature — `combined_gas_final_temperature`

The temperature of a fixed amount of gas after its pressure and volume change.

$$
T_2 = \frac{P_2 V_2 T_1}{P_1 V_1}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | initial pressure |  |
| `V1` | initial volume |  |
| `T1` | initial temperature |  |
| `P2` | final pressure |  |
| `V2` | final volume |  |

| output | meaning |
|---|---|
| `T2` | final temperature |

- **Method:** $P1V1/T_{1} = P2V2/T_{2}$ solved for T2
- **Assumes:** all values > 0 (temperatures in kelvin); fixed amount of gas
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The combined gas law on a P–V diagram: state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-temperature-pv-states)

*The combined gas law on a P–V diagram: state 1 (P1 = 101.325 kPa, V1 = 2 L, T1 = 300 K) lies on the isotherm T1 and state 2 on the isotherm T2; each isotherm is P·V constant through its state, drawn with the package's Boyle relation at 61 volumes. Stated: P2 = 202.65 kPa, V2 = 1.5 L. Marked: the returned T2 = 450 K.*

![T2 (final temperature) against P1 (initial pressure) from 50662](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-temperature-curve)

*T2 (final temperature) against P1 (initial pressure) from 50662.5 Pa to 151988 Pa. One curve for each value of V1 (initial volume): 0.002 m^3, 0.001 m^3, 0.004 m^3. Other inputs as in the worked example: T1 = 300.0 K, P2 = 202650.0 Pa, V2 = 0.0015 m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, P1 = 101325 Pa, T2 = 450 K.*

```python
chemistry_ai999x.solve('combined_gas_final_temperature', P1=101325.0, V1=0.002, T1=300.0, P2=202650.0, V2=0.0015)
# -> T2 = 450 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| P1 = 1.00 atm, V1 = 2.00 L, T1 = 300 K, P2 = 2.00 atm, V2 = 1.50 L. Find the final temperature. | SOLVED | T2 = 450 K |
| A gas at 100 kPa, 5.0 L and 290 K is changed to 150 kPa and 4.0 L. What is the final temperature? | SOLVED | T2 = 348 K |

<a id="partial_pressure"></a>

### Partial pressure (Dalton's law) — `partial_pressure`

The partial pressure of a gas in a mixture from its mole fraction and the total pressure.

$$
p_i = x_i P_{\text{total}}
$$

| input | meaning | condition |
|---|---|---|
| `x` | mole fraction of the gas |  |
| `P_total` | total pressure |  |

| output | meaning |
|---|---|
| `p` | partial pressure |

- **Method:** $p = x$ P_total
- **Assumes:** $0 < x  \leq  1$; P_total > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Partial pressures in dry air at a total of 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/partial-pressure-air)

*Partial pressures in dry air at a total of 101.325 kPa (logarithmic axis), each from the package with a stated mole fraction: N₂ 0.7808, O₂ 0.2095, Ar 0.00934, CO₂ 0.00042 (the worked example rounds oxygen to 0.21). Marked by value: O₂ 21227.6 Pa, CO₂ 42.5565 Pa.*

![p (partial pressure) against x (mole fraction of the gas) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/partial-pressure-curve)

*p (partial pressure) against x (mole fraction of the gas) from 0.02 to 0.98. One curve for each value of P_total (total pressure): 101325 Pa, 50662.5 Pa, 202650 Pa. 41 points per curve computed by chemistry-ai999x; marked: the worked example, x = 0.21, p = 21278.2 Pa.*

```python
chemistry_ai999x.solve('partial_pressure', x=0.21, P_total=101325.0)
# -> p = 21278.2 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The mole fraction of O2 in air is 0.21. What is its partial pressure at a total pressure of 1.00 atm? | SOLVED | p = 21278.2 Pa |
| A gas has mole fraction 1.5 in a mixture at 2.0 atm. What is its partial pressure? | INVALID_INPUT | Partial pressure (Dalton's law): a mole fraction lies between 0 and 1. |

<a id="dalton_mixture"></a>

### Partial pressures in a gas mixture — `dalton_mixture`

Mole fractions and partial pressures of every gas in a mixture from the amounts and the total pressure.

$$
x_i = \frac{n_i}{\sum_j n_j},\quad p_i = x_i P
$$

| input | meaning | condition |
|---|---|---|
| `moles` | amount of each gas (mol), e.g. '2.0, 1.0, 0.5' |  |
| `P_total` | total pressure |  |

| output | meaning |
|---|---|
| `partial_pressures` | partial pressure of each gas, in the order given |
| `mole_fractions` | mole fraction of each gas |

- **Method:** mole fractions from the amounts, times the total pressure
- **Assumes:** every amount > 0; P_total > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Dalton's law: a mixture of 2, 1, 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dalton-mixture-partial-pressures)

*Dalton's law: a mixture of 2, 1, 0.5 mol of three gases at a total pressure of 354.637 kPa; each bar is a partial pressure returned by the package with its mole fraction, and the right-hand column stacks them back to the stated total.*

```python
chemistry_ai999x.solve('dalton_mixture', moles='2.0, 1.0, 0.5', P_total=354637.5)
# -> partial_pressures = [202650.0, 101325.0, 50662.5] Pa, mole_fractions = [0.5714285714285714, 0.2857142857142857, 0.14285714285714285]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A mixture of 2.0 mol N2, 1.0 mol O2 and 0.50 mol Ar has a total pressure of 3.5 atm. Find the partial pressures. | SOLVED | partial_pressures = [202650.0, 101325.0, 50662.5] Pa, mole_fractions = [0.5714285714285714, 0.2857142857142857, 0.14285714285714285] |
| A mixture of 1.0 mol He and 3.0 mol Ne is at 200 kPa. What are the partial pressures? | SOLVED | partial_pressures = [50000.0, 150000.0] Pa, mole_fractions = [0.25, 0.75] |

<a id="graham_rate_ratio"></a>

### Graham's law: ratio of effusion rates — `graham_rate_ratio`

How many times faster one gas effuses (or diffuses) than another.

$$
\frac{r_1}{r_2} = \sqrt{\frac{M_2}{M_1}}
$$

| input | meaning | condition |
|---|---|---|
| `formula_1` | first gas (numerator) |  |
| `formula_2` | second gas |  |

| output | meaning |
|---|---|
| `ratio` | rate of gas 1 / rate of gas 2 |

- **Method:** square root of the inverse ratio of molar masses
- **Assumes:** ideal gases at the same temperature and pressure
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Graham's law: how many times faster each gas effuses than O₂ (rate ratio = √(M_O₂ / M)), each ratio returned by the package](https://www.ai999x.com/?ax_fig=chemistry-ai999x/graham-rate-ratio-by-gas)

*Graham's law: how many times faster each gas effuses than O₂ (rate ratio = √(M_O₂ / M)), each ratio returned by the package. Highlighted: the worked example, H₂ against O₂, ratio 3.98397.*

```python
chemistry_ai999x.solve('graham_rate_ratio', formula_1='H2', formula_2='O2')
# -> ratio = 3.98397   (SOLVED)
```

```python
chemistry_ai999x.solve('graham_rate_ratio', formula_1='He', formula_2='CH4')
# -> ratio = 2.00194   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How many times faster does H2 effuse than O2? | SOLVED | ratio = 3.98397 |
| What is the ratio of the rates of effusion of He to CH4? | SOLVED | ratio = 2.00194 |
| How many times slower does O2 effuse than H2? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="graham_molar_mass"></a>

### Graham's law: molar mass of an unknown gas — `graham_molar_mass`

The molar mass of a gas from how fast it effuses relative to a known gas.

$$
M_x = \frac{M_{\text{known}}}{(r_x / r_{\text{known}})^2}
$$

| input | meaning | condition |
|---|---|---|
| `ratio` | rate of the unknown gas / rate of the known gas |  |
| `formula_known` | formula of the known gas |  |

| output | meaning |
|---|---|
| `M` | molar mass of the unknown gas |
| `M_g` | molar mass in g/mol |

- **Method:** M_x = M_known / ratio^2
- **Assumes:** ratio > 0; same temperature and pressure
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (molar mass of the unknown gas) against ratio (rate of the unknown gas / rate of the known gas) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/graham-molar-mass-curve)

*M (molar mass of the unknown gas) against ratio (rate of the unknown gas / rate of the known gas) from 0.75 to 2.25. Other inputs as in the worked example: formula_known = O2. 31 points computed by chemistry-ai999x; marked: the worked example, ratio = 1.5, M = 0.0142213 kg/mol.*

```python
chemistry_ai999x.solve('graham_molar_mass', ratio=1.5, formula_known='O2')
# -> M = 0.0142213 kg/mol, M_g = 14.2213   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| An unknown gas effuses 1.50 times as fast as O2. What is its molar mass? | SOLVED | M = 0.0142213 kg/mol, M_g = 14.2213 |
| An unknown gas diffuses 0.50 times as fast as CH4. What is the molar mass of the unknown gas? | SOLVED | M = 0.064172 kg/mol, M_g = 64.172 |

<a id="van_der_waals_pressure"></a>

### Van der Waals equation: pressure — `van_der_waals_pressure`

The pressure of a real gas from the van der Waals equation (and the ideal-gas pressure for comparison).

$$
P = \frac{nRT}{V - nb} - \frac{a n^2}{V^2}
$$

| input | meaning | condition |
|---|---|---|
| `n` | amount of gas |  |
| `V` | volume |  |
| `T` | temperature |  |
| `a` | van der Waals a |  |
| `b` | van der Waals b |  |

| output | meaning |
|---|---|
| `P` | van der Waals pressure |
| `P_ideal` | ideal-gas pressure |
| `P_atm` | van der Waals pressure in atm |

- **Method:** the van der Waals equation solved for P
- **Assumes:** n, V, T, a, $b > 0$; $V > n$ b
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Pressure of 2 mol of gas at 300 K against volume, van der Waals (a = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/van-der-waals-pressure-real-vs-ideal)

*Pressure of 2 mol of gas at 300 K against volume, van der Waals (a = 0.368823 Pa·m⁶/mol², b = 4.2670e-05 m³/mol, CO₂-like) and ideal, both returned by the package at 71 volumes from 0.12 L to 20 L (logarithmic axes). At large volume the two agree; squeezed, attraction first lowers the pressure and the molecules' own volume then raises it. Marked: V = 5 L, P = 9.56049 bar (ideal 9.97736 bar).*

```python
chemistry_ai999x.solve('van_der_waals_pressure', n=2.0, V=0.005, T=300.0, a=0.368823, b=4.267e-05)
# -> P = 956049 Pa, P_ideal = 997736 Pa, P_atm = 9.43547   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Using the van der Waals equation, find the pressure of 2.00 mol of CO2 in 5.00 L at 300 K (a = 3.640 L^2·atm/mol^2, b = 0.04267 L/mol). | SOLVED | P = 956049 Pa, P_ideal = 997736 Pa, P_atm = 9.43547 |
| What is the van der Waals pressure of 1.00 mol of N2 in 1.00 L at 273 K with a = 1.370 L^2 bar/mol^2 and b = 0.0387 L/mol? | SOLVED | P = 2.22423e+06 Pa, P_ideal = 2.26985e+06 Pa, P_atm = 21.9514 |
| Using the van der Waals equation, find the pressure of 2.00 mol of CO2 in 5.00 L at 300 K. | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="rms_speed"></a>

### Root-mean-square speed of gas molecules — `rms_speed`

The root-mean-square speed of the molecules of an ideal gas at a temperature.

$$
u_{\text{rms}} = \sqrt{\frac{3RT}{M}}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the gas |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `u_rms` | root-mean-square speed |

- **Method:** u_rms = sqrt(3RT/M), M in kg/mol
- **Assumes:** $T > 0$ K; ideal gas
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Rms speed of gas molecules against temperature for four gases, each computed by the package at 46 temperatures from 100 K to 1000 K (it grows as √(T/M))](https://www.ai999x.com/?ax_fig=chemistry-ai999x/rms-speed-by-gas)

*Rms speed of gas molecules against temperature for four gases, each computed by the package at 46 temperatures from 100 K to 1000 K (it grows as √(T/M)). At 300 K: H₂ 1927, He 1367, N₂ 516.8, CO₂ 412.4 m/s. Marked: the worked example, N₂ at 300 K, 516.834 m/s.*

```python
chemistry_ai999x.solve('rms_speed', formula='N2', T=300.0)
# -> u_rms = 516.834 m/s   (SOLVED)
```

```python
chemistry_ai999x.solve('rms_speed', formula='O2', T=298.15)
# -> u_rms = 482.096 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the rms speed of N2 molecules at 300 K? | SOLVED | u_rms = 516.834 m/s |
| Calculate the root-mean-square speed of O2 at 25 °C. | SOLVED | u_rms = 482.096 m/s |

<a id="mean_speed"></a>

### Mean speed of gas molecules — `mean_speed`

The mean (average) speed of the molecules of an ideal gas (Maxwell-Boltzmann distribution).

$$
\bar{u} = \sqrt{\frac{8RT}{\pi M}}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the gas |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `u_mean` | mean speed |

- **Method:** mean of the Maxwell-Boltzmann speed distribution
- **Assumes:** $T > 0$ K; ideal gas
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Mean speed of gas molecules against temperature for four gases, each computed by the package at 46 temperatures from 100 K to 1000 K (it grows as √(T/M))](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mean-speed-by-gas)

*Mean speed of gas molecules against temperature for four gases, each computed by the package at 46 temperatures from 100 K to 1000 K (it grows as √(T/M)). At 300 K: H₂ 1775, He 1260, N₂ 476.2, CO₂ 379.9 m/s. Marked: the worked example, CO₂ at 300 K, 379.907 m/s.*

```python
chemistry_ai999x.solve('mean_speed', formula='CO2', T=300.0)
# -> u_mean = 379.907 m/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the mean speed of CO2 molecules at 300 K? | SOLVED | u_mean = 379.907 m/s |
| Find the average speed of He atoms at 1000 K. | SOLVED | u_mean = 2299.82 m/s |

<a id="molarity_from_mass"></a>

### Molarity from a dissolved mass — `molarity_from_mass`

The molar concentration of a solution made by dissolving a mass of a compound to a final volume.

$$
c = \frac{m}{M V}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the solute |  |
| `m` | mass of solute |  |
| `V` | volume of solution |  |

| output | meaning |
|---|---|
| `c` | molar concentration |
| `c_molL` | concentration in mol/L |

- **Method:** moles of solute (m / M) over the volume of solution
- **Assumes:** m, $V > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![c (molar concentration) against V (volume of solution) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molarity-from-mass-curve)

*c (molar concentration) against V (volume of solution) from 0.000125 m^3 to 0.000375 m^3. One curve for each value of m (mass of solute): 0.00585 kg, 0.002925 kg, 0.0117 kg. Other inputs as in the worked example: formula = NaCl. 31 points per curve computed by chemistry-ai999x; marked: the worked example, V = 0.00025 m^3, c = 400.39 mol/m^3.*

![c (molar concentration) over m (mass of solute) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molarity-from-mass-field)

*c (molar concentration) over m (mass of solute) from 0.002925 kg to 0.008775 kg and V (volume of solution) from 0.000125 m^3 to 0.000375 m^3: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 133.463 mol/m^3 to dark 1201.17 mol/m^3); iso-lines at c = 200, 400, 600, 800, 1000 mol/m^3. Other inputs as in the worked example: formula = NaCl. Marked: the worked example, m = 0.00585 kg, V = 0.00025 m^3, c = 400.39 mol/m^3.*

```python
chemistry_ai999x.solve('molarity_from_mass', formula='NaCl', m=0.00585, V=0.00025)
# -> c = 400.39 mol/m^3, c_molL = 0.40039   (SOLVED)
```

```python
chemistry_ai999x.solve('molarity_from_mass', formula='C6H12O6', m=0.09, V=0.5)
# -> c = 0.999134 mol/m^3, c_molL = 0.000999134   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the molarity of a solution made by dissolving 5.85 g of NaCl in 250 mL of solution? | SOLVED | c = 400.39 mol/m^3, c_molL = 0.40039 |
| Find the concentration of a solution of 90.0 g of C6H12O6 in 0.500 L of solution. | SOLVED | c = 999.134 mol/m^3, c_molL = 0.999134 |

<a id="solute_mass_for_solution"></a>

### Mass of solute for a solution — `solute_mass_for_solution`

The mass of a compound needed to prepare a volume of solution of a given molarity.

$$
m = c V M
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the solute |  |
| `c` | molar concentration |  |
| `V` | volume of solution |  |

| output | meaning |
|---|---|
| `m` | mass of solute |
| `m_g` | mass in grams |

- **Method:** $m = c$ V M
- **Assumes:** c, $V > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![m (mass of solute) against c (molar concentration) from 50 mol/m^3 to 150 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/solute-mass-for-solution-curve)

*m (mass of solute) against c (molar concentration) from 50 mol/m^3 to 150 mol/m^3. One curve for each value of V (volume of solution): 0.00025 m^3, 0.000125 m^3, 0.0005 m^3. Other inputs as in the worked example: formula = NaCl. 31 points per curve computed by chemistry-ai999x; marked: the worked example, c = 100 mol/m^3, m = 0.00146108 kg.*

![m (mass of solute) over c (molar concentration) from 50 mol/m^3 to 150 mol/m^3 and V (volume of solution) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/solute-mass-for-solution-field)

*m (mass of solute) over c (molar concentration) from 50 mol/m^3 to 150 mol/m^3 and V (volume of solution) from 0.000125 m^3 to 0.000375 m^3: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 0.000365269 kg to dark 0.00328742 kg); iso-lines at m = 5.00e-4, 0.001, 0.0015, 0.002, 0.0025, 0.003 kg. Other inputs as in the worked example: formula = NaCl. Marked: the worked example, c = 100 mol/m^3, V = 0.00025 m^3, m = 0.00146108 kg.*

```python
chemistry_ai999x.solve('solute_mass_for_solution', formula='NaCl', c=100.0, V=0.00025)
# -> m = 0.00146108 kg, m_g = 1.46108   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What mass of NaCl is needed to prepare 250 mL of 0.100 M solution? | SOLVED | m = 0.00146108 kg, m_g = 1.46108 |
| How many grams of KMnO4 are needed to make 2.00 L of a 0.0200 M solution? | SOLVED | m = 0.00632128 kg, m_g = 6.32128 |

<a id="dilution_final_concentration"></a>

### Dilution: final concentration — `dilution_final_concentration`

The concentration after diluting a volume of solution to a larger volume $(c1V1 = c2V2)$.

$$
c_2 = \frac{c_1 V_1}{V_2}
$$

| input | meaning | condition |
|---|---|---|
| `c1` | initial concentration |  |
| `V1` | initial volume |  |
| `V2` | final volume |  |

| output | meaning |
|---|---|
| `c2` | final concentration |
| `c2_molL` | in mol/L |

- **Method:** moles of solute are conserved: c1 $V_{1} = c_{2}$ V2
- **Assumes:** c1, V1, $V_{2} > 0$; $V_{2}  \geq  V_{1}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![c2 (final concentration) against V2 (final volume) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dilution-final-concentration-curve)

*c2 (final concentration) against V2 (final volume) from 0.25 m^3 to 0.75 m^3. One curve for each value of c1 (initial concentration): 6000 mol/m^3, 3000 mol/m^3, 12000 mol/m^3. Other inputs as in the worked example: V1 = 0.025 m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, V2 = 0.5 m^3, c2 = 300 mol/m^3.*

```python
chemistry_ai999x.solve('dilution_final_concentration', c1=6000.0, V1=0.025, V2=0.5)
# -> c2 = 300 mol/m^3, c2_molL = 0.3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| 25.0 mL of 6.0 M HCl is diluted to 500 mL. What is the new concentration? | SOLVED | c2 = 300 mol/m^3, c2_molL = 0.3 |
| A 10.0 mL sample of 2.00 M NaOH is diluted to a total volume of 250 mL. Find the final molarity. | SOLVED | c2 = 80 mol/m^3, c2_molL = 0.08 |

<a id="dilution_volume"></a>

### Dilution: volume of stock needed — `dilution_volume`

The volume of a stock solution needed to prepare a volume of a more dilute solution.

$$
V_1 = \frac{c_2 V_2}{c_1}
$$

| input | meaning | condition |
|---|---|---|
| `c1` | stock concentration |  |
| `c2` | target concentration |  |
| `V2` | target volume |  |

| output | meaning |
|---|---|
| `V1` | volume of stock solution |
| `V1_mL` | in mL |

- **Method:** c1 $V_{1} = c_{2}$ V2 solved for V1
- **Assumes:** $c_{1}  \geq  c_{2} > 0$; $V_{2} > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V1 (volume of stock solution) against c1 (stock concentration) from 6000 mol/m^3 to 18000 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dilution-volume-curve)

*V1 (volume of stock solution) against c1 (stock concentration) from 6000 mol/m^3 to 18000 mol/m^3. One curve for each value of c2 (target concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. Other inputs as in the worked example: V2 = 0.5 m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, c1 = 12000 mol/m^3, V1 = 0.00416667 m^3.*

```python
chemistry_ai999x.solve('dilution_volume', c1=12000.0, c2=100.0, V2=0.5)
# -> V1 = 0.00416667 m^3, V1_mL = 4166.67   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What volume of 12.0 M HCl is needed to prepare 500 mL of 0.100 M HCl? | SOLVED | V1 = 4.16667e-06 m^3, V1_mL = 4.16667 |
| C1 = 2.0 M, C2 = 0.50 M, V2 = 100 mL. What volume of stock solution is needed? | SOLVED | V1 = 2.5e-05 m^3, V1_mL = 25 |

<a id="molality"></a>

### Molality of a solution — `molality`

The molality (moles of solute per kilogram of solvent) from the masses of solute and solvent.

$$
b = \frac{m_{\text{solute}} / M}{m_{\text{solvent}}}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the solute |  |
| `m_solute` | mass of solute |  |
| `m_solvent` | mass of solvent |  |

| output | meaning |
|---|---|
| `b` | molality |

- **Method:** moles of solute per kilogram of solvent
- **Assumes:** masses > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![b (molality) against m_solvent (mass of solvent) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molality-curve)

*b (molality) against m_solvent (mass of solvent) from 0.25 kg to 0.75 kg. One curve for each value of m_solute (mass of solute): 0.01 kg, 0.005 kg, 0.02 kg. Other inputs as in the worked example: formula = NaCl. 31 points per curve computed by chemistry-ai999x; marked: the worked example, m_solvent = 0.5 kg, b = 0.342214 mol/kg.*

![b (molality) over m_solute (mass of solute) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molality-field)

*b (molality) over m_solute (mass of solute) from 0.005 kg to 0.015 kg and m_solvent (mass of solvent) from 0.25 kg to 0.75 kg: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 0.114071 mol/kg to dark 1.02664 mol/kg); iso-lines at b = 0.2, 0.4, 0.6, 0.8 mol/kg. Other inputs as in the worked example: formula = NaCl. Marked: the worked example, m_solute = 0.01 kg, m_solvent = 0.5 kg, b = 0.342214 mol/kg.*

```python
chemistry_ai999x.solve('molality', formula='NaCl', m_solute=0.01, m_solvent=0.5)
# -> b = 0.342214 mol/kg   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the molality of a solution of 10.0 g of NaCl dissolved in 500 g of water? | SOLVED | b = 0.342214 mol/kg |
| Find the molality when 45.0 g of C6H12O6 is dissolved in 1.00 kg of water. | SOLVED | b = 0.249784 mol/kg |

<a id="mole_fraction"></a>

### Mole fraction in a solution — `mole_fraction`

The mole fractions of solute and solvent from their masses.

$$
x_{\text{solute}} = \frac{n_{\text{solute}}}{n_{\text{solute}} + n_{\text{solvent}}}
$$

| input | meaning | condition |
|---|---|---|
| `formula_solute` | formula of the solute |  |
| `m_solute` | mass of solute |  |
| `formula_solvent` | formula of the solvent |  |
| `m_solvent` | mass of solvent |  |

| output | meaning |
|---|---|
| `x_solute` | mole fraction of solute |
| `x_solvent` | mole fraction of solvent |

- **Method:** moles of each from its mass and molar mass
- **Assumes:** masses > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![x_solute (mole fraction of solute) against m_solvent (mass of solvent) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mole-fraction-curve)

*x_solute (mole fraction of solute) against m_solvent (mass of solvent) from 0.05 kg to 0.15 kg. One curve for each value of m_solute (mass of solute): 0.02 kg, 0.01 kg, 0.04 kg. Other inputs as in the worked example: formula_solute = C6H12O6, formula_solvent = H2O. 31 points per curve computed by chemistry-ai999x; marked: the worked example, m_solvent = 0.1 kg, x_solute = 0.0196072.*

![x_solute (mole fraction of solute) over m_solute (mass of solute) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mole-fraction-field)

*x_solute (mole fraction of solute) over m_solute (mass of solute) from 0.01 kg to 0.03 kg and m_solvent (mass of solvent) from 0.05 kg to 0.15 kg: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 0.0066223 to dark 0.056602); iso-lines at x_solute = 0.01, 0.02, 0.03, 0.04, 0.05. Other inputs as in the worked example: formula_solute = C6H12O6, formula_solvent = H2O. Marked: the worked example, m_solute = 0.02 kg, m_solvent = 0.1 kg, x_solute = 0.0196072.*

```python
chemistry_ai999x.solve('mole_fraction', formula_solute='C6H12O6', m_solute=0.02, formula_solvent='H2O', m_solvent=0.1)
# -> x_solute = 0.0196072, x_solvent = 0.980393   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the mole fraction of solute in a solution of 20.0 g of C6H12O6 dissolved in 100 g of H2O? | SOLVED | x_solute = 0.0196072, x_solvent = 0.980393 |
| Find the mole fractions for 46.0 g of C2H5OH in 100 g of H2O. | SOLVED | x_solute = 0.152456, x_solvent = 0.847544 |

<a id="raoult_vapor_pressure"></a>

### Vapor pressure of a solution (Raoult's law) — `raoult_vapor_pressure`

The vapor pressure of a solvent above a solution of a nonvolatile solute, from the masses of both.

$$
P = x_{\text{solvent}} P^\circ
$$

| input | meaning | condition |
|---|---|---|
| `formula_solute` | formula of the solute |  |
| `m_solute` | mass of solute |  |
| `formula_solvent` | formula of the solvent |  |
| `m_solvent` | mass of solvent |  |
| `P0` | vapor pressure of the pure solvent |  |

| output | meaning |
|---|---|
| `P` | vapor pressure of the solution |
| `dP` | vapor pressure lowering |

- **Method:** Raoult's law with the solvent mole fraction from the masses
- **Assumes:** nonvolatile, non-dissociating solute; ideal solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![P (vapor pressure of the solution) against m_solvent (mass of solvent) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/raoult-vapor-pressure-curve)

*P (vapor pressure of the solution) against m_solvent (mass of solvent) from 0.05 kg to 5 kg on a logarithmic axis. One curve for each value of P0 (vapor pressure of the pure solvent): 3173.07 Pa, 1586.54 Pa, 6346.14 Pa. Other inputs as in the worked example: formula_solute = C12H22O11, m_solute = 0.05 kg, formula_solvent = H2O. 41 points per curve computed by chemistry-ai999x; marked: the worked example, m_solvent = 0.5 kg, P = 3156.46 Pa.*

```python
chemistry_ai999x.solve('raoult_vapor_pressure', formula_solute='C12H22O11', m_solute=0.05, formula_solvent='H2O', m_solvent=0.5, P0=3173.0723684210525)
# -> P = 3156.46 Pa, dP = 16.6124 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the vapor pressure of a solution of 50.0 g of C12H22O11 in 500 g of H2O if the vapor pressure of pure water is 23.8 torr? | SOLVED | P = 3156.46 Pa, dP = 16.6124 Pa |

<a id="freezing_point_depression"></a>

### Freezing-point depression — `freezing_point_depression`

The change of the freezing point of a solvent caused by a solute (dT = i K b).

$$
\Delta T = i K b
$$

| input | meaning | condition |
|---|---|---|
| `i` | van 't Hoff factor |  |
| `K` | cryoscopic constant |  |
| `b` | molality of the solute |  |

| output | meaning |
|---|---|
| `dT` | freezing-point change |

- **Method:** $dT = i$ K b
- **Assumes:** i, K, $b > 0$; dilute solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dT (freezing-point change) against i (van 't Hoff factor) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/freezing-point-depression-curve)

*dT (freezing-point change) against i (van 't Hoff factor) from 0.5 to 1.5. One curve for each value of K (cryoscopic constant): 1.86 K*kg/mol, 0.93 K*kg/mol, 3.72 K*kg/mol. Other inputs as in the worked example: b = 0.5 mol/kg. 31 points per curve computed by chemistry-ai999x; marked: the worked example, i = 1, dT = 0.93 K.*

```python
chemistry_ai999x.solve('freezing_point_depression', i=1.0, K=1.86, b=0.5)
# -> dT = 0.93 K   (SOLVED)
```

```python
chemistry_ai999x.solve('freezing_point_depression', i=2.0, K=1.86, b=0.2)
# -> dT = 0.744 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the freezing point change of a 0.50 mol/kg aqueous solution of a nonelectrolyte (Kf = 1.86 K·kg/mol)? | SOLVED | dT = 0.93 K |
| Find the freezing point change for b = 0.20 mol/kg, i = 2, Kf = 1.86 °C/m. | SOLVED | dT = 0.744 K |
| What is the freezing point change of a 0.50 mol/kg solution with Kf = 1.86 K·kg/mol? | NEEDS_INPUT | freezing_point_depression needs i (van 't Hoff factor). Example: solve('freezing_point_depression', i=1, K='1.86 K*kg/mo |

<a id="boiling_point_elevation"></a>

### Boiling-point elevation — `boiling_point_elevation`

The change of the boiling point of a solvent caused by a solute (dT = i K b).

$$
\Delta T = i K b
$$

| input | meaning | condition |
|---|---|---|
| `i` | van 't Hoff factor |  |
| `K` | ebullioscopic constant |  |
| `b` | molality of the solute |  |

| output | meaning |
|---|---|
| `dT` | boiling-point change |

- **Method:** $dT = i$ K b
- **Assumes:** i, K, $b > 0$; dilute solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dT (boiling-point change) against i (van 't Hoff factor) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boiling-point-elevation-curve)

*dT (boiling-point change) against i (van 't Hoff factor) from 0.5 to 1.5. One curve for each value of K (ebullioscopic constant): 0.512 K*kg/mol, 0.256 K*kg/mol, 1.024 K*kg/mol. Other inputs as in the worked example: b = 0.5 mol/kg. 31 points per curve computed by chemistry-ai999x; marked: the worked example, i = 1, dT = 0.256 K.*

```python
chemistry_ai999x.solve('boiling_point_elevation', i=1.0, K=0.512, b=0.5)
# -> dT = 0.256 K   (SOLVED)
```

```python
chemistry_ai999x.solve('boiling_point_elevation', i=2.0, K=0.512, b=0.2)
# -> dT = 0.2048 K   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the boiling point change of a 0.50 mol/kg aqueous solution of a nonelectrolyte (Kb = 0.512 K·kg/mol)? | SOLVED | dT = 0.256 K |
| Find the boiling point change for b = 0.20 mol/kg, i = 2, Kb = 0.512 °C/m. | SOLVED | dT = 0.2048 K |
| What is the boiling point change of a 0.50 mol/kg solution with Kb = 0.512 K·kg/mol? | NEEDS_INPUT | boiling_point_elevation needs i (van 't Hoff factor). Example: solve('boiling_point_elevation', i=1, K='0.512 K*kg/mol', |

<a id="molar_mass_from_freezing_point"></a>

### Molar mass from freezing-point depression — `molar_mass_from_freezing_point`

The molar mass of a solute from the freezing-point depression it causes.

$$
M = \frac{i K_f m_{\text{solute}}}{\Delta T_f\, m_{\text{solvent}}}
$$

| input | meaning | condition |
|---|---|---|
| `m_solute` | mass of solute |  |
| `m_solvent` | mass of solvent |  |
| `K` | cryoscopic constant Kf |  |
| `dT` | freezing-point depression |  |
| `i` | van 't Hoff factor |  |

| output | meaning |
|---|---|
| `M` | molar mass of solute |
| `M_g` | in g/mol |

- **Method:** b = dT/(i Kf); M = m_solute / (b m_solvent)
- **Assumes:** all inputs > 0; dilute solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (molar mass of solute) against m_solvent (mass of solvent) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-freezing-point-curve)

*M (molar mass of solute) against m_solvent (mass of solvent) from 0.05 kg to 0.15 kg. One curve for each value of m_solute (mass of solute): 0.002 kg, 0.001 kg, 0.004 kg. Other inputs as in the worked example: K = 1.86 K*kg/mol, dT = 0.372 K, i = 1.0. 31 points per curve computed by chemistry-ai999x; marked: the worked example, m_solvent = 0.1 kg, M = 0.1 kg/mol.*

```python
chemistry_ai999x.solve('molar_mass_from_freezing_point', m_solute=0.002, m_solvent=0.1, K=1.86, dT=0.372, i=1.0)
# -> M = 0.1 kg/mol, M_g = 100   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Dissolving 2.00 g of a nonelectrolyte in 100 g of water lowers the freezing point by 0.372 °C (Kf = 1.86 K·kg/mol). What is the molar mass of the solute? | SOLVED | M = 0.1 kg/mol, M_g = 100 |
| A solution of 1.00 g of a nonelectrolyte in 50.0 g of water freezes at -0.200 °C (Kf = 1.86 K·kg/mol). What is the molar mass of the solute? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="osmotic_pressure"></a>

### Osmotic pressure — `osmotic_pressure`

The osmotic pressure of a dilute solution (van 't Hoff equation).

$$
\Pi = i c R T
$$

| input | meaning | condition |
|---|---|---|
| `i` | van 't Hoff factor |  |
| `c` | molar concentration |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `Pi` | osmotic pressure |
| `Pi_atm` | in atm |

- **Method:** $Pi = i$ c R T
- **Assumes:** i, c, $T > 0$; dilute solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Pi (osmotic pressure) against i (van 't Hoff factor) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/osmotic-pressure-curve)

*Pi (osmotic pressure) against i (van 't Hoff factor) from 0.5 to 1.5. One curve for each value of c (molar concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. Other inputs as in the worked example: T = 298.15 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, i = 1, Pi = 247896 Pa.*

```python
chemistry_ai999x.solve('osmotic_pressure', i=1.0, c=100.0, T=298.15)
# -> Pi = 247896 Pa, Pi_atm = 2.44654   (SOLVED)
```

```python
chemistry_ai999x.solve('osmotic_pressure', i=2.0, c=150.0, T=310.0)
# -> Pi = 773245 Pa, Pi_atm = 7.63134   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the osmotic pressure of a 0.100 M glucose solution (a nonelectrolyte) at 25 °C? | SOLVED | Pi = 247896 Pa, Pi_atm = 2.44654 |
| Find the osmotic pressure of 0.150 M NaCl at 310 K with i = 2. | SOLVED | Pi = 773245 Pa, Pi_atm = 7.63134 |
| What is the osmotic pressure of 0.150 M NaCl at 310 K? | NEEDS_INPUT | osmotic_pressure needs i (van 't Hoff factor). Example: solve('osmotic_pressure', i=1, c='100 mol/m^3', T='298.15 K'). |

<a id="molar_mass_from_osmotic_pressure"></a>

### Molar mass from osmotic pressure — `molar_mass_from_osmotic_pressure`

The molar mass of a solute (e.g. a protein) from the osmotic pressure of its solution.

$$
M = \frac{i m R T}{\Pi V}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass of solute |  |
| `V` | volume of solution |  |
| `Pi` | osmotic pressure |  |
| `T` | temperature |  |
| `i` | van 't Hoff factor |  |

| output | meaning |
|---|---|
| `M` | molar mass |
| `M_g` | in g/mol |

- **Method:** c = Pi/(i R T); M = m / (c V)
- **Assumes:** all inputs > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![M (molar mass) against V (volume of solution) from 5](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-osmotic-pressure-curve)

*M (molar mass) against V (volume of solution) from 5.0000e-05 m^3 to 0.00015 m^3. One curve for each value of m (mass of solute): 0.001 kg, 0.0005 kg, 0.002 kg. Other inputs as in the worked example: Pi = 333.30592105263156 Pa, T = 298.15 K, i = 1.0. 31 points per curve computed by chemistry-ai999x; marked: the worked example, V = 0.0001 m^3, M = 74.3748 kg/mol.*

```python
chemistry_ai999x.solve('molar_mass_from_osmotic_pressure', m=0.001, V=0.0001, Pi=333.30592105263156, T=298.15, i=1.0)
# -> M = 74.3748 kg/mol, M_g = 74374.8   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A solution of 1.00 g of a protein (a nonelectrolyte) in 100 mL of water has an osmotic pressure of 2.50 torr at 25 °C. What is the molar mass of the protein? | SOLVED | M = 74.3748 kg/mol, M_g = 74374.8 |

<a id="kp_from_kc"></a>

### Kp from Kc — `kp_from_kc`

The pressure equilibrium constant from the concentration one (Kp = Kc $(RT)^{dn}$, atm and mol/L).

$$
K_p = K_c (RT)^{\Delta n}
$$

| input | meaning | condition |
|---|---|---|
| `Kc` | concentration equilibrium constant |  |
| `T` | temperature |  |
| `dn` | change in moles of gas (products - reactants) |  |

| output | meaning |
|---|---|
| `Kp` | pressure equilibrium constant (atm basis) |

- **Method:** $Kp = Kc$ $(RT)^{dn}$ with $R = 0.082057$ L·atm/(mol·K)
- **Assumes:** Kc, $T > 0$; dn counts gaseous species only
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Kp (pressure equilibrium constant (atm basis)) against T (temperature) from 200 K to 600 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/kp-from-kc-curve)

*Kp (pressure equilibrium constant (atm basis)) against T (temperature) from 200 K to 600 K. One curve for each value of dn (change in moles of gas (products - reactants)): -2, -1, -4. Other inputs as in the worked example: Kc = 0.5. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 400 K, Kp = 0.000464104.*

```python
chemistry_ai999x.solve('kp_from_kc', Kc=0.5, T=400.0, dn=-2.0)
# -> Kp = 0.000464104   (SOLVED)
```

```python
chemistry_ai999x.solve('kp_from_kc', Kc=4.0, T=500.0, dn=1.0)
# -> Kp = 164.115   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For N2(g) + 3H2(g) ⇌ 2NH3(g), Kc = 0.50 at 400 K. What is Kp? | SOLVED | Kp = 0.000464104 |
| Kc = 4.0 at 500 K and Δn = 1. Find Kp. | SOLVED | Kp = 164.115 |
| Kc = 4.0 at 500 K. Find Kp. | NEEDS_INPUT | kp_from_kc needs dn (change in moles of gas (products - reactants)). Example: solve('kp_from_kc', Kc=0.5, T='400 K', dn= |

<a id="kc_from_kp"></a>

### Kc from Kp — `kc_from_kp`

The concentration equilibrium constant from the pressure one $(Kc = Kp / (RT)^{dn})$.

$$
K_c = \frac{K_p}{(RT)^{\Delta n}}
$$

| input | meaning | condition |
|---|---|---|
| `Kp` | pressure equilibrium constant (atm basis) |  |
| `T` | temperature |  |
| `dn` | change in moles of gas (products - reactants) |  |

| output | meaning |
|---|---|
| `Kc` | concentration equilibrium constant |

- **Method:** $Kc = Kp / (RT)^{dn}$ with $R = 0.082057$ L·atm/(mol·K)
- **Assumes:** Kp, $T > 0$; dn counts gaseous species only
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Kc (concentration equilibrium constant) against dn (change in moles of gas (products - reactants)) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/kc-from-kp-curve)

*Kc (concentration equilibrium constant) against dn (change in moles of gas (products - reactants)) from 0.5 to 1.5. One curve for each value of Kp (pressure equilibrium constant (atm basis)): 1.8, 0.9, 3.6. Other inputs as in the worked example: T = 523.0 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, dn = 1, Kc = 0.0419424.*

```python
chemistry_ai999x.solve('kc_from_kp', Kp=1.8, T=523.0, dn=1.0)
# -> Kc = 0.0419424   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For PCl5(g) ⇌ PCl3(g) + Cl2(g), Kp = 1.8 at 523 K. What is Kc? | SOLVED | Kc = 0.0419424 |
| Kp = 0.25 at 700 K and Δn = -1. Find Kc. | SOLVED | Kc = 14.36 |

<a id="equilibrium_ice"></a>

### Equilibrium concentrations (ICE table) — `equilibrium_ice`

Equilibrium concentrations (or partial pressures) from the initial ones and K, for any one reaction as written.

$$
K = \prod_i (c_{i,0} + \nu_i x)^{\nu_i}
$$

| input | meaning | condition |
|---|---|---|
| `equation` | the reaction as written (its coefficients define K) |  |
| `initial` | initial concentration of each species in the order written (mol/L, or atm for Kp) |  |
| `K` | equilibrium constant |  |

| output | meaning |
|---|---|
| `x` | extent of reaction (mol/L) |
| `equilibrium` | equilibrium concentrations in the order written |

- **Method:** the extent x solved from the mass-action expression by bisection between the limits set by the initial amounts
- **Assumes:** one reaction, coefficients as written; no pure solids or liquids; $K > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The plane vectors of the worked example as arrows from the origin: input initial = (0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/equilibrium-ice-vectors)

*The plane vectors of the worked example as arrows from the origin: input initial = (0.1, 0); returned by chemistry-ai999x: equilibrium = (0.0898358, 0.0203284); also returned x = 0.0101642. Other inputs: equation = N2O4 ⇌ 2 NO2, K = 0.0046. Marked: the tip of each returned vector.*

![x (extent of reaction (mol/L)) against K (equilibrium constant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/equilibrium-ice-curve)

*x (extent of reaction (mol/L)) against K (equilibrium constant) from 0.0023 to 0.0069. Other inputs as in the worked example: equation = N2O4 ⇌ 2 NO2, initial = 0.10, 0. 31 points computed by chemistry-ai999x; marked: the worked example, K = 0.0046, x = 0.0101642.*

```python
chemistry_ai999x.solve('equilibrium_ice', equation='N2O4 ⇌ 2 NO2', initial='0.10, 0', K=0.0046)
# -> x = 0.0101642, equilibrium = [0.08983579029909557, 0.020328419401808878]   (SOLVED)
```

```python
chemistry_ai999x.solve('equilibrium_ice', equation='H2 + I2 ⇌ 2 HI', initial='0.50, 0.50, 0', K=50.0)
# -> x = 0.389759, equilibrium = [0.11024060460577123, 0.11024060460577123, 0.7795187907884575]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For N2O4 ⇌ 2 NO2, Kc = 4.6e-3. Initially [N2O4] = 0.10 M and [NO2] = 0. What are the equilibrium concentrations? | SOLVED | x = 0.0101642, equilibrium = [0.08983579029909557, 0.020328419401808878] |
| For H2 + I2 ⇌ 2 HI, K = 50.0. Initially [H2] = 0.50 M, [I2] = 0.50 M and [HI] = 0. Find the equilibrium concentrations. | SOLVED | x = 0.389759, equilibrium = [0.11024060460577123, 0.11024060460577123, 0.7795187907884575] |
| For N2O4 ⇌ 2 NO2, Kc = 4.6e-3. Initially [N2O4] = 0.10 M. What are the equilibrium concentrations? | NEEDS_INPUT | equilibrium_ice needs initial (initial concentration of each species in the order written (mol/L, or atm for Kp)). Examp |

<a id="k_from_delta_g"></a>

### Equilibrium constant from the standard Gibbs energy — `k_from_delta_g`

The equilibrium constant from the standard Gibbs energy change (K = exp(-dG°/RT)).

$$
K = e^{-\Delta G^\circ / RT}
$$

| input | meaning | condition |
|---|---|---|
| `dG` | standard Gibbs energy change |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `K` | equilibrium constant |
| `lnK` | ln K |

- **Method:** ln $K = -dG$°/(RT)
- **Assumes:** $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![lnK (ln K) against T (temperature) from 149 K to 447 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-delta-g-curve)

*lnK (ln K) against T (temperature) from 149 K to 447 K. One curve for each value of dG (standard Gibbs energy change): -33000 J/mol, -16500 J/mol, -66000 J/mol. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 298 K, lnK = 13.3188.*

![lnK (ln K) over T (temperature) from 149 K to 447 K and dG (standard Gibbs energy change) from -49500 J/mol to -16500 J/mol: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-delta-g-field)

*lnK (ln K) over T (temperature) from 149 K to 447 K and dG (standard Gibbs energy change) from -49500 J/mol to -16500 J/mol: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 4.43958 to dark 39.9563); iso-lines at lnK = 10, 20, 30. Marked: the worked example, T = 298 K, dG = -33000 J/mol, lnK = 13.3188.*

```python
chemistry_ai999x.solve('k_from_delta_g', dG=-33000.0, T=298.0)
# -> K = 608499, lnK = 13.3188   (SOLVED)
```

```python
chemistry_ai999x.solve('k_from_delta_g', dG=10000.0, T=500.0)
# -> K = 0.0902251, lnK = -2.40545   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| ΔG° = -33.0 kJ/mol at 298 K. What is the equilibrium constant? | SOLVED | K = 608499, lnK = 13.3188 |
| Find the equilibrium constant K at 500 K if the standard Gibbs energy change is 10.0 kJ/mol. | SOLVED | K = 0.0902251, lnK = -2.40545 |

<a id="delta_g_from_k"></a>

### Standard Gibbs energy from the equilibrium constant — `delta_g_from_k`

The standard Gibbs energy change from the equilibrium constant (dG° = -RT ln K).

$$
\Delta G^\circ = -RT\ln K
$$

| input | meaning | condition |
|---|---|---|
| `K` | equilibrium constant |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `dG` | standard Gibbs energy change |

- **Method:** dG° = -RT ln K
- **Assumes:** K, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dG (standard Gibbs energy change) against T (temperature) from 149 K to 447 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-k-curve)

*dG (standard Gibbs energy change) against T (temperature) from 149 K to 447 K. One curve for each value of K (equilibrium constant): 100000, 50000, 200000. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 298 K, dG = -28525.7 J/mol.*

![dG (standard Gibbs energy change) over T (temperature) from 149 K to 447 K and K (equilibrium constant) from 50000 to 150000: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light -44295](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-k-field)

*dG (standard Gibbs energy change) over T (temperature) from 149 K to 447 K and K (equilibrium constant) from 50000 to 150000: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light -44295.5 J/mol to dark -13404.1 J/mol); iso-lines at dG = -40000, -30000, -20000 J/mol. Marked: the worked example, T = 298 K, K = 100000, dG = -28525.7 J/mol.*

```python
chemistry_ai999x.solve('delta_g_from_k', K=100000.0, T=298.0)
# -> dG = -28525.7 J/mol   (SOLVED)
```

```python
chemistry_ai999x.solve('delta_g_from_k', K=0.25, T=400.0)
# -> dG = 4610.52 J/mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| K = 1.0e5 at 298 K. What is ΔG°? | SOLVED | dG = -28525.7 J/mol |
| Find the standard Gibbs energy change if the equilibrium constant is 0.25 at 400 K. | SOLVED | dG = 4610.52 J/mol |

<a id="vant_hoff_k2"></a>

### Van 't Hoff equation: K at another temperature — `vant_hoff_k2`

The equilibrium constant at a second temperature from K at a first one and the reaction enthalpy.

$$
\ln\frac{K_2}{K_1} = -\frac{\Delta H^\circ}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)
$$

| input | meaning | condition |
|---|---|---|
| `K1` | equilibrium constant at T1 |  |
| `T1` | first temperature |  |
| `T2` | second temperature |  |
| `dH` | standard enthalpy change |  |

| output | meaning |
|---|---|
| `K2` | equilibrium constant at T2 |
| `lnK2` | ln K2 |

- **Method:** integrated van 't Hoff equation (dH° constant)
- **Assumes:** K1, T1, $T_{2} > 0$; dH° independent of T
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Van 't Hoff equation: K at another temperature: K2 against T2 from 250 to 450, 81 runs of the package; other inputs as in the worked example (K1 = 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/vant-hoff-k2-curve)

*Van 't Hoff equation: K at another temperature: K2 against T2 from 250 to 450, 81 runs of the package; other inputs as in the worked example (K1 = 1.8e-05, T1 = 298.0, dH = 50000.0). Marked: the worked example, T2 = 350, K2 = 0.000360875.*

```python
chemistry_ai999x.solve('vant_hoff_k2', K1=1.8e-05, T1=298.0, T2=350.0, dH=50000.0)
# -> K2 = 0.000360875, lnK2 = -7.92698   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| K = 1.8e-5 at 298 K and ΔH° = 50.0 kJ/mol. What is K at 350 K? | SOLVED | K2 = 0.000360875, lnK2 = -7.92698 |
| K1 = 0.50, T1 = 400 K, T2 = 500 K, ΔH° = -92.0 kJ/mol. Find the equilibrium constant at T2. | SOLVED | K2 = 0.00197799, lnK2 = -6.22568 |

<a id="delta_g_nonstandard"></a>

### Gibbs energy change at non-standard conditions — `delta_g_nonstandard`

The Gibbs energy change of a reaction at a given reaction quotient (dG = dG° + RT ln Q).

$$
\Delta G = \Delta G^\circ + RT\ln Q
$$

| input | meaning | condition |
|---|---|---|
| `dG0` | standard Gibbs energy change |  |
| `T` | temperature |  |
| `Q` | reaction quotient |  |

| output | meaning |
|---|---|
| `dG` | Gibbs energy change |

- **Method:** $dG = dG$° + RT ln Q
- **Assumes:** T, $Q > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Gibbs energy change at non-standard conditions: dG against Q from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-nonstandard-curve)

*Gibbs energy change at non-standard conditions: dG against Q from 1.0000e-10 to 1.0000e+10 (logarithmic axis), 81 runs of the package; other inputs as in the worked example (dG0 = -33000.0, T = 298.0). Marked: the worked example, Q = 100, dG = -21589.7.*

```python
chemistry_ai999x.solve('delta_g_nonstandard', dG0=-33000.0, T=298.0, Q=100.0)
# -> dG = -21589.7 J/mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| ΔG° = -33.0 kJ/mol, T = 298 K and Q = 100. What is ΔG? | SOLVED | dG = -21589.7 J/mol |
| The standard Gibbs energy change is 5.0 kJ/mol at 350 K and the reaction quotient is 0.010. Find ΔG. | SOLVED | dG = -8401.33 J/mol |

<a id="ksp_molar_solubility"></a>

### Molar solubility from Ksp — `ksp_molar_solubility`

The molar solubility of a sparingly soluble salt MxAy in pure water from its solubility product.

$$
K_{sp} = (x s)^x (y s)^y
$$

| input | meaning | condition |
|---|---|---|
| `Ksp` | solubility product |  |
| `x` | cations per formula unit |  |
| `y` | anions per formula unit |  |

| output | meaning |
|---|---|
| `s` | molar solubility |
| `s_molL` | in mol/L |

- **Method:** s = (Ksp / (x^x y^y))^(1/(x+y))
- **Assumes:** Ksp > 0; no common ion, no side reactions
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Molar solubility from Ksp: s_molL against Ksp from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ksp-molar-solubility-curve)

*Molar solubility from Ksp: s_molL against Ksp from 1.0000e-20 to 0.0001 (logarithmic axis), 81 runs of the package per curve (MX (x = 1, y = 1); MX₂ (x = 1, y = 2); MX₃ (x = 1, y = 3)); other inputs as in the worked example (x = 1, y = 2). Marked: the worked example, Ksp = 3.9000e-11, s_molL = 0.000213633.*

```python
chemistry_ai999x.solve('ksp_molar_solubility', Ksp=3.9e-11, x=1, y=2)
# -> s = 0.213633 mol/m^3, s_molL = 0.000213633, log10_s = -3.67033   (SOLVED)
```

```python
chemistry_ai999x.solve('ksp_molar_solubility', Ksp=1.8e-10, x=1, y=1)
# -> s = 0.0134164 mol/m^3, s_molL = 1.34164e-05, log10_s = -4.87236   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the molar solubility of CaF2 (Ksp = 3.9e-11)? | SOLVED | s = 0.213633 mol/m^3, s_molL = 0.000213633, log10_s = -3.67033 |
| Find the molar solubility of Ag2CrO4 if Ksp = 1.1 × 10^-12. | SOLVED | s = 0.0650296 mol/m^3, s_molL = 6.50296e-05, log10_s = -4.18689 |
| What is the molar solubility of Ca3(PO4)2 (Ksp = 2.0e-29)? | SOLVED | s = 0.000713709 mol/m^3, s_molL = 7.13709e-07, log10_s = -6.14648 |
| What is the molar solubility of CaF2? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="ksp_from_solubility"></a>

### Ksp from molar solubility — `ksp_from_solubility`

The solubility product of a salt MxAy from its molar solubility.

$$
K_{sp} = (x s)^x (y s)^y
$$

| input | meaning | condition |
|---|---|---|
| `s` | molar solubility |  |
| `x` | cations per formula unit |  |
| `y` | anions per formula unit |  |

| output | meaning |
|---|---|
| `Ksp` | solubility product |

- **Method:** Ksp = (x s)^x (y s)^y
- **Assumes:** $s > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Ksp (solubility product) against s (molar solubility) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ksp-from-solubility-curve)

*Ksp (solubility product) against s (molar solubility) from 0.75 mol/m^3 to 2.25 mol/m^3. One curve for each value of y (anions per formula unit): 2, 3, 4. Other inputs as in the worked example: x = 1. 31 points per curve computed by chemistry-ai999x; marked: the worked example, s = 1.5 mol/m^3, Ksp = 1.3500e-08.*

```python
chemistry_ai999x.solve('ksp_from_solubility', s=1.5, x=1, y=2)
# -> Ksp = 1.35e-08, pKsp = 7.86967   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The molar solubility of PbI2 is 1.5e-3 M. What is its Ksp? | SOLVED | Ksp = 1.35e-08, pKsp = 7.86967 |
| The molar solubility of AgCl is 1.3 × 10^-5 M. Calculate the Ksp of AgCl. | SOLVED | Ksp = 1.69e-10, pKsp = 9.77211 |

<a id="common_ion_solubility"></a>

### Molar solubility with a common ion — `common_ion_solubility`

The molar solubility of a salt MxAy in a solution that already contains one of its ions.

$$
K_{sp} = (x s + c_{M})^x (y s + c_{A})^y
$$

| input | meaning | condition |
|---|---|---|
| `Ksp` | solubility product |  |
| `x` | cations per formula unit |  |
| `y` | anions per formula unit |  |
| `c_common` | concentration of the common ion |  |
| `common` | which ion is common |  |

| output | meaning |
|---|---|
| `s` | molar solubility |
| `s_molL` | in mol/L |

- **Method:** the solubility s solved from the Ksp expression with the common ion's concentration, by bisection
- **Assumes:** Ksp > 0; the common ion comes from a fully dissolved salt
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![s (molar solubility) against c_common (concentration of the common ion) from 50 mol/m^3 to 150 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/common-ion-solubility-curve)

*s (molar solubility) against c_common (concentration of the common ion) from 50 mol/m^3 to 150 mol/m^3. One curve for each value of x (cations per formula unit): 1, 2, 3. Other inputs as in the worked example: Ksp = 3.9e-11, y = 2, common = anion. 31 points per curve computed by chemistry-ai999x; marked: the worked example, c_common = 100 mol/m^3, s = 3.9000e-06 mol/m^3.*

```python
chemistry_ai999x.solve('common_ion_solubility', Ksp=3.9e-11, x=1, y=2, c_common=100.0, common='anion')
# -> s = 3.9e-06 mol/m^3, s_molL = 3.9e-09, log10_s = -8.40894   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the molar solubility of CaF2 in 0.10 M NaF (Ksp = 3.9e-11)? | SOLVED | s = 3.9e-06 mol/m^3, s_molL = 3.9e-09, log10_s = -8.40894 |
| Find the molar solubility of AgCl in 0.010 M AgNO3 (Ksp = 1.8e-10). | SOLVED | s = 1.8e-05 mol/m^3, s_molL = 1.8e-08, log10_s = -7.74473 |
| What is the molar solubility of PbI2 in 0.050 M Pb(NO3)2 (Ksp = 9.8e-9)? | SOLVED | s = 0.220872 mol/m^3, s_molL = 0.000220872, log10_s = -3.65586 |

<a id="ph_from_hydronium"></a>

### pH from the hydronium concentration — `ph_from_hydronium`

pH, pOH and [OH-] from the hydronium (hydrogen ion) concentration at 25 °C.

$$
\mathrm{pH} = -\log_{10}[\mathrm{H_3O^+}]
$$

| input | meaning | condition |
|---|---|---|
| `h` | hydronium ion concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `pOH` | pOH |
| `oh` | [OH-] |

- **Method:** $pH = -log10$[H3O+]; pOH = 14 - pH (25 °C)
- **Assumes:** [H3O+] > 0 (mol/L); 25 °C $(Kw = 1.0e-14)$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against h (hydronium ion concentration) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-from-hydronium-curve)

*pH (pH) against h (hydronium ion concentration) from 0.1 mol/m^3 to 10 mol/m^3 on a logarithmic axis. 41 points computed by chemistry-ai999x; marked: the worked example, h = 1 mol/m^3, pH = 3.*

```python
chemistry_ai999x.solve('ph_from_hydronium', h=1.0)
# -> pH = 3, pOH = 11, oh = 1e-08 mol/m^3   (SOLVED)
```

```python
chemistry_ai999x.solve('ph_from_hydronium', h=0.025)
# -> pH = 4.60206, pOH = 9.39794, oh = 4e-07 mol/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of a solution with [H+] = 1.0e-3 M? | SOLVED | pH = 3, pOH = 11, oh = 1e-08 mol/m^3 |
| Calculate the pH if the hydronium ion concentration is 2.5 × 10^-5 M. | SOLVED | pH = 4.60206, pOH = 9.39794, oh = 4e-07 mol/m^3 |

<a id="hydronium_from_ph"></a>

### Hydronium concentration from pH — `hydronium_from_ph`

[H3O+], [OH-] and pOH from the pH at 25 °C.

$$
[\mathrm{H_3O^+}] = 10^{-\mathrm{pH}}
$$

| input | meaning | condition |
|---|---|---|
| `pH` | pH |  |

| output | meaning |
|---|---|
| `h` | [H3O+] |
| `h_molL` | [H3O+] in mol/L |
| `pOH` | pOH |
| `oh` | [OH-] |

- **Method:** [H3O+] = $10^{-pH}$
- **Assumes:** 25 °C $(Kw = 1.0e-14)$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![h ([H3O+]) against pH (pH) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydronium-from-ph-curve)

*h ([H3O+]) against pH (pH) from 1.6 to 4.8, logarithmic output axis. Outputs with the same unit drawn together: oh ([OH-]). 31 points computed by chemistry-ai999x; marked: the worked example, pH = 3.2, h = 0.630957 mol/m^3.*

```python
chemistry_ai999x.solve('hydronium_from_ph', pH=3.2)
# -> h = 0.630957 mol/m^3, h_molL = 0.000630957, pOH = 10.8, oh = 1.58489e-08 mol/m^3   (SOLVED)
```

```python
chemistry_ai999x.solve('hydronium_from_ph', pH=9.5)
# -> h = 3.16228e-07 mol/m^3, h_molL = 3.16228e-10, pOH = 4.5, oh = 0.0316228 mol/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is [H+] in a solution of pH 3.20? | SOLVED | h = 0.630957 mol/m^3, h_molL = 0.000630957, pOH = 10.8, oh = 1.58489e-08 mol/m^3 |
| Find the hydronium ion concentration of a solution with pH = 9.5. | SOLVED | h = 3.16228e-07 mol/m^3, h_molL = 3.16228e-10, pOH = 4.5, oh = 0.0316228 mol/m^3 |

<a id="ph_from_hydroxide"></a>

### pH from the hydroxide concentration — `ph_from_hydroxide`

pOH and pH from the hydroxide ion concentration at 25 °C.

$$
\mathrm{pH} = 14 + \log_{10}[\mathrm{OH^-}]
$$

| input | meaning | condition |
|---|---|---|
| `oh` | hydroxide ion concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `pOH` | pOH |

- **Method:** pOH = -log10[OH-]; pH = 14 - pOH (25 °C)
- **Assumes:** [OH-] > 0; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH from the hydroxide concentration: pH against oh from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-from-hydroxide-curve)

*pH from the hydroxide concentration: pH against oh from 1.0000e-14 to 1 (logarithmic axis), 81 runs of the package; other inputs as in the worked example (). Marked: the worked example, oh = 0.1, pH = 10.*

```python
chemistry_ai999x.solve('ph_from_hydroxide', oh=0.1)
# -> pH = 10, pOH = 4   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of a solution with [OH-] = 1.0e-4 M? | SOLVED | pH = 10, pOH = 4 |
| Find the pOH and pH if the hydroxide ion concentration is 3.2 × 10^-3 M. | SOLVED | pH = 11.5051, pOH = 2.49485 |

<a id="ph_strong_acid"></a>

### pH of a strong monoprotic acid — `ph_strong_acid`

The pH of a strong monoprotic acid solution (water's own ionization included, so dilute solutions are right).

$$
[\mathrm{H_3O^+}] = \frac{c}{2} + \sqrt{\frac{c^2}{4} + K_w}
$$

| input | meaning | condition |
|---|---|---|
| `c` | acid concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `h` | [H3O+] |

- **Method:** charge balance [H3O+] = c + Kw/[H3O+] solved exactly
- **Assumes:** monoprotic strong acid; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against c (acid concentration) from 1 mol/m^3 to 100 mol/m^3 on a logarithmic axis](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-strong-acid-curve)

*pH (pH) against c (acid concentration) from 1 mol/m^3 to 100 mol/m^3 on a logarithmic axis. 41 points computed by chemistry-ai999x; marked: the worked example, c = 10 mol/m^3, pH = 2.*

```python
chemistry_ai999x.solve('ph_strong_acid', c=10.0)
# -> pH = 2, h = 10 mol/m^3   (SOLVED)
```

```python
chemistry_ai999x.solve('ph_strong_acid', c=1e-05)
# -> pH = 6.97829, h = 0.000105125 mol/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of 0.010 M HCl? | SOLVED | pH = 2, h = 10 mol/m^3 |
| Calculate the pH of a 1.0e-8 M HNO3 solution. | SOLVED | pH = 6.97829, h = 0.000105125 mol/m^3 |
| What is the pH of 0.25 M hydrochloric acid? | SOLVED | pH = 0.60206, h = 250 mol/m^3 |

<a id="ph_strong_base"></a>

### pH of a strong base — `ph_strong_base`

The pH and pOH of a strong (group 1 or 2) hydroxide solution.

$$
[\mathrm{OH^-}] = n c,\quad \mathrm{pH} = 14 - \mathrm{pOH}
$$

| input | meaning | condition |
|---|---|---|
| `c` | base concentration |  |
| `n_oh` | hydroxide ions per formula unit |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `pOH` | pOH |

- **Method:** charge balance with water's ionization, [OH-] = n c + Kw/[OH-]
- **Assumes:** strong base fully dissociated; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against c (base concentration) from 5 mol/m^3 to 15 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-strong-base-curve)

*pH (pH) against c (base concentration) from 5 mol/m^3 to 15 mol/m^3. One curve for each value of n_oh (hydroxide ions per formula unit): 1, 2, 3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, c = 10 mol/m^3, pH = 12.*

```python
chemistry_ai999x.solve('ph_strong_base', c=10.0, n_oh=1)
# -> pH = 12, pOH = 2   (SOLVED)
```

```python
chemistry_ai999x.solve('ph_strong_base', c=5.0, n_oh=2)
# -> pH = 12, pOH = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of 0.010 M NaOH? | SOLVED | pH = 12, pOH = 2 |
| Calculate the pH of 0.0050 M Ba(OH)2. | SOLVED | pH = 12, pOH = 2 |
| What is the pH of a 0.020 M strong base? | NEEDS_INPUT | ph_strong_base needs n_oh (hydroxide ions per formula unit). Example: solve('ph_strong_base', c='10 mol/m^3', n_oh=1). |

<a id="ph_weak_acid"></a>

### pH of a weak acid — `ph_weak_acid`

The pH and percent ionization of a weak monoprotic acid from Ka (or pKa) and its concentration.

$$
[\mathrm{H_3O^+}] = \frac{K_a c}{K_a + [\mathrm{H_3O^+}]} + \frac{K_w}{[\mathrm{H_3O^+}]}
$$

| input | meaning | condition |
|---|---|---|
| `Ka` | acid dissociation constant |  |
| `c` | acid concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `h` | [H3O+] |
| `percent_ionized` | percent ionization |

- **Method:** exact charge balance (no 5% approximation), solved by bisection
- **Assumes:** monoprotic acid; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against Ka (acid dissociation constant) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-acid-curve)

*pH (pH) against Ka (acid dissociation constant) from 1.8000e-06 to 0.00018 on a logarithmic axis. One curve for each value of c (acid concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. 41 points per curve computed by chemistry-ai999x; marked: the worked example, Ka = 1.8000e-05, pH = 2.87528.*

![pH (pH) over c (acid concentration) from 10 mol/m^3 to 1000 mol/m^3 and Ka (acid dissociation constant) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-acid-field)

*pH (pH) over c (acid concentration) from 10 mol/m^3 to 1000 mol/m^3 and Ka (acid dissociation constant) from 1.8000e-06 to 0.00018 (logarithmic axes): each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 1.87528 to dark 3.87528); iso-lines at pH = 2, 2.5, 3, 3.5. Marked: the worked example, c = 100 mol/m^3, Ka = 1.8000e-05, pH = 2.87528.*

```python
chemistry_ai999x.solve('ph_weak_acid', Ka=1.8e-05, c=100.0)
# -> pH = 2.87528, h = 1.33267 mol/m^3, percent_ionized = 1.33267   (SOLVED)
```

```python
chemistry_ai999x.solve('ph_weak_acid', Ka=6.309573444801929e-05, c=50.0)
# -> pH = 2.75823, h = 1.7449 mol/m^3, percent_ionized = 3.48981   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)? | SOLVED | pH = 2.87528, h = 1.33267 mol/m^3, percent_ionized = 1.33267 |
| Find the pH of a 0.050 M weak acid with pKa = 4.20. | SOLVED | pH = 2.75823, h = 1.7449 mol/m^3, percent_ionized = 3.48981 |
| What is the percent ionization of 0.20 M HF (Ka = 6.8e-4)? | SOLVED | pH = 1.94589, h = 11.3269 mol/m^3, percent_ionized = 5.66343 |
| What is the pH of HF (Ka = 6.8e-4)? | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: chemistry_ai999x.templates(); eve |

<a id="ph_weak_base"></a>

### pH of a weak base — `ph_weak_base`

The pH, pOH and percent ionization of a weak base from Kb (or pKb) and its concentration.

$$
[\mathrm{OH^-}] = \frac{K_b c}{K_b + [\mathrm{OH^-}]} + \frac{K_w}{[\mathrm{OH^-}]}
$$

| input | meaning | condition |
|---|---|---|
| `Kb` | base dissociation constant |  |
| `c` | base concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `pOH` | pOH |
| `percent_ionized` | percent ionization |

- **Method:** exact charge balance for [OH-], solved by bisection; pH = 14 - pOH
- **Assumes:** 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against Kb (base dissociation constant) from 9](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-base-curve)

*pH (pH) against Kb (base dissociation constant) from 9.0000e-06 to 2.7000e-05. One curve for each value of c (base concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, Kb = 1.8000e-05, pH = 11.1247.*

![pH (pH) over c (base concentration) from 50 mol/m^3 to 150 mol/m^3 and Kb (base dissociation constant) from 9](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-base-field)

*pH (pH) over c (base concentration) from 50 mol/m^3 to 150 mol/m^3 and Kb (base dissociation constant) from 9.0000e-06 to 2.7000e-05: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 10.8237 to dark 11.3008); iso-lines at pH = 10.9, 11, 11.1, 11.2. Marked: the worked example, c = 100 mol/m^3, Kb = 1.8000e-05, pH = 11.1247.*

```python
chemistry_ai999x.solve('ph_weak_base', Kb=1.8e-05, c=100.0)
# -> pH = 11.1247, pOH = 2.87528, percent_ionized = 1.33267   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of 0.10 M NH3 (Kb = 1.8 × 10^-5)? | SOLVED | pH = 11.1247, pOH = 2.87528, percent_ionized = 1.33267 |
| Find the pH of a 0.25 M weak base with pKb = 3.36. | SOLVED | pH = 12.0099, pOH = 1.9901, percent_ionized = 4.0922 |

<a id="ka_from_ph"></a>

### Ka from the pH of a weak acid — `ka_from_ph`

The acid dissociation constant of a weak monoprotic acid from the pH of a solution of known concentration.

$$
K_a = \frac{[\mathrm{H_3O^+}][\mathrm{A^-}]}{c - [\mathrm{A^-}]},\ [\mathrm{A^-}] = [\mathrm{H_3O^+}] - K_w/[\mathrm{H_3O^+}]
$$

| input | meaning | condition |
|---|---|---|
| `pH` | measured pH |  |
| `c` | acid concentration |  |

| output | meaning |
|---|---|
| `pKa` | pKa |
| `Ka` | Ka |

- **Method:** [A-] from the charge balance at the measured pH, then the mass-action expression
- **Assumes:** monoprotic weak acid; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pKa (pKa) against pH (measured pH) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ka-from-ph-curve)

*pKa (pKa) against pH (measured pH) from 1.435 to 4.305. One curve for each value of c (acid concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, pH = 2.87, pKa = 4.7341.*

![pKa (pKa) over pH (measured pH) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ka-from-ph-field)

*pKa (pKa) over pH (measured pH) from 1.435 to 4.305 and c (acid concentration) from 10 mol/m^3 to 1000 mol/m^3 (logarithmic vertical axis): each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its sign and magnitude (second colour negative); iso-lines at pKa = 2, 4, 6, 8; blank cells: inputs the capability refuses. Marked: the worked example, pH = 2.87, c = 100 mol/m^3, pKa = 4.7341.*

```python
chemistry_ai999x.solve('ka_from_ph', pH=2.87, c=100.0)
# -> Ka = 1.84458e-05, pKa = 4.7341   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A 0.10 M solution of a weak acid has a pH of 2.87. What is its Ka? | SOLVED | Ka = 1.84458e-05, pKa = 4.7341 |
| The pH of 0.050 M HNO2 is 2.30. Find the acid dissociation constant. | SOLVED | Ka = 0.000558344, pKa = 3.2531 |

<a id="buffer_ph"></a>

### Buffer pH (Henderson-Hasselbalch) — `buffer_ph`

The pH of a buffer from the acid's Ka (or pKa) and the concentrations of the weak acid and its conjugate base.

$$
\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{[\mathrm{A^-}]}{[\mathrm{HA}]}
$$

| input | meaning | condition |
|---|---|---|
| `Ka` | acid dissociation constant |  |
| `c_base` | conjugate base concentration |  |
| `c_acid` | weak acid concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |

- **Method:** Henderson-Hasselbalch equation
- **Assumes:** both concentrations well above Ka and [H3O+]
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against Ka (acid dissociation constant) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/buffer-ph-curve)

*pH (pH) against Ka (acid dissociation constant) from 1.7378e-06 to 0.00017378 on a logarithmic axis. One curve for each value of c_base (conjugate base concentration): 150 mol/m^3, 75 mol/m^3, 300 mol/m^3. Other inputs as in the worked example: c_acid = 100.0 mol/m^3. 41 points per curve computed by chemistry-ai999x; marked: the worked example, Ka = 1.7378e-05, pH = 4.93609.*

```python
chemistry_ai999x.solve('buffer_ph', Ka=1.7378008287493764e-05, c_base=150.0, c_acid=100.0)
# -> pH = 4.93609   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of a buffer that is 0.10 M acetic acid and 0.15 M sodium acetate (pKa = 4.76)? | SOLVED | pH = 4.93609 |
| Find the pH of a buffer with [HA] = 0.20 M, [A-] = 0.10 M and Ka = 1.8 × 10^-5. | SOLVED | pH = 4.4437 |

<a id="titration_equivalence_volume"></a>

### Titration: equivalence-point volume — `titration_equivalence_volume`

The volume of titrant needed to reach the equivalence point of a neutralization.

$$
V_{eq} = \frac{\nu\, c_a V_a}{c_t}
$$

| input | meaning | condition |
|---|---|---|
| `c_t` | titrant concentration |  |
| `V_a` | volume of the analyte |  |
| `c_a` | analyte concentration |  |
| `ratio` | moles of titrant per mole of analyte |  |

| output | meaning |
|---|---|
| `V_eq` | titrant volume at equivalence |
| `V_eq_mL` | in mL |

- **Method:** moles of titrant = ratio x moles of analyte
- **Assumes:** complete neutralization; the acid's protons and base's hydroxides from the formulas
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V_eq (titrant volume at equivalence) against c_t (titrant concentration) from 50 mol/m^3 to 150 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-equivalence-volume-curve)

*V_eq (titrant volume at equivalence) against c_t (titrant concentration) from 50 mol/m^3 to 150 mol/m^3. One curve for each value of V_a (volume of the analyte): 2.5000e-05 m^3, 1.2500e-05 m^3, 5.0000e-05 m^3. Other inputs as in the worked example: c_a = 150.0 mol/m^3, ratio = 1.0. 31 points per curve computed by chemistry-ai999x; marked: the worked example, c_t = 100 mol/m^3, V_eq = 3.7500e-05 m^3.*

```python
chemistry_ai999x.solve('titration_equivalence_volume', c_t=100.0, V_a=2.5e-05, c_a=150.0, ratio=1.0)
# -> V_eq = 3.75e-05 m^3, V_eq_mL = 37.5   (SOLVED)
```

```python
chemistry_ai999x.solve('titration_equivalence_volume', c_t=100.0, V_a=2e-05, c_a=100.0, ratio=2.0)
# -> V_eq = 4e-05 m^3, V_eq_mL = 40   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What volume of 0.100 M NaOH is needed to neutralize 25.0 mL of 0.150 M HCl? | SOLVED | V_eq = 3.75e-05 m^3, V_eq_mL = 37.5 |
| What volume of 0.100 M NaOH is needed to neutralize 20.0 mL of 0.100 M H2SO4? | SOLVED | V_eq = 4e-05 m^3, V_eq_mL = 40 |
| What volume of 0.200 M HCl neutralizes 30.0 mL of 0.100 M Ba(OH)2? | SOLVED | V_eq = 3e-05 m^3, V_eq_mL = 30 |

<a id="titration_ph_strong_acid_strong_base"></a>

### pH during a strong acid - strong base titration — `titration_ph_strong_acid_strong_base`

The pH after adding a volume of strong base to a strong monoprotic acid (exact, any point of the titration).

$$
[\mathrm{H_3O^+}] - \frac{K_w}{[\mathrm{H_3O^+}]} = \frac{c_a V_a - c_b V_b}{V_a + V_b}
$$

| input | meaning | condition |
|---|---|---|
| `c_a` | acid concentration |  |
| `V_a` | acid volume |  |
| `c_b` | base concentration |  |
| `V_b` | volume of base added |  |

| output | meaning |
|---|---|
| `pH` | pH |

- **Method:** excess strong acid or base after mixing, with water's ionization (exact at the equivalence point too)
- **Assumes:** strong monoprotic acid titrated with a strong monohydroxide base; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Titration curve of a strong acid (100](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-strong-acid-strong-base-titration)

*Titration curve of a strong acid (100.0 mol/m³, 25 mL) with a strong base (100.0 mol/m³): the pH the package returned for 121 added volumes from 0 to twice the equivalence volume (equivalence at c_a·V_a/c_b = 25 mL, computed from the stated inputs). Marked: the worked example, V_b = 10 mL, pH = 1.36798.*

```python
chemistry_ai999x.solve('titration_ph_strong_acid_strong_base', c_a=100.0, V_a=2.5e-05, c_b=100.0, V_b=1e-05)
# -> pH = 1.36798   (SOLVED)
```

```python
chemistry_ai999x.solve('titration_ph_strong_acid_strong_base', c_a=100.0, V_a=2.5e-05, c_b=100.0, V_b=2.5e-05)
# -> pH = 7   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| 25.0 mL of 0.100 M HCl is titrated with 0.100 M NaOH. What is the pH after adding 10.0 mL of base? | SOLVED | pH = 1.36798 |
| 50.0 mL of 0.200 M HNO3 is titrated with 0.100 M KOH. Find the pH after adding 150 mL of base. | SOLVED | pH = 12.3979 |
| 25.0 mL of 0.100 M HCl is titrated with 0.100 M NaOH. What is the pH? | NEEDS_INPUT | titration_ph_strong_acid_strong_base needs V_b (volume of base added). Example: solve('titration_ph_strong_acid_strong_b |

<a id="titration_ph_weak_acid_strong_base"></a>

### pH during a weak acid - strong base titration — `titration_ph_weak_acid_strong_base`

The pH after adding strong base to a weak monoprotic acid, at any point (exact charge balance).

$$
[\mathrm{H_3O^+}] + [\mathrm{Na^+}] = \frac{K_w}{[\mathrm{H_3O^+}]} + \frac{K_a C}{K_a + [\mathrm{H_3O^+}]}
$$

| input | meaning | condition |
|---|---|---|
| `Ka` | acid dissociation constant |  |
| `c_a` | acid concentration |  |
| `V_a` | acid volume |  |
| `c_b` | base concentration |  |
| `V_b` | volume of base added |  |

| output | meaning |
|---|---|
| `pH` | pH |

- **Method:** exact charge balance after mixing, solved by bisection (before, at and after equivalence)
- **Assumes:** weak monoprotic acid, strong monohydroxide base; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH against volume of 0](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/titration-ph-weak-acid-strong-base-titration.svg)

*pH against volume of 0.100 M NaOH added to 25.0 mL of 0.100 M acid, computed by the package at 101 volumes for a strong acid and a weak acid (Ka = 1.8e-5). Shaded: the weak acid's buffer region, 10 % to 90 % neutralised (pH 3.79886 to 5.69915); at half-equivalence pH = 4.7452 (= pKa). Marked: equivalence V_b = 25 mL, pH = 8.7219.*

![pH (pH) against c_b (base concentration) from 10 mol/m^3 to 1000 mol/m^3 on a logarithmic axis](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-weak-acid-strong-base-curve)

*pH (pH) against c_b (base concentration) from 10 mol/m^3 to 1000 mol/m^3 on a logarithmic axis. One curve for each value of V_a (acid volume): 2.5000e-05 m^3, 1.2500e-05 m^3, 5.0000e-05 m^3. Other inputs as in the worked example: Ka = 1.8e-05, c_a = 100.0 mol/m^3, V_b = 1.25e-05 m^3. 41 points per curve computed by chemistry-ai999x; marked: the worked example, c_b = 100 mol/m^3, pH = 4.7452.*

```python
chemistry_ai999x.solve('titration_ph_weak_acid_strong_base', Ka=1.8e-05, c_a=100.0, V_a=2.5e-05, c_b=100.0, V_b=1.25e-05)
# -> pH = 4.7452   (SOLVED)
```

```python
chemistry_ai999x.solve('titration_ph_weak_acid_strong_base', Ka=1.8e-05, c_a=100.0, V_a=2.5e-05, c_b=100.0, V_b=2.5e-05)
# -> pH = 8.7219   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| 25.0 mL of 0.100 M CH3COOH (Ka = 1.8e-5) is titrated with 0.100 M NaOH. What is the pH after adding 12.5 mL of base? | SOLVED | pH = 4.7452 |
| 25.0 mL of 0.100 M CH3COOH (Ka = 1.8e-5) is titrated with 0.100 M NaOH. What is the pH after adding 25.0 mL of base? | SOLVED | pH = 8.7219 |

<a id="ph_diprotic_acid"></a>

### pH of a diprotic acid — `ph_diprotic_acid`

The pH of a diprotic acid H2A from both dissociation constants, solved exactly.

$$
[\mathrm{H^+}] = C\frac{K_1[\mathrm{H^+}] + 2K_1K_2}{[\mathrm{H^+}]^2 + K_1[\mathrm{H^+}] + K_1K_2} + \frac{K_w}{[\mathrm{H^+}]}
$$

| input | meaning | condition |
|---|---|---|
| `Ka1` | first dissociation constant |  |
| `Ka2` | second dissociation constant |  |
| `c` | acid concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |

- **Method:** exact charge balance of H2A, solved by bisection
- **Assumes:** 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against Ka1 (first dissociation constant) from 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-diprotic-acid-curve)

*pH (pH) against Ka1 (first dissociation constant) from 2.1500e-07 to 6.4500e-07. One curve for each value of c (acid concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. Other inputs as in the worked example: Ka2 = 4.8e-11. 31 points per curve computed by chemistry-ai999x; marked: the worked example, Ka1 = 4.3000e-07, pH = 3.68372.*

```python
chemistry_ai999x.solve('ph_diprotic_acid', Ka1=4.3e-07, Ka2=4.8e-11, c=100.0)
# -> pH = 3.68372   (SOLVED)
```

```python
chemistry_ai999x.solve('ph_diprotic_acid', Ka1=0.059, Ka2=6.4e-05, c=100.0)
# -> pH = 1.27699   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of 0.10 M H2CO3 (Ka1 = 4.3e-7, Ka2 = 4.8e-11)? | SOLVED | pH = 3.68372 |
| Find the pH of 0.10 M oxalic acid with Ka1 = 5.9e-2 and Ka2 = 6.4e-5. | SOLVED | pH = 1.27699 |

<a id="ph_salt_of_weak_acid"></a>

### pH of a salt of a weak acid — `ph_salt_of_weak_acid`

The pH of a solution of a salt of a weak acid and a strong base (e.g. sodium acetate) from the acid's Ka.

$$
K_b = \frac{K_w}{K_a}
$$

| input | meaning | condition |
|---|---|---|
| `Ka` | acid dissociation constant |  |
| `c` | salt concentration |  |

| output | meaning |
|---|---|
| `pH` | pH |
| `Kb` | Kb of the anion (Kw/Ka) |

- **Method:** the anion as a weak base with $Kb = Kw/Ka$, exact charge balance
- **Assumes:** salt of a weak monoprotic acid and a strong base; 25 °C
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![pH (pH) against Ka (acid dissociation constant) from 9](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-salt-of-weak-acid-curve)

*pH (pH) against Ka (acid dissociation constant) from 9.0000e-06 to 2.7000e-05. One curve for each value of c (salt concentration): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, Ka = 1.8000e-05, pH = 8.87239.*

![pH (pH) over c (salt concentration) from 50 mol/m^3 to 150 mol/m^3 and Ka (acid dissociation constant) from 9](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-salt-of-weak-acid-field)

*pH (pH) over c (salt concentration) from 50 mol/m^3 to 150 mol/m^3 and Ka (acid dissociation constant) from 9.0000e-06 to 2.7000e-05: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 8.6339 to dark 9.11092); iso-lines at pH = 8.7, 8.8, 8.9, 9, 9.1. Marked: the worked example, c = 100 mol/m^3, Ka = 1.8000e-05, pH = 8.87239.*

```python
chemistry_ai999x.solve('ph_salt_of_weak_acid', Ka=1.8e-05, c=100.0)
# -> pH = 8.87239, Kb = 5.55556e-10   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the pH of 0.10 M sodium acetate (Ka of acetic acid = 1.8e-5)? | SOLVED | pH = 8.87239, Kb = 5.55556e-10 |
| Find the pH of a 0.20 M solution of the salt NaF if Ka for HF is 6.8e-4. | SOLVED | pH = 8.235, Kb = 1.47059e-11 |

<a id="zero_order_concentration"></a>

### Zero-order integrated rate law — `zero_order_concentration`

The concentration of a reactant after a time in a zero-order reaction.

$$
[A] = [A]_0 - k t
$$

| input | meaning | condition |
|---|---|---|
| `A0` | initial concentration |  |
| `k` | zero-order rate constant |  |
| `t` | elapsed time |  |

| output | meaning |
|---|---|
| `A` | concentration at time t |
| `A_molL` | in mol/L |

- **Method:** [A] = [A]$0 - k$ t
- **Assumes:** [A]0, k, $t > 0$; [A] stays >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A (concentration at time t) against A0 (initial concentration) from 250 mol/m^3 to 750 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/zero-order-concentration-curve)

*A (concentration at time t) against A0 (initial concentration) from 250 mol/m^3 to 750 mol/m^3. One curve for each value of k (zero-order rate constant): 2 mol/(m^3*s), 1 mol/(m^3*s), 4 mol/(m^3*s). Other inputs as in the worked example: t = 100.0 s. 31 points per curve computed by chemistry-ai999x; marked: the worked example, A0 = 500 mol/m^3, A = 300 mol/m^3.*

```python
chemistry_ai999x.solve('zero_order_concentration', A0=500.0, k=2.0, t=100.0)
# -> A = 300 mol/m^3, A_molL = 0.3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A zero-order reaction has k = 0.0020 M/s. If [A]0 = 0.50 M, what is the concentration after 100 s? | SOLVED | A = 300 mol/m^3, A_molL = 0.3 |
| A zero-order reaction has k = 0.0020 M/s. If [A]0 = 0.50 M, what is the concentration after 1000 s? | INVALID_INPUT | Zero-order integrated rate law: the reactant is used up before this time (A0 - k t < 0). |

<a id="first_order_concentration"></a>

### First-order integrated rate law — `first_order_concentration`

The concentration of a reactant after a time in a first-order reaction.

$$
[A] = [A]_0\, e^{-kt}
$$

| input | meaning | condition |
|---|---|---|
| `A0` | initial concentration |  |
| `k` | first-order rate constant |  |
| `t` | elapsed time |  |

| output | meaning |
|---|---|
| `A` | concentration at time t |
| `A_molL` | in mol/L |
| `fraction_left` | fraction remaining |

- **Method:** [A] = [A]0 exp(-k t)
- **Assumes:** [A]0, k, $t > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Reactant concentration against time from A0 = 500 mol/m³ for zero-, first- and second-order rate laws with the same initial rate (2 mol/(m³·s)), each computed by the package at up to 121 times](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/first-order-concentration-kinetics.svg)

*Reactant concentration against time from A0 = 500 mol/m³ for zero-, first- and second-order rate laws with the same initial rate (2 mol/(m³·s)), each computed by the package at up to 121 times. At t = 200 s: 100, 224.664 and 277.778 mol/m³; marked: t = 200 s, [A] = 224.664 mol/m³ (first order).*

![A (concentration at time t) against k (first-order rate constant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-concentration-curve)

*A (concentration at time t) against k (first-order rate constant) from 0.0015 1/s to 0.15 1/s on a logarithmic axis. One curve for each value of A0 (initial concentration): 800 mol/m^3, 400 mol/m^3, 1600 mol/m^3. Other inputs as in the worked example: t = 60.0 s. 41 points per curve computed by chemistry-ai999x; marked: the worked example, k = 0.015 1/s, A = 325.256 mol/m^3.*

```python
chemistry_ai999x.solve('first_order_concentration', A0=800.0, k=0.015, t=60.0)
# -> A = 325.256 mol/m^3, A_molL = 0.325256, fraction_left = 0.40657   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A first-order reaction has k = 0.0150 s^-1. If [A]0 = 0.800 M, what is the concentration after 60.0 s? | SOLVED | A = 325.256 mol/m^3, A_molL = 0.325256, fraction_left = 0.40657 |
| For a first-order decomposition with k = 2.5e-3 min^-1 and an initial concentration of 0.20 M, find the concentration after 2.0 h. | SOLVED | A = 148.164 mol/m^3, A_molL = 0.148164, fraction_left = 0.740818 |

<a id="second_order_concentration"></a>

### Second-order integrated rate law — `second_order_concentration`

The concentration of a reactant after a time in a second-order reaction (rate = k[A]^2).

$$
\frac{1}{[A]} = \frac{1}{[A]_0} + k t
$$

| input | meaning | condition |
|---|---|---|
| `A0` | initial concentration |  |
| `k` | second-order rate constant |  |
| `t` | elapsed time |  |

| output | meaning |
|---|---|
| `A` | concentration at time t |
| `A_molL` | in mol/L |

- **Method:** 1/[A] = 1/[A]$0 + k$ t
- **Assumes:** [A]0, k, $t > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A (concentration at time t) against k (second-order rate constant) from 5](https://www.ai999x.com/?ax_fig=chemistry-ai999x/second-order-concentration-curve)

*A (concentration at time t) against k (second-order rate constant) from 5.0000e-05 m^3/(mol*s) to 0.005 m^3/(mol*s) on a logarithmic axis. One curve for each value of t (elapsed time): 30 s, 15 s, 60 s. Other inputs as in the worked example: A0 = 200.0 mol/m^3. 41 points per curve computed by chemistry-ai999x; marked: the worked example, k = 0.0005 m^3/(mol*s), A = 50 mol/m^3.*

```python
chemistry_ai999x.solve('second_order_concentration', A0=200.0, k=0.0005, t=30.0)
# -> A = 50 mol/m^3, A_molL = 0.05   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A second-order reaction has k = 0.50 M^-1 s^-1. If [A]0 = 0.20 M, what is the concentration after 30 s? | SOLVED | A = 50 mol/m^3, A_molL = 0.05 |

<a id="first_order_half_life"></a>

### Half-life of a first-order reaction — `first_order_half_life`

The half-life of a first-order reaction from its rate constant.

$$
t_{1/2} = \frac{\ln 2}{k}
$$

| input | meaning | condition |
|---|---|---|
| `k` | first-order rate constant |  |

| output | meaning |
|---|---|
| `t_half` | half-life |

- **Method:** $t_{1}/2 = ln$ $2 / k$
- **Assumes:** $k > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![t_half (half-life) against k (first-order rate constant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-half-life-curve)

*t_half (half-life) against k (first-order rate constant) from 0.0125 1/s to 0.0375 1/s. 31 points computed by chemistry-ai999x; marked: the worked example, k = 0.025 1/s, t_half = 27.7259 s.*

```python
chemistry_ai999x.solve('first_order_half_life', k=0.025)
# -> t_half = 27.7259 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the half-life of a first-order reaction with k = 0.0250 s^-1? | SOLVED | t_half = 27.7259 s |
| Find the half-life of a first-order process whose rate constant is 1.2e-4 min^-1. | SOLVED | t_half = 346574 s |

<a id="first_order_time"></a>

### Time to reach a concentration (first order) — `first_order_time`

How long a first-order reaction takes to bring a reactant from one concentration to another.

$$
t = \frac{1}{k}\ln\frac{[A]_0}{[A]}
$$

| input | meaning | condition |
|---|---|---|
| `A0` | initial concentration |  |
| `A` | final concentration |  |
| `k` | first-order rate constant |  |

| output | meaning |
|---|---|
| `t` | time needed |

- **Method:** $t = ln$([A]0/[A]) / k
- **Assumes:** 0 < [A] < [A]0; $k > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![t (time needed) against k (first-order rate constant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-time-curve)

*t (time needed) against k (first-order rate constant) from 0.015 1/s to 0.045 1/s. One curve for each value of A0 (initial concentration): 500 mol/m^3, 250 mol/m^3, 1000 mol/m^3. Other inputs as in the worked example: A = 100.0 mol/m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, k = 0.03 1/s, t = 53.6479 s.*

```python
chemistry_ai999x.solve('first_order_time', A0=500.0, A=100.0, k=0.03)
# -> t = 53.6479 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A first-order reaction has k = 0.0300 s^-1. How long does it take for the concentration to fall from 0.500 M to 0.100 M? | SOLVED | t = 53.6479 s |

<a id="arrhenius_k2"></a>

### Arrhenius equation: rate constant at another temperature — `arrhenius_k2`

The rate constant at a second temperature from the rate constant at a first one and the activation energy.

$$
\ln\frac{k_2}{k_1} = -\frac{E_a}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)
$$

| input | meaning | condition |
|---|---|---|
| `k1` | rate constant at T1 (any units) |  |
| `T1` | first temperature |  |
| `T2` | second temperature |  |
| `Ea` | activation energy |  |

| output | meaning |
|---|---|
| `k2` | rate constant at T2 (units of k1) |
| `ratio` | $k_{2} / k_{1}$ |

- **Method:** two-point Arrhenius equation
- **Assumes:** k1, T1, T2, $Ea > 0$; Ea independent of T
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Arrhenius equation: rate constant at another temperature: k2 against T2 from 250 to 450, 81 runs of the package; other inputs as in the worked example (k1 = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/arrhenius-k2-curve)

*Arrhenius equation: rate constant at another temperature: k2 against T2 from 250 to 450, 81 runs of the package; other inputs as in the worked example (k1 = 0.0012, T1 = 300.0, Ea = 75000.0). Marked: the worked example, T2 = 350, k2 = 0.0880375.*

```python
chemistry_ai999x.solve('arrhenius_k2', k1=0.0012, T1=300.0, T2=350.0, Ea=75000.0)
# -> k2 = 0.0880375, ratio = 73.3646   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A reaction has k = 1.2e-3 s^-1 at 300 K and Ea = 75.0 kJ/mol. What is k at 350 K? | SOLVED | k2 = 0.0880375, ratio = 73.3646 |
| k1 = 0.050 M^-1 s^-1, T1 = 298 K, T2 = 318 K and the activation energy is 52 kJ/mol. Find the rate constant at T2. | SOLVED | k2 = 0.187161, ratio = 3.74321 |

<a id="activation_energy"></a>

### Activation energy from two rate constants — `activation_energy`

The Arrhenius activation energy from rate constants measured at two temperatures.

$$
E_a = \frac{R\ln(k_2/k_1)}{1/T_1 - 1/T_2}
$$

| input | meaning | condition |
|---|---|---|
| `k1` | rate constant at T1 |  |
| `T1` | first temperature |  |
| `k2` | rate constant at T2 |  |
| `T2` | second temperature |  |

| output | meaning |
|---|---|
| `Ea` | activation energy |
| `Ea_kJ` | in kJ/mol |

- **Method:** two-point Arrhenius equation solved for Ea
- **Assumes:** k1, $k_{2} > 0$; $T_{1}  \neq  T_{2}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Ea (activation energy) against T1 (first temperature) from 30 K to 300 K on a logarithmic axis](https://www.ai999x.com/?ax_fig=chemistry-ai999x/activation-energy-curve)

*Ea (activation energy) against T1 (first temperature) from 30 K to 300 K on a logarithmic axis. One curve for each value of k1 (rate constant at T1): 0.001, 0.0005, 0.002. Other inputs as in the worked example: k2 = 0.005, T2 = 320.0 K. 21 points per curve computed by chemistry-ai999x; marked: the worked example, T1 = 300 K, Ea = 64231.7 J/mol.*

```python
chemistry_ai999x.solve('activation_energy', k1=0.001, T1=300.0, k2=0.005, T2=320.0)
# -> Ea = 64231.7 J/mol, Ea_kJ = 64.2317   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| k1 = 1.0e-3 s^-1 at T1 = 300 K and k2 = 5.0e-3 s^-1 at T2 = 320 K. What is the activation energy? | SOLVED | Ea = 64231.7 J/mol, Ea_kJ = 64.2317 |

<a id="arrhenius_rate_constant"></a>

### Arrhenius equation: rate constant — `arrhenius_rate_constant`

The rate constant from the pre-exponential factor, the activation energy and the temperature.

$$
k = A\,e^{-E_a/RT}
$$

| input | meaning | condition |
|---|---|---|
| `A` | pre-exponential factor (units of k) |  |
| `Ea` | activation energy |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `k` | rate constant (units of A) |

- **Method:** $k = A$ exp(-Ea/RT)
- **Assumes:** A, Ea, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Arrhenius plot: the rate constant (logarithmic axis) against 1000/T for A = 1](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/arrhenius-rate-constant-arrhenius.svg)

*Arrhenius plot: the rate constant (logarithmic axis) against 1000/T for A = 1.0000e+13 1/s and Ea = 100 kJ/mol, computed by the package at 61 temperatures from 350 K to 650 K — a straight line of slope −Ea/R. Marked: T = 500 K, k = 357.5 1/s.*

![k (rate constant (units of A)) against T (temperature) from 450 K to 550 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/arrhenius-rate-constant-curve)

*k (rate constant (units of A)) against T (temperature) from 450 K to 550 K. One curve for each value of Ea (activation energy): 100000 J/mol, 50000 J/mol, 200000 J/mol. Other inputs as in the worked example: A = 10000000000000.0. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 500 K, k = 357.5.*

```python
chemistry_ai999x.solve('arrhenius_rate_constant', A=10000000000000.0, Ea=100000.0, T=500.0)
# -> k = 357.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A reaction has a pre-exponential factor of 1.0e13 s^-1 and Ea = 100 kJ/mol. What is the rate constant at 500 K? | SOLVED | k = 357.5 |

<a id="reaction_order_from_rates"></a>

### Reaction order from initial rates — `reaction_order_from_rates`

The order with respect to one reactant from two initial-rate experiments (other concentrations fixed).

$$
n = \frac{\ln(r_2/r_1)}{\ln(c_2/c_1)}
$$

| input | meaning | condition |
|---|---|---|
| `c1` | concentration in experiment 1 |  |
| `r1` | initial rate in experiment 1 |  |
| `c2` | concentration in experiment 2 |  |
| `r2` | initial rate in experiment 2 |  |

| output | meaning |
|---|---|
| `order` | reaction order |

- **Method:** ratio of the two rate laws
- **Assumes:** $c_{1}  \neq  c_{2}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![order (reaction order) against c2 (concentration in experiment 2) from 100 mol/m^3 to 300 mol/m^3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-order-from-rates-curve)

*order (reaction order) against c2 (concentration in experiment 2) from 100 mol/m^3 to 300 mol/m^3. One curve for each value of c1 (concentration in experiment 1): 100 mol/m^3, 50 mol/m^3, 200 mol/m^3. Other inputs as in the worked example: r1 = 2.0 mol/(m^3*s), r2 = 8.0 mol/(m^3*s). 30 points per curve computed by chemistry-ai999x; marked: the worked example, c2 = 200 mol/m^3, order = 2.*

```python
chemistry_ai999x.solve('reaction_order_from_rates', c1=100.0, r1=2.0, c2=200.0, r2=8.0)
# -> order = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| c1 = 0.10 M, r1 = 2.0e-3 M/s, c2 = 0.20 M, r2 = 8.0e-3 M/s. What is the order with respect to A? | SOLVED | order = 2 |

<a id="consecutive_first_order"></a>

### Consecutive first-order reactions (A -> B -> C) — `consecutive_first_order`

Concentrations of the reactant and the intermediate of A -> B -> C (both steps first order) at a time.

$$
[B] = \frac{k_1 [A]_0}{k_2 - k_1}\left(e^{-k_1 t} - e^{-k_2 t}\right)
$$

| input | meaning | condition |
|---|---|---|
| `A0` | initial concentration of A (B and C start at 0) |  |
| `k1` | rate constant of A -> B |  |
| `k2` | rate constant of B -> C |  |
| `t` | elapsed time |  |

| output | meaning |
|---|---|
| `B` | intermediate concentration |
| `A` | reactant |
| `C` | product |

- **Method:** analytic solution of the linear rate equations (the $k_{1} = k_{2}$ limit included)
- **Assumes:** both steps first order; B and C absent at $t = 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The consecutive first-order reaction A → B → C with k1 = 0](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/consecutive-first-order-scheme.svg)

*The consecutive first-order reaction A → B → C with k1 = 0.1 1/s and k2 = 0.05 1/s, starting from A0 = 1000 mol/m³: the concentrations the package returns at t = 10 s are A = 367.879, B = 477.302 and C = 154.818 mol/m³.*

![How the starting 1000 mol/m³ of A divides between A, B and C over time (stacked shares, k1 = 0](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/consecutive-first-order-shares.svg)

*How the starting 1000 mol/m³ of A divides between A, B and C over time (stacked shares, k1 = 0.1 1/s, k2 = 0.05 1/s), each concentration computed by the package at 80 times. B is largest near t = 14 s (499.977 mol/m³, a share of 0.499977); at t = 80 s the shares are 0.000335463, 0.0359604 and 0.963704.*

![B (intermediate concentration) against t (elapsed time) from 1 s to 100 s on a logarithmic axis](https://www.ai999x.com/?ax_fig=chemistry-ai999x/consecutive-first-order-curve)

*B (intermediate concentration) against t (elapsed time) from 1 s to 100 s on a logarithmic axis. One curve for each value of A0 (initial concentration of A (B and C start at 0)): 1000 mol/m^3, 500 mol/m^3, 2000 mol/m^3. Other inputs as in the worked example: k1 = 0.1 1/s, k2 = 0.05 1/s. 41 points per curve computed by chemistry-ai999x; marked: the worked example, t = 10 s, B = 477.302 mol/m^3.*

```python
chemistry_ai999x.solve('consecutive_first_order', A0=1000.0, k1=0.1, k2=0.05, t=10.0)
# -> B = 477.302 mol/m^3, A = 367.879 mol/m^3, C = 154.818 mol/m^3   (SOLVED)
```

```python
chemistry_ai999x.solve('consecutive_first_order', A0=1000.0, k1=0.1, k2=0.1, t=10.0)
# -> B = 367.879 mol/m^3, A = 367.879 mol/m^3, C = 264.241 mol/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For A -> B -> C with k1 = 0.10 s^-1, k2 = 0.050 s^-1 and [A]0 = 1.0 M, what is the concentration of the intermediate B after 10 s? | SOLVED | B = 477.302 mol/m^3, A = 367.879 mol/m^3, C = 154.818 mol/m^3 |

<a id="calorimetry_heat"></a>

### Heat from a temperature change — `calorimetry_heat`

The heat absorbed (positive) or released (negative) by a mass of substance whose temperature changes.

$$
q = m c (T_2 - T_1)
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass |  |
| `c` | specific heat capacity |  |
| `T1` | initial temperature |  |
| `T2` | final temperature |  |

| output | meaning |
|---|---|
| `q` | heat absorbed (negative: released) |

- **Method:** $q = m$ c dT
- **Assumes:** no phase change
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![q (heat absorbed (negative: released)) against m (mass) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/calorimetry-heat-curve)

*q (heat absorbed (negative: released)) against m (mass) from 0.025 kg to 0.075 kg. One curve for each value of T2 (final temperature): 353.15 K, 176.575 K, 706.3 K. Other inputs as in the worked example: c = 4184.0 J/(kg*K), T1 = 293.15 K. 31 points per curve computed by chemistry-ai999x; marked: the worked example, m = 0.05 kg, q = 12552 J.*

```python
chemistry_ai999x.solve('calorimetry_heat', m=0.05, c=4184.0, T1=293.15, T2=353.15)
# -> q = 12552 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How much heat is needed to warm 50.0 g of water from 20.0 °C to 80.0 °C (c = 4.184 J/(g·°C))? | SOLVED | q = 12552 J |
| How much heat is released when 200 g of iron (c = 0.449 J/(g·K)) cools from 400 K to 300 K? | SOLVED | q = -8980 J |

<a id="enthalpy_from_calorimetry"></a>

### Reaction enthalpy from coffee-cup calorimetry — `enthalpy_from_calorimetry`

The molar enthalpy of a reaction from the temperature change of the solution it takes place in.

$$
\Delta H = -\frac{m c (T_2 - T_1)}{n}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass of the solution |  |
| `c` | specific heat of the solution |  |
| `T1` | initial temperature |  |
| `T2` | final temperature |  |
| `n` | moles of the reaction (limiting reactant) |  |

| output | meaning |
|---|---|
| `dH` | reaction enthalpy |
| `dH_kJ` | in kJ/mol |
| `q_solution` | heat taken up by the solution |

- **Method:** heat gained by the solution, with the opposite sign, per mole
- **Assumes:** no heat lost to the calorimeter or surroundings; constant pressure
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dH (reaction enthalpy) against n (moles of the reaction (limiting reactant)) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enthalpy-from-calorimetry-curve)

*dH (reaction enthalpy) against n (moles of the reaction (limiting reactant)) from 0.005 mol to 0.5 mol on a logarithmic axis. One curve for each value of T2 (final temperature): 301.95 K, 150.975 K, 603.9 K. Other inputs as in the worked example: m = 0.1 kg, c = 4180.0 J/(kg*K), T1 = 295.15 K. 41 points per curve computed by chemistry-ai999x; marked: the worked example, n = 0.05 mol, dH = -56848 J/mol.*

```python
chemistry_ai999x.solve('enthalpy_from_calorimetry', m=0.1, c=4180.0, T1=295.15, T2=301.95, n=0.05)
# -> dH = -56848 J/mol, dH_kJ = -56.848, q_solution = 2842.4 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| When 0.0500 mol of HCl reacts in 100 g of solution (c = 4.18 J/(g·°C)), the temperature rises from 22.0 °C to 28.8 °C. What is the enthalpy of reaction? | SOLVED | dH = -56848 J/mol, dH_kJ = -56.848, q_solution = 2842.4 J |

<a id="reaction_enthalpy_from_formation"></a>

### Reaction enthalpy from enthalpies of formation (Hess's law) — `reaction_enthalpy_from_formation`

The standard reaction enthalpy from the standard enthalpies of formation of every species (balanced equation).

$$
\Delta H^\circ_{rxn} = \sum \nu_p \Delta H_f^\circ(p) - \sum \nu_r \Delta H_f^\circ(r)
$$

| input | meaning | condition |
|---|---|---|
| `equation` | balanced equation |  |
| `values` | standard enthalpy of formation of each species in the order written (kJ/mol) |  |

| output | meaning |
|---|---|
| `dH` | standard reaction enthalpy |
| `dH_kJ` | in kJ/mol |

- **Method:** coefficient-weighted sum of formation enthalpies, products minus reactants
- **Assumes:** the equation as written conserves every element; one value per species, in kJ/mol
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dH (standard reaction enthalpy) as value 4 of values (standard enthalpy of formation of each species in the order written (kJ/mol)) moves from -590](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-enthalpy-from-formation-sensitivity)

*dH (standard reaction enthalpy) as value 4 of values (standard enthalpy of formation of each species in the order written (kJ/mol)) moves from -590.25 to 196.75, the other values as given (-74.8, 0, -393.5, -285.8); 41 runs of chemistry-ai999x. Other inputs: equation = CH4 + 2 O2 -> CO2 + 2 H2O. Marked: the worked example, values[4] = -285.8, dH = -890300 J/mol.*

```python
chemistry_ai999x.solve('reaction_enthalpy_from_formation', equation='CH4 + 2 O2 -> CO2 + 2 H2O', values='-74.8, 0, -393.5, -285.8')
# -> dH = -890300 J/mol, dH_kJ = -890.3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Using enthalpies of formation CH4 = -74.8, O2 = 0, CO2 = -393.5, H2O = -285.8 (kJ/mol), find ΔH° for CH4 + 2 O2 -> CO2 + 2 H2O. | SOLVED | dH = -890300 J/mol, dH_kJ = -890.3 |
| Using enthalpies of formation CH4 = -74.8, O2 = 0, CO2 = -393.5, H2O = -285.8 (kJ/mol), find ΔH° for CH4 + O2 -> CO2 + H2O. | INVALID_INPUT | Reaction enthalpy from enthalpies of formation (Hess's law): the equation as written is not balanced; balance it first. |

<a id="gibbs_free_energy"></a>

### Gibbs energy change (dG = dH - T dS) — `gibbs_free_energy`

The Gibbs energy change of a process from its enthalpy and entropy changes, and whether it is spontaneous.

$$
\Delta G = \Delta H - T\Delta S
$$

| input | meaning | condition |
|---|---|---|
| `dH` | standard enthalpy change |  |
| `dS` | entropy change |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `dG` | Gibbs energy change |
| `dG_kJ` | in kJ/mol |
| `spontaneity` | spontaneous / non-spontaneous |

- **Method:** $dG = dH - T$ dS
- **Assumes:** dH and dS taken as constant
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Gibbs energy change (dG = dH - T dS): dG against T from 100 to 1000, 81 runs of the package; other inputs as in the worked example (dH = -92200](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gibbs-free-energy-curve)

*Gibbs energy change (dG = dH - T dS): dG against T from 100 to 1000, 81 runs of the package; other inputs as in the worked example (dH = -92200.0, dS = -198.7). Marked: the worked example, T = 298, dG = -32987.4.*

![spontaneous / non-spontaneous (spontaneity) over dH (standard enthalpy change) from -138300 J/mol to -46100 J/mol and T (temperature) from 29](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gibbs-free-energy-decision)

*spontaneous / non-spontaneous (spontaneity) over dH (standard enthalpy change) from -138300 J/mol to -46100 J/mol and T (temperature) from 29.8 K to 2980 K (logarithmic vertical axis): each of the 31 × 21 cells is one run of chemistry-ai999x, coloured by the verdict it returned — non-spontaneous: 273 cells, spontaneous: 378 cells. Other inputs as in the worked example: dS = -198.7 J/(mol*K). Marked: the worked example, dH = -92200 J/mol, T = 298 K → spontaneous.*

```python
chemistry_ai999x.solve('gibbs_free_energy', dH=-92200.0, dS=-198.7, T=298.0)
# -> dG = -32987.4 J/mol, dG_kJ = -32.9874, spontaneity = spontaneous   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For a reaction ΔH = -92.2 kJ/mol and ΔS = -198.7 J/(mol·K). What is ΔG at 298 K? Is it spontaneous? | SOLVED | dG = -32987.4 J/mol, dG_kJ = -32.9874, spontaneity = spontaneous |
| ΔH = 178 kJ/mol, ΔS = 161 J/(mol·K). Find ΔG at 1000 K. | SOLVED | dG = 17000 J/mol, dG_kJ = 17, spontaneity = non-spontaneous |

<a id="crossover_temperature"></a>

### Temperature at which dG = 0 — `crossover_temperature`

The temperature at which a reaction changes between spontaneous and non-spontaneous $(dG = 0)$.

$$
T = \frac{\Delta H}{\Delta S}
$$

| input | meaning | condition |
|---|---|---|
| `dH` | standard enthalpy change |  |
| `dS` | entropy change |  |

| output | meaning |
|---|---|
| `T` | temperature where $dG = 0$ |
| `T_C` | in °C |

- **Method:** $T = dH / dS$
- **Assumes:** dH and dS of the same sign
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![T (temperature where dG = 0) against dS (entropy change) from 80](https://www.ai999x.com/?ax_fig=chemistry-ai999x/crossover-temperature-curve)

*T (temperature where dG = 0) against dS (entropy change) from 80.5 J/(mol*K) to 241.5 J/(mol*K). One curve for each value of dH (standard enthalpy change): 178000 J/mol, 89000 J/mol, 356000 J/mol. 31 points per curve computed by chemistry-ai999x; marked: the worked example, dS = 161 J/(mol*K), T = 1105.59 K.*

![T (temperature where dG = 0) over dH (standard enthalpy change) from 89000 J/mol to 267000 J/mol and dS (entropy change) from 80](https://www.ai999x.com/?ax_fig=chemistry-ai999x/crossover-temperature-field)

*T (temperature where dG = 0) over dH (standard enthalpy change) from 89000 J/mol to 267000 J/mol and dS (entropy change) from 80.5 J/(mol*K) to 241.5 J/(mol*K): each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light 368.53 K to dark 3316.77 K); iso-lines at T = 500, 1000, 1500, 2000, 2500, 3000 K. Marked: the worked example, dH = 178000 J/mol, dS = 161 J/(mol*K), T = 1105.59 K.*

```python
chemistry_ai999x.solve('crossover_temperature', dH=178000.0, dS=161.0)
# -> T = 1105.59 K, T_C = 832.44   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For ΔH = 178 kJ/mol and ΔS = 161 J/(mol·K), above what temperature is the reaction spontaneous? | SOLVED | T = 1105.59 K, T_C = 832.44 |
| ΔH = -50 kJ/mol and ΔS = 100 J/(mol·K). At what temperature does the reaction become spontaneous? | INVALID_INPUT | Temperature at which dG = 0: dH and dS must have the same sign for a temperature where dG = 0 to exist. |

<a id="heat_from_reaction_mass"></a>

### Heat of reaction for a given mass — `heat_from_reaction_mass`

The heat of a reaction for a given mass of a reactant, from its molar enthalpy (per mole of that reactant).

$$
q = \frac{m}{M}\Delta H
$$

| input | meaning | condition |
|---|---|---|
| `formula` | formula of the reactant |  |
| `m` | mass of the reactant |  |
| `dH` | standard enthalpy change |  |

| output | meaning |
|---|---|
| `q` | enthalpy change (negative: heat released) |
| `heat_released` | heat released |

- **Method:** moles of reactant times its molar enthalpy
- **Assumes:** dH is per mole of the given reactant
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![q (enthalpy change (negative: heat released)) against m (mass of the reactant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/heat-from-reaction-mass-curve)

*q (enthalpy change (negative: heat released)) against m (mass of the reactant) from 0.005 kg to 0.015 kg. One curve for each value of dH (standard enthalpy change): -890000 J/mol, -445000 J/mol, -1.78e+06 J/mol. Other inputs as in the worked example: formula = CH4. 31 points per curve computed by chemistry-ai999x; marked: the worked example, m = 0.01 kg, q = -554759 J.*

![q (enthalpy change (negative: heat released)) over m (mass of the reactant) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/heat-from-reaction-mass-field)

*q (enthalpy change (negative: heat released)) over m (mass of the reactant) from 0.005 kg to 0.015 kg and dH (standard enthalpy change) from -1.335e+06 J/mol to -445000 J/mol: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light -1.24821e+06 J to dark -138690 J); iso-lines at q = -1.20e6, -1.00e6, -8.00e5, -6.00e5, -4.00e5, -2.00e5 J. Other inputs as in the worked example: formula = CH4. Marked: the worked example, m = 0.01 kg, dH = -890000 J/mol, q = -554759 J.*

```python
chemistry_ai999x.solve('heat_from_reaction_mass', formula='CH4', m=0.01, dH=-890000.0)
# -> q = -554759 J, heat_released = 554759 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How much heat is released when 10.0 g of CH4 burns? ΔH = -890 kJ/mol. | SOLVED | q = -554759 J, heat_released = 554759 J |

<a id="clausius_clapeyron_pressure"></a>

### Clausius-Clapeyron: vapor pressure at another temperature — `clausius_clapeyron_pressure`

The vapor pressure of a liquid at a second temperature from one known point and the enthalpy of vaporization.

$$
\ln\frac{P_2}{P_1} = -\frac{\Delta H_{vap}}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)
$$

| input | meaning | condition |
|---|---|---|
| `P1` | vapor pressure at T1 |  |
| `T1` | first temperature |  |
| `T2` | second temperature |  |
| `dH` | enthalpy of vaporization |  |

| output | meaning |
|---|---|
| `P2` | vapor pressure at T2 |

- **Method:** integrated Clausius-Clapeyron equation
- **Assumes:** dHvap constant; ideal vapor
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Clausius-Clapeyron: vapor pressure at another temperature: P2 against T2 from 273](https://www.ai999x.com/?ax_fig=chemistry-ai999x/clausius-clapeyron-pressure-curve)

*Clausius-Clapeyron: vapor pressure at another temperature: P2 against T2 from 273.15 to 373.15, 81 runs of the package; other inputs as in the worked example (P1 = 3173.0723684210525, T1 = 298.15, dH = 43900.0). Marked: the worked example, T2 = 323.15, P2 = 12487.6.*

```python
chemistry_ai999x.solve('clausius_clapeyron_pressure', P1=3173.0723684210525, T1=298.15, T2=323.15, dH=43900.0)
# -> P2 = 12487.6 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Water has a vapor pressure of 23.8 torr at 25 °C and ΔHvap = 43.9 kJ/mol. What is its vapor pressure at 50 °C? | SOLVED | P2 = 12487.6 Pa |

<a id="clausius_clapeyron_enthalpy"></a>

### Clausius-Clapeyron: enthalpy of vaporization — `clausius_clapeyron_enthalpy`

The enthalpy of vaporization from vapor pressures at two temperatures.

$$
\Delta H_{vap} = \frac{R\ln(P_2/P_1)}{1/T_1 - 1/T_2}
$$

| input | meaning | condition |
|---|---|---|
| `P1` | vapor pressure at T1 |  |
| `T1` | first temperature |  |
| `P2` | vapor pressure at T2 |  |
| `T2` | second temperature |  |

| output | meaning |
|---|---|
| `dH` | enthalpy of vaporization |

- **Method:** two-point Clausius-Clapeyron equation
- **Assumes:** $T_{1}  \neq  T_{2}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dH (enthalpy of vaporization) against T1 (first temperature) from 30 K to 3000 K on a logarithmic axis](https://www.ai999x.com/?ax_fig=chemistry-ai999x/clausius-clapeyron-enthalpy-curve)

*dH (enthalpy of vaporization) against T1 (first temperature) from 30 K to 3000 K on a logarithmic axis. One curve for each value of T2 (second temperature): 340 K, 170 K, 680 K. Other inputs as in the worked example: P1 = 13332.236842105263 Pa, P2 = 53328.94736842105 Pa. 41 points per curve computed by chemistry-ai999x; marked: the worked example, T1 = 300 K, dH = 29392 J/mol.*

```python
chemistry_ai999x.solve('clausius_clapeyron_enthalpy', P1=13332.236842105263, T1=300.0, P2=53328.94736842105, T2=340.0)
# -> dH = 29392 J/mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| P1 = 100 torr at T1 = 300 K and P2 = 400 torr at T2 = 340 K. What is the enthalpy of vaporization? | SOLVED | dH = 29392 J/mol |

<a id="delta_g_from_cell_potential"></a>

### Gibbs energy from a cell potential — `delta_g_from_cell_potential`

The (standard) Gibbs energy change of a cell reaction from its potential (dG = -nFE).

$$
\Delta G = -nFE
$$

| input | meaning | condition |
|---|---|---|
| `n` | electrons transferred |  |
| `E` | cell potential |  |

| output | meaning |
|---|---|
| `dG` | Gibbs energy change |
| `dG_kJ` | in kJ/mol |

- **Method:** $dG = -n$ F E
- **Assumes:** $n > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![dG (Gibbs energy change) against n (electrons transferred) from 1 to 3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-cell-potential-curve)

*dG (Gibbs energy change) against n (electrons transferred) from 1 to 3. One curve for each value of E (cell potential): 1.1 V, 0.55 V, 2.2 V. 31 points per curve computed by chemistry-ai999x; marked: the worked example, n = 2, dG = -212268 J/mol.*

![dG (Gibbs energy change) over n (electrons transferred) from 1 to 3 and E (cell potential) from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-cell-potential-field)

*dG (Gibbs energy change) over n (electrons transferred) from 1 to 3 and E (cell potential) from 0.55 V to 1.65 V: each of the 21 × 15 cells is one run of chemistry-ai999x, shaded by its value (light -477602 J/mol to dark -53066.9 J/mol); iso-lines at dG = -4.00e5, -3.00e5, -2.00e5, -1.00e5 J/mol. Marked: the worked example, n = 2, E = 1.1 V, dG = -212268 J/mol.*

```python
chemistry_ai999x.solve('delta_g_from_cell_potential', n=2.0, E=1.1)
# -> dG = -212268 J/mol, dG_kJ = -212.268   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A cell has E° = 1.10 V and n = 2. What is ΔG°? | SOLVED | dG = -212268 J/mol, dG_kJ = -212.268 |
| A cell has E° = 1.10 V. What is ΔG°? | NEEDS_INPUT | delta_g_from_cell_potential needs n (electrons transferred). Example: solve('delta_g_from_cell_potential', n=2, E='1.1 V |

<a id="k_from_cell_potential"></a>

### Equilibrium constant from a standard cell potential — `k_from_cell_potential`

The equilibrium constant of a cell reaction from its standard potential (ln K = nFE°/RT).

$$
\ln K = \frac{nFE^\circ}{RT}
$$

| input | meaning | condition |
|---|---|---|
| `n` | electrons transferred |  |
| `E` | standard cell potential |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `lnK` | ln K |
| `K` | K |

- **Method:** ln $K = n$ F E° / (R T)
- **Assumes:** n, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![lnK (ln K) against T (temperature) from 149 K to 447 K](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-cell-potential-curve)

*lnK (ln K) against T (temperature) from 149 K to 447 K. One curve for each value of n (electrons transferred): 2, 1, 4. Other inputs as in the worked example: E = 0.46 V. 31 points per curve computed by chemistry-ai999x; marked: the worked example, T = 298 K, lnK = 35.826.*

```python
chemistry_ai999x.solve('k_from_cell_potential', n=2.0, E=0.46, T=298.0)
# -> lnK = 35.826, K = 3.62282e+15   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A cell has E° = 0.46 V with n = 2 at 298 K. What is the equilibrium constant? | SOLVED | lnK = 35.826, K = 3.62282e+15 |

<a id="nernst_potential"></a>

### Nernst equation — `nernst_potential`

The cell potential at non-standard conditions from E°, the electrons transferred and the reaction quotient.

$$
E = E^\circ - \frac{RT}{nF}\ln Q
$$

| input | meaning | condition |
|---|---|---|
| `E0` | standard cell potential |  |
| `n` | electrons transferred |  |
| `Q` | reaction quotient |  |
| `T` | temperature |  |

| output | meaning |
|---|---|
| `E` | cell potential |

- **Method:** $E = E$° - (RT/nF) ln Q
- **Assumes:** n, Q, $T > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Nernst equation: E against Q from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/nernst-potential-curve)

*Nernst equation: E against Q from 1.0000e-06 to 1e+06 (logarithmic axis), 81 runs of the package; other inputs as in the worked example (E0 = 1.1, n = 2.0, T = 298.0). Marked: the worked example, Q = 0.01, E = 1.15913.*

```python
chemistry_ai999x.solve('nernst_potential', E0=1.1, n=2.0, Q=0.01, T=298.0)
# -> E = 1.15913 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| E° = 1.10 V, n = 2, Q = 0.010 at 298 K. What is the cell potential? | SOLVED | E = 1.15913 V |
| Use the Nernst equation: E° = 0.80 V, n = 1, reaction quotient 50, T = 310 K. Find E. | SOLVED | E = 0.695495 V |

<a id="electrolysis_mass"></a>

### Mass deposited in electrolysis (Faraday's law) — `electrolysis_mass`

The mass of a metal deposited by a current flowing for a time.

$$
m = \frac{I t}{n F} M
$$

| input | meaning | condition |
|---|---|---|
| `metal` | the element deposited (symbol or name) |  |
| `I` | current |  |
| `t` | time |  |
| `n` | electrons transferred |  |

| output | meaning |
|---|---|
| `m` | mass deposited |
| `m_g` | in grams |

- **Method:** charge / (n F) gives moles of metal; times its molar mass
- **Assumes:** 100% current efficiency
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![m (mass deposited) against n (electrons transferred) from 1 to 3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/electrolysis-mass-curve)

*m (mass deposited) against n (electrons transferred) from 1 to 3. One curve for each value of I (current): 2 A, 1 A, 4 A. Other inputs as in the worked example: metal = Cu, t = 1800.0 s. 31 points per curve computed by chemistry-ai999x; marked: the worked example, n = 2, m = 0.00118549 kg.*

```python
chemistry_ai999x.solve('electrolysis_mass', metal='Cu', I=2.0, t=1800.0, n=2.0)
# -> m = 0.00118549 kg, m_g = 1.18549   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What mass of copper is deposited from a Cu2+ solution by a current of 2.00 A flowing for 30.0 min? | SOLVED | m = 0.00118549 kg, m_g = 1.18549 |
| What mass of silver is plated from Ag+ by 0.500 A in 2.00 h? | SOLVED | m = 0.0040247 kg, m_g = 4.0247 |

<a id="electrolysis_time"></a>

### Time for electrolysis (Faraday's law) — `electrolysis_time`

How long a current must flow to deposit a given mass of a metal.

$$
t = \frac{m n F}{M I}
$$

| input | meaning | condition |
|---|---|---|
| `metal` | the element deposited (symbol or name) |  |
| `m` | mass to deposit |  |
| `I` | current |  |
| `n` | electrons transferred |  |

| output | meaning |
|---|---|
| `t` | time needed |

- **Method:** $t = m$ n F / (M I)
- **Assumes:** 100% current efficiency
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![t (time needed) against I (current) from 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/electrolysis-time-curve)

*t (time needed) against I (current) from 2.5 A to 7.5 A. One curve for each value of m (mass to deposit): 0.01 kg, 0.005 kg, 0.02 kg. Other inputs as in the worked example: metal = Ni, n = 2.0. 31 points per curve computed by chemistry-ai999x; marked: the worked example, I = 5 A, t = 6575.59 s.*

```python
chemistry_ai999x.solve('electrolysis_time', metal='Ni', m=0.01, I=5.0, n=2.0)
# -> t = 6575.59 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How long must a current of 5.00 A flow to deposit 10.0 g of Ni from a Ni2+ solution? | SOLVED | t = 6575.59 s |

<a id="photon_energy"></a>

### Energy of a photon from its wavelength — `photon_energy`

The energy (per photon and per mole of photons) and frequency of light of a given wavelength.

$$
E = \frac{hc}{\lambda}
$$

| input | meaning | condition |
|---|---|---|
| `lam` | wavelength |  |

| output | meaning |
|---|---|
| `E` | energy per photon |
| `E_eV` | in eV |
| `E_molar` | energy per mole of photons |
| `nu` | frequency |

- **Method:** $E = h$ c / lambda
- **Assumes:** $\lambda > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![E (energy per photon) against lam (wavelength) from 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/photon-energy-curve)

*E (energy per photon) against lam (wavelength) from 2.5000e-07 m to 7.5000e-07 m. 31 points computed by chemistry-ai999x; marked: the worked example, lam = 5.0000e-07 m, E = 3.9729e-19 J.*

```python
chemistry_ai999x.solve('photon_energy', lam=5e-07)
# -> E = 3.97289e-19 J, E_eV = 2.47968, E_molar = 239253 J/mol, nu = 5.99585e+14 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the energy of a photon of wavelength 500 nm? | SOLVED | E = 3.97289e-19 J, E_eV = 2.47968, E_molar = 239253 J/mol, nu = 5.99585e+14 Hz |
| Calculate the energy per mole of photons of light with a wavelength of 254 nm. | SOLVED | E = 7.82065e-19 J, E_eV = 4.88127, E_molar = 470971 J/mol, nu = 1.18029e+15 Hz |

<a id="photon_wavelength"></a>

### Wavelength of a photon from its energy — `photon_wavelength`

The wavelength of a photon of a given energy.

$$
\lambda = \frac{hc}{E}
$$

| input | meaning | condition |
|---|---|---|
| `E` | photon energy |  |

| output | meaning |
|---|---|
| `lam` | wavelength |
| `lam_nm` | in nm |

- **Method:** $\lambda = h$ c / E
- **Assumes:** $E > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![lam (wavelength) against E (photon energy) from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/photon-wavelength-curve)

*lam (wavelength) against E (photon energy) from 1.5000e-19 J to 4.5000e-19 J. 31 points computed by chemistry-ai999x; marked: the worked example, E = 3.0000e-19 J, lam = 6.6215e-07 m.*

```python
chemistry_ai999x.solve('photon_wavelength', E=3e-19)
# -> lam = 6.62149e-07 m, lam_nm = 662.149   (SOLVED)
```

```python
chemistry_ai999x.solve('photon_wavelength', E=4.005441585e-19)
# -> lam = 4.95937e-07 m, lam_nm = 495.937   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the wavelength of a photon with an energy of 3.0e-19 J? | SOLVED | lam = 6.62149e-07 m, lam_nm = 662.149 |
| Find the wavelength of a 2.5 eV photon. | SOLVED | lam = 4.95937e-07 m, lam_nm = 495.937 |

<a id="beer_lambert_absorbance"></a>

### Beer-Lambert law: absorbance — `beer_lambert_absorbance`

The absorbance of a solution from its molar absorptivity, path length and concentration.

$$
A = \varepsilon l c
$$

| input | meaning | condition |
|---|---|---|
| `eps` | molar absorptivity |  |
| `l` | path length |  |
| `c` | concentration |  |

| output | meaning |
|---|---|
| `A` | absorbance |
| `T_percent` | percent transmittance |

- **Method:** A = epsilon l c
- **Assumes:** dilute solution, monochromatic light
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A (absorbance) against eps (molar absorptivity) from 750 m^2/mol to 2250 m^2/mol](https://www.ai999x.com/?ax_fig=chemistry-ai999x/beer-lambert-absorbance-curve)

*A (absorbance) against eps (molar absorptivity) from 750 m^2/mol to 2250 m^2/mol. One curve for each value of l (path length): 0.01 m, 0.005 m, 0.02 m. Other inputs as in the worked example: c = 0.02 mol/m^3. 31 points per curve computed by chemistry-ai999x; marked: the worked example, eps = 1500 m^2/mol, A = 0.3.*

```python
chemistry_ai999x.solve('beer_lambert_absorbance', eps=1500.0, l=0.01, c=0.02)
# -> A = 0.3, T_percent = 50.1187   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A 1.00 cm cell holds a 2.0e-5 M solution with molar absorptivity 15000 L/(mol·cm). What is the absorbance? | SOLVED | A = 0.3, T_percent = 50.1187 |

<a id="beer_lambert_concentration"></a>

### Beer-Lambert law: concentration — `beer_lambert_concentration`

The concentration of a solution from its absorbance, molar absorptivity and path length.

$$
c = \frac{A}{\varepsilon l}
$$

| input | meaning | condition |
|---|---|---|
| `A` | absorbance |  |
| `eps` | molar absorptivity |  |
| `l` | path length |  |

| output | meaning |
|---|---|
| `c` | concentration |
| `c_molL` | in mol/L |

- **Method:** c = A / (epsilon l)
- **Assumes:** dilute solution
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![c (concentration) against eps (molar absorptivity) from 625 m^2/mol to 1875 m^2/mol](https://www.ai999x.com/?ax_fig=chemistry-ai999x/beer-lambert-concentration-curve)

*c (concentration) against eps (molar absorptivity) from 625 m^2/mol to 1875 m^2/mol. One curve for each value of A (absorbance): 0.45, 0.225, 0.9. Other inputs as in the worked example: l = 0.01 m. 31 points per curve computed by chemistry-ai999x; marked: the worked example, eps = 1250 m^2/mol, c = 0.036 mol/m^3.*

```python
chemistry_ai999x.solve('beer_lambert_concentration', A=0.45, eps=1250.0, l=0.01)
# -> c = 0.036 mol/m^3, c_molL = 3.6e-05   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A solution has an absorbance of 0.450 in a 1.00 cm cell; ε = 1.25e4 L/(mol·cm). What is its concentration? | SOLVED | c = 0.036 mol/m^3, c_molL = 3.6e-05 |

<a id="hydrogen_transition_wavelength"></a>

### Hydrogen atom transition wavelength (Rydberg) — `hydrogen_transition_wavelength`

The wavelength of the photon emitted or absorbed in a transition between two levels of hydrogen.

$$
\frac{1}{\lambda} = R_H\left(\frac{1}{n_1^2} - \frac{1}{n_2^2}\right)
$$

| input | meaning | condition |
|---|---|---|
| `n_upper` | upper level |  |
| `n_lower` | lower level |  |

| output | meaning |
|---|---|
| `lam` | wavelength |
| `lam_nm` | in nm |
| `E` | photon energy |

- **Method:** Rydberg formula with the hydrogen Rydberg constant (reduced-mass corrected)
- **Assumes:** hydrogen atom
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The hydrogen line spectrum: wavelength of the first six lines of three series, computed by the package (Rydberg formula), each drawn at its photon energy (also computed by the package)](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/hydrogen-transition-wavelength-spectrum.svg)

*The hydrogen line spectrum: wavelength of the first six lines of three series, computed by the package (Rydberg formula), each drawn at its photon energy (also computed by the package). Ranges: Lyman 121.568–93.0758 nm; Balmer 656.47–389.019 nm; Paschen 1875.63–923.16 nm. Marked: Hα (n = 3 → 2), λ = 656.47 nm, E = 1.88865 eV.*

![lam (wavelength) against n_upper (upper level) from 1 to 11](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydrogen-transition-wavelength-curve)

*lam (wavelength) against n_upper (upper level) from 1 to 11. One curve for each value of n_lower (lower level): 2, 3. 9 points per curve computed by chemistry-ai999x; marked: the worked example, n_upper = 3, lam = 6.5647e-07 m.*

```python
chemistry_ai999x.solve('hydrogen_transition_wavelength', n_upper=3, n_lower=2)
# -> lam = 6.5647e-07 m, lam_nm = 656.47, E = 3.02595e-19 J   (SOLVED)
```

```python
chemistry_ai999x.solve('hydrogen_transition_wavelength', n_upper=2, n_lower=1)
# -> lam = 1.21568e-07 m, lam_nm = 121.568, E = 1.63401e-18 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the wavelength of light emitted when an electron in hydrogen falls from n = 3 to n = 2? | SOLVED | lam = 6.5647e-07 m, lam_nm = 656.47, E = 3.02595e-19 J |
| Find the wavelength of the hydrogen transition from n = 1 to n = 4. | SOLVED | lam = 9.72548e-08 m, lam_nm = 97.2548, E = 2.04252e-18 J |

<a id="degree_of_unsaturation"></a>

### Degree of unsaturation — `degree_of_unsaturation`

The number of rings plus pi bonds implied by a molecular formula.

$$
\mathrm{DoU} = \frac{2C + 2 + N - H - X}{2}
$$

| input | meaning | condition |
|---|---|---|
| `formula` | molecular formula |  |

| output | meaning |
|---|---|
| `dou` | degree of unsaturation (rings + pi bonds) |

- **Method:** $(2C + 2 + N - H - X)/2$ (Si as C, P and B as N, O and S ignored)
- **Assumes:** a neutral organic formula
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Degree of unsaturation (rings plus π bonds) of eight molecules, each returned by the package from its formula: ethane 0, ethene 1, ethyne 2, cyclohexane 1, benzene 4, pyridine 4, naphthalene 7, vinyl chloride 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/degree-of-unsaturation-by-molecule)

*Degree of unsaturation (rings plus π bonds) of eight molecules, each returned by the package from its formula: ethane 0, ethene 1, ethyne 2, cyclohexane 1, benzene 4, pyridine 4, naphthalene 7, vinyl chloride 1. The worked example, C₆H₆ (benzene), gives 4: one ring and three double bonds.*

```python
chemistry_ai999x.solve('degree_of_unsaturation', formula='C6H6')
# -> dou = 4   (SOLVED)
```

```python
chemistry_ai999x.solve('degree_of_unsaturation', formula='C5H9NO')
# -> dou = 2   (SOLVED)
```

```python
chemistry_ai999x.solve('degree_of_unsaturation', formula='C4H8Cl2')
# -> dou = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the degree of unsaturation of C6H6? | SOLVED | dou = 4 |
| Find the degree of unsaturation for C5H9NO. | SOLVED | dou = 2 |
| What is the degree of unsaturation of C2H7? | INVALID_INPUT | Degree of unsaturation: this formula has no valid structure (negative or half-integer unsaturation). |

<a id="vibrational_wavenumber"></a>

### Vibrational wavenumber of a diatomic molecule — `vibrational_wavenumber`

The harmonic vibrational wavenumber of a diatomic molecule from its bond force constant.

$$
\tilde{\nu} = \frac{1}{2\pi c}\sqrt{\frac{k}{\mu}}
$$

| input | meaning | condition |
|---|---|---|
| `molecule` | diatomic molecule, e.g. HCl or CO |  |
| `k` | bond force constant |  |

| output | meaning |
|---|---|
| `wavenumber` | wavenumber |
| `wavenumber_cm` | in $cm^{-1}$ |
| `nu` | vibrational frequency |

- **Method:** harmonic oscillator with the reduced mass of the most abundant isotopes
- **Assumes:** harmonic approximation; most abundant isotopes (1H, 12C, 16O, 35Cl ...)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![wavenumber (wavenumber) against k (bond force constant) from 258 N/m to 774 N/m](https://www.ai999x.com/?ax_fig=chemistry-ai999x/vibrational-wavenumber-curve)

*wavenumber (wavenumber) against k (bond force constant) from 258 N/m to 774 N/m. Other inputs as in the worked example: molecule = HCl. 31 points computed by chemistry-ai999x; marked: the worked example, k = 516 N/m, wavenumber = 299004 1/m.*

```python
chemistry_ai999x.solve('vibrational_wavenumber', molecule='HCl', k=516.0)
# -> wavenumber = 299004 1/m, wavenumber_cm = 2990.04, nu = 8.96391e+13 Hz   (SOLVED)
```

```python
chemistry_ai999x.solve('vibrational_wavenumber', molecule='CO', k=1902.0)
# -> wavenumber = 216989 1/m, wavenumber_cm = 2169.89, nu = 6.50517e+13 Hz   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the vibrational wavenumber of HCl if its force constant is 516 N/m? | SOLVED | wavenumber = 299004 1/m, wavenumber_cm = 2990.04, nu = 8.96391e+13 Hz |
| Find the vibrational frequency of CO with a force constant of 1902 N/m. | SOLVED | wavenumber = 216989 1/m, wavenumber_cm = 2169.89, nu = 6.50517e+13 Hz |
