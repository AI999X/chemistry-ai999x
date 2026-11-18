# Chemistry: the desktop app, user guide

**Chemistry**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Chemistry covers stoichiometry, gas laws, solutions, chemical equilibrium, acids and bases, kinetics, thermochemistry, electrochemistry, molecular properties and spectroscopy. Its desktop app holds all 119 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemistry-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemistry-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-chemistry-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-chemistry.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-chemistry-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-chemistry-1.6.5-linux-x86_64.tar.gz
./ai999x-chemistry-1.6.5-linux-x86_64/ai999x-chemistry
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Chemistry app, opened on pH of a weak acid with its worked example.](https://www.ai999x.com/?ax_shot=chemistry-ai999x/hero.webp)

*The Chemistry app, opened on pH of a weak acid with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Acids and bases › pH of a weak acid (or type part of its name in the search box). It opens with a worked example filled in: “What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?” The answer: pH = 2.87527706, h = 1.332670977 mol/m^3, percent_ionized = 1.332670969.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![pH of a weak acid: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-1.webp)

*pH of a weak acid: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: pH of a weak acid
pH = 2.87527706
h = 1.332670977 mol/m^3
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Balancing a chemical equation](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-2.webp)

*Balancing a chemical equation*

![Ideal gas law: pressure](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-3.webp)

*Ideal gas law: pressure*

![Arrhenius equation: rate constant at another temperature](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-4.webp)

*Arrhenius equation: rate constant at another temperature*

![Average atomic mass from isotopes](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-5.webp)

*Average atomic mass from isotopes*

![Gibbs energy from a cell potential](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-6.webp)

*Gibbs energy from a cell potential*

![Molar solubility with a common ion](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-7.webp)

*Molar solubility with a common ion*

![Degree of unsaturation](https://www.ai999x.com/?ax_shot=chemistry-ai999x/scene-8.webp)

*Degree of unsaturation*

![pH of a weak acid: a chart, enlarged.](https://www.ai999x.com/?ax_shot=chemistry-ai999x/chart-1.webp)

*pH of a weak acid: a chart, enlarged.*

![Balancing a chemical equation: a chart, enlarged.](https://www.ai999x.com/?ax_shot=chemistry-ai999x/chart-2.webp)

*Balancing a chemical equation: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Chemistry app in the dark theme.](https://www.ai999x.com/?ax_shot=chemistry-ai999x/hero-dark.webp)

*The Chemistry app in the dark theme.*

## 7. Every calculation

119 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/chemistry-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Acids and bases | Buffer pH (Henderson-Hasselbalch), Conjugate acid-base constants (Ka Kb = Kw), Hydronium concentration from pH, Ka from the pH of a weak acid, pH during a strong acid - strong base titration, pH during a weak acid - strong base titration, pH from the hydronium concentration, pH from the hydroxide concentration, pH of a diprotic acid, pH of a salt of a weak acid, pH of a strong base, pH of a strong monoprotic acid, pH of a weak acid, pH of a weak base, Titration: equivalence-point volume |
| Atomic structure | Abundances of two isotopes from the average atomic mass, Average atomic mass from isotopes |
| Electrochemistry | Equilibrium constant from a standard cell potential, Gibbs energy from a cell potential, Mass deposited in electrolysis (Faraday's law), Nernst equation, Standard cell potential from two half-reactions, Time for electrolysis (Faraday's law) |
| Equilibrium | Equilibrium concentrations (ICE table), Equilibrium constant from the standard Gibbs energy, Gibbs energy change at non-standard conditions, Kc from Kp, Kp from Kc, Ksp from molar solubility, Molar solubility from Ksp, Molar solubility with a common ion, Precipitation test (Q versus Ksp), Reaction quotient and the direction of shift, Standard Gibbs energy from the equilibrium constant, Van 't Hoff equation: K at another temperature |
| Gases | Boyle's law: final pressure, Boyle's law: final volume, Charles's law: final volume, Combined gas law: final pressure, Combined gas law: final temperature, Combined gas law: final volume, Density of an ideal gas, Gay-Lussac's law: final pressure, Graham's law: molar mass of an unknown gas, Graham's law: ratio of effusion rates, Ideal gas law: amount, Ideal gas law: pressure, Ideal gas law: temperature, Ideal gas law: volume, Mass of a gas sample (ideal gas), Mean speed of gas molecules, Molar mass from a gas sample, Molar mass from gas density, Partial pressure (Dalton's law), Partial pressures in a gas mixture, Root-mean-square speed of gas molecules, Van der Waals equation: pressure |
| Kinetics | Activation energy from two rate constants, Arrhenius equation: rate constant, Arrhenius equation: rate constant at another temperature, Consecutive first-order reactions (A -> B -> C), First-order integrated rate law, Half-life of a first-order reaction, Reaction order from initial rates, Second-order integrated rate law, Time to reach a concentration (first order), Zero-order integrated rate law |
| Molecular properties | Degree of unsaturation |
| Nuclear chemistry | Age from the fraction remaining (radioactive dating), Amount remaining after radioactive decay (half-life), Mass defect and nuclear binding energy |
| Organic chemistry | Enantiomeric excess from optical rotation, Enantiomeric excess from the composition of a mixture, Specific rotation from an observed rotation |
| Solutions | Boiling-point elevation, Dilution: final concentration, Dilution: volume of stock needed, Freezing-point depression, Henry's law: solubility of a gas, Mass of solute for a solution, Molality of a solution, Molar mass from freezing-point depression, Molar mass from osmotic pressure, Molarity from a dissolved mass, Mole fraction in a solution, Osmotic pressure, Vapor pressure of a solution (Raoult's law) |
| Spectroscopy | Beer-Lambert law: absorbance, Beer-Lambert law: concentration, Energy of a photon from its wavelength, Hydrogen atom transition wavelength (Rydberg), Vibrational wavenumber of a diatomic molecule, Wavelength of a photon from its energy |
| Stoichiometry | Atoms of an element in a mass of compound, Balancing a chemical equation, Empirical formula from percent composition, Limiting reagent and theoretical yield, Mass from moles, Mass of an element in a sample, Mass percent of an element, Molar mass of a formula, Molecular formula from the empirical formula, Moles from a mass, Moles from a number of particles, Number of molecules in a mass, Number of particles in an amount, Percent yield of a reaction, Theoretical yield from one reactant |
| Thermochemistry | Clausius-Clapeyron: enthalpy of vaporization, Clausius-Clapeyron: vapor pressure at another temperature, Gibbs energy change (dG = dH - T dS), Heat from a temperature change, Heat of reaction for a given mass, Reaction enthalpy from bond enthalpies, Reaction enthalpy from coffee-cup calorimetry, Reaction enthalpy from enthalpies of formation (Hess's law), Standard reaction entropy from standard molar entropies, Standard reaction Gibbs energy from Gibbs energies of formation, Temperature at which dG = 0 |

## 8. Other ways to use it

The same engine runs on the command line (`chemistry`, its own guide), in Python (`pip install chemistry-ai999x`, `import chemistry_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/chemistry-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/chemistry-ai999x/
