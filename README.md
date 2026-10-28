# chemistry-ai999x — offline, verified chemistry solver with step-by-step answers

[![tests](https://github.com/AI999X/chemistry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemistry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemistry-ai999x)](https://pypi.org/project/chemistry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemistry-ai999x)

**[Install](https://pypi.org/project/chemistry-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/chemistry-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/chemistry-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Physical Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

chemistry-ai999x is an offline, deterministic Python solver that answers chemistry questions with verified results from 103 certified capabilities, gives 377 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**chemistry-ai999x** is a compact, deterministic chemistry engine for Python. It reads a question in plain English (or a typed call), picks a certified chemistry capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified chemistry capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 377 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** stoichiometry, gas laws, solutions, chemical equilibrium, acids and bases, kinetics, thermochemistry, electrochemistry, molecular properties and spectroscopy.

## Model overview

| Property | Value |
|---|---|
| Package | `chemistry-ai999x` 1.0.3 (import `chemistry_ai999x`) |
| Type | Deterministic neuro-symbolic chemistry engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 103 certified procedures |
| Knowledge pack | version 1.0.1 |
| Golden answers | 377 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install chemistry-ai999x
```

```python
import chemistry_ai999x as chemistry

chemistry.ask("What is the pH of 0.10 M acetic acid (Ka = 1.8 \u00d7 10^-5)?").show()
# -> SOLVED — pH = 2.87528, h = 1.33267, percent_ionized = 1.33267  (ph_weak_acid)
chemistry.ask("Balance the equation C3H8 + O2 -> CO2 + H2O.").show()
# -> SOLVED — coefficients = [1.0, 5.0, 3.0, 4.0], balanced = C3H8 + 5 O2 -> 3 CO2 + 4 H2O  (balance_equation)
chemistry.ask("What is the molar mass of CuSO4\u00b75H2O?").show()
# -> SOLVED — M = 0.249684, M_g = 249.684  (molar_mass)
chemistry.solve("ideal_gas_pressure", n="2.0 mol", V="10 L", T="300 K").show()
# -> SOLVED — P = 498868, P_atm = 4.92344  (ideal_gas_pressure)
chemistry.ask("A reaction has k = 1.2e-3 s^-1 at 300 K and Ea = 75.0 kJ/mol. What is k at 350 K?").show()
# -> SOLVED — k2 = 0.0880375, ratio = 73.3646  (arrhenius_k2)
chemistry.ask("Who created you?").message
# -> chemistry-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `chemistry-ai999x "<question>"`, `chemistry-ai999x --capabilities`. The outputs above were produced by running this exact code on chemistry-ai999x 1.0.3.

## Find the right capability, and draw it

`chemistry.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "chemistry-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import chemistry_ai999x as chemistry

chemistry.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `chemistry-ai999x`; the answer is the one it returned, with the steps it took.

**pH of a weak acid** · _Acids and bases_

> What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?

→ **pH = 2.87527706, h = 1.332670977 mol/m^3, percent_ionized = 1.332670969**

<details><summary>Steps</summary>

- pH of a weak acid: Ka = 1.8e-05, c = 100 mol/m^3 → pH = 2.87527706, h = 1.332670977 mol/m^3, percent_ionized = 1.332670969

</details>

**Balancing a chemical equation** · _Stoichiometry_

> Balance the equation C3H8 + O2 -> CO2 + H2O.

→ **coefficients = [1, 5, 3, 4], balanced = C3H8 + 5 O2 -> 3 CO2 + 4 H2O**

<details><summary>Steps</summary>

- Balancing a chemical equation: equation = C3H8 + O2 -> CO2 + H2O → coefficients = [1, 5, 3, 4], balanced = C3H8 + 5 O2 -> 3 CO2 + 4 H2O

</details>

**Molar mass of a formula** · _Stoichiometry_

> What is the molar mass of CuSO4·5H2O?

→ **M = 0.249684 kg/mol, M_g = 249.684**

<details><summary>Steps</summary>

- Molar mass of a formula: formula = CuSO4·5H2O → M = 0.249684 kg/mol, M_g = 249.684 kg/mol

</details>

**Ideal gas law: pressure** · _Gases_

`m.solve("ideal_gas_pressure", n="2.0 mol", V="10 L", T="300 K")`

→ **P = 498867.7571 Pa, P_atm = 4.923441965**

<details><summary>Steps</summary>

- Ideal gas law: pressure: n = 2 mol, V = 10 L, T = 300 K → P = 498867.7571 Pa, P_atm = 4.923441965 Pa

</details>

**Arrhenius equation: rate constant at another temperature** · _Kinetics_

> A reaction has k = 1.2e-3 s^-1 at 300 K and Ea = 75.0 kJ/mol. What is k at 350 K?

→ **k2 = 0.08803749544, ratio = 73.36457954**

<details><summary>Steps</summary>

- Arrhenius equation: rate constant at another temperature: k1 = 0.0012, T1 = 300 K, Ea = 75000 J/mol, T2 = 350 K → k2 = 0.08803749544, ratio = 73.36457954

</details>

**Gibbs energy from a cell potential** · _Electrochemistry_

> A cell has E° = 1.10 V and n = 2. What is ΔG°?

→ **dG = -212267.7307 J/mol, dG_kJ = -212.2677307**

<details><summary>Steps</summary>

- Gibbs energy from a cell potential: n = 2, E = 1.1 V → dG = -212267.7307 J/mol, dG_kJ = -212.2677307 J/mol

</details>

## Figures

Every point was computed by `chemistry-ai999x`.

**Titration curve with the buffer region**

![Titration curve with the buffer region — computed by chemistry-ai999x](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/titration-ph-weak-acid-strong-base-titration.svg)

_pH against volume of 0.100 M NaOH added to 25.0 mL of 0.100 M acid, computed by the package at 101 volumes for a strong acid and a weak acid (Ka = 1.8e-5). Shaded: the weak acid's buffer region, 10 % to 90 % neutralised (pH 3.79886 to 5.69915); at half-equivalence pH = 4.7452 (= pKa). Marked: equivalence V_b = 25 mL, pH = 8.7219._

**Reaction scheme A → B → C with its rate constants**

![Reaction scheme A → B → C with its rate constants — computed by chemistry-ai999x](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/consecutive-first-order-scheme.svg)

_The consecutive first-order reaction A → B → C with k1 = 0.1 1/s and k2 = 0.05 1/s, starting from A0 = 1000 mol/m³: the concentrations the package returns at t = 10 s are A = 367.879, B = 477.302 and C = 154.818 mol/m³._

**Species shares over time, stacked**

![Species shares over time, stacked — computed by chemistry-ai999x](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/consecutive-first-order-shares.svg)

_How the starting 1000 mol/m³ of A divides between A, B and C over time (stacked shares, k1 = 0.1 1/s, k2 = 0.05 1/s), each concentration computed by the package at 80 times. B is largest near t = 14 s (499.977 mol/m³, a share of 0.499977); at t = 80 s the shares are 0.000335463, 0.0359604 and 0.963704._

**Hydrogen line spectrum: Lyman, Balmer and Paschen series**

![Hydrogen line spectrum: Lyman, Balmer and Paschen series — computed by chemistry-ai999x](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/hydrogen-transition-wavelength-spectrum.svg)

_The hydrogen line spectrum: wavelength of the first six lines of three series, computed by the package (Rydberg formula), each drawn at its photon energy (also computed by the package). Ranges: Lyman 121.568–93.0758 nm; Balmer 656.47–389.019 nm; Paschen 1875.63–923.16 nm. Marked: Hα (n = 3 → 2), λ = 656.47 nm, E = 1.88865 eV._

**Concentration against time: zero, first and second order**

![Concentration against time: zero, first and second order — computed by chemistry-ai999x](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/first-order-concentration-kinetics.svg)

_Reactant concentration against time from A0 = 500 mol/m³ for zero-, first- and second-order rate laws with the same initial rate (2 mol/(m³·s)), each computed by the package at up to 121 times. At t = 200 s: 100, 224.664 and 277.778 mol/m³; marked: t = 200 s, [A] = 224.664 mol/m³ (first order)._

**Arrhenius plot: rate constant against inverse temperature**

![Arrhenius plot: rate constant against inverse temperature — computed by chemistry-ai999x](https://huggingface.co/AI999X/chemistry-ai999x/resolve/main/figures/arrhenius-rate-constant-arrhenius.svg)

_Arrhenius plot: the rate constant (logarithmic axis) against 1000/T for A = 1.0000e+13 1/s and Ea = 100 kJ/mol, computed by the package at 61 temperatures from 350 K to 650 K — a straight line of slope −Ea/R. Marked: T = 500 K, k = 357.5 1/s._

## Capabilities

103 certified procedures. A few of them:

- **Procedures:** Activation energy from two rate constants; Arrhenius equation: rate constant; Arrhenius equation: rate constant at another temperature; Atoms of an element in a mass of compound; Balancing a chemical equation; Beer-Lambert law: absorbance; Beer-Lambert law: concentration; Boiling-point elevation; Boyle's law: final pressure; Boyle's law: final volume; Buffer pH (Henderson-Hasselbalch); Charles's law: final volume; Clausius-Clapeyron: enthalpy of vaporization; Clausius-Clapeyron: vapor pressure at another temperature; Combined gas law: final pressure; Combined gas law: final temperature; Combined gas law: final volume; Consecutive first-order reactions (A -> B -> C); Degree of unsaturation; Density of an ideal gas; Dilution: final concentration; Dilution: volume of stock needed; Empirical formula from percent composition; Energy of a photon from its wavelength …

`chemistry_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified chemistry capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (stoichiometry, gas laws, solutions, chemical equilibrium, acids and bases, kinetics, thermochemistry, electrochemistry, molecular properties and spectroscopy); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{chemistry_ai999x_1_0_3,
  title  = {chemistry-ai999x: an offline, verified chemistry solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
  url    = {https://huggingface.co/AI999X/chemistry-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/chemistry-ai999x/discussions or visit https://ai999x.com.

## Support the project

chemistry-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemistry-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/chemistry-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/chemistry-ai999x/tree/main/onnx).
