# chemistry: the Chemistry program — user guide

**chemistry-ai999x 1.0.3**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Chemistry covers stoichiometry, gas laws, solutions, chemical equilibrium, acids and bases, kinetics, thermochemistry, electrochemistry, molecular properties and spectroscopy. This guide is about `chemistry`, the model as a self-contained program for Windows and Linux: 103 capabilities in 10 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/chemistry-ai999x/); the Python package (`pip install chemistry-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.4.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS; the newest version is always under https://www.ai999x.com/downloads/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Chemistry | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-chemistry-1.4.0-windows-x86_64.zip |
| Desktop app: Chemistry | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-chemistry-1.4.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz |

### Desktop app

Chemistry in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-chemistry-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-chemistry.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-chemistry-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-chemistry-1.4.0-linux-x86_64.tar.gz
./ai999x-chemistry-1.4.0-linux-x86_64/ai999x-chemistry
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.4.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.4.0-linux-x86_64.tar.gz
./ai999x-1.4.0-linux-x86_64/ai999x
```

### Command line — Linux

The bundle holds one program per model; `chemistry` is this one. The checksum line should print OK.

```shell
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
chemistry --version
```

### Command line — Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
chemistry --version
```

### In your browser

Open https://www.ai999x.com/models/chemistry-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install chemistry-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ chemistry --version
chemistry-ai999x 1.0.3 (Rust build, ax-engine 0.1.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ chemistry ask "What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?"
Read as: pH of a weak acid
pH = 2.87527706
h = 1.332670977 mol/m^3
percent_ionized = 1.332670969
  by: pH of a weak acid  (exact charge balance (no 5% approximation), solved by bisection)
  Ka = 1.8e-05
  c = 100 mol/m^3
  assumes: monoprotic acid; 25 °C
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ chemistry search "ph of a weak acid"
ph_weak_acid                         pH of a weak acid  (acids and bases / weak acids)
ka_from_ph                           Ka from the pH of a weak acid  (acids and bases / weak acids)
ph_salt_of_weak_acid                 pH of a salt of a weak acid  (acids and bases / salt hydrolysis)
titration_ph_weak_acid_strong_base   pH during a weak acid - strong base titration  (acids and bases / titrations)
ph_weak_base                         pH of a weak base  (acids and bases / weak bases)
ph_diprotic_acid                     pH of a diprotic acid  (acids and bases / polyprotic acids)
ph_strong_acid                       pH of a strong monoprotic acid  (acids and bases / strong acids)
titration_ph_strong_acid_strong_base pH during a strong acid - strong base titration  (acids and bases / titrations)
… 6 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ chemistry list
activation_energy                        Activation energy from two rate constants
arrhenius_k2                             Arrhenius equation: rate constant at another temperature
arrhenius_rate_constant                  Arrhenius equation: rate constant
atoms_of_element                         Atoms of an element in a mass of compound
balance_equation                         Balancing a chemical equation
beer_lambert_absorbance                  Beer-Lambert law: absorbance
beer_lambert_concentration               Beer-Lambert law: concentration
boiling_point_elevation                  Boiling-point elevation
… 95 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ chemistry info ph_weak_acid
pH of a weak acid
  exact charge balance (no 5% approximation), solved by bisection
  in   Ka       acid dissociation constant []
  in   c        acid concentration [mol/m^3]
  out  pH       pH []
  out  h        [H3O+] [mol/m^3]
  out  percent_ionized percent ionization []
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ chemistry solve ph_weak_acid Ka=1.8e-05 "c=100 mol/m^3"
pH = 2.87527706
h = 1.332670977 mol/m^3
percent_ionized = 1.332670969
  by: pH of a weak acid  (exact charge balance (no 5% approximation), solved by bisection)
  Ka = 1.8e-05
  c = 100 mol/m^3
  assumes: monoprotic acid; 25 °C
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ chemistry solve ph_weak_acid Ka=1.8e-05 "c=100 mol/m^3" --json
{"outcome":"SOLVED","message":"","value":2.8752770603192666,"unit":"","solved_for":"pH","si":[["Ka",0.000018],["c",100.0]],"values":[["pH",2.8752770603192666,""],["h",1.332670976855007,"mol/m^3"],["percent_ionized",1.3326709693512784,""]],"target":"pH","phrase":"pH","assumptions":["monoprotic acid","25 °C"],"name":"pH of a weak acid","equation":"exact charge balance (no 5% approximation), solved by bisection"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ chemistry solve ph_weak_acid Ka=1.8e-05
NEEDS_INPUT: ph_weak_acid needs c (acid concentration). Example: solve('ph_weak_acid', Ka=1.8e-05, c='100 mol/m^3').
```

### 2.9 Draw the figures

The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```shell
$ chemistry figures ph_weak_acid Ka=1.8e-05 "c=100 mol/m^3" --out .
[
  {
    "slug": "ph-weak-acid-field",
    "title": "pH of a weak acid over concentration and Ka",
    "caption": "pH (pH) over c from 10 to 1000 and Ka from 1.8e-06 to 0.00018 (logarithmic axes): each of the 21 x 15 cells is one run of the model, shaded by its value (light 1.875 to dark 3.875); iso-lines at pH = 2, 2.5, 3, 3.5. Marked: the inputs given, c = 100, Ka = 1.8e-05, pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.",
    "file": ".\\ph-weak-acid-field.svg"
  },
  {
    "slug": "ph-weak-acid-live-1",
    "title": "PH as acid concentration varies",
    "caption": "PH as c runs from 25 to 175 mol/m^3, every other input as you entered it (41 solutions). Marked: your c = 100 mol/m^3 gives pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.",
    "file": ".\\ph-weak-acid-live-1.svg"
  },
  {
    "slug": "ph-weak-acid-live-2",
    "title": "PH as acid dissociation constant varies",
    "caption": "PH as Ka runs from 4.5e-06 to 3.15e-05, every other input as you entered it (29 solutions). Marked: your Ka = 1.8e-05 gives pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.",
    "file": ".\\ph-weak-acid-live-2.svg"
  }
]
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | chemistry batch

`cases.jsonl`:

```json
{"key": "ph_weak_acid", "inputs": {"Ka": 1.8e-05, "c": "100 mol/m^3"}}
{"key": "balance_equation", "inputs": {"equation": "C3H8 + O2 -> CO2 + H2O"}}
```

```shell
$ chemistry batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":2.8752770603192666,"unit":"","solved_for":"pH","si":[["Ka",0.000018],["c",100.0]],"values":[["pH",2.8752770603192666,""],["h",1.332670976855007,"mol/m^3"],["percent_ionized",1.3326709693512784,""]],"target":"pH","phrase":"pH","assumptions":["monoprotic acid","25 °C"],"name":"pH of a weak acid","equation":"exact charge balance (no 5% approximation), solved by bisection"}
{"outcome":"SOLVED","message":"","value":null,"unit":"","solved_for":"balanced","values":[["coefficients",[1.0,5.0,3.0,4.0],""],["balanced","C3H8 + 5 O2 -> 3 CO2 + 4 H2O",""]],"target":"balanced","phrase":"the balanced equation","assumptions":["one independent way to balance (a one-dimensional null space)","neutral species"],"name":"Balancing a chemical equation","equation":"exact rational null space of the element-conservation matrix, scaled to the smallest whole numbers"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `chemistry info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`chemistry solve ... --json` prints one answer as one line of JSON. `chemistry batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | chemistry batch`. `chemistry catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["chemistry", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`chemistry figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

```shell
$ chemistry figures ph_weak_acid Ka=1.8e-05 "c=100 mol/m^3" --out .
[
  {
    "slug": "ph-weak-acid-field",
    "title": "pH of a weak acid over concentration and Ka",
    "caption": "pH (pH) over c from 10 to 1000 and Ka from 1.8e-06 to 0.00018 (logarithmic axes): each of the 21 x 15 cells is one run of the model, shaded by its value (light 1.875 to dark 3.875); iso-lines at pH = 2, 2.5, 3, 3.5. Marked: the inputs given, c = 100, Ka = 1.8e-05, pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.",
    "file": ".\\ph-weak-acid-field.svg"
  },
  {
    "slug": "ph-weak-acid-live-1",
    "title": "PH as acid concentration varies",
    "caption": "PH as c runs from 25 to 175 mol/m^3, every other input as you entered it (41 solutions). Marked: your c = 100 mol/m^3 gives pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.",
    "file": ".\\ph-weak-acid-live-1.svg"
  },
  {
    "slug": "ph-weak-acid-live-2",
    "title": "PH as acid dissociation constant varies",
    "caption": "PH as Ka runs from 4.5e-06 to 3.15e-05, every other input as you entered it (29 solutions). Marked: your Ka = 1.8e-05 gives pH = 2.875. Your inputs: Ka = 1.8e-05, c = 100 mol/m^3.",
    "file": ".\\ph-weak-acid-live-2.svg"
  }
]
```

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `chemistry_ai999x.wasm` (https://www.ai999x.com/compute/rust/chemistry_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("chemistry_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `chemistry info <id>` shows the same inputs and outputs in the terminal.

### Acids and bases (14)

<a id="buffer_ph"></a>

#### Buffer pH (Henderson-Hasselbalch) — `buffer_ph`

The pH of a buffer from the acid's Ka (or pKa) and the concentrations of the weak acid and its conjugate base.

$$
\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{[\mathrm{A^-}]}{[\mathrm{HA}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ka` | in | acid dissociation constant | — |
| `c_base` | in | conjugate base concentration | mol/m^3 |
| `c_acid` | in | weak acid concentration | mol/m^3 |
| `pH` | out | pH | — |

**Applies when:** both concentrations well above Ka and [H3O+].

```shell
$ chemistry solve buffer_ph Ka=1.737800829e-05 "c_base=150 mol/m^3" "c_acid=100 mol/m^3"
pH = 4.936091259
  by: Buffer pH (Henderson-Hasselbalch)  (Henderson-Hasselbalch equation)
  Ka = 1.737800829e-05
  c_base = 150 mol/m^3
  c_acid = 100 mol/m^3
  assumes: both concentrations well above Ka and [H3O+]
```

**What the worked example means:** This calculation determines the pH of a buffer solution using the Henderson-Hasselbalch equation. By combining the pKa, which is derived from the input Ka value of 1.7378e-05, with the logarithm of the ratio between the conjugate base concentration 150 mol/m^3 and the weak acid concentration 100 mol/m^3, the model predicts the equilibrium acidity. The resulting pH value of 4.93609 indicates how resistant the solution is to changes in hydrogen ion concentration, assuming ideal behavior where activity coefficients are close to one.

<a id="hydronium_from_ph"></a>

#### Hydronium concentration from pH — `hydronium_from_ph`

[H3O+], [OH-] and pOH from the pH at 25 °C.

$$
[\mathrm{H_3O^+}] = 10^{-\mathrm{pH}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pH` | in | pH | — |
| `h` | out | [H3O+] | mol/m^3 |
| `h_molL` | out | [H3O+] in mol/L | — |
| `pOH` | out | pOH | — |
| `oh` | out | [OH-] | mol/m^3 |

**Applies when:** 25 °C (Kw = 1.0e-14).

```shell
$ chemistry solve hydronium_from_ph pH=3.2
h = 0.6309573445 mol/m^3
h_molL = 0.0006309573445
pOH = 10.8
oh = 1.584893192e-08 mol/m^3
  by: Hydronium concentration from pH  ([H3O+] = 10^-pH)
  pH = 3.2
  assumes: 25 °C (Kw = 1.0e-14)
```

**What the worked example means:** This calculation determines the ion concentrations in an aqueous solution based on its pH value. Using the definition that hydronium concentration equals ten raised to the negative pH, the result for 3.2 is 0.000630957. The pOH is derived by subtracting the pH from fourteen, yielding 10.8, which corresponds to a hydroxide concentration of 1.58489e-08 mol/m^3. These values describe the acidity or basicity of the solution at standard temperature.

<a id="ka_from_ph"></a>

#### Ka from the pH of a weak acid — `ka_from_ph`

The acid dissociation constant of a weak monoprotic acid from the pH of a solution of known concentration.

$$
K_a = \frac{[\mathrm{H_3O^+}][\mathrm{A^-}]}{c - [\mathrm{A^-}]},\ [\mathrm{A^-}] = [\mathrm{H_3O^+}] - K_w/[\mathrm{H_3O^+}]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pH` | in | measured pH | — |
| `c` | in | acid concentration | mol/m^3 |
| `pKa` | out | pKa | — |
| `Ka` | out | Ka | — |

**Applies when:** monoprotic weak acid; 25 °C.

```shell
$ chemistry solve ka_from_ph pH=2.87 "c=100 mol/m^3"
Ka = 1.844583597e-05
pKa = 4.734101658
  by: Ka from the pH of a weak acid  ([A-] from the charge balance at the measured pH, then the mass-action expression)
  pH = 2.87
  c = 100 mol/m^3
  assumes: monoprotic weak acid; 25 °C
```

**What the worked example means:** This calculation determines the acid dissociation constant for a weak monoprotic acid using experimental data. By applying the charge balance equation to find the conjugate base concentration and substituting it into the mass-action expression, the model computes Ka as 1.84458e-05 and pKa as 4.7341. These values quantify the strength of the acid in solution, indicating how readily it donates protons under the specific conditions of 25 degrees Celsius.

<a id="titration_ph_strong_acid_strong_base"></a>

#### pH during a strong acid - strong base titration — `titration_ph_strong_acid_strong_base`

The pH after adding a volume of strong base to a strong monoprotic acid (exact, any point of the titration).

$$
[\mathrm{H_3O^+}] - \frac{K_w}{[\mathrm{H_3O^+}]} = \frac{c_a V_a - c_b V_b}{V_a + V_b}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c_a` | in | acid concentration | mol/m^3 |
| `V_a` | in | acid volume | m^3 |
| `c_b` | in | base concentration | mol/m^3 |
| `V_b` | in | volume of base added | m^3 |
| `pH` | out | pH | — |

**Applies when:** strong monoprotic acid titrated with a strong monohydroxide base; 25 °C.

```shell
$ chemistry solve titration_ph_strong_acid_strong_base "V_a=2.5e-05 m^3" "c_a=100 mol/m^3" "c_b=100 mol/m^3" "V_b=1e-05 m^3"
pH = 1.367976785
  by: pH during a strong acid - strong base titration  (excess strong acid or base after mixing, with water's ionization (exact at the equivalence point too))
  V_a = 2.5e-05 m^3
  c_a = 100 mol/m^3
  c_b = 100 mol/m^3
  V_b = 1e-05 m^3
  assumes: strong monoprotic acid titrated with a strong monohydroxide base; 25 °C
```

**What the worked example means:** This calculation determines the pH at any point during a titration of a strong monoprotic acid with a strong monohydroxide base. It uses the initial acid concentration 100 mol/m^3 and volume 2.5e-05 m^3, along with the added base concentration 100 mol/m^3 and volume 1e-05 m^3, to find the net hydronium ion concentration. By accounting for water's autoionization, it provides an exact result, yielding a final pH of 1.36798. This value indicates whether the solution is acidic, neutral, or basic based on the relative amounts of reactants mixed.

<a id="titration_ph_weak_acid_strong_base"></a>

#### pH during a weak acid - strong base titration — `titration_ph_weak_acid_strong_base`

The pH after adding strong base to a weak monoprotic acid, at any point (exact charge balance).

$$
[\mathrm{H_3O^+}] + [\mathrm{Na^+}] = \frac{K_w}{[\mathrm{H_3O^+}]} + \frac{K_a C}{K_a + [\mathrm{H_3O^+}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ka` | in | acid dissociation constant | — |
| `c_a` | in | acid concentration | mol/m^3 |
| `V_a` | in | acid volume | m^3 |
| `c_b` | in | base concentration | mol/m^3 |
| `V_b` | in | volume of base added | m^3 |
| `pH` | out | pH | — |

**Applies when:** weak monoprotic acid, strong monohydroxide base; 25 °C.

```shell
$ chemistry solve titration_ph_weak_acid_strong_base "V_a=2.5e-05 m^3" "c_a=100 mol/m^3" "c_b=100 mol/m^3" "V_b=1.25e-05 m^3" Ka=1.8e-05
pH = 4.745196013
  by: pH during a weak acid - strong base titration  (exact charge balance after mixing, solved by bisection (before, at and after equivalence))
  V_a = 2.5e-05 m^3
  c_a = 100 mol/m^3
  c_b = 100 mol/m^3
  V_b = 1.25e-05 m^3
  Ka = 1.8e-05
  assumes: weak monoprotic acid, strong monohydroxide base; 25 °C
```

**What the worked example means:** This calculation determines the exact pH at a specific point in the titration of a weak monoprotic acid with a strong base. It uses the initial acid concentration 100 mol/m^3, volume 2.5e-05 m^3, and dissociation constant 1.8e-05, along with the added base concentration 100 mol/m^3 and volume 1.25e-05 m^3. The result, 4.7452, reflects the precise charge balance of the solution, accounting for hydronium ions, sodium ions from the base, hydroxide ions, and the distribution of the acid species between its protonated and deprotonated forms.

<a id="ph_from_hydronium"></a>

#### pH from the hydronium concentration — `ph_from_hydronium`

pH, pOH and [OH-] from the hydronium (hydrogen ion) concentration at 25 °C.

$$
\mathrm{pH} = -\log_{10}[\mathrm{H_3O^+}]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | hydronium ion concentration | mol/m^3 |
| `pH` | out | pH | — |
| `pOH` | out | pOH | — |
| `oh` | out | [OH-] | mol/m^3 |

**Applies when:** [H3O+] > 0 (mol/L); 25 °C (Kw = 1.0e-14).

```shell
$ chemistry solve ph_from_hydronium "h=1 mol/m^3"
pH = 3
pOH = 11
oh = 1e-08 mol/m^3
  by: pH from the hydronium concentration  (pH = -log10[H3O+]; pOH = 14 - pH (25 °C))
  h = 1 mol/m^3
  assumes: [H3O+] > 0 (mol/L); 25 °C (Kw = 1.0e-14)
```

**What the worked example means:** This calculation determines the acidity or basicity of an aqueous solution at twenty-five degrees Celsius using the hydronium ion concentration. Given an input hydronium concentration of 1 mol/m^3, the resulting pH value is 3. The complementary pOH value is calculated as 11, which leads to a derived hydroxide ion concentration of 1e-08 mol/m^3. These values characterize the solution's acidic nature based on the logarithmic scale of hydrogen ion activity.

<a id="ph_from_hydroxide"></a>

#### pH from the hydroxide concentration — `ph_from_hydroxide`

pOH and pH from the hydroxide ion concentration at 25 °C.

$$
\mathrm{pH} = 14 + \log_{10}[\mathrm{OH^-}]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `oh` | in | hydroxide ion concentration | mol/m^3 |
| `pH` | out | pH | — |
| `pOH` | out | pOH | — |

**Applies when:** [OH-] > 0; 25 °C.

```shell
$ chemistry solve ph_from_hydroxide "oh=0.1 mol/m^3"
pH = 10
pOH = 4
  by: pH from the hydroxide concentration  (pOH = -log10[OH-]; pH = 14 - pOH (25 °C))
  oh = 0.1 mol/m^3
  assumes: [OH-] > 0; 25 °C
```

**What the worked example means:** This calculation determines the acidity or basicity of a solution based on its hydroxide ion content. Starting with a measured hydroxide ion concentration of 0.1 mol/m^3, the method first computes the pOH, which represents the negative logarithm of that concentration, yielding a value of 4. At standard temperature, the sum of pH and pOH is constant, so subtracting the computed pOH from fourteen gives the final pH of 10. This result indicates how basic the solution is; a higher pH signifies greater alkalinity.

<a id="ph_diprotic_acid"></a>

#### pH of a diprotic acid — `ph_diprotic_acid`

The pH of a diprotic acid H2A from both dissociation constants, solved exactly.

$$
[\mathrm{H^+}] = C\frac{K_1[\mathrm{H^+}] + 2K_1K_2}{[\mathrm{H^+}]^2 + K_1[\mathrm{H^+}] + K_1K_2} + \frac{K_w}{[\mathrm{H^+}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ka1` | in | first dissociation constant | — |
| `Ka2` | in | second dissociation constant | — |
| `c` | in | acid concentration | mol/m^3 |
| `pH` | out | pH | — |

**Applies when:** 25 °C.

```shell
$ chemistry solve ph_diprotic_acid Ka1=4.3e-07 Ka2=4.8e-11 "c=100 mol/m^3"
pH = 3.683715907
  by: pH of a diprotic acid  (exact charge balance of H2A, solved by bisection)
  Ka1 = 4.3e-07
  Ka2 = 4.8e-11
  c = 100 mol/m^3
  assumes: 25 °C
```

**What the worked example means:** This calculation determines the exact pH of a solution containing a diprotic acid, using the first dissociation constant 4.3e-07, the second dissociation constant 4.8e-11, and the total acid concentration 100 mol/m^3. By solving the charge balance equation without simplifying assumptions, the model finds that the hydrogen ion concentration leads to a final pH of 3.68372. This result reflects the combined effect of both proton release steps at the given temperature.

<a id="ph_salt_of_weak_acid"></a>

#### pH of a salt of a weak acid — `ph_salt_of_weak_acid`

The pH of a solution of a salt of a weak acid and a strong base (e.g. sodium acetate) from the acid's Ka.

$$
K_b = \frac{K_w}{K_a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ka` | in | acid dissociation constant | — |
| `c` | in | salt concentration | mol/m^3 |
| `pH` | out | pH | — |
| `Kb` | out | Kb of the anion (Kw/Ka) | — |

**Applies when:** salt of a weak monoprotic acid and a strong base; 25 °C.

```shell
$ chemistry solve ph_salt_of_weak_acid Ka=1.8e-05 "c=100 mol/m^3"
pH = 8.87238665
Kb = 5.555555556e-10
  by: pH of a salt of a weak acid  (the anion as a weak base with Kb = Kw/Ka, exact charge balance)
  Ka = 1.8e-05
  c = 100 mol/m^3
  assumes: salt of a weak monoprotic acid and a strong base; 25 °C
```

**What the worked example means:** This calculation determines the pH of a solution containing a salt formed from a monoprotic weak acid and a strong base. Treating the anion as a weak base gives its dissociation constant Kb from the given acid dissociation constant Ka using the relationship Kb equals Kw divided by Ka. The resulting value 5.55556e-10 represents the basicity of the anion in water. Using this constant and the initial salt concentration 100 mol/m^3, the equilibrium position is solved to find the final hydrogen ion concentration, which translates to the reported pH of 8.87239. This indicates that the solution is basic, as expected for salts derived from weak acids.

<a id="ph_strong_base"></a>

#### pH of a strong base — `ph_strong_base`

The pH and pOH of a strong (group 1 or 2) hydroxide solution.

$$
[\mathrm{OH^-}] = n c,\quad \mathrm{pH} = 14 - \mathrm{pOH}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | base concentration | mol/m^3 |
| `n_oh` | in | hydroxide ions per formula unit | — |
| `pH` | out | pH | — |
| `pOH` | out | pOH | — |

**Applies when:** strong base fully dissociated; 25 °C.

```shell
$ chemistry solve ph_strong_base n_oh=1 "c=10 mol/m^3"
pH = 12
pOH = 2
  by: pH of a strong base  (charge balance with water's ionization, [OH-] = n c + Kw/[OH-])
  c = 10 mol/m^3
  assumes: strong base fully dissociated; 25 °C
```

**What the worked example means:** This calculation determines the acidity level of a solution containing a fully dissociated strong base. Using the input base concentration 10 mol/m^3 and the number of hydroxide ions released per formula unit 1, the system computes the resulting hydroxide ion concentration. At twenty-five degrees Celsius, this leads to a final pH value of 12 and a corresponding pOH value of 2, reflecting the basicity of the solution.

<a id="ph_strong_acid"></a>

#### pH of a strong monoprotic acid — `ph_strong_acid`

The pH of a strong monoprotic acid solution (water's own ionization included, so dilute solutions are right).

$$
[\mathrm{H_3O^+}] = \frac{c}{2} + \sqrt{\frac{c^2}{4} + K_w}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | acid concentration | mol/m^3 |
| `pH` | out | pH | — |
| `h` | out | [H3O+] | mol/m^3 |

**Applies when:** monoprotic strong acid; 25 °C.

```shell
$ chemistry solve ph_strong_acid "c=10 mol/m^3"
pH = 2
h = 10 mol/m^3
  by: pH of a strong monoprotic acid  (charge balance [H3O+] = c + Kw/[H3O+] solved exactly)
  c = 10 mol/m^3
  assumes: monoprotic strong acid; 25 °C
```

**What the worked example means:** This calculation determines the exact hydronium ion concentration and pH for a strong monoprotic acid solution with an input concentration of 10 mol/m^3. By solving the charge balance equation, which includes the autoionization of water, the method yields a precise hydrogen ion activity of 10 mol/m^3, corresponding to a pH value of 2. This approach ensures accuracy even at very low concentrations where simpler approximations would fail.

<a id="ph_weak_acid"></a>

#### pH of a weak acid — `ph_weak_acid`

The pH and percent ionization of a weak monoprotic acid from Ka (or pKa) and its concentration.

$$
[\mathrm{H_3O^+}] = \frac{K_a c}{K_a + [\mathrm{H_3O^+}]} + \frac{K_w}{[\mathrm{H_3O^+}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ka` | in | acid dissociation constant | — |
| `c` | in | acid concentration | mol/m^3 |
| `pH` | out | pH | — |
| `h` | out | [H3O+] | mol/m^3 |
| `percent_ionized` | out | percent ionization | — |

**Applies when:** monoprotic acid; 25 °C.

```shell
$ chemistry solve ph_weak_acid Ka=1.8e-05 "c=100 mol/m^3"
pH = 2.87527706
h = 1.332670977 mol/m^3
percent_ionized = 1.332670969
  by: pH of a weak acid  (exact charge balance (no 5% approximation), solved by bisection)
  Ka = 1.8e-05
  c = 100 mol/m^3
  assumes: monoprotic acid; 25 °C
```

**What the worked example means:** This calculation determines the exact equilibrium state of a weak monoprotic acid by solving the charge balance equation without using standard approximations. Given an acid dissociation constant of 1.8e-05 and a total concentration of 100 mol/m^3, the model finds that the hydronium ion concentration reaches 1.33267 mol/m^3. Consequently, the solution has a pH of 2.87528, which indicates how acidic the mixture is on the logarithmic scale.

<a id="ph_weak_base"></a>

#### pH of a weak base — `ph_weak_base`

The pH, pOH and percent ionization of a weak base from Kb (or pKb) and its concentration.

$$
[\mathrm{OH^-}] = \frac{K_b c}{K_b + [\mathrm{OH^-}]} + \frac{K_w}{[\mathrm{OH^-}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kb` | in | base dissociation constant | — |
| `c` | in | base concentration | mol/m^3 |
| `pH` | out | pH | — |
| `pOH` | out | pOH | — |
| `percent_ionized` | out | percent ionization | — |

**Applies when:** 25 °C.

```shell
$ chemistry solve ph_weak_base Kb=1.8e-05 "c=100 mol/m^3"
pH = 11.12472294
pOH = 2.87527706
percent_ionized = 1.332670969
  by: pH of a weak base  (exact charge balance for [OH-], solved by bisection; pH = 14 - pOH)
  Kb = 1.8e-05
  c = 100 mol/m^3
  assumes: 25 °C
```

**What the worked example means:** This calculation determines the acidity or basicity of a solution containing a weak base. Using the provided base dissociation constant and initial concentration, an exact charge balance equation for hydroxide ions was solved numerically. The resulting hydroxide concentration yields a pOH of 2.87528, which translates to a pH of 11.1247. The fraction of base molecules that have reacted with water to form ions is expressed as a percent ionization of 1.33267. These values assume standard conditions of twenty-five degrees Celsius.

<a id="titration_equivalence_volume"></a>

#### Titration: equivalence-point volume — `titration_equivalence_volume`

The volume of titrant needed to reach the equivalence point of a neutralization.

$$
V_{eq} = \frac{\nu\, c_a V_a}{c_t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c_t` | in | titrant concentration | mol/m^3 |
| `V_a` | in | volume of the analyte | m^3 |
| `c_a` | in | analyte concentration | mol/m^3 |
| `ratio` | in | moles of titrant per mole of analyte | — |
| `V_eq` | out | titrant volume at equivalence | m^3 |
| `V_eq_mL` | out | in mL | — |

**Applies when:** complete neutralization; the acid's protons and base's hydroxides from the formulas.

```shell
$ chemistry solve titration_equivalence_volume "c_t=100 mol/m^3" "V_a=2.5e-05 m^3" "c_a=150 mol/m^3" ratio=1
V_eq = 3.75e-05 m^3
V_eq_mL = 37.5
  by: Titration: equivalence-point volume  (moles of titrant = ratio x moles of analyte)
  c_t = 100 mol/m^3
  V_a = 2.5e-05 m^3
  c_a = 150 mol/m^3
  ratio = 1
  assumes: complete neutralization; the acid's protons and base's hydroxides from the formulas
```

**What the worked example means:** This calculation determines the volume of titrant required to reach the equivalence point, which is 3.75e-05 m^3. It uses the analyte concentration of 150 mol/m^3, the analyte volume of 2.5e-05 m^3, and the titrant concentration of 100 mol/m^3, adjusted by the stoichiometric ratio of 1. The result indicates how much base or acid must be added to completely neutralize the sample.

### Electrochemistry (5)

<a id="k_from_cell_potential"></a>

#### Equilibrium constant from a standard cell potential — `k_from_cell_potential`

The equilibrium constant of a cell reaction from its standard potential (ln K = nFE°/RT).

$$
\ln K = \frac{nFE^\circ}{RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | electrons transferred | — |
| `E` | in | standard cell potential | V |
| `T` | in | temperature | K |
| `lnK` | out | ln K | — |
| `K` | out | K | — |

**Applies when:** n, T > 0.

```shell
$ chemistry solve k_from_cell_potential n=2 "E=0.46 V" "T=298 K"
lnK = 35.8260291
K = 3.622819917e+15
  by: Equilibrium constant from a standard cell potential  (ln K = n F E° / (R T))
  n = 2
  E = 0.46 V
  T = 298 K
  assumes: n, T > 0
```

**What the worked example means:** This calculation determines the equilibrium constant for a redox reaction using its standard cell potential. Applying the relationship between Gibbs free energy and electrochemical potential shows that with 2 electrons transferred at 298 K, the natural logarithm of the equilibrium constant is 35.826. This implies an equilibrium constant K of 3.62282e+15, indicating the extent to which the reaction proceeds toward products under standard conditions.

<a id="delta_g_from_cell_potential"></a>

#### Gibbs energy from a cell potential — `delta_g_from_cell_potential`

The (standard) Gibbs energy change of a cell reaction from its potential (dG = -nFE).

$$
\Delta G = -nFE
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | electrons transferred | — |
| `E` | in | cell potential | V |
| `dG` | out | Gibbs energy change | J/mol |
| `dG_kJ` | out | in kJ/mol | — |

**Applies when:** n > 0.

```shell
$ chemistry solve delta_g_from_cell_potential n=2 "E=1.1 V"
dG = -212267.7307 J/mol
dG_kJ = -212.2677307
  by: Gibbs energy from a cell potential  (dG = -n F E)
  n = 2
  E = 1.1 V
  assumes: n > 0
```

**What the worked example means:** This calculation determines the Gibbs free energy change for an electrochemical reaction using the relationship between electrical work and thermodynamic spontaneity. Multiplying the number of electrons transferred, Faraday's constant, and the measured cell potential gives the energy change in Joules per mole. The result, -212268 J/mol, indicates whether the reaction releases or absorbs energy under these conditions. A negative value suggests the process is spontaneous, while a positive value implies non-spontaneity. This metric helps predict battery performance and reaction feasibility without needing direct calorimetric measurements.

<a id="electrolysis_mass"></a>

#### Mass deposited in electrolysis (Faraday's law) — `electrolysis_mass`

The mass of a metal deposited by a current flowing for a time.

$$
m = \frac{I t}{n F} M
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `metal` | in | the element deposited (symbol or name) | — |
| `I` | in | current | A |
| `t` | in | time | s |
| `n` | in | electrons transferred | — |
| `m` | out | mass deposited | kg |
| `m_g` | out | in grams | — |

**Applies when:** 100% current efficiency.

```shell
$ chemistry solve electrolysis_mass n=2 metal=Cu "I=2 A" "t=1800 s"
m = 0.001185494183 kg
m_g = 1.185494183
  by: Mass deposited in electrolysis (Faraday's law)  (charge / (n F) gives moles of metal; times its molar mass)
  n = 2
  I = 2 A
  t = 1800 s
  metal = Cu
  assumes: 100% current efficiency
```

**What the worked example means:** This calculation determines the amount of metal deposited during electrolysis using Faraday's law. By combining the current 2 A, the duration 1800 s, and the number of electrons transferred 2, the model computes the total charge passed through the circuit. This charge is then related to the moles of substance reduced at the electrode and converted into mass. The result shows that a current of 2 A flowing for 1800 s deposits 0.00118549 kg of the specified metal.

<a id="nernst_potential"></a>

#### Nernst equation — `nernst_potential`

The cell potential at non-standard conditions from E°, the electrons transferred and the reaction quotient.

$$
E = E^\circ - \frac{RT}{nF}\ln Q
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E0` | in | standard cell potential | V |
| `n` | in | electrons transferred | — |
| `Q` | in | reaction quotient | — |
| `T` | in | temperature | K |
| `E` | out | cell potential | V |

**Applies when:** n, Q, T > 0.

```shell
$ chemistry solve nernst_potential n=2 Q=0.01 "E0=1.1 V" "T=298 K"
E = 1.159129586 V
  by: Nernst equation  (E = E° - (RT/nF) ln Q)
  n = 2
  Q = 0.01
  E0 = 1.1 V
  T = 298 K
  assumes: n, Q, T > 0
```

**What the worked example means:** This calculation determines the actual voltage of an electrochemical cell under non-standard conditions using the Nernst equation. Starting with a standard cell potential of 1.1 V, the model adjusts this value based on the number of electrons transferred, which is 2, and the current reaction quotient, 0.01, at a temperature of 298 K. The resulting cell potential of 1.15913 V indicates whether the reaction remains spontaneous or how its driving force changes as concentrations shift away from standard states.

<a id="electrolysis_time"></a>

#### Time for electrolysis (Faraday's law) — `electrolysis_time`

How long a current must flow to deposit a given mass of a metal.

$$
t = \frac{m n F}{M I}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `metal` | in | the element deposited (symbol or name) | — |
| `m` | in | mass to deposit | kg |
| `I` | in | current | A |
| `n` | in | electrons transferred | — |
| `t` | out | time needed | s |

**Applies when:** 100% current efficiency.

```shell
$ chemistry solve electrolysis_time n=2 metal=Ni "I=5 A" "m=0.01 kg"
t = 6575.593827 s
  by: Time for electrolysis (Faraday's law)  (t = m n F / (M I))
  n = 2
  I = 5 A
  m = 0.01 kg
  metal = Ni
  assumes: 100% current efficiency
```

**What the worked example means:** This calculation determines the time needed to deposit a specific mass of metal using Faraday's law. By inputting the desired mass 0.01 kg, the applied current 5 A, and the number of electrons transferred 2, the model computes the duration 6575.59 s. This result indicates how long the electrolysis process must run to achieve the target deposition under ideal conditions.

### Equilibrium (10)

<a id="equilibrium_ice"></a>

#### Equilibrium concentrations (ICE table) — `equilibrium_ice`

Equilibrium concentrations (or partial pressures) from the initial ones and K, for any one reaction as written.

$$
K = \prod_i (c_{i,0} + \nu_i x)^{\nu_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `equation` | in | the reaction as written (its coefficients define K) | — |
| `initial` | in | initial concentration of each species in the order written (mol/L, or atm for Kp) | — |
| `K` | in | equilibrium constant | — |
| `x` | out | extent of reaction (mol/L) | — |
| `equilibrium` | out | equilibrium concentrations in the order written | — |

**Applies when:** one reaction, coefficients as written; no pure solids or liquids; K > 0.

```shell
$ chemistry solve equilibrium_ice "equation=N2O4 ⇌ 2 NO2" "initial=[0.1, 0]" K=0.0046
x = 0.0101642097
equilibrium = [0.08983579029909557,0.020328419401808878]
  by: Equilibrium concentrations (ICE table)  (the extent x solved from the mass-action expression by bisection between the limits set by the initial amounts)
  K = 0.0046
  equation = N2O4 ⇌ 2 NO2
  assumes: one reaction, coefficients as written; no pure solids or liquids; K > 0
```

**What the worked example means:** This calculation determines the equilibrium state for a single reaction using an iterative numerical method. By solving the mass action expression, it finds the extent of reaction, which is 0.0101642, to adjust the initial amounts given as [0.1, 0]. The resulting equilibrium concentrations or partial pressures are reported in the order written as [0.0898358, 0.0203284]. This approach handles complex stoichiometries where simple algebraic solutions are difficult, ensuring that the system satisfies the equilibrium constant 0.0046.

<a id="k_from_delta_g"></a>

#### Equilibrium constant from the standard Gibbs energy — `k_from_delta_g`

The equilibrium constant from the standard Gibbs energy change (K = exp(-dG°/RT)).

$$
K = e^{-\Delta G^\circ / RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dG` | in | standard Gibbs energy change | J/mol |
| `T` | in | temperature | K |
| `K` | out | equilibrium constant | — |
| `lnK` | out | ln K | — |

**Applies when:** T > 0.

```shell
$ chemistry solve k_from_delta_g "dG=-33000 J/mol" "T=298 K"
K = 608499.1084
lnK = 13.31875073
  by: Equilibrium constant from the standard Gibbs energy  (ln K = -dG°/(RT))
  dG = -33000 J/mol
  T = 298 K
  assumes: T > 0
```

**What the worked example means:** This calculation determines the equilibrium constant, 608499, using the relationship between thermodynamic favorability and reaction position. It takes the standard Gibbs energy change, -33000 J/mol, and the absolute temperature, 298 K, to compute the natural logarithm of the equilibrium constant, 13.3188. The result indicates how strongly products are favored over reactants at equilibrium under these specific conditions.

<a id="delta_g_nonstandard"></a>

#### Gibbs energy change at non-standard conditions — `delta_g_nonstandard`

The Gibbs energy change of a reaction at a given reaction quotient (dG = dG° + RT ln Q).

$$
\Delta G = \Delta G^\circ + RT\ln Q
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dG0` | in | standard Gibbs energy change | J/mol |
| `T` | in | temperature | K |
| `Q` | in | reaction quotient | — |
| `dG` | out | Gibbs energy change | J/mol |

**Applies when:** T, Q > 0.

```shell
$ chemistry solve delta_g_nonstandard "dG0=-33000 J/mol" Q=100 "T=298 K"
dG = -21589.72442 J/mol
  by: Gibbs energy change at non-standard conditions  (dG = dG° + RT ln Q)
  dG0 = -33000 J/mol
  Q = 100
  T = 298 K
  assumes: T, Q > 0
```

**What the worked example means:** This calculation determines the actual Gibbs energy change for a reaction under non-standard conditions using the standard Gibbs energy change -33000 J/mol, temperature 298 K, and reaction quotient 100. The result, -21589.7 J/mol, indicates whether the reaction will proceed spontaneously in the forward direction if negative, remain at equilibrium if zero, or favor the reverse direction if positive.

<a id="kc_from_kp"></a>

#### Kc from Kp — `kc_from_kp`

The concentration equilibrium constant from the pressure one (Kc = Kp / (RT)^dn).

$$
K_c = \frac{K_p}{(RT)^{\Delta n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kp` | in | pressure equilibrium constant (atm basis) | — |
| `T` | in | temperature | K |
| `dn` | in | change in moles of gas (products - reactants) | — |
| `Kc` | out | concentration equilibrium constant | — |

**Applies when:** Kp, T > 0; dn counts gaseous species only.

```shell
$ chemistry solve kc_from_kp dn=1 Kp=1.8 "T=523 K"
Kc = 0.0419423967
  by: Kc from Kp  (Kc = Kp / (RT)^dn with R = 0.082057 L·atm/(mol·K))
  dn = 1
  Kp = 1.8
  T = 523 K
  assumes: Kp, T > 0; dn counts gaseous species only
```

**What the worked example means:** This calculation determines the concentration-based equilibrium constant 0.0419424 using the provided pressure-based constant 1.8, the absolute temperature 523 K, and the change in moles of gaseous species 1. The relationship relies on the ideal gas law to convert between partial pressures and molar concentrations. A positive value for the mole change indicates that more gas molecules are produced than consumed, which typically lowers the concentration constant relative to the pressure constant at standard conditions.

<a id="kp_from_kc"></a>

#### Kp from Kc — `kp_from_kc`

The pressure equilibrium constant from the concentration one (Kp = Kc (RT)^dn, atm and mol/L).

$$
K_p = K_c (RT)^{\Delta n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kc` | in | concentration equilibrium constant | — |
| `T` | in | temperature | K |
| `dn` | in | change in moles of gas (products - reactants) | — |
| `Kp` | out | pressure equilibrium constant (atm basis) | — |

**Applies when:** Kc, T > 0; dn counts gaseous species only.

```shell
$ chemistry solve kp_from_kc dn=-2 Kc=0.5 "T=400 K"
Kp = 0.000464103535
  by: Kp from Kc  (Kp = Kc (RT)^dn with R = 0.082057 L·atm/(mol·K))
  dn = -2
  Kc = 0.5
  T = 400 K
  assumes: Kc, T > 0; dn counts gaseous species only
```

**What the worked example means:** This calculation determines the equilibrium constant in terms of partial pressures, Kp, using the known concentration equilibrium constant, Kc. The relationship relies on the ideal gas law, scaling Kc by the factor (RT) raised to the power of dn, where R is the gas constant and T is the absolute temperature. For this reaction, with a change in moles of gas equal to -2, the resulting value is 0.000464104. This conversion allows for comparison between experimental data collected as concentrations versus those collected as pressures.

<a id="ksp_from_solubility"></a>

#### Ksp from molar solubility — `ksp_from_solubility`

The solubility product of a salt MxAy from its molar solubility.

$$
K_{sp} = (x s)^x (y s)^y
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `s` | in | molar solubility | mol/m^3 |
| `x` | in | cations per formula unit | — |
| `y` | in | anions per formula unit | — |
| `Ksp` | out | solubility product | — |

**Applies when:** s > 0.

```shell
$ chemistry solve ksp_from_solubility x=1 y=2 "s=1.5 mol/m^3"
Ksp = 1.35e-08
pKsp = 7.869666232
  by: Ksp from molar solubility  (Ksp = (x s)^x (y s)^y)
  s = 1.5 mol/m^3
  assumes: s > 0
```

**What the worked example means:** This calculation determines the solubility product constant for a salt using its measured molar solubility. By applying the stoichiometric coefficients to the solubility value, the model computes the equilibrium constant representing the product of ion concentrations at saturation. The resulting solubility product is 1.35e-08, derived from a molar solubility of 1.5 mol/m^3. This value quantifies the extent to which the solid dissolves in water under standard conditions.

<a id="ksp_molar_solubility"></a>

#### Molar solubility from Ksp — `ksp_molar_solubility`

The molar solubility of a sparingly soluble salt MxAy in pure water from its solubility product.

$$
K_{sp} = (x s)^x (y s)^y
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ksp` | in | solubility product | — |
| `x` | in | cations per formula unit | — |
| `y` | in | anions per formula unit | — |
| `s` | out | molar solubility | mol/m^3 |
| `s_molL` | out | in mol/L | — |

**Applies when:** Ksp > 0; no common ion, no side reactions.

```shell
$ chemistry solve ksp_molar_solubility x=1 y=2 Ksp=3.9e-11
s = 0.2136329341 mol/m^3
s_molL = 0.0002136329341
log10_s = -3.670331795
  by: Molar solubility from Ksp  (s = (Ksp / (x^x y^y))^(1/(x+y)))
  Ksp = 3.9e-11
  assumes: Ksp > 0; no common ion, no side reactions
```

**What the worked example means:** This calculation determines the maximum concentration of a sparingly soluble salt that can exist in pure water before precipitation begins. Using the provided solubility product constant and the stoichiometric coefficients for the cation and anion, the model computes the molar solubility. The result indicates that at equilibrium, the dissolved amount reaches 0.213633 mol/m^3, meaning no more solid can dissolve without changing conditions or adding other ions.

<a id="common_ion_solubility"></a>

#### Molar solubility with a common ion — `common_ion_solubility`

The molar solubility of a salt MxAy in a solution that already contains one of its ions.

$$
K_{sp} = (x s + c_{M})^x (y s + c_{A})^y
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ksp` | in | solubility product | — |
| `x` | in | cations per formula unit | — |
| `y` | in | anions per formula unit | — |
| `c_common` | in | concentration of the common ion | mol/m^3 |
| `common` | in | which ion is common | — |
| `s` | out | molar solubility | mol/m^3 |
| `s_molL` | out | in mol/L | — |

**Applies when:** Ksp > 0; the common ion comes from a fully dissolved salt.

```shell
$ chemistry solve common_ion_solubility x=1 y=2 common=anion "c_common=100 mol/m^3" Ksp=3.9e-11
s = 3.899999392e-06 mol/m^3
s_molL = 3.899999392e-09
log10_s = -8.408935461
  by: Molar solubility with a common ion  (the solubility s solved from the Ksp expression with the common ion's concentration, by bisection)
  c_common = 100 mol/m^3
  Ksp = 3.9e-11
  common = anion
  assumes: Ksp > 0; the common ion comes from a fully dissolved salt
```

**What the worked example means:** This calculation determines the molar solubility of a salt when one of its ions is already present in the solution. Using the provided inputs, the result shows that the equilibrium shifts to reduce dissolution, yielding a solubility of 3.9e-06 mol/m^3. This value represents the maximum amount of the solid that can dissolve under these specific conditions.

<a id="delta_g_from_k"></a>

#### Standard Gibbs energy from the equilibrium constant — `delta_g_from_k`

The standard Gibbs energy change from the equilibrium constant (dG° = -RT ln K).

$$
\Delta G^\circ = -RT\ln K
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | equilibrium constant | — |
| `T` | in | temperature | K |
| `dG` | out | standard Gibbs energy change | J/mol |

**Applies when:** K, T > 0.

```shell
$ chemistry solve delta_g_from_k K=100000 "T=298 K"
dG = -28525.68894 J/mol
  by: Standard Gibbs energy from the equilibrium constant  (dG° = -RT ln K)
  K = 100000
  T = 298 K
  assumes: K, T > 0
```

**What the worked example means:** This calculation determines the standard Gibbs energy change, -28525.7 J/mol, using the equilibrium constant 100000 and temperature 298 K. The relationship connects chemical equilibrium with thermodynamic favorability. A negative value indicates that products are favored under standard conditions, while a positive value suggests reactants are favored. The magnitude reflects the extent of this preference.

<a id="vant_hoff_k2"></a>

#### Van 't Hoff equation: K at another temperature — `vant_hoff_k2`

The equilibrium constant at a second temperature from K at a first one and the reaction enthalpy.

$$
\ln\frac{K_2}{K_1} = -\frac{\Delta H^\circ}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K1` | in | equilibrium constant at T1 | — |
| `T1` | in | first temperature | K |
| `T2` | in | second temperature | K |
| `dH` | in | standard enthalpy change | J/mol |
| `K2` | out | equilibrium constant at T2 | — |
| `lnK2` | out | ln K2 | — |

**Applies when:** K1, T1, T2 > 0; dH° independent of T.

```shell
$ chemistry solve vant_hoff_k2 K1=1.8e-05 "T1=298 K" "dH=50000 J/mol" "T2=350 K"
K2 = 0.0003608751648
lnK2 = -7.926978463
  by: Van 't Hoff equation: K at another temperature  (integrated van 't Hoff equation (dH° constant))
  K1 = 1.8e-05
  T1 = 298 K
  dH = 50000 J/mol
  T2 = 350 K
  assumes: K1, T1, T2 > 0; dH° independent of T
```

**What the worked example means:** This calculation determines the new equilibrium constant using the integrated van 't Hoff equation, which relates changes in the equilibrium constant to temperature shifts assuming a constant standard enthalpy of reaction. Starting with the known equilibrium constant 1.8e-05 at temperature 298 K, the method applies the enthalpy change 50000 J/mol to predict the constant at temperature 350 K. The resulting natural logarithm of the new constant is -7.92698, which corresponds to an equilibrium constant of 0.000360875. This result indicates how the position of chemical equilibrium shifts when the system is heated or cooled under ideal conditions.

### Gases (22)

<a id="boyle_final_pressure"></a>

#### Boyle's law: final pressure — `boyle_final_pressure`

The pressure of a gas after a volume change at constant temperature.

$$
P_2 = \frac{P_1 V_1}{V_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | initial pressure | Pa |
| `V1` | in | initial volume | m^3 |
| `V2` | in | final volume | m^3 |
| `P2` | out | final pressure | Pa |

**Applies when:** P1, V1, V2 > 0; constant temperature and amount.

```shell
$ chemistry solve boyle_final_pressure "V2=0.001 m^3" "P1=101325 Pa" "V1=0.004 m^3"
P2 = 405300 Pa
  by: Boyle's law: final pressure  (P2 = P1 V1 / V2)
  V2 = 0.001 m^3
  P1 = 101325 Pa
  V1 = 0.004 m^3
  assumes: P1, V1, V2 > 0; constant temperature and amount
```

**What the worked example means:** This calculation determines the final pressure of a gas after its volume changes, assuming temperature and the amount of gas remain constant. Using Boyle's law, which states that pressure and volume are inversely proportional, the result shows that for an initial pressure of 101325 Pa and volumes of 0.004 m^3 and 0.001 m^3, the new pressure settles at 405300 Pa. This means that if the gas was compressed, the pressure rose, or if it expanded, the pressure fell, reflecting the direct impact of volume changes on gas pressure in ideal conditions.

<a id="boyle_final_volume"></a>

#### Boyle's law: final volume — `boyle_final_volume`

The volume of a gas after a pressure change at constant temperature (P1V1 = P2V2).

$$
V_2 = \frac{P_1 V_1}{P_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | initial pressure | Pa |
| `V1` | in | initial volume | m^3 |
| `P2` | in | final pressure | Pa |
| `V2` | out | final volume | m^3 |

**Applies when:** P1, V1, P2 > 0; constant temperature and amount.

```shell
$ chemistry solve boyle_final_volume "P2=303975 Pa" "V1=0.002 m^3" "P1=101325 Pa"
V2 = 0.0006666666667 m^3
  by: Boyle's law: final volume  (V2 = P1 V1 / P2)
  P2 = 303975 Pa
  V1 = 0.002 m^3
  P1 = 101325 Pa
  assumes: P1, V1, P2 > 0; constant temperature and amount
```

**What the worked example means:** This calculation determines the new volume of a gas when its pressure changes while temperature remains constant. Applying Boyle's law, which states that pressure and volume are inversely proportional, uses the initial pressure 101325 Pa and initial volume 0.002 m^3 to find the final volume 0.000666667 m^3 given the final pressure 303975 Pa. The result indicates how much space the gas occupies after the compression or expansion process is complete.

<a id="charles_final_volume"></a>

#### Charles's law: final volume — `charles_final_volume`

The volume of a gas after a temperature change at constant pressure (V1/T1 = V2/T2).

$$
V_2 = V_1 \frac{T_2}{T_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V1` | in | initial volume | m^3 |
| `T1` | in | initial temperature | K |
| `T2` | in | final temperature | K |
| `V2` | out | final volume | m^3 |

**Applies when:** V1 > 0, T1, T2 > 0 K; constant pressure and amount.

```shell
$ chemistry solve charles_final_volume "T2=348.15 K" "V1=0.0025 m^3" "T1=298.15 K"
V2 = 0.002919252054 m^3
  by: Charles's law: final volume  (V2 = V1 T2 / T1 (kelvin))
  T2 = 348.15 K
  V1 = 0.0025 m^3
  T1 = 298.15 K
  assumes: V1 > 0, T1, T2 > 0 K; constant pressure and amount
```

**What the worked example means:** This calculation finds the new volume of a gas after a temperature change, assuming pressure and amount stay constant. Applying Charles's law shows that the final volume 0.00291925 m^3 results from scaling the initial volume 0.0025 m^3 by the factor of the final temperature 348.15 K divided by the initial temperature 298.15 K. This explains why balloons expand when heated and shrink when cooled, provided the pressures remain equal.

<a id="combined_gas_final_pressure"></a>

#### Combined gas law: final pressure — `combined_gas_final_pressure`

The pressure of a fixed amount of gas after its volume and temperature change.

$$
P_2 = \frac{P_1 V_1 T_2}{T_1 V_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | initial pressure | Pa |
| `V1` | in | initial volume | m^3 |
| `T1` | in | initial temperature | K |
| `V2` | in | final volume | m^3 |
| `T2` | in | final temperature | K |
| `P2` | out | final pressure | Pa |

**Applies when:** all values > 0 (temperatures in kelvin); fixed amount of gas.

```shell
$ chemistry solve combined_gas_final_pressure "V2=0.0015 m^3" "T2=450 K" "P1=101325 Pa" "T1=300 K" "V1=0.003 m^3"
P2 = 303975 Pa
  by: Combined gas law: final pressure  (P1V1/T1 = P2V2/T2 solved for P2)
  V2 = 0.0015 m^3
  T2 = 450 K
  P1 = 101325 Pa
  T1 = 300 K
  V1 = 0.003 m^3
  assumes: all values > 0 (temperatures in kelvin); fixed amount of gas
```

**What the worked example means:** This calculation finds the new pressure of a fixed amount of gas after its volume and temperature have changed. Combining the initial conditions with the final state using the combined gas law shows that the resulting pressure is 303975 Pa. This value represents the force exerted by the gas per unit area in the new configuration.

<a id="combined_gas_final_temperature"></a>

#### Combined gas law: final temperature — `combined_gas_final_temperature`

The temperature of a fixed amount of gas after its pressure and volume change.

$$
T_2 = \frac{P_2 V_2 T_1}{P_1 V_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | initial pressure | Pa |
| `V1` | in | initial volume | m^3 |
| `T1` | in | initial temperature | K |
| `P2` | in | final pressure | Pa |
| `V2` | in | final volume | m^3 |
| `T2` | out | final temperature | K |

**Applies when:** all values > 0 (temperatures in kelvin); fixed amount of gas.

```shell
$ chemistry solve combined_gas_final_temperature "P1=101325 Pa" "V1=0.002 m^3" "T1=300 K" "P2=202650 Pa" "V2=0.0015 m^3"
T2 = 450 K
  by: Combined gas law: final temperature  (P1V1/T1 = P2V2/T2 solved for T2)
  P1 = 101325 Pa
  V1 = 0.002 m^3
  T1 = 300 K
  P2 = 202650 Pa
  V2 = 0.0015 m^3
  assumes: all values > 0 (temperatures in kelvin); fixed amount of gas
```

**What the worked example means:** This calculation determines the final temperature of a fixed amount of gas after changes in pressure and volume. Using the combined gas law, the result is derived from the initial pressure, initial volume, initial temperature, final pressure, and final volume. The computed final temperature is 450 K, which reflects how the thermal state adjusts to the new mechanical conditions within the sealed system.

<a id="combined_gas_final_volume"></a>

#### Combined gas law: final volume — `combined_gas_final_volume`

The volume of a fixed amount of gas after its pressure and temperature change.

$$
V_2 = \frac{P_1 V_1 T_2}{T_1 P_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | initial pressure | Pa |
| `V1` | in | initial volume | m^3 |
| `T1` | in | initial temperature | K |
| `P2` | in | final pressure | Pa |
| `T2` | in | final temperature | K |
| `V2` | out | final volume | m^3 |

**Applies when:** all values > 0 (temperatures in kelvin); fixed amount of gas.

```shell
$ chemistry solve combined_gas_final_volume "P1=101325 Pa" "T1=300 K" "P2=202650 Pa" "T2=400 K" "V1=0.004 m^3"
V2 = 0.002666666667 m^3
  by: Combined gas law: final volume  (P1V1/T1 = P2V2/T2 solved for V2)
  P1 = 101325 Pa
  T1 = 300 K
  P2 = 202650 Pa
  T2 = 400 K
  V1 = 0.004 m^3
  assumes: all values > 0 (temperatures in kelvin); fixed amount of gas
```

**What the worked example means:** This calculation determines the final volume of a fixed amount of gas after changes in both pressure and temperature. Applying the combined gas law, which merges Boyle's and Charles's laws, solves for the new volume using the formula where the final volume equals the product of initial pressure, initial volume, and final temperature divided by the product of initial temperature and final pressure. The result, 0.00266667 m^3, represents the space the gas occupies under the new conditions of 202650 Pa and 400 K.

<a id="gas_density"></a>

#### Density of an ideal gas — `gas_density`

The density of a named gas at a given pressure and temperature (ideal gas).

$$
\rho = \frac{PM}{RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the gas | — |
| `P` | in | pressure | Pa |
| `T` | in | temperature | K |
| `rho` | out | density | kg/m^3 |

**Applies when:** P, T > 0.

```shell
$ chemistry solve gas_density formula=CO2 "P=101325 Pa" "T=298 K"
rho = 1.799731275 kg/m^3
  by: Density of an ideal gas  (rho = PM / RT)
  P = 101325 Pa
  T = 298 K
  formula = CO2
  assumes: P, T > 0
```

**What the worked example means:** This calculation determines the density of an ideal gas, denoted as 1.79973 kg/m^3, based on its pressure 101325 Pa and temperature 298 K. The result represents the mass per unit volume of the gas under these specific conditions. Applying the ideal gas law relationship predicts how tightly packed the gas molecules are without needing to physically measure a sample.

<a id="gay_lussac_final_pressure"></a>

#### Gay-Lussac's law: final pressure — `gay_lussac_final_pressure`

The pressure of a gas after a temperature change at constant volume (P1/T1 = P2/T2).

$$
P_2 = P_1 \frac{T_2}{T_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | initial pressure | Pa |
| `T1` | in | initial temperature | K |
| `T2` | in | final temperature | K |
| `P2` | out | final pressure | Pa |

**Applies when:** P1 > 0, T1, T2 > 0 K; constant volume and amount.

```shell
$ chemistry solve gay_lussac_final_pressure "T2=318.15 K" "P1=222915 Pa" "T1=293.15 K"
P2 = 241925.3189 Pa
  by: Gay-Lussac's law: final pressure  (P2 = P1 T2 / T1 (kelvin))
  T2 = 318.15 K
  P1 = 222915 Pa
  T1 = 293.15 K
  assumes: P1 > 0, T1, T2 > 0 K; constant volume and amount
```

**What the worked example means:** This calculation determines the new pressure of a gas after its temperature changes, assuming the volume and amount of gas remain constant. By applying Gay-Lussac's law, the final pressure 241925 Pa is derived from the initial pressure 222915 Pa and the ratio of the final temperature 318.15 K to the initial temperature 293.15 K. This result indicates how much the internal force on the container walls increases or decreases due solely to thermal energy changes.

<a id="graham_molar_mass"></a>

#### Graham's law: molar mass of an unknown gas — `graham_molar_mass`

The molar mass of a gas from how fast it effuses relative to a known gas.

$$
M_x = \frac{M_{\text{known}}}{(r_x / r_{\text{known}})^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ratio` | in | rate of the unknown gas / rate of the known gas | — |
| `formula_known` | in | formula of the known gas | — |
| `M` | out | molar mass of the unknown gas | kg/mol |
| `M_g` | out | molar mass in g/mol | — |

**Applies when:** ratio > 0; same temperature and pressure.

```shell
$ chemistry solve graham_molar_mass ratio=1.5 formula_known=O2
M = 0.01422133333 kg/mol
M_g = 14.22133333
  by: Graham's law: molar mass of an unknown gas  (M_x = M_known / ratio^2)
  ratio = 1.5
  formula_known = O2
  assumes: ratio > 0; same temperature and pressure
```

**What the worked example means:** This calculation determines the molar mass of an unknown gas using Graham's law. Comparing the effusion rate of the unknown gas to that of a known reference shows that the molar mass is 0.0142213 kg/mol. This result indicates how heavy the gas molecules are relative to the standard, allowing for identification without direct chemical analysis.

<a id="graham_rate_ratio"></a>

#### Graham's law: ratio of effusion rates — `graham_rate_ratio`

How many times faster one gas effuses (or diffuses) than another.

$$
\frac{r_1}{r_2} = \sqrt{\frac{M_2}{M_1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula_1` | in | first gas (numerator) | — |
| `formula_2` | in | second gas | — |
| `ratio` | out | rate of gas 1 / rate of gas 2 | — |

**Applies when:** ideal gases at the same temperature and pressure.

```shell
$ chemistry solve graham_rate_ratio formula_1=H2 formula_2=O2
ratio = 3.98397086
  by: Graham's law: ratio of effusion rates  (square root of the inverse ratio of molar masses)
  formula_1 = H2
  formula_2 = O2
  assumes: ideal gases at the same temperature and pressure
```

**What the worked example means:** This calculation determines the relative speed at which two gases escape through a small opening. Applying Graham's Law compares the molar masses of the first gas and the second gas to find the ratio of their effusion rates. The result indicates that gas one moves 3.98397 times faster than gas two under identical conditions.

<a id="ideal_gas_moles"></a>

#### Ideal gas law: amount — `ideal_gas_moles`

The amount (moles) of an ideal gas from its pressure, volume and temperature.

$$
n = \frac{PV}{RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | pressure | Pa |
| `V` | in | volume | m^3 |
| `T` | in | temperature | K |
| `n` | out | amount of gas | mol |

**Applies when:** P, V, T > 0 (T in kelvin).

```shell
$ chemistry solve ideal_gas_moles "V=0.005 m^3" "P=202650 Pa" "T=300 K"
n = 0.4062198792 mol
  by: Ideal gas law: amount  (n = PV / RT)
  V = 0.005 m^3
  P = 202650 Pa
  T = 300 K
  assumes: P, V, T > 0 (T in kelvin)
```

In the worked example, V = 0.005 m^3, P = 202650 Pa, T = 300 K go through the equation above and give n = 0.4062198792 mol. It applies only when P, V, T > 0 (T in kelvin): outside that it says what is wrong instead of returning a number.

<a id="ideal_gas_pressure"></a>

#### Ideal gas law: pressure — `ideal_gas_pressure`

The pressure of an ideal gas from its amount, volume and temperature (PV = nRT).

$$
P = \frac{nRT}{V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | amount of gas | mol |
| `V` | in | volume | m^3 |
| `T` | in | temperature | K |
| `P` | out | pressure | Pa |
| `P_atm` | out | pressure in atm | — |

**Applies when:** n, V, T > 0 (T in kelvin).

```shell
$ chemistry solve ideal_gas_pressure "n=2 mol" "V=0.01 m^3" "T=300 K"
P = 498867.7571 Pa
P_atm = 4.923441965
  by: Ideal gas law: pressure  (P = nRT / V)
  n = 2 mol
  V = 0.01 m^3
  T = 300 K
  assumes: n, V, T > 0 (T in kelvin)
```

**What the worked example means:** This calculation determines the pressure of an ideal gas using its amount, volume, and temperature. Applying the ideal gas law shows that the pressure is 498868 Pa, which converts to 4.92344. This result indicates how much force the gas exerts per unit area under the given conditions.

<a id="ideal_gas_temperature"></a>

#### Ideal gas law: temperature — `ideal_gas_temperature`

The temperature of an ideal gas from its pressure, volume and amount.

$$
T = \frac{PV}{nR}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | pressure | Pa |
| `V` | in | volume | m^3 |
| `n` | in | amount of gas | mol |
| `T` | out | temperature | K |
| `T_C` | out | temperature in °C | — |

**Applies when:** P, V, n > 0.

```shell
$ chemistry solve ideal_gas_temperature "n=2 mol" "V=0.05 m^3" "P=101325 Pa"
T = 304.6649094 K
T_C = 31.51490937
  by: Ideal gas law: temperature  (T = PV / nR)
  n = 2 mol
  V = 0.05 m^3
  P = 101325 Pa
  assumes: P, V, n > 0
```

**What the worked example means:** This calculation determines the absolute temperature of an ideal gas using its pressure, volume, and amount. Applying the relationship where temperature equals pressure times volume divided by the product of the amount and the gas constant shows that the gas has a temperature of 304.665 K. This result also corresponds to 31.5149 when converted to degrees Celsius, providing a clear picture of the thermal state based on the mechanical variables provided.

<a id="ideal_gas_volume"></a>

#### Ideal gas law: volume — `ideal_gas_volume`

The volume of an ideal gas from its amount, pressure and temperature.

$$
V = \frac{nRT}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | amount of gas | mol |
| `P` | in | pressure | Pa |
| `T` | in | temperature | K |
| `V` | out | volume | m^3 |
| `V_L` | out | volume in litres | — |

**Applies when:** n, P, T > 0 (T in kelvin).

```shell
$ chemistry solve ideal_gas_volume "n=1 mol" "P=101325 Pa" "T=273.15 K"
V = 0.02241396955 m^3
V_L = 22.41396955
  by: Ideal gas law: volume  (V = nRT / P)
  n = 1 mol
  P = 101325 Pa
  T = 273.15 K
  assumes: n, P, T > 0 (T in kelvin)
```

**What the worked example means:** This calculation determines the volume of an ideal gas using the amount, pressure, and temperature. Applying the formula V equals nRT divided by P shows that with an amount of 1 mol, a pressure of 101325 Pa, and a temperature of 273.15 K, the resulting volume is 0.022414 m^3. This value represents the space occupied by the gas under these specific conditions, equivalent to 22.414 in litres.

<a id="ideal_gas_mass"></a>

#### Mass of a gas sample (ideal gas) — `ideal_gas_mass`

The mass of a named gas that fills a volume at a given pressure and temperature.

$$
m = \frac{PV}{RT}\,M
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the gas | — |
| `P` | in | pressure | Pa |
| `V` | in | volume | m^3 |
| `T` | in | temperature | K |
| `m` | out | mass of gas | kg |
| `n` | out | amount of gas | mol |

**Applies when:** P, V, T > 0.

```shell
$ chemistry solve ideal_gas_mass formula=CO2 "V=0.005 m^3" "P=101325 Pa" "T=300 K"
m = 0.008938665331 kg
n = 0.2031099396 mol
  by: Mass of a gas sample (ideal gas)  (n = PV/RT, then m = nM)
  V = 0.005 m^3
  P = 101325 Pa
  T = 300 K
  formula = CO2
  assumes: P, V, T > 0
```

**What the worked example means:** This calculation determines the mass of a specific gas sample based on its pressure, volume, and temperature. Applying the ideal gas law to find the number of moles uses the molar mass derived from the chemical formula to convert that amount into physical mass. The result shows that the sample contains 0.20311 mol of substance with a total mass of 0.00893867 kg.

<a id="mean_speed"></a>

#### Mean speed of gas molecules — `mean_speed`

The mean (average) speed of the molecules of an ideal gas (Maxwell-Boltzmann distribution).

$$
\bar{u} = \sqrt{\frac{8RT}{\pi M}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the gas | — |
| `T` | in | temperature | K |
| `u_mean` | out | mean speed | m/s |

**Applies when:** T > 0 K; ideal gas.

```shell
$ chemistry solve mean_speed formula=CO2 "T=300 K"
u_mean = 379.9067648 m/s
  by: Mean speed of gas molecules  (mean of the Maxwell-Boltzmann speed distribution)
  T = 300 K
  formula = CO2
  assumes: T > 0 K; ideal gas
```

**What the worked example means:** This calculation determines the average speed of molecules in an ideal gas using the Maxwell-Boltzmann distribution. It takes the temperature 300 K and the molar mass derived from the chemical formula CO2 as inputs. The resulting mean speed, 379.907 m/s, represents how fast the gas particles are moving on average under these conditions.

<a id="molar_mass_from_gas_sample"></a>

#### Molar mass from a gas sample — `molar_mass_from_gas_sample`

The molar mass of a gas from a sample's mass, volume, pressure and temperature.

$$
M = \frac{mRT}{PV}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the sample | kg |
| `V` | in | volume | m^3 |
| `P` | in | pressure | Pa |
| `T` | in | temperature | K |
| `M` | out | molar mass | kg/mol |
| `M_g` | out | molar mass in g/mol | — |

**Applies when:** m, V, P, T > 0.

```shell
$ chemistry solve molar_mass_from_gas_sample "m=0.001 kg" "V=0.0005 m^3" "P=101325 Pa" "T=300 K"
M = 0.04923441965 kg/mol
M_g = 49.23441965
  by: Molar mass from a gas sample  (M = mRT / PV)
  m = 0.001 kg
  V = 0.0005 m^3
  P = 101325 Pa
  T = 300 K
  assumes: m, V, P, T > 0
```

**What the worked example means:** This calculation determines the molar mass of a gas sample using its measured mass, volume, pressure, and temperature. Applying the ideal gas law relationship shows that the molar mass is 0.0492344 kg/mol, which is equivalent to 49.2344. This result allows for the identification of the substance based on its molecular weight derived from experimental data.

<a id="molar_mass_from_gas_density"></a>

#### Molar mass from gas density — `molar_mass_from_gas_density`

The molar mass of a gas from its density at a known pressure and temperature.

$$
M = \frac{\rho R T}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | gas density | kg/m^3 |
| `P` | in | pressure | Pa |
| `T` | in | temperature | K |
| `M` | out | molar mass | kg/mol |
| `M_g` | out | molar mass in g/mol | — |

**Applies when:** rho, P, T > 0.

```shell
$ chemistry solve molar_mass_from_gas_density "rho=1.25 kg/m^3" "P=101325 Pa" "T=273 K"
M = 0.02800207618 kg/mol
M_g = 28.00207618
  by: Molar mass from gas density  (M = rho R T / P)
  rho = 1.25 kg/m^3
  P = 101325 Pa
  T = 273 K
  assumes: rho, P, T > 0
```

**What the worked example means:** This calculation determines the molar mass of a gas using its measured density, pressure, and temperature. Applying the ideal gas law rearrangement M equals rho R T over P shows that the substance has a molar mass of 0.0280021 kg/mol in kilograms per mole, which converts to 28.0021 in grams per mole. This result allows for the identification or characterization of the gas based on its physical properties under the specified conditions.

<a id="partial_pressure"></a>

#### Partial pressure (Dalton's law) — `partial_pressure`

The partial pressure of a gas in a mixture from its mole fraction and the total pressure.

$$
p_i = x_i P_{\text{total}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | mole fraction of the gas | — |
| `P_total` | in | total pressure | Pa |
| `p` | out | partial pressure | Pa |

**Applies when:** 0 < x <= 1; P_total > 0.

```shell
$ chemistry solve partial_pressure x=0.21 "P_total=101325 Pa"
p = 21278.25 Pa
  by: Partial pressure (Dalton's law)  (p = x P_total)
  x = 0.21
  P_total = 101325 Pa
  assumes: 0 < x <= 1; P_total > 0
```

**What the worked example means:** This calculation determines the partial pressure of a specific gas in a mixture by multiplying its mole fraction by the total pressure. With a mole fraction of 0.21 and a total pressure of 101325 Pa, the resulting partial pressure is 21278.2 Pa. This value represents the contribution of that single gas component to the overall pressure exerted by the mixture, assuming ideal behavior.

<a id="dalton_mixture"></a>

#### Partial pressures in a gas mixture — `dalton_mixture`

Mole fractions and partial pressures of every gas in a mixture from the amounts and the total pressure.

$$
x_i = \frac{n_i}{\sum_j n_j},\quad p_i = x_i P
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `moles` | in | amount of each gas (mol), e.g. '2.0, 1.0, 0.5' | — |
| `P_total` | in | total pressure | Pa |
| `partial_pressures` | out | partial pressure of each gas, in the order given | Pa |
| `mole_fractions` | out | mole fraction of each gas | — |

**Applies when:** every amount > 0; P_total > 0.

```shell
$ chemistry solve dalton_mixture "moles=[2, 1, 0.5]" "P_total=354637.5 Pa"
partial_pressures = [202650.0,101325.0,50662.5] Pa
mole_fractions = [0.5714285714285714,0.2857142857142857,0.14285714285714285]
  by: Partial pressures in a gas mixture  (mole fractions from the amounts, times the total pressure)
  P_total = 354637.5 Pa
  assumes: every amount > 0; P_total > 0
```

**What the worked example means:** This calculation determines the partial pressure for every component in a gas mixture. It uses the amount of each gas and the total pressure to find the mole fraction, which represents the proportion of that gas relative to the whole mixture. The partial pressure is then derived by multiplying this mole fraction by the total pressure. For the given inputs, the resulting partial pressures are [202650, 101325, 50662.5] Pa and the corresponding mole fractions are [0.571429, 0.285714, 0.142857]. This method assumes ideal behavior where gases do not interact significantly with each other.

<a id="rms_speed"></a>

#### Root-mean-square speed of gas molecules — `rms_speed`

The root-mean-square speed of the molecules of an ideal gas at a temperature.

$$
u_{\text{rms}} = \sqrt{\frac{3RT}{M}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the gas | — |
| `T` | in | temperature | K |
| `u_rms` | out | root-mean-square speed | m/s |

**Applies when:** T > 0 K; ideal gas.

```shell
$ chemistry solve rms_speed formula=N2 "T=300 K"
u_rms = 516.8336537 m/s
  by: Root-mean-square speed of gas molecules  (u_rms = sqrt(3RT/M), M in kg/mol)
  T = 300 K
  formula = N2
  assumes: T > 0 K; ideal gas
```

**What the worked example means:** This calculation determines the root-mean-square speed of gas molecules using the formula involving temperature and molar mass. Given a temperature of 300 K, the resulting speed is 516.834 m/s. This value represents the square root of the average of the squares of the molecular speeds, providing a measure of the typical kinetic energy of the gas particles at that specific thermal state.

<a id="van_der_waals_pressure"></a>

#### Van der Waals equation: pressure — `van_der_waals_pressure`

The pressure of a real gas from the van der Waals equation (and the ideal-gas pressure for comparison).

$$
P = \frac{nRT}{V - nb} - \frac{a n^2}{V^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | amount of gas | mol |
| `V` | in | volume | m^3 |
| `T` | in | temperature | K |
| `a` | in | van der Waals a | Pa*m^6/mol^2 |
| `b` | in | van der Waals b | m^3/mol |
| `P` | out | van der Waals pressure | Pa |
| `P_ideal` | out | ideal-gas pressure | Pa |
| `P_atm` | out | van der Waals pressure in atm | — |

**Applies when:** n, V, T, a, b > 0; V > n b.

```shell
$ chemistry solve van_der_waals_pressure "a=0.368823 Pa*m^6/mol^2" "b=4.267e-05 m^3/mol" "n=2 mol" "V=0.005 m^3" "T=300 K"
P = 956048.888 Pa
P_ideal = 997735.5142 Pa
P_atm = 9.435468916
  by: Van der Waals equation: pressure  (the van der Waals equation solved for P)
  a = 0.368823 Pa*m^6/mol^2
  b = 4.267e-05 m^3/mol
  n = 2 mol
  V = 0.005 m^3
  T = 300 K
  assumes: n, V, T, a, b > 0; V > n b
```

**What the worked example means:** This calculation determines the pressure of a real gas using the van der Waals equation, which adjusts for molecular volume and intermolecular attractions. Using an amount of gas of 2 mol, a volume of 0.005 m^3, and a temperature of 300 K, along with the specific constants 0.368823 Pa*m^6/mol^2 and 4.267e-05 m^3/mol, the resulting van der Waals pressure is 956049 Pa. For comparison, the ideal-gas pressure under these same conditions is 997736 Pa. The difference between these two values highlights the impact of non-ideal behavior in the gas.

### Kinetics (10)

<a id="activation_energy"></a>

#### Activation energy from two rate constants — `activation_energy`

The Arrhenius activation energy from rate constants measured at two temperatures.

$$
E_a = \frac{R\ln(k_2/k_1)}{1/T_1 - 1/T_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k1` | in | rate constant at T1 | — |
| `T1` | in | first temperature | K |
| `k2` | in | rate constant at T2 | — |
| `T2` | in | second temperature | K |
| `Ea` | out | activation energy | J/mol |
| `Ea_kJ` | out | in kJ/mol | — |

**Applies when:** k1, k2 > 0; T1 != T2.

```shell
$ chemistry solve activation_energy k1=0.001 "T1=300 K" k2=0.005 "T2=320 K"
Ea = 64231.73452 J/mol
Ea_kJ = 64.23173452
  by: Activation energy from two rate constants  (two-point Arrhenius equation solved for Ea)
  k1 = 0.001
  T1 = 300 K
  k2 = 0.005
  T2 = 320 K
  assumes: k1, k2 > 0; T1 != T2
```

**What the worked example means:** This calculation determines the Arrhenius activation energy using rate constants measured at two distinct temperatures. By applying the two-point form of the Arrhenius equation, the method derives the energy barrier from the ratio of the rate constants and the difference in inverse temperatures. The resulting value, expressed as 64231.7 J/mol, quantifies the minimum energy required for reactant molecules to undergo successful collisions leading to product formation.

<a id="arrhenius_rate_constant"></a>

#### Arrhenius equation: rate constant — `arrhenius_rate_constant`

The rate constant from the pre-exponential factor, the activation energy and the temperature.

$$
k = A\,e^{-E_a/RT}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | pre-exponential factor (units of k) | — |
| `Ea` | in | activation energy | J/mol |
| `T` | in | temperature | K |
| `k` | out | rate constant (units of A) | — |

**Applies when:** A, Ea, T > 0.

```shell
$ chemistry solve arrhenius_rate_constant A=1e+13 "Ea=100000 J/mol" "T=500 K"
k = 357.4999422
  by: Arrhenius equation: rate constant  (k = A exp(-Ea/RT))
  A = 1e+13
  Ea = 100000 J/mol
  T = 500 K
  assumes: A, Ea, T > 0
```

**What the worked example means:** This calculation determines the rate constant of a chemical reaction using the Arrhenius equation. It takes the pre-exponential factor, which represents the frequency of collisions with correct orientation, along with the activation energy required to initiate the reaction and the absolute temperature. The result, 357.5, indicates how rapidly the reaction proceeds under these specific conditions. A higher value signifies a faster reaction rate.

<a id="arrhenius_k2"></a>

#### Arrhenius equation: rate constant at another temperature — `arrhenius_k2`

The rate constant at a second temperature from the rate constant at a first one and the activation energy.

$$
\ln\frac{k_2}{k_1} = -\frac{E_a}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k1` | in | rate constant at T1 (any units) | — |
| `T1` | in | first temperature | K |
| `T2` | in | second temperature | K |
| `Ea` | in | activation energy | J/mol |
| `k2` | out | rate constant at T2 (units of k1) | — |
| `ratio` | out | k2 / k1 | — |

**Applies when:** k1, T1, T2, Ea > 0; Ea independent of T.

```shell
$ chemistry solve arrhenius_k2 k1=0.0012 "T1=300 K" "Ea=75000 J/mol" "T2=350 K"
k2 = 0.08803749544
ratio = 73.36457954
  by: Arrhenius equation: rate constant at another temperature  (two-point Arrhenius equation)
  k1 = 0.0012
  T1 = 300 K
  Ea = 75000 J/mol
  T2 = 350 K
  assumes: k1, T1, T2, Ea > 0; Ea independent of T
```

**What the worked example means:** This calculation determines the new rate constant, 0.0880375, using the two-point Arrhenius equation. It relies on the known rate constant 0.0012, the activation energy 75000 J/mol, and the temperatures 300 K and 350 K. The result shows that the reaction speed changes by a factor of 73.3646 when moving from the first temperature to the second.

<a id="consecutive_first_order"></a>

#### Consecutive first-order reactions (A -> B -> C) — `consecutive_first_order`

Concentrations of the reactant and the intermediate of A -> B -> C (both steps first order) at a time.

$$
[B] = \frac{k_1 [A]_0}{k_2 - k_1}\left(e^{-k_1 t} - e^{-k_2 t}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | initial concentration of A (B and C start at 0) | mol/m^3 |
| `k1` | in | rate constant of A -> B | 1/s |
| `k2` | in | rate constant of B -> C | 1/s |
| `t` | in | elapsed time | s |
| `B` | out | intermediate concentration | mol/m^3 |
| `A` | out | reactant | mol/m^3 |
| `C` | out | product | mol/m^3 |

**Applies when:** both steps first order; B and C absent at t = 0.

```shell
$ chemistry solve consecutive_first_order "k1=0.1 1/s" "k2=0.05 1/s" "A0=1000 mol/m^3" "t=10 s"
B = 477.3024371 mol/m^3
A = 367.8794412 mol/m^3
C = 154.8181217 mol/m^3
  by: Consecutive first-order reactions (A -> B -> C)  (analytic solution of the linear rate equations (the k1 = k2 limit included))
  k1 = 0.1 1/s
  k2 = 0.05 1/s
  A0 = 1000 mol/m^3
  t = 10 s
  assumes: both steps first order; B and C absent at t = 0
```

**What the worked example means:** This calculation determines the concentrations of all species in a sequential first-order reaction chain where A converts to B, which then converts to C. Starting with an initial reactant concentration of 1000 mol/m^3, the model computes that at elapsed time 10 s, the remaining reactant A is 367.879 mol/m^3, the accumulated intermediate B is 477.302 mol/m^3, and the final product C is 154.818 mol/m^3. These values reflect how the intermediate builds up as it is formed from A but decays as it turns into C, governed by the rate constants 0.1 1/s for the first step and 0.05 1/s for the second step.

<a id="first_order_concentration"></a>

#### First-order integrated rate law — `first_order_concentration`

The concentration of a reactant after a time in a first-order reaction.

$$
[A] = [A]_0\, e^{-kt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | initial concentration | mol/m^3 |
| `k` | in | first-order rate constant | 1/s |
| `t` | in | elapsed time | s |
| `A` | out | concentration at time t | mol/m^3 |
| `A_molL` | out | in mol/L | — |
| `fraction_left` | out | fraction remaining | — |

**Applies when:** [A]0, k, t > 0.

```shell
$ chemistry solve first_order_concentration "k=0.015 1/s" "A0=800 mol/m^3" "t=60 s"
A = 325.2557278 mol/m^3
A_molL = 0.3252557278
fraction_left = 0.4065696597
  by: First-order integrated rate law  ([A] = [A]0 exp(-k t))
  k = 0.015 1/s
  A0 = 800 mol/m^3
  t = 60 s
  assumes: [A]0, k, t > 0
```

**What the worked example means:** This calculation determines the remaining concentration of a reactant in a first-order reaction after a specific elapsed time. Starting from an initial concentration of 800 mol/m^3, the system decays exponentially according to the rate constant 0.015 1/s over the duration 60 s. The resulting concentration is 325.256 mol/m^3, which corresponds to 0.325256 when converted, and represents 0.40657 of the original amount still present.

<a id="first_order_half_life"></a>

#### Half-life of a first-order reaction — `first_order_half_life`

The half-life of a first-order reaction from its rate constant.

$$
t_{1/2} = \frac{\ln 2}{k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | first-order rate constant | 1/s |
| `t_half` | out | half-life | s |

**Applies when:** k > 0.

```shell
$ chemistry solve first_order_half_life "k=0.025 1/s"
t_half = 27.72588722 s
  by: Half-life of a first-order reaction  (t1/2 = ln 2 / k)
  k = 0.025 1/s
  assumes: k > 0
```

**What the worked example means:** This calculation determines the time required for the reactant concentration to decrease by half, known as the half-life. It uses the first-order rate constant 0.025 1/s in the relationship where half-life equals the natural logarithm of two divided by the rate constant. The resulting half-life is 27.7259 s, indicating how quickly the substance decays under these conditions.

<a id="reaction_order_from_rates"></a>

#### Reaction order from initial rates — `reaction_order_from_rates`

The order with respect to one reactant from two initial-rate experiments (other concentrations fixed).

$$
n = \frac{\ln(r_2/r_1)}{\ln(c_2/c_1)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c1` | in | concentration in experiment 1 | mol/m^3 |
| `r1` | in | initial rate in experiment 1 | mol/(m^3*s) |
| `c2` | in | concentration in experiment 2 | mol/m^3 |
| `r2` | in | initial rate in experiment 2 | mol/(m^3*s) |
| `order` | out | reaction order | — |

**Applies when:** c1 != c2.

```shell
$ chemistry solve reaction_order_from_rates "c1=100 mol/m^3" "r1=2 mol/(m^3*s)" "c2=200 mol/m^3" "r2=8 mol/(m^3*s)"
order = 2
  by: Reaction order from initial rates  (ratio of the two rate laws)
  c1 = 100 mol/m^3
  r1 = 2 mol/(m^3*s)
  c2 = 200 mol/m^3
  r2 = 8 mol/(m^3*s)
  assumes: c1 != c2
```

**What the worked example means:** This calculation determines the reaction order with respect to a specific reactant by comparing two initial-rate experiments. Using the natural logarithm of the rate ratio, formed by dividing the second experiment's initial rate 8 mol/(m^3*s) by the first experiment's initial rate 2 mol/(m^3*s), and dividing it by the natural logarithm of the concentration ratio, formed by dividing the second experiment's concentration 200 mol/m^3 by the first experiment's concentration 100 mol/m^3, gives the order 2. This value indicates how sensitive the reaction speed is to changes in that reactant's concentration.

<a id="second_order_concentration"></a>

#### Second-order integrated rate law — `second_order_concentration`

The concentration of a reactant after a time in a second-order reaction (rate = k[A]^2).

$$
\frac{1}{[A]} = \frac{1}{[A]_0} + k t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | initial concentration | mol/m^3 |
| `k` | in | second-order rate constant | m^3/(mol*s) |
| `t` | in | elapsed time | s |
| `A` | out | concentration at time t | mol/m^3 |
| `A_molL` | out | in mol/L | — |

**Applies when:** [A]0, k, t > 0.

```shell
$ chemistry solve second_order_concentration "k=0.0005 m^3/(mol*s)" "A0=200 mol/m^3" "t=30 s"
A = 50 mol/m^3
A_molL = 0.05
  by: Second-order integrated rate law  (1/[A] = 1/[A]0 + k t)
  k = 0.0005 m^3/(mol*s)
  A0 = 200 mol/m^3
  t = 30 s
  assumes: [A]0, k, t > 0
```

**What the worked example means:** This calculation determines the remaining concentration of a single reactant in a second-order reaction using the integrated rate law. Given an initial concentration of 200 mol/m^3, a rate constant of 0.0005 m^3/(mol*s), and an elapsed time of 30 s, the model computes the current concentration as 50 mol/m^3. This result indicates how much reactant remains after the specified duration, reflecting the inverse relationship between concentration and time for this reaction order.

<a id="first_order_time"></a>

#### Time to reach a concentration (first order) — `first_order_time`

How long a first-order reaction takes to bring a reactant from one concentration to another.

$$
t = \frac{1}{k}\ln\frac{[A]_0}{[A]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | initial concentration | mol/m^3 |
| `A` | in | final concentration | mol/m^3 |
| `k` | in | first-order rate constant | 1/s |
| `t` | out | time needed | s |

**Applies when:** 0 < [A] < [A]0; k > 0.

```shell
$ chemistry solve first_order_time "k=0.03 1/s" "A0=500 mol/m^3" "A=100 mol/m^3"
t = 53.64793041 s
  by: Time to reach a concentration (first order)  (t = ln([A]0/[A]) / k)
  k = 0.03 1/s
  A0 = 500 mol/m^3
  A = 100 mol/m^3
  assumes: 0 < [A] < [A]0; k > 0
```

**What the worked example means:** This calculation determines how long a first-order reaction takes to reduce the reactant concentration from its initial value of 500 mol/m^3 down to the final target value of 100 mol/m^3, given a rate constant of 0.03 1/s. The result, 53.6479 s, represents the elapsed time in seconds needed for this specific change in concentration. It reflects the exponential decay nature of first-order kinetics where the rate depends solely on the current amount of reactant present.

<a id="zero_order_concentration"></a>

#### Zero-order integrated rate law — `zero_order_concentration`

The concentration of a reactant after a time in a zero-order reaction.

$$
[A] = [A]_0 - k t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A0` | in | initial concentration | mol/m^3 |
| `k` | in | zero-order rate constant | mol/(m^3*s) |
| `t` | in | elapsed time | s |
| `A` | out | concentration at time t | mol/m^3 |
| `A_molL` | out | in mol/L | — |

**Applies when:** [A]0, k, t > 0; [A] stays >= 0.

```shell
$ chemistry solve zero_order_concentration "k=2 mol/(m^3*s)" "A0=500 mol/m^3" "t=100 s"
A = 300 mol/m^3
A_molL = 0.3
  by: Zero-order integrated rate law  ([A] = [A]0 - k t)
  k = 2 mol/(m^3*s)
  A0 = 500 mol/m^3
  t = 100 s
  assumes: [A]0, k, t > 0; [A] stays >= 0
```

**What the worked example means:** This calculation determines the reactant concentration at a specific time using the zero-order integrated rate law. Starting from an initial concentration of 500 mol/m^3, the system consumes the reactant at a constant rate defined by the rate constant 2 mol/(m^3*s) over the elapsed time 100 s. The resulting concentration is 300 mol/m^3, which converts to 0.3. This linear decrease reflects that the reaction speed does not depend on how much reactant is left, a behavior typical when active sites are fully saturated.

### Molecular properties (1)

<a id="degree_of_unsaturation"></a>

#### Degree of unsaturation — `degree_of_unsaturation`

The number of rings plus pi bonds implied by a molecular formula.

$$
\mathrm{DoU} = \frac{2C + 2 + N - H - X}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | molecular formula | — |
| `dou` | out | degree of unsaturation (rings + pi bonds) | — |

**Applies when:** a neutral organic formula.

```shell
$ chemistry solve degree_of_unsaturation formula=C6H6
dou = 4
  by: Degree of unsaturation  ((2C + 2 + N - H - X)/2 (Si as C, P and B as N, O and S ignored))
  formula = C6H6
  assumes: a neutral organic formula
```

**What the worked example means:** This calculation determines the degree of unsaturation from the molecular formula C6H6. The result, 4, represents the sum of rings and pi bonds implied by the atom counts. It helps predict structural features such as double bonds, triple bonds, or cyclic structures without needing a detailed diagram.

### Solutions (12)

<a id="boiling_point_elevation"></a>

#### Boiling-point elevation — `boiling_point_elevation`

The change of the boiling point of a solvent caused by a solute (dT = i K b).

$$
\Delta T = i K b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `i` | in | van 't Hoff factor | — |
| `K` | in | ebullioscopic constant | K*kg/mol |
| `b` | in | molality of the solute | mol/kg |
| `dT` | out | boiling-point change | K |

**Applies when:** i, K, b > 0; dilute solution.

```shell
$ chemistry solve boiling_point_elevation "K=0.512 K*kg/mol" "b=0.5 mol/kg" i=1
dT = 0.256 K
  by: Boiling-point elevation  (dT = i K b)
  K = 0.512 K*kg/mol
  b = 0.5 mol/kg
  i = 1
  assumes: i, K, b > 0; dilute solution
```

**What the worked example means:** This calculation determines the change in a solvent's boiling temperature due to a dissolved solute. Using the van 't Hoff factor of 1, the ebullioscopic constant of 0.512 K*kg/mol, and the molality of 0.5 mol/kg, the resulting boiling-point elevation is computed as 0.256 K. This value represents how much higher the solution boils compared to the pure solvent.

<a id="dilution_final_concentration"></a>

#### Dilution: final concentration — `dilution_final_concentration`

The concentration after diluting a volume of solution to a larger volume (c1V1 = c2V2).

$$
c_2 = \frac{c_1 V_1}{V_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c1` | in | initial concentration | mol/m^3 |
| `V1` | in | initial volume | m^3 |
| `V2` | in | final volume | m^3 |
| `c2` | out | final concentration | mol/m^3 |
| `c2_molL` | out | in mol/L | — |

**Applies when:** c1, V1, V2 > 0; V2 >= V1.

```shell
$ chemistry solve dilution_final_concentration "V2=0.0005 m^3" "V1=2.5e-05 m^3" "c1=6000 mol/m^3"
c2 = 300 mol/m^3
c2_molL = 0.3
  by: Dilution: final concentration  (moles of solute are conserved: c1 V1 = c2 V2)
  V2 = 0.0005 m^3
  V1 = 2.5e-05 m^3
  c1 = 6000 mol/m^3
  assumes: c1, V1, V2 > 0; V2 >= V1
```

**What the worked example means:** This calculation determines the new molarity of a solution after it has been diluted with solvent. It uses the principle that the amount of solute remains constant, so the product of initial concentration and volume equals the product of final concentration and volume. Given an initial concentration of 6000 mol/m^3 and an initial volume of 2.5e-05 m^3, expanding the total volume to 0.0005 m^3 results in a final concentration of 300 mol/m^3, which is equivalent to 0.3 when expressed in moles per liter.

<a id="dilution_volume"></a>

#### Dilution: volume of stock needed — `dilution_volume`

The volume of a stock solution needed to prepare a volume of a more dilute solution.

$$
V_1 = \frac{c_2 V_2}{c_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c1` | in | stock concentration | mol/m^3 |
| `c2` | in | target concentration | mol/m^3 |
| `V2` | in | target volume | m^3 |
| `V1` | out | volume of stock solution | m^3 |
| `V1_mL` | out | in mL | — |

**Applies when:** c1 >= c2 > 0; V2 > 0.

```shell
$ chemistry solve dilution_volume "c1=12000 mol/m^3" "c2=100 mol/m^3" "V2=0.0005 m^3"
V1 = 4.166666667e-06 m^3
V1_mL = 4.166666667
  by: Dilution: volume of stock needed  (c1 V1 = c2 V2 solved for V1)
  c1 = 12000 mol/m^3
  c2 = 100 mol/m^3
  V2 = 0.0005 m^3
  assumes: c1 >= c2 > 0; V2 > 0
```

**What the worked example means:** This calculation determines the volume of a concentrated stock solution needed to prepare a specific target volume at a lower concentration. Applying the dilution principle solves for the initial volume using the known stock concentration, target concentration, and target volume. The result indicates that 4.16667e-06 m^3 of the stock solution must be measured and diluted to achieve the desired final state.

<a id="freezing_point_depression"></a>

#### Freezing-point depression — `freezing_point_depression`

The change of the freezing point of a solvent caused by a solute (dT = i K b).

$$
\Delta T = i K b
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `i` | in | van 't Hoff factor | — |
| `K` | in | cryoscopic constant | K*kg/mol |
| `b` | in | molality of the solute | mol/kg |
| `dT` | out | freezing-point change | K |

**Applies when:** i, K, b > 0; dilute solution.

```shell
$ chemistry solve freezing_point_depression "K=1.86 K*kg/mol" "b=0.5 mol/kg" i=1
dT = 0.93 K
  by: Freezing-point depression  (dT = i K b)
  K = 1.86 K*kg/mol
  b = 0.5 mol/kg
  i = 1
  assumes: i, K, b > 0; dilute solution
```

**What the worked example means:** This calculation determines the change in the freezing point of a solvent, denoted as 0.93 K, caused by dissolving a solute. The result is derived using the formula delta T equals i times K times b, where 1 represents the van 't Hoff factor indicating particle count, 1.86 K*kg/mol is the specific cryoscopic constant of the solvent, and 0.5 mol/kg is the molality of the solution. A higher value for any of these inputs leads to a greater reduction in the freezing temperature.

<a id="solute_mass_for_solution"></a>

#### Mass of solute for a solution — `solute_mass_for_solution`

The mass of a compound needed to prepare a volume of solution of a given molarity.

$$
m = c V M
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the solute | — |
| `c` | in | molar concentration | mol/m^3 |
| `V` | in | volume of solution | m^3 |
| `m` | out | mass of solute | kg |
| `m_g` | out | mass in grams | — |

**Applies when:** c, V > 0.

```shell
$ chemistry solve solute_mass_for_solution formula=NaCl "V=0.00025 m^3" "c=100 mol/m^3"
m = 0.001461075 kg
m_g = 1.461075
  by: Mass of solute for a solution  (m = c V M)
  V = 0.00025 m^3
  c = 100 mol/m^3
  formula = NaCl
  assumes: c, V > 0
```

**What the worked example means:** This calculation determines the mass of solute needed to create a solution with a specific volume and molar concentration. Multiplying the desired concentration by the target volume and the molar mass of the substance shows that 100 mol/m^3 combined with 0.00025 m^3 results in a necessary solute mass of 0.00146108 kg. This value ensures the final solution achieves the intended chemical strength for laboratory use.

<a id="molality"></a>

#### Molality of a solution — `molality`

The molality (moles of solute per kilogram of solvent) from the masses of solute and solvent.

$$
b = \frac{m_{\text{solute}} / M}{m_{\text{solvent}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the solute | — |
| `m_solute` | in | mass of solute | kg |
| `m_solvent` | in | mass of solvent | kg |
| `b` | out | molality | mol/kg |

**Applies when:** masses > 0.

```shell
$ chemistry solve molality formula=NaCl "m_solvent=0.5 kg" "m_solute=0.01 kg"
b = 0.3422137809 mol/kg
  by: Molality of a solution  (moles of solute per kilogram of solvent)
  m_solvent = 0.5 kg
  m_solute = 0.01 kg
  formula = NaCl
  assumes: masses > 0
```

**What the worked example means:** This calculation determines the molality of a solution, which represents the concentration of solute particles per kilogram of solvent. Taking the mass of the solute and dividing it by its molar mass to find the number of moles, then dividing that value by the mass of the solvent in kilograms gives the result 0.342214 mol/kg. This measure is particularly useful because it remains constant regardless of temperature changes, unlike volume-based concentrations.

<a id="molar_mass_from_freezing_point"></a>

#### Molar mass from freezing-point depression — `molar_mass_from_freezing_point`

The molar mass of a solute from the freezing-point depression it causes.

$$
M = \frac{i K_f m_{\text{solute}}}{\Delta T_f\, m_{\text{solvent}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m_solute` | in | mass of solute | kg |
| `m_solvent` | in | mass of solvent | kg |
| `K` | in | cryoscopic constant Kf | K*kg/mol |
| `dT` | in | freezing-point depression | K |
| `i` | in | van 't Hoff factor | — |
| `M` | out | molar mass of solute | kg/mol |
| `M_g` | out | in g/mol | — |

**Applies when:** all inputs > 0; dilute solution.

```shell
$ chemistry solve molar_mass_from_freezing_point "m_solvent=0.1 kg" "dT=0.372 K" "K=1.86 K*kg/mol" "m_solute=0.002 kg" i=1
M = 0.1 kg/mol
M_g = 100
  by: Molar mass from freezing-point depression  (b = dT/(i Kf); M = m_solute / (b m_solvent))
  m_solvent = 0.1 kg
  dT = 0.372 K
  K = 1.86 K*kg/mol
  m_solute = 0.002 kg
  i = 1
  assumes: all inputs > 0; dilute solution
```

**What the worked example means:** This calculation determines the molar mass of a dissolved substance by analyzing how much it lowers the freezing point of the solvent. Using the measured mass of the solute, the mass of the solvent, and the observed temperature drop, along with the solvent's specific cryoscopic constant, shows that the solute has a molar mass of 0.1 kg/mol. This value provides essential information for identifying unknown compounds based on their colligative properties in dilute solutions.

<a id="molar_mass_from_osmotic_pressure"></a>

#### Molar mass from osmotic pressure — `molar_mass_from_osmotic_pressure`

The molar mass of a solute (e.g. a protein) from the osmotic pressure of its solution.

$$
M = \frac{i m R T}{\Pi V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of solute | kg |
| `V` | in | volume of solution | m^3 |
| `Pi` | in | osmotic pressure | Pa |
| `T` | in | temperature | K |
| `i` | in | van 't Hoff factor | — |
| `M` | out | molar mass | kg/mol |
| `M_g` | out | in g/mol | — |

**Applies when:** all inputs > 0.

```shell
$ chemistry solve molar_mass_from_osmotic_pressure "m=0.001 kg" "V=0.0001 m^3" "Pi=333.3059211 Pa" "T=298.15 K" i=1
M = 74.37482723 kg/mol
M_g = 74374.82723
  by: Molar mass from osmotic pressure  (c = Pi/(i R T); M = m / (c V))
  m = 0.001 kg
  V = 0.0001 m^3
  Pi = 333.3059211 Pa
  T = 298.15 K
  i = 1
  assumes: all inputs > 0
```

**What the worked example means:** This calculation determines the molar mass of a solute, such as a protein, using its measured osmotic pressure. By applying the relationship between pressure, temperature, and concentration, the method yields a molar mass of 74.3748 kg/mol in kilograms per mole, which is equivalent to 74374.8 in grams per mole. This result reflects the mass required for one mole of the substance based on the solution's behavior under ideal conditions.

<a id="molarity_from_mass"></a>

#### Molarity from a dissolved mass — `molarity_from_mass`

The molar concentration of a solution made by dissolving a mass of a compound to a final volume.

$$
c = \frac{m}{M V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the solute | — |
| `m` | in | mass of solute | kg |
| `V` | in | volume of solution | m^3 |
| `c` | out | molar concentration | mol/m^3 |
| `c_molL` | out | concentration in mol/L | — |

**Applies when:** m, V > 0.

```shell
$ chemistry solve molarity_from_mass formula=NaCl "m=0.00585 kg" "V=0.00025 m^3"
c = 400.3901237 mol/m^3
c_molL = 0.4003901237
  by: Molarity from a dissolved mass  (moles of solute (m / M) over the volume of solution)
  m = 0.00585 kg
  V = 0.00025 m^3
  formula = NaCl
  assumes: m, V > 0
```

**What the worked example means:** This calculation determines the molar concentration of a solution by dividing the moles of solute, derived from a mass of 0.00585 kg, by the final volume of 0.00025 m^3. The result is a concentration of 400.39 mol/m^3, equivalent to 0.40039 when expressed in standard laboratory units. This method assumes complete dissolution and accounts for any volume changes in the final measurement.

<a id="mole_fraction"></a>

#### Mole fraction in a solution — `mole_fraction`

The mole fractions of solute and solvent from their masses.

$$
x_{\text{solute}} = \frac{n_{\text{solute}}}{n_{\text{solute}} + n_{\text{solvent}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula_solute` | in | formula of the solute | — |
| `m_solute` | in | mass of solute | kg |
| `formula_solvent` | in | formula of the solvent | — |
| `m_solvent` | in | mass of solvent | kg |
| `x_solute` | out | mole fraction of solute | — |
| `x_solvent` | out | mole fraction of solvent | — |

**Applies when:** masses > 0.

```shell
$ chemistry solve mole_fraction "m_solute=0.02 kg" formula_solute=C6H12O6 "m_solvent=0.1 kg" formula_solvent=H2O
x_solute = 0.01960720291
x_solvent = 0.9803927971
  by: Mole fraction in a solution  (moles of each from its mass and molar mass)
  m_solute = 0.02 kg
  m_solvent = 0.1 kg
  formula_solvent = H2O
  formula_solute = C6H12O6
  assumes: masses > 0
```

**What the worked example means:** This calculation determines the mole fractions for both components in a solution using their given masses. Converting the mass of the solute, specified as 0.02 kg, and the mass of the solvent, specified as 0.1 kg, into moles based on their respective chemical formulas gives the proportion of each substance relative to the total number of particles. The resulting mole fraction of the solute is 0.0196072, while the mole fraction of the solvent is 0.980393. These values indicate how much of the mixture consists of each component on a molecular basis, independent of their individual weights.

<a id="osmotic_pressure"></a>

#### Osmotic pressure — `osmotic_pressure`

The osmotic pressure of a dilute solution (van 't Hoff equation).

$$
\Pi = i c R T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `i` | in | van 't Hoff factor | — |
| `c` | in | molar concentration | mol/m^3 |
| `T` | in | temperature | K |
| `Pi` | out | osmotic pressure | Pa |
| `Pi_atm` | out | in atm | — |

**Applies when:** i, c, T > 0; dilute solution.

```shell
$ chemistry solve osmotic_pressure "c=100 mol/m^3" "T=298.15 K" i=1
Pi = 247895.703 Pa
Pi_atm = 2.44654037
  by: Osmotic pressure  (Pi = i c R T)
  c = 100 mol/m^3
  T = 298.15 K
  i = 1
  assumes: i, c, T > 0; dilute solution
```

**What the worked example means:** This calculation determines the osmotic pressure of a dilute solution using the van 't Hoff equation. By combining the van 't Hoff factor, molar concentration, and absolute temperature, the model computes the pressure required to prevent solvent flow across a semipermeable membrane. The resulting osmotic pressure is 247896 Pa in pascals, equivalent to 2.44654 in atmospheres. This value indicates the tendency of water to move into the solution due to solute presence, a critical concept for understanding cellular behavior and determining molecular weights in biochemical contexts.

<a id="raoult_vapor_pressure"></a>

#### Vapor pressure of a solution (Raoult's law) — `raoult_vapor_pressure`

The vapor pressure of a solvent above a solution of a nonvolatile solute, from the masses of both.

$$
P = x_{\text{solvent}} P^\circ
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula_solute` | in | formula of the solute | — |
| `m_solute` | in | mass of solute | kg |
| `formula_solvent` | in | formula of the solvent | — |
| `m_solvent` | in | mass of solvent | kg |
| `P0` | in | vapor pressure of the pure solvent | Pa |
| `P` | out | vapor pressure of the solution | Pa |
| `dP` | out | vapor pressure lowering | Pa |

**Applies when:** nonvolatile, non-dissociating solute; ideal solution.

```shell
$ chemistry solve raoult_vapor_pressure "m_solute=0.05 kg" formula_solute=C12H22O11 "m_solvent=0.5 kg" formula_solvent=H2O "P0=3173.072368 Pa"
P = 3156.460003 Pa
dP = 16.61236498 Pa
  by: Vapor pressure of a solution (Raoult's law)  (Raoult's law with the solvent mole fraction from the masses)
  m_solute = 0.05 kg
  m_solvent = 0.5 kg
  P0 = 3173.072368 Pa
  formula_solvent = H2O
  formula_solute = C12H22O11
  assumes: nonvolatile, non-dissociating solute; ideal solution
```

**What the worked example means:** This calculation determines the vapor pressure of a solvent above a solution containing a nonvolatile solute. By applying Raoult's law, the model uses the masses of the solute and solvent to find the mole fraction of the solvent. The resulting vapor pressure, 3156.46 Pa, represents the equilibrium pressure exerted by the solvent molecules escaping into the gas phase. This value is lower than the pure solvent pressure because the solute particles reduce the surface area available for evaporation. The magnitude of this reduction, known as the vapor pressure lowering, is calculated as 16.6124 Pa.

### Spectroscopy (6)

<a id="beer_lambert_absorbance"></a>

#### Beer-Lambert law: absorbance — `beer_lambert_absorbance`

The absorbance of a solution from its molar absorptivity, path length and concentration.

$$
A = \varepsilon l c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps` | in | molar absorptivity | m^2/mol |
| `l` | in | path length | m |
| `c` | in | concentration | mol/m^3 |
| `A` | out | absorbance | — |
| `T_percent` | out | percent transmittance | — |

**Applies when:** dilute solution, monochromatic light.

```shell
$ chemistry solve beer_lambert_absorbance "l=0.01 m" "c=0.02 mol/m^3" "eps=1500 m^2/mol"
A = 0.3
T_percent = 50.11872336
  by: Beer-Lambert law: absorbance  (A = epsilon l c)
  l = 0.01 m
  c = 0.02 mol/m^3
  eps = 1500 m^2/mol
  assumes: dilute solution, monochromatic light
```

**What the worked example means:** This calculation determines the optical properties of a solution using the Beer-Lambert law. Multiplying the molar absorptivity, path length, and concentration gives the absorbance value of 0.3. This dimensionless quantity indicates how much light is blocked by the sample. The result is then converted into percent transmittance, yielding 50.1187, which represents the fraction of incident light that passes through the solution.

<a id="beer_lambert_concentration"></a>

#### Beer-Lambert law: concentration — `beer_lambert_concentration`

The concentration of a solution from its absorbance, molar absorptivity and path length.

$$
c = \frac{A}{\varepsilon l}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | absorbance | — |
| `eps` | in | molar absorptivity | m^2/mol |
| `l` | in | path length | m |
| `c` | out | concentration | mol/m^3 |
| `c_molL` | out | in mol/L | — |

**Applies when:** dilute solution.

```shell
$ chemistry solve beer_lambert_concentration A=0.45 "l=0.01 m" "eps=1250 m^2/mol"
c = 0.036 mol/m^3
c_molL = 3.6e-05
  by: Beer-Lambert law: concentration  (c = A / (epsilon l))
  A = 0.45
  l = 0.01 m
  eps = 1250 m^2/mol
  assumes: dilute solution
```

**What the worked example means:** This calculation determines the concentration of a solute using the Beer-Lambert law. Dividing the measured absorbance by the product of molar absorptivity and path length shows that the concentration is 0.036 mol/m^3, which is equivalent to 3.6e-05. This result assumes the solution is dilute enough for linear absorption behavior to hold.

<a id="photon_energy"></a>

#### Energy of a photon from its wavelength — `photon_energy`

The energy (per photon and per mole of photons) and frequency of light of a given wavelength.

$$
E = \frac{hc}{\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `lam` | in | wavelength | m |
| `E` | out | energy per photon | J |
| `E_eV` | out | in eV | — |
| `E_molar` | out | energy per mole of photons | J/mol |
| `nu` | out | frequency | Hz |

**Applies when:** lambda > 0.

```shell
$ chemistry solve photon_energy "lam=5e-07 m"
E = 3.972891714e-19 J
E_eV = 2.479683969
E_molar = 239253.1313 J/mol
nu = 5.99584916e+14 Hz
  by: Energy of a photon from its wavelength  (E = h c / lambda)
  lam = 5e-07 m
  assumes: lambda > 0
```

**What the worked example means:** This calculation determines the energy, frequency, and molar energy for light at a specific wavelength. Given an input wavelength of 5e-07 m, the resulting energy per photon is 3.97289e-19 J, which corresponds to 2.47968. The frequency of this light is calculated as 5.99585e+14 Hz, and the total energy for one mole of such photons is 239253 J/mol. These values are crucial for understanding interactions in photochemistry and spectroscopy.

<a id="hydrogen_transition_wavelength"></a>

#### Hydrogen atom transition wavelength (Rydberg) — `hydrogen_transition_wavelength`

The wavelength of the photon emitted or absorbed in a transition between two levels of hydrogen.

$$
\frac{1}{\lambda} = R_H\left(\frac{1}{n_1^2} - \frac{1}{n_2^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n_upper` | in | upper level | — |
| `n_lower` | in | lower level | — |
| `lam` | out | wavelength | m |
| `lam_nm` | out | in nm | — |
| `E` | out | photon energy | J |

**Applies when:** hydrogen atom.

```shell
$ chemistry solve hydrogen_transition_wavelength n_upper=3 n_lower=2
lam = 6.564696063e-07 m
lam_nm = 656.4696063
E = 3.025952516e-19 J
  by: Hydrogen atom transition wavelength (Rydberg)  (Rydberg formula with the hydrogen Rydberg constant (reduced-mass corrected))
  assumes: hydrogen atom
```

**What the worked example means:** This calculation determines the wavelength of light emitted or absorbed when an electron in a hydrogen atom moves between two specific energy levels. Using the Rydberg formula with the reduced-mass corrected constant, it computes the precise spectral line from the upper level at 3 and the lower level at 2. The resulting wavelength is 6.5647e-07 m, which converts to 656.47 nanometers, representing a photon energy of 3.02595e-19 J joules.

<a id="vibrational_wavenumber"></a>

#### Vibrational wavenumber of a diatomic molecule — `vibrational_wavenumber`

The harmonic vibrational wavenumber of a diatomic molecule from its bond force constant.

$$
\tilde{\nu} = \frac{1}{2\pi c}\sqrt{\frac{k}{\mu}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `molecule` | in | diatomic molecule, e.g. HCl or CO | — |
| `k` | in | bond force constant | N/m |
| `wavenumber` | out | wavenumber | 1/m |
| `wavenumber_cm` | out | in cm^-1 | — |
| `nu` | out | vibrational frequency | Hz |

**Applies when:** harmonic approximation; most abundant isotopes (1H, 12C, 16O, 35Cl ...).

```shell
$ chemistry solve vibrational_wavenumber molecule=HCl "k=516 N/m"
wavenumber = 299003.9985 1/m
wavenumber_cm = 2990.039985
nu = 8.963914367e+13 Hz
  by: Vibrational wavenumber of a diatomic molecule  (harmonic oscillator with the reduced mass of the most abundant isotopes)
  k = 516 N/m
  molecule = HCl
  assumes: harmonic approximation; most abundant isotopes (1H, 12C, 16O, 35Cl ...)
```

In the worked example, molecule = HCl, k = 516 N/m go through the equation above and give wavenumber = 299003.9985 1/m, wavenumber_cm = 2990.039985, nu = 8.963914367e+13 Hz. It applies only when harmonic approximation; most abundant isotopes (1H, 12C, 16O, 35Cl ...): outside that it says what is wrong instead of returning a number.

<a id="photon_wavelength"></a>

#### Wavelength of a photon from its energy — `photon_wavelength`

The wavelength of a photon of a given energy.

$$
\lambda = \frac{hc}{E}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E` | in | photon energy | J |
| `lam` | out | wavelength | m |
| `lam_nm` | out | in nm | — |

**Applies when:** E > 0.

```shell
$ chemistry solve photon_wavelength "E=3e-19 J"
lam = 6.62148619e-07 m
lam_nm = 662.148619
  by: Wavelength of a photon from its energy  (lambda = h c / E)
  E = 3e-19 J
  assumes: E > 0
```

**What the worked example means:** This calculation determines the wavelength of a single photon based on its energy. Using the relationship where wavelength equals Planck's constant times the speed of light divided by energy, the input value 3e-19 J yields a result of 6.62149e-07 m. This corresponds to 662.149, which helps identify the spectral region of the light.

### Stoichiometry (15)

<a id="atoms_of_element"></a>

#### Atoms of an element in a mass of compound — `atoms_of_element`

How many atoms of one element a given mass of a compound contains.

$$
N_X = \frac{m}{M} \, \nu_X \, N_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula of the compound | — |
| `element` | in | the element (symbol or name) | — |
| `m` | in | mass of the compound | kg |
| `N_atoms` | out | number of atoms of the element | — |
| `n_element` | out | amount of the element | mol |

**Applies when:** m > 0; the element occurs in the formula.

```shell
$ chemistry solve atoms_of_element element=O formula=CO2 "m=0.01 kg"
N_atoms = 2.736776914e+23
n_element = 0.4544524984 mol
  by: Atoms of an element in a mass of compound  (moles of compound times the number of X atoms per formula, times N_A)
  m = 0.01 kg
  element = O
  formula = CO2
  assumes: m > 0; the element occurs in the formula
```

**What the worked example means:** This calculation determines how many individual atoms of a specific element are present in a given mass of a compound. It starts by converting the total mass of the substance into moles using its molar mass. Then, it multiplies by the number of that element's atoms in each formula unit and Avogadro's constant. The result shows that the provided mass corresponds to 2.73678e+23 atoms, which is equivalent to 0.454452 mol mol of that element.

<a id="balance_equation"></a>

#### Balancing a chemical equation — `balance_equation`

The smallest whole-number coefficients that balance a chemical equation.

$$
A\,\boldsymbol{\nu} = \mathbf{0},\ \boldsymbol{\nu} \in \mathbb{Z}_{>0}^{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `equation` | in | the equation, e.g. 'C3H8 + O2 -> CO2 + H2O' | — |
| `balanced` | out | the balanced equation | — |
| `coefficients` | out | coefficients in the order the species are written | — |

**Applies when:** one independent way to balance (a one-dimensional null space); neutral species.

```shell
$ chemistry solve balance_equation "equation=C3H8 + O2 -> CO2 + H2O"
coefficients = [1.0,5.0,3.0,4.0]
balanced = C3H8 + 5 O2 -> 3 CO2 + 4 H2O
  by: Balancing a chemical equation  (exact rational null space of the element-conservation matrix, scaled to the smallest whole numbers)
  equation = C3H8 + O2 -> CO2 + H2O
  assumes: one independent way to balance (a one-dimensional null space); neutral species
```

**What the worked example means:** This calculation determines the smallest whole-number coefficients that satisfy the law of conservation of mass for the input equation C3H8 + O2 -> CO2 + H2O. By constructing a matrix where each row represents an element and each column represents a chemical species, the algorithm finds the null space vector. The resulting coefficients [1, 5, 3, 4] ensure that the number of atoms for every element is identical on both sides of the reaction, yielding the final balanced expression C3H8 + 5 O2 -> 3 CO2 + 4 H2O.

<a id="empirical_formula"></a>

#### Empirical formula from percent composition — `empirical_formula`

The simplest whole-number formula of a compound from its mass percentages.

$$
n_i = \frac{w_i}{A_r(i)},\quad \text{subscripts} \propto \frac{n_i}{\min_j n_j}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `composition` | in | element percentages (or masses), e.g. 'C 40.0, H 6.71, O 53.3' | — |
| `formula` | out | empirical formula | — |
| `counts` | out | subscripts in the order given | — |

**Applies when:** at least two elements; a whole-number ratio with subscripts up to 12 times the smallest.

```shell
$ chemistry solve empirical_formula "composition=C 40.0, H 6.71, O 53.3"
formula = CH2O
counts = [1.0,2.0,1.0]
  by: Empirical formula from percent composition  (moles per 100 g, divided by the smallest, multiplied by the smallest k (1..12) giving whole numbers within 1.5%)
  composition = C 40.0, H 6.71, O 53.3
  assumes: at least two elements; a whole-number ratio with subscripts up to 12 times the smallest
```

**What the worked example means:** This calculation determines the simplest whole-number ratio of atoms in a compound using the provided mass percentages. Converting each element's percentage to moles and dividing by the smallest mole value gives relative proportions. These ratios are then scaled to the nearest integers to produce the empirical formula CH2O. The resulting subscripts [1, 2, 1] indicate the number of atoms for each element in the order given by the input C 40.0, H 6.71, O 53.3. This method assumes the data represents a single pure substance and that experimental errors allow for clear integer identification.

<a id="limiting_reagent"></a>

#### Limiting reagent and theoretical yield — `limiting_reagent`

Which of two reactants limits a reaction, the mass of a product formed and the excess left over.

$$
\xi = \min_i \frac{n_i}{\nu_i},\quad m_P = \xi\,\nu_P M_P
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `equation` | in | the reaction (balanced or not) | — |
| `reactant_a` | in | first reactant | — |
| `m_a` | in | mass of the first reactant | kg |
| `reactant_b` | in | second reactant | — |
| `m_b` | in | mass of the second reactant | kg |
| `product` | in | the product asked for | — |
| `product_mass` | out | mass of product formed | kg |
| `limiting` | out | limiting reagent | — |
| `excess_left` | out | mass of the excess reagent left over | kg |

**Applies when:** the equation balances uniquely; two different reactants and one product of the equation.

```shell
$ chemistry solve limiting_reagent "equation=H2 + O2 -> H2O" "m_a=0.004 kg" reactant_a=H2 "m_b=0.016 kg" reactant_b=O2 product=H2O
product_mass = 0.01801612601 kg
limiting = O2
excess_left = 0.001983873992 kg
  by: Limiting reagent and theoretical yield  (balance the equation, moles of each reactant over its coefficient; the smallest is the extent)
  m_a = 0.004 kg
  m_b = 0.016 kg
  equation = H2 + O2 -> H2O
  reactant_b = O2
  reactant_a = H2
  product = H2O
  assumes: the equation balances uniquely; two different reactants and one product of the equation
```

**What the worked example means:** This calculation determines which reactant runs out first in the chemical reaction defined by H2 + O2 -> H2O. By comparing the available moles of H2 (0.004 kg) and O2 (0.016 kg) against their stoichiometric coefficients, the model identifies O2 as the limiting reagent. The theoretical yield of H2O is calculated to be 0.0180161 kg, while the remaining mass of the excess reactant is found to be 0.00198387 kg. This result indicates the maximum possible amount of product formed under ideal conditions and the quantity of unused starting material.

<a id="mass_from_moles"></a>

#### Mass from moles — `mass_from_moles`

The mass of a given amount (moles) of a compound.

$$
m = n M
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula | — |
| `n` | in | amount of substance | mol |
| `m` | out | mass | kg |
| `m_g` | out | mass in grams | — |
| `N` | out | number of formula units | — |

**Applies when:** n > 0; a neutral formula.

```shell
$ chemistry solve mass_from_moles formula=NaCl "n=2.5 mol"
m = 0.1461075 kg
m_g = 146.1075
N = 1.50553519e+24
  by: Mass from moles  (m = n M with M from the formula)
  n = 2.5 mol
  formula = NaCl
  assumes: n > 0; a neutral formula
```

**What the worked example means:** This calculation determines the physical mass of a substance based on its chemical formula and amount in moles. Using the molar mass derived from the formula, it computes that 2.5 mol corresponds to a mass of 0.146108 kg. This result represents the precise weight needed for laboratory preparation or stoichiometric analysis.

<a id="mass_of_element"></a>

#### Mass of an element in a sample — `mass_of_element`

The mass of one element contained in a given mass of a compound.

$$
m_X = m \, \frac{\nu_X A_r(X)}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula | — |
| `element` | in | the element (symbol or name) | — |
| `m` | in | mass of the compound | kg |
| `m_element` | out | mass of the element | kg |
| `fraction` | out | mass fraction | — |

**Applies when:** m > 0; the element occurs in the formula.

```shell
$ chemistry solve mass_of_element element=Fe formula=Fe2O3 "m=0.05 kg"
m_element = 0.03497153807 kg
fraction = 0.6994307614
  by: Mass of an element in a sample  (the sample mass times the element's mass fraction)
  m = 0.05 kg
  element = Fe
  formula = Fe2O3
  assumes: m > 0; the element occurs in the formula
```

**What the worked example means:** This calculation determines the mass of a specific element contained within a given sample of a compound. Multiplying the total sample mass, which is 0.05 kg, by the element's mass fraction gives the elemental mass of 0.0349715 kg. The resulting mass fraction, shown as 0.699431, represents the proportion of the total weight that belongs to the specified element. This method relies on the fixed stoichiometric ratios defined by the chemical formula.

<a id="mass_percent"></a>

#### Mass percent of an element — `mass_percent`

The percentage by mass of an element in a compound (and the full percent composition).

$$
w_X = \frac{\nu_X A_r(X)}{M} \times 100\%
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula | — |
| `element` | in | the element (symbol or name) | — |
| `percent` | out | mass percent of the element (%) | — |
| `composition` | out | mass percent of every element (%) | — |

**Applies when:** the element occurs in the formula.

```shell
$ chemistry solve mass_percent element=O formula=H2SO4
percent = 65.24944178
composition = {"H":2.055485883828342,"S":32.69507233964457,"O":65.24944177652708}
  by: Mass percent of an element  (mass of the element per formula over the molar mass)
  element = O
  formula = H2SO4
  assumes: the element occurs in the formula
```

**What the worked example means:** This calculation determines the percentage by mass that a specific element contributes to the total molar mass of a chemical compound. Taking the atomic mass of the target element, multiplying it by its count in the formula, and dividing by the total molar mass of the compound gives the result 65.2494. This value represents the proportion of the compound's weight attributable to O, providing essential data for understanding material composition.

<a id="molar_mass"></a>

#### Molar mass of a formula — `molar_mass`

The molar mass of a chemical formula (parentheses, brackets and hydrates are read).

$$
M = \sum_i n_i A_r(i)\ \mathrm{g\,mol^{-1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula, e.g. Ca(OH)2 or CuSO4·5H2O | — |
| `M` | out | molar mass (kg/mol) | kg/mol |
| `M_g` | out | molar mass in g/mol | — |

**Applies when:** a neutral formula of element symbols with standard atomic weights.

```shell
$ chemistry solve molar_mass "formula=Ca(OH)2"
M = 0.074092 kg/mol
M_g = 74.092
  by: Molar mass of a formula  (atom counts read from the formula (nested groups multiply, hydrate parts add), times standard atomic weights)
  formula = Ca(OH)2
  assumes: a neutral formula of element symbols with standard atomic weights
```

**What the worked example means:** This calculation determines the total mass of one mole of a substance by summing the products of the number of atoms for each element and their respective standard atomic weights. The input formula, such as Ca(OH)2, is parsed to identify all constituent elements, including those within parentheses or hydrate structures. The resulting value, 74.092, represents the mass in grams per mole, while 0.074092 kg/mol expresses this same quantity in kilograms per mole. This figure allows chemists to convert between the measurable mass of a sample and the number of moles present, facilitating precise stoichiometric calculations.

<a id="molecular_formula"></a>

#### Molecular formula from the empirical formula — `molecular_formula`

The molecular formula from an empirical formula and the molar mass.

$$
n = \frac{M}{M_{\text{empirical}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `empirical` | in | empirical formula | — |
| `M` | in | molar mass of the compound | kg/mol |
| `multiple` | out | whole-number multiple n | — |
| `formula` | out | molecular formula | — |

**Applies when:** M > 0; M is a whole multiple of the empirical formula mass (within 2%).

```shell
$ chemistry solve molecular_formula empirical=CH2O "M=0.18016 kg/mol"
multiple = 6
formula = C6H12O6
  by: Molecular formula from the empirical formula  (n = M / M(empirical), rounded (must be within 2% of a whole number))
  M = 0.18016 kg/mol
  empirical = CH2O
  assumes: M > 0; M is a whole multiple of the empirical formula mass (within 2%)
```

**What the worked example means:** This calculation determines the actual molecular formula by scaling the empirical formula. It uses the given molar mass of 0.18016 kg/mol to find the whole-number multiple, which is calculated as 6. This multiple indicates how many times the empirical unit repeats in the full molecule, resulting in the final formula C6H12O6.

<a id="moles_from_mass"></a>

#### Moles from a mass — `moles_from_mass`

The amount of substance (and number of formula units) in a given mass of a compound.

$$
n = \frac{m}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula | — |
| `m` | in | mass of the sample | kg |
| `n` | out | amount of substance | mol |
| `N` | out | number of formula units | — |
| `M` | out | molar mass | kg/mol |

**Applies when:** m > 0; a neutral formula.

```shell
$ chemistry solve moles_from_mass formula=H2O "m=0.036 kg"
n = 1.998334721 mol
N = 1.203425298e+24
M = 0.018015 kg/mol
  by: Moles from a mass  (n = m / M with M from the formula)
  m = 0.036 kg
  formula = H2O
  assumes: m > 0; a neutral formula
```

**What the worked example means:** This calculation determines the amount of substance in moles from a given mass using the molar mass derived from the chemical formula. The input mass 0.036 kg is divided by the molar mass 0.018015 kg/mol to yield the result 1.99833 mol. This value represents the number of moles of the compound, which corresponds to 1.20343e+24 individual formula units, allowing for precise stoichiometric comparisons in chemical reactions.

<a id="moles_from_particles"></a>

#### Moles from a number of particles — `moles_from_particles`

The amount of substance that contains a given number of particles.

$$
n = \frac{N}{N_A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of particles | — |
| `n` | out | amount of substance | mol |

**Applies when:** N > 0.

```shell
$ chemistry solve moles_from_particles N=3.01e+23
n = 0.4998222592 mol
  by: Moles from a number of particles  (n = N / N_A)
  N = 3.01e+23
  assumes: N > 0
```

**What the worked example means:** This calculation determines the amount of substance by dividing the number of particles, given as 3.01e+23, by Avogadro's constant. The resulting value, 0.499822 mol, represents the quantity in moles. This conversion allows microscopic counts to be expressed in standard chemical units for use in stoichiometric equations.

<a id="particles_from_mass"></a>

#### Number of molecules in a mass — `particles_from_mass`

The number of molecules (formula units) in a given mass of a compound.

$$
N = \frac{m}{M} N_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | chemical formula | — |
| `m` | in | mass of the sample | kg |
| `N` | out | number of molecules / formula units | — |
| `n` | out | amount of substance | mol |

**Applies when:** m > 0.

```shell
$ chemistry solve particles_from_mass formula=CO2 "m=0.01 kg"
N = 1.368388457e+23
n = 0.2272262492 mol
  by: Number of molecules in a mass  (N = (m / M) N_A)
  m = 0.01 kg
  formula = CO2
  assumes: m > 0
```

**What the worked example means:** The calculation determines the number of discrete particles in a given mass by first finding the amount of substance and then scaling it by Avogadro's constant. For a sample weighing 0.01 kg, the result is 1.36839e+23 molecules or formula units, which represents 0.227226 mol moles of the compound. This bridges the gap between macroscopic measurements and microscopic particle counts.

<a id="particles_from_moles"></a>

#### Number of particles in an amount — `particles_from_moles`

The number of particles in a given amount of substance (Avogadro's number).

$$
N = n N_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | amount of substance | mol |
| `N` | out | number of particles | — |

**Applies when:** n > 0.

```shell
$ chemistry solve particles_from_moles "n=0.25 mol"
N = 1.50553519e+23
  by: Number of particles in an amount  (N = n N_A)
  n = 0.25 mol
  assumes: n > 0
```

**What the worked example means:** This calculation determines the total count of discrete particles, such as atoms or molecules, contained within a specific quantity of matter. Multiplying the given amount of substance in moles by Avogadro's constant translates a macroscopic measurement into a microscopic particle count. The result shows that an amount of 0.25 mol corresponds to exactly 1.50554e+23 individual particles.

<a id="percent_yield"></a>

#### Percent yield of a reaction — `percent_yield`

The percent yield from the mass of reactant used and the mass of product obtained.

$$
\%\,\text{yield} = \frac{m_{\text{actual}}}{m_{\text{theoretical}}} \times 100
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `equation` | in | the reaction (balanced or not) | — |
| `reactant` | in | the reactant whose mass is given | — |
| `m` | in | mass of the reactant | kg |
| `product` | in | the product obtained | — |
| `actual` | in | actual mass of product obtained | kg |
| `percent_yield` | out | percent yield (%) | — |
| `theoretical_mass` | out | theoretical mass of product | kg |

**Applies when:** the equation balances uniquely; the other reactants are in excess.

```shell
$ chemistry solve percent_yield "equation=CaCO3 -> CaO + CO2" "m=0.1 kg" reactant=CaCO3 "actual=0.05 kg" product=CaO
percent_yield = 89.23979528
theoretical_mass = 0.05602881522 kg
  by: Percent yield of a reaction  (theoretical yield from the balanced equation, then actual / theoretical x 100)
  m = 0.1 kg
  actual = 0.05 kg
  equation = CaCO3 -> CaO + CO2
  reactant = CaCO3
  product = CaO
  assumes: the equation balances uniquely; the other reactants are in excess
```

**What the worked example means:** This calculation determines the efficiency of the reaction by comparing the actual mass of product obtained, 0.05 kg, to the theoretical mass predicted from stoichiometry, 0.0560288 kg. The result, 89.2398, indicates how much of the expected product was successfully isolated. A value near one hundred suggests high efficiency, while lower values imply losses due to incomplete reaction or purification steps.

<a id="theoretical_yield"></a>

#### Theoretical yield from one reactant — `theoretical_yield`

The mass of a product formed from a given mass of one reactant (in excess of the others).

$$
m_P = \frac{m_R}{M_R}\,\frac{\nu_P}{\nu_R}\,M_P
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `equation` | in | the reaction (balanced or not) | — |
| `reactant` | in | the reactant whose mass is given | — |
| `m` | in | mass of the reactant | kg |
| `product` | in | the product asked for | — |
| `product_mass` | out | theoretical mass of product | kg |
| `n_product` | out | moles of product | mol |

**Applies when:** the equation balances uniquely; the other reactants are in excess.

```shell
$ chemistry solve theoretical_yield "equation=CH4 + O2 -> CO2 + H2O" "m=0.016 kg" reactant=CH4 product=CO2
product_mass = 0.04389104282 kg
n_product = 0.9973197033 mol
  by: Theoretical yield from one reactant  (balance the equation; moles of reactant times the coefficient ratio times the product's molar mass)
  m = 0.016 kg
  reactant = CH4
  product = CO2
  equation = CH4 + O2 -> CO2 + H2O
  assumes: the equation balances uniquely; the other reactants are in excess
```

**What the worked example means:** This calculation determines the maximum possible mass of a product, denoted as 0.043891 kg, that can be formed from a specific mass of one reactant, given as 0.016 kg. The process assumes that all other necessary reactants are present in excess and that the chemical reaction proceeds completely according to its balanced stoichiometry. Converting the input mass into moles, applying the ratio of coefficients between the reactant and the product, and then converting back to mass using the product's molar mass gives the theoretical limit for this reaction scenario.

### Thermochemistry (8)

<a id="clausius_clapeyron_enthalpy"></a>

#### Clausius-Clapeyron: enthalpy of vaporization — `clausius_clapeyron_enthalpy`

The enthalpy of vaporization from vapor pressures at two temperatures.

$$
\Delta H_{vap} = \frac{R\ln(P_2/P_1)}{1/T_1 - 1/T_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | vapor pressure at T1 | Pa |
| `T1` | in | first temperature | K |
| `P2` | in | vapor pressure at T2 | Pa |
| `T2` | in | second temperature | K |
| `dH` | out | enthalpy of vaporization | J/mol |

**Applies when:** T1 != T2.

```shell
$ chemistry solve clausius_clapeyron_enthalpy "P1=13332.23684 Pa" "T1=300 K" "P2=53328.94737 Pa" "T2=340 K"
dH = 29392.04624 J/mol
  by: Clausius-Clapeyron: enthalpy of vaporization  (two-point Clausius-Clapeyron equation)
  P1 = 13332.23684 Pa
  T1 = 300 K
  P2 = 53328.94737 Pa
  T2 = 340 K
  assumes: T1 != T2
```

**What the worked example means:** This calculation determines the enthalpy of vaporization, which represents the energy required to convert a liquid into a gas. By applying the Clausius-Clapeyron equation to two measured data points, specifically vapor pressures 13332.2 Pa and 53328.9 Pa recorded at temperatures 300 K and 340 K, the model computes a value of 29392 J/mol. This result quantifies the strength of intermolecular forces within the substance; a higher value indicates that more energy is needed for molecules to escape the liquid phase.

<a id="clausius_clapeyron_pressure"></a>

#### Clausius-Clapeyron: vapor pressure at another temperature — `clausius_clapeyron_pressure`

The vapor pressure of a liquid at a second temperature from one known point and the enthalpy of vaporization.

$$
\ln\frac{P_2}{P_1} = -\frac{\Delta H_{vap}}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | vapor pressure at T1 | Pa |
| `T1` | in | first temperature | K |
| `T2` | in | second temperature | K |
| `dH` | in | enthalpy of vaporization | J/mol |
| `P2` | out | vapor pressure at T2 | Pa |

**Applies when:** dHvap constant; ideal vapor.

```shell
$ chemistry solve clausius_clapeyron_pressure "T1=298.15 K" "dH=43900 J/mol" "T2=323.15 K" "P1=3173.072368 Pa"
P2 = 12487.5781 Pa
  by: Clausius-Clapeyron: vapor pressure at another temperature  (integrated Clausius-Clapeyron equation)
  T1 = 298.15 K
  dH = 43900 J/mol
  T2 = 323.15 K
  P1 = 3173.072368 Pa
  assumes: dHvap constant; ideal vapor
```

**What the worked example means:** This calculation determines the vapor pressure of a liquid at a new temperature, denoted as P2. It uses the integrated Clausius-Clapeyron equation, which relates the natural logarithm of the ratio of two pressures to the enthalpy of vaporization and the difference in inverse temperatures. Given the initial vapor pressure 3173.07 Pa at temperature 298.15 K, the final temperature 323.15 K, and the constant enthalpy of vaporization 43900 J/mol, the model computes the resulting pressure 12487.6 Pa. This result indicates how the volatility of the substance changes between these two thermal states.

<a id="gibbs_free_energy"></a>

#### Gibbs energy change (dG = dH - T dS) — `gibbs_free_energy`

The Gibbs energy change of a process from its enthalpy and entropy changes, and whether it is spontaneous.

$$
\Delta G = \Delta H - T\Delta S
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dH` | in | standard enthalpy change | J/mol |
| `dS` | in | entropy change | J/(mol*K) |
| `T` | in | temperature | K |
| `dG` | out | Gibbs energy change | J/mol |
| `dG_kJ` | out | in kJ/mol | — |
| `spontaneity` | out | spontaneous / non-spontaneous | — |

**Applies when:** dH and dS taken as constant.

```shell
$ chemistry solve gibbs_free_energy "dH=-92200 J/mol" "dS=-198.7 J/(mol*K)" "T=298 K"
dG = -32987.4 J/mol
dG_kJ = -32.9874
spontaneity = spontaneous
  by: Gibbs energy change (dG = dH - T dS)  (dG = dH - T dS)
  dH = -92200 J/mol
  dS = -198.7 J/(mol*K)
  T = 298 K
  assumes: dH and dS taken as constant
```

**What the worked example means:** This calculation determines the Gibbs energy change for a chemical process using the enthalpy change, entropy change, and temperature. Applying the relationship where Gibbs energy equals enthalpy minus the product of temperature and entropy shows that the result is -32987.4 J/mol. This value corresponds to -32.9874 when converted to kilojoules per mole. The sign of this result shows whether the process is spontaneous or non-spontaneous under these conditions, specifically showing it is spontaneous.

<a id="calorimetry_heat"></a>

#### Heat from a temperature change — `calorimetry_heat`

The heat absorbed (positive) or released (negative) by a mass of substance whose temperature changes.

$$
q = m c (T_2 - T_1)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `c` | in | specific heat capacity | J/(kg*K) |
| `T1` | in | initial temperature | K |
| `T2` | in | final temperature | K |
| `q` | out | heat absorbed (negative: released) | J |

**Applies when:** no phase change.

```shell
$ chemistry solve calorimetry_heat "T1=293.15 K" "T2=353.15 K" "m=0.05 kg" "c=4184 J/(kg*K)"
q = 12552 J
  by: Heat from a temperature change  (q = m c dT)
  T1 = 293.15 K
  T2 = 353.15 K
  m = 0.05 kg
  c = 4184 J/(kg*K)
  assumes: no phase change
```

**What the worked example means:** This calculation determines the thermal energy transferred, denoted as 12552 J, when a substance of mass 0.05 kg undergoes a temperature shift. The result depends on the material's specific heat capacity 4184 J/(kg*K) and the difference between the final temperature 353.15 K and the initial temperature 293.15 K. A positive value for 12552 J indicates that the system absorbed heat, while a negative value signifies that heat was released into the surroundings.

<a id="heat_from_reaction_mass"></a>

#### Heat of reaction for a given mass — `heat_from_reaction_mass`

The heat of a reaction for a given mass of a reactant, from its molar enthalpy (per mole of that reactant).

$$
q = \frac{m}{M}\Delta H
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `formula` | in | formula of the reactant | — |
| `m` | in | mass of the reactant | kg |
| `dH` | in | standard enthalpy change | J/mol |
| `q` | out | enthalpy change (negative: heat released) | J |
| `heat_released` | out | heat released | J |

**Applies when:** dH is per mole of the given reactant.

```shell
$ chemistry solve heat_from_reaction_mass formula=CH4 "dH=-890000 J/mol" "m=0.01 kg"
q = -554759.085 J
heat_released = 554759.085 J
  by: Heat of reaction for a given mass  (moles of reactant times its molar enthalpy)
  dH = -890000 J/mol
  m = 0.01 kg
  formula = CH4
  assumes: dH is per mole of the given reactant
```

**What the worked example means:** This calculation determines the total thermal energy change by converting the given mass of reactant into moles and multiplying by the standard enthalpy change per mole. For a reactant mass of 0.01 kg and an enthalpy change of -890000 J/mol, the resulting heat exchange is -554759 J. A negative value indicates that 554759 J of energy was released to the surroundings, while a positive value would indicate absorption.

<a id="enthalpy_from_calorimetry"></a>

#### Reaction enthalpy from coffee-cup calorimetry — `enthalpy_from_calorimetry`

The molar enthalpy of a reaction from the temperature change of the solution it takes place in.

$$
\Delta H = -\frac{m c (T_2 - T_1)}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of the solution | kg |
| `c` | in | specific heat of the solution | J/(kg*K) |
| `T1` | in | initial temperature | K |
| `T2` | in | final temperature | K |
| `n` | in | moles of the reaction (limiting reactant) | mol |
| `dH` | out | reaction enthalpy | J/mol |
| `dH_kJ` | out | in kJ/mol | — |
| `q_solution` | out | heat taken up by the solution | J |

**Applies when:** no heat lost to the calorimeter or surroundings; constant pressure.

```shell
$ chemistry solve enthalpy_from_calorimetry "T1=295.15 K" "T2=301.95 K" "n=0.05 mol" "m=0.1 kg" "c=4180 J/(kg*K)"
dH = -56848 J/mol
dH_kJ = -56.848
q_solution = 2842.4 J
  by: Reaction enthalpy from coffee-cup calorimetry  (heat gained by the solution, with the opposite sign, per mole)
  T1 = 295.15 K
  T2 = 301.95 K
  n = 0.05 mol
  m = 0.1 kg
  c = 4180 J/(kg*K)
  assumes: no heat lost to the calorimeter or surroundings; constant pressure
```

**What the worked example means:** This calculation determines the molar enthalpy change of a reaction, -56.848, based on the temperature shift observed in a coffee-cup calorimeter. The process assumes that all heat exchanged between the reaction and the surroundings occurs within the solution itself. Measuring the mass of the solution, its specific heat capacity, and the initial and final temperatures gives the total heat gained or lost by the solution. This value is then divided by the number of moles of the limiting reactant to find the enthalpy per mole. A negative result indicates an exothermic reaction where heat is released, while a positive result signifies an endothermic process absorbing heat.

<a id="reaction_enthalpy_from_formation"></a>

#### Reaction enthalpy from enthalpies of formation (Hess's law) — `reaction_enthalpy_from_formation`

The standard reaction enthalpy from the standard enthalpies of formation of every species (balanced equation).

$$
\Delta H^\circ_{rxn} = \sum \nu_p \Delta H_f^\circ(p) - \sum \nu_r \Delta H_f^\circ(r)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `equation` | in | balanced equation | — |
| `values` | in | standard enthalpy of formation of each species in the order written (kJ/mol) | — |
| `dH` | out | standard reaction enthalpy | J/mol |
| `dH_kJ` | out | in kJ/mol | — |

**Applies when:** the equation as written conserves every element; one value per species, in kJ/mol.

```shell
$ chemistry solve reaction_enthalpy_from_formation "equation=CH4 + 2 O2 -> CO2 + 2 H2O" "values=[-74.8, 0, -393.5, -285.8]"
dH = -890300 J/mol
dH_kJ = -890.3
  by: Reaction enthalpy from enthalpies of formation (Hess's law)  (coefficient-weighted sum of formation enthalpies, products minus reactants)
  equation = CH4 + 2 O2 -> CO2 + 2 H2O
  assumes: the equation as written conserves every element; one value per species, in kJ/mol
```

**What the worked example means:** This calculation determines the heat change for the balanced chemical equation by applying Hess's Law. It sums the standard enthalpies of formation for all product species, each multiplied by its stoichiometric coefficient, and subtracts the corresponding sum for all reactant species. The resulting value, -890.3, represents the energy released or absorbed under standard conditions.

<a id="crossover_temperature"></a>

#### Temperature at which dG = 0 — `crossover_temperature`

The temperature at which a reaction changes between spontaneous and non-spontaneous (dG = 0).

$$
T = \frac{\Delta H}{\Delta S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dH` | in | standard enthalpy change | J/mol |
| `dS` | in | entropy change | J/(mol*K) |
| `T` | out | temperature where dG = 0 | K |
| `T_C` | out | in °C | — |

**Applies when:** dH and dS of the same sign.

```shell
$ chemistry solve crossover_temperature "dH=178000 J/mol" "dS=161 J/(mol*K)"
T = 1105.590062 K
T_C = 832.4400621
  by: Temperature at which dG = 0  (T = dH / dS)
  dH = 178000 J/mol
  dS = 161 J/(mol*K)
  assumes: dH and dS of the same sign
```

**What the worked example means:** This calculation determines the specific temperature at which the Gibbs free energy change becomes zero, marking the transition point for reaction spontaneity. Dividing the standard enthalpy change of 178000 J/mol by the entropy change of 161 J/(mol*K) shows that this equilibrium occurs at 1105.59 K. This value indicates the threshold above or below which the process shifts from spontaneous to non-spontaneous behavior.

Questions and reports: support@ai999x.com. AI999X — The Future Is Self-Improving.
