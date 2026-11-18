# Chemistry (chemistry-ai999x)

[![tests](https://github.com/AI999X/chemistry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemistry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemistry-ai999x)](https://pypi.org/project/chemistry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemistry-ai999x)

Chemistry covers stoichiometry, gas laws, solutions, chemical equilibrium, acids and bases, kinetics, thermochemistry, electrochemistry, molecular properties and spectroscopy: 119 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Chemistry desktop app: pH of a weak acid, with its worked example and live scene](https://www.ai999x.com/?ax_shot=chemistry-ai999x/hero.webp)](https://www.ai999x.com/models/chemistry-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemistry-1.6.5-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemistry-1.6.5-linux-x86_64.tar.gz) |
| **Command line** `chemistry` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/chemistry-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install chemistry-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Chemistry desktop app: every scene and chart is drawn from the computed answer.

[![Chemistry desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=chemistry-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=chemistry-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=chemistry-ai999x/clip.mp4)*

| | |
|---|---|
| [![pH of a weak acid: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-1.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-ph_weak_acid)<br>pH of a weak acid | [![Balancing a chemical equation: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-2.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-balance_equation)<br>Balancing a chemical equation |
| [![Ideal gas law: pressure: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-3.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-ideal_gas_pressure)<br>Ideal gas law: pressure | [![Arrhenius equation: rate constant at another temperature: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-4.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-arrhenius_k2)<br>Arrhenius equation: rate constant at another temperature |
| [![Average atomic mass from isotopes: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-5.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-average_atomic_mass)<br>Average atomic mass from isotopes | [![Gibbs energy from a cell potential: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-6.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-delta_g_from_cell_potential)<br>Gibbs energy from a cell potential |
| [![Molar solubility with a common ion: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-7.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-common_ion_solubility)<br>Molar solubility with a common ion | [![Degree of unsaturation: the live scene in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-8.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-degree_of_unsaturation)<br>Degree of unsaturation |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![pH of a weak acid: a chart in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/chart-1.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-ph_weak_acid)<br>pH of a weak acid | [![Balancing a chemical equation: a chart in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/chart-2.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-balance_equation)<br>Balancing a chemical equation | [![Ideal gas law: pressure: a chart in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/chart-3.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-ideal_gas_pressure)<br>Ideal gas law: pressure | [![Arrhenius equation: rate constant at another temperature: a chart in the Chemistry desktop app](https://www.ai999x.com/?ax_shot=chemistry-ai999x/chart-4.webp)](https://www.ai999x.com/models/chemistry-ai999x/#cap-arrhenius_k2)<br>Arrhenius equation: rate constant at another temperature |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import chemistry_ai999x as chemistry

a = chemistry.ask(
    "What is the pH of a buffer that is 0.10 M acetic acid and 0.15 M "
    "sodium acetate (pKa = 4.76)?"
)
print(a.outcome, a.values)
# SOLVED {'pH': 4.936091259055681}

a = chemistry.ask(
    "A cell has E° = 0.46 V with n = 2 at 298 K. What is the "
    "equilibrium constant?"
)
print(a.outcome, a.values)
# SOLVED {'lnK': 35.8260291000875, 'K': 3622819916970735.0}

a = chemistry.ask(
    "A gas at 1.00 atm occupies 4.00 L. What is the new pressure if it "
    "is compressed to 1.00 L at constant temperature?"
)
print(a.outcome, a.values)
# SOLVED {'P2': 405300.0}

a = chemistry.ask(
    "k1 = 1.0e-3 s^-1 at T1 = 300 K and k2 = 5.0e-3 s^-1 at T2 = 320 "
    "K. What is the activation energy?"
)
print(a.outcome, a.values)
# SOLVED {'Ea': 64231.73452402517, 'Ea_kJ': 64.23173452402517}

a = chemistry.ask("What is the degree of unsaturation of C6H6?")
print(a.outcome, a.values)
# SOLVED {'dou': 4.0}

a = chemistry.ask(
    "A wood sample has 25% of the original carbon-14 remaining. The "
    "half-life of carbon-14 is 5730 years. How old is the sample?"
)
print(a.outcome, a.values)
# SOLVED {'t': 361650096000.0, 't_years': 11460.0, 'half_lives': 2.0}

a = chemistry.ask(
    "What is the boiling point change of a 0.50 mol/kg aqueous "
    "solution of a nonelectrolyte (Kb = 0.512 K·kg/mol)?"
)
print(a.outcome, a.values)
# SOLVED {'dT': 0.256}

a = chemistry.ask(
    "A 1.00 cm cell holds a 2.0e-5 M solution with molar absorptivity "
    "15000 L/(mol·cm). What is the absorbance?"
)
print(a.outcome, a.values)
# SOLVED {'A': 0.3, 'T_percent': 50.11872336272722}

a = chemistry.ask(
    "P1 = 100 torr at T1 = 300 K and P2 = 400 torr at T2 = 340 K. What "
    "is the enthalpy of vaporization?"
)
print(a.outcome, a.values)
# SOLVED {'dH': 29392.046240384272}

a = chemistry.ask(
    "Isotope masses are 34.97 u and 36.97 u with abundances 75.78% and "
    "24.22%. What is the average atomic mass?"
)
print(a.outcome, a.values)
# SOLVED {'A_r': 35.4544, 'M': 0.0354544}

a = chemistry.ask("A cell has E° = 1.10 V and n = 2. What is ΔG°?")
print(a.outcome, a.values)
# SOLVED {'dG': -212267.73067128204, 'dG_kJ': -212.26773067128204}

a = chemistry.ask(
    "ΔG° = -33.0 kJ/mol at 298 K. What is the equilibrium constant?"
)
print(a.outcome, a.values)
# SOLVED {'K': 608499.1084253575, 'lnK': 13.318750726207918}

a = chemistry.ask(
    "A gas occupies 2.00 L at 1.00 atm. What is its volume when "
    "compressed to 3.00 atm at constant temperature?"
)
print(a.outcome, a.values)
# SOLVED {'V2': 0.0006666666666666666}

a = chemistry.ask(
    "A reaction has a pre-exponential factor of 1.0e13 s^-1 and Ea = "
    "100 kJ/mol. What is the rate constant at 500 K?"
)
print(a.outcome, a.values)
# SOLVED {'k': 357.4999421719933}

a = chemistry.ask(
    "25.0 mL of 6.0 M HCl is diluted to 500 mL. What is the new "
    "concentration?"
)
print(a.outcome, a.values)
# SOLVED {'c2': 300.0, 'c2_molL': 0.3}
```

## Command line: 15 examples

Each command below was run with `chemistry` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ chemistry solve buffer_ph Ka=1.737800829e-05 "c_base=150 mol/m^3" "c_acid=100 mol/m^3"
pH = 4.936091259

$ chemistry solve isotope_abundance m1=34.97 m2=36.97 A_r=35.45
x1 = 0.76
x2 = 0.24
percent1 = 76
percent2 = 24

$ chemistry solve k_from_cell_potential n=2 "E=0.46 V" "T=298 K"
lnK = 35.8260291
K = 3.622819917e+15

$ chemistry solve equilibrium_ice "equation=N2O4 ⇌ 2 NO2" "initial=[0.1, 0]" K=0.0046
x = 0.0101642097
equilibrium = [0.08983579029909557,0.020328419401808878]

$ chemistry solve boyle_final_pressure "V2=0.001 m^3" "P1=101325 Pa" "V1=0.004 m^3"
P2 = 405300 Pa

$ chemistry solve activation_energy k1=0.001 "T1=300 K" k2=0.005 "T2=320 K"
Ea = 64231.73452 J/mol
Ea_kJ = 64.23173452

$ chemistry solve degree_of_unsaturation formula=C6H6
dou = 4

$ chemistry solve radioactive_dating_age fraction=0.25 "t_half=1.80825048e+11 s"
t = 3.61650096e+11 s
t_years = 11460
half_lives = 2

$ chemistry solve enantiomeric_excess_from_rotation alpha_obs=12 alpha_pure=24
ee = 50
major_percent = 75
minor_percent = 25
predominant = the same enantiomer as the pure sample

$ chemistry solve boiling_point_elevation "K=0.512 K*kg/mol" "b=0.5 mol/kg" i=1
dT = 0.256 K

$ chemistry solve beer_lambert_absorbance "l=0.01 m" "c=0.02 mol/m^3" "eps=1500 m^2/mol"
A = 0.3
T_percent = 50.11872336

$ chemistry solve atoms_of_element element=O formula=CO2 "m=0.01 kg"
N_atoms = 2.736776914e+23
n_element = 0.4544524984 mol

$ chemistry solve clausius_clapeyron_enthalpy "P1=13332.23684 Pa" "T1=300 K" "P2=53328.94737 Pa" "T2=340 K"
dH = 29392.04624 J/mol

$ chemistry solve conjugate_dissociation_constant K=1.8e-05 known=Ka
K_conj = 5.555555556e-10
pK = 4.744727495
pK_conj = 9.255272505
conjugate = Kb of the conjugate base

$ chemistry solve average_atomic_mass "masses=[34.97, 36.97]" "abundances=[0.7578, 0.2422]"
A_r = 35.4544
M = 0.0354544 kg/mol
```

## Rust library: 15 examples

The crate `chemistry-ai999x` (in `rust/` of the GitHub repository) answers a JSON request with `chemistry_ai999x::solve_json`. Each call below was compiled and run on this release; the comment under it is what it printed. The whole program is `rust/chemistry/examples/readme.rs`: `cd rust && cargo run --release --example readme -p chemistry-ai999x`.

```rust
use serde_json::{json, Value};

fn show(request: Value) {
    let answer: Value = serde_json::from_str(&chemistry_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

show(json!({"key": "buffer_ph", "inputs": {"Ka": "1.737800829e-05", "c_base": "150 mol/m^3", "c_acid": "100 mol/m^3"}}));
// SOLVED
// pH = 4.936091258993048

show(json!({"key": "isotope_abundance", "inputs": {"m1": "34.97", "m2": "36.97", "A_r": "35.45"}}));
// SOLVED
// x1 = 0.759999999999998
// x2 = 0.240000000000002
// percent1 = 75.9999999999998
// percent2 = 24.0000000000002

show(json!({"key": "k_from_cell_potential", "inputs": {"n": "2", "E": "0.46 V", "T": "298 K"}}));
// SOLVED
// lnK = 35.8260291000875
// K = 3622819916970735.0

show(json!({"key": "equilibrium_ice", "inputs": {"equation": "N2O4 ⇌ 2 NO2", "initial": [0.1, 0], "K": "0.0046"}}));
// SOLVED
// x = 0.010164209700904439
// equilibrium = [0.08983579029909557,0.020328419401808878]

show(json!({"key": "boyle_final_pressure", "inputs": {"V2": "0.001 m^3", "P1": "101325 Pa", "V1": "0.004 m^3"}}));
// SOLVED
// P2 = 405300.0 Pa

show(json!({"key": "activation_energy", "inputs": {"k1": "0.001", "T1": "300 K", "k2": "0.005", "T2": "320 K"}}));
// SOLVED
// Ea = 64231.73452402517 J/mol
// Ea_kJ = 64.23173452402517

show(json!({"key": "degree_of_unsaturation", "inputs": {"formula": "C6H6"}}));
// SOLVED
// dou = 4.0

show(json!({"key": "radioactive_dating_age", "inputs": {"fraction": "0.25", "t_half": "1.80825048e+11 s"}}));
// SOLVED
// t = 361650096000.0 s
// t_years = 11460.0
// half_lives = 2.0

show(json!({"key": "enantiomeric_excess_from_rotation", "inputs": {"alpha_obs": "12", "alpha_pure": "24"}}));
// SOLVED
// ee = 50.0
// major_percent = 75.0
// minor_percent = 25.0
// predominant = "the same enantiomer as the pure sample"

show(json!({"key": "boiling_point_elevation", "inputs": {"K": "0.512 K*kg/mol", "b": "0.5 mol/kg", "i": "1"}}));
// SOLVED
// dT = 0.256 K

show(json!({"key": "beer_lambert_absorbance", "inputs": {"l": "0.01 m", "c": "0.02 mol/m^3", "eps": "1500 m^2/mol"}}));
// SOLVED
// A = 0.3
// T_percent = 50.11872336272722

show(json!({"key": "atoms_of_element", "inputs": {"element": "O", "formula": "CO2", "m": "0.01 kg"}}));
// SOLVED
// N_atoms = 2.7367769138130837e+23
// n_element = 0.4544524983526097 mol

show(json!({"key": "clausius_clapeyron_enthalpy", "inputs": {"P1": "13332.23684 Pa", "T1": "300 K", "P2": "53328.94737 Pa", "T2": "340 K"}}));
// SOLVED
// dH = 29392.046244359954 J/mol

show(json!({"key": "conjugate_dissociation_constant", "inputs": {"K": "1.8e-05", "known": "Ka"}}));
// SOLVED
// K_conj = 5.555555555555555e-10
// pK = 4.7447274948966935
// pK_conj = 9.255272505103306
// conjugate = "Kb of the conjugate base"

show(json!({"key": "average_atomic_mass", "inputs": {"masses": [34.97, 36.97], "abundances": [0.7578, 0.2422]}}));
// SOLVED
// A_r = 35.4544
// M = 0.0354544 kg/mol
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Chemistry on ai999x.com](https://www.ai999x.com/models/chemistry-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemistry-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/chemistry-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `chemistry` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/chemistry`) and its data (`rust/models/chemistry/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p chemistry-ai999x
./target/release/chemistry --help
cargo test -p chemistry-ai999x
```
