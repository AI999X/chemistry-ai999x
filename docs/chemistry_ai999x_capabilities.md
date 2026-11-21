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
| Acids and bases / conjugate pairs | [Conjugate acid-base constants (Ka Kb = Kw)](#conjugate_dissociation_constant) |
| Acids and bases / pH | [PH from the hydronium concentration](#ph_from_hydronium), [Hydronium concentration from pH](#hydronium_from_ph), [PH from the hydroxide concentration](#ph_from_hydroxide) |
| Acids and bases / polyprotic acids | [PH of a diprotic acid](#ph_diprotic_acid) |
| Acids and bases / salt hydrolysis | [PH of a salt of a weak acid](#ph_salt_of_weak_acid) |
| Acids and bases / strong acids | [PH of a strong monoprotic acid](#ph_strong_acid) |
| Acids and bases / strong bases | [PH of a strong base](#ph_strong_base) |
| Acids and bases / titrations | [Titration: equivalence-point volume](#titration_equivalence_volume), [PH during a strong acid - strong base titration](#titration_ph_strong_acid_strong_base), [PH during a weak acid - strong base titration](#titration_ph_weak_acid_strong_base) |
| Acids and bases / weak acids | [PH of a weak acid](#ph_weak_acid), [Ka from the pH of a weak acid](#ka_from_ph) |
| Acids and bases / weak bases | [PH of a weak base](#ph_weak_base) |
| Atomic structure / isotopes | [Average atomic mass from isotopes](#average_atomic_mass), [Abundances of two isotopes from the average atomic mass](#isotope_abundance) |
| Electrochemistry / cell potentials | [Nernst equation](#nernst_potential), [Standard cell potential from two half-reactions](#standard_cell_potential) |
| Electrochemistry / cell thermodynamics | [Gibbs energy from a cell potential](#delta_g_from_cell_potential), [Equilibrium constant from a standard cell potential](#k_from_cell_potential) |
| Electrochemistry / electrolysis | [Mass deposited in electrolysis (Faraday's law)](#electrolysis_mass), [Time for electrolysis (Faraday's law)](#electrolysis_time) |
| Equilibrium / equilibrium constants | [Kp from Kc](#kp_from_kc), [Kc from Kp](#kc_from_kp) |
| Equilibrium / ICE tables | [Equilibrium concentrations (ICE table)](#equilibrium_ice) |
| Equilibrium / reaction quotient | [Reaction quotient and the direction of shift](#reaction_quotient_direction) |
| Equilibrium / solubility | [Molar solubility from Ksp](#ksp_molar_solubility), [Ksp from molar solubility](#ksp_from_solubility), [Molar solubility with a common ion](#common_ion_solubility), [Precipitation test (Q versus Ksp)](#precipitation_test) |
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
| Nuclear chemistry / binding energy | [Mass defect and nuclear binding energy](#nuclear_binding_energy) |
| Nuclear chemistry / radioactive decay | [Amount remaining after radioactive decay (half-life)](#radioactive_decay_remaining), [Age from the fraction remaining (radioactive dating)](#radioactive_dating_age) |
| Organic chemistry / stereochemistry | [Specific rotation from an observed rotation](#specific_rotation), [Enantiomeric excess from optical rotation](#enantiomeric_excess_from_rotation), [Enantiomeric excess from the composition of a mixture](#enantiomeric_excess_from_composition) |
| Solutions / colligative properties | [Vapor pressure of a solution (Raoult's law)](#raoult_vapor_pressure), [Freezing-point depression](#freezing_point_depression), [Boiling-point elevation](#boiling_point_elevation), [Molar mass from freezing-point depression](#molar_mass_from_freezing_point), [Osmotic pressure](#osmotic_pressure), [Molar mass from osmotic pressure](#molar_mass_from_osmotic_pressure) |
| Solutions / concentration | [Molarity from a dissolved mass](#molarity_from_mass), [Mass of solute for a solution](#solute_mass_for_solution), [Molality of a solution](#molality), [Mole fraction in a solution](#mole_fraction) |
| Solutions / dilution | [Dilution: final concentration](#dilution_final_concentration), [Dilution: volume of stock needed](#dilution_volume) |
| Solutions / gas solubility | [Henry's law: solubility of a gas](#henry_law_solubility) |
| Spectroscopy / absorption | [Beer-Lambert law: absorbance](#beer_lambert_absorbance), [Beer-Lambert law: concentration](#beer_lambert_concentration) |
| Spectroscopy / atomic spectra | [Hydrogen atom transition wavelength (Rydberg)](#hydrogen_transition_wavelength) |
| Spectroscopy / infrared | [Vibrational wavenumber of a diatomic molecule](#vibrational_wavenumber) |
| Spectroscopy / light | [Energy of a photon from its wavelength](#photon_energy), [Wavelength of a photon from its energy](#photon_wavelength) |
| Stoichiometry / chemical equations | [Balancing a chemical equation](#balance_equation) |
| Stoichiometry / composition | [Mass percent of an element](#mass_percent), [Mass of an element in a sample](#mass_of_element), [Empirical formula from percent composition](#empirical_formula), [Molecular formula from the empirical formula](#molecular_formula) |
| Stoichiometry / molar mass | [Molar mass of a formula](#molar_mass) |
| Stoichiometry / reaction yields | [Limiting reagent and theoretical yield](#limiting_reagent), [Theoretical yield from one reactant](#theoretical_yield), [Percent yield of a reaction](#percent_yield) |
| Stoichiometry / the mole | [Moles from a mass](#moles_from_mass), [Mass from moles](#mass_from_moles), [Number of molecules in a mass](#particles_from_mass), [Number of particles in an amount](#particles_from_moles), [Moles from a number of particles](#moles_from_particles), [Atoms of an element in a mass of compound](#atoms_of_element) |
| Thermochemistry / bond enthalpies | [Reaction enthalpy from bond enthalpies](#bond_enthalpy_reaction) |
| Thermochemistry / calorimetry | [Heat from a temperature change](#calorimetry_heat), [Reaction enthalpy from coffee-cup calorimetry](#enthalpy_from_calorimetry) |
| Thermochemistry / entropy | [Standard reaction entropy from standard molar entropies](#reaction_entropy_from_standard_entropies) |
| Thermochemistry / Hess's law | [Reaction enthalpy from enthalpies of formation (Hess's law)](#reaction_enthalpy_from_formation) |
| Thermochemistry / phase equilibria | [Clausius-Clapeyron: vapor pressure at another temperature](#clausius_clapeyron_pressure), [Clausius-Clapeyron: enthalpy of vaporization](#clausius_clapeyron_enthalpy) |
| Thermochemistry / reaction enthalpy | [Heat of reaction for a given mass](#heat_from_reaction_mass) |
| Thermochemistry / spontaneity | [Gibbs energy change (dG = dH - T dS)](#gibbs_free_energy), [Temperature at which dG = 0](#crossover_temperature), [Standard reaction Gibbs energy from Gibbs energies of formation](#reaction_gibbs_from_formation) |

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
| average_atomic_mass | Average atomic mass from isotopes | masses, abundances | A_r, M |
| isotope_abundance | Abundances of two isotopes from the average atomic mass | m1, m2, A_r | x1, x2, percent1, percent2 |
| standard_cell_potential | Standard cell potential from two half-reactions | E_cathode, E_anode | E_cell, spontaneity |
| bond_enthalpy_reaction | Reaction enthalpy from bond enthalpies | broken_n, broken_D, formed_n, formed_D | dH, dH_kJ, broken_kJ, formed_kJ |
| henry_law_solubility | Henry's law: solubility of a gas | kH, P | c, c_molL |
| reaction_quotient_direction | Reaction quotient and the direction of shift | equation, conc, K | Q, log10_Q, direction |
| precipitation_test | Precipitation test (Q versus Ksp) | Ksp, x, y, c_cation, c_anion | Q, log10_Q, result |
| reaction_entropy_from_standard_entropies | Standard reaction entropy from standard molar entropies | equation, values | dS |
| reaction_gibbs_from_formation | Standard reaction Gibbs energy from Gibbs energies of formation | equation, values | dG, dG_kJ, spontaneity |
| nuclear_binding_energy | Mass defect and nuclear binding energy | Z, A, mass | dm, E_b, E_b_MeV, E_per_nucleon_MeV |
| radioactive_decay_remaining | Amount remaining after radioactive decay (half-life) | m0, t_half, t | m, m_g, fraction, half_lives |
| radioactive_dating_age | Age from the fraction remaining (radioactive dating) | fraction, t_half | t, t_years, half_lives |
| conjugate_dissociation_constant | Conjugate acid-base constants (Ka Kb = Kw) | K, known | K_conj, pK, pK_conj, conjugate |
| specific_rotation | Specific rotation from an observed rotation | alpha, l, c | specific_rotation |
| enantiomeric_excess_from_rotation | Enantiomeric excess from optical rotation | alpha_obs, alpha_pure | ee, major_percent, minor_percent, predominant |
| enantiomeric_excess_from_composition | Enantiomeric excess from the composition of a mixture | R, S | ee, percent_R, percent_S, predominant |

<a id="molar_mass"></a>

### Molar mass of a formula: `molar_mass`

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

![The molar mass for 8 substances, each returned by the model: H₂O 18](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/molar-mass-by-substance.svg)

*The molar mass for 8 substances, each returned by the model: H₂O 18.02, NH₃ 17.03, CH₄ 16.04, CO₂ 44.01, NaCl 58.44, H₂SO₄ 98.08, CaCO₃ 100.1, C₆H₁₂O₆ 180.2. Highlighted: the formula given, H₂O. In SI units: M = 0.01801 kg/mol. Your inputs: formula = H2O.*

![Where the molar mass of H₂O comes from: each bar is one element's share, its mass percent (returned by the model) times the molar mass M = 18](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-composition)

*Where the molar mass of H₂O comes from: each bar is one element's share, its mass percent (returned by the model) times the molar mass M = 18.02 g/mol (returned by the model); the shares add up to M: H 2.016 g/mol, O 16 g/mol. In SI units: M = 0.01801 kg/mol. Your inputs: formula = H2O.*

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

### Moles from a mass: `moles_from_mass`

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

![The amount of substance for 8 substances, each returned by the model with m = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/moles-from-mass-by-substance)

*The amount of substance for 8 substances, each returned by the model with m = 0.036 kg: H₂O 1.998, NH₃ 2.114, CH₄ 2.244, CO₂ 0.818, NaCl 0.616, H₂SO₄ 0.3671, CaCO₃ 0.3597, C₆H₁₂O₆ 0.1998. Highlighted: the formula given, H₂O. Your inputs: formula = H2O, m = 0.036 kg.*

![Amount of substance as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/moles-from-mass-live-1)

*Amount of substance as m runs from 0.0216 to 0.0504 kg, every other input as you entered it (41 solutions). Marked: your m = 0.036 kg gives n = 1.998. Your inputs: formula = H2O, m = 0.036 kg.*

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

### Mass from moles: `mass_from_moles`

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

![The mass for 8 substances, each returned by the model with n = 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-from-moles-by-substance)

*The mass for 8 substances, each returned by the model with n = 2.5 mol: H₂O 45.04, NH₃ 42.58, CH₄ 40.11, CO₂ 110, NaCl 146.1, H₂SO₄ 245.2, CaCO₃ 250.2, C₆H₁₂O₆ 450.4. Highlighted: the formula given, NaCl. In SI units: m = 0.1461 kg. Your inputs: formula = NaCl, n = 2.5 mol.*

![Mass as n runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-from-moles-live-1)

*Mass as n runs from 1.5 to 3.5 mol, every other input as you entered it (41 solutions). Marked: your n = 2.5 mol gives m = 0.1461. Your inputs: formula = NaCl, n = 2.5 mol.*

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

### Number of molecules in a mass: `particles_from_mass`

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

![The number of molecules for 9 substances, each returned by the model with m = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/particles-from-mass-by-gas)

*The number of molecules for 9 substances, each returned by the model with m = 0.01 kg: H₂ 2.987e+24, He 1.504e+24, CH₄ 3.754e+23, NH₃ 3.536e+23, N₂ 2.15e+23, O₂ 1.882e+23, Ar 1.507e+23, CO₂ 1.368e+23, SO₂ 9.4e+22. Highlighted: the formula given, CO₂. Your inputs: formula = CO2, m = 0.01 kg.*

![N as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/particles-from-mass-live-1)

*N as m runs from 0.006 to 0.014 kg, every other input as you entered it (41 solutions). Marked: your m = 0.01 kg gives N = 1.368e+23. Your inputs: formula = CO2, m = 0.01 kg.*

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

### Number of particles in an amount: `particles_from_moles`

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

![Number of particles as n runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/particles-from-moles-live-1)

*Number of particles as n runs from 0.15 to 0.35 mol, every other input as you entered it (41 solutions). Marked: your n = 0.25 mol gives N = 1.506e+23. Your inputs: n = 0.25 mol.*

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

### Moles from a number of particles: `moles_from_particles`

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

![Amount of substance as N runs from 7](https://www.ai999x.com/?ax_fig=chemistry-ai999x/moles-from-particles-live-1)

*Amount of substance as N runs from 7.525e+22 to 5.268e+23, every other input as you entered it (41 solutions). Marked: your N = 3.01e+23 gives n = 0.4998. Your inputs: N = 3.01e+23.*

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

### Atoms of an element in a mass of compound: `atoms_of_element`

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

![Number of O atoms in 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/atoms-of-element-by-substance)

*Number of O atoms in 0.01 kg of each substance, each returned by the model: CO₂ 2.737e+23, H₂O 3.343e+23, H₂SO₄ 2.456e+23, CaCO₃ 1.805e+23, C₆H₁₂O₆ 2.006e+23, KMnO₄ 1.524e+23, SiO₂ 2.005e+23, FeO 8.382e+22. Highlighted: the formula given, CO₂. Your inputs: formula = CO2, element = O, m = 0.01 kg.*

![Number of atoms of the element as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/atoms-of-element-live-1)

*Number of atoms of the element as m runs from 0.006 to 0.014 kg, every other input as you entered it (41 solutions). Marked: your m = 0.01 kg gives N_atoms = 2.737e+23. Your inputs: formula = CO2, element = O, m = 0.01 kg.*

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

### Mass percent of an element: `mass_percent`

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

![Mass composition of H₂SO₄: the percent by mass of every element, returned by the model (H 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-percent-composition)

*Mass composition of H₂SO₄: the percent by mass of every element, returned by the model (H 2.055 %, S 32.7 %, O 65.25 %). Highlighted: the element asked, O. Your inputs: formula = H2SO4, element = O.*

![Percent O by mass in compounds containing it, each returned by the model: H₂SO₄ 65](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-percent-oxygen-share)

*Percent O by mass in compounds containing it, each returned by the model: H₂SO₄ 65.25, H₂O 88.81, CO₂ 72.71, CaCO₃ 47.96, C₆H₁₂O₆ 53.28, KMnO₄ 40.5, SiO₂ 53.26, FeO 22.27. Highlighted: the formula given, H₂SO₄. Your inputs: formula = H2SO4, element = O.*

![Mass percent of every element for your inputs: H 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-percent-live-1)

*Mass percent of every element for your inputs: H 2.055, S 32.7, O 65.25. Your inputs: formula = H2SO4, element = O.*

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

### Mass of an element in a sample: `mass_of_element`

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

![Mass of Fe contained in 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-of-element-by-compound)

*Mass of Fe contained in 0.05 kg of each compound, each returned by the model: Fe₂O₃ 34.97, FeO 38.87, Fe₃O₄ 36.18, FeS₂ 23.27, FeCO₃ 24.1, FeSO₄ 18.38. Highlighted: the formula given, Fe₂O₃. In SI units: m_element = 0.03497 kg. Your inputs: formula = Fe2O3, element = Fe, m = 0.05 kg.*

![Mass of the element as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mass-of-element-live-1)

*Mass of the element as m runs from 0.03 to 0.07 kg, every other input as you entered it (41 solutions). Marked: your m = 0.05 kg gives m_element = 34.97 g. Your inputs: formula = Fe2O3, element = Fe, m = 0.05 kg.*

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

### Empirical formula from percent composition, `empirical_formula`

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

*Top: the stated composition, C 40 %, H 6.71 %, O 53.3 % by mass. Bottom: the whole-number atom ratio the model returns, C 1, H 2, O 1: the empirical formula CH₂O. Your inputs: composition = C 40.0, H 6.71, O 53.3.*

![Every entry of counts as computed for your inputs (3 entries: 1, 2, 1)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/empirical-formula-live-1)

*Every entry of counts as computed for your inputs (3 entries: 1, 2, 1). Your inputs: composition = C 40.0, H 6.71, O 53.3.*

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

### Molecular formula from the empirical formula, `molecular_formula`

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

![From the empirical formula CH₂O to the molecular formula: for 7 molar masses (whole multiples of the one given) the model returns the multiple and the formula: 30](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molecular-formula-multiples)

*From the empirical formula CH₂O to the molecular formula: for 7 molar masses (whole multiples of the one given) the model returns the multiple and the formula: 30.03 g/mol -> CH₂O, 60.05 g/mol -> C₂H₄O₂, 90.08 g/mol -> C₃H₆O₃, 120.1 g/mol -> C₄H₈O₄, 150.1 g/mol -> C₅H₁₀O₅, 180.2 g/mol -> C₆H₁₂O₆, 210.2 g/mol -> C₇H₁₄O₇. The molar mass given (180.2 g/mol) gives C₆H₁₂O₆. Your inputs: empirical = CH2O, M = 0.18016 kg/mol.*

![Whole-number multiple n as M runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molecular-formula-live-1)

*Whole-number multiple n as M runs from 0.1189 to 0.2414 kg/mol, every other input as you entered it (16 solutions). Values in this range where the inputs admit no answer (43 of 59) are left out. Marked: your M = 0.1802 kg/mol gives multiple = 6. Your inputs: empirical = CH2O, M = 0.18016 kg/mol.*

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

### Balancing a chemical equation: `balance_equation`

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

![Atom balance of C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: for each element, the atoms on the reactant and on the product side, each the returned coefficient times the element's atoms in one formula unit (counted by the model) : C 3 = 3; H 8 = 8; O 10 = 10](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/balance-equation-atom-balance.svg)

*Atom balance of C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: for each element, the atoms on the reactant and on the product side, each the returned coefficient times the element's atoms in one formula unit (counted by the model) : C 3 = 3; H 8 = 8; O 10 = 10. Your inputs: equation = C3H8 + O2 -> CO2 + H2O.*

![The balanced equation C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: the coefficient of each reactant (left of the dashed line) and product (right), as returned by the model for the stated skeleton C3H8 + O2 -> CO2 + H2O](https://www.ai999x.com/?ax_fig=chemistry-ai999x/balance-equation-coefficients)

*The balanced equation C₃H₈ + 5 O₂ -> 3 CO₂ + 4 H₂O: the coefficient of each reactant (left of the dashed line) and product (right), as returned by the model for the stated skeleton C3H8 + O2 -> CO2 + H2O. Your inputs: equation = C3H8 + O2 -> CO2 + H2O.*

![Every entry of coefficients as computed for your inputs (4 entries: 1, 5, 3, 4)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/balance-equation-live-1)

*Every entry of coefficients as computed for your inputs (4 entries: 1, 5, 3, 4). Your inputs: equation = C3H8 + O2 -> CO2 + H2O.*

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

### Limiting reagent and theoretical yield: `limiting_reagent`

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

![Masses before and after the reaction H2 + O2 -> H2O with 4 g of H₂ and 16 g of O₂](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-before-after)

*Masses before and after the reaction H2 + O2 -> H2O with 4 g of H₂ and 16 g of O₂. The model returns the limiting reactant (O₂, used up), the excess left (1.984 g) and the product formed (18.02 g of H₂O). In SI units: product_mass = 0.01802 kg. Your inputs: equation = H2 + O2 -> H2O, reactant_a = H2, m_a = 0.004 kg, reactant_b = O2, m_b = 0.016 kg, product = H2O.*

![limiting (limiting reagent) over m_a from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-decision)

*limiting (limiting reagent) over m_a from 0.0004 to 0.04 and m_b from 0.0016 to 0.16: each of the 25 x 15 cells is one run of the model, coloured by the verdict it returned (O2: 279 cells, H2: 96 cells). Marked: the inputs given, m_a = 0.004, m_b = 0.016 gives O2. In SI units: product_mass = 0.01802 kg. Your inputs: equation = H2 + O2 -> H2O, reactant_a = H2, m_a = 0.004 kg, reactant_b = O2, m_b = 0.016 kg, product = H2O.*

![Mass of the excess reagent left over as m_a runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-live-1)

*Mass of the excess reagent left over as m_a runs from 0.0024 to 0.0056 kg, every other input as you entered it (41 solutions). Marked: your m_a = 0.004 kg gives excess_left = 1.984 g. Your inputs: equation = H2 + O2 -> H2O, reactant_a = H2, m_a = 0.004 kg, reactant_b = O2, m_b = 0.016 kg, product = H2O.*

![Limiting reagent for 25 5 x 5 combinations of m_a and m_b around your inputs (the other inputs as entered); the ring marks your inputs, which give O2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-live-2)

*Limiting reagent for 25 5 x 5 combinations of m_a and m_b around your inputs (the other inputs as entered); the ring marks your inputs, which give O2. Blank cells admit no answer. Your inputs: equation = H2 + O2 -> H2O, reactant_a = H2, m_a = 0.004 kg, reactant_b = O2, m_b = 0.016 kg, product = H2O.*

![Mass of product formed as m_b runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/limiting-reagent-live-3)

*Mass of product formed as m_b runs from 0.0096 to 0.0224 kg, every other input as you entered it (29 solutions). Marked: your m_b = 0.016 kg gives product_mass = 18.02 g. Your inputs: equation = H2 + O2 -> H2O, reactant_a = H2, m_a = 0.004 kg, reactant_b = O2, m_b = 0.016 kg, product = H2O.*

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

### Theoretical yield from one reactant: `theoretical_yield`

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

![Theoretical yield of every product of CH4 + O2 -> CO2 + H2O from 16 g of CH₄ (the other reactants in excess), each returned by the model: CO₂ 43](https://www.ai999x.com/?ax_fig=chemistry-ai999x/theoretical-yield-products)

*Theoretical yield of every product of CH4 + O2 -> CO2 + H2O from 16 g of CH₄ (the other reactants in excess), each returned by the model: CO₂ 43.89 g (0.9973 mol), H₂O 35.93 g (1.995 mol). Highlighted: the product asked, CO₂. In SI units: product_mass = 0.04389 kg. Your inputs: equation = CH4 + O2 -> CO2 + H2O, reactant = CH4, m = 0.016 kg, product = CO2.*

![Theoretical mass of product as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/theoretical-yield-live-1)

*Theoretical mass of product as m runs from 0.0096 to 0.0224 kg, every other input as you entered it (41 solutions). Marked: your m = 0.016 kg gives product_mass = 43.89 g. Your inputs: equation = CH4 + O2 -> CO2 + H2O, reactant = CH4, m = 0.016 kg, product = CO2.*

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

### Percent yield of a reaction: `percent_yield`

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

![Percent yield of CaO from 100 g of CaCO₃ by CaCO3 -> CaO + CO2: the model's theoretical mass (56](https://www.ai999x.com/?ax_fig=chemistry-ai999x/percent-yield-yield-bars)

*Percent yield of CaO from 100 g of CaCO₃ by CaCO3 -> CaO + CO2: the model's theoretical mass (56.03 g) against the stated actual mass (50 g); right: the difference not obtained (6.029 g, derived). Marked: percent yield 89.24 %. Your inputs: equation = CaCO3 -> CaO + CO2, reactant = CaCO3, m = 0.1 kg, product = CaO, actual = 0.05 kg.*

![Percent yield as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/percent-yield-live-1)

*Percent yield as m runs from 0.06 to 0.14 kg, every other input as you entered it (41 solutions). Marked: your m = 0.1 kg gives percent_yield = 89.24. Your inputs: equation = CaCO3 -> CaO + CO2, reactant = CaCO3, m = 0.1 kg, product = CaO, actual = 0.05 kg.*

![Percent yield as actual runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/percent-yield-live-2)

*Percent yield as actual runs from 0.03 to 0.07 kg, every other input as you entered it (29 solutions). Marked: your actual = 0.05 kg gives percent_yield = 89.24. Your inputs: equation = CaCO3 -> CaO + CO2, reactant = CaCO3, m = 0.1 kg, product = CaO, actual = 0.05 kg.*

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

### Ideal gas law: pressure, `ideal_gas_pressure`

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

![Isotherms of 2 mol of ideal gas: pressure against volume at T = 200, 300, 400 K, each computed by the model at 61 volumes from 4 L to 40 L](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/ideal-gas-pressure-isotherms.svg)

*Isotherms of 2 mol of ideal gas: pressure against volume at T = 200, 300, 400 K, each computed by the model at 61 volumes from 4 L to 40 L. Marked: V = 10 L at 300 K, P = 498.9 kPa. In SI units: P = 4.989e+05 Pa. Your inputs: n = 2 mol, V = 0.01 m^3, T = 300 K.*

![Pressure as n runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-pressure-live-1)

*Pressure as n runs from 1.2 to 2.8 mol, every other input as you entered it (41 solutions). Marked: your n = 2 mol gives P = 498.9 kPa. Your inputs: n = 2 mol, V = 0.01 m^3, T = 300 K.*

![Pressure as T runs from 260 to 340 K, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-pressure-live-2)

*Pressure as T runs from 260 to 340 K, every other input as you entered it (29 solutions). Marked: your T = 300 K gives P = 498.9 kPa. Your inputs: n = 2 mol, V = 0.01 m^3, T = 300 K.*

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

### Ideal gas law: volume, `ideal_gas_volume`

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

![Volume of 1 mol of ideal gas against temperature (isobars) at half, once and twice P = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-volume-isobars)

*Volume of 1 mol of ideal gas against temperature (isobars) at half, once and twice P = 101.3 kPa, computed by the model at 51 temperatures up to 500 K: straight lines through 0 K. Marked: T = 273.1 K, P = 101.3 kPa, V = 22.41 L. In SI units: V = 0.02241 m^3. Your inputs: n = 1 mol, P = 101325 Pa, T = 273.15 K.*

![Volume as n runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-volume-live-1)

*Volume as n runs from 0.6 to 1.4 mol, every other input as you entered it (41 solutions). Marked: your n = 1 mol gives V = 22.41 L. Your inputs: n = 1 mol, P = 101325 Pa, T = 273.15 K.*

![Volume as P runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-volume-live-2)

*Volume as P runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (29 solutions). Marked: your P = 1.013e+05 Pa gives V = 22.41 L. Your inputs: n = 1 mol, P = 101325 Pa, T = 273.15 K.*

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

### Ideal gas law: amount, `ideal_gas_moles`

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

![Amount of gas as T runs from 260 to 340 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-moles-live-1)

*Amount of gas as T runs from 260 to 340 K, every other input as you entered it (41 solutions). Marked: your T = 300 K gives n = 0.4062. Your inputs: P = 202650 Pa, V = 0.005 m^3, T = 300 K.*

![Amount of gas as P runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-moles-live-2)

*Amount of gas as P runs from 1.216e+05 to 2.837e+05 Pa, every other input as you entered it (29 solutions). Marked: your P = 2.026e+05 Pa gives n = 0.4062. Your inputs: P = 202650 Pa, V = 0.005 m^3, T = 300 K.*

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

### Ideal gas law: temperature, `ideal_gas_temperature`

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

*Which isotherm passes through the state P = 101.3 kPa, V = 50 L of 2 mol? The model returns T = 304.7 K; drawn (logarithmic pressure axis): that isotherm and those at half and twice the temperature, each from the model's ideal-gas pressure at 61 volumes from 10 L to 200 L. Marked: the state, T = 304.7 K. Your inputs: P = 101325 Pa, V = 0.05 m^3, n = 2 mol.*

![Temperature as P runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-temperature-live-1)

*Temperature as P runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (41 solutions). Marked: your P = 1.013e+05 Pa gives T = 304.7. Your inputs: P = 101325 Pa, V = 0.05 m^3, n = 2 mol.*

![Temperature as V runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-temperature-live-2)

*Temperature as V runs from 0.03 to 0.07 m^3, every other input as you entered it (29 solutions). Marked: your V = 0.05 m^3 gives T = 304.7. Your inputs: P = 101325 Pa, V = 0.05 m^3, n = 2 mol.*

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

### Mass of a gas sample (ideal gas): `ideal_gas_mass`

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

![The mass for 9 substances, each returned by the model with P = 101325 Pa, V = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-mass-by-gas)

*The mass for 9 substances, each returned by the model with P = 101325 Pa, V = 0.005 m^3, T = 300 K: H₂ 0.4095, He 0.813, CH₄ 3.258, NH₃ 3.459, N₂ 5.69, O₂ 6.499, Ar 8.114, CO₂ 8.939, SO₂ 13.01. Highlighted: the formula given, CO₂. In SI units: m = 0.008939 kg. Your inputs: formula = CO2, P = 101325 Pa, V = 0.005 m^3, T = 300 K.*

![Mass of gas as T runs from 260 to 340 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-mass-live-1)

*Mass of gas as T runs from 260 to 340 K, every other input as you entered it (41 solutions). Marked: your T = 300 K gives m = 8.939 g. Your inputs: formula = CO2, P = 101325 Pa, V = 0.005 m^3, T = 300 K.*

![Mass of gas as P runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ideal-gas-mass-live-2)

*Mass of gas as P runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (29 solutions). Marked: your P = 1.013e+05 Pa gives m = 8.939 g. Your inputs: formula = CO2, P = 101325 Pa, V = 0.005 m^3, T = 300 K.*

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

### Density of an ideal gas: `gas_density`

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

![The density for 9 substances, each returned by the model with P = 101325 Pa, T = 298 K: H₂ 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gas-density-by-gas)

*The density for 9 substances, each returned by the model with P = 101325 Pa, T = 298 K: H₂ 0.08244, He 0.1637, CH₄ 0.6561, NH₃ 0.6965, N₂ 1.146, O₂ 1.309, Ar 1.634, CO₂ 1.8, SO₂ 2.62. Highlighted: the formula given, CO₂. Your inputs: formula = CO2, P = 101325 Pa, T = 298 K.*

![Density as P runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gas-density-live-1)

*Density as P runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (41 solutions). Marked: your P = 1.013e+05 Pa gives ρ = 1.8. Your inputs: formula = CO2, P = 101325 Pa, T = 298 K.*

![Density as T runs from 258 to 338 K, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gas-density-live-2)

*Density as T runs from 258 to 338 K, every other input as you entered it (29 solutions). Marked: your T = 298 K gives ρ = 1.8. Your inputs: formula = CO2, P = 101325 Pa, T = 298 K.*

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

### Molar mass from gas density: `molar_mass_from_gas_density`

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

![Molar mass as ρ runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-gas-density-live-1)

*Molar mass as ρ runs from 0.75 to 1.75 kg/m^3, every other input as you entered it (41 solutions). Marked: your ρ = 1.25 kg/m^3 gives M = 28 g/mol. Your inputs: rho = 1.25 kg/m^3, P = 101325 Pa, T = 273 K.*

![Molar mass as P runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-gas-density-live-2)

*Molar mass as P runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (29 solutions). Marked: your P = 1.013e+05 Pa gives M = 28 g/mol. Your inputs: rho = 1.25 kg/m^3, P = 101325 Pa, T = 273 K.*

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

### Molar mass from a gas sample: `molar_mass_from_gas_sample`

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

![Molar mass as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-gas-sample-live-1)

*Molar mass as m runs from 0.0006 to 0.0014 kg, every other input as you entered it (41 solutions). Marked: your m = 0.001 kg gives M = 49.23 g/mol. Your inputs: m = 0.001 kg, V = 0.0005 m^3, P = 101325 Pa, T = 300 K.*

![Molar mass as V runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-gas-sample-live-2)

*Molar mass as V runs from 0.0003 to 0.0007 m^3, every other input as you entered it (29 solutions). Marked: your V = 0.0005 m^3 gives M = 49.23 g/mol. Your inputs: m = 0.001 kg, V = 0.0005 m^3, P = 101325 Pa, T = 300 K.*

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

### Boyle's law: final volume, `boyle_final_volume`

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

![Boyle's law on a P-V diagram: the isotherm through state 1 (P1 = 100 kPa, V1 = 5 L), each point the model's the final volume for 61 final pressures; arrow: the change to P2 = 250 kPa](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-volume-isotherm)

*Boyle's law on a P-V diagram: the isotherm through state 1 (P1 = 100 kPa, V1 = 5 L), each point the model's the final volume for 61 final pressures; arrow: the change to P2 = 250 kPa. Marked: V2 = 2 L. In SI units: V2 = 0.002 m^3. Your inputs: P1 = 100 kPa, V1 = 5.0 L, P2 = 250 kPa.*

![Final volume as P1 runs from 60 to 140 kPa, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-volume-live-1)

*Final volume as P1 runs from 60 to 140 kPa, every other input as you entered it (41 solutions). Marked: your P1 = 100 kPa gives V2 = 2 L. Your inputs: P1 = 100 kPa, V1 = 5.0 L, P2 = 250 kPa.*

![Final volume as V1 runs from 3 to 7 L, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-volume-live-2)

*Final volume as V1 runs from 3 to 7 L, every other input as you entered it (29 solutions). Marked: your V1 = 5 L gives V2 = 2 L. Your inputs: P1 = 100 kPa, V1 = 5.0 L, P2 = 250 kPa.*

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

### Boyle's law: final pressure, `boyle_final_pressure`

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

![Boyle's law on a P-V diagram: the isotherm through state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-pressure-isotherm)

*Boyle's law on a P-V diagram: the isotherm through state 1 (P1 = 101.3 kPa, V1 = 4 L), each point the model's the final pressure for 61 final volumes; arrow: the change to V2 = 1 L. Marked: P2 = 405.3 kPa. In SI units: P2 = 4.053e+05 Pa. Your inputs: P1 = 101325 Pa, V1 = 0.004 m^3, V2 = 0.001 m^3.*

![Final pressure as V2 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-pressure-live-1)

*Final pressure as V2 runs from 0.0006 to 0.0014 m^3, every other input as you entered it (41 solutions). Marked: your V2 = 0.001 m^3 gives P2 = 405.3 kPa. Your inputs: P1 = 101325 Pa, V1 = 0.004 m^3, V2 = 0.001 m^3.*

![Final pressure as V1 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boyle-final-pressure-live-2)

*Final pressure as V1 runs from 0.0024 to 0.0056 m^3, every other input as you entered it (29 solutions). Marked: your V1 = 0.004 m^3 gives P2 = 405.3 kPa. Your inputs: P1 = 101325 Pa, V1 = 0.004 m^3, V2 = 0.001 m^3.*

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

### Charles's law: final volume, `charles_final_volume`

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

*Charles's law: volume against absolute temperature at constant pressure through state 1 (V1 = 2.5 L, T1 = 298.1 K), the model's final volume at 41 temperatures from 193.8 K to 435.2 K; dashed: the same law toward 0 K. Arrow: the change to T2 = 348.1 K. Marked: V2 = 2.919 L. In SI units: V2 = 0.002919 m^3. Your inputs: V1 = 0.0025 m^3, T1 = 298.15 K, T2 = 348.15 K.*

![Final volume as V1 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/charles-final-volume-live-1)

*Final volume as V1 runs from 0.0015 to 0.0035 m^3, every other input as you entered it (41 solutions). Marked: your V1 = 0.0025 m^3 gives V2 = 2.919 L. Your inputs: V1 = 0.0025 m^3, T1 = 298.15 K, T2 = 348.15 K.*

![Final volume as T2 runs from 308](https://www.ai999x.com/?ax_fig=chemistry-ai999x/charles-final-volume-live-2)

*Final volume as T2 runs from 308.1 to 388.1 K, every other input as you entered it (29 solutions). Marked: your T2 = 348.1 K gives V2 = 2.919 L. Your inputs: V1 = 0.0025 m^3, T1 = 298.15 K, T2 = 348.15 K.*

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

### Gay-Lussac's law: final pressure, `gay_lussac_final_pressure`

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

*Gay-Lussac's law: pressure against absolute temperature at constant volume through state 1 (P1 = 222.9 kPa, T1 = 293.1 K), the model's final pressure at 41 temperatures from 190.5 K to 397.7 K; dashed: the same law toward 0 K. Arrow: the change to T2 = 318.1 K. Marked: P2 = 241.9 kPa. In SI units: P2 = 2.419e+05 Pa. Your inputs: P1 = 222915.00000000003 Pa, T1 = 293.15 K, T2 = 318.15 K.*

![Final pressure as T1 runs from 253](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gay-lussac-final-pressure-live-1)

*Final pressure as T1 runs from 253.1 to 333.1 K, every other input as you entered it (41 solutions). Marked: your T1 = 293.1 K gives P2 = 241.9 kPa. Your inputs: P1 = 222915.00000000003 Pa, T1 = 293.15 K, T2 = 318.15 K.*

![Final pressure as P1 runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gay-lussac-final-pressure-live-2)

*Final pressure as P1 runs from 1.337e+05 to 3.121e+05 Pa, every other input as you entered it (29 solutions). Marked: your P1 = 2.229e+05 Pa gives P2 = 241.9 kPa. Your inputs: P1 = 222915.00000000003 Pa, T1 = 293.15 K, T2 = 318.15 K.*

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

### Combined gas law: final volume, `combined_gas_final_volume`

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

![The combined gas law on a P-V diagram: state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-volume-pv-states)

*The combined gas law on a P-V diagram: state 1 (P1 = 101.3 kPa, V1 = 2 L, T1 = 273 K) lies on the isotherm T1 and state 2 (P2 = 50.66 kPa, V2 = 8 L) on the isotherm T2 = 546 K; each isotherm is P V constant through its state, drawn with the model's Boyle relation at 61 volumes. Marked: the returned V2 = 8 L. In SI units: V2 = 0.008 m^3. Your inputs: P1 = 1.0 atm, V1 = 2.0 L, T1 = 273 K, P2 = 0.5 atm, T2 = 546 K.*

![Final volume as P1 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-volume-live-1)

*Final volume as P1 runs from 0.6 to 1.4 atm, every other input as you entered it (41 solutions). Marked: your P1 = 1 atm gives V2 = 8 L. Your inputs: P1 = 1.0 atm, V1 = 2.0 L, T1 = 273 K, P2 = 0.5 atm, T2 = 546 K.*

![Final volume as V1 runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-volume-live-2)

*Final volume as V1 runs from 1.2 to 2.8 L, every other input as you entered it (29 solutions). Marked: your V1 = 2 L gives V2 = 8 L. Your inputs: P1 = 1.0 atm, V1 = 2.0 L, T1 = 273 K, P2 = 0.5 atm, T2 = 546 K.*

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

### Combined gas law: final pressure, `combined_gas_final_pressure`

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

![The combined gas law on a P-V diagram: state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-pressure-pv-states)

*The combined gas law on a P-V diagram: state 1 (P1 = 101.3 kPa, V1 = 3 L, T1 = 300 K) lies on the isotherm T1 and state 2 (P2 = 304 kPa, V2 = 1.5 L) on the isotherm T2 = 450 K; each isotherm is P V constant through its state, drawn with the model's Boyle relation at 61 volumes. Marked: the returned P2 = 304 kPa. In SI units: P2 = 3.04e+05 Pa. Your inputs: P1 = 101325 Pa, V1 = 0.003 m^3, T1 = 300 K, V2 = 0.0015 m^3, T2 = 450 K.*

![Final pressure as P1 runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-pressure-live-1)

*Final pressure as P1 runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (41 solutions). Marked: your P1 = 1.013e+05 Pa gives P2 = 304 kPa. Your inputs: P1 = 101325 Pa, V1 = 0.003 m^3, T1 = 300 K, V2 = 0.0015 m^3, T2 = 450 K.*

![Final pressure as V1 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-pressure-live-2)

*Final pressure as V1 runs from 0.0018 to 0.0042 m^3, every other input as you entered it (29 solutions). Marked: your V1 = 0.003 m^3 gives P2 = 304 kPa. Your inputs: P1 = 101325 Pa, V1 = 0.003 m^3, T1 = 300 K, V2 = 0.0015 m^3, T2 = 450 K.*

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

### Combined gas law: final temperature, `combined_gas_final_temperature`

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

![The combined gas law on a P-V diagram: state 1 (P1 = 101](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-temperature-pv-states)

*The combined gas law on a P-V diagram: state 1 (P1 = 101.3 kPa, V1 = 2 L, T1 = 300 K) lies on the isotherm T1 and state 2 (P2 = 202.7 kPa, V2 = 1.5 L) on the isotherm T2 = 450 K; each isotherm is P V constant through its state, drawn with the model's Boyle relation at 61 volumes. Marked: the returned T2 = 450 K. Your inputs: P1 = 1.00 atm, V1 = 2.00 L, T1 = 300 K, P2 = 2.00 atm, V2 = 1.50 L.*

![Final temperature as V1 runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-temperature-live-1)

*Final temperature as V1 runs from 1.2 to 2.8 L, every other input as you entered it (41 solutions). Marked: your V1 = 2 L gives T2 = 450. Your inputs: P1 = 1.00 atm, V1 = 2.00 L, T1 = 300 K, P2 = 2.00 atm, V2 = 1.50 L.*

![Final temperature as P1 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/combined-gas-final-temperature-live-2)

*Final temperature as P1 runs from 0.6 to 1.4 atm, every other input as you entered it (29 solutions). Marked: your P1 = 1 atm gives T2 = 450. Your inputs: P1 = 1.00 atm, V1 = 2.00 L, T1 = 300 K, P2 = 2.00 atm, V2 = 1.50 L.*

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

### Partial pressure (Dalton's law): `partial_pressure`

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

*Partial pressures in dry air at a total of 101.3 kPa (logarithmic axis), each from the model with a stated mole fraction: N2 0.7808, O2 0.2095, Ar 0.00934, CO2 0.00042; last bar: the mole fraction given, x = 0.21, p = 2.128e+04 Pa. Your inputs: x = 0.21, P_total = 101325 Pa.*

![Partial pressure as P_total runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/partial-pressure-live-1)

*Partial pressure as P_total runs from 6.08e+04 to 1.419e+05 Pa, every other input as you entered it (41 solutions). Marked: your P_total = 1.013e+05 Pa gives p = 21.28 kPa. Your inputs: x = 0.21, P_total = 101325 Pa.*

![Partial pressure as x runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/partial-pressure-live-2)

*Partial pressure as x runs from 0.0525 to 0.3675, every other input as you entered it (29 solutions). Marked: your x = 0.21 gives p = 21.28 kPa. Your inputs: x = 0.21, P_total = 101325 Pa.*

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

### Partial pressures in a gas mixture: `dalton_mixture`

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

*Dalton's law: a mixture of 2, 1, 0.5 mol of 3 gases at a total pressure of 354.6 kPa; each bar is a partial pressure returned by the model with its mole fraction (0.5714, 0.2857, 0.1429), and the last bar adds them back up to 354.6 kPa. Your inputs: moles = 2.0, 1.0, 0.5, P_total = 354637.5 Pa.*

![Partial pressure of each gas, in the order given, entry 1 as P_total runs from 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dalton-mixture-live-1)

*Partial pressure of each gas, in the order given, entry 1 as P_total runs from 2.128e+05 to 4.965e+05 Pa, every other input as you entered it (41 solutions). Marked: your P_total = 3.546e+05 Pa gives partial_pressures[1] = 202.7 kPa. Your inputs: moles = 2.0, 1.0, 0.5, P_total = 354637.5 Pa.*

![Every entry of partial_pressures as computed for your inputs (3 entries: 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dalton-mixture-live-2)

*Every entry of partial_pressures as computed for your inputs (3 entries: 2.026e+05, 1.013e+05, 5.066e+04). Your inputs: moles = 2.0, 1.0, 0.5, P_total = 354637.5 Pa.*

![Partial pressure of each gas, in the order given, entry 3 as moles[3] runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dalton-mixture-live-3)

*Partial pressure of each gas, in the order given, entry 3 as moles[3] runs from 0.125 to 0.875, every other input as you entered it (29 solutions). Marked: your moles[3] = 0.5 gives partial_pressures[3] = 50.66 kPa. Your inputs: moles = 2.0, 1.0, 0.5, P_total = 354637.5 Pa.*

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

### Graham's law: ratio of effusion rates, `graham_rate_ratio`

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

![Graham's law: how many times faster each gas effuses than O₂ (dashed: as fast), each ratio returned by the model: H₂ 3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/graham-rate-ratio-by-gas)

*Graham's law: how many times faster each gas effuses than O₂ (dashed: as fast), each ratio returned by the model: H₂ 3.984, He 2.827, CH₄ 1.412, NH₃ 1.371, N₂ 1.069, O₂ 1, Ar 0.895, CO₂ 0.8527, SO₂ 0.7067, Cl₂ 0.6718. Highlighted: the gas given, H₂ against O₂, ratio 3.984. Your inputs: formula_1 = H2, formula_2 = O2.*

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

### Graham's law: molar mass of an unknown gas, `graham_molar_mass`

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

![Molar mass of the unknown gas as ratio runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/graham-molar-mass-live-1)

*Molar mass of the unknown gas as ratio runs from 0.375 to 2.625, every other input as you entered it (41 solutions). Marked: your ratio = 1.5 gives M = 14.22 g/mol. Your inputs: ratio = 1.5, formula_known = O2.*

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

### Van der Waals equation: pressure, `van_der_waals_pressure`

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

![Pressure of 2 mol of gas at 300 K against volume, van der Waals and ideal, both returned by the model at 71 volumes from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/van-der-waals-pressure-real-vs-ideal)

*Pressure of 2 mol of gas at 300 K against volume, van der Waals and ideal, both returned by the model at 71 volumes from 0.125 L to 20 L (logarithmic axes; a non-positive pressure is left out). At large volume the two agree. Marked: V = 5 L, P = 9.56 bar (ideal 9.977 bar). In SI units: P = 9.56e+05 Pa. Your inputs: n = 2 mol, V = 0.005 m^3, T = 300 K, a = 0.368823 Pa*m^6/mol^2, b = 4.267e-05 m^3/mol.*

![Van der Waals pressure and ideal-gas pressure as T runs from 260 to 340 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/van-der-waals-pressure-live-1)

*Van der Waals pressure and ideal-gas pressure as T runs from 260 to 340 K, every other input as you entered it (41 solutions). Marked: your T = 300 K gives P = 956 kPa. Your inputs: n = 2 mol, V = 0.005 m^3, T = 300 K, a = 0.368823 Pa*m^6/mol^2, b = 4.267e-05 m^3/mol.*

![Van der Waals pressure and ideal-gas pressure as n runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/van-der-waals-pressure-live-2)

*Van der Waals pressure and ideal-gas pressure as n runs from 1.2 to 2.8 mol, every other input as you entered it (29 solutions). Marked: your n = 2 mol gives P = 956 kPa. Your inputs: n = 2 mol, V = 0.005 m^3, T = 300 K, a = 0.368823 Pa*m^6/mol^2, b = 4.267e-05 m^3/mol.*

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

### Root-mean-square speed of gas molecules: `rms_speed`

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

![The rms speed of gas molecules against temperature for 4 gases, each computed by the model at 46 temperatures from 100 K to 1000 K (it grows as the square root of T/M)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/rms-speed-by-gas)

*The rms speed of gas molecules against temperature for 4 gases, each computed by the model at 46 temperatures from 100 K to 1000 K (it grows as the square root of T/M). At 300 K: H₂ 1927, He 1367, N₂ 516.8, CO₂ 412.4 m/s. Marked: N₂ at 300 K, 516.8 m/s. Your inputs: formula = N2, T = 300 K.*

![Root-mean-square speed as T runs from 260 to 340 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/rms-speed-live-1)

*Root-mean-square speed as T runs from 260 to 340 K, every other input as you entered it (41 solutions). Marked: your T = 300 K gives u_rms = 516.8. Your inputs: formula = N2, T = 300 K.*

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

### Mean speed of gas molecules: `mean_speed`

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

![The mean speed of gas molecules against temperature for 4 gases, each computed by the model at 46 temperatures from 100 K to 1000 K (it grows as the square root of T/M)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mean-speed-by-gas)

*The mean speed of gas molecules against temperature for 4 gases, each computed by the model at 46 temperatures from 100 K to 1000 K (it grows as the square root of T/M). At 300 K: H₂ 1775, He 1260, N₂ 476.2, CO₂ 379.9 m/s. Marked: CO₂ at 300 K, 379.9 m/s. Your inputs: formula = CO2, T = 300 K.*

![Mean speed as T runs from 260 to 340 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mean-speed-live-1)

*Mean speed as T runs from 260 to 340 K, every other input as you entered it (41 solutions). Marked: your T = 300 K gives u_mean = 379.9. Your inputs: formula = CO2, T = 300 K.*

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

### Molarity from a dissolved mass: `molarity_from_mass`

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

![c (molar concentration) over m from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molarity-from-mass-field)

*c (molar concentration) over m from 0.002925 to 0.008775 and V from 0.000125 to 0.000375: each of the 21 x 15 cells is one run of the model, shaded by its value (light 133.5 to dark 1201 mol/m^3); iso-lines at c = 200, 400, 600, 800, 1000, 1200. Marked: the inputs given, m = 0.00585, V = 0.00025, c = 400.4 mol/m^3. Your inputs: formula = NaCl, m = 0.00585 kg, V = 0.00025 m^3.*

![Molar concentration as V runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molarity-from-mass-live-1)

*Molar concentration as V runs from 0.00015 to 0.00035 m^3, every other input as you entered it (41 solutions). Marked: your V = 0.00025 m^3 gives c = 0.4004 mol/L. Your inputs: formula = NaCl, m = 0.00585 kg, V = 0.00025 m^3.*

![Molar concentration as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molarity-from-mass-live-2)

*Molar concentration as m runs from 0.00351 to 0.00819 kg, every other input as you entered it (29 solutions). Marked: your m = 0.00585 kg gives c = 0.4004 mol/L. Your inputs: formula = NaCl, m = 0.00585 kg, V = 0.00025 m^3.*

```python
chemistry_ai999x.solve('molarity_from_mass', formula='NaCl', m=0.00585, V=0.00025)
# -> c = 400.39 mol/m^3, c_molL = 0.40039   (SOLVED)
```

```python
chemistry_ai999x.solve('molarity_from_mass', formula='C6H12O6', m=0.09, V=0.0005)
# -> c = 999.134 mol/m^3, c_molL = 0.999134   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the molarity of a solution made by dissolving 5.85 g of NaCl in 250 mL of solution? | SOLVED | c = 400.39 mol/m^3, c_molL = 0.40039 |
| Find the concentration of a solution of 90.0 g of C6H12O6 in 0.500 L of solution. | SOLVED | c = 999.134 mol/m^3, c_molL = 0.999134 |

<a id="solute_mass_for_solution"></a>

### Mass of solute for a solution: `solute_mass_for_solution`

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

![m (mass of solute) over c from 50 to 150 and V from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/solute-mass-for-solution-field)

*m (mass of solute) over c from 50 to 150 and V from 0.000125 to 0.000375: each of the 21 x 15 cells is one run of the model, shaded by its value (light 0.0003653 to dark 0.003287 kg); iso-lines at m = 0.0005, 0.001, 0.0015, 0.002, 0.0025, 0.003. Marked: the inputs given, c = 100, V = 0.00025, m = 0.001461 kg. Your inputs: formula = NaCl, c = 100 mol/m^3, V = 0.00025 m^3.*

![Mass of solute as c runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/solute-mass-for-solution-live-1)

*Mass of solute as c runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c = 100 mol/m^3 gives m = 1.461 g. Your inputs: formula = NaCl, c = 100 mol/m^3, V = 0.00025 m^3.*

![Mass of solute as V runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/solute-mass-for-solution-live-2)

*Mass of solute as V runs from 0.00015 to 0.00035 m^3, every other input as you entered it (29 solutions). Marked: your V = 0.00025 m^3 gives m = 1.461 g. Your inputs: formula = NaCl, c = 100 mol/m^3, V = 0.00025 m^3.*

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

### Dilution: final concentration, `dilution_final_concentration`

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

![Final concentration as V1 runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dilution-final-concentration-live-1)

*Final concentration as V1 runs from 1.5e-05 to 3.5e-05 m^3, every other input as you entered it (41 solutions). Marked: your V1 = 2.5e-05 m^3 gives c2 = 0.3 mol/L. Your inputs: c1 = 6000 mol/m^3, V1 = 2.5e-05 m^3, V2 = 0.0005 m^3.*

![Final concentration as V2 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dilution-final-concentration-live-2)

*Final concentration as V2 runs from 0.0003 to 0.0007 m^3, every other input as you entered it (29 solutions). Marked: your V2 = 0.0005 m^3 gives c2 = 0.3 mol/L. Your inputs: c1 = 6000 mol/m^3, V1 = 2.5e-05 m^3, V2 = 0.0005 m^3.*

```python
chemistry_ai999x.solve('dilution_final_concentration', c1=6000.0, V1=2.5e-05, V2=0.0005)
# -> c2 = 300 mol/m^3, c2_molL = 0.3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| 25.0 mL of 6.0 M HCl is diluted to 500 mL. What is the new concentration? | SOLVED | c2 = 300 mol/m^3, c2_molL = 0.3 |
| A 10.0 mL sample of 2.00 M NaOH is diluted to a total volume of 250 mL. Find the final molarity. | SOLVED | c2 = 80 mol/m^3, c2_molL = 0.08 |

<a id="dilution_volume"></a>

### Dilution: volume of stock needed, `dilution_volume`

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

![Volume of stock solution as c2 runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dilution-volume-live-1)

*Volume of stock solution as c2 runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c2 = 100 mol/m^3 gives V1 = 4.167 mL. Your inputs: c1 = 12000 mol/m^3, c2 = 100 mol/m^3, V2 = 0.0005 m^3.*

![Volume of stock solution as V2 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/dilution-volume-live-2)

*Volume of stock solution as V2 runs from 0.0003 to 0.0007 m^3, every other input as you entered it (29 solutions). Marked: your V2 = 0.0005 m^3 gives V1 = 4.167 mL. Your inputs: c1 = 12000 mol/m^3, c2 = 100 mol/m^3, V2 = 0.0005 m^3.*

```python
chemistry_ai999x.solve('dilution_volume', c1=12000.0, c2=100.0, V2=0.0005)
# -> V1 = 4.16667e-06 m^3, V1_mL = 4.16667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What volume of 12.0 M HCl is needed to prepare 500 mL of 0.100 M HCl? | SOLVED | V1 = 4.16667e-06 m^3, V1_mL = 4.16667 |
| C1 = 2.0 M, C2 = 0.50 M, V2 = 100 mL. What volume of stock solution is needed? | SOLVED | V1 = 2.5e-05 m^3, V1_mL = 25 |

<a id="molality"></a>

### Molality of a solution: `molality`

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

![b (molality) over m_solute from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molality-field)

*b (molality) over m_solute from 0.005 to 0.015 and m_solvent from 0.25 to 0.75: each of the 21 x 15 cells is one run of the model, shaded by its value (light 0.1141 to dark 1.027 mol/kg); iso-lines at b = 0.2, 0.4, 0.6, 0.8, 1. Marked: the inputs given, m_solute = 0.01, m_solvent = 0.5, b = 0.3422 mol/kg. Your inputs: formula = NaCl, m_solute = 0.01 kg, m_solvent = 0.5 kg.*

![Molality as m_solute runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molality-live-1)

*Molality as m_solute runs from 0.006 to 0.014 kg, every other input as you entered it (41 solutions). Marked: your m_solute = 0.01 kg gives b = 0.3422. Your inputs: formula = NaCl, m_solute = 0.01 kg, m_solvent = 0.5 kg.*

![Molality as m_solvent runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molality-live-2)

*Molality as m_solvent runs from 0.3 to 0.7 kg, every other input as you entered it (29 solutions). Marked: your m_solvent = 0.5 kg gives b = 0.3422. Your inputs: formula = NaCl, m_solute = 0.01 kg, m_solvent = 0.5 kg.*

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

### Mole fraction in a solution: `mole_fraction`

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

![x_solute (mole fraction of solute) over m_solute from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mole-fraction-field)

*x_solute (mole fraction of solute) over m_solute from 0.01 to 0.03 and m_solvent from 0.05 to 0.15: each of the 21 x 15 cells is one run of the model, shaded by its value (light 0.006622 to dark 0.0566); iso-lines at x_solute = 0.01, 0.02, 0.03, 0.04, 0.05. Marked: the inputs given, m_solute = 0.02, m_solvent = 0.1, x_solute = 0.01961. Your inputs: formula_solute = C6H12O6, m_solute = 0.02 kg, formula_solvent = H2O, m_solvent = 0.1 kg.*

![Mole fraction of solute as m_solute runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mole-fraction-live-1)

*Mole fraction of solute as m_solute runs from 0.012 to 0.028 kg, every other input as you entered it (41 solutions). Marked: your m_solute = 0.02 kg gives x_solute = 0.01961. Your inputs: formula_solute = C6H12O6, m_solute = 0.02 kg, formula_solvent = H2O, m_solvent = 0.1 kg.*

![Mole fraction of solute as m_solvent runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/mole-fraction-live-2)

*Mole fraction of solute as m_solvent runs from 0.06 to 0.14 kg, every other input as you entered it (29 solutions). Marked: your m_solvent = 0.1 kg gives x_solute = 0.01961. Your inputs: formula_solute = C6H12O6, m_solute = 0.02 kg, formula_solvent = H2O, m_solvent = 0.1 kg.*

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

### Vapor pressure of a solution (Raoult's law), `raoult_vapor_pressure`

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

![Vapor pressure of the solution as P0 runs from 1904 to 4442 Pa, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/raoult-vapor-pressure-live-1)

*Vapor pressure of the solution as P0 runs from 1904 to 4442 Pa, every other input as you entered it (41 solutions). Marked: your P0 = 3173 Pa gives P = 3156. Your inputs: formula_solute = C12H22O11, m_solute = 0.05 kg, formula_solvent = H2O, m_solvent = 0.5 kg, P0 = 3173.0723684210525 Pa.*

![Vapor pressure of the solution as m_solute runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/raoult-vapor-pressure-live-2)

*Vapor pressure of the solution as m_solute runs from 0.03 to 0.07 kg, every other input as you entered it (29 solutions). Marked: your m_solute = 0.05 kg gives P = 3156. Your inputs: formula_solute = C12H22O11, m_solute = 0.05 kg, formula_solvent = H2O, m_solvent = 0.5 kg, P0 = 3173.0723684210525 Pa.*

```python
chemistry_ai999x.solve('raoult_vapor_pressure', formula_solute='C12H22O11', m_solute=0.05, formula_solvent='H2O', m_solvent=0.5, P0=3173.0723684210525)
# -> P = 3156.46 Pa, dP = 16.6124 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| What is the vapor pressure of a solution of 50.0 g of C12H22O11 in 500 g of H2O if the vapor pressure of pure water is 23.8 torr? | SOLVED | P = 3156.46 Pa, dP = 16.6124 Pa |

<a id="freezing_point_depression"></a>

### Freezing-point depression: `freezing_point_depression`

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

![Freezing-point change as i runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/freezing-point-depression-live-1)

*Freezing-point change as i runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your i = 1 gives dT = 0.93. Your inputs: i = 1, K = 1.86 K*kg/mol, b = 0.5 mol/kg.*

![Freezing-point change as K runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/freezing-point-depression-live-2)

*Freezing-point change as K runs from 1.116 to 2.604 K*kg/mol, every other input as you entered it (29 solutions). Marked: your K = 1.86 K*kg/mol gives dT = 0.93. Your inputs: i = 1, K = 1.86 K*kg/mol, b = 0.5 mol/kg.*

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

### Boiling-point elevation: `boiling_point_elevation`

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

![Boiling-point change as i runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boiling-point-elevation-live-1)

*Boiling-point change as i runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your i = 1 gives dT = 0.256. Your inputs: i = 1, K = 0.512 K*kg/mol, b = 0.5 mol/kg.*

![Boiling-point change as K runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/boiling-point-elevation-live-2)

*Boiling-point change as K runs from 0.3072 to 0.7168 K*kg/mol, every other input as you entered it (29 solutions). Marked: your K = 0.512 K*kg/mol gives dT = 0.256. Your inputs: i = 1, K = 0.512 K*kg/mol, b = 0.5 mol/kg.*

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

### Molar mass from freezing-point depression: `molar_mass_from_freezing_point`

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

![Molar mass of solute as m_solvent runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-freezing-point-live-1)

*Molar mass of solute as m_solvent runs from 0.06 to 0.14 kg, every other input as you entered it (41 solutions). Marked: your m_solvent = 0.1 kg gives M = 100 g/mol. Your inputs: m_solute = 0.002 kg, m_solvent = 0.1 kg, K = 1.86 K*kg/mol, dT = 0.372 K, i = 1.*

![Molar mass of solute as dT runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-freezing-point-live-2)

*Molar mass of solute as dT runs from 0.186 to 0.651 K, every other input as you entered it (30 solutions). Marked: your dT = 0.372 K gives M = 100 g/mol. Your inputs: m_solute = 0.002 kg, m_solvent = 0.1 kg, K = 1.86 K*kg/mol, dT = 0.372 K, i = 1.*

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

### Osmotic pressure: `osmotic_pressure`

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

![Osmotic pressure as i runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/osmotic-pressure-live-1)

*Osmotic pressure as i runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your i = 1 gives Pi = 247.9 kPa. Your inputs: i = 1, c = 100 mol/m^3, T = 298.15 K.*

![Osmotic pressure as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/osmotic-pressure-live-2)

*Osmotic pressure as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions). Marked: your c = 100 mol/m^3 gives Pi = 247.9 kPa. Your inputs: i = 1, c = 100 mol/m^3, T = 298.15 K.*

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

### Molar mass from osmotic pressure: `molar_mass_from_osmotic_pressure`

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

![Molar mass as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-osmotic-pressure-live-1)

*Molar mass as m runs from 0.0006 to 0.0014 kg, every other input as you entered it (41 solutions). Marked: your m = 0.001 kg gives M = 74.37. Your inputs: m = 0.001 kg, V = 0.0001 m^3, Pi = 333.30592105263156 Pa, T = 298.15 K, i = 1.*

![Molar mass as V runs from 6e-05 to 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/molar-mass-from-osmotic-pressure-live-2)

*Molar mass as V runs from 6e-05 to 0.00014 m^3, every other input as you entered it (29 solutions). Marked: your V = 0.0001 m^3 gives M = 74.37. Your inputs: m = 0.001 kg, V = 0.0001 m^3, Pi = 333.30592105263156 Pa, T = 298.15 K, i = 1.*

```python
chemistry_ai999x.solve('molar_mass_from_osmotic_pressure', m=0.001, V=0.0001, Pi=333.30592105263156, T=298.15, i=1.0)
# -> M = 74.3748 kg/mol, M_g = 74374.8   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A solution of 1.00 g of a protein (a nonelectrolyte) in 100 mL of water has an osmotic pressure of 2.50 torr at 25 °C. What is the molar mass of the protein? | SOLVED | M = 74.3748 kg/mol, M_g = 74374.8 |

<a id="kp_from_kc"></a>

### Kp from Kc: `kp_from_kc`

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

![Pressure equilibrium constant as dn runs from -3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/kp-from-kc-live-1)

*Pressure equilibrium constant as dn runs from -3.5 to -0.5, every other input as you entered it (41 solutions). Marked: your dn = -2 gives Kp = 0.0004641. Your inputs: Kc = 0.5, T = 400 K, dn = -2.*

![Pressure equilibrium constant as T runs from 360 to 440 K, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/kp-from-kc-live-2)

*Pressure equilibrium constant as T runs from 360 to 440 K, every other input as you entered it (29 solutions). Marked: your T = 400 K gives Kp = 0.0004641. Your inputs: Kc = 0.5, T = 400 K, dn = -2.*

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

### Kc from Kp: `kc_from_kp`

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

![Concentration equilibrium constant as dn runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/kc-from-kp-live-1)

*Concentration equilibrium constant as dn runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your dn = 1 gives Kc = 0.04194. Your inputs: Kp = 1.8, T = 523 K, dn = 1.*

![Concentration equilibrium constant as Kp runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/kc-from-kp-live-2)

*Concentration equilibrium constant as Kp runs from 0.45 to 3.15, every other input as you entered it (29 solutions). Marked: your Kp = 1.8 gives Kc = 0.04194. Your inputs: Kp = 1.8, T = 523 K, dn = 1.*

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

### Equilibrium concentrations (ICE table): `equilibrium_ice`

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

![The concentration vectors as arrows from the origin: the initial concentrations (0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/equilibrium-ice-vectors)

*The concentration vectors as arrows from the origin: the initial concentrations (0.1, 0) as given, and the equilibrium concentrations the model returns (0.08984, 0.02033), with the extent x = 0.01016 mol/L. Your inputs: equation = N2O4 ⇌ 2 NO2, initial = 0.10, 0, K = 0.0046.*

![Extent of reaction as initial[2] runs from 0 to 1, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/equilibrium-ice-live-1)

*Extent of reaction as initial[2] runs from 0 to 1, every other input as you entered it (21 solutions). Marked: your initial[2] = 0 gives x = 0.01016. Your inputs: equation = N2O4 ⇌ 2 NO2, initial = 0.10, 0, K = 0.0046.*

![Every entry of equilibrium as computed for your inputs (2 entries: 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/equilibrium-ice-live-2)

*Every entry of equilibrium as computed for your inputs (2 entries: 0.08984, 0.02033). Your inputs: equation = N2O4 ⇌ 2 NO2, initial = 0.10, 0, K = 0.0046.*

![Extent of reaction as initial[1] runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/equilibrium-ice-live-3)

*Extent of reaction as initial[1] runs from 0.025 to 0.175, every other input as you entered it (29 solutions). Marked: your initial[1] = 0.1 gives x = 0.01016. Your inputs: equation = N2O4 ⇌ 2 NO2, initial = 0.10, 0, K = 0.0046.*

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

### Equilibrium constant from the standard Gibbs energy, `k_from_delta_g`

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

![lnK (ln K) over T from 149 to 447 and dG from -4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-delta-g-field)

*lnK (ln K) over T from 149 to 447 and dG from -4.95e+04 to -1.65e+04: each of the 21 x 15 cells is one run of the model, shaded by its value (light 4.44 to dark 39.96); iso-lines at lnK = 10, 20, 30. Marked: the inputs given, T = 298, dG = -3.3e+04, lnK = 13.32. Your inputs: dG = -33000 J/mol, T = 298 K.*

![Ln K as T runs from 258 to 338 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-delta-g-live-1)

*Ln K as T runs from 258 to 338 K, every other input as you entered it (41 solutions). Marked: your T = 298 K gives lnK = 13.32. Your inputs: dG = -33000 J/mol, T = 298 K.*

![Ln K as dG runs from -4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-delta-g-live-2)

*Ln K as dG runs from -4.62e+04 to -1.98e+04 J/mol, every other input as you entered it (29 solutions). Marked: your dG = -3.3e+04 J/mol gives lnK = 13.32. Your inputs: dG = -33000 J/mol, T = 298 K.*

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

### Standard Gibbs energy from the equilibrium constant, `delta_g_from_k`

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

![dG (standard Gibbs energy change) over T from 149 to 447 and K from 5e+04 to 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-k-field)

*dG (standard Gibbs energy change) over T from 149 to 447 and K from 5e+04 to 1.5e+05: each of the 21 x 15 cells is one run of the model, shaded by its value (light -4.43e+04 to dark -1.34e+04 J/mol); iso-lines at dG = -4e+04, -3.5e+04, -3e+04, -2.5e+04, -2e+04, -1.5e+04. Marked: the inputs given, T = 298, K = 1e+05, dG = -2.853e+04 J/mol. Your inputs: K = 100000, T = 298 K.*

![Standard Gibbs energy change as T runs from 258 to 338 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-k-live-1)

*Standard Gibbs energy change as T runs from 258 to 338 K, every other input as you entered it (41 solutions). Marked: your T = 298 K gives dG = -2.853e+04. Your inputs: K = 100000, T = 298 K.*

![Standard Gibbs energy change as K runs from 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-k-live-2)

*Standard Gibbs energy change as K runs from 2.5e+04 to 1.75e+05, every other input as you entered it (29 solutions). Marked: your K = 1e+05 gives dG = -2.853e+04. Your inputs: K = 100000, T = 298 K.*

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

### Van 't Hoff equation: K at another temperature, `vant_hoff_k2`

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

![Ln K2 as T1 runs from 258 to 338 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/vant-hoff-k2-live-1)

*Ln K2 as T1 runs from 258 to 338 K, every other input as you entered it (41 solutions). Marked: your T1 = 298 K gives lnK2 = -7.927. Your inputs: K1 = 1.8e-05, T1 = 298 K, T2 = 350 K, dH = 50000 J/mol.*

![Ln K2 as T2 runs from 310 to 390 K, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/vant-hoff-k2-live-2)

*Ln K2 as T2 runs from 310 to 390 K, every other input as you entered it (29 solutions). Marked: your T2 = 350 K gives lnK2 = -7.927. Your inputs: K1 = 1.8e-05, T1 = 298 K, T2 = 350 K, dH = 50000 J/mol.*

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

### Gibbs energy change at non-standard conditions, `delta_g_nonstandard`

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

![Gibbs energy change as dG0 runs from -4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-nonstandard-live-1)

*Gibbs energy change as dG0 runs from -4.62e+04 to -1.98e+04 J/mol, every other input as you entered it (41 solutions). Marked: your dG0 = -3.3e+04 J/mol gives dG = -2.159e+04. Your inputs: dG0 = -33000 J/mol, T = 298 K, Q = 100.*

![Gibbs energy change as T runs from 258 to 338 K, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-nonstandard-live-2)

*Gibbs energy change as T runs from 258 to 338 K, every other input as you entered it (29 solutions). Marked: your T = 298 K gives dG = -2.159e+04. Your inputs: dG0 = -33000 J/mol, T = 298 K, Q = 100.*

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

### Molar solubility from Ksp: `ksp_molar_solubility`

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

![Molar solubility as y runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ksp-molar-solubility-live-1)

*Molar solubility as y runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your y = 2 gives s = 0.2136 mmol/L. Your inputs: Ksp = 3.9e-11, x = 1, y = 2.*

![Molar solubility as x runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ksp-molar-solubility-live-2)

*Molar solubility as x runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your x = 1 gives s = 0.2136 mmol/L. Your inputs: Ksp = 3.9e-11, x = 1, y = 2.*

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

### Ksp from molar solubility: `ksp_from_solubility`

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

![Solubility product as x runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ksp-from-solubility-live-1)

*Solubility product as x runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your x = 1 gives Ksp = 1.35e-08. Your inputs: s = 1.5 mol/m^3, x = 1, y = 2.*

![Solubility product as y runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ksp-from-solubility-live-2)

*Solubility product as y runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your y = 2 gives Ksp = 1.35e-08. Your inputs: s = 1.5 mol/m^3, x = 1, y = 2.*

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

### Molar solubility with a common ion: `common_ion_solubility`

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

![Molar solubility as x runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/common-ion-solubility-live-1)

*Molar solubility as x runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your x = 1 gives s = 3.9e-06 mmol/L. Your inputs: Ksp = 3.9e-11, x = 1, y = 2, c_common = 100 mol/m^3, common = anion.*

![Molar solubility for every choice of common, every other input as you entered it (highlighted: your choice)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/common-ion-solubility-live-2)

*Molar solubility for every choice of common, every other input as you entered it (highlighted: your choice). cation: 0.009874; anion: 3.9e-06. Your inputs: Ksp = 3.9e-11, x = 1, y = 2, c_common = 100 mol/m^3, common = anion.*

![Molar solubility as y runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/common-ion-solubility-live-3)

*Molar solubility as y runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your y = 2 gives s = 3.9 nmol/L. Your inputs: Ksp = 3.9e-11, x = 1, y = 2, c_common = 100 mol/m^3, common = anion.*

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

### pH from the hydronium concentration, `ph_from_hydronium`

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

![PH as h runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-from-hydronium-live-1)

*PH as h runs from 0.6 to 1.4 mol/m^3, every other input as you entered it (41 solutions). Marked: your h = 1 mol/m^3 gives pH = 3. Your inputs: h = 1 mol/m^3.*

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

### Hydronium concentration from pH: `hydronium_from_ph`

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

![H as pH runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydronium-from-ph-live-1)

*H as pH runs from 0.8 to 5.6, every other input as you entered it (41 solutions). Marked: your pH = 3.2 gives h = 0.631 mmol/L. Your inputs: pH = 3.2.*

![Every answer of one unit for your inputs: h ([H3O+]) = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydronium-from-ph-live-2)

*Every answer of one unit for your inputs: h ([H3O+]) = 0.631; oh ([OH-]) = 1.585e-08. Your inputs: pH = 3.2.*

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

### pH from the hydroxide concentration, `ph_from_hydroxide`

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

![PH as oh runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-from-hydroxide-live-1)

*PH as oh runs from 0.06 to 0.14 mol/m^3, every other input as you entered it (41 solutions). Marked: your oh = 0.1 mol/m^3 gives pH = 10. Your inputs: oh = 0.1 mol/m^3.*

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

### pH of a strong monoprotic acid, `ph_strong_acid`

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

![PH as c runs from 6 to 14 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-strong-acid-live-1)

*PH as c runs from 6 to 14 mol/m^3, every other input as you entered it (41 solutions). Marked: your c = 10 mol/m^3 gives pH = 2. Your inputs: c = 10 mol/m^3.*

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

### pH of a strong base, `ph_strong_base`

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

![PH as n_oh runs from 1 to 3, every other input as you entered it (3 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-strong-base-live-1)

*PH as n_oh runs from 1 to 3, every other input as you entered it (3 solutions). Marked: your n_oh = 1 gives pH = 12. Your inputs: c = 10 mol/m^3, n_oh = 1.*

![PH as c runs from 6 to 14 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-strong-base-live-2)

*PH as c runs from 6 to 14 mol/m^3, every other input as you entered it (29 solutions). Marked: your c = 10 mol/m^3 gives pH = 12. Your inputs: c = 10 mol/m^3, n_oh = 1.*

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

### pH of a weak acid, `ph_weak_acid`

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

![pH (pH) over c from 10 to 1000 and Ka from 1](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/ph-weak-acid-field.svg)

*pH (pH) over c from 10 to 1000 and Ka from 1.8e-06 to 0.00018 (logarithmic axes): each of the 21 x 15 cells is one run of the model, shaded by its value (light 1.875 to dark 3.875); iso-lines at pH = 2, 2.5, 3, 3.5. Marked: the inputs given, c = 100, Ka = 1.8e-05, pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.*

![PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-acid-live-1)

*PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c = 100 mol/m^3 gives pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.*

![PH as Ka runs from 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-acid-live-2)

*PH as Ka runs from 4.5e-06 to 3.15e-05, every other input as you entered it (29 solutions). Marked: your Ka = 1.8e-05 gives pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.*

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

### pH of a weak base, `ph_weak_base`

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

![pH (pH) over c from 50 to 150 and Kb from 9e-06 to 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-base-field)

*pH (pH) over c from 50 to 150 and Kb from 9e-06 to 2.7e-05: each of the 21 x 15 cells is one run of the model, shaded by its value (light 10.82 to dark 11.3); iso-lines at pH = 10.9, 11, 11.1, 11.2, 11.3. Marked: the inputs given, c = 100, Kb = 1.8e-05, pH = 11.12. Your inputs: Kb = 1.8e-05, c = 100 mol/m^3.*

![PH as Kb runs from 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-base-live-1)

*PH as Kb runs from 4.5e-06 to 3.15e-05, every other input as you entered it (41 solutions). Marked: your Kb = 1.8e-05 gives pH = 11.12. Your inputs: Kb = 1.8e-05, c = 100 mol/m^3.*

![PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-weak-base-live-2)

*PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions). Marked: your c = 100 mol/m^3 gives pH = 11.12. Your inputs: Kb = 1.8e-05, c = 100 mol/m^3.*

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

### Ka from the pH of a weak acid: `ka_from_ph`

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

![pKa (pKa) over pH from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ka-from-ph-field)

*pKa (pKa) over pH from 1.435 to 4.305 and c from 10 to 1000 (logarithmic vertical axis): each of the 21 x 15 cells is one run of the model, shaded by its value (light -0.3915 to dark 8.61); iso-lines at pKa = 0, 2, 4, 6, 8; blank cells: inputs the model refuses. Marked: the inputs given, pH = 2.87, c = 100, pKa = 4.734. Your inputs: pH = 2.87, c = 100 mol/m^3.*

![PKa as pH runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ka-from-ph-live-1)

*PKa as pH runs from 1.04 to 5.022, every other input as you entered it (38 solutions). Marked: your pH = 2.87 gives pKa = 4.734. Your inputs: pH = 2.87, c = 100 mol/m^3.*

![PKa as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ka-from-ph-live-2)

*PKa as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions). Marked: your c = 100 mol/m^3 gives pKa = 4.734. Your inputs: pH = 2.87, c = 100 mol/m^3.*

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

### Buffer pH (Henderson-Hasselbalch): `buffer_ph`

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

![PH as c_base runs from 90 to 210 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/buffer-ph-live-1)

*PH as c_base runs from 90 to 210 mol/m^3, every other input as you entered it (41 solutions). Marked: your c_base = 150 mol/m^3 gives pH = 4.936. Your inputs: Ka = 1.7378008287493764e-05, c_base = 150 mol/m^3, c_acid = 100 mol/m^3.*

![PH as Ka runs from 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/buffer-ph-live-2)

*PH as Ka runs from 4.345e-06 to 3.041e-05, every other input as you entered it (29 solutions). Marked: your Ka = 1.738e-05 gives pH = 4.936. Your inputs: Ka = 1.7378008287493764e-05, c_base = 150 mol/m^3, c_acid = 100 mol/m^3.*

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

### Titration: equivalence-point volume, `titration_equivalence_volume`

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

![Titrant volume at equivalence as c_t runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-equivalence-volume-live-1)

*Titrant volume at equivalence as c_t runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c_t = 100 mol/m^3 gives V_eq = 37.5 mL. Your inputs: c_t = 100 mol/m^3, V_a = 2.5e-05 m^3, c_a = 150 mol/m^3, ratio = 1.*

![Titrant volume at equivalence as V_a runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-equivalence-volume-live-2)

*Titrant volume at equivalence as V_a runs from 1.5e-05 to 3.5e-05 m^3, every other input as you entered it (29 solutions). Marked: your V_a = 2.5e-05 m^3 gives V_eq = 37.5 mL. Your inputs: c_t = 100 mol/m^3, V_a = 2.5e-05 m^3, c_a = 150 mol/m^3, ratio = 1.*

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

### pH during a strong acid - strong base titration, `titration_ph_strong_acid_strong_base`

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

![Titration curve of a strong acid (100 mol/m^3, 25 mL) with a strong base (100 mol/m^3): the pH the model returns for 121 added volumes from 0 to 50 mL; the equivalence volume 25 mL is the model's own equivalence-volume capability](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-strong-acid-strong-base-titration)

*Titration curve of a strong acid (100 mol/m^3, 25 mL) with a strong base (100 mol/m^3): the pH the model returns for 121 added volumes from 0 to 50 mL; the equivalence volume 25 mL is the model's own equivalence-volume capability. Marked: V_b = 10 mL, pH = 1.368. Your inputs: c_a = 100 mol/m^3, V_a = 2.5e-05 m^3, c_b = 100 mol/m^3, V_b = 1e-05 m^3.*

![PH as c_a runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-strong-acid-strong-base-live-1)

*PH as c_a runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c_a = 100 mol/m^3 gives pH = 1.368. Your inputs: c_a = 100 mol/m^3, V_a = 2.5e-05 m^3, c_b = 100 mol/m^3, V_b = 1e-05 m^3.*

![PH as V_a runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-strong-acid-strong-base-live-2)

*PH as V_a runs from 1.5e-05 to 3.5e-05 m^3, every other input as you entered it (29 solutions). Marked: your V_a = 2.5e-05 m^3 gives pH = 1.368. Your inputs: c_a = 100 mol/m^3, V_a = 2.5e-05 m^3, c_b = 100 mol/m^3, V_b = 1e-05 m^3.*

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

### pH during a weak acid - strong base titration, `titration_ph_weak_acid_strong_base`

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

![pH against volume of 100 mol/m^3 base added to 25 mL of 100 mol/m^3 acid, computed by the model at 101 volumes for the weak acid (Ka = 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-weak-acid-strong-base-titration)

*pH against volume of 100 mol/m^3 base added to 25 mL of 100 mol/m^3 acid, computed by the model at 101 volumes for the weak acid (Ka = 1.8e-05) and for a strong acid of the same concentration. Shaded: the buffer region, 10 % to 90 % neutralised (pH 3.799 to 5.699). At equivalence (25 mL) pH = 8.722. Marked: V_b = 12.5 mL, pH = 4.745. Your inputs: Ka = 1.8e-05, c_a = 100 mol/m^3, V_a = 2.5e-05 m^3, c_b = 100 mol/m^3, V_b = 1.25e-05 m^3.*

![PH as c_a runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-weak-acid-strong-base-live-1)

*PH as c_a runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c_a = 100 mol/m^3 gives pH = 4.745. Your inputs: Ka = 1.8e-05, c_a = 100 mol/m^3, V_a = 2.5e-05 m^3, c_b = 100 mol/m^3, V_b = 1.25e-05 m^3.*

![PH as V_a runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/titration-ph-weak-acid-strong-base-live-2)

*PH as V_a runs from 1.5e-05 to 3.5e-05 m^3, every other input as you entered it (29 solutions). Marked: your V_a = 2.5e-05 m^3 gives pH = 4.745. Your inputs: Ka = 1.8e-05, c_a = 100 mol/m^3, V_a = 2.5e-05 m^3, c_b = 100 mol/m^3, V_b = 1.25e-05 m^3.*

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

### pH of a diprotic acid, `ph_diprotic_acid`

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

![PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-diprotic-acid-live-1)

*PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (41 solutions). Marked: your c = 100 mol/m^3 gives pH = 3.684. Your inputs: Ka1 = 4.3e-07, Ka2 = 4.8e-11, c = 100 mol/m^3.*

![PH as Ka1 runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-diprotic-acid-live-2)

*PH as Ka1 runs from 1.075e-07 to 7.525e-07, every other input as you entered it (29 solutions). Marked: your Ka1 = 4.3e-07 gives pH = 3.684. Your inputs: Ka1 = 4.3e-07, Ka2 = 4.8e-11, c = 100 mol/m^3.*

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

### pH of a salt of a weak acid, `ph_salt_of_weak_acid`

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

![pH (pH) over c from 50 to 150 and Ka from 9e-06 to 2](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-salt-of-weak-acid-field)

*pH (pH) over c from 50 to 150 and Ka from 9e-06 to 2.7e-05: each of the 21 x 15 cells is one run of the model, shaded by its value (light 8.634 to dark 9.111); iso-lines at pH = 8.7, 8.8, 8.9, 9, 9.1. Marked: the inputs given, c = 100, Ka = 1.8e-05, pH = 8.872. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.*

![PH as Ka runs from 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-salt-of-weak-acid-live-1)

*PH as Ka runs from 4.5e-06 to 3.15e-05, every other input as you entered it (41 solutions). Marked: your Ka = 1.8e-05 gives pH = 8.872. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.*

![PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/ph-salt-of-weak-acid-live-2)

*PH as c runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions). Marked: your c = 100 mol/m^3 gives pH = 8.872. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.*

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

### Zero-order integrated rate law: `zero_order_concentration`

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

![Concentration at time t as A0 runs from 300 to 700 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/zero-order-concentration-live-1)

*Concentration at time t as A0 runs from 300 to 700 mol/m^3, every other input as you entered it (41 solutions). Marked: your A0 = 500 mol/m^3 gives A = 0.3 mol/L. Your inputs: A0 = 500 mol/m^3, k = 2 mol/(m^3*s), t = 100 s.*

![Concentration at time t as k runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/zero-order-concentration-live-2)

*Concentration at time t as k runs from 1.2 to 2.8 mol/(m^3*s), every other input as you entered it (29 solutions). Marked: your k = 2 mol/(m^3*s) gives A = 0.3 mol/L. Your inputs: A0 = 500 mol/m^3, k = 2 mol/(m^3*s), t = 100 s.*

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

### First-order integrated rate law: `first_order_concentration`

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

![Reactant concentration against time from A0 = 800 mol/m^3 for zero-, first- and second-order rate laws with the same initial rate (12 mol/(m^3 s); k = 12 mol/(m^3 s), 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-concentration-kinetics)

*Reactant concentration against time from A0 = 800 mol/m^3 for zero-, first- and second-order rate laws with the same initial rate (12 mol/(m^3 s); k = 12 mol/(m^3 s), 0.015 1/s and 1.875e-05 m^3/(mol s)), each computed by the model at 61 times up to 90 s. Marked: t = 60 s, [A] = 325.3 mol/m^3 (first order). Your inputs: A0 = 800 mol/m^3, k = 0.015 1/s, t = 60 s.*

![Concentration at time t as A0 runs from 480 to 1120 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-concentration-live-1)

*Concentration at time t as A0 runs from 480 to 1120 mol/m^3, every other input as you entered it (41 solutions). Marked: your A0 = 800 mol/m^3 gives A = 0.3253 mol/L. Your inputs: A0 = 800 mol/m^3, k = 0.015 1/s, t = 60 s.*

![Concentration at time t as k runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-concentration-live-2)

*Concentration at time t as k runs from 0.009 to 0.021 1/s, every other input as you entered it (29 solutions). Marked: your k = 0.015 1/s gives A = 0.3253 mol/L. Your inputs: A0 = 800 mol/m^3, k = 0.015 1/s, t = 60 s.*

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

### Second-order integrated rate law: `second_order_concentration`

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

![Concentration at time t as k runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/second-order-concentration-live-1)

*Concentration at time t as k runs from 0.0003 to 0.0007 m^3/(mol*s), every other input as you entered it (41 solutions). Marked: your k = 0.0005 m^3/(mol*s) gives A = 50 mmol/L. Your inputs: A0 = 200 mol/m^3, k = 0.0005 m^3/(mol*s), t = 30 s.*

![Concentration at time t as t runs from 18 to 42 s, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/second-order-concentration-live-2)

*Concentration at time t as t runs from 18 to 42 s, every other input as you entered it (29 solutions). Marked: your t = 30 s gives A = 50 mmol/L. Your inputs: A0 = 200 mol/m^3, k = 0.0005 m^3/(mol*s), t = 30 s.*

```python
chemistry_ai999x.solve('second_order_concentration', A0=200.0, k=0.0005, t=30.0)
# -> A = 50 mol/m^3, A_molL = 0.05   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A second-order reaction has k = 0.50 M^-1 s^-1. If [A]0 = 0.20 M, what is the concentration after 30 s? | SOLVED | A = 50 mol/m^3, A_molL = 0.05 |

<a id="first_order_half_life"></a>

### Half-life of a first-order reaction: `first_order_half_life`

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

![Half-life as k runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-half-life-live-1)

*Half-life as k runs from 0.015 to 0.035 1/s, every other input as you entered it (41 solutions). Marked: your k = 0.025 1/s gives t_half = 27.73. Your inputs: k = 0.025 1/s.*

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

### Time to reach a concentration (first order), `first_order_time`

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

![Time needed as k runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-time-live-1)

*Time needed as k runs from 0.018 to 0.042 1/s, every other input as you entered it (41 solutions). Marked: your k = 0.03 1/s gives t = 53.65. Your inputs: A0 = 500 mol/m^3, A = 100 mol/m^3, k = 0.03 1/s.*

![Time needed as A0 runs from 300 to 700 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/first-order-time-live-2)

*Time needed as A0 runs from 300 to 700 mol/m^3, every other input as you entered it (29 solutions). Marked: your A0 = 500 mol/m^3 gives t = 53.65. Your inputs: A0 = 500 mol/m^3, A = 100 mol/m^3, k = 0.03 1/s.*

```python
chemistry_ai999x.solve('first_order_time', A0=500.0, A=100.0, k=0.03)
# -> t = 53.6479 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A first-order reaction has k = 0.0300 s^-1. How long does it take for the concentration to fall from 0.500 M to 0.100 M? | SOLVED | t = 53.6479 s |

<a id="arrhenius_k2"></a>

### Arrhenius equation: rate constant at another temperature, `arrhenius_k2`

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

![Rate constant at T2 as T1 runs from 260 to 340 K, every other input as you entered it (41 solutions)](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/arrhenius-k2-live-1.svg)

*Rate constant at T2 as T1 runs from 260 to 340 K, every other input as you entered it (41 solutions). Marked: your T1 = 300 K gives k2 = 0.08804. Your inputs: k1 = 0.0012, T1 = 300 K, T2 = 350 K, Ea = 75000 J/mol.*

![Rate constant at T2 as T2 runs from 310 to 390 K, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/arrhenius-k2-live-2)

*Rate constant at T2 as T2 runs from 310 to 390 K, every other input as you entered it (29 solutions). Marked: your T2 = 350 K gives k2 = 0.08804. Your inputs: k1 = 0.0012, T1 = 300 K, T2 = 350 K, Ea = 75000 J/mol.*

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

### Activation energy from two rate constants: `activation_energy`

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

![Activation energy as T1 runs from 260 to 318 K, every other input as you entered it (30 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/activation-energy-live-1)

*Activation energy as T1 runs from 260 to 318 K, every other input as you entered it (30 solutions). Marked: your T1 = 300 K gives Ea = 6.423e+04. Your inputs: k1 = 0.001, T1 = 300 K, k2 = 0.005, T2 = 320 K.*

![Activation energy as T2 runs from 302](https://www.ai999x.com/?ax_fig=chemistry-ai999x/activation-energy-live-2)

*Activation energy as T2 runs from 302.9 to 360 K, every other input as you entered it (21 solutions). Marked: your T2 = 320 K gives Ea = 6.423e+04. Your inputs: k1 = 0.001, T1 = 300 K, k2 = 0.005, T2 = 320 K.*

```python
chemistry_ai999x.solve('activation_energy', k1=0.001, T1=300.0, k2=0.005, T2=320.0)
# -> Ea = 64231.7 J/mol, Ea_kJ = 64.2317   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| k1 = 1.0e-3 s^-1 at T1 = 300 K and k2 = 5.0e-3 s^-1 at T2 = 320 K. What is the activation energy? | SOLVED | Ea = 64231.7 J/mol, Ea_kJ = 64.2317 |

<a id="arrhenius_rate_constant"></a>

### Arrhenius equation: rate constant, `arrhenius_rate_constant`

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

![Arrhenius plot: the rate constant (logarithmic axis) against 1000/T, each point a run of the model at 61 temperatures from 350 K to 650 K, the other inputs as given: a straight line of slope -Ea/R](https://www.ai999x.com/?ax_fig=chemistry-ai999x/arrhenius-rate-constant-arrhenius)

*Arrhenius plot: the rate constant (logarithmic axis) against 1000/T, each point a run of the model at 61 temperatures from 350 K to 650 K, the other inputs as given: a straight line of slope -Ea/R. Marked: T = 500 K, k = 357.5. Your inputs: A = 10000000000000, Ea = 100000 J/mol, T = 500 K.*

![Rate constant as T runs from 460 to 540 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/arrhenius-rate-constant-live-1)

*Rate constant as T runs from 460 to 540 K, every other input as you entered it (41 solutions). Marked: your T = 500 K gives k = 357.5. Your inputs: A = 10000000000000, Ea = 100000 J/mol, T = 500 K.*

![Rate constant as Ea runs from 6e+04 to 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/arrhenius-rate-constant-live-2)

*Rate constant as Ea runs from 6e+04 to 1.4e+05 J/mol, every other input as you entered it (29 solutions). Marked: your Ea = 1e+05 J/mol gives k = 357.5. Your inputs: A = 10000000000000, Ea = 100000 J/mol, T = 500 K.*

```python
chemistry_ai999x.solve('arrhenius_rate_constant', A=10000000000000.0, Ea=100000.0, T=500.0)
# -> k = 357.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A reaction has a pre-exponential factor of 1.0e13 s^-1 and Ea = 100 kJ/mol. What is the rate constant at 500 K? | SOLVED | k = 357.5 |

<a id="reaction_order_from_rates"></a>

### Reaction order from initial rates: `reaction_order_from_rates`

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

![Reaction order as c2 runs from 120 to 280 mol/m^3, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-order-from-rates-live-1)

*Reaction order as c2 runs from 120 to 280 mol/m^3, every other input as you entered it (41 solutions). Marked: your c2 = 200 mol/m^3 gives order = 2. Your inputs: c1 = 100 mol/m^3, r1 = 2 mol/(m^3*s), c2 = 200 mol/m^3, r2 = 8 mol/(m^3*s).*

![Reaction order as c1 runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-order-from-rates-live-2)

*Reaction order as c1 runs from 60 to 140 mol/m^3, every other input as you entered it (29 solutions). Marked: your c1 = 100 mol/m^3 gives order = 2. Your inputs: c1 = 100 mol/m^3, r1 = 2 mol/(m^3*s), c2 = 200 mol/m^3, r2 = 8 mol/(m^3*s).*

```python
chemistry_ai999x.solve('reaction_order_from_rates', c1=100.0, r1=2.0, c2=200.0, r2=8.0)
# -> order = 2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| c1 = 0.10 M, r1 = 2.0e-3 M/s, c2 = 0.20 M, r2 = 8.0e-3 M/s. What is the order with respect to A? | SOLVED | order = 2 |

<a id="consecutive_first_order"></a>

### Consecutive first-order reactions (A -> B -> C), `consecutive_first_order`

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

![The consecutive first-order reaction A -> B -> C with k1 = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/consecutive-first-order-scheme)

*The consecutive first-order reaction A -> B -> C with k1 = 0.1 1/s and k2 = 0.05 1/s, starting from A0 = 1000 mol/m^3: the concentrations the model returns at t = 10 s are A = 367.9, B = 477.3 and C = 154.8 mol/m^3. Your inputs: A0 = 1000 mol/m^3, k1 = 0.1 1/s, k2 = 0.05 1/s, t = 10 s.*

![How the starting 1000 mol/m^3 of A divides between A, B and C over time (stacked shares, k1 = 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/consecutive-first-order-shares)

*How the starting 1000 mol/m^3 of A divides between A, B and C over time (stacked shares, k1 = 0.1 1/s, k2 = 0.05 1/s), each concentration computed by the model at 79 times. B is largest near t = 14.18 s (a share of 0.4999); at t = 80 s the shares are 0.0003355, 0.03596 and 0.9637. In SI units: B = 477.3 mol/m^3. Your inputs: A0 = 1000 mol/m^3, k1 = 0.1 1/s, k2 = 0.05 1/s, t = 10 s.*

![Intermediate concentration, reactant and product as t runs from 6 to 14 s, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/consecutive-first-order-live-1)

*Intermediate concentration, reactant and product as t runs from 6 to 14 s, every other input as you entered it (41 solutions). Marked: your t = 10 s gives B = 0.4773 mol/L. Your inputs: A0 = 1000 mol/m^3, k1 = 0.1 1/s, k2 = 0.05 1/s, t = 10 s.*

![Intermediate concentration, reactant and product as A0 runs from 600 to 1400 mol/m^3, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/consecutive-first-order-live-2)

*Intermediate concentration, reactant and product as A0 runs from 600 to 1400 mol/m^3, every other input as you entered it (29 solutions). Marked: your A0 = 1000 mol/m^3 gives B = 0.4773 mol/L. Your inputs: A0 = 1000 mol/m^3, k1 = 0.1 1/s, k2 = 0.05 1/s, t = 10 s.*

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

### Heat from a temperature change: `calorimetry_heat`

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

![Heat absorbed (negative: released) as T1 runs from 253](https://www.ai999x.com/?ax_fig=chemistry-ai999x/calorimetry-heat-live-1)

*Heat absorbed (negative: released) as T1 runs from 253.1 to 333.1 K, every other input as you entered it (41 solutions). Marked: your T1 = 293.1 K gives q = 12.55 kJ. Your inputs: m = 0.05 kg, c = 4184 J/(kg*K), T1 = 293.15 K, T2 = 353.15 K.*

![Heat absorbed (negative: released) as T2 runs from 313](https://www.ai999x.com/?ax_fig=chemistry-ai999x/calorimetry-heat-live-2)

*Heat absorbed (negative: released) as T2 runs from 313.1 to 393.1 K, every other input as you entered it (29 solutions). Marked: your T2 = 353.1 K gives q = 12.55 kJ. Your inputs: m = 0.05 kg, c = 4184 J/(kg*K), T1 = 293.15 K, T2 = 353.15 K.*

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

### Reaction enthalpy from coffee-cup calorimetry, `enthalpy_from_calorimetry`

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

![Reaction enthalpy as T1 runs from 255](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enthalpy-from-calorimetry-live-1)

*Reaction enthalpy as T1 runs from 255.1 to 335.1 K, every other input as you entered it (41 solutions). Marked: your T1 = 295.1 K gives dH = -5.685e+04. Your inputs: m = 0.1 kg, c = 4180 J/(kg*K), T1 = 295.15 K, T2 = 301.95 K, n = 0.05 mol.*

![Reaction enthalpy as T2 runs from 261](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enthalpy-from-calorimetry-live-2)

*Reaction enthalpy as T2 runs from 261.9 to 341.9 K, every other input as you entered it (29 solutions). Marked: your T2 = 301.9 K gives dH = -5.685e+04. Your inputs: m = 0.1 kg, c = 4180 J/(kg*K), T1 = 295.15 K, T2 = 301.95 K, n = 0.05 mol.*

```python
chemistry_ai999x.solve('enthalpy_from_calorimetry', m=0.1, c=4180.0, T1=295.15, T2=301.95, n=0.05)
# -> dH = -56848 J/mol, dH_kJ = -56.848, q_solution = 2842.4 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| When 0.0500 mol of HCl reacts in 100 g of solution (c = 4.18 J/(g·°C)), the temperature rises from 22.0 °C to 28.8 °C. What is the enthalpy of reaction? | SOLVED | dH = -56848 J/mol, dH_kJ = -56.848, q_solution = 2842.4 J |

<a id="reaction_enthalpy_from_formation"></a>

### Reaction enthalpy from enthalpies of formation (Hess's law), `reaction_enthalpy_from_formation`

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

![dH (standard reaction enthalpy, kJ/mol) as value 4 of values (enthalpy of formation of each species in the order written) moves from -571](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-enthalpy-from-formation-sensitivity)

*dH (standard reaction enthalpy, kJ/mol) as value 4 of values (enthalpy of formation of each species in the order written) moves from -571.6 to 0, the other values as given (-74.8, 0, -393.5, -285.8); 41 runs of the model. Equation: CH4 + 2 O2 -> CO2 + 2 H2O. Marked: the values given, dH = -890.3 kJ/mol. In SI units: dH = -8.903e+05 J/mol. Your inputs: equation = CH4 + 2 O2 -> CO2 + 2 H2O, values = -74.8, 0, -393.5, -285.8.*

![Standard reaction enthalpy as values[4] runs from -500](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-enthalpy-from-formation-live-1)

*Standard reaction enthalpy as values[4] runs from -500.2 to -71.45, every other input as you entered it (39 solutions). Marked: your values[4] = -285.8 gives dH = -8.903e+05. Your inputs: equation = CH4 + 2 O2 -> CO2 + 2 H2O, values = -74.8, 0, -393.5, -285.8.*

![Standard reaction enthalpy as values[3] runs from -688](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-enthalpy-from-formation-live-2)

*Standard reaction enthalpy as values[3] runs from -688.6 to -98.38, every other input as you entered it (29 solutions). Marked: your values[3] = -393.5 gives dH = -8.903e+05. Your inputs: equation = CH4 + 2 O2 -> CO2 + 2 H2O, values = -74.8, 0, -393.5, -285.8.*

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

### Gibbs energy change (dG = dH - T dS), `gibbs_free_energy`

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

![spontaneity (spontaneous / non-spontaneous) over dH from -1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gibbs-free-energy-decision)

*spontaneity (spontaneous / non-spontaneous) over dH from -1.383e+05 to -4.61e+04 and T from 29.8 to 2980: each of the 25 x 15 cells is one run of the model, coloured by the verdict it returned (spontaneous: 64 cells, non-spontaneous: 311 cells). Marked: the inputs given, dH = -9.22e+04, T = 298 gives spontaneous. In SI units: dG = -3.299e+04 J/mol. Your inputs: dH = -92200 J/mol, dS = -198.7 J/(mol*K), T = 298 K.*

![Gibbs energy change as dH runs from -1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gibbs-free-energy-live-1)

*Gibbs energy change as dH runs from -1.291e+05 to -5.532e+04 J/mol, every other input as you entered it (41 solutions). Marked: your dH = -9.22e+04 J/mol gives dG = -3.299e+04. Shaded by spontaneous / non-spontaneous: spontaneous, non-spontaneous. Your inputs: dH = -92200 J/mol, dS = -198.7 J/(mol*K), T = 298 K.*

![Spontaneous / non-spontaneous for 25 5 x 5 combinations of dH and T around your inputs (the other inputs as entered); the ring marks your inputs, which give spontaneous](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gibbs-free-energy-live-2)

*Spontaneous / non-spontaneous for 25 5 x 5 combinations of dH and T around your inputs (the other inputs as entered); the ring marks your inputs, which give spontaneous. Blank cells admit no answer. Your inputs: dH = -92200 J/mol, dS = -198.7 J/(mol*K), T = 298 K.*

![Gibbs energy change as T runs from 258 to 338 K, every other input as you entered it (27 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/gibbs-free-energy-live-3)

*Gibbs energy change as T runs from 258 to 338 K, every other input as you entered it (27 solutions). Marked: your T = 298 K gives dG = -3.299e+04. Your inputs: dH = -92200 J/mol, dS = -198.7 J/(mol*K), T = 298 K.*

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

### Temperature at which dG = 0, `crossover_temperature`

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

![T (temperature where dG = 0) over dH from 8](https://www.ai999x.com/?ax_fig=chemistry-ai999x/crossover-temperature-field)

*T (temperature where dG = 0) over dH from 8.9e+04 to 2.67e+05 and dS from 80.5 to 241.5: each of the 21 x 15 cells is one run of the model, shaded by its value (light 368.5 to dark 3317 K); iso-lines at T = 500, 1000, 1500, 2000, 2500, 3000. Marked: the inputs given, dH = 1.78e+05, dS = 161, T = 1106 K. Your inputs: dH = 178000 J/mol, dS = 161 J/(mol*K).*

![T as dH runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/crossover-temperature-live-1)

*T as dH runs from 1.068e+05 to 2.492e+05 J/mol, every other input as you entered it (41 solutions). Marked: your dH = 1.78e+05 J/mol gives T = 1106. Your inputs: dH = 178000 J/mol, dS = 161 J/(mol*K).*

![T as dS runs from 96](https://www.ai999x.com/?ax_fig=chemistry-ai999x/crossover-temperature-live-2)

*T as dS runs from 96.6 to 225.4 J/(mol*K), every other input as you entered it (29 solutions). Marked: your dS = 161 J/(mol*K) gives T = 1106. Your inputs: dH = 178000 J/mol, dS = 161 J/(mol*K).*

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

### Heat of reaction for a given mass: `heat_from_reaction_mass`

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

![q (enthalpy change (negative: heat released)) over m from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/heat-from-reaction-mass-field)

*q (enthalpy change (negative: heat released)) over m from 0.005 to 0.015 and dH from -1.335e+06 to -4.45e+05: each of the 21 x 15 cells is one run of the model, shaded by its value (light -1.248e+06 to dark -1.387e+05 J); iso-lines at q = -1.2e+06, -1e+06, -8e+05, -6e+05, -4e+05, -2e+05. Marked: the inputs given, m = 0.01, dH = -8.9e+05, q = -5.548e+05 J. Your inputs: formula = CH4, m = 0.01 kg, dH = -890000 J/mol.*

![Enthalpy change and heat released as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/heat-from-reaction-mass-live-1)

*Enthalpy change and heat released as m runs from 0.006 to 0.014 kg, every other input as you entered it (41 solutions). Marked: your m = 0.01 kg gives q = -554.8 kJ. Your inputs: formula = CH4, m = 0.01 kg, dH = -890000 J/mol.*

![Enthalpy change and heat released as dH runs from -1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/heat-from-reaction-mass-live-2)

*Enthalpy change and heat released as dH runs from -1.246e+06 to -5.34e+05 J/mol, every other input as you entered it (29 solutions). Marked: your dH = -8.9e+05 J/mol gives q = -554.8 kJ. Your inputs: formula = CH4, m = 0.01 kg, dH = -890000 J/mol.*

```python
chemistry_ai999x.solve('heat_from_reaction_mass', formula='CH4', m=0.01, dH=-890000.0)
# -> q = -554759 J, heat_released = 554759 J   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How much heat is released when 10.0 g of CH4 burns? ΔH = -890 kJ/mol. | SOLVED | q = -554759 J, heat_released = 554759 J |

<a id="clausius_clapeyron_pressure"></a>

### Clausius-Clapeyron: vapor pressure at another temperature, `clausius_clapeyron_pressure`

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

![Vapor pressure at T2 as T1 runs from 258](https://www.ai999x.com/?ax_fig=chemistry-ai999x/clausius-clapeyron-pressure-live-1)

*Vapor pressure at T2 as T1 runs from 258.1 to 338.1 K, every other input as you entered it (41 solutions). Marked: your T1 = 298.1 K gives P2 = 12.49 kPa. Your inputs: P1 = 3173.0723684210525 Pa, T1 = 298.15 K, T2 = 323.15 K, dH = 43900 J/mol.*

![Vapor pressure at T2 as T2 runs from 283](https://www.ai999x.com/?ax_fig=chemistry-ai999x/clausius-clapeyron-pressure-live-2)

*Vapor pressure at T2 as T2 runs from 283.1 to 363.1 K, every other input as you entered it (29 solutions). Marked: your T2 = 323.1 K gives P2 = 12.49 kPa. Your inputs: P1 = 3173.0723684210525 Pa, T1 = 298.15 K, T2 = 323.15 K, dH = 43900 J/mol.*

```python
chemistry_ai999x.solve('clausius_clapeyron_pressure', P1=3173.0723684210525, T1=298.15, T2=323.15, dH=43900.0)
# -> P2 = 12487.6 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Water has a vapor pressure of 23.8 torr at 25 °C and ΔHvap = 43.9 kJ/mol. What is its vapor pressure at 50 °C? | SOLVED | P2 = 12487.6 Pa |

<a id="clausius_clapeyron_enthalpy"></a>

### Clausius-Clapeyron: enthalpy of vaporization, `clausius_clapeyron_enthalpy`

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

![Enthalpy of vaporization as T1 runs from 260 to 338 K, every other input as you entered it (40 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/clausius-clapeyron-enthalpy-live-1)

*Enthalpy of vaporization as T1 runs from 260 to 338 K, every other input as you entered it (40 solutions). Marked: your T1 = 300 K gives dH = 2.939e+04. Your inputs: P1 = 13332.236842105263 Pa, T1 = 300 K, P2 = 53328.94736842105 Pa, T2 = 340 K.*

![Enthalpy of vaporization as T2 runs from 302](https://www.ai999x.com/?ax_fig=chemistry-ai999x/clausius-clapeyron-enthalpy-live-2)

*Enthalpy of vaporization as T2 runs from 302.9 to 380 K, every other input as you entered it (28 solutions). Marked: your T2 = 340 K gives dH = 2.939e+04. Your inputs: P1 = 13332.236842105263 Pa, T1 = 300 K, P2 = 53328.94736842105 Pa, T2 = 340 K.*

```python
chemistry_ai999x.solve('clausius_clapeyron_enthalpy', P1=13332.236842105263, T1=300.0, P2=53328.94736842105, T2=340.0)
# -> dH = 29392 J/mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| P1 = 100 torr at T1 = 300 K and P2 = 400 torr at T2 = 340 K. What is the enthalpy of vaporization? | SOLVED | dH = 29392 J/mol |

<a id="delta_g_from_cell_potential"></a>

### Gibbs energy from a cell potential: `delta_g_from_cell_potential`

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

![dG (Gibbs energy change) over n from 1 to 3 and E from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-cell-potential-field)

*dG (Gibbs energy change) over n from 1 to 3 and E from 0.55 to 1.65: each of the 21 x 15 cells is one run of the model, shaded by its value (light -4.776e+05 to dark -5.307e+04 J/mol); iso-lines at dG = -4e+05, -3e+05, -2e+05, -1e+05. Marked: the inputs given, n = 2, E = 1.1, dG = -2.123e+05 J/mol. Your inputs: n = 2, E = 1.1 V.*

![Gibbs energy change as n runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-cell-potential-live-1)

*Gibbs energy change as n runs from 0.5 to 3.5, every other input as you entered it (41 solutions). Marked: your n = 2 gives dG = -2.123e+05. Your inputs: n = 2, E = 1.1 V.*

![Gibbs energy change as E runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/delta-g-from-cell-potential-live-2)

*Gibbs energy change as E runs from 0.66 to 1.54 V, every other input as you entered it (29 solutions). Marked: your E = 1.1 V gives dG = -2.123e+05. Your inputs: n = 2, E = 1.1 V.*

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

### Equilibrium constant from a standard cell potential, `k_from_cell_potential`

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

![Ln K as T runs from 258 to 338 K, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-cell-potential-live-1)

*Ln K as T runs from 258 to 338 K, every other input as you entered it (41 solutions). Marked: your T = 298 K gives lnK = 35.83. Your inputs: n = 2, E = 0.46 V, T = 298 K.*

![Ln K as n runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/k-from-cell-potential-live-2)

*Ln K as n runs from 0.5 to 3.5, every other input as you entered it (29 solutions). Marked: your n = 2 gives lnK = 35.83. Your inputs: n = 2, E = 0.46 V, T = 298 K.*

```python
chemistry_ai999x.solve('k_from_cell_potential', n=2.0, E=0.46, T=298.0)
# -> lnK = 35.826, K = 3.62282e+15   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A cell has E° = 0.46 V with n = 2 at 298 K. What is the equilibrium constant? | SOLVED | lnK = 35.826, K = 3.62282e+15 |

<a id="nernst_potential"></a>

### Nernst equation: `nernst_potential`

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

![Cell potential as E0 runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/nernst-potential-live-1)

*Cell potential as E0 runs from 0.66 to 1.54 V, every other input as you entered it (41 solutions). Marked: your E0 = 1.1 V gives E = 1.159. Your inputs: E0 = 1.1 V, n = 2, Q = 0.01, T = 298 K.*

![Cell potential as n runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/nernst-potential-live-2)

*Cell potential as n runs from 0.5 to 3.5, every other input as you entered it (29 solutions). Marked: your n = 2 gives E = 1.159. Your inputs: E0 = 1.1 V, n = 2, Q = 0.01, T = 298 K.*

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

### Mass deposited in electrolysis (Faraday's law), `electrolysis_mass`

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

![Mass deposited as I runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/electrolysis-mass-live-1)

*Mass deposited as I runs from 1.2 to 2.8 A, every other input as you entered it (41 solutions). Marked: your I = 2 A gives m = 1.185 g. Your inputs: metal = Cu, I = 2 A, t = 1800 s, n = 2.*

![Mass deposited as t runs from 1080 to 2520 s, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/electrolysis-mass-live-2)

*Mass deposited as t runs from 1080 to 2520 s, every other input as you entered it (29 solutions). Marked: your t = 1800 s gives m = 1.185 g. Your inputs: metal = Cu, I = 2 A, t = 1800 s, n = 2.*

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

### Time for electrolysis (Faraday's law): `electrolysis_time`

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

![Time needed as m runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/electrolysis-time-live-1)

*Time needed as m runs from 0.006 to 0.014 kg, every other input as you entered it (41 solutions). Marked: your m = 0.01 kg gives t = 1.827 h. Your inputs: metal = Ni, m = 0.01 kg, I = 5 A, n = 2.*

![Time needed as I runs from 3 to 7 A, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/electrolysis-time-live-2)

*Time needed as I runs from 3 to 7 A, every other input as you entered it (29 solutions). Marked: your I = 5 A gives t = 1.827 h. Your inputs: metal = Ni, m = 0.01 kg, I = 5 A, n = 2.*

```python
chemistry_ai999x.solve('electrolysis_time', metal='Ni', m=0.01, I=5.0, n=2.0)
# -> t = 6575.59 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| How long must a current of 5.00 A flow to deposit 10.0 g of Ni from a Ni2+ solution? | SOLVED | t = 6575.59 s |

<a id="photon_energy"></a>

### Energy of a photon from its wavelength: `photon_energy`

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

![Energy per photon as λ runs from 3e-07 to 7e-07 m, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/photon-energy-live-1)

*Energy per photon as λ runs from 3e-07 to 7e-07 m, every other input as you entered it (41 solutions). Marked: your λ = 5e-07 m gives E = 2.48 eV. Your inputs: lam = 5e-07 m.*

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

### Wavelength of a photon from its energy: `photon_wavelength`

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

![Wavelength as E runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/photon-wavelength-live-1)

*Wavelength as E runs from 1.8e-19 to 4.2e-19 J, every other input as you entered it (41 solutions). Marked: your E = 3e-19 J gives λ = 0.6621 um. Your inputs: E = 3e-19 J.*

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

### Beer-Lambert law: absorbance, `beer_lambert_absorbance`

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

![Absorbance as ε runs from 900 to 2100 m^2/mol, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/beer-lambert-absorbance-live-1)

*Absorbance as ε runs from 900 to 2100 m^2/mol, every other input as you entered it (41 solutions). Marked: your ε = 1500 m^2/mol gives A = 0.3. Your inputs: eps = 1500 m^2/mol, l = 0.01 m, c = 0.02 mol/m^3.*

![Absorbance as l runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/beer-lambert-absorbance-live-2)

*Absorbance as l runs from 0.006 to 0.014 m, every other input as you entered it (29 solutions). Marked: your l = 0.01 m gives A = 0.3. Your inputs: eps = 1500 m^2/mol, l = 0.01 m, c = 0.02 mol/m^3.*

```python
chemistry_ai999x.solve('beer_lambert_absorbance', eps=1500.0, l=0.01, c=0.02)
# -> A = 0.3, T_percent = 50.1187   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A 1.00 cm cell holds a 2.0e-5 M solution with molar absorptivity 15000 L/(mol·cm). What is the absorbance? | SOLVED | A = 0.3, T_percent = 50.1187 |

<a id="beer_lambert_concentration"></a>

### Beer-Lambert law: concentration, `beer_lambert_concentration`

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

![Concentration as A runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/beer-lambert-concentration-live-1)

*Concentration as A runs from 0.1125 to 0.7875, every other input as you entered it (41 solutions). Marked: your A = 0.45 gives c = 36 umol/L. Your inputs: A = 0.45, eps = 1250 m^2/mol, l = 0.01 m.*

![Concentration as ε runs from 750 to 1750 m^2/mol, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/beer-lambert-concentration-live-2)

*Concentration as ε runs from 750 to 1750 m^2/mol, every other input as you entered it (29 solutions). Marked: your ε = 1250 m^2/mol gives c = 36 umol/L. Your inputs: A = 0.45, eps = 1250 m^2/mol, l = 0.01 m.*

```python
chemistry_ai999x.solve('beer_lambert_concentration', A=0.45, eps=1250.0, l=0.01)
# -> c = 0.036 mol/m^3, c_molL = 3.6e-05   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A solution has an absorbance of 0.450 in a 1.00 cm cell; ε = 1.25e4 L/(mol·cm). What is its concentration? | SOLVED | c = 0.036 mol/m^3, c_molL = 3.6e-05 |

<a id="hydrogen_transition_wavelength"></a>

### Hydrogen atom transition wavelength (Rydberg), `hydrogen_transition_wavelength`

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

![The hydrogen line spectrum: the wavelength of the first six lines of each series, computed by the model, each drawn at its photon energy (also computed by the model, in eV)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydrogen-transition-wavelength-spectrum)

*The hydrogen line spectrum: the wavelength of the first six lines of each series, computed by the model, each drawn at its photon energy (also computed by the model, in eV). Ranges: Lyman 121.6 to 93.08 nm; Balmer 656.5 to 389 nm; Paschen 1876 to 923.2 nm. Marked: n = 3 -> 2, wavelength 656.5 nm, E = 1.889 eV. In SI units: lam = 6.565e-07 m. Your inputs: n_upper = 3, n_lower = 2.*

![Wavelength as n_lower runs from 1 to 2, every other input as you entered it (2 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydrogen-transition-wavelength-live-1)

*Wavelength as n_lower runs from 1 to 2, every other input as you entered it (2 solutions). Marked: your n_lower = 2 gives λ = 0.6565 um. Your inputs: n_upper = 3, n_lower = 2.*

![Wavelength as n_upper runs from 3 to 7, every other input as you entered it (5 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/hydrogen-transition-wavelength-live-2)

*Wavelength as n_upper runs from 3 to 7, every other input as you entered it (5 solutions). Marked: your n_upper = 3 gives λ = 0.6565 um. Your inputs: n_upper = 3, n_lower = 2.*

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

### Degree of unsaturation: `degree_of_unsaturation`

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

![The degree of unsaturation for 8 substances, each returned by the model: C₂H₆ 0, C₂H₄ 1, C₂H₂ 2, C₆H₁₂ 1, C₆H₆ 4, C₅H₅N 4, C₁₀H₈ 7, C₂H₃Cl 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/degree-of-unsaturation-by-molecule)

*The degree of unsaturation for 8 substances, each returned by the model: C₂H₆ 0, C₂H₄ 1, C₂H₂ 2, C₆H₁₂ 1, C₆H₆ 4, C₅H₅N 4, C₁₀H₈ 7, C₂H₃Cl 1. Highlighted: the formula given, C₆H₆. Your inputs: formula = C6H6.*

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

### Vibrational wavenumber of a diatomic molecule, `vibrational_wavenumber`

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

![Wavenumber as k runs from 309](https://www.ai999x.com/?ax_fig=chemistry-ai999x/vibrational-wavenumber-live-1)

*Wavenumber as k runs from 309.6 to 722.4 N/m, every other input as you entered it (41 solutions). Marked: your k = 516 N/m gives wavenumber = 2.99e+05. Your inputs: molecule = HCl, k = 516 N/m.*

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

<a id="average_atomic_mass"></a>

### Average atomic mass from isotopes: `average_atomic_mass`

The average (relative) atomic mass of an element from the masses and natural abundances of its isotopes.

$$
\bar{A} = \sum_i f_i\, m_i
$$

| input | meaning | condition |
|---|---|---|
| `masses` | mass of each isotope (u) |  |
| `abundances` | abundance of each isotope, in the same order (fractions adding up to 1, or percentages adding up to 100) |  |

| output | meaning |
|---|---|
| `A_r` | average atomic mass (u, numerically g/mol) |
| `M` | molar mass of the element |

- **Method:** abundance-weighted sum of the isotope masses (abundances used as written)
- **Assumes:** one abundance per isotope; abundances add up to 1 (within 0.002) or to 100 (within 0.2); masses > 0, abundances >= 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The answer for your inputs: A_r (average atomic mass (u, numerically g/mol)) = 35](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/average-atomic-mass-live-1.svg)

*The answer for your inputs: A_r (average atomic mass (u, numerically g/mol)) = 35.45. Your inputs: masses = 34.97, 36.97, abundances = 75.78, 24.22.*

```python
chemistry_ai999x.solve('average_atomic_mass', masses='34.97, 36.97', abundances='75.78, 24.22')
# -> A_r = 35.4544, M = 0.0354544 kg/mol   (SOLVED)
```

```python
chemistry_ai999x.solve('average_atomic_mass', masses='23.985, 24.986, 25.983', abundances='0.7899, 0.1000, 0.1101')
# -> A_r = 24.3051, M = 0.0243051 kg/mol   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Isotope masses are 34.97 u and 36.97 u with abundances 75.78% and 24.22%. What is the average atomic mass? | SOLVED | A_r = 35.4544, M = 0.0354544 kg/mol |
| Find the average atomic mass of magnesium from isotope masses 23.985, 24.986 and 25.983 u and abundances 0.7899, 0.1000 and 0.1101. | SOLVED | A_r = 24.3051, M = 0.0243051 kg/mol |
| Isotope masses are 34.97 u and 36.97 u with abundances 75.78% and 30.00%. What is the average atomic mass? | INVALID_INPUT | Average atomic mass from isotopes: the abundances must add up to 1 (fractions) or to 100 (percent). |

<a id="isotope_abundance"></a>

### Abundances of two isotopes from the average atomic mass, `isotope_abundance`

The natural abundances of an element's two isotopes from their masses and the average atomic mass.

$$
x_1 = \frac{\bar{A} - m_2}{m_1 - m_2},\quad x_2 = 1 - x_1
$$

| input | meaning | condition |
|---|---|---|
| `m1` | mass of the first isotope (u) |  |
| `m2` | mass of the second isotope (u) |  |
| `A_r` | average atomic mass (u) |  |

| output | meaning |
|---|---|
| `x1` | fractional abundance of the first isotope |
| `x2` | fractional abundance of the second isotope |
| `percent1` | percent abundance of the first isotope |
| `percent2` | percent abundance of the second isotope |

- **Method:** the average mass is x1 $m_{1} + (1 - x_{1})$ m2, solved for x1
- **Assumes:** two isotopes of different mass; the average mass lies strictly between them
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Percent abundance of the first isotope as m2 runs from 35](https://www.ai999x.com/?ax_fig=chemistry-ai999x/isotope-abundance-live-1)

*Percent abundance of the first isotope as m2 runs from 35.58 to 64.7, every other input as you entered it (22 solutions). Marked: your m2 = 36.97 gives percent1 = 76. Your inputs: m1 = 34.97, m2 = 36.97, A_r = 35.45.*

![Percent abundance of the first isotope as m1 runs from 8](https://www.ai999x.com/?ax_fig=chemistry-ai999x/isotope-abundance-live-2)

*Percent abundance of the first isotope as m1 runs from 8.742 to 34.97, every other input as you entered it (15 solutions). Marked: your m1 = 34.97 gives percent1 = 76. Your inputs: m1 = 34.97, m2 = 36.97, A_r = 35.45.*

```python
chemistry_ai999x.solve('isotope_abundance', m1=34.97, m2=36.97, A_r=35.45)
# -> x1 = 0.76, x2 = 0.24, percent1 = 76, percent2 = 24   (SOLVED)
```

```python
chemistry_ai999x.solve('isotope_abundance', m1=62.93, m2=64.93, A_r=63.55)
# -> x1 = 0.69, x2 = 0.31, percent1 = 69, percent2 = 31   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The average atomic mass of chlorine is 35.45 u. Its isotope masses are 34.97 u and 36.97 u. What are the percent abundances of the two isotopes? | SOLVED | x1 = 0.76, x2 = 0.24, percent1 = 76, percent2 = 24 |
| The average atomic mass of copper is 63.55 u and the isotope masses are 62.93 u and 64.93 u. Find the abundance of each isotope. | SOLVED | x1 = 0.69, x2 = 0.31, percent1 = 69, percent2 = 31 |
| The average atomic mass of copper is 66.00 u and the isotope masses are 62.93 u and 64.93 u. Find the abundance of each isotope. | INVALID_INPUT | Abundances of two isotopes from the average atomic mass: the average atomic mass must lie between the two isotope masses |

<a id="standard_cell_potential"></a>

### Standard cell potential from two half-reactions, `standard_cell_potential`

The standard potential of a galvanic cell from the standard reduction potentials of its cathode and anode half-reactions, and whether the cell reaction is spontaneous.

$$
E^\circ_{cell} = E^\circ_{cathode} - E^\circ_{anode}
$$

| input | meaning | condition |
|---|---|---|
| `E_cathode` | standard reduction potential of the cathode half-reaction |  |
| `E_anode` | standard reduction potential of the anode half-reaction |  |

| output | meaning |
|---|---|
| `E_cell` | standard cell potential |
| `spontaneity` | spontaneous (galvanic) or not (electrolytic) |

- **Method:** both potentials are reduction potentials; the anode runs as an oxidation, so its potential is subtracted
- **Assumes:** both values are standard REDUCTION potentials; |E°| <= 3.5 V
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Standard cell potential as E_anode runs from -1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/standard-cell-potential-live-1)

*Standard cell potential as E_anode runs from -1.064 to -0.456 V, every other input as you entered it (41 solutions). Marked: your E_anode = -0.76 V gives E_cell = 1.1. Your inputs: E_cathode = 0.34 V, E_anode = -0.76 V.*

![Standard cell potential as E_cathode runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/standard-cell-potential-live-2)

*Standard cell potential as E_cathode runs from 0.204 to 0.476 V, every other input as you entered it (29 solutions). Marked: your E_cathode = 0.34 V gives E_cell = 1.1. Your inputs: E_cathode = 0.34 V, E_anode = -0.76 V.*

```python
chemistry_ai999x.solve('standard_cell_potential', E_cathode=0.34, E_anode=-0.76)
# -> E_cell = 1.1 V, spontaneity = spontaneous (galvanic)   (SOLVED)
```

```python
chemistry_ai999x.solve('standard_cell_potential', E_cathode=0.34, E_anode=0.8)
# -> E_cell = -0.46 V, spontaneity = non-spontaneous (electrolytic)   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| E°(cathode) = 0.34 V and E°(anode) = -0.76 V. What is the standard cell potential? | SOLVED | E_cell = 1.1 V, spontaneity = spontaneous (galvanic) |
| The cathode half-reaction has E° = 0.80 V and the anode half-reaction has E° = 0.34 V. Find the standard cell potential. | SOLVED | E_cell = 0.46 V, spontaneity = spontaneous (galvanic) |
| E°(cathode) = 4.20 V and E°(anode) = -0.76 V. What is the standard cell potential? | INVALID_INPUT | Standard cell potential from two half-reactions: standard reduction potentials lie between about -3.1 V and +3.0 V (\|E°\| |

<a id="bond_enthalpy_reaction"></a>

### Reaction enthalpy from bond enthalpies: `bond_enthalpy_reaction`

An estimate of a gas-phase reaction enthalpy from the average bond enthalpies of the bonds broken and formed.

$$
\Delta H \approx \sum_{\text{broken}} n\,D - \sum_{\text{formed}} n\,D
$$

| input | meaning | condition |
|---|---|---|
| `broken_n` | number of each kind of bond broken |  |
| `broken_D` | bond enthalpy of each kind of bond broken (kJ/mol), same order |  |
| `formed_n` | number of each kind of bond formed |  |
| `formed_D` | bond enthalpy of each kind of bond formed (kJ/mol), same order |  |

| output | meaning |
|---|---|
| `dH` | estimated reaction enthalpy |
| `dH_kJ` | in kJ/mol |
| `broken_kJ` | energy to break the bonds (kJ/mol) |
| `formed_kJ` | energy released forming the bonds (kJ/mol) |

- **Method:** energy to break the bonds minus energy released forming the new ones (average bond enthalpies)
- **Assumes:** bond counts and bond enthalpies > 0; one count per bond enthalpy; an estimate: average bond enthalpies, gas phase
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The answer for your inputs: dH (estimated reaction enthalpy) = -8](https://www.ai999x.com/?ax_fig=chemistry-ai999x/bond-enthalpy-reaction-live-1)

*The answer for your inputs: dH (estimated reaction enthalpy) = -8.18e+05. Your inputs: broken_n = 4, 2, broken_D = 413, 498, formed_n = 2, 4, formed_D = 799, 467.*

```python
chemistry_ai999x.solve('bond_enthalpy_reaction', broken_n='4, 2', broken_D='413, 498', formed_n='2, 4', formed_D='799, 467')
# -> dH = -818000 J/mol, dH_kJ = -818, broken_kJ = 2648, formed_kJ = 3466   (SOLVED)
```

```python
chemistry_ai999x.solve('bond_enthalpy_reaction', broken_n='1, 1', broken_D='436, 243', formed_n='2', formed_D='432')
# -> dH = -185000 J/mol, dH_kJ = -185, broken_kJ = 679, formed_kJ = 864   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bonds broken: 4 × 413, 2 × 498; bonds formed: 2 × 799, 4 × 467 (all in kJ/mol). Estimate the reaction enthalpy from these bond enthalpies. | SOLVED | dH = -818000 J/mol, dH_kJ = -818, broken_kJ = 2648, formed_kJ = 3466 |
| Bonds broken: 436, 243; bonds formed: 2 × 432. Estimate the reaction enthalpy from bond enthalpies in kJ/mol. | SOLVED | dH = -185000 J/mol, dH_kJ = -185, broken_kJ = 679, formed_kJ = 864 |
| Bonds broken: 436, -243; bonds formed: 2 × 432. Estimate the reaction enthalpy from bond enthalpies in kJ/mol. | INVALID_INPUT | Reaction enthalpy from bond enthalpies: bond enthalpies are positive (the energy needed to break a bond). |

<a id="henry_law_solubility"></a>

### Henry's law: solubility of a gas, `henry_law_solubility`

The concentration of a dissolved gas from its Henry's law constant and its partial pressure.

$$
c = k_H\, P
$$

| input | meaning | condition |
|---|---|---|
| `kH` | Henry's law constant (solubility form, $c = kH$ P) |  |
| `P` | partial pressure of the gas |  |

| output | meaning |
|---|---|
| `c` | dissolved gas concentration |
| `c_molL` | in mol/L |

- **Method:** $c = kH$ P
- **Assumes:** kH, $P > 0$; dilute solution, no reaction with the solvent
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Dissolved gas concentration as P runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/henry-law-solubility-live-1)

*Dissolved gas concentration as P runs from 1.277e+04 to 2.979e+04 Pa, every other input as you entered it (41 solutions). Marked: your P = 2.128e+04 Pa gives c = 0.273 mmol/L. Your inputs: kH = 1.2830002467308166e-05..., P = 21278.25 Pa.*

![Dissolved gas concentration as kH runs from 7](https://www.ai999x.com/?ax_fig=chemistry-ai999x/henry-law-solubility-live-2)

*Dissolved gas concentration as kH runs from 7.698e-06 to 1.796e-05 mol/(m^3*Pa), every other input as you entered it (29 solutions). Marked: your kH = 1.283e-05 mol/(m^3*Pa) gives c = 0.273 mmol/L. Your inputs: kH = 1.2830002467308166e-05..., P = 21278.25 Pa.*

```python
chemistry_ai999x.solve('henry_law_solubility', kH=1.2830002467308166e-05, P=21278.25)
# -> c = 0.273 mol/m^3, c_molL = 0.000273   (SOLVED)
```

```python
chemistry_ai999x.solve('henry_law_solubility', kH=0.00033555391068344434, P=405300.0)
# -> c = 136 mol/m^3, c_molL = 0.136   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The Henry's law constant for O2 in water is 1.3e-3 M/atm. What is the solubility of O2 at a partial pressure of 0.21 atm? | SOLVED | c = 0.273 mol/m^3, c_molL = 0.000273 |
| Henry's law constant of CO2 = 3.4e-2 M/atm. Find the solubility of CO2 when its partial pressure is 4.0 atm. | SOLVED | c = 136 mol/m^3, c_molL = 0.136 |
| The Henry's law constant for N2 in water is 6.1e-4 M/atm. What is the solubility of N2 at a partial pressure of -0.78 atm? | INVALID_INPUT | Henry's law: solubility of a gas: P must be a positive number. |

<a id="reaction_quotient_direction"></a>

### Reaction quotient and the direction of shift, `reaction_quotient_direction`

The reaction quotient Q from the current concentrations (or partial pressures) and, compared with K, the direction in which the reaction proceeds to reach equilibrium.

$$
Q = \prod_i c_i^{\nu_i};\ Q < K \Rightarrow \text{forward},\ Q > K \Rightarrow \text{reverse}
$$

| input | meaning | condition |
|---|---|---|
| `equation` | the reaction as written (its coefficients define Q and K) |  |
| `conc` | current concentration of each species in the order written (mol/L, or atm for Kp) |  |
| `K` | equilibrium constant |  |

| output | meaning |
|---|---|
| `Q` | reaction quotient |
| `log10_Q` | log10 Q |
| `direction` | direction of the shift |

- **Method:** mass-action product of the current concentrations, compared with K
- **Assumes:** one reaction, coefficients as written; no pure solids or liquids in the equation; every concentration > 0; $K > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Reaction quotient as conc[2] runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-quotient-direction-live-1)

*Reaction quotient as conc[2] runs from 0.075 to 0.525, every other input as you entered it (41 solutions). Marked: your conc[2] = 0.3 gives Q = 1.852. Your inputs: equation = N2 + 3 H2 ⇌ 2 NH3, conc = 0.20, 0.30, 0.10, K = 0.105.*

![Reaction quotient as conc[3] runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-quotient-direction-live-2)

*Reaction quotient as conc[3] runs from 0.025 to 0.175, every other input as you entered it (29 solutions). Marked: your conc[3] = 0.1 gives Q = 1.852. Your inputs: equation = N2 + 3 H2 ⇌ 2 NH3, conc = 0.20, 0.30, 0.10, K = 0.105.*

```python
chemistry_ai999x.solve('reaction_quotient_direction', equation='N2 + 3 H2 ⇌ 2 NH3', conc='0.20, 0.30, 0.10', K=0.105)
# -> Q = 1.85185, log10_Q = 0.267606, direction = reverse (toward reactants)   (SOLVED)
```

```python
chemistry_ai999x.solve('reaction_quotient_direction', equation='H2 + I2 ⇌ 2 HI', conc='0.10, 0.20, 0.40', K=50.0)
# -> Q = 8, log10_Q = 0.90309, direction = forward (toward products)   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| For N2 + 3 H2 ⇌ 2 NH3, Kc = 0.105. The concentrations are [N2] = 0.20 M, [H2] = 0.30 M and [NH3] = 0.10 M. Calculate the reaction quotient Q and predict the direction of the shift. | SOLVED | Q = 1.85185, log10_Q = 0.267606, direction = reverse (toward reactants) |
| For H2 + I2 ⇌ 2 HI, K = 50.0. The concentrations are [H2] = 0.10 M, [I2] = 0.20 M and [HI] = 0.40 M. Find the reaction quotient and the direction of the shift. | SOLVED | Q = 8, log10_Q = 0.90309, direction = forward (toward products) |
| For N2O4 ⇌ 2 NO2, Kc = 4.6e-3. The concentrations are [N2O4] = 0.10 M and [NO2] = 0 M. Calculate the reaction quotient and the direction of the shift. | INVALID_INPUT | Reaction quotient and the direction of shift: every concentration must be above zero. |

<a id="precipitation_test"></a>

### Precipitation test (Q versus Ksp): `precipitation_test`

Whether a salt MxAy precipitates from a solution with given ion concentrations: the ion product Q compared with Ksp.

$$
Q = [M]^x[A]^y;\ Q > K_{sp} \Rightarrow \text{precipitate}
$$

| input | meaning | condition |
|---|---|---|
| `Ksp` | solubility product |  |
| `x` | cations per formula unit |  |
| `y` | anions per formula unit |  |
| `c_cation` | cation concentration (after any mixing) |  |
| `c_anion` | anion concentration (after any mixing) |  |

| output | meaning |
|---|---|
| `Q` | ion product Q |
| `log10_Q` | log10 Q |
| `result` | precipitate forms or not |

- **Method:** ion product from the stated ion concentrations, compared with Ksp (in logarithms)
- **Assumes:** Ksp > 0; ion concentrations > 0 (mol/L, after mixing); the bracketed ions are the salt's own
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Log10 Q as y runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/precipitation-test-live-1)

*Log10 Q as y runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your y = 2 gives log10_Q = -8. Shaded by precipitate forms or not: precipitate forms (Q > Ksp), no precipitate (Q < Ksp). Your inputs: Ksp = 9.8e-09, x = 1, y = 2, c_cation = 10 mol/m^3, c_anion = 1 mol/m^3.*

![Precipitate forms or not for 16 4 x 4 combinations of y and x around your inputs (the other inputs as entered); the ring marks your inputs, which give precipitate forms (Q > Ksp)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/precipitation-test-live-2)

*Precipitate forms or not for 16 4 x 4 combinations of y and x around your inputs (the other inputs as entered); the ring marks your inputs, which give precipitate forms (Q > Ksp). Blank cells admit no answer. Your inputs: Ksp = 9.8e-09, x = 1, y = 2, c_cation = 10 mol/m^3, c_anion = 1 mol/m^3.*

![Log10 Q as x runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/precipitation-test-live-3)

*Log10 Q as x runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your x = 1 gives log10_Q = -8. Shaded by precipitate forms or not: precipitate forms (Q > Ksp), no precipitate (Q < Ksp). Your inputs: Ksp = 9.8e-09, x = 1, y = 2, c_cation = 10 mol/m^3, c_anion = 1 mol/m^3.*

```python
chemistry_ai999x.solve('precipitation_test', Ksp=9.8e-09, x=1, y=2, c_cation=10.0, c_anion=1.0)
# -> Q = 1e-08, log10_Q = -8, result = precipitate forms (Q > Ksp)   (SOLVED)
```

```python
chemistry_ai999x.solve('precipitation_test', Ksp=1.8e-10, x=1, y=1, c_cation=0.01, c_anion=0.001)
# -> Q = 1e-11, log10_Q = -11, result = no precipitate (Q < Ksp)   (SOLVED)
```

```python
chemistry_ai999x.solve('precipitation_test', Ksp=3.9e-11, x=1, y=2, c_cation=0.2, c_anion=1.0)
# -> Q = 2e-10, log10_Q = -9.69897, result = precipitate forms (Q > Ksp)   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The Ksp of PbI2 is 9.8e-9. If [Pb2+] = 0.010 M and [I-] = 0.0010 M after mixing, will a precipitate form? | SOLVED | Q = 1e-08, log10_Q = -8, result = precipitate forms (Q > Ksp) |
| The Ksp of AgCl is 1.8e-10. If [Ag+] = 1.0e-5 M and [Cl-] = 1.0e-6 M, will a precipitate form? | SOLVED | Q = 1e-11, log10_Q = -11, result = no precipitate (Q < Ksp) |
| The Ksp of CaF2 is 3.9e-11. If [Ca2+] = 2.0e-4 M and [F-] = 1.0e-3 M, will a precipitate form? | SOLVED | Q = 2e-10, log10_Q = -9.69897, result = precipitate forms (Q > Ksp) |
| The Ksp of PbI2 is 9.8e-9. If [Pb2+] = -0.010 M and [I-] = 0.0010 M after mixing, will a precipitate form? | INVALID_INPUT | Precipitation test (Q versus Ksp): c_cation must be a positive number. |

<a id="reaction_entropy_from_standard_entropies"></a>

### Standard reaction entropy from standard molar entropies, `reaction_entropy_from_standard_entropies`

The standard entropy change of a reaction from the standard molar entropies of every species (balanced equation).

$$
\Delta S^\circ_{rxn} = \sum \nu_p S^\circ(p) - \sum \nu_r S^\circ(r)
$$

| input | meaning | condition |
|---|---|---|
| `equation` | balanced equation |  |
| `values` | standard molar entropy of each species in the order written (J/(mol·K)) |  |

| output | meaning |
|---|---|
| `dS` | standard reaction entropy |

- **Method:** coefficient-weighted sum of standard molar entropies, products minus reactants
- **Assumes:** the equation as written conserves every element; one value per species, in J/(mol·K)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Standard reaction entropy as values[2] runs from 32](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-entropy-from-standard-entropies-live-1)

*Standard reaction entropy as values[2] runs from 32.67 to 228.7, every other input as you entered it (39 solutions). Marked: your values[2] = 130.7 gives dS = -198.7. Your inputs: equation = N2 + 3 H2 -> 2 NH3, values = 191.6, 130.7, 192.5.*

![Standard reaction entropy as values[3] runs from 48](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-entropy-from-standard-entropies-live-2)

*Standard reaction entropy as values[3] runs from 48.12 to 336.9, every other input as you entered it (29 solutions). Marked: your values[3] = 192.5 gives dS = -198.7. Your inputs: equation = N2 + 3 H2 -> 2 NH3, values = 191.6, 130.7, 192.5.*

```python
chemistry_ai999x.solve('reaction_entropy_from_standard_entropies', equation='N2 + 3 H2 -> 2 NH3', values='191.6, 130.7, 192.5')
# -> dS = -198.7 J/(mol*K)   (SOLVED)
```

```python
chemistry_ai999x.solve('reaction_entropy_from_standard_entropies', equation='CaCO3 -> CaO + CO2', values='92.9, 39.8, 213.7')
# -> dS = 160.6 J/(mol*K)   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Using standard molar entropies N2 = 191.6, H2 = 130.7, NH3 = 192.5 (J/(mol·K)), find ΔS° for N2 + 3 H2 -> 2 NH3. | SOLVED | dS = -198.7 J/(mol*K) |
| Standard molar entropies: CaCO3 = 92.9, CaO = 39.8, CO2 = 213.7 J/(mol·K). Calculate the entropy change ΔS° for CaCO3 -> CaO + CO2. | SOLVED | dS = 160.6 J/(mol*K) |
| Using standard molar entropies N2 = 191.6, H2 = 130.7, NH3 = 192.5 (J/(mol·K)), find ΔS° for N2 + H2 -> NH3. | INVALID_INPUT | Standard reaction entropy from standard molar entropies: the equation as written is not balanced; balance it first. |

<a id="reaction_gibbs_from_formation"></a>

### Standard reaction Gibbs energy from Gibbs energies of formation, `reaction_gibbs_from_formation`

The standard Gibbs energy change of a reaction from the standard Gibbs energies of formation of every species (balanced equation), and whether it is spontaneous under standard conditions.

$$
\Delta G^\circ_{rxn} = \sum \nu_p \Delta G_f^\circ(p) - \sum \nu_r \Delta G_f^\circ(r)
$$

| input | meaning | condition |
|---|---|---|
| `equation` | balanced equation |  |
| `values` | standard Gibbs energy of formation of each species in the order written (kJ/mol) |  |

| output | meaning |
|---|---|
| `dG` | standard reaction Gibbs energy |
| `dG_kJ` | in kJ/mol |
| `spontaneity` | spontaneous / non-spontaneous under standard conditions |

- **Method:** coefficient-weighted sum of Gibbs energies of formation, products minus reactants
- **Assumes:** the equation as written conserves every element; one value per species, in kJ/mol
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Standard reaction Gibbs energy as values[4] runs from -414](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-gibbs-from-formation-live-1)

*Standard reaction Gibbs energy as values[4] runs from -414.9 to -59.27, every other input as you entered it (39 solutions). Marked: your values[4] = -237.1 gives dG = -8.178e+05. Your inputs: equation = CH4 + 2 O2 -> CO2 + 2 H2O, values = -50.8, 0, -394.4, -237.1.*

![Standard reaction Gibbs energy as values[3] runs from -690](https://www.ai999x.com/?ax_fig=chemistry-ai999x/reaction-gibbs-from-formation-live-2)

*Standard reaction Gibbs energy as values[3] runs from -690.2 to -98.6, every other input as you entered it (29 solutions). Marked: your values[3] = -394.4 gives dG = -8.178e+05. Your inputs: equation = CH4 + 2 O2 -> CO2 + 2 H2O, values = -50.8, 0, -394.4, -237.1.*

```python
chemistry_ai999x.solve('reaction_gibbs_from_formation', equation='CH4 + 2 O2 -> CO2 + 2 H2O', values='-50.8, 0, -394.4, -237.1')
# -> dG = -817800 J/mol, dG_kJ = -817.8, spontaneity = spontaneous   (SOLVED)
```

```python
chemistry_ai999x.solve('reaction_gibbs_from_formation', equation='N2 + 3 H2 -> 2 NH3', values='0, 0, -16.4')
# -> dG = -32800 J/mol, dG_kJ = -32.8, spontaneity = spontaneous   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Using standard Gibbs energies of formation CH4 = -50.8, O2 = 0, CO2 = -394.4, H2O = -237.1 (kJ/mol), find ΔG° for CH4 + 2 O2 -> CO2 + 2 H2O. | SOLVED | dG = -817800 J/mol, dG_kJ = -817.8, spontaneity = spontaneous |
| Standard Gibbs energies of formation: NH3 = -16.4, N2 = 0, H2 = 0 kJ/mol. Find ΔG° for N2 + 3 H2 -> 2 NH3. | SOLVED | dG = -32800 J/mol, dG_kJ = -32.8, spontaneity = spontaneous |
| Using standard Gibbs energies of formation CH4 = -50.8, O2 = 0, CO2 = -394.4, H2O = -237.1 (kJ/mol), find ΔG° for CH4 + O2 -> CO2 + H2O. | INVALID_INPUT | Standard reaction Gibbs energy from Gibbs energies of formation: the equation as written is not balanced; balance it fir |

<a id="nuclear_binding_energy"></a>

### Mass defect and nuclear binding energy: `nuclear_binding_energy`

The mass defect, binding energy and binding energy per nucleon of a nuclide from its atomic mass.

$$
\Delta m = Z\,m(^1\mathrm{H}) + (A - Z)\,m_n - m_{atom},\quad E_b = \Delta m c^2
$$

| input | meaning | condition |
|---|---|---|
| `Z` | atomic number (protons) |  |
| `A` | mass number (nucleons) |  |
| `mass` | atomic mass of the nuclide (u) |  |

| output | meaning |
|---|---|
| `dm` | mass defect (u) |
| `E_b` | binding energy |
| `E_b_MeV` | binding energy in MeV |
| `E_per_nucleon_MeV` | binding energy per nucleon in MeV |

- **Method:** mass defect with atomic masses (Z hydrogen atoms carry the electrons), times 931.494 MeV per u
- **Assumes:** $1  \leq  Z  \leq  A$, $A  \geq  2$; atomic mass in u, within 0.5 u of A; mass defect > 0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Binding energy in MeV as mass runs from 3](https://www.ai999x.com/?ax_fig=chemistry-ai999x/nuclear-binding-energy-live-1)

*Binding energy in MeV as mass runs from 3.522 to 4.033, every other input as you entered it (20 solutions). Marked: your mass = 4.003 gives E_b_MeV = 28.3. Your inputs: Z = 2, A = 4, mass = 4.0026.*

![Binding energy in MeV as Z runs from 1 to 4, every other input as you entered it (4 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/nuclear-binding-energy-live-2)

*Binding energy in MeV as Z runs from 1 to 4, every other input as you entered it (4 solutions). Marked: your Z = 2 gives E_b_MeV = 28.3. Your inputs: Z = 2, A = 4, mass = 4.0026.*

```python
chemistry_ai999x.solve('nuclear_binding_energy', Z=2, A=4, mass=4.0026)
# -> dm = 0.0303799, E_b = 4.53395e-12 J, E_b_MeV = 28.2987, E_per_nucleon_MeV = 7.07467   (SOLVED)
```

```python
chemistry_ai999x.solve('nuclear_binding_energy', Z=26, A=56, mass=55.9349)
# -> dm = 0.528498, E_b = 7.8874e-11 J, E_b_MeV = 492.293, E_per_nucleon_MeV = 8.79095   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Find the nuclear binding energy of a nuclide with Z = 2, A = 4 and atomic mass 4.0026 u. | SOLVED | dm = 0.0303799, E_b = 4.53395e-12 J, E_b_MeV = 28.2987, E_per_nucleon_MeV = 7.07467 |
| What is the mass defect and binding energy per nucleon of a nuclide with Z = 26, A = 56 and atomic mass 55.9349 u? | SOLVED | dm = 0.528498, E_b = 7.8874e-11 J, E_b_MeV = 492.293, E_per_nucleon_MeV = 8.79095 |
| Find the nuclear binding energy of a nuclide with Z = 26, A = 56 and atomic mass 65.9349 u. | INVALID_INPUT | Mass defect and nuclear binding energy: the atomic mass (in u) must lie within 0.5 u of the mass number A. |

<a id="radioactive_decay_remaining"></a>

### Amount remaining after radioactive decay (half-life), `radioactive_decay_remaining`

The mass of a radioactive nuclide that remains after a time, from its half-life.

$$
m = m_0 \left(\tfrac{1}{2}\right)^{t/t_{1/2}}
$$

| input | meaning | condition |
|---|---|---|
| `m0` | initial mass |  |
| `t_half` | half-life |  |
| `t` | elapsed time |  |

| output | meaning |
|---|---|
| `m` | mass remaining |
| `m_g` | in grams |
| `fraction` | fraction remaining |
| `half_lives` | number of half-lives |

- **Method:** $m = m_{0}$ $(1/2)^{t / t_{1}/2}$
- **Assumes:** m0, $t_{1}/2 > 0$; $t  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Mass remaining as t_half runs from 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/radioactive-decay-remaining-live-1)

*Mass remaining as t_half runs from 4.158e+05 to 9.701e+05 s, every other input as you entered it (41 solutions). Marked: your t_half = 6.929e+05 s gives m = 12.5 mg. Your inputs: m0 = 0.0001 kg, t_half = 692928 s, t = 2078784 s.*

![Mass remaining as t runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/radioactive-decay-remaining-live-2)

*Mass remaining as t runs from 1.247e+06 to 2.91e+06 s, every other input as you entered it (29 solutions). Marked: your t = 2.079e+06 s gives m = 12.5 mg. Your inputs: m0 = 0.0001 kg, t_half = 692928 s, t = 2078784 s.*

```python
chemistry_ai999x.solve('radioactive_decay_remaining', m0=0.0001, t_half=692928.0, t=2078784.0)
# -> m = 1.25e-05 kg, m_g = 0.0125, fraction = 0.125, half_lives = 3   (SOLVED)
```

```python
chemistry_ai999x.solve('radioactive_decay_remaining', m0=0.005, t_half=180825048000.0, t=361650096000.0)
# -> m = 0.00125 kg, m_g = 1.25, fraction = 0.25, half_lives = 2   (SOLVED)
```

```python
chemistry_ai999x.solve('radioactive_decay_remaining', m0=0.001, t_half=30.0, t=15.0)
# -> m = 0.000707107 kg, m_g = 0.707107, fraction = 0.707107, half_lives = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The half-life of iodine-131 is 8.02 days. How much of a 100 mg sample remains after 24.06 days? | SOLVED | m = 1.25e-05 kg, m_g = 0.0125, fraction = 0.125, half_lives = 3 |
| Carbon-14 has a half-life of 5730 years. What mass of a 5.00 g sample is left after 11460 years? | SOLVED | m = 0.00125 kg, m_g = 1.25, fraction = 0.25, half_lives = 2 |
| The half-life of iodine-131 is 8.02 days. How much of a 100 mg sample remains after -5.0 days? | INVALID_INPUT | Amount remaining after radioactive decay (half-life): the elapsed time cannot be negative. |

<a id="radioactive_dating_age"></a>

### Age from the fraction remaining (radioactive dating), `radioactive_dating_age`

The age of a sample (the time elapsed) from the fraction of a radioactive nuclide remaining and its half-life.

$$
t = t_{1/2}\,\log_2\frac{1}{f}
$$

| input | meaning | condition |
|---|---|---|
| `fraction` | fraction of the original nuclide remaining $(0 < f  \leq  1)$ |  |
| `t_half` | half-life |  |

| output | meaning |
|---|---|
| `t` | age (time elapsed) |
| `t_years` | in years |
| `half_lives` | number of half-lives |

- **Method:** $t = t_{1}/2$ ln(1/f) / ln 2
- **Assumes:** $0 < f  \leq  1$; $t_{1}/2 > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Age (time elapsed) as t_half runs from 1](https://www.ai999x.com/?ax_fig=chemistry-ai999x/radioactive-dating-age-live-1)

*Age (time elapsed) as t_half runs from 1.085e+11 to 2.532e+11 s, every other input as you entered it (41 solutions). Marked: your t_half = 1.808e+11 s gives t = 1.146e+04 yr. Your inputs: fraction = 0.25, t_half = 180825048000 s.*

![Age (time elapsed) as fraction runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/radioactive-dating-age-live-2)

*Age (time elapsed) as fraction runs from 0.0625 to 0.4375, every other input as you entered it (29 solutions). Marked: your fraction = 0.25 gives t = 1.146e+04 yr. Your inputs: fraction = 0.25, t_half = 180825048000 s.*

```python
chemistry_ai999x.solve('radioactive_dating_age', fraction=0.25, t_half=180825048000.0)
# -> t = 3.6165e+11 s, t_years = 11460, half_lives = 2   (SOLVED)
```

```python
chemistry_ai999x.solve('radioactive_dating_age', fraction=0.1, t_half=180825048000.0)
# -> t = 6.00688e+11 s, t_years = 19034.6, half_lives = 3.32193   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A wood sample has 25% of the original carbon-14 remaining. The half-life of carbon-14 is 5730 years. How old is the sample? | SOLVED | t = 3.6165e+11 s, t_years = 11460, half_lives = 2 |
| The fraction remaining is 0.10 and the half-life is 5730 years. What is the age of the sample? | SOLVED | t = 6.00688e+11 s, t_years = 19034.6, half_lives = 3.32193 |
| The fraction remaining is 1.5 and the half-life is 5730 years. What is the age of the sample? | INVALID_INPUT | Age from the fraction remaining (radioactive dating): the fraction remaining lies between 0 and 1. |

<a id="conjugate_dissociation_constant"></a>

### Conjugate acid-base constants (Ka Kb = Kw), `conjugate_dissociation_constant`

The dissociation constant of the conjugate base of an acid (or of the conjugate acid of a base), with pKa and pKb, at 25 °C.

$$
K_a K_b = K_w,\quad \mathrm{p}K_a + \mathrm{p}K_b = 14
$$

| input | meaning | condition |
|---|---|---|
| `K` | the given Ka (acid) or Kb (base) |  |
| `known` | which constant is given |  |

| output | meaning |
|---|---|
| `K_conj` | dissociation constant of the conjugate |
| `pK` | pK of the given constant |
| `pK_conj` | pK of the conjugate |
| `conjugate` | which constant the conjugate's is |

- **Method:** K_conj = Kw / K; $pK = -log10$ K
- **Assumes:** $K > 0$; 25 °C $(Kw = 1.0e-14)$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Dissociation constant of the conjugate as K runs from 4](https://www.ai999x.com/?ax_fig=chemistry-ai999x/conjugate-dissociation-constant-live-1)

*Dissociation constant of the conjugate as K runs from 4.5e-06 to 3.15e-05, every other input as you entered it (41 solutions). Marked: your K = 1.8e-05 gives K_conj = 5.556e-10. Your inputs: K = 1.8e-05, known = Ka.*

```python
chemistry_ai999x.solve('conjugate_dissociation_constant', K=1.8e-05, known='Ka')
# -> K_conj = 5.55556e-10, pK = 4.74473, pK_conj = 9.25527, conjugate = Kb of the conjugate base   (SOLVED)
```

```python
chemistry_ai999x.solve('conjugate_dissociation_constant', K=1e-05, known='Kb')
# -> K_conj = 1e-09, pK = 5, pK_conj = 9, conjugate = Ka of the conjugate acid   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Ka of acetic acid = 1.8e-5. What is Kb of its conjugate base? | SOLVED | K_conj = 5.55556e-10, pK = 4.74473, pK_conj = 9.25527, conjugate = Kb of the conjugate base |
| The pKa of an acid HA is 3.00. Find Kb of its conjugate base. | SOLVED | K_conj = 1e-11, pK = 3, pK_conj = 11, conjugate = Kb of the conjugate base |
| The pKb of a base B is 5.00. What is Ka of its conjugate acid? | SOLVED | K_conj = 1e-09, pK = 5, pK_conj = 9, conjugate = Ka of the conjugate acid |
| Ka of an acid HA = -1.0e-5. What is Kb of its conjugate base? | INVALID_INPUT | Conjugate acid-base constants (Ka Kb = Kw): K must be a positive number. |

<a id="specific_rotation"></a>

### Specific rotation from an observed rotation, `specific_rotation`

The specific rotation of an optically active compound from the observed rotation, the path length and the concentration of its solution.

$$
[\alpha] = \frac{\alpha}{l\,c}\quad (l \text{ in dm},\ c \text{ in g/mL})
$$

| input | meaning | condition |
|---|---|---|
| `alpha` | observed rotation (degrees) |  |
| `l` | path length of the cell |  |
| `c` | concentration in mass per volume (g/mL) |  |

| output | meaning |
|---|---|
| `specific_rotation` | specific rotation (degrees mL $g^{-1}$ dm^-1) |

- **Method:** observed rotation over path length (dm) times concentration (g/mL)
- **Assumes:** l, $c > 0$; rotation in degrees, written with °
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Specific rotation as α runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/specific-rotation-live-1)

*Specific rotation as α runs from 0.3 to 2.1, every other input as you entered it (41 solutions). Marked: your α = 1.2 gives specific_rotation = 24. Your inputs: alpha = 1.2, l = 0.1 m, c = 50 kg/m^3.*

![Specific rotation as l runs from 0](https://www.ai999x.com/?ax_fig=chemistry-ai999x/specific-rotation-live-2)

*Specific rotation as l runs from 0.06 to 0.14 m, every other input as you entered it (29 solutions). Marked: your l = 0.1 m gives specific_rotation = 24. Your inputs: alpha = 1.2, l = 0.1 m, c = 50 kg/m^3.*

```python
chemistry_ai999x.solve('specific_rotation', alpha=1.2, l=0.1, c=50.0)
# -> specific_rotation = 24   (SOLVED)
```

```python
chemistry_ai999x.solve('specific_rotation', alpha=-4.32, l=0.2, c=120.0)
# -> specific_rotation = -18   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A solution of 0.0500 g/mL of a compound in a 1.00 dm tube shows an observed rotation of +1.20°. What is its specific rotation? | SOLVED | specific_rotation = 24 |
| The observed rotation is -4.32° for a 0.120 g/mL solution in a 2.00 dm cell. Find the specific rotation. | SOLVED | specific_rotation = -18 |
| The observed rotation is -4.32° for a 0.120 g/mL solution in a 0 dm cell. Find the specific rotation. | INVALID_INPUT | Specific rotation from an observed rotation: l must be a positive number. |

<a id="enantiomeric_excess_from_rotation"></a>

### Enantiomeric excess from optical rotation: `enantiomeric_excess_from_rotation`

The enantiomeric excess (optical purity) of a mixture and the percent of each enantiomer, from its specific rotation and that of the pure enantiomer.

$$
ee = \frac{[\alpha]_{mixture}}{[\alpha]_{pure}} \times 100\%,\quad \%_{major} = 50 + \frac{ee}{2}
$$

| input | meaning | condition |
|---|---|---|
| `alpha_obs` | specific rotation of the mixture (degrees) |  |
| `alpha_pure` | specific rotation of the pure enantiomer (degrees) |  |

| output | meaning |
|---|---|
| `ee` | enantiomeric excess (%) |
| `major_percent` | percent of the major enantiomer |
| `minor_percent` | percent of the minor enantiomer |
| `predominant` | which enantiomer predominates |

- **Method:** optical purity = ratio of the specific rotations; the major enantiomer is 50% + $ee/2$
- **Assumes:** the pure enantiomer's rotation is not zero; |observed| <= |pure|
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Enantiomeric excess as α_pure runs from 12](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enantiomeric-excess-from-rotation-live-1)

*Enantiomeric excess as α_pure runs from 12.3 to 42, every other input as you entered it (34 solutions). Marked: your α_pure = 24 gives ee = 50. Your inputs: alpha_obs = 12, alpha_pure = 24.*

![Enantiomeric excess as α_obs runs from 3 to 21, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enantiomeric-excess-from-rotation-live-2)

*Enantiomeric excess as α_obs runs from 3 to 21, every other input as you entered it (29 solutions). Marked: your α_obs = 12 gives ee = 50. Your inputs: alpha_obs = 12, alpha_pure = 24.*

```python
chemistry_ai999x.solve('enantiomeric_excess_from_rotation', alpha_obs=12.0, alpha_pure=24.0)
# -> ee = 50, major_percent = 75, minor_percent = 25, predominant = the same enantiomer as the pure sample   (SOLVED)
```

```python
chemistry_ai999x.solve('enantiomeric_excess_from_rotation', alpha_obs=-6.0, alpha_pure=24.0)
# -> ee = 25, major_percent = 62.5, minor_percent = 37.5, predominant = the opposite enantiomer   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| The observed specific rotation of a mixture is +12.0° and the pure enantiomer has a specific rotation of +24.0°. What is the enantiomeric excess? | SOLVED | ee = 50, major_percent = 75, minor_percent = 25, predominant = the same enantiomer as the pure sample |
| The observed rotation of a sample is -6.0° and the pure enantiomer has a specific rotation of +24.0°. What is the optical purity (enantiomeric excess)? | SOLVED | ee = 25, major_percent = 62.5, minor_percent = 37.5, predominant = the opposite enantiomer |
| The observed specific rotation of a mixture is +30.0° and the pure enantiomer has a specific rotation of +24.0°. What is the enantiomeric excess? | INVALID_INPUT | Enantiomeric excess from optical rotation: the observed rotation cannot be larger in size than the pure enantiomer's. |

<a id="enantiomeric_excess_from_composition"></a>

### Enantiomeric excess from the composition of a mixture, `enantiomeric_excess_from_composition`

The enantiomeric excess and the percent of each enantiomer from the amounts of the R and S enantiomers.

$$
ee = \frac{|n_R - n_S|}{n_R + n_S} \times 100\%
$$

| input | meaning | condition |
|---|---|---|
| `R` | amount of the R enantiomer (any unit, the same for both) |  |
| `S` | amount of the S enantiomer (same unit) |  |

| output | meaning |
|---|---|
| `ee` | enantiomeric excess (%) |
| `percent_R` | percent R |
| `percent_S` | percent S |
| `predominant` | which enantiomer predominates |

- **Method:** difference of the two amounts over their sum
- **Assumes:** amounts >= 0, not both zero; one unit for both
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Enantiomeric excess as R runs from 18](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enantiomeric-excess-from-composition-live-1)

*Enantiomeric excess as R runs from 18.75 to 131.2, every other input as you entered it (41 solutions). Marked: your R = 75 gives ee = 50. Shaded by which enantiomer predominates: S, R. Your inputs: R = 75, S = 25.*

![Which enantiomer predominates for 25 5 x 5 combinations of R and S around your inputs (the other inputs as entered); the ring marks your inputs, which give R](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enantiomeric-excess-from-composition-live-2)

*Which enantiomer predominates for 25 5 x 5 combinations of R and S around your inputs (the other inputs as entered); the ring marks your inputs, which give R. Blank cells admit no answer. Your inputs: R = 75, S = 25.*

![Enantiomeric excess as S runs from 6](https://www.ai999x.com/?ax_fig=chemistry-ai999x/enantiomeric-excess-from-composition-live-3)

*Enantiomeric excess as S runs from 6.25 to 43.75, every other input as you entered it (29 solutions). Marked: your S = 25 gives ee = 50. Your inputs: R = 75, S = 25.*

```python
chemistry_ai999x.solve('enantiomeric_excess_from_composition', R=75.0, S=25.0)
# -> ee = 50, percent_R = 75, percent_S = 25, predominant = R   (SOLVED)
```

```python
chemistry_ai999x.solve('enantiomeric_excess_from_composition', R=1.0, S=3.0)
# -> ee = 50, percent_R = 25, percent_S = 75, predominant = S   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| A mixture contains 75% of the R enantiomer and 25% of the S enantiomer. What is the enantiomeric excess? | SOLVED | ee = 50, percent_R = 75, percent_S = 25, predominant = R |
| A sample contains 1.0 g of the (R)-enantiomer and 3.0 g of the (S)-enantiomer. Find the enantiomeric excess. | SOLVED | ee = 50, percent_R = 25, percent_S = 75, predominant = S |
| A sample contains -1.0 g of the (R)-enantiomer and 3.0 g of the (S)-enantiomer. Find the enantiomeric excess. | INVALID_INPUT | Enantiomeric excess from the composition of a mixture: amounts cannot be negative. |
