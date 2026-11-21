# Chemistry (chemistry-ai999x)

[![tests](https://github.com/AI999X/chemistry-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/chemistry-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/chemistry-ai999x)](https://pypi.org/project/chemistry-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/chemistry-ai999x)

Chemistry covers stoichiometry, gas laws, solutions, chemical equilibrium, acids and bases, kinetics, thermochemistry, electrochemistry, molecular properties and spectroscopy: 119 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Chemistry desktop app: pH of a weak acid, with its worked example and live scene](https://www.ai999x.com/?ax_shot=chemistry-ai999x/hero.webp)](https://www.ai999x.com/models/chemistry-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-chemistry-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-chemistry-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `chemistry` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
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

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/chemistry-ai999x/blob/main/docs/chemistry_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Chemistry on ai999x.com](https://www.ai999x.com/models/chemistry-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/chemistry-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/chemistry-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `chemistry` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/chemistry`) and its data (`rust/models/chemistry/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p chemistry-ai999x
./target/release/chemistry --help
cargo test -p chemistry-ai999x
```
