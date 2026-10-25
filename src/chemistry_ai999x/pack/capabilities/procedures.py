"""Chemistry procedures: deterministic algorithms with typed inputs and several outputs (domain code, loaded by the
pack; enabled only once tools/certify_procedures.py has certified them).

Every procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it reads,
worked examples, an INDEPENDENT reference computation (used only by certification, calling the checking libraries:
RDKit's periodic table with the formula parser of `chemicals`, `chemicals` and `thermo` property routines, Cantera's
rate expressions, SciPy integration, root finding and statistics, SymPy's exact linear algebra) and a deliberately
wrong variant (certification shows that the checks reject it).

Units: every physical input and output is in SI (mol/m^3 for a concentration: 1 mol/L = 1000 mol/m^3, kg/mol for a
molar mass); where a chemist expects another unit, a plain-number companion output gives it (`M_g` in g/mol,
`c_molL` in mol/L, `wavelength_nm` in nm, ...). Questions may state values in the usual laboratory units (atm, torr,
mmHg, L, mL, g, °C, M, kJ/mol, nm, ...); the reader converts them to SI.

Constants (exact SI values or CODATA 2022): R = 8.31446261815324 J/(mol K), F = 96485.33212331001 C/mol,
N_A = 6.02214076e23 /mol, h = 6.62607015e-34 J s, c = 299792458 m/s, k_B = 1.380649e-23 J/K, Kw = 1.0e-14 (25 °C).
Atomic weights: the standard atomic weights as tabulated by RDKit (H 1.008, C 12.011, N 14.007, O 15.999, ...).
"""
from __future__ import annotations

import math
import re
from fractions import Fraction

R = 8.31446261815324
F = 96485.33212331001
NA = 6.02214076e23
H_PLANCK = 6.62607015e-34
C_LIGHT = 299792458.0
KB = 1.380649e-23
KW = 1.0e-14
ATM = 101325.0
EV = 1.602176634e-19
R_INF = 10973731.568157
M_E = 9.1093837139e-31
M_P = 1.67262192595e-27

# standard atomic weights (g/mol) as tabulated by RDKit (elements with a standard atomic weight)
W = {"H": 1.008, "He": 4.003, "Li": 6.941, "Be": 9.012, "B": 10.812, "C": 12.011, "N": 14.007, "O": 15.999,
     "F": 18.998, "Ne": 20.18, "Na": 22.99, "Mg": 24.305, "Al": 26.982, "Si": 28.086, "P": 30.974, "S": 32.067,
     "Cl": 35.453, "Ar": 39.948, "K": 39.098, "Ca": 40.078, "Sc": 44.956, "Ti": 47.867, "V": 50.944, "Cr": 51.996,
     "Mn": 54.938, "Fe": 55.845, "Co": 58.933, "Ni": 58.693, "Cu": 63.546, "Zn": 65.39, "Ga": 69.723, "Ge": 72.61,
     "As": 74.922, "Se": 78.96, "Br": 79.904, "Kr": 83.8, "Rb": 85.468, "Sr": 87.62, "Y": 88.906, "Zr": 91.224,
     "Nb": 92.906, "Mo": 95.94, "Ru": 101.07, "Rh": 102.906, "Pd": 106.42, "Ag": 107.868, "Cd": 112.412,
     "In": 114.818, "Sn": 118.711, "Sb": 121.76, "Te": 127.6, "I": 126.904, "Xe": 131.29, "Cs": 132.905,
     "Ba": 137.328, "La": 138.906, "Ce": 140.116, "Pr": 140.908, "Nd": 144.24, "Sm": 150.36, "Eu": 151.964,
     "Gd": 157.25, "Tb": 158.925, "Dy": 162.5, "Ho": 164.93, "Er": 167.26, "Tm": 168.934, "Yb": 173.04,
     "Lu": 174.967, "Hf": 178.49, "Ta": 180.948, "W": 183.84, "Re": 186.207, "Os": 190.23, "Ir": 192.217,
     "Pt": 195.078, "Au": 196.967, "Hg": 200.59, "Tl": 204.383, "Pb": 207.2, "Bi": 208.98, "Th": 232.038,
     "Pa": 231.036, "U": 238.029}
# mass of the most abundant isotope (u), for vibrational and rotational spectroscopy
ISO = {"H": 1.007825032, "C": 12.0, "N": 14.003074, "O": 15.99491462, "F": 18.99840322, "Na": 22.98976928,
       "Si": 27.97692653, "P": 30.97376163, "S": 31.972071, "Cl": 34.96885268, "Br": 78.9183371, "I": 126.904473,
       "B": 11.0093054, "Li": 7.01600455, "K": 38.96370668, "Mg": 23.9850417, "Al": 26.98153863, "Ca": 39.96259098}
U_KG = 1.66053906892e-27      # atomic mass constant (kg)

ELEMENT_NAMES = {
    "hydrogen": "H", "helium": "He", "lithium": "Li", "beryllium": "Be", "boron": "B", "carbon": "C",
    "nitrogen": "N", "oxygen": "O", "fluorine": "F", "neon": "Ne", "sodium": "Na", "magnesium": "Mg",
    "aluminium": "Al", "aluminum": "Al", "silicon": "Si", "phosphorus": "P", "sulfur": "S", "sulphur": "S",
    "chlorine": "Cl", "argon": "Ar", "potassium": "K", "calcium": "Ca", "titanium": "Ti", "chromium": "Cr",
    "manganese": "Mn", "iron": "Fe", "cobalt": "Co", "nickel": "Ni", "copper": "Cu", "zinc": "Zn",
    "bromine": "Br", "silver": "Ag", "tin": "Sn", "iodine": "I", "barium": "Ba", "platinum": "Pt", "gold": "Au",
    "mercury": "Hg", "lead": "Pb", "uranium": "U", "cadmium": "Cd", "strontium": "Sr"}


# --- small numerical helpers ----------------------------------------------------------------------------------
def _rel(a, b) -> float:
    """Relative difference of two numbers (0 when both are 0)."""
    a, b = float(a), float(b)
    s = max(abs(a), abs(b))
    return 0.0 if s == 0 else abs(a - b) / s


def _pos(**kw):
    for k, v in kw.items():
        if not (isinstance(v, (int, float)) and math.isfinite(v)) or v <= 0:
            raise ValueError(f"{k} must be a positive number")


def _finite(**kw):
    for k, v in kw.items():
        if not (isinstance(v, (int, float)) and math.isfinite(v)):
            raise ValueError(f"{k} must be a finite number")


def _bisect(f, lo, hi, iters=400):
    """Root of a function that changes sign on [lo, hi], to the last representable digit."""
    flo = f(lo)
    if flo == 0:
        return lo
    for _ in range(iters):
        mid = 0.5 * (lo + hi)
        if mid == lo or mid == hi:
            break
        fm = f(mid)
        if fm == 0:
            return mid
        if (fm < 0) == (flo < 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return 0.5 * (lo + hi)


# --- chemical formulas ------------------------------------------------------------------------------------------
_SUBSCRIPT = str.maketrans("₀₁₂₃₄₅₆₇₈₉", "0123456789")
_HYDRATE_SPLIT = re.compile(r"[·•⋅∙*]|\.(?=\d|[A-Z(\[])")
_FTOK = re.compile(r"([A-Z][a-z]?)(\d*)|([(\[{])|([)\]}])(\d*)")
_PAIRS = {"(": ")", "[": "]", "{": "}"}


def _group_counts(s: str) -> dict:
    stack, opens, pos = [{}], [], 0
    while pos < len(s):
        m = _FTOK.match(s, pos)
        if not m:
            raise ValueError(f"cannot read the formula at '{s[pos:]}'")
        if m.group(1):
            el = m.group(1)
            if el not in W:
                raise ValueError(f"'{el}' is not an element symbol with a standard atomic weight")
            n = int(m.group(2)) if m.group(2) else 1
            if n == 0:
                raise ValueError("a subscript of 0 is not allowed")
            stack[-1][el] = stack[-1].get(el, 0) + n
        elif m.group(3):
            stack.append({})
            opens.append(m.group(3))
        else:
            if len(stack) == 1 or _PAIRS[opens[-1]] != m.group(4):
                raise ValueError("unbalanced brackets in the formula")
            opens.pop()
            n = int(m.group(5)) if m.group(5) else 1
            if n == 0:
                raise ValueError("a subscript of 0 is not allowed")
            top = stack.pop()
            if not top:
                raise ValueError("empty brackets in the formula")
            for el, c in top.items():
                stack[-1][el] = stack[-1].get(el, 0) + c * n
        pos = m.end()
    if len(stack) != 1 or not stack[0]:
        raise ValueError("unbalanced brackets or an empty formula")
    return stack[0]


def formula_counts(text) -> dict:
    """Atom counts of a neutral formula: parentheses/brackets ('Ca(OH)2', 'K4[Fe(CN)6]') and hydrates
    ('CuSO4·5H2O', 'CuSO4.5H2O', 'CuSO4*5H2O'). Charges and unknown symbols raise ValueError."""
    if not isinstance(text, str):
        raise ValueError("a formula is text, such as 'H2O'")
    s = text.translate(_SUBSCRIPT).replace(" ", "")
    if not s:
        raise ValueError("empty formula")
    if re.search(r"[+\-^]", s):
        raise ValueError("ions are not supported: give a neutral formula")
    total = {}
    for part in _HYDRATE_SPLIT.split(s):
        if not part:
            raise ValueError("malformed hydrate formula")
        m = re.match(r"(\d+)(?=[A-Z(\[{])", part)
        mult = int(m.group(1)) if m else 1
        if m:
            part = part[m.end():]
        if mult == 0:
            raise ValueError("a multiplier of 0 is not allowed")
        for el, n in _group_counts(part).items():
            total[el] = total.get(el, 0) + n * mult
    return total


def _counts_rd(text) -> dict:
    """A second, recursive-descent reading of a formula (used by the verifiers, independent of formula_counts)."""
    s = text.translate(_SUBSCRIPT).replace(" ", "")
    parts = re.split(r"[·•⋅∙*]|\.(?=\d|[A-Z(\[])", s)
    out = {}

    def number(t, i):
        j = i
        while j < len(t) and t[j].isdigit():
            j += 1
        return (int(t[i:j]) if j > i else 1), j

    def seq(t, i, close):
        acc = {}
        while i < len(t) and t[i] not in ")]}":
            if t[i] in "([{":
                inner, i = seq(t, i + 1, _PAIRS[t[i]])
                i += 1
                n, i = number(t, i)
                for k, v in inner.items():
                    acc[k] = acc.get(k, 0) + v * n
            else:
                sym = t[i] + (t[i + 1] if i + 1 < len(t) and t[i + 1].islower() else "")
                i += len(sym)
                n, i = number(t, i)
                acc[sym] = acc.get(sym, 0) + n
        return acc, i

    for p in parts:
        mult, i = number(p, 0) if p[:1].isdigit() else (1, 0)
        inner, _ = seq(p, i, None)
        for k, v in inner.items():
            out[k] = out.get(k, 0) + v * mult
    return out


def molar_mass_g(text) -> float:
    """Molar mass in g/mol."""
    return math.fsum(n * W[el] for el, n in formula_counts(text).items())


def _mm_rd(text) -> float:
    return math.fsum(n * W[el] for el, n in _counts_rd(text).items())


def _element(text) -> str:
    """An element given by symbol ('O') or by name ('oxygen')."""
    if not isinstance(text, str):
        raise ValueError("an element is text, such as 'O' or 'oxygen'")
    t = text.strip()
    if t in W:
        return t
    if t.lower() in ELEMENT_NAMES:
        return ELEMENT_NAMES[t.lower()]
    raise ValueError(f"'{text}' is not an element symbol or name")


def _write_formula(order, counts) -> str:
    return "".join(el + (str(counts[el]) if counts[el] != 1 else "") for el in order if counts.get(el))


# --- chemical equations -------------------------------------------------------------------------------------------
_ARROW = re.compile(r"\s*(?:<=>|<->|⇌|⇄|⟷|->|-->|→|⟶|=>|=)\s*")
_STATE = re.compile(r"\((?:s|l|g|aq)\)")


def parse_equation(text):
    """'2 H2 + O2 -> 2 H2O' -> ([(2, 'H2'), (1, 'O2')], [(2, 'H2O')]); coefficients are optional (default 1)."""
    if not isinstance(text, str):
        raise ValueError("an equation is text, such as 'H2 + O2 -> H2O'")
    t = _STATE.sub("", text.translate(_SUBSCRIPT))
    sides = _ARROW.split(t.strip())
    if len(sides) != 2 or not sides[0].strip() or not sides[1].strip():
        raise ValueError("an equation needs reactants, one arrow and products")
    out = []
    for side in sides:
        terms = []
        for term in side.split("+"):
            term = term.strip()
            m = re.fullmatch(r"(\d+(?:/\d+)?)?\s*(\S+)", term)
            if not m:
                raise ValueError(f"cannot read the term '{term}'")
            coef = Fraction(m.group(1)) if m.group(1) else Fraction(1)
            if coef <= 0:
                raise ValueError("coefficients must be positive")
            formula_counts(m.group(2))
            terms.append((coef, m.group(2)))
        out.append(terms)
    names = [f for _c, f in out[0] + out[1]]
    if len(set(names)) != len(names):
        raise ValueError("a species appears more than once")
    return out[0], out[1]


def _element_matrix(reactants, products):
    species = [f for _c, f in reactants] + [f for _c, f in products]
    counts = [formula_counts(f) for f in species]
    elements = sorted({e for c in counts for e in c})
    nr = len(reactants)
    rows = [[Fraction(c.get(e, 0) * (1 if j < nr else -1)) for j, c in enumerate(counts)] for e in elements]
    return rows, species


def _nullspace_1(rows, n):
    """The one-dimensional null space of an integer matrix (exact fractions), or ValueError."""
    m = [r[:] for r in rows]
    pivots, r = [], 0
    for c in range(n):
        p = next((i for i in range(r, len(m)) if m[i][c] != 0), None)
        if p is None:
            continue
        m[r], m[p] = m[p], m[r]
        piv = m[r][c]
        m[r] = [x / piv for x in m[r]]
        for i in range(len(m)):
            if i != r and m[i][c] != 0:
                f = m[i][c]
                m[i] = [a - f * b for a, b in zip(m[i], m[r])]
        pivots.append(c)
        r += 1
    free = [c for c in range(n) if c not in pivots]
    if len(free) != 1:
        raise ValueError("the equation cannot be balanced uniquely (it has no solution or several independent ones)")
    fc = free[0]
    v = [Fraction(0)] * n
    v[fc] = Fraction(1)
    for i, pc in enumerate(pivots):
        v[pc] = -m[i][fc]
    if any(x == 0 for x in v):
        raise ValueError("the equation cannot be balanced with every species taking part")
    if all(x < 0 for x in v):
        v = [-x for x in v]
    if any(x < 0 for x in v):
        raise ValueError("the equation cannot be balanced with positive coefficients")
    lcm = 1
    for x in v:
        lcm = lcm * x.denominator // math.gcd(lcm, x.denominator)
    ints = [int(x * lcm) for x in v]
    g = 0
    for x in ints:
        g = math.gcd(g, x)
    return [x // g for x in ints]


def balance(text):
    """Smallest whole-number coefficients of an equation (its written coefficients are ignored)."""
    reactants, products = parse_equation(text)
    rows, species = _element_matrix(reactants, products)
    coefs = _nullspace_1(rows, len(species))
    return coefs, species, len(reactants)


def _written(coefs, species, nr) -> str:
    term = lambda c, f: (f"{c} " if c != 1 else "") + f
    return " + ".join(term(c, f) for c, f in zip(coefs[:nr], species[:nr])) + " -> " + \
        " + ".join(term(c, f) for c, f in zip(coefs[nr:], species[nr:]))


def _conserved(coefs, species, nr) -> bool:
    tot = {}
    for j, (c, f) in enumerate(zip(coefs, species)):
        for el, n in _counts_rd(f).items():
            tot[el] = tot.get(el, 0) + (c * n if j < nr else -c * n)
    return all(v == 0 for v in tot.values())


def _as_written_balanced(text):
    """The equation with its written coefficients, which must conserve every element."""
    reactants, products = parse_equation(text)
    coefs = [c for c, _f in reactants + products]
    species = [f for _c, f in reactants + products]
    if not _conserved(coefs, species, len(reactants)):
        raise ValueError("the equation as written is not balanced; balance it first")
    return [float(c) for c in coefs], species, len(reactants)


# --- references: the checking libraries (imported inside, never shipped) --------------------------------------------
def _ref_counts(formula):
    """Atom counts by the formula parser of `chemicals` (hydrates split on '·')."""
    from chemicals.elements import nested_formula_parser
    total = {}
    for part in formula.replace(" ", "").split("·"):
        m = re.match(r"(\d+)(?=[A-Z(\[])", part)
        mult = int(m.group(1)) if m else 1
        if m:
            part = part[m.end():]
        for el, n in nested_formula_parser(part).items():
            total[el] = total.get(el, 0) + n * mult
    return total


def _ref_w(el):
    from rdkit import Chem
    return Chem.GetPeriodicTable().GetAtomicWeight(el)


def _ref_mw(formula):
    """g/mol: RDKit's periodic table with the formula parser of `chemicals`."""
    return sum(n * _ref_w(el) for el, n in _ref_counts(formula).items())


def _ref_balance(text):
    """Smallest whole-number coefficients by SymPy's exact null space."""
    import sympy as sp
    t = re.sub(r"\((?:s|l|g|aq)\)", "", text)
    left, right = re.split(r"\s*(?:->|=)\s*", t)
    sp_l = [re.sub(r"^\d+\s*", "", x.strip()) for x in left.split("+")]
    sp_r = [re.sub(r"^\d+\s*", "", x.strip()) for x in right.split("+")]
    species = sp_l + sp_r
    counts = [_ref_counts(s) for s in species]
    els = sorted({e for c in counts for e in c})
    M = sp.Matrix([[c.get(e, 0) * (1 if j < len(sp_l) else -1) for j, c in enumerate(counts)] for e in els])
    ns = M.nullspace()
    assert len(ns) == 1
    v = ns[0]
    lcm = sp.ilcm(*[x.q for x in v])
    v = [x * lcm for x in v]
    g = sp.igcd(*v)
    v = [int(x / g) for x in v]
    if v[0] < 0:
        v = [-x for x in v]
    return v, species, len(sp_l)


# --- the question reader (documented templates; never a guess) -------------------------------------------------------
_SUPER = {"⁰": "0", "¹": "1", "²": "2", "³": "3", "⁴": "4", "⁵": "5", "⁶": "6", "⁷": "7", "⁸": "8", "⁹": "9",
          "⁻": "-", "⁺": "+"}


def _norm(q: str) -> str:
    q = q.replace("−", "-").replace("–", "-").replace(" ", " ").replace("º", "°").replace("˚", "°")
    q = re.sub(r"[⁻⁺]?[⁰¹²³⁴⁵⁶⁷⁸⁹]+", lambda m: "^" + "".join(_SUPER[c] for c in m.group(0)), q)
    q = re.sub(r"(\d(?:\.\d+)?)\s*(?:×|x|\*|·)\s*10\s*\^\s*(?:\(\s*([-+]?\d+)\s*\)|([-+]?\d+))",
               lambda m: f"{m.group(1)}e{m.group(2) or m.group(3)}", q)
    q = re.sub(r"(?<![\w.])10\s*\^\s*(?:\(\s*([-+]?\d+)\s*\)|([-+]?\d+))(?![\d.])",
               lambda m: f"1e{m.group(1) or m.group(2)}", q)
    return q


# unit spelling -> (dimension class, factor to SI); Celsius offsets in _OFFSET. '·' in a spelling means an optional
# multiplication sign (or a space), a space means optional whitespace.
_UNIT_TABLE = [
    # pressure
    ("atm", "pressure", 101325.0), ("atmospheres", "pressure", 101325.0), ("atmosphere", "pressure", 101325.0),
    ("kPa", "pressure", 1e3), ("MPa", "pressure", 1e6), ("Pa", "pressure", 1.0), ("bar", "pressure", 1e5),
    ("mbar", "pressure", 100.0), ("mmHg", "pressure", 133.322387415), ("mm Hg", "pressure", 133.322387415),
    ("torr", "pressure", 101325.0 / 760), ("Torr", "pressure", 101325.0 / 760), ("psi", "pressure", 6894.757293168361),
    # volume
    ("L", "volume", 1e-3), ("mL", "volume", 1e-6), ("µL", "volume", 1e-9), ("uL", "volume", 1e-9),
    ("m^3", "volume", 1.0), ("cm^3", "volume", 1e-6), ("dm^3", "volume", 1e-3), ("liters", "volume", 1e-3),
    ("litres", "volume", 1e-3), ("liter", "volume", 1e-3), ("litre", "volume", 1e-3), ("milliliters", "volume", 1e-6),
    ("millilitres", "volume", 1e-6), ("mL", "volume", 1e-6),
    # temperature (offset handled separately)
    ("K", "temperature", 1.0), ("kelvin", "temperature", 1.0), ("°C", "temperature", 1.0), ("° C", "temperature", 1.0),
    ("degC", "temperature", 1.0), ("degrees Celsius", "temperature", 1.0), ("degrees C", "temperature", 1.0),
    ("Celsius", "temperature", 1.0),
    # mass
    ("kg", "mass", 1.0), ("g", "mass", 1e-3), ("mg", "mass", 1e-6), ("µg", "mass", 1e-9), ("grams", "mass", 1e-3),
    ("gram", "mass", 1e-3), ("kilograms", "mass", 1.0), ("milligrams", "mass", 1e-6),
    # amount
    ("mol", "amount", 1.0), ("mmol", "amount", 1e-3), ("µmol", "amount", 1e-6), ("moles", "amount", 1.0),
    ("mole", "amount", 1.0), ("millimoles", "amount", 1e-3),
    # time
    ("s", "time", 1.0), ("ms", "time", 1e-3), ("min", "time", 60.0), ("h", "time", 3600.0), ("hr", "time", 3600.0),
    ("seconds", "time", 1.0), ("second", "time", 1.0), ("minutes", "time", 60.0), ("minute", "time", 60.0),
    ("hours", "time", 3600.0), ("hour", "time", 3600.0), ("days", "time", 86400.0), ("day", "time", 86400.0),
    ("years", "time", 3.15576e7), ("year", "time", 3.15576e7), ("yr", "time", 3.15576e7),
    # concentration (SI: mol/m^3)
    ("M", "molar_concentration", 1000.0), ("mM", "molar_concentration", 1.0), ("µM", "molar_concentration", 1e-3),
    ("uM", "molar_concentration", 1e-3), ("nM", "molar_concentration", 1e-6), ("mol/L", "molar_concentration", 1000.0),
    ("mol/dm^3", "molar_concentration", 1000.0), ("mmol/L", "molar_concentration", 1.0),
    ("mol/m^3", "molar_concentration", 1.0), ("mol L^-1", "molar_concentration", 1000.0),
    ("mol dm^-3", "molar_concentration", 1000.0), ("molar", "molar_concentration", 1000.0),
    # molality
    ("mol/kg", "molality", 1.0), ("mmol/kg", "molality", 1e-3), ("mol kg^-1", "molality", 1.0),
    ("molal", "molality", 1.0),
    # molar energy and entropy
    ("kJ/mol", "molar_energy", 1e3), ("J/mol", "molar_energy", 1.0), ("kcal/mol", "molar_energy", 4184.0),
    ("cal/mol", "molar_energy", 4.184), ("kJ mol^-1", "molar_energy", 1e3), ("J mol^-1", "molar_energy", 1.0),
    ("J/(mol·K)", "molar_entropy", 1.0), ("J/(K·mol)", "molar_entropy", 1.0), ("J/mol/K", "molar_entropy", 1.0),
    ("J/K/mol", "molar_entropy", 1.0), ("J mol^-1 K^-1", "molar_entropy", 1.0), ("J K^-1 mol^-1", "molar_entropy", 1.0),
    ("J/mol·K", "molar_entropy", 1.0), ("kJ/(mol·K)", "molar_entropy", 1e3), ("kJ/(K·mol)", "molar_entropy", 1e3),
    ("kJ/mol/K", "molar_entropy", 1e3), ("kJ mol^-1 K^-1", "molar_entropy", 1e3), ("kJ/mol·K", "molar_entropy", 1e3),
    ("cal/(mol·K)", "molar_entropy", 4.184),
    # energy
    ("J", "energy", 1.0), ("kJ", "energy", 1e3), ("MJ", "energy", 1e6), ("cal", "energy", 4.184),
    ("kcal", "energy", 4184.0), ("eV", "energy", EV), ("joules", "energy", 1.0), ("kilojoules", "energy", 1e3),
    # length
    ("m", "length", 1.0), ("cm", "length", 1e-2), ("mm", "length", 1e-3), ("µm", "length", 1e-6),
    ("um", "length", 1e-6), ("nm", "length", 1e-9), ("pm", "length", 1e-12), ("Å", "length", 1e-10),
    ("angstrom", "length", 1e-10), ("angstroms", "length", 1e-10), ("nanometers", "length", 1e-9),
    ("nanometres", "length", 1e-9), ("centimeters", "length", 1e-2), ("centimetres", "length", 1e-2),
    # electricity
    ("A", "current", 1.0), ("mA", "current", 1e-3), ("amperes", "current", 1.0), ("amps", "current", 1.0),
    ("ampere", "current", 1.0), ("C", "charge", 1.0), ("coulombs", "charge", 1.0),
    ("V", "voltage", 1.0), ("mV", "voltage", 1e-3), ("volts", "voltage", 1.0),
    # per time (first-order rate constants, frequencies)
    ("s^-1", "per_time", 1.0), ("1/s", "per_time", 1.0), ("min^-1", "per_time", 1 / 60), ("1/min", "per_time", 1 / 60),
    ("h^-1", "per_time", 1 / 3600), ("1/h", "per_time", 1 / 3600), ("hr^-1", "per_time", 1 / 3600),
    ("yr^-1", "per_time", 1 / 3.15576e7), ("Hz", "per_time", 1.0), ("kHz", "per_time", 1e3),
    ("MHz", "per_time", 1e6), ("GHz", "per_time", 1e9), ("per second", "per_time", 1.0),
    ("per minute", "per_time", 1 / 60),
    # spectroscopy
    ("cm^-1", "wavenumber", 100.0), ("1/cm", "wavenumber", 100.0), ("m^-1", "wavenumber", 1.0),
    ("N/m", "spring_constant", 1.0), ("mdyn/Å", "spring_constant", 100.0),
    ("L/(mol·cm)", "molar_absorptivity", 0.1), ("L mol^-1 cm^-1", "molar_absorptivity", 0.1),
    ("M^-1 cm^-1", "molar_absorptivity", 0.1), ("M^-1·cm^-1", "molar_absorptivity", 0.1),
    ("cm^-1 M^-1", "molar_absorptivity", 0.1), ("L/mol/cm", "molar_absorptivity", 0.1),
    ("L/mol·cm", "molar_absorptivity", 0.1), ("m^2/mol", "molar_absorptivity", 1.0),
    ("T", "magnetic_field", 1.0),
    # colligative constants
    ("K·kg/mol", "colligative_constant", 1.0), ("°C·kg/mol", "colligative_constant", 1.0),
    ("K kg mol^-1", "colligative_constant", 1.0), ("°C kg mol^-1", "colligative_constant", 1.0),
    ("°C/m", "colligative_constant", 1.0), ("K/m", "colligative_constant", 1.0), ("°C/molal", "colligative_constant", 1.0),
    ("K/molal", "colligative_constant", 1.0),
    # van der Waals constants
    ("L^2·atm/mol^2", "van_der_waals_a", 0.101325), ("atm·L^2/mol^2", "van_der_waals_a", 0.101325),
    ("L^2 atm mol^-2", "van_der_waals_a", 0.101325), ("atm L^2 mol^-2", "van_der_waals_a", 0.101325),
    ("bar·L^2/mol^2", "van_der_waals_a", 0.1), ("L^2·bar/mol^2", "van_der_waals_a", 0.1),
    ("Pa·m^6/mol^2", "van_der_waals_a", 1.0),
    ("L/mol", "molar_volume", 1e-3), ("m^3/mol", "molar_volume", 1.0), ("cm^3/mol", "molar_volume", 1e-6),
    ("mL/mol", "molar_volume", 1e-6), ("L mol^-1", "molar_volume", 1e-3),
    # rate constants
    ("M/s", "zero_order_rate_constant", 1000.0), ("M s^-1", "zero_order_rate_constant", 1000.0),
    ("mol/(L·s)", "zero_order_rate_constant", 1000.0), ("mol L^-1 s^-1", "zero_order_rate_constant", 1000.0),
    ("M/min", "zero_order_rate_constant", 1000.0 / 60), ("mol/(L·min)", "zero_order_rate_constant", 1000.0 / 60),
    ("M^-1 s^-1", "second_order_rate_constant", 1e-3), ("M^-1·s^-1", "second_order_rate_constant", 1e-3),
    ("L/(mol·s)", "second_order_rate_constant", 1e-3), ("L mol^-1 s^-1", "second_order_rate_constant", 1e-3),
    ("M^-1 min^-1", "second_order_rate_constant", 1e-3 / 60),
    ("L/(mol·min)", "second_order_rate_constant", 1e-3 / 60),
    # specific heat, molar mass, density, Henry constant
    ("J/(g·K)", "specific_heat", 1e3), ("J/(g·°C)", "specific_heat", 1e3), ("J/g·°C", "specific_heat", 1e3),
    ("J/g°C", "specific_heat", 1e3), ("J/g/°C", "specific_heat", 1e3), ("J/g/K", "specific_heat", 1e3),
    ("J g^-1 K^-1", "specific_heat", 1e3), ("J/g·K", "specific_heat", 1e3), ("J/(kg·K)", "specific_heat", 1.0),
    ("J/(kg·°C)", "specific_heat", 1.0), ("cal/(g·°C)", "specific_heat", 4184.0), ("cal/g°C", "specific_heat", 4184.0),
    ("g/mol", "molar_mass", 1e-3), ("kg/mol", "molar_mass", 1.0), ("g mol^-1", "molar_mass", 1e-3),
    ("g/L", "density", 1.0), ("kg/m^3", "density", 1.0), ("g/mL", "density", 1e3), ("g/cm^3", "density", 1e3),
    ("mol/(L·atm)", "henry_constant", 1000.0 / 101325.0), ("M/atm", "henry_constant", 1000.0 / 101325.0),
    ("mol L^-1 atm^-1", "henry_constant", 1000.0 / 101325.0),
    ("%", "percent", 1.0), ("percent", "percent", 1.0),
]
_OFFSET = {"°C": 273.15, "° C": 273.15, "degC": 273.15, "degrees Celsius": 273.15, "degrees C": 273.15,
           "Celsius": 273.15}


def _unit_rx(sp: str) -> str:
    out = []
    for ch in sp:
        if ch == "·":
            out.append(r"\s*[*·⋅]?\s*")
        elif ch == " ":
            out.append(r"\s*")
        else:
            out.append(re.escape(ch))
    return "".join(out)


_UNIT_ORDER = sorted(_UNIT_TABLE, key=lambda u: -len(u[0]))
_UNIT_RX = re.compile(r"\s*(" + "|".join(f"(?:{_unit_rx(u[0])})" for u in _UNIT_ORDER) + r")(?![A-Za-z0-9^_/])")
_NUM_RX = re.compile(r"(?<![\w.\])\[^-])([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?)")


def _unit_of(text):
    """The table entry matching a unit text exactly (compared without spaces and multiplication signs)."""
    key = re.sub(r"[\s*·⋅]", "", text)
    for sp, dim, fac in _UNIT_ORDER:
        if re.sub(r"[\s*·⋅]", "", sp) == key:
            return sp, dim, fac
    return None


def _quantities(text):
    """Numbers in the text with their unit: [(start, end, value, unit spelling or '', dim, factor)]."""
    out = []
    for m in _NUM_RX.finditer(text):
        if out and m.start() < out[-1][1]:
            continue                                   # an exponent inside the previous quantity's unit
        um = _UNIT_RX.match(text, m.end())
        if um:
            hit = _unit_of(um.group(1))
            if hit:
                out.append((m.start(), um.end(), float(m.group(1)), hit[0], hit[1], hit[2]))
                continue
        nxt = text[m.end():m.end() + 1]
        if nxt.isalpha() or nxt == "^":
            continue                                   # '2nd', '3rd', a symbol subscript: not a quantity
        out.append((m.start(), m.end(), float(m.group(1)), "", "none", 1.0))
    return out


def _si(qty, kind):
    """A quantity's SI value for an input of `kind` (a temperature difference ignores the Celsius offset)."""
    _s, _e, v, unit, _dim, fac = qty
    if kind == "temperature" and unit in _OFFSET:
        return v + _OFFSET[unit]
    return v * fac


_KIND_DIM = {"reaction_rate": "zero_order_rate_constant", "first_order_rate_constant": "per_time", "frequency": "per_time", "temperature_difference": "temperature",
             "number": "none"}
_LABEL_TAIL = r"\s*(?:\([^)]*\)\s*)?(?:,\s*)?(?:=|:|of|is|was|at|equals?|equal to|to|reaches|becomes|by|is about)?\s*(?:about|approximately|exactly)?\s*$"


def _dim_of(inp):
    return inp.get("dim") or _KIND_DIM.get(inp.get("kind", "number"), inp.get("kind", "number"))


def _dim_ok(inp, d):
    """A stated quantity of dimension class `d` may bind to `inp` (its own class, or one it lists in `dims`)."""
    return d == _dim_of(inp) or d in inp.get("dims", ())


def _question_span(text):
    """The one sentence that asks (ends with '?' or starts with what/find/calculate ...), or None."""
    spans, start = [], 0
    for m in re.finditer(r"[.?!](?=\s|$)", text):
        spans.append((start, m.end(), text[m.end() - 1]))
        start = m.end()
    if start < len(text) and text[start:].strip():
        spans.append((start, len(text), ""))
    asks = [(a, b) for a, b, end in spans if end == "?" or
            re.match(r"\s*(?:what|find|calculate|compute|determine|how)\b", text[a:b], re.I)]
    return asks[0] if len(asks) == 1 else None


def _assign(text, inputs, preset):
    """Bind every number in `text` to one input: by its label (or a written symbol), by the state its wording names
    (initial / final, inherited along 'and'), by being the only one of its dimension. None when a number cannot be
    bound or two numbers compete for one input: never a guess."""
    qs = _quantities(text)
    got = dict(preset)
    free = [i for i in inputs if i["symbol"] not in got and i.get("type", "number") in ("number", "integer")]
    direct, hints = [None] * len(qs), [None] * len(qs)
    prev_end = 0
    for qi, q in enumerate(qs):
        prefix = text[prev_end:q[0]].lower()
        nxt = qs[qi + 1][0] if qi + 1 < len(qs) else len(text)
        suffix = text[q[1]:nxt].lower()
        best, best_len = None, -1
        for inp in free:
            if not _dim_ok(inp, q[4]):
                continue
            labs = list(inp.get("labels", [])) + ([_sym_label(inp["symbol"])] if inp.get("symbols") else [])
            for lab in labs:
                m = re.search(r"(?:" + lab + r")" + _LABEL_TAIL, prefix)
                if m and len(m.group(0)) > best_len:
                    best, best_len = inp, len(m.group(0))
            for sl in inp.get("suffix_labels", []):
                if best_len < 0 and re.match(r"\s*(?:of\s+)?(?:" + sl + r")", suffix):
                    best, best_len = inp, 0
        if best is not None:
            direct[qi] = best["symbol"]
            hints[qi] = best.get("role")
        elif qi > 0 and hints[qi - 1] and re.fullmatch(r"\s*(?:,|and|,\s*and|&)\s*", prefix):
            hints[qi] = hints[qi - 1]
        prev_end = q[1]
    qspan = _question_span(text)
    if qspan:
        for d in {q[4] for q in qs}:
            roles = {i.get("role") for i in free if _dim_of(i) == d}
            if not {"initial", "final"} <= roles:
                continue
            u = [qi for qi, q in enumerate(qs) if q[4] == d and not direct[qi] and not hints[qi]]
            inside = [qi for qi in u if qspan[0] <= qs[qi][0] < qspan[1]]
            if len(u) == 2 and len(inside) == 1:
                for qi in u:
                    hints[qi] = "final" if qi in inside else "initial"
    used = [s for s in direct if s]
    for qi, q in enumerate(qs):
        if direct[qi] or not hints[qi]:
            continue
        cands = [i for i in free if _dim_ok(i, q[4]) and i.get("role") == hints[qi] and i["symbol"] not in used]
        if len(cands) == 1:
            direct[qi] = cands[0]["symbol"]
            used.append(cands[0]["symbol"])
    if len(used) != len(set(used)):
        return None
    by = {i["symbol"]: i for i in free}
    for qi, s in enumerate(direct):
        if s:
            got[s] = _value_for(qs[qi], by[s])
    left_q = [qi for qi in range(len(qs)) if not direct[qi]]
    left_i = [i for i in free if i["symbol"] not in used and not i.get("label_only")]
    for qi in left_q:
        q = qs[qi]
        cands = [i for i in left_i if _dim_ok(i, q[4])]
        same_dim_q = [x for x in left_q if qs[x][4] == q[4]]
        if len(cands) != 1 or len(same_dim_q) != 1:
            return None                                # an unbound number, or two candidates: never guess
        got[cands[0]["symbol"]] = _value_for(q, cands[0])
        left_i.remove(cands[0])
    return got


def _value_for(q, inp):
    kind = inp.get("kind", "number")
    if kind == "number":
        v = q[2]
        if inp.get("transform") == "p10":
            v = 10.0 ** (-v)
        return v
    return _si(q, kind)


def _make_parse(spec):
    """The parser of one procedure: its keyword trigger, its text extractor, then the number binding."""
    kw = [re.compile(k) for k in spec.get("kw", [])]
    nokw = [re.compile(k) for k in spec.get("nokw", [])]
    inputs, text_fn, need, post = spec["inputs"], spec.get("text"), spec.get("need"), spec.get("post")

    def parse(question):
        if not isinstance(question, str):
            return None
        q = _norm(question)
        ql = q.lower()
        if not all(k.search(ql) for k in kw) or any(k.search(ql) for k in nokw):
            return None
        preset, masked = {}, q
        if text_fn:
            res = text_fn(q)
            if res is None:
                return None
            preset, spans = res
            for a, b in sorted(spans, reverse=True):
                masked = masked[:a] + " " * (b - a) + masked[b:]
        masked = _symbols_to_labels(masked, inputs)
        if need and not need(masked, _quantities(masked)):
            return None
        got = _assign(masked, inputs, preset)
        if got is None:
            return None
        if post:
            got = post(got, q)
        return got
    return parse


def _symbols_to_labels(text, inputs):
    """'Ka = 1.8e-5', 'P1 = 1 atm': a written symbol is an exact label (case-sensitive)."""
    for inp in inputs:
        for s in inp.get("symbols", []):
            text = re.sub(r"(?<![\w\[])" + s + r"(?![\w\]])\s*(=|:)", lambda m, i=inp: f" ⟦{i['symbol']}⟧ =", text)
    return text


def _sym_label(sym):
    return r"⟦" + re.escape(sym.lower()) + r"⟧\s*="


def _dims(qs):
    out = {}
    for q in qs:
        out[q[4]] = out.get(q[4], 0) + 1
    return out


def _has(*need, exactly=None, without=()):
    """A `need` predicate: the stated dimensions include `need` (counted), and none of `without`."""
    def f(_text, qs):
        d = _dims(qs)
        want = {}
        for n in need:
            want[n] = want.get(n, 0) + 1
        if exactly:
            return all(d.get(k, 0) == v for k, v in want.items()) and not any(d.get(w) for w in without)
        return all(d.get(k, 0) >= v for k, v in want.items()) and not any(d.get(w) for w in without)
    return f


# formula tokens in questions
_FTOKEN = r"[A-Z(\[](?:[A-Za-z0-9()\[\]]*)(?:[·•⋅∙*.]\d*[A-Z(\[][A-Za-z0-9()\[\]]*)*"


def _formula_at(q, cue, group_name="formula"):
    """The formula written right after `cue` (a regex), validated; (value, span) or (None, None)."""
    for m in re.finditer(r"(?:" + cue + r")\s*(" + _FTOKEN + r")", q):
        tok = m.group(1).rstrip(".")
        tok = re.sub(r"[.]$", "", tok)
        try:
            formula_counts(tok)
        except ValueError:
            continue
        if not re.search(r"[A-Z]", tok):
            continue
        return tok, (m.start(1), m.start(1) + len(tok))
    return None, None


def _element_at(q, cue):
    names = "|".join(sorted(ELEMENT_NAMES, key=len, reverse=True))
    m = re.search(r"(?:" + cue + r")\s*(" + names + r")\b", q, re.I)
    if m:
        return ELEMENT_NAMES[m.group(1).lower()], (m.start(1), m.end(1))
    m = re.search(r"(?:" + cue + r")\s*([A-Z][a-z]?)(?![A-Za-z0-9(])", q)
    if m and m.group(1) in W:
        return m.group(1), (m.start(1), m.end(1))
    return None, None


_EQTERM = r"(?:\d+\s*)?" + r"[A-Z(\[][A-Za-z0-9()\[\]·]*(?:\((?:s|l|g|aq)\))?"
_EQ_RX = re.compile(r"(" + _EQTERM + r"(?:\s*\+\s*" + _EQTERM + r")*\s*(?:<=>|<->|⇌|⇄|⟷|->|-->|→|⟶|=>|=)\s*" + _EQTERM
                    + r"(?:\s*\+\s*" + _EQTERM + r")*)")


def _equation_in(q):
    for m in _EQ_RX.finditer(q):
        try:
            parse_equation(m.group(1))
        except ValueError:
            continue
        return m.group(1).strip(), (m.start(1), m.end(1))
    return None, None


def _texts(*getters):
    """Combine text extractors: each returns (symbol, value, span) or None; a missing value is left out."""
    def f(q):
        preset, spans = {}, []
        for g in getters:
            r = g(q, spans)
            if r is None:                              # the extractor saw something it must not read: refuse
                return None
            for sym, val, span in r:
                if val is None:
                    continue
                if any(not (span[1] <= a or span[0] >= b) for a, b in spans):
                    continue
                preset[sym] = val
                spans.append(span)
        return preset, spans
    return f


def _tf(sym, cue):
    """A text extractor for one formula after `cue`."""
    def g(q, spans):
        masked = q
        for a, b in spans:
            masked = masked[:a] + " " * (b - a) + masked[b:]
        val, span = _formula_at(masked, cue)
        return [(sym, val, span)] if val else []
    return g


def _te(sym, cue):
    def g(q, spans):
        masked = q
        for a, b in spans:
            masked = masked[:a] + " " * (b - a) + masked[b:]
        val, span = _element_at(masked, cue)
        return [(sym, val, span)] if val else []
    return g


def _teq(sym="equation"):
    def g(q, spans):
        val, span = _equation_in(q)
        return [(sym, val, span)] if val else []
    return g


_MASS_OF = r"(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)\s*(kg|g|mg|grams|gram|kilograms|milligrams)\s+(?:of\s+)?(" + _FTOKEN + r")"
_MASSF = {"kg": 1.0, "g": 1e-3, "mg": 1e-6, "grams": 1e-3, "gram": 1e-3, "kilograms": 1.0, "milligrams": 1e-6}


def _mass_pairs(q):
    """'4.0 g of H2' pairs in order: [(mass in kg, formula, span)]."""
    out = []
    for m in re.finditer(_MASS_OF, q):
        tok = m.group(3).rstrip(".,")
        try:
            formula_counts(tok)
        except ValueError:
            continue
        out.append((float(m.group(1)) * _MASSF[m.group(2)], tok, (m.start(), m.start(3) + len(tok))))
    return out


# --- the procedure builder ------------------------------------------------------------------------------------------
PROCEDURES: dict = {}
LEVELS: dict = {}
_EX: dict = {}       # worked example inputs and question texts, attached with their outputs at the end of the file


def _inp(symbol, kind, meaning, *, labels=(), symbols=(), required=True, **extra):
    d = {"symbol": symbol, "kind": kind, "meaning": meaning, "required": required}
    if labels:
        d["labels"] = list(labels)
    if symbols:
        d["symbols"] = list(symbols)
    d.update(extra)
    return d


def _out(symbol, kind, meaning):
    return {"symbol": symbol, "kind": kind, "meaning": meaning}


def _proc(key, *, name, topic, level, purpose, inputs, outputs, primary, run, verify, reference, wrong, rand,
          templates, examples, questions, latex, method, conditions, kw=(), nokw=(), text=None, need=None, post=None):
    spec = {"name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
            "conditions": conditions, "method": method, "latex": latex, "run": run, "verify": verify,
            "reference": reference, "wrong": wrong, "random_inputs": rand, "templates": templates,
            "kw": list(kw), "nokw": list(nokw), "text": text, "need": need, "post": post}
    spec["parse"] = _make_parse(spec)
    for k in ("kw", "nokw", "text", "need", "post"):
        spec.pop(k)
    spec["examples"] = []
    spec["questions"] = []
    PROCEDURES[key] = spec
    LEVELS[key] = level
    _EX[key] = (examples, questions)


def _check_outputs(run, keys):
    """The default verifier: every listed output recomputed from the inputs, relative difference."""
    def verify(i, o):
        want = run(**i)
        return max(_rel(o[k], want[k]) for k in keys)
    return verify


def _u_(rng, lo, hi):
    """A log-uniform draw (realistic magnitudes)."""
    return math.exp(rng.uniform(math.log(lo), math.log(hi)))


# common labels
L_P1 = [r"\bp1", r"\binitial pressure", r"\boriginal pressure", r"\bstarting pressure", r"\binitially at",
        r"\bfrom", r"\binitially"]
L_P2 = [r"\bp2", r"\bfinal pressure", r"\bnew pressure", r"\bto"]
L_V1 = [r"\bv1", r"\binitial volume", r"\boriginal volume", r"\bstarting volume", r"\binitially occupies",
        r"\bfrom", r"\binitially"]
L_V2 = [r"\bv2", r"\bfinal volume", r"\bnew volume", r"\bto"]
L_T1 = [r"\bt1", r"\binitial temperature", r"\boriginal temperature", r"\bstarting temperature", r"\bfrom",
        r"\binitially"]
L_T2 = [r"\bt2", r"\bfinal temperature", r"\bnew temperature", r"\bto"]
ASK = r"(?:what|find|calculate|compute|determine|how)\b"

_RAND_FORMULA_EL = ["H", "C", "N", "O", "S", "P", "Cl", "Na", "K", "Ca", "Mg", "Fe", "Cu", "Al", "Br", "I", "F", "Zn",
                    "Ag", "Ba", "Mn", "Cr", "Ni", "Li", "Si", "B", "Co", "Pb", "Sn", "Hg"]


def _rand_formula(rng, hydrate=True, parens=True):
    s = "".join(e + (str(n) if n > 1 else "") for e, n in
                ((rng.choice(_RAND_FORMULA_EL), rng.choice([1, 1, 2, 3, 4, 6, 12])) for _ in range(rng.randint(1, 3))))
    if parens and rng.random() < 0.4:
        inner = "".join(e + (str(n) if n > 1 else "") for e, n in
                        ((rng.choice(["O", "H", "N", "C", "S", "P"]), rng.choice([1, 2, 3, 4])) for _ in range(2)))
        s += f"({inner}){rng.randint(2, 4)}"
    if hydrate and rng.random() < 0.3:
        s += f"·{rng.randint(1, 10)}H2O"
    return s


_GAS_FORMULAS = ["H2", "He", "N2", "O2", "CO2", "CH4", "NH3", "Ar", "Cl2", "SO2", "NO2", "C3H8", "C2H6", "Ne", "CO",
                 "H2S", "HCl", "Kr", "Xe", "F2", "C4H10", "N2O", "SF6", "UF6", "C2H4"]


# ====================================================================================================================
# STOICHIOMETRY
# ====================================================================================================================
def molar_mass(formula):
    mg = molar_mass_g(formula)
    return {"M": mg / 1000.0, "M_g": mg}


_proc("molar_mass", name="Molar mass of a formula", topic="stoichiometry / molar mass", level="beginner",
      purpose="The molar mass of a chemical formula (parentheses, brackets and hydrates are read).",
      inputs=[_inp("formula", "number", "chemical formula, e.g. Ca(OH)2 or CuSO4·5H2O", type="text", min_length=1,
                   max_length=80)],
      outputs=[_out("M", "molar_mass", "molar mass (kg/mol)"), _out("M_g", "number", "molar mass in g/mol")],
      primary="M", run=molar_mass,
      verify=lambda i, o: max(_rel(o["M_g"], _mm_rd(i["formula"])), _rel(o["M"] * 1000.0, _mm_rd(i["formula"]))),
      reference=lambda formula: {"M": _ref_mw(formula) / 1000.0, "M_g": _ref_mw(formula)},
      wrong=lambda formula: {"M_g": sum(n * round(W[e]) for e, n in formula_counts(formula).items()),
                             "M": sum(n * round(W[e]) for e, n in formula_counts(formula).items()) / 1000.0},
      rand=lambda rng: {"formula": _rand_formula(rng)},
      kw=[r"\b(?:molar mass|molecular weight|formula weight|molecular mass|formula mass|molar weight)\b"],
      nokw=[r"\b(?:density|effus|diffus|freezing|boiling|osmotic|empirical|molecular formula|unknown|sample|"
            r"occupies|moles?\b|grams?\b)"],
      text=_texts(_tf("formula", r"(?:molar mass|molecular weight|formula weight|molecular mass|formula mass|"
                                 r"molar weight)\s+of(?:\s+the compound)?")),
      need=lambda t, qs: not qs,
      latex=r"M = \sum_i n_i A_r(i)\ \mathrm{g\,mol^{-1}}",
      method="atom counts read from the formula (nested groups multiply, hydrate parts add), times standard atomic weights",
      conditions=["a neutral formula of element symbols with standard atomic weights"],
      templates=["What is the molar mass of Ca(OH)2?", "Find the molar mass of CuSO4·5H2O.",
                 "Calculate the molecular weight of C6H12O6."],
      examples=[{"formula": "H2O"}, {"formula": "Ca(OH)2"}, {"formula": "CuSO4·5H2O"}, {"formula": "K4[Fe(CN)6]"}],
      questions=["What is the molar mass of Ca(OH)2?", "Find the molar mass of CuSO4·5H2O.",
                 "Calculate the molecular weight of C6H12O6.", "What is the molar mass of Xy2O?"])


def moles_from_mass(formula, m):
    _pos(m=m)
    mg = molar_mass_g(formula)
    n = m / (mg / 1000.0)
    return {"n": n, "N": n * NA, "M": mg / 1000.0}


_proc("moles_from_mass", name="Moles from a mass", topic="stoichiometry / the mole", level="beginner",
      purpose="The amount of substance (and number of formula units) in a given mass of a compound.",
      inputs=[_inp("formula", "number", "chemical formula", type="text", min_length=1, max_length=80),
              _inp("m", "mass", "mass of the sample")],
      outputs=[_out("n", "amount", "amount of substance"), _out("N", "number", "number of formula units"),
               _out("M", "molar_mass", "molar mass")],
      primary="n", run=moles_from_mass,
      verify=lambda i, o: _rel(o["n"] * _mm_rd(i["formula"]) / 1000.0, i["m"]),
      reference=lambda formula, m: {"n": m * 1000.0 / _ref_mw(formula)},
      wrong=lambda formula, m: {"n": m * molar_mass_g(formula) / 1000.0},
      rand=lambda rng: {"formula": _rand_formula(rng), "m": _u_(rng, 1e-5, 2.0)},
      kw=[r"\b(?:how many moles|number of moles|amount of substance|moles (?:are|is) (?:there )?in|convert .* to moles|"
          r"how many mol\b|amount in moles)"],
      nokw=[r"\b(?:molecules|atoms|particles|formula units|ions)\b", r"\bproduced|formed|react"],
      text=_texts(_tf("formula", r"(?:kg|g|mg|grams|gram|kilograms|milligrams)\s+of")),
      need=_has("mass", exactly=True),
      latex=r"n = \frac{m}{M}", method="n = m / M with M from the formula",
      conditions=["m > 0", "a neutral formula"],
      templates=["How many moles are in 36.0 g of H2O?", "How many moles are there in 10.0 g of NaCl?"],
      examples=[{"formula": "H2O", "m": 0.036}, {"formula": "NaCl", "m": 0.01}],
      questions=["How many moles are in 36.0 g of H2O?", "How many moles are there in 10.0 g of NaCl?",
                 "How many moles are in 250 mg of CaCO3?", "How many moles are in -5 g of H2O?"])


def mass_from_moles(formula, n):
    _pos(n=n)
    mg = molar_mass_g(formula)
    return {"m": n * mg / 1000.0, "m_g": n * mg, "N": n * NA}


_proc("mass_from_moles", name="Mass from moles", topic="stoichiometry / the mole", level="beginner",
      purpose="The mass of a given amount (moles) of a compound.",
      inputs=[_inp("formula", "number", "chemical formula", type="text", min_length=1, max_length=80),
              _inp("n", "amount", "amount of substance")],
      outputs=[_out("m", "mass", "mass"), _out("m_g", "number", "mass in grams"),
               _out("N", "number", "number of formula units")],
      primary="m", run=mass_from_moles,
      verify=lambda i, o: _rel(o["m"], i["n"] * _mm_rd(i["formula"]) / 1000.0),
      reference=lambda formula, n: {"m": n * _ref_mw(formula) / 1000.0},
      wrong=lambda formula, n: {"m": n / molar_mass_g(formula) / 1000.0},
      rand=lambda rng: {"formula": _rand_formula(rng), "n": _u_(rng, 1e-4, 20.0)},
      kw=[r"\b(?:mass|how many grams|how many kilograms|how many milligrams|weigh)"],
      nokw=[r"\b(?:percent|fraction|molar mass|molecular weight|element|produced|formed|react|yield|deposit)"],
      text=_texts(_tf("formula", r"(?:mol|moles|mole|mmol)\s+of")),
      need=_has("amount", exactly=True, without=("mass",)),
      latex=r"m = n M", method="m = n M with M from the formula",
      conditions=["n > 0", "a neutral formula"],
      templates=["What is the mass of 2.50 mol of NaCl?", "How many grams are in 0.150 mol of H2SO4?"],
      examples=[{"formula": "NaCl", "n": 2.5}, {"formula": "H2SO4", "n": 0.15}],
      questions=["What is the mass of 2.50 mol of NaCl?", "How many grams are in 0.150 mol of H2SO4?",
                 "What is the mass of 3 mmol of C6H12O6?"])


def particles_from_mass(formula, m):
    _pos(m=m)
    n = m / (molar_mass_g(formula) / 1000.0)
    return {"N": n * NA, "n": n}


_proc("particles_from_mass", name="Number of molecules in a mass", topic="stoichiometry / the mole", level="beginner",
      purpose="The number of molecules (formula units) in a given mass of a compound.",
      inputs=[_inp("formula", "number", "chemical formula", type="text", min_length=1, max_length=80),
              _inp("m", "mass", "mass of the sample")],
      outputs=[_out("N", "number", "number of molecules / formula units"), _out("n", "amount", "amount of substance")],
      primary="N", run=particles_from_mass,
      verify=lambda i, o: _rel(o["N"], i["m"] * 1000.0 / _mm_rd(i["formula"]) * NA),
      reference=lambda formula, m: {"N": m * 1000.0 / _ref_mw(formula) * __import__("scipy.constants").constants.Avogadro},
      wrong=lambda formula, m: {"N": m * 1000.0 / molar_mass_g(formula)},
      rand=lambda rng: {"formula": _rand_formula(rng), "m": _u_(rng, 1e-6, 5.0)},
      kw=[r"\b(?:how many|number of)\s+(?:molecules|formula units|particles)\b"],
      nokw=[r"\batoms?\b", r"\bhow many moles\b"],
      text=_texts(_tf("formula", r"(?:kg|g|mg|grams|gram|kilograms|milligrams)\s+of")),
      need=_has("mass", exactly=True),
      latex=r"N = \frac{m}{M} N_A", method="N = (m / M) N_A",
      conditions=["m > 0"],
      templates=["How many molecules are in 10.0 g of CO2?", "How many formula units are in 5.00 g of NaCl?"],
      examples=[{"formula": "CO2", "m": 0.01}, {"formula": "NaCl", "m": 0.005}],
      questions=["How many molecules are in 10.0 g of CO2?", "How many formula units are in 5.00 g of NaCl?"])


def particles_from_moles(n):
    _pos(n=n)
    return {"N": n * NA}


_proc("particles_from_moles", name="Number of particles in an amount", topic="stoichiometry / the mole",
      level="beginner", purpose="The number of particles in a given amount of substance (Avogadro's number).",
      inputs=[_inp("n", "amount", "amount of substance")],
      outputs=[_out("N", "number", "number of particles")],
      primary="N", run=particles_from_moles,
      verify=lambda i, o: _rel(o["N"] / NA, i["n"]),
      reference=lambda n: {"N": n * __import__("scipy.constants").constants.Avogadro},
      wrong=lambda n: {"N": n * 6.02e22},
      rand=lambda rng: {"n": _u_(rng, 1e-6, 100.0)},
      kw=[r"\b(?:how many|number of)\s+(?:molecules|atoms|formula units|particles|ions)\b"],
      nokw=[r"\bhow many moles\b", r"\batoms of\b"],
      need=_has("amount", exactly=True, without=("mass",)),
      latex=r"N = n N_A", method="N = n N_A",
      conditions=["n > 0"],
      templates=["How many molecules are in 0.250 mol of water?", "How many atoms are in 2.0 mol of helium?"],
      examples=[{"n": 0.25}, {"n": 2.0}],
      questions=["How many molecules are in 0.250 mol of water?", "How many atoms are in 2.0 mol of helium?"])


def moles_from_particles(N):
    _pos(N=N)
    return {"n": N / NA}


_proc("moles_from_particles", name="Moles from a number of particles", topic="stoichiometry / the mole",
      level="beginner", purpose="The amount of substance that contains a given number of particles.",
      inputs=[_inp("N", "number", "number of particles",
                   suffix_labels=[r"molecules|atoms|particles|ions|formula units"])],
      outputs=[_out("n", "amount", "amount of substance")],
      primary="n", run=moles_from_particles,
      verify=lambda i, o: _rel(o["n"] * NA, i["N"]),
      reference=lambda N: {"n": N / __import__("scipy.constants").constants.Avogadro},
      wrong=lambda N: {"n": N * NA / 1e47},
      rand=lambda rng: {"N": _u_(rng, 1e18, 1e26)},
      kw=[r"\bhow many moles|number of moles|amount (?:of substance )?in moles",
          r"\b(?:molecules|atoms|particles|ions|formula units)\b"],
      need=lambda t, qs: _dims(qs).get("none", 0) == 1 and len(qs) == 1,
      latex=r"n = \frac{N}{N_A}", method="n = N / N_A",
      conditions=["N > 0"],
      templates=["How many moles are 3.01e23 molecules of CO2?", "How many moles is 1.204 × 10^24 atoms of iron?"],
      examples=[{"N": 3.01e23}, {"N": 1.204e24}],
      questions=["How many moles are 3.01e23 molecules of CO2?", "How many moles is 1.204 × 10^24 atoms of iron?"])


def atoms_of_element(formula, element, m):
    _pos(m=m)
    el = _element(element)
    counts = formula_counts(formula)
    if el not in counts:
        raise ValueError(f"{el} does not occur in {formula}")
    n = m / (molar_mass_g(formula) / 1000.0)
    return {"N_atoms": n * counts[el] * NA, "n_element": n * counts[el]}


_proc("atoms_of_element", name="Atoms of an element in a mass of compound", topic="stoichiometry / the mole",
      level="intermediate", purpose="How many atoms of one element a given mass of a compound contains.",
      inputs=[_inp("formula", "number", "chemical formula of the compound", type="text", min_length=1, max_length=80),
              _inp("element", "number", "the element (symbol or name)", type="text", min_length=1, max_length=20),
              _inp("m", "mass", "mass of the compound")],
      outputs=[_out("N_atoms", "number", "number of atoms of the element"),
               _out("n_element", "amount", "amount of the element")],
      primary="N_atoms", run=atoms_of_element,
      verify=lambda i, o: _rel(o["N_atoms"], i["m"] * 1000.0 / _mm_rd(i["formula"]) *
                               _counts_rd(i["formula"])[_element(i["element"])] * NA),
      reference=lambda formula, element, m: {"N_atoms": m * 1000.0 / _ref_mw(formula) * _ref_counts(formula)[element]
                                             * 6.02214076e23},
      wrong=lambda formula, element, m: {"N_atoms": m * 1000.0 / molar_mass_g(formula) * NA},
      rand=lambda rng: (lambda f: {"formula": f, "element": rng.choice(sorted(formula_counts(f))),
                                   "m": _u_(rng, 1e-4, 10.0)})(_rand_formula(rng)),
      kw=[r"\b(?:how many|number of)\s+(?:\w+\s+)?atoms of\b"],
      text=_texts(_te("element", r"atoms of"), _tf("formula", r"(?:kg|g|mg|grams|gram|kilograms|milligrams)\s+of")),
      need=_has("mass", exactly=True),
      latex=r"N_X = \frac{m}{M} \, \nu_X \, N_A", method="moles of compound times the number of X atoms per formula, times N_A",
      conditions=["m > 0", "the element occurs in the formula"],
      templates=["How many atoms of oxygen are in 10.0 g of CO2?", "How many atoms of H are in 5.00 g of CH4?"],
      examples=[{"formula": "CO2", "element": "O", "m": 0.01}, {"formula": "CH4", "element": "H", "m": 0.005}],
      questions=["How many atoms of oxygen are in 10.0 g of CO2?", "How many atoms of H are in 5.00 g of CH4?",
                 "How many atoms of N are in 10.0 g of CO2?"])


def mass_percent(formula, element):
    el = _element(element)
    counts = formula_counts(formula)
    if el not in counts:
        raise ValueError(f"{el} does not occur in {formula}")
    total = molar_mass_g(formula)
    comp = {e: 100.0 * n * W[e] / total for e, n in counts.items()}
    return {"percent": comp[el], "composition": comp}


_proc("mass_percent", name="Mass percent of an element", topic="stoichiometry / composition", level="beginner",
      purpose="The percentage by mass of an element in a compound (and the full percent composition).",
      inputs=[_inp("formula", "number", "chemical formula", type="text", min_length=1, max_length=80),
              _inp("element", "number", "the element (symbol or name)", type="text", min_length=1, max_length=20)],
      outputs=[_out("percent", "number", "mass percent of the element (%)"),
               _out("composition", "number", "mass percent of every element (%)")],
      primary="percent", run=mass_percent,
      verify=lambda i, o: _rel(o["percent"], 100.0 * _counts_rd(i["formula"])[_element(i["element"])] *
                               W[_element(i["element"])] / _mm_rd(i["formula"])),
      reference=lambda formula, element: {"percent": 100.0 * _ref_counts(formula)[element] * _ref_w(element)
                                          / _ref_mw(formula)},
      wrong=lambda formula, element: {"percent": 100.0 * formula_counts(formula)[element]
                                      / sum(formula_counts(formula).values())},
      rand=lambda rng: (lambda f: {"formula": f, "element": rng.choice(sorted(formula_counts(f)))})(_rand_formula(rng)),
      kw=[r"\b(?:mass percent|percent by mass|percentage by mass|mass percentage|percent composition|"
          r"percentage composition|mass fraction|percent of)\b"],
      nokw=[r"\byield\b", r"\bsolution\b", r"\bempirical\b"],
      text=_texts(_te("element", r"(?:percent|percentage|fraction|composition) (?:by mass )?of"),
                  _tf("formula", r"\bin")),
      need=lambda t, qs: not qs,
      latex=r"w_X = \frac{\nu_X A_r(X)}{M} \times 100\%", method="mass of the element per formula over the molar mass",
      conditions=["the element occurs in the formula"],
      templates=["What is the mass percent of O in H2SO4?", "Find the percent by mass of nitrogen in NH4NO3."],
      examples=[{"formula": "H2SO4", "element": "O"}, {"formula": "NH4NO3", "element": "N"}],
      questions=["What is the mass percent of O in H2SO4?", "Find the percent by mass of nitrogen in NH4NO3.",
                 "What is the mass percent of Fe in Fe2O3?", "What is the mass percent of Cl in H2SO4?"])


def mass_of_element(formula, element, m):
    _pos(m=m)
    el = _element(element)
    counts = formula_counts(formula)
    if el not in counts:
        raise ValueError(f"{el} does not occur in {formula}")
    frac = counts[el] * W[el] / molar_mass_g(formula)
    return {"m_element": m * frac, "fraction": frac}


_proc("mass_of_element", name="Mass of an element in a sample", topic="stoichiometry / composition",
      level="intermediate", purpose="The mass of one element contained in a given mass of a compound.",
      inputs=[_inp("formula", "number", "chemical formula", type="text", min_length=1, max_length=80),
              _inp("element", "number", "the element (symbol or name)", type="text", min_length=1, max_length=20),
              _inp("m", "mass", "mass of the compound")],
      outputs=[_out("m_element", "mass", "mass of the element"), _out("fraction", "number", "mass fraction")],
      primary="m_element", run=mass_of_element,
      verify=lambda i, o: _rel(o["m_element"], i["m"] * _counts_rd(i["formula"])[_element(i["element"])]
                               * W[_element(i["element"])] / _mm_rd(i["formula"])),
      reference=lambda formula, element, m: {"m_element": m * _ref_counts(formula)[element] * _ref_w(element)
                                             / _ref_mw(formula)},
      wrong=lambda formula, element, m: {"m_element": m * W[element] / molar_mass_g(formula)},
      rand=lambda rng: (lambda f: {"formula": f, "element": rng.choice([e for e, n in formula_counts(f).items()]),
                                   "m": _u_(rng, 1e-4, 10.0)})(_rand_formula(rng, parens=True)),
      kw=[r"\b(?:what|find the|calculate the|how much)\s+mass of\s+\w+\s+(?:is|are)?\s*(?:in|contained in|present in)\b"],
      text=_texts(_te("element", r"mass of"), _tf("formula", r"(?:kg|g|mg|grams|gram|kilograms|milligrams)\s+of")),
      need=_has("mass", exactly=True),
      latex=r"m_X = m \, \frac{\nu_X A_r(X)}{M}", method="the sample mass times the element's mass fraction",
      conditions=["m > 0", "the element occurs in the formula"],
      templates=["What mass of iron is in 50.0 g of Fe2O3?", "What mass of N is contained in 10.0 kg of NH4NO3?"],
      examples=[{"formula": "Fe2O3", "element": "Fe", "m": 0.05}, {"formula": "NH4NO3", "element": "N", "m": 10.0}],
      questions=["What mass of iron is in 50.0 g of Fe2O3?", "What mass of N is contained in 10.0 kg of NH4NO3?"])


def _composition(text):
    pairs = re.findall(r"([A-Z][a-z]?)\s*[:=]?\s*(\d+(?:\.\d+)?)", text)
    if len(pairs) < 2:
        raise ValueError("give at least two elements with their percentages, e.g. 'C 40.0, H 6.71, O 53.3'")
    comp = {}
    for el, v in pairs:
        el = _element(el)
        if el in comp:
            raise ValueError(f"{el} is given twice")
        comp[el] = float(v)
        if comp[el] <= 0:
            raise ValueError("every percentage must be positive")
    return comp


def empirical_formula(composition):
    comp = _composition(composition)
    order = list(comp)
    moles = {e: comp[e] / W[e] for e in order}
    lo = min(moles.values())
    ratio = {e: moles[e] / lo for e in order}
    for k in range(1, 13):
        ints = {e: round(k * r) for e, r in ratio.items()}
        if all(ints[e] >= 1 and abs(k * ratio[e] - ints[e]) <= 0.015 * ints[e] for e in order):
            g = 0
            for v in ints.values():
                g = math.gcd(g, v)
            ints = {e: v // g for e, v in ints.items()}
            return {"formula": _write_formula(order, ints), "counts": [float(ints[e]) for e in order]}
    raise ValueError("the composition does not fit a small whole-number ratio (up to 12)")


def _verify_empirical(i, o):
    comp = _composition(i["composition"])
    counts = _counts_rd(o["formula"])
    if set(counts) != set(comp):
        return 1.0
    g = 0
    for v in counts.values():
        g = math.gcd(g, v)
    if g != 1:
        return 1.0
    tot_given = sum(comp.values())
    tot_mass = sum(counts[e] * W[e] for e in counts)
    dev = max(abs(counts[e] * W[e] / tot_mass - comp[e] / tot_given) / (comp[e] / tot_given) for e in comp)
    return 0.0 if dev <= 0.03 else 1.0


def _ref_empirical(composition):
    comp = _composition(composition)
    order = list(comp)
    moles = {e: comp[e] / _ref_w(e) for e in order}
    lo = min(moles.values())
    fr = {e: Fraction(moles[e] / lo).limit_denominator(12) for e in order}
    lcm = 1
    for f in fr.values():
        lcm = lcm * f.denominator // math.gcd(lcm, f.denominator)
    ints = {e: int(fr[e] * lcm) for e in order}
    g = 0
    for v in ints.values():
        g = math.gcd(g, v)
    return {"formula": "".join(e + (str(ints[e] // g) if ints[e] // g != 1 else "") for e in order)}


def _rand_composition(rng):
    els = rng.sample(["C", "H", "O", "N", "S", "Cl", "Na", "K", "Fe", "Ca", "P", "Mg"], rng.randint(2, 4))
    while True:
        counts = {e: rng.randint(1, 6) for e in els}
        g = 0
        for v in counts.values():
            g = math.gcd(g, v)
        if g == 1:
            break
    tot = sum(counts[e] * W[e] for e in els)
    return {"composition": ", ".join(f"{e} {100.0 * counts[e] * W[e] / tot:.3f}" for e in els)}


def _comp_text(q, spans):
    pairs = re.findall(r"(\d+(?:\.\d+)?)\s*%\s*(?:of\s+)?([A-Z][a-z]?|" + "|".join(ELEMENT_NAMES) + r")\b", q)
    pairs = [(v, e) for v, e in pairs if e in W or e.lower() in ELEMENT_NAMES]
    if len(pairs) < 2:
        return []
    comp = ", ".join(f"{_element(e) if e not in W else e} {v}" for v, e in pairs)
    return [("composition", comp, (0, len(q)))]


_proc("empirical_formula", name="Empirical formula from percent composition", topic="stoichiometry / composition",
      level="intermediate", purpose="The simplest whole-number formula of a compound from its mass percentages.",
      inputs=[_inp("composition", "number", "element percentages (or masses), e.g. 'C 40.0, H 6.71, O 53.3'",
                   type="text", min_length=3, max_length=200)],
      outputs=[_out("formula", "text", "empirical formula"), _out("counts", "number", "subscripts in the order given")],
      primary="formula", run=empirical_formula, verify=_verify_empirical, reference=_ref_empirical,
      wrong=lambda composition: (lambda c: {"formula": _write_formula(list(c), {e: max(1, round(v / min(c.values())))
                                                                                for e, v in c.items()})})(
          _composition(composition)),
      rand=_rand_composition,
      kw=[r"\bempirical formula\b"], nokw=[r"\bmolecular formula\b", r"\bmolar mass\b"],
      text=_texts(_comp_text),
      latex=r"n_i = \frac{w_i}{A_r(i)},\quad \text{subscripts} \propto \frac{n_i}{\min_j n_j}",
      method="moles per 100 g, divided by the smallest, multiplied by the smallest k (1..12) giving whole numbers within 1.5%",
      conditions=["at least two elements", "a whole-number ratio with subscripts up to 12 times the smallest"],
      templates=["Find the empirical formula of a compound with 40.0% C, 6.71% H and 53.3% O.",
                 "What is the empirical formula of a compound that is 85.6% C and 14.4% H?"],
      examples=[{"composition": "C 40.0, H 6.71, O 53.3"}, {"composition": "C 85.6, H 14.4"},
                {"composition": "Fe 69.94, O 30.06"}],
      questions=["Find the empirical formula of a compound with 40.0% C, 6.71% H and 53.3% O.",
                 "What is the empirical formula of a compound that is 85.6% C and 14.4% H?",
                 "Find the empirical formula of a compound containing 69.94% Fe and 30.06% O."])


def molecular_formula(empirical, M):
    _pos(M=M)
    counts = formula_counts(empirical)
    order = list(dict.fromkeys(re.findall(r"[A-Z][a-z]?", empirical)))
    me = molar_mass_g(empirical) / 1000.0
    k = M / me
    n = round(k)
    if n < 1 or abs(k - n) > 0.02 * n:
        raise ValueError(f"the molar mass is {k:.3f} times the empirical formula mass, not a whole multiple")
    return {"multiple": float(n), "formula": _write_formula(order, {e: c * n for e, c in counts.items()})}


_proc("molecular_formula", name="Molecular formula from the empirical formula", topic="stoichiometry / composition",
      level="intermediate", purpose="The molecular formula from an empirical formula and the molar mass.",
      inputs=[_inp("empirical", "number", "empirical formula", type="text", min_length=1, max_length=60),
              _inp("M", "molar_mass", "molar mass of the compound")],
      outputs=[_out("multiple", "number", "whole-number multiple n"), _out("formula", "text", "molecular formula")],
      primary="multiple", run=molecular_formula,
      verify=lambda i, o: 0.0 if (o["multiple"] == round(o["multiple"]) and
                                   abs(i["M"] / (_mm_rd(i["empirical"]) / 1000.0) - o["multiple"]) <= 0.02 * o["multiple"]
                                   and _counts_rd(o["formula"]) == {e: c * int(o["multiple"]) for e, c in
                                                                    _counts_rd(i["empirical"]).items()}) else 1.0,
      reference=lambda empirical, M: {"multiple": float(round(M * 1000.0 / _ref_mw(empirical)))},
      wrong=lambda empirical, M: {"multiple": float(round(M * 1000.0 / molar_mass_g(empirical)) + 1)},
      rand=lambda rng: (lambda f, n: {"empirical": f, "M": molar_mass_g(f) * n / 1000.0 * (1 + rng.uniform(-0.005, 0.005))})(
          rng.choice(["CH2O", "CH", "CH2", "NO2", "C2H5", "CH3O", "C3H4", "HO", "C2H3O", "P2O5", "CH4N", "C4H5N2O"]),
          rng.randint(1, 6)),
      kw=[r"\bmolecular formula\b"],
      text=_texts(_tf("empirical", r"empirical formula(?: of| is)?")),
      need=_has("molar_mass", exactly=True),
      latex=r"n = \frac{M}{M_{\text{empirical}}}", method="n = M / M(empirical), rounded (must be within 2% of a whole number)",
      conditions=["M > 0", "M is a whole multiple of the empirical formula mass (within 2%)"],
      templates=["A compound has the empirical formula CH2O and a molar mass of 180.16 g/mol. What is its molecular formula?"],
      examples=[{"empirical": "CH2O", "M": 0.18016}, {"empirical": "CH", "M": 0.07811}],
      questions=["A compound has the empirical formula CH2O and a molar mass of 180.16 g/mol. What is its molecular formula?",
                 "The empirical formula is CH and the molar mass is 78.11 g/mol. Find the molecular formula.",
                 "The empirical formula is CH2O and the molar mass is 100 g/mol. Find the molecular formula."])


def balance_equation(equation):
    coefs, species, nr = balance(equation)
    return {"coefficients": [float(c) for c in coefs], "balanced": _written(coefs, species, nr)}


def _verify_balance(i, o):
    reactants, products = parse_equation(i["equation"])
    species = [f for _c, f in reactants + products]
    c = o["coefficients"]
    if len(c) != len(species) or any(x <= 0 or x != int(x) for x in c):
        return 1.0
    c = [int(x) for x in c]
    g = 0
    for x in c:
        g = math.gcd(g, x)
    return 0.0 if g == 1 and _conserved(c, species, len(reactants)) else 1.0


_REACTIONS = ["H2 + O2 -> H2O", "CH4 + O2 -> CO2 + H2O", "C3H8 + O2 -> CO2 + H2O", "Fe + O2 -> Fe2O3",
              "Al + O2 -> Al2O3", "N2 + H2 -> NH3", "KClO3 -> KCl + O2", "C2H6 + O2 -> CO2 + H2O",
              "Fe2O3 + CO -> Fe + CO2", "Al + HCl -> AlCl3 + H2", "C6H12O6 + O2 -> CO2 + H2O", "Na + H2O -> NaOH + H2",
              "Ca(OH)2 + H3PO4 -> Ca3(PO4)2 + H2O", "NH3 + O2 -> NO + H2O", "C4H10 + O2 -> CO2 + H2O",
              "Cu + HNO3 -> Cu(NO3)2 + NO + H2O", "KMnO4 + HCl -> KCl + MnCl2 + H2O + Cl2", "P4 + O2 -> P4O10",
              "Zn + HCl -> ZnCl2 + H2", "H2O2 -> H2O + O2", "SO2 + O2 -> SO3", "NO2 + H2O -> HNO3 + NO",
              "C8H18 + O2 -> CO2 + H2O", "Fe + Cl2 -> FeCl3", "Mg + N2 -> Mg3N2", "Al2(SO4)3 + Ca(OH)2 -> Al(OH)3 + CaSO4",
              "C2H5OH + O2 -> CO2 + H2O", "FeS2 + O2 -> Fe2O3 + SO2", "Pb(NO3)2 -> PbO + NO2 + O2",
              "NaHCO3 -> Na2CO3 + CO2 + H2O", "Ca3(PO4)2 + SiO2 + C -> CaSiO3 + P4 + CO", "Cr2O3 + Al -> Al2O3 + Cr",
              "K + Br2 -> KBr", "HCl + NaOH -> NaCl + H2O", "CaCO3 -> CaO + CO2", "Li + N2 -> Li3N",
              "C12H22O11 + O2 -> CO2 + H2O", "Na2O2 + H2O -> NaOH + O2", "AgNO3 + Cu -> Cu(NO3)2 + Ag",
              "NH4NO3 -> N2O + H2O", "Si + Cl2 -> SiCl4", "B2H6 + O2 -> B2O3 + H2O"]

_proc("balance_equation", name="Balancing a chemical equation", topic="stoichiometry / chemical equations",
      level="intermediate", purpose="The smallest whole-number coefficients that balance a chemical equation.",
      inputs=[_inp("equation", "number", "the equation, e.g. 'C3H8 + O2 -> CO2 + H2O'", type="text", min_length=4,
                   max_length=300)],
      outputs=[_out("balanced", "text", "the balanced equation"),
               _out("coefficients", "number", "coefficients in the order the species are written")],
      primary="balanced", run=balance_equation, verify=_verify_balance,
      reference=lambda equation: {"coefficients": [float(x) for x in _ref_balance(equation)[0]]},
      wrong=lambda equation: {"coefficients": [1.0] * len(balance(equation)[0]), "balanced": "?"},
      rand=lambda rng: {"equation": rng.choice(_REACTIONS)},
      kw=[r"\bbalanc"], text=_texts(_teq()), need=lambda t, qs: not qs,
      latex=r"A\,\boldsymbol{\nu} = \mathbf{0},\ \boldsymbol{\nu} \in \mathbb{Z}_{>0}^{n}",
      method="exact rational null space of the element-conservation matrix, scaled to the smallest whole numbers",
      conditions=["one independent way to balance (a one-dimensional null space)", "neutral species"],
      templates=["Balance the equation C3H8 + O2 -> CO2 + H2O.", "Balance: Fe + O2 -> Fe2O3"],
      examples=[{"equation": "C3H8 + O2 -> CO2 + H2O"}, {"equation": "Fe + O2 -> Fe2O3"},
                {"equation": "Cu + HNO3 -> Cu(NO3)2 + NO + H2O"}],
      questions=["Balance the equation C3H8 + O2 -> CO2 + H2O.", "Balance: Fe + O2 -> Fe2O3",
                 "Balance KMnO4 + HCl → KCl + MnCl2 + H2O + Cl2.", "Balance the equation H2 + O2 -> NaCl."])


def _role(species, nr, formula):
    if formula not in species:
        raise ValueError(f"{formula} does not appear in the equation")
    j = species.index(formula)
    return j, j < nr


def limiting_reagent(equation, reactant_a, m_a, reactant_b, m_b, product):
    _pos(m_a=m_a, m_b=m_b)
    coefs, species, nr = balance(equation)
    ja, ra = _role(species, nr, reactant_a)
    jb, rb = _role(species, nr, reactant_b)
    jp, rp = _role(species, nr, product)
    if not (ra and rb) or ja == jb:
        raise ValueError("the two given substances must be two different reactants")
    if rp:
        raise ValueError(f"{product} is a reactant, not a product")
    na, nb = m_a / molar_mass_g(reactant_a) * 1000.0, m_b / molar_mass_g(reactant_b) * 1000.0
    xa, xb = na / coefs[ja], nb / coefs[jb]
    ext, lim, other, n_o, j_o = (xa, reactant_a, reactant_b, nb, jb) if xa <= xb else (xb, reactant_b, reactant_a, na, ja)
    m_p = ext * coefs[jp] * molar_mass_g(product) / 1000.0
    left = (n_o - ext * coefs[j_o]) * molar_mass_g(other) / 1000.0
    return {"product_mass": m_p, "limiting": lim, "excess_left": left}


def _verify_limiting(i, o):
    coefs, species, nr = balance(i["equation"])
    ja, jb, jp = species.index(i["reactant_a"]), species.index(i["reactant_b"]), species.index(i["product"])
    na, nb = i["m_a"] * 1000.0 / _mm_rd(i["reactant_a"]), i["m_b"] * 1000.0 / _mm_rd(i["reactant_b"])
    ext = min(na / coefs[ja], nb / coefs[jb])
    return _rel(o["product_mass"], ext * coefs[jp] * _mm_rd(i["product"]) / 1000.0)


def _ref_limiting(equation, reactant_a, m_a, reactant_b, m_b, product):
    v, species, nr = _ref_balance(equation)
    ja, jb, jp = species.index(reactant_a), species.index(reactant_b), species.index(product)
    xa, xb = m_a * 1000.0 / _ref_mw(reactant_a) / v[ja], m_b * 1000.0 / _ref_mw(reactant_b) / v[jb]
    return {"product_mass": min(xa, xb) * v[jp] * _ref_mw(product) / 1000.0,
            "limiting": reactant_a if xa <= xb else reactant_b}


def _rand_two_reactants(rng):
    while True:
        eq = rng.choice(_REACTIONS)
        r, p = parse_equation(eq)
        if len(r) >= 2:
            a, b = rng.sample([f for _c, f in r], 2)
            return {"equation": eq, "reactant_a": a, "m_a": _u_(rng, 1e-3, 1.0), "reactant_b": b,
                    "m_b": _u_(rng, 1e-3, 1.0), "product": rng.choice([f for _c, f in p])}


def _limiting_text(q, spans):
    eq, span = _equation_in(q)
    if not eq:
        return []
    out = [("equation", eq, span)]
    masked = q[:span[0]] + " " * (span[1] - span[0]) + q[span[1]:]
    pairs = _mass_pairs(masked)
    if len(pairs) > 2:
        return None
    for (mass, f, sp), (ms, fs) in zip(pairs, (("m_a", "reactant_a"), ("m_b", "reactant_b"))):
        out += [(ms, mass, sp), (fs, f, (sp[1], sp[1]))]
    p, pspan = _formula_at(masked, r"(?:mass of|grams of|how much)")
    if p:
        out.append(("product", p, pspan))
    return out


_proc("limiting_reagent", name="Limiting reagent and theoretical yield", topic="stoichiometry / reaction yields",
      level="intermediate",
      purpose="Which of two reactants limits a reaction, the mass of a product formed and the excess left over.",
      inputs=[_inp("equation", "number", "the reaction (balanced or not)", type="text", min_length=4, max_length=300),
              _inp("reactant_a", "number", "first reactant", type="text", min_length=1, max_length=60),
              _inp("m_a", "mass", "mass of the first reactant"),
              _inp("reactant_b", "number", "second reactant", type="text", min_length=1, max_length=60),
              _inp("m_b", "mass", "mass of the second reactant"),
              _inp("product", "number", "the product asked for", type="text", min_length=1, max_length=60)],
      outputs=[_out("product_mass", "mass", "mass of product formed"), _out("limiting", "text", "limiting reagent"),
               _out("excess_left", "mass", "mass of the excess reagent left over")],
      primary="product_mass", run=limiting_reagent, verify=_verify_limiting, reference=_ref_limiting,
      wrong=lambda equation, reactant_a, m_a, reactant_b, m_b, product: (lambda na, nb: {
          "product_mass": min(na, nb) * molar_mass_g(product) / 1000.0,
          "limiting": reactant_a if na <= nb else reactant_b})(
          m_a * 1000.0 / molar_mass_g(reactant_a), m_b * 1000.0 / molar_mass_g(reactant_b)),
      rand=_rand_two_reactants,
      kw=[r"\blimiting (?:reagent|reactant)"], text=_texts(_limiting_text), need=lambda t, qs: not qs,
      latex=r"\xi = \min_i \frac{n_i}{\nu_i},\quad m_P = \xi\,\nu_P M_P",
      method="balance the equation, moles of each reactant over its coefficient; the smallest is the extent",
      conditions=["the equation balances uniquely", "two different reactants and one product of the equation"],
      templates=["For H2 + O2 -> H2O, 4.0 g of H2 react with 16.0 g of O2. Which is the limiting reagent and what mass of H2O forms?"],
      examples=[{"equation": "H2 + O2 -> H2O", "reactant_a": "H2", "m_a": 0.004, "reactant_b": "O2", "m_b": 0.016,
                 "product": "H2O"},
                {"equation": "N2 + H2 -> NH3", "reactant_a": "N2", "m_a": 0.028, "reactant_b": "H2", "m_b": 0.010,
                 "product": "NH3"}],
      questions=["For H2 + O2 -> H2O, 4.0 g of H2 react with 16.0 g of O2. Which is the limiting reagent and what mass of H2O forms?",
                 "In N2 + 3H2 -> 2NH3, 28.0 g of N2 and 10.0 g of H2 are mixed. Find the limiting reactant and the mass of NH3 produced.",
                 "For H2 + O2 -> H2O, 4.0 g of H2 react with oxygen. Which is the limiting reagent and what mass of H2O forms?"])


def theoretical_yield(equation, reactant, m, product):
    _pos(m=m)
    coefs, species, nr = balance(equation)
    jr, rr = _role(species, nr, reactant)
    jp, rp = _role(species, nr, product)
    if not rr or rp:
        raise ValueError("give a reactant and a product of the equation")
    n_p = m * 1000.0 / molar_mass_g(reactant) * coefs[jp] / coefs[jr]
    return {"product_mass": n_p * molar_mass_g(product) / 1000.0, "n_product": n_p}


def _yield_text(q, spans):
    eq, span = _equation_in(q)
    if not eq:
        return []
    out = [("equation", eq, span)]
    masked = q[:span[0]] + " " * (span[1] - span[0]) + q[span[1]:]
    r, p = parse_equation(eq)
    reac, prod = [f for _c, f in r], [f for _c, f in p]
    for mass, f, sp in _mass_pairs(masked):
        if f in reac and "reactant" not in [o[0] for o in out]:
            out += [("m", mass, sp), ("reactant", f, (sp[1], sp[1]))]
        elif f in prod and "actual" not in [o[0] for o in out]:
            out += [("actual", mass, sp), ("product", f, (sp[1], sp[1]))]
    if "product" not in [o[0] for o in out]:
        pf, pspan = _formula_at(masked, r"(?:mass of|grams of|yield of)")
        if pf:
            out.append(("product", pf, pspan))
    return out


_proc("theoretical_yield", name="Theoretical yield from one reactant", topic="stoichiometry / reaction yields",
      level="intermediate", purpose="The mass of a product formed from a given mass of one reactant (in excess of the others).",
      inputs=[_inp("equation", "number", "the reaction (balanced or not)", type="text", min_length=4, max_length=300),
              _inp("reactant", "number", "the reactant whose mass is given", type="text", min_length=1, max_length=60),
              _inp("m", "mass", "mass of the reactant"),
              _inp("product", "number", "the product asked for", type="text", min_length=1, max_length=60)],
      outputs=[_out("product_mass", "mass", "theoretical mass of product"), _out("n_product", "amount", "moles of product")],
      primary="product_mass", run=theoretical_yield,
      verify=lambda i, o: (lambda c, s, nr: _rel(o["product_mass"], i["m"] * 1000.0 / _mm_rd(i["reactant"])
                                                 * c[s.index(i["product"])] / c[s.index(i["reactant"])]
                                                 * _mm_rd(i["product"]) / 1000.0))(*balance(i["equation"])),
      reference=lambda equation, reactant, m, product: (lambda v, s, nr: {
          "product_mass": m * 1000.0 / _ref_mw(reactant) * v[s.index(product)] / v[s.index(reactant)] * _ref_mw(product)
          / 1000.0})(*_ref_balance(equation)),
      wrong=lambda equation, reactant, m, product: {"product_mass": m * molar_mass_g(product) / molar_mass_g(reactant)},
      rand=lambda rng: (lambda eq: (lambda r, p: {"equation": eq, "reactant": rng.choice([f for _c, f in r]),
                                                  "m": _u_(rng, 1e-3, 10.0), "product": rng.choice([f for _c, f in p])})(
          *parse_equation(eq)))(rng.choice(_REACTIONS)),
      kw=[r"\b(?:theoretical yield|what mass of|how many (?:grams|kilograms|milligrams) of|how much)\b",
          r"\b(?:produc|form|obtain|yield|made|make)"],
      nokw=[r"\blimiting\b", r"\bpercent(?:age)? yield\b", r"\bvolume\b", r"\bactual\b"],
      text=_texts(_yield_text), need=lambda t, qs: not qs,
      latex=r"m_P = \frac{m_R}{M_R}\,\frac{\nu_P}{\nu_R}\,M_P",
      method="balance the equation; moles of reactant times the coefficient ratio times the product's molar mass",
      conditions=["the equation balances uniquely", "the other reactants are in excess"],
      templates=["For CH4 + O2 -> CO2 + H2O, what mass of CO2 is produced from 16.0 g of CH4?",
                 "What is the theoretical yield of Fe from 100 g of Fe2O3 in Fe2O3 + CO -> Fe + CO2?"],
      examples=[{"equation": "CH4 + O2 -> CO2 + H2O", "reactant": "CH4", "m": 0.016, "product": "CO2"},
                {"equation": "Fe2O3 + CO -> Fe + CO2", "reactant": "Fe2O3", "m": 0.1, "product": "Fe"}],
      questions=["For CH4 + O2 -> CO2 + H2O, what mass of CO2 is produced from 16.0 g of CH4?",
                 "What is the theoretical yield of Fe from 100 g of Fe2O3 in Fe2O3 + CO -> Fe + CO2?",
                 "For CH4 + O2 -> CO2 + H2O, what mass of CO2 is produced from methane?"])


def percent_yield(equation, reactant, m, product, actual):
    _pos(m=m, actual=actual)
    t = theoretical_yield(equation, reactant, m, product)["product_mass"]
    return {"percent_yield": 100.0 * actual / t, "theoretical_mass": t}


_proc("percent_yield", name="Percent yield of a reaction", topic="stoichiometry / reaction yields",
      level="intermediate", purpose="The percent yield from the mass of reactant used and the mass of product obtained.",
      inputs=[_inp("equation", "number", "the reaction (balanced or not)", type="text", min_length=4, max_length=300),
              _inp("reactant", "number", "the reactant whose mass is given", type="text", min_length=1, max_length=60),
              _inp("m", "mass", "mass of the reactant"),
              _inp("product", "number", "the product obtained", type="text", min_length=1, max_length=60),
              _inp("actual", "mass", "actual mass of product obtained")],
      outputs=[_out("percent_yield", "number", "percent yield (%)"),
               _out("theoretical_mass", "mass", "theoretical mass of product")],
      primary="percent_yield", run=percent_yield,
      verify=lambda i, o: (lambda c, s, nr: _rel(o["percent_yield"] / 100.0 * (
          i["m"] * 1000.0 / _mm_rd(i["reactant"]) * c[s.index(i["product"])] / c[s.index(i["reactant"])]
          * _mm_rd(i["product"]) / 1000.0), i["actual"]))(*balance(i["equation"])),
      reference=lambda equation, reactant, m, product, actual: (lambda v, s, nr: {
          "percent_yield": 100.0 * actual / (m * 1000.0 / _ref_mw(reactant) * v[s.index(product)] / v[s.index(reactant)]
                                             * _ref_mw(product) / 1000.0)})(*_ref_balance(equation)),
      wrong=lambda equation, reactant, m, product, actual: {"percent_yield": 100.0 * actual / m},
      rand=lambda rng: (lambda eq: (lambda r, p: (lambda d: dict(d, actual=theoretical_yield(**d)["product_mass"]
                                                                  * rng.uniform(0.3, 0.99)))(
          {"equation": eq, "reactant": rng.choice([f for _c, f in r]), "m": _u_(rng, 1e-3, 10.0),
           "product": rng.choice([f for _c, f in p])}))(*parse_equation(eq)))(rng.choice(_REACTIONS)),
      kw=[r"\bpercent(?:age)? yield\b"], text=_texts(_yield_text), need=lambda t, qs: not qs,
      latex=r"\%\,\text{yield} = \frac{m_{\text{actual}}}{m_{\text{theoretical}}} \times 100",
      method="theoretical yield from the balanced equation, then actual / theoretical x 100",
      conditions=["the equation balances uniquely", "the other reactants are in excess"],
      templates=["For CaCO3 -> CaO + CO2, 100.0 g of CaCO3 gave 50.0 g of CaO. What is the percent yield?"],
      examples=[{"equation": "CaCO3 -> CaO + CO2", "reactant": "CaCO3", "m": 0.1, "product": "CaO", "actual": 0.05},
                {"equation": "N2 + H2 -> NH3", "reactant": "N2", "m": 0.028, "product": "NH3", "actual": 0.0255}],
      questions=["For CaCO3 -> CaO + CO2, 100.0 g of CaCO3 gave 50.0 g of CaO. What is the percent yield?",
                 "In N2 + H2 -> NH3, 28.0 g of N2 produced 25.5 g of NH3. Calculate the percentage yield."])


# ====================================================================================================================
# GASES
# ====================================================================================================================
def _ig_pressure(n, V, T):
    _pos(n=n, V=V, T=T)
    return {"P": n * R * T / V, "P_atm": n * R * T / V / ATM}


def _ref_vm(T, P):
    from chemicals.volume import ideal_gas
    return ideal_gas(T, P)


_NO_IG = [r"\bvan der waals\b", r"\breal gas\b", r"\binitial|final|new |originally|initially|compress|expand|"
          r"heated|cooled|changed\b", r"\bpartial\b", r"\bdensity\b", r"\bmolar mass\b"]

_proc("ideal_gas_pressure", name="Ideal gas law: pressure", topic="gases / ideal gas law", level="beginner",
      purpose="The pressure of an ideal gas from its amount, volume and temperature (PV = nRT).",
      inputs=[_inp("n", "amount", "amount of gas"), _inp("V", "volume", "volume"), _inp("T", "temperature", "temperature")],
      outputs=[_out("P", "pressure", "pressure"), _out("P_atm", "number", "pressure in atm")],
      primary="P", run=_ig_pressure, verify=lambda i, o: _rel(o["P"] * i["V"], i["n"] * R * i["T"]),
      reference=lambda n, V, T: {"P": n * _ref_vm(T, 1.0) / V},
      wrong=lambda n, V, T: {"P": n * R * (T - 273.15) / V},
      rand=lambda rng: {"n": _u_(rng, 1e-3, 50.0), "V": _u_(rng, 1e-4, 1.0), "T": rng.uniform(150.0, 1500.0)},
      kw=[ASK + r"[^.?]*\bpressure\b"], nokw=_NO_IG,
      need=_has("amount", "volume", "temperature", exactly=True, without=("pressure", "mass")),
      latex=r"P = \frac{nRT}{V}", method="P = nRT / V",
      conditions=["n, V, T > 0 (T in kelvin)"],
      templates=["What is the pressure of 2.0 mol of gas in 10.0 L at 300 K?",
                 "Find the pressure exerted by 0.50 mol of an ideal gas in a 5.0 L container at 25 °C."],
      examples=[{"n": 2.0, "V": 0.01, "T": 300.0}, {"n": 0.5, "V": 0.005, "T": 298.15}],
      questions=["What is the pressure of 2.0 mol of gas in 10.0 L at 300 K?",
                 "Find the pressure exerted by 0.50 mol of an ideal gas in a 5.0 L container at 25 °C.",
                 "What is the pressure of 2.0 mol of gas in 10.0 L?"])


def _ig_volume(n, P, T):
    _pos(n=n, P=P, T=T)
    return {"V": n * R * T / P, "V_L": n * R * T / P * 1000.0}


_proc("ideal_gas_volume", name="Ideal gas law: volume", topic="gases / ideal gas law", level="beginner",
      purpose="The volume of an ideal gas from its amount, pressure and temperature.",
      inputs=[_inp("n", "amount", "amount of gas"), _inp("P", "pressure", "pressure"),
              _inp("T", "temperature", "temperature")],
      outputs=[_out("V", "volume", "volume"), _out("V_L", "number", "volume in litres")],
      primary="V", run=_ig_volume, verify=lambda i, o: _rel(o["V"] * i["P"], i["n"] * R * i["T"]),
      reference=lambda n, P, T: {"V": n * _ref_vm(T, P)},
      wrong=lambda n, P, T: {"V": n * R * (T - 273.15) / P},
      rand=lambda rng: {"n": _u_(rng, 1e-3, 50.0), "P": _u_(rng, 1e3, 1e7), "T": rng.uniform(150.0, 1500.0)},
      kw=[ASK + r"[^.?]*\b(?:volume|how many lit)"], nokw=_NO_IG,
      need=_has("amount", "pressure", "temperature", exactly=True, without=("volume", "mass")),
      latex=r"V = \frac{nRT}{P}", method="V = nRT / P", conditions=["n, P, T > 0 (T in kelvin)"],
      templates=["What volume does 1.00 mol of gas occupy at 1.00 atm and 273.15 K?",
                 "Calculate the volume of 0.250 mol of an ideal gas at 2.00 atm and 350 K."],
      examples=[{"n": 1.0, "P": 101325.0, "T": 273.15}, {"n": 0.25, "P": 202650.0, "T": 350.0}],
      questions=["What volume does 1.00 mol of gas occupy at 1.00 atm and 273.15 K?",
                 "Calculate the volume of 0.250 mol of an ideal gas at 2.00 atm and 350 K."])


def _ig_moles(P, V, T):
    _pos(P=P, V=V, T=T)
    return {"n": P * V / (R * T)}


_proc("ideal_gas_moles", name="Ideal gas law: amount", topic="gases / ideal gas law", level="beginner",
      purpose="The amount (moles) of an ideal gas from its pressure, volume and temperature.",
      inputs=[_inp("P", "pressure", "pressure"), _inp("V", "volume", "volume"), _inp("T", "temperature", "temperature")],
      outputs=[_out("n", "amount", "amount of gas")],
      primary="n", run=_ig_moles, verify=lambda i, o: _rel(o["n"] * R * i["T"], i["P"] * i["V"]),
      reference=lambda P, V, T: {"n": V / _ref_vm(T, P)},
      wrong=lambda P, V, T: {"n": P * V / (R * (T - 273.15))},
      rand=lambda rng: {"P": _u_(rng, 1e3, 1e7), "V": _u_(rng, 1e-4, 1.0), "T": rng.uniform(150.0, 1500.0)},
      kw=[r"\b(?:how many moles|number of moles|amount of gas|how many mol\b|amount \(in moles\))"], nokw=_NO_IG,
      need=_has("pressure", "volume", "temperature", exactly=True, without=("amount", "mass")),
      latex=r"n = \frac{PV}{RT}", method="n = PV / RT", conditions=["P, V, T > 0 (T in kelvin)"],
      templates=["How many moles of gas are in a 5.00 L flask at 2.00 atm and 300 K?"],
      examples=[{"P": 202650.0, "V": 0.005, "T": 300.0}, {"P": 101325.0, "V": 0.0224, "T": 273.15}],
      questions=["How many moles of gas are in a 5.00 L flask at 2.00 atm and 300 K?",
                 "How many moles of gas occupy 22.4 L at 1.00 atm and 0 °C?"])


def _ig_temperature(P, V, n):
    _pos(P=P, V=V, n=n)
    T = P * V / (n * R)
    return {"T": T, "T_C": T - 273.15}


_proc("ideal_gas_temperature", name="Ideal gas law: temperature", topic="gases / ideal gas law", level="beginner",
      purpose="The temperature of an ideal gas from its pressure, volume and amount.",
      inputs=[_inp("P", "pressure", "pressure"), _inp("V", "volume", "volume"), _inp("n", "amount", "amount of gas")],
      outputs=[_out("T", "temperature", "temperature"), _out("T_C", "number", "temperature in °C")],
      primary="T", run=_ig_temperature, verify=lambda i, o: _rel(o["T"] * i["n"] * R, i["P"] * i["V"]),
      reference=lambda P, V, n: {"T": __import__("scipy.optimize").optimize.brentq(
          lambda T: n * _ref_vm(T, P) - V, 1e-6, 1e9, xtol=1e-14, rtol=1e-15)},
      wrong=lambda P, V, n: {"T": P * V / (n * R) - 273.15},
      rand=lambda rng: {"P": _u_(rng, 1e3, 1e7), "V": _u_(rng, 1e-4, 1.0), "n": _u_(rng, 1e-3, 50.0)},
      kw=[ASK + r"[^.?]*\btemperature\b"], nokw=_NO_IG,
      need=_has("pressure", "volume", "amount", exactly=True, without=("temperature", "mass")),
      latex=r"T = \frac{PV}{nR}", method="T = PV / nR", conditions=["P, V, n > 0"],
      templates=["At what temperature does 2.00 mol of gas occupy 50.0 L at 1.00 atm?",
                 "Find the temperature of 0.100 mol of gas in a 2.00 L vessel at 150 kPa."],
      examples=[{"P": 101325.0, "V": 0.05, "n": 2.0}, {"P": 150000.0, "V": 0.002, "n": 0.1}],
      questions=["At what temperature does 2.00 mol of gas occupy 50.0 L at 1.00 atm?",
                 "Find the temperature of 0.100 mol of gas in a 2.00 L vessel at 150 kPa."])


def gas_mass(formula, P, V, T):
    _pos(P=P, V=V, T=T)
    n = P * V / (R * T)
    return {"m": n * molar_mass_g(formula) / 1000.0, "n": n}


_proc("ideal_gas_mass", name="Mass of a gas sample (ideal gas)", topic="gases / ideal gas law", level="intermediate",
      purpose="The mass of a named gas that fills a volume at a given pressure and temperature.",
      inputs=[_inp("formula", "number", "formula of the gas", type="text", min_length=1, max_length=40),
              _inp("P", "pressure", "pressure"), _inp("V", "volume", "volume"), _inp("T", "temperature", "temperature")],
      outputs=[_out("m", "mass", "mass of gas"), _out("n", "amount", "amount of gas")],
      primary="m", run=gas_mass,
      verify=lambda i, o: _rel(o["m"] * R * i["T"], i["P"] * i["V"] * _mm_rd(i["formula"]) / 1000.0),
      reference=lambda formula, P, V, T: {"m": V / _ref_vm(T, P) * _ref_mw(formula) / 1000.0},
      wrong=lambda formula, P, V, T: {"m": P * V / (R * T) * molar_mass_g(formula)},
      rand=lambda rng: {"formula": rng.choice(_GAS_FORMULAS), "P": _u_(rng, 1e3, 1e7), "V": _u_(rng, 1e-4, 1.0),
                        "T": rng.uniform(150.0, 1500.0)},
      kw=[r"\b(?:what mass|how many grams|mass of)\b"], nokw=_NO_IG + [r"\bpercent\b"],
      text=_texts(_tf("formula", r"(?:mass of|grams of)")),
      need=_has("pressure", "volume", "temperature", exactly=True, without=("mass", "amount")),
      latex=r"m = \frac{PV}{RT}\,M", method="n = PV/RT, then m = nM", conditions=["P, V, T > 0"],
      templates=["What mass of CO2 occupies 5.00 L at 1.00 atm and 300 K?"],
      examples=[{"formula": "CO2", "P": 101325.0, "V": 0.005, "T": 300.0}],
      questions=["What mass of CO2 occupies 5.00 L at 1.00 atm and 300 K?",
                 "How many grams of N2 fill a 10.0 L tank at 2.00 atm and 25 °C?"])


def gas_density(formula, P, T):
    _pos(P=P, T=T)
    M = molar_mass_g(formula) / 1000.0
    return {"rho": P * M / (R * T)}


_proc("gas_density", name="Density of an ideal gas", topic="gases / ideal gas law", level="intermediate",
      purpose="The density of a named gas at a given pressure and temperature (ideal gas).",
      inputs=[_inp("formula", "number", "formula of the gas", type="text", min_length=1, max_length=40),
              _inp("P", "pressure", "pressure"), _inp("T", "temperature", "temperature")],
      outputs=[_out("rho", "density", "density")],
      primary="rho", run=gas_density,
      verify=lambda i, o: _rel(o["rho"] * R * i["T"], i["P"] * _mm_rd(i["formula"]) / 1000.0),
      reference=lambda formula, P, T: {"rho": _ref_mw(formula) / 1000.0 / _ref_vm(T, P)},
      wrong=lambda formula, P, T: {"rho": molar_mass_g(formula) / 1000.0 * R * T / P},
      rand=lambda rng: {"formula": rng.choice(_GAS_FORMULAS), "P": _u_(rng, 1e3, 1e7), "T": rng.uniform(150.0, 1500.0)},
      kw=[r"\bdensity\b"], nokw=[r"\bmolar mass\b", r"\bsolution\b"],
      text=_texts(_tf("formula", r"(?:density of(?: the gas)?|density of gaseous)")),
      need=_has("pressure", "temperature", exactly=True, without=("density", "volume", "mass")),
      latex=r"\rho = \frac{PM}{RT}", method="rho = PM / RT", conditions=["P, T > 0"],
      templates=["What is the density of CO2 at 1.00 atm and 298 K?"],
      examples=[{"formula": "CO2", "P": 101325.0, "T": 298.0}, {"formula": "N2", "P": 200000.0, "T": 350.0}],
      questions=["What is the density of CO2 at 1.00 atm and 298 K?",
                 "Calculate the density of N2 at 200 kPa and 350 K."])


def molar_mass_from_density(rho, P, T):
    _pos(rho=rho, P=P, T=T)
    M = rho * R * T / P
    return {"M": M, "M_g": M * 1000.0}


_proc("molar_mass_from_gas_density", name="Molar mass from gas density", topic="gases / ideal gas law",
      level="intermediate", purpose="The molar mass of a gas from its density at a known pressure and temperature.",
      inputs=[_inp("rho", "density", "gas density"), _inp("P", "pressure", "pressure"),
              _inp("T", "temperature", "temperature")],
      outputs=[_out("M", "molar_mass", "molar mass"), _out("M_g", "number", "molar mass in g/mol")],
      primary="M", run=molar_mass_from_density, verify=lambda i, o: _rel(o["M"] * i["P"], i["rho"] * R * i["T"]),
      reference=lambda rho, P, T: {"M": rho * _ref_vm(T, P)},
      wrong=lambda rho, P, T: {"M": rho * R * T / P / 1000.0},
      rand=lambda rng: {"rho": _u_(rng, 0.05, 50.0), "P": _u_(rng, 1e3, 1e7), "T": rng.uniform(150.0, 1500.0)},
      kw=[r"\bmolar mass|molecular weight\b"],
      need=_has("density", "pressure", "temperature", exactly=True, without=("mass", "volume")),
      latex=r"M = \frac{\rho R T}{P}", method="M = rho R T / P", conditions=["rho, P, T > 0"],
      templates=["A gas has a density of 1.25 g/L at 1.00 atm and 273 K. What is its molar mass?"],
      examples=[{"rho": 1.25, "P": 101325.0, "T": 273.0}],
      questions=["A gas has a density of 1.25 g/L at 1.00 atm and 273 K. What is its molar mass?",
                 "Find the molar mass of a gas whose density is 2.0 g/L at 0.95 atm and 300 K."])


def molar_mass_from_sample(m, V, P, T):
    _pos(m=m, V=V, P=P, T=T)
    M = m * R * T / (P * V)
    return {"M": M, "M_g": M * 1000.0}


_proc("molar_mass_from_gas_sample", name="Molar mass from a gas sample", topic="gases / ideal gas law",
      level="intermediate", purpose="The molar mass of a gas from a sample's mass, volume, pressure and temperature.",
      inputs=[_inp("m", "mass", "mass of the sample"), _inp("V", "volume", "volume"), _inp("P", "pressure", "pressure"),
              _inp("T", "temperature", "temperature")],
      outputs=[_out("M", "molar_mass", "molar mass"), _out("M_g", "number", "molar mass in g/mol")],
      primary="M", run=molar_mass_from_sample, verify=lambda i, o: _rel(o["M"] * i["P"] * i["V"], i["m"] * R * i["T"]),
      reference=lambda m, V, P, T: {"M": m / (V / _ref_vm(T, P))},
      wrong=lambda m, V, P, T: {"M": m * R * T / (P * V) * 1000.0},
      rand=lambda rng: {"m": _u_(rng, 1e-4, 1.0), "V": _u_(rng, 1e-4, 1.0), "P": _u_(rng, 1e3, 1e7),
                        "T": rng.uniform(150.0, 1500.0)},
      kw=[r"\bmolar mass|molecular weight\b"],
      nokw=[r"osmotic|protein|solution"],
      need=_has("mass", "volume", "pressure", "temperature", exactly=True, without=("density",)),
      latex=r"M = \frac{mRT}{PV}", method="M = mRT / PV", conditions=["m, V, P, T > 0"],
      templates=["A 1.00 g sample of a gas occupies 0.500 L at 1.00 atm and 300 K. What is its molar mass?"],
      examples=[{"m": 0.001, "V": 0.0005, "P": 101325.0, "T": 300.0}],
      questions=["A 1.00 g sample of a gas occupies 0.500 L at 1.00 atm and 300 K. What is its molar mass?",
                 "A 2.50 g sample of gas fills 1.20 L at 750 torr and 25 °C. Find its molar mass."])


def boyle_v2(P1, V1, P2):
    _pos(P1=P1, V1=V1, P2=P2)
    return {"V2": P1 * V1 / P2}


_GASLAW_NO = [r"\bvan der waals\b", r"\bmoles?\b|\bmol\b"]
_proc("boyle_final_volume", name="Boyle's law: final volume", topic="gases / gas laws", level="beginner",
      purpose="The volume of a gas after a pressure change at constant temperature (P1V1 = P2V2).",
      inputs=[_inp("P1", "pressure", "initial pressure", labels=L_P1, symbols=["P1", "P_1"], role="initial"),
              _inp("V1", "volume", "initial volume", labels=L_V1, symbols=["V1", "V_1"], role="initial"),
              _inp("P2", "pressure", "final pressure", labels=L_P2, symbols=["P2", "P_2"], role="final")],
      outputs=[_out("V2", "volume", "final volume")],
      primary="V2", run=boyle_v2, verify=lambda i, o: _rel(o["V2"] * i["P2"], i["P1"] * i["V1"]),
      reference=lambda P1, V1, P2: {"V2": V1 / _ref_vm(300.0, P1) * _ref_vm(300.0, P2)},
      wrong=lambda P1, V1, P2: {"V2": V1 * P2 / P1},
      rand=lambda rng: {"P1": _u_(rng, 1e3, 1e7), "V1": _u_(rng, 1e-5, 1.0), "P2": _u_(rng, 1e3, 1e7)},
      kw=[ASK + r"[^.?]*\b(?:volume)\b"], nokw=_GASLAW_NO,
      need=_has("pressure", "pressure", "volume", exactly=True, without=("temperature",)),
      latex=r"V_2 = \frac{P_1 V_1}{P_2}", method="V2 = P1 V1 / P2", conditions=["P1, V1, P2 > 0", "constant temperature and amount"],
      templates=["A gas occupies 2.00 L at 1.00 atm. What is its volume when compressed to 3.00 atm at constant temperature?",
                 "P1 = 100 kPa, V1 = 5.0 L, P2 = 250 kPa. Find the final volume (constant temperature)."],
      examples=[{"P1": 101325.0, "V1": 0.002, "P2": 303975.0}, {"P1": 1e5, "V1": 0.005, "P2": 2.5e5}],
      questions=["A gas occupies 2.00 L at 1.00 atm. What is its volume when compressed to 3.00 atm at constant temperature?",
                 "P1 = 100 kPa, V1 = 5.0 L, P2 = 250 kPa. Find the final volume (constant temperature).",
                 "A gas at 1.00 atm and 2.00 atm occupies 2.00 L. What is its volume?"])


def boyle_p2(P1, V1, V2):
    _pos(P1=P1, V1=V1, V2=V2)
    return {"P2": P1 * V1 / V2}


_proc("boyle_final_pressure", name="Boyle's law: final pressure", topic="gases / gas laws", level="beginner",
      purpose="The pressure of a gas after a volume change at constant temperature.",
      inputs=[_inp("P1", "pressure", "initial pressure", labels=L_P1, symbols=["P1", "P_1"], role="initial"),
              _inp("V1", "volume", "initial volume", labels=L_V1, symbols=["V1", "V_1"], role="initial"),
              _inp("V2", "volume", "final volume", labels=L_V2, symbols=["V2", "V_2"], role="final")],
      outputs=[_out("P2", "pressure", "final pressure")],
      primary="P2", run=boyle_p2, verify=lambda i, o: _rel(o["P2"] * i["V2"], i["P1"] * i["V1"]),
      reference=lambda P1, V1, V2: {"P2": __import__("scipy.optimize").optimize.brentq(
          lambda P: V1 / _ref_vm(300.0, P1) * _ref_vm(300.0, P) - V2, P1 * V1 / V2 * 0.5, P1 * V1 / V2 * 2.0,
          xtol=1e-12, rtol=1e-15)},
      wrong=lambda P1, V1, V2: {"P2": P1 * V2 / V1},
      rand=lambda rng: {"P1": _u_(rng, 1e3, 1e7), "V1": _u_(rng, 1e-5, 1.0), "V2": _u_(rng, 1e-5, 1.0)},
      kw=[ASK + r"[^.?]*\b(?:pressure)\b"], nokw=_GASLAW_NO,
      need=_has("pressure", "volume", "volume", exactly=True, without=("temperature",)),
      latex=r"P_2 = \frac{P_1 V_1}{V_2}", method="P2 = P1 V1 / V2", conditions=["P1, V1, V2 > 0", "constant temperature and amount"],
      templates=["A gas at 1.00 atm occupies 4.00 L. What is the new pressure if it is compressed to 1.00 L at constant temperature?"],
      examples=[{"P1": 101325.0, "V1": 0.004, "V2": 0.001}],
      questions=["A gas at 1.00 atm occupies 4.00 L. What is the new pressure if it is compressed to 1.00 L at constant temperature?",
                 "P1 = 2.0 atm, V1 = 3.0 L, V2 = 6.0 L. Find the final pressure."])


def charles_v2(V1, T1, T2):
    _pos(V1=V1, T1=T1, T2=T2)
    return {"V2": V1 * T2 / T1}


_proc("charles_final_volume", name="Charles's law: final volume", topic="gases / gas laws", level="beginner",
      purpose="The volume of a gas after a temperature change at constant pressure (V1/T1 = V2/T2).",
      inputs=[_inp("V1", "volume", "initial volume", labels=L_V1, symbols=["V1", "V_1"], role="initial"),
              _inp("T1", "temperature", "initial temperature", labels=L_T1, symbols=["T1", "T_1"], role="initial"),
              _inp("T2", "temperature", "final temperature", labels=L_T2, symbols=["T2", "T_2"], role="final")],
      outputs=[_out("V2", "volume", "final volume")],
      primary="V2", run=charles_v2, verify=lambda i, o: _rel(o["V2"] * i["T1"], i["V1"] * i["T2"]),
      reference=lambda V1, T1, T2: {"V2": V1 / _ref_vm(T1, 1e5) * _ref_vm(T2, 1e5)},
      wrong=lambda V1, T1, T2: {"V2": V1 * (T2 - 273.15) / (T1 - 273.15)},
      rand=lambda rng: {"V1": _u_(rng, 1e-5, 1.0), "T1": rng.uniform(150.0, 1200.0), "T2": rng.uniform(150.0, 1200.0)},
      kw=[ASK + r"[^.?]*\b(?:volume)\b"], nokw=_GASLAW_NO,
      need=_has("volume", "temperature", "temperature", exactly=True, without=("pressure",)),
      latex=r"V_2 = V_1 \frac{T_2}{T_1}", method="V2 = V1 T2 / T1 (kelvin)",
      conditions=["V1 > 0, T1, T2 > 0 K", "constant pressure and amount"],
      templates=["A balloon holds 2.50 L at 25 °C. What is its volume when heated to 75 °C at constant pressure?"],
      examples=[{"V1": 0.0025, "T1": 298.15, "T2": 348.15}],
      questions=["A balloon holds 2.50 L at 25 °C. What is its volume when heated to 75 °C at constant pressure?",
                 "V1 = 1.0 L, T1 = 300 K, T2 = 600 K. Find the final volume."])


def gay_lussac_p2(P1, T1, T2):
    _pos(P1=P1, T1=T1, T2=T2)
    return {"P2": P1 * T2 / T1}


_proc("gay_lussac_final_pressure", name="Gay-Lussac's law: final pressure", topic="gases / gas laws", level="beginner",
      purpose="The pressure of a gas after a temperature change at constant volume (P1/T1 = P2/T2).",
      inputs=[_inp("P1", "pressure", "initial pressure", labels=L_P1, symbols=["P1", "P_1"], role="initial"),
              _inp("T1", "temperature", "initial temperature", labels=L_T1, symbols=["T1", "T_1"], role="initial"),
              _inp("T2", "temperature", "final temperature", labels=L_T2, symbols=["T2", "T_2"], role="final")],
      outputs=[_out("P2", "pressure", "final pressure")],
      primary="P2", run=gay_lussac_p2, verify=lambda i, o: _rel(o["P2"] * i["T1"], i["P1"] * i["T2"]),
      reference=lambda P1, T1, T2: {"P2": __import__("scipy.optimize").optimize.brentq(
          lambda P: _ref_vm(T2, P) - _ref_vm(T1, P1), P1 * T2 / T1 * 0.5, P1 * T2 / T1 * 2.0, xtol=1e-12, rtol=1e-15)},
      wrong=lambda P1, T1, T2: {"P2": P1 * (T2 - 273.15) / (T1 - 273.15)},
      rand=lambda rng: {"P1": _u_(rng, 1e3, 1e7), "T1": rng.uniform(150.0, 1200.0), "T2": rng.uniform(150.0, 1200.0)},
      kw=[ASK + r"[^.?]*\b(?:pressure)\b"], nokw=_GASLAW_NO,
      need=_has("pressure", "temperature", "temperature", exactly=True, without=("volume",)),
      latex=r"P_2 = P_1 \frac{T_2}{T_1}", method="P2 = P1 T2 / T1 (kelvin)",
      conditions=["P1 > 0, T1, T2 > 0 K", "constant volume and amount"],
      templates=["A tyre is at 2.20 atm at 20 °C. What is the pressure when it warms to 45 °C at constant volume?"],
      examples=[{"P1": 2.2 * ATM, "T1": 293.15, "T2": 318.15}],
      questions=["A tyre is at 2.20 atm at 20 °C. What is the pressure when it warms to 45 °C at constant volume?",
                 "P1 = 100 kPa, T1 = 300 K, T2 = 450 K. Find the final pressure."])


def combined_v2(P1, V1, T1, P2, T2):
    _pos(P1=P1, V1=V1, T1=T1, P2=P2, T2=T2)
    return {"V2": P1 * V1 * T2 / (T1 * P2)}


def combined_p2(P1, V1, T1, V2, T2):
    _pos(P1=P1, V1=V1, T1=T1, V2=V2, T2=T2)
    return {"P2": P1 * V1 * T2 / (T1 * V2)}


def combined_t2(P1, V1, T1, P2, V2):
    _pos(P1=P1, V1=V1, T1=T1, P2=P2, V2=V2)
    return {"T2": P2 * V2 * T1 / (P1 * V1)}


_CG = lambda: [_inp("P1", "pressure", "initial pressure", labels=L_P1, symbols=["P1", "P_1"], role="initial"),
               _inp("V1", "volume", "initial volume", labels=L_V1, symbols=["V1", "V_1"], role="initial"),
               _inp("T1", "temperature", "initial temperature", labels=L_T1, symbols=["T1", "T_1"], role="initial")]
_cg_rand = lambda rng: {"P1": _u_(rng, 1e3, 1e7), "V1": _u_(rng, 1e-5, 1.0), "T1": rng.uniform(150.0, 1200.0),
                        "P2": _u_(rng, 1e3, 1e7), "V2": _u_(rng, 1e-5, 1.0), "T2": rng.uniform(150.0, 1200.0)}

_proc("combined_gas_final_volume", name="Combined gas law: final volume", topic="gases / gas laws",
      level="intermediate", purpose="The volume of a fixed amount of gas after its pressure and temperature change.",
      inputs=_CG() + [_inp("P2", "pressure", "final pressure", labels=L_P2, symbols=["P2", "P_2"], role="final"),
                      _inp("T2", "temperature", "final temperature", labels=L_T2, symbols=["T2", "T_2"], role="final")],
      outputs=[_out("V2", "volume", "final volume")],
      primary="V2", run=combined_v2,
      verify=lambda i, o: _rel(o["V2"] * i["P2"] * i["T1"], i["P1"] * i["V1"] * i["T2"]),
      reference=lambda P1, V1, T1, P2, T2: {"V2": V1 / _ref_vm(T1, P1) * _ref_vm(T2, P2)},
      wrong=lambda P1, V1, T1, P2, T2: {"V2": P1 * V1 * T1 / (T2 * P2)},
      rand=lambda rng: {k: v for k, v in _cg_rand(rng).items() if k != "V2"},
      kw=[ASK + r"[^.?]*\b(?:volume)\b"], nokw=_GASLAW_NO,
      need=_has("pressure", "pressure", "volume", "temperature", "temperature", exactly=True),
      latex=r"V_2 = \frac{P_1 V_1 T_2}{T_1 P_2}", method="P1V1/T1 = P2V2/T2 solved for V2",
      conditions=["all values > 0 (temperatures in kelvin)", "fixed amount of gas"],
      templates=["A gas occupies 4.00 L at 1.00 atm and 300 K. What is its volume at 2.00 atm and 400 K?",
                 "P1 = 1.0 atm, V1 = 2.0 L, T1 = 273 K, P2 = 0.5 atm, T2 = 546 K. Find the final volume."],
      examples=[{"P1": ATM, "V1": 0.004, "T1": 300.0, "P2": 2 * ATM, "T2": 400.0}],
      questions=["A gas occupies 4.00 L at 1.00 atm and 300 K. What is its volume at 2.00 atm and 400 K?",
                 "P1 = 1.0 atm, V1 = 2.0 L, T1 = 273 K, P2 = 0.5 atm, T2 = 546 K. Find the final volume."])

_proc("combined_gas_final_pressure", name="Combined gas law: final pressure", topic="gases / gas laws",
      level="intermediate", purpose="The pressure of a fixed amount of gas after its volume and temperature change.",
      inputs=_CG() + [_inp("V2", "volume", "final volume", labels=L_V2, symbols=["V2", "V_2"], role="final"),
                      _inp("T2", "temperature", "final temperature", labels=L_T2, symbols=["T2", "T_2"], role="final")],
      outputs=[_out("P2", "pressure", "final pressure")],
      primary="P2", run=combined_p2,
      verify=lambda i, o: _rel(o["P2"] * i["V2"] * i["T1"], i["P1"] * i["V1"] * i["T2"]),
      reference=lambda P1, V1, T1, V2, T2: {"P2": __import__("scipy.optimize").optimize.brentq(
          lambda P: V1 / _ref_vm(T1, P1) * _ref_vm(T2, P) - V2, 1e-3, 1e13, xtol=1e-12, rtol=1e-15)},
      wrong=lambda P1, V1, T1, V2, T2: {"P2": P1 * V2 * T2 / (T1 * V1)},
      rand=lambda rng: {k: v for k, v in _cg_rand(rng).items() if k != "P2"},
      kw=[ASK + r"[^.?]*\b(?:pressure)\b"], nokw=_GASLAW_NO,
      need=_has("pressure", "volume", "volume", "temperature", "temperature", exactly=True),
      latex=r"P_2 = \frac{P_1 V_1 T_2}{T_1 V_2}", method="P1V1/T1 = P2V2/T2 solved for P2",
      conditions=["all values > 0 (temperatures in kelvin)", "fixed amount of gas"],
      templates=["A gas at 1.00 atm and 300 K occupies 3.00 L. Find the final pressure if V2 = 1.50 L and T2 = 450 K."],
      examples=[{"P1": ATM, "V1": 0.003, "T1": 300.0, "V2": 0.0015, "T2": 450.0}],
      questions=["A gas at 1.00 atm and 300 K occupies 3.00 L. Find the final pressure if V2 = 1.50 L and T2 = 450 K.",
                 "P1 = 100 kPa, V1 = 10 L, T1 = 300 K, V2 = 5 L, T2 = 330 K. What is the final pressure?"])

_proc("combined_gas_final_temperature", name="Combined gas law: final temperature", topic="gases / gas laws",
      level="intermediate", purpose="The temperature of a fixed amount of gas after its pressure and volume change.",
      inputs=_CG() + [_inp("P2", "pressure", "final pressure", labels=L_P2, symbols=["P2", "P_2"], role="final"),
                      _inp("V2", "volume", "final volume", labels=L_V2, symbols=["V2", "V_2"], role="final")],
      outputs=[_out("T2", "temperature", "final temperature")],
      primary="T2", run=combined_t2,
      verify=lambda i, o: _rel(o["T2"] * i["P1"] * i["V1"], i["P2"] * i["V2"] * i["T1"]),
      reference=lambda P1, V1, T1, P2, V2: {"T2": __import__("scipy.optimize").optimize.brentq(
          lambda T: V1 / _ref_vm(T1, P1) * _ref_vm(T, P2) - V2, 1e-6, 1e12, xtol=1e-14, rtol=1e-15)},
      wrong=lambda P1, V1, T1, P2, V2: {"T2": P1 * V1 * T1 / (P2 * V2)},
      rand=lambda rng: {k: v for k, v in _cg_rand(rng).items() if k != "T2"},
      kw=[ASK + r"[^.?]*\b(?:temperature)\b"], nokw=_GASLAW_NO,
      need=_has("pressure", "pressure", "volume", "volume", "temperature", exactly=True),
      latex=r"T_2 = \frac{P_2 V_2 T_1}{P_1 V_1}", method="P1V1/T1 = P2V2/T2 solved for T2",
      conditions=["all values > 0 (temperatures in kelvin)", "fixed amount of gas"],
      templates=["P1 = 1.00 atm, V1 = 2.00 L, T1 = 300 K, P2 = 2.00 atm, V2 = 1.50 L. Find the final temperature."],
      examples=[{"P1": ATM, "V1": 0.002, "T1": 300.0, "P2": 2 * ATM, "V2": 0.0015}],
      questions=["P1 = 1.00 atm, V1 = 2.00 L, T1 = 300 K, P2 = 2.00 atm, V2 = 1.50 L. Find the final temperature.",
                 "A gas at 100 kPa, 5.0 L and 290 K is changed to 150 kPa and 4.0 L. What is the final temperature?"])


def partial_pressure(x, P_total):
    _pos(P_total=P_total)
    if not 0 < x <= 1:
        raise ValueError("a mole fraction lies between 0 and 1")
    return {"p": x * P_total}


_proc("partial_pressure", name="Partial pressure (Dalton's law)", topic="gases / gas mixtures", level="beginner",
      purpose="The partial pressure of a gas in a mixture from its mole fraction and the total pressure.",
      inputs=[_inp("x", "number", "mole fraction of the gas", labels=[r"\bmole fraction(?: of \w+)?"], symbols=["x", "χ"]),
              _inp("P_total", "pressure", "total pressure")],
      outputs=[_out("p", "pressure", "partial pressure")],
      primary="p", run=partial_pressure, verify=lambda i, o: _rel(o["p"], i["x"] * i["P_total"]),
      reference=lambda x, P_total: {"p": float(__import__("numpy").multiply(x, P_total))},
      wrong=lambda x, P_total: {"p": P_total / x},
      rand=lambda rng: {"x": rng.uniform(0.01, 0.99), "P_total": _u_(rng, 1e3, 1e7)},
      kw=[r"\bpartial pressure\b", r"\bmole fraction\b"],
      need=_has("pressure", exactly=True),
      latex=r"p_i = x_i P_{\text{total}}", method="p = x P_total", conditions=["0 < x <= 1", "P_total > 0"],
      templates=["The mole fraction of O2 in air is 0.21. What is its partial pressure at a total pressure of 1.00 atm?"],
      examples=[{"x": 0.21, "P_total": ATM}],
      questions=["The mole fraction of O2 in air is 0.21. What is its partial pressure at a total pressure of 1.00 atm?",
                 "A gas has mole fraction 1.5 in a mixture at 2.0 atm. What is its partial pressure?"])


def dalton_mixture(moles, P_total):
    _pos(P_total=P_total)
    if any(n <= 0 for n in moles):
        raise ValueError("every amount must be positive")
    tot = math.fsum(moles)
    x = [n / tot for n in moles]
    return {"partial_pressures": [xi * P_total for xi in x], "mole_fractions": x}


def _dalton_text(q, spans):
    pairs = list(re.finditer(r"(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)\s*(mol|mmol|moles)\s+(?:of\s+)?(" + _FTOKEN + r")", q))
    if len(pairs) < 2:
        return []
    f = {"mol": 1.0, "moles": 1.0, "mmol": 1e-3}
    vals = ", ".join(repr(float(m.group(1)) * f[m.group(2)]) for m in pairs)
    return [("moles", vals, (pairs[0].start(), pairs[-1].end()))]


_proc("dalton_mixture", name="Partial pressures in a gas mixture", topic="gases / gas mixtures", level="intermediate",
      purpose="Mole fractions and partial pressures of every gas in a mixture from the amounts and the total pressure.",
      inputs=[_inp("moles", "number", "amount of each gas (mol), e.g. '2.0, 1.0, 0.5'", type="vector", min_length=2,
                   max_length=20),
              _inp("P_total", "pressure", "total pressure")],
      outputs=[_out("partial_pressures", "pressure", "partial pressure of each gas, in the order given"),
               _out("mole_fractions", "number", "mole fraction of each gas")],
      primary="partial_pressures", run=dalton_mixture,
      verify=lambda i, o: max([_rel(p, n / math.fsum(i["moles"]) * i["P_total"]) for p, n in
                               zip(o["partial_pressures"], i["moles"])] + [0.0 if len(o["partial_pressures"]) ==
                                                                           len(i["moles"]) else 1.0]),
      reference=lambda moles, P_total: {"partial_pressures": [float(v) for v in __import__("numpy").asarray(moles)
                                                              / __import__("numpy").sum(moles) * P_total]},
      wrong=lambda moles, P_total: {"partial_pressures": [P_total / len(moles)] * len(moles)},
      rand=lambda rng: {"moles": [_u_(rng, 0.01, 10.0) for _ in range(rng.randint(2, 5))], "P_total": _u_(rng, 1e3, 1e7)},
      kw=[r"\bpartial pressures?\b"], nokw=[r"\bmole fraction\b"], text=_texts(_dalton_text),
      need=_has("pressure", exactly=True),
      latex=r"x_i = \frac{n_i}{\sum_j n_j},\quad p_i = x_i P", method="mole fractions from the amounts, times the total pressure",
      conditions=["every amount > 0", "P_total > 0"],
      templates=["A mixture of 2.0 mol N2, 1.0 mol O2 and 0.50 mol Ar has a total pressure of 3.5 atm. Find the partial pressures."],
      examples=[{"moles": "2.0, 1.0, 0.5", "P_total": 3.5 * ATM}],
      questions=["A mixture of 2.0 mol N2, 1.0 mol O2 and 0.50 mol Ar has a total pressure of 3.5 atm. Find the partial pressures.",
                 "A mixture of 1.0 mol He and 3.0 mol Ne is at 200 kPa. What are the partial pressures?"])


def graham_ratio(formula_1, formula_2):
    return {"ratio": math.sqrt(molar_mass_g(formula_2) / molar_mass_g(formula_1))}


def _graham_text(q, spans):
    m = re.search(r"(?:does|do|will)\s+(" + _FTOKEN + r")\s+(?:effuse|diffuse)[^.?]*?\bthan\s+(" + _FTOKEN + r")", q) or \
        re.search(r"(?:rates? of (?:effusion|diffusion) of|effusion rates? of|diffusion rates? of)\s+(" + _FTOKEN +
                  r")\s+(?:to|and|relative to|compared (?:to|with))\s+(?:that of\s+)?(" + _FTOKEN + r")", q)
    if not m:
        return []
    a, b = m.group(1).rstrip(".,?"), m.group(2).rstrip(".,?")
    return [("formula_1", a, (m.start(1), m.start(1) + len(a))), ("formula_2", b, (m.start(2), m.start(2) + len(b)))]


_proc("graham_rate_ratio", name="Graham's law: ratio of effusion rates", topic="gases / kinetic theory",
      level="intermediate", purpose="How many times faster one gas effuses (or diffuses) than another.",
      inputs=[_inp("formula_1", "number", "first gas (numerator)", type="text", min_length=1, max_length=40),
              _inp("formula_2", "number", "second gas", type="text", min_length=1, max_length=40)],
      outputs=[_out("ratio", "number", "rate of gas 1 / rate of gas 2")],
      primary="ratio", run=graham_ratio,
      verify=lambda i, o: _rel(o["ratio"] ** 2 * _mm_rd(i["formula_1"]), _mm_rd(i["formula_2"])),
      reference=lambda formula_1, formula_2: {"ratio": float(__import__("numpy").sqrt(_ref_mw(formula_2) / _ref_mw(formula_1)))},
      wrong=lambda formula_1, formula_2: {"ratio": molar_mass_g(formula_2) / molar_mass_g(formula_1)},
      rand=lambda rng: dict(zip(("formula_1", "formula_2"), rng.sample(_GAS_FORMULAS, 2))),
      kw=[r"\b(?:effus|diffus|graham)"], nokw=[r"\bslower\b", r"\bunknown\b"], text=_texts(_graham_text),
      need=lambda t, qs: not qs,
      latex=r"\frac{r_1}{r_2} = \sqrt{\frac{M_2}{M_1}}", method="square root of the inverse ratio of molar masses",
      conditions=["ideal gases at the same temperature and pressure"],
      templates=["How many times faster does H2 effuse than O2?",
                 "What is the ratio of the rates of effusion of He to CH4?"],
      examples=[{"formula_1": "H2", "formula_2": "O2"}, {"formula_1": "He", "formula_2": "CH4"}],
      questions=["How many times faster does H2 effuse than O2?",
                 "What is the ratio of the rates of effusion of He to CH4?",
                 "How many times slower does O2 effuse than H2?"])


def graham_unknown(ratio, formula_known):
    _pos(ratio=ratio)
    M = molar_mass_g(formula_known) / ratio ** 2
    return {"M": M / 1000.0, "M_g": M}


def _graham_unknown_text(q, spans):
    m = re.search(r"(\d+(?:\.\d+)?)\s+times\s+(?:as fast as|faster than)\s+(" + _FTOKEN + r")", q)
    if not m:
        return []
    f = m.group(2).rstrip(".,?")
    try:
        formula_counts(f)
    except ValueError:
        return []
    return [("ratio", float(m.group(1)), (m.start(1), m.end(1))), ("formula_known", f, (m.start(2), m.start(2) + len(f)))]


_proc("graham_molar_mass", name="Graham's law: molar mass of an unknown gas", topic="gases / kinetic theory",
      level="intermediate", purpose="The molar mass of a gas from how fast it effuses relative to a known gas.",
      inputs=[_inp("ratio", "number", "rate of the unknown gas / rate of the known gas"),
              _inp("formula_known", "number", "formula of the known gas", type="text", min_length=1, max_length=40)],
      outputs=[_out("M", "molar_mass", "molar mass of the unknown gas"), _out("M_g", "number", "molar mass in g/mol")],
      primary="M", run=graham_unknown,
      verify=lambda i, o: _rel(o["M"] * 1000.0 * i["ratio"] ** 2, _mm_rd(i["formula_known"])),
      reference=lambda ratio, formula_known: {"M": __import__("scipy.optimize").optimize.brentq(
          lambda M: (_ref_mw(formula_known) / 1000.0 / M) ** 0.5 - ratio, 1e-9, 1e3, xtol=1e-16, rtol=1e-15)},
      wrong=lambda ratio, formula_known: {"M": molar_mass_g(formula_known) / ratio / 1000.0},
      rand=lambda rng: {"ratio": _u_(rng, 0.1, 5.0), "formula_known": rng.choice(_GAS_FORMULAS)},
      kw=[r"\b(?:effus|diffus|graham)", r"\bunknown\b|\bmolar mass\b"], nokw=[r"\bslower\b"],
      text=_texts(_graham_unknown_text), need=lambda t, qs: not qs,
      latex=r"M_x = \frac{M_{\text{known}}}{(r_x / r_{\text{known}})^2}", method="M_x = M_known / ratio^2",
      conditions=["ratio > 0", "same temperature and pressure"],
      templates=["An unknown gas effuses 1.50 times as fast as O2. What is its molar mass?"],
      examples=[{"ratio": 1.5, "formula_known": "O2"}],
      questions=["An unknown gas effuses 1.50 times as fast as O2. What is its molar mass?",
                 "An unknown gas diffuses 0.50 times as fast as CH4. What is the molar mass of the unknown gas?"])


def vdw_pressure(n, V, T, a, b):
    _pos(n=n, V=V, T=T, a=a, b=b)
    if V <= n * b:
        raise ValueError("the volume must exceed n*b (the excluded volume)")
    P = n * R * T / (V - n * b) - a * n * n / (V * V)
    return {"P": P, "P_ideal": n * R * T / V, "P_atm": P / ATM}


def _ref_vdw(n, V, T, a, b):
    from thermo.eos import VDW
    Tc, Pc = 8 * a / (27 * R * b), a / (27 * b * b)
    return {"P": VDW(Tc=Tc, Pc=Pc, T=T, V=V / n).P}


_proc("van_der_waals_pressure", name="Van der Waals equation: pressure", topic="gases / real gases", level="advanced",
      purpose="The pressure of a real gas from the van der Waals equation (and the ideal-gas pressure for comparison).",
      inputs=[_inp("n", "amount", "amount of gas"), _inp("V", "volume", "volume"), _inp("T", "temperature", "temperature"),
              _inp("a", "van_der_waals_a", "van der Waals a", labels=[r"\ba", r"\bconstant a"], symbols=["a"]),
              _inp("b", "molar_volume", "van der Waals b", labels=[r"\bb", r"\bconstant b"], symbols=["b"])],
      outputs=[_out("P", "pressure", "van der Waals pressure"), _out("P_ideal", "pressure", "ideal-gas pressure"),
               _out("P_atm", "number", "van der Waals pressure in atm")],
      primary="P", run=vdw_pressure,
      verify=lambda i, o: _rel((o["P"] + i["a"] * i["n"] ** 2 / i["V"] ** 2) * (i["V"] - i["n"] * i["b"]),
                               i["n"] * R * i["T"]),
      reference=_ref_vdw,
      wrong=lambda n, V, T, a, b: {"P": n * R * T / (V - b) - a * n * n / (V * V)},
      rand=lambda rng: (lambda n, b: {"n": n, "b": b, "V": n * b * rng.uniform(3.0, 200.0), "T": rng.uniform(150.0, 900.0),
                                      "a": _u_(rng, 0.003, 2.0)})(rng.uniform(1.5, 8.0), _u_(rng, 1.7e-5, 1.5e-4)),
      kw=[r"\bvan der waals\b", ASK + r"[^.?]*\bpressure\b"],
      need=_has("amount", "volume", "temperature", "van_der_waals_a", "molar_volume", exactly=True),
      latex=r"P = \frac{nRT}{V - nb} - \frac{a n^2}{V^2}", method="the van der Waals equation solved for P",
      conditions=["n, V, T, a, b > 0", "V > n b"],
      templates=["Using the van der Waals equation, find the pressure of 2.00 mol of CO2 in 5.00 L at 300 K "
                 "(a = 3.640 L^2·atm/mol^2, b = 0.04267 L/mol)."],
      examples=[{"n": 2.0, "V": 0.005, "T": 300.0, "a": 3.640 * 0.101325, "b": 0.04267e-3}],
      questions=["Using the van der Waals equation, find the pressure of 2.00 mol of CO2 in 5.00 L at 300 K "
                 "(a = 3.640 L^2·atm/mol^2, b = 0.04267 L/mol).",
                 "What is the van der Waals pressure of 1.00 mol of N2 in 1.00 L at 273 K with a = 1.370 L^2 bar/mol^2 "
                 "and b = 0.0387 L/mol?",
                 "Using the van der Waals equation, find the pressure of 2.00 mol of CO2 in 5.00 L at 300 K."])


def rms_speed(formula, T):
    _pos(T=T)
    return {"u_rms": math.sqrt(3 * R * T / (molar_mass_g(formula) / 1000.0))}


def mean_speed(formula, T):
    _pos(T=T)
    return {"u_mean": math.sqrt(8 * R * T / (math.pi * molar_mass_g(formula) / 1000.0))}


def _maxwell_scale(formula, T):
    import scipy.constants as sc
    return math.sqrt(sc.k * T / (_ref_mw(formula) / 1000.0 / sc.N_A))


_proc("rms_speed", name="Root-mean-square speed of gas molecules", topic="gases / kinetic theory", level="intermediate",
      purpose="The root-mean-square speed of the molecules of an ideal gas at a temperature.",
      inputs=[_inp("formula", "number", "formula of the gas", type="text", min_length=1, max_length=40),
              _inp("T", "temperature", "temperature")],
      outputs=[_out("u_rms", "speed", "root-mean-square speed")],
      primary="u_rms", run=rms_speed,
      verify=lambda i, o: _rel(o["u_rms"] ** 2 * _mm_rd(i["formula"]) / 1000.0, 3 * R * i["T"]),
      reference=lambda formula, T: {"u_rms": math.sqrt(__import__("scipy.stats").stats.maxwell.moment(
          2, scale=_maxwell_scale(formula, T)))},
      wrong=lambda formula, T: {"u_rms": math.sqrt(3 * R * T / molar_mass_g(formula))},
      rand=lambda rng: {"formula": rng.choice(_GAS_FORMULAS), "T": rng.uniform(50.0, 3000.0)},
      kw=[r"\b(?:rms|root[- ]mean[- ]square)\s+(?:speed|velocity)"],
      text=_texts(_tf("formula", r"(?:molecules of|of|for)")), need=_has("temperature", exactly=True),
      latex=r"u_{\text{rms}} = \sqrt{\frac{3RT}{M}}", method="u_rms = sqrt(3RT/M), M in kg/mol",
      conditions=["T > 0 K", "ideal gas"],
      templates=["What is the rms speed of N2 molecules at 300 K?",
                 "Calculate the root-mean-square speed of O2 at 25 °C."],
      examples=[{"formula": "N2", "T": 300.0}, {"formula": "O2", "T": 298.15}],
      questions=["What is the rms speed of N2 molecules at 300 K?",
                 "Calculate the root-mean-square speed of O2 at 25 °C."])

_proc("mean_speed", name="Mean speed of gas molecules", topic="gases / kinetic theory", level="advanced",
      purpose="The mean (average) speed of the molecules of an ideal gas (Maxwell-Boltzmann distribution).",
      inputs=[_inp("formula", "number", "formula of the gas", type="text", min_length=1, max_length=40),
              _inp("T", "temperature", "temperature")],
      outputs=[_out("u_mean", "speed", "mean speed")],
      primary="u_mean", run=mean_speed,
      verify=lambda i, o: _rel(o["u_mean"] ** 2 * math.pi * _mm_rd(i["formula"]) / 1000.0, 8 * R * i["T"]),
      reference=lambda formula, T: {"u_mean": float(__import__("scipy.stats").stats.maxwell.mean(
          scale=_maxwell_scale(formula, T)))},
      wrong=lambda formula, T: {"u_mean": math.sqrt(3 * R * T / (molar_mass_g(formula) / 1000.0))},
      rand=lambda rng: {"formula": rng.choice(_GAS_FORMULAS), "T": rng.uniform(50.0, 3000.0)},
      kw=[r"\b(?:mean|average)\s+(?:molecular\s+)?(?:speed|velocity)"], nokw=[r"\b(?:rms|root|square)\b"],
      text=_texts(_tf("formula", r"(?:molecules of|of|for)")), need=_has("temperature", exactly=True),
      latex=r"\bar{u} = \sqrt{\frac{8RT}{\pi M}}", method="mean of the Maxwell-Boltzmann speed distribution",
      conditions=["T > 0 K", "ideal gas"],
      templates=["What is the mean speed of CO2 molecules at 300 K?"],
      examples=[{"formula": "CO2", "T": 300.0}],
      questions=["What is the mean speed of CO2 molecules at 300 K?", "Find the average speed of He atoms at 1000 K."])

# ====================================================================================================================
# SOLUTIONS
# ====================================================================================================================
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def molarity_from_mass(formula, m, V):
    _pos(m=m, V=V)
    c = m / (molar_mass_g(formula) / 1000.0) / V
    return {"c": c, "c_molL": c / 1000.0}


_proc("molarity_from_mass", name="Molarity from a dissolved mass", topic="solutions / concentration", level="beginner",
      purpose="The molar concentration of a solution made by dissolving a mass of a compound to a final volume.",
      inputs=[_inp("formula", "number", "formula of the solute", type="text", min_length=1, max_length=80),
              _inp("m", "mass", "mass of solute"), _inp("V", "volume", "volume of solution")],
      outputs=[_out("c", "molar_concentration", "molar concentration"), _out("c_molL", "number", "concentration in mol/L")],
      primary="c", run=molarity_from_mass,
      verify=lambda i, o: _rel(o["c"] * i["V"] * _mm_rd(i["formula"]) / 1000.0, i["m"]),
      reference=lambda formula, m, V: {"c": m * 1000.0 / _ref_mw(formula) / V},
      wrong=lambda formula, m, V: {"c": m * molar_mass_g(formula) / V},
      rand=lambda rng: {"formula": _rand_formula(rng), "m": _u_(rng, 1e-4, 0.5), "V": _u_(rng, 1e-4, 2.0)},
      kw=[r"\b(?:molarity|molar concentration|concentration)\b"], nokw=[r"dilut", r"\bmolality\b", r"\bph\b"],
      text=_texts(_tf("formula", r"(?:kg|g|mg|grams|gram)\s+of")), need=_has("mass", "volume", exactly=True),
      latex=r"c = \frac{m}{M V}", method="moles of solute (m / M) over the volume of solution",
      conditions=["m, V > 0"],
      templates=["What is the molarity of a solution made by dissolving 5.85 g of NaCl in 250 mL of solution?"],
      examples=[{"formula": "NaCl", "m": 0.00585, "V": 0.00025}, {"formula": "C6H12O6", "m": 0.09, "V": 0.5}],
      questions=["What is the molarity of a solution made by dissolving 5.85 g of NaCl in 250 mL of solution?",
                 "Find the concentration of a solution of 90.0 g of C6H12O6 in 0.500 L of solution."])


def solute_mass(formula, c, V):
    _pos(c=c, V=V)
    m = c * V * molar_mass_g(formula) / 1000.0
    return {"m": m, "m_g": m * 1000.0}


_proc("solute_mass_for_solution", name="Mass of solute for a solution", topic="solutions / concentration",
      level="beginner", purpose="The mass of a compound needed to prepare a volume of solution of a given molarity.",
      inputs=[_inp("formula", "number", "formula of the solute", type="text", min_length=1, max_length=80),
              _inp("c", "molar_concentration", "molar concentration"), _inp("V", "volume", "volume of solution")],
      outputs=[_out("m", "mass", "mass of solute"), _out("m_g", "number", "mass in grams")],
      primary="m", run=solute_mass,
      verify=lambda i, o: _rel(o["m"], i["c"] * i["V"] * _mm_rd(i["formula"]) / 1000.0),
      reference=lambda formula, c, V: {"m": c * V * _ref_mw(formula) / 1000.0},
      wrong=lambda formula, c, V: {"m": c * V / molar_mass_g(formula)},
      rand=lambda rng: {"formula": _rand_formula(rng), "c": _u_(rng, 1.0, 5000.0), "V": _u_(rng, 1e-4, 2.0)},
      kw=[r"\b(?:what mass|how many grams|mass of|how much)\b", r"\b(?:solution|prepare|make)\b"],
      nokw=[r"dilut", r"\bph\b", r"\bproduc|react"],
      text=_texts(_tf("formula", r"(?:mass of|grams of|much)")),
      need=_has("molar_concentration", "volume", exactly=True, without=("mass",)),
      latex=r"m = c V M", method="m = c V M", conditions=["c, V > 0"],
      templates=["What mass of NaCl is needed to prepare 250 mL of 0.100 M solution?"],
      examples=[{"formula": "NaCl", "c": 100.0, "V": 0.00025}],
      questions=["What mass of NaCl is needed to prepare 250 mL of 0.100 M solution?",
                 "How many grams of KMnO4 are needed to make 2.00 L of a 0.0200 M solution?"])

_C1 = [r"\bc1", r"\bm1", r"\binitial concentration", r"\bstock(?: solution)?(?: of| is)?", r"volume of", r"concentrated"]


def dilution_c2(c1, V1, V2):
    _pos(c1=c1, V1=V1, V2=V2)
    if V2 < V1:
        raise ValueError("a dilution needs a final volume at least the initial volume")
    return {"c2": c1 * V1 / V2, "c2_molL": c1 * V1 / V2 / 1000.0}


_proc("dilution_final_concentration", name="Dilution: final concentration", topic="solutions / dilution",
      level="beginner", purpose="The concentration after diluting a volume of solution to a larger volume (c1V1 = c2V2).",
      inputs=[_inp("c1", "molar_concentration", "initial concentration", symbols=["c1", "C1", "M1"]),
              _inp("V1", "volume", "initial volume", labels=L_V1, symbols=["V1"], role="initial"),
              _inp("V2", "volume", "final volume", labels=L_V2 + [r"diluted to", r"total volume", r"made up to"],
                   symbols=["V2"], role="final")],
      outputs=[_out("c2", "molar_concentration", "final concentration"), _out("c2_molL", "number", "in mol/L")],
      primary="c2", run=dilution_c2, verify=lambda i, o: _rel(o["c2"] * i["V2"], i["c1"] * i["V1"]),
      reference=lambda c1, V1, V2: {"c2": _brentq(lambda c: c * V2 - c1 * V1, 0.0, 2 * c1)},
      wrong=lambda c1, V1, V2: {"c2": c1 * V2 / V1},
      rand=lambda rng: (lambda V1: {"c1": _u_(rng, 1.0, 1e4), "V1": V1, "V2": V1 * _u_(rng, 1.01, 100.0)})(
          _u_(rng, 1e-6, 0.5)),
      kw=[r"dilut", r"(?:concentration|molarity)"],
      need=_has("molar_concentration", "volume", "volume", exactly=True),
      latex=r"c_2 = \frac{c_1 V_1}{V_2}", method="moles of solute are conserved: c1 V1 = c2 V2",
      conditions=["c1, V1, V2 > 0", "V2 >= V1"],
      templates=["25.0 mL of 6.0 M HCl is diluted to 500 mL. What is the new concentration?"],
      examples=[{"c1": 6000.0, "V1": 0.025, "V2": 0.5}],
      questions=["25.0 mL of 6.0 M HCl is diluted to 500 mL. What is the new concentration?",
                 "A 10.0 mL sample of 2.00 M NaOH is diluted to a total volume of 250 mL. Find the final molarity."])


def dilution_v1(c1, c2, V2):
    _pos(c1=c1, c2=c2, V2=V2)
    if c2 > c1:
        raise ValueError("the stock solution must be more concentrated than the diluted one")
    V1 = c2 * V2 / c1
    return {"V1": V1, "V1_mL": V1 * 1e6}


_proc("dilution_volume", name="Dilution: volume of stock needed", topic="solutions / dilution", level="beginner",
      purpose="The volume of a stock solution needed to prepare a volume of a more dilute solution.",
      inputs=[_inp("c1", "molar_concentration", "stock concentration", labels=_C1, symbols=["c1", "C1", "M1"]),
              _inp("c2", "molar_concentration", "target concentration", labels=[r"\bof", r"\bc2", r"\bm2", r"final concentration",
                                                                               r"target concentration"],
                   symbols=["c2", "C2", "M2"]),
              _inp("V2", "volume", "target volume", symbols=["V2"])],
      outputs=[_out("V1", "volume", "volume of stock solution"), _out("V1_mL", "number", "in mL")],
      primary="V1", run=dilution_v1, verify=lambda i, o: _rel(o["V1"] * i["c1"], i["c2"] * i["V2"]),
      reference=lambda c1, c2, V2: {"V1": _brentq(lambda v: c1 * v - c2 * V2, 0.0, 2 * V2)},
      wrong=lambda c1, c2, V2: {"V1": c1 * V2 / c2},
      rand=lambda rng: (lambda c1: {"c1": c1, "c2": c1 / _u_(rng, 1.01, 1000.0), "V2": _u_(rng, 1e-4, 5.0)})(
          _u_(rng, 10.0, 2e4)),
      kw=[r"\b(?:dilut|prepare|stock|make)", r"\b(?:what volume|how many (?:ml|millilit|lit)|volume of)"],
      nokw=[r"neutrali|titrat"],
      need=_has("molar_concentration", "molar_concentration", "volume", exactly=True),
      latex=r"V_1 = \frac{c_2 V_2}{c_1}", method="c1 V1 = c2 V2 solved for V1", conditions=["c1 >= c2 > 0", "V2 > 0"],
      templates=["What volume of 12.0 M HCl is needed to prepare 500 mL of 0.100 M HCl?"],
      examples=[{"c1": 12000.0, "c2": 100.0, "V2": 0.5}],
      questions=["What volume of 12.0 M HCl is needed to prepare 500 mL of 0.100 M HCl?",
                 "C1 = 2.0 M, C2 = 0.50 M, V2 = 100 mL. What volume of stock solution is needed?"])

_SOLVENT_LABELS = dict(labels=[r"\bin", r"dissolved in", r"mass of (?:the )?solvent", r"solvent mass"],
                       suffix_labels=[r"(?:of\s+)?(?:water|solvent|benzene|ethanol|cyclohexane|camphor|h2o)"])


def molality(formula, m_solute, m_solvent):
    _pos(m_solute=m_solute, m_solvent=m_solvent)
    return {"b": m_solute / (molar_mass_g(formula) / 1000.0) / m_solvent}


_proc("molality", name="Molality of a solution", topic="solutions / concentration", level="intermediate",
      purpose="The molality (moles of solute per kilogram of solvent) from the masses of solute and solvent.",
      inputs=[_inp("formula", "number", "formula of the solute", type="text", min_length=1, max_length=80),
              _inp("m_solute", "mass", "mass of solute"), _inp("m_solvent", "mass", "mass of solvent", **_SOLVENT_LABELS)],
      outputs=[_out("b", "molality", "molality")],
      primary="b", run=molality,
      verify=lambda i, o: _rel(o["b"] * i["m_solvent"] * _mm_rd(i["formula"]) / 1000.0, i["m_solute"]),
      reference=lambda formula, m_solute, m_solvent: {"b": m_solute * 1000.0 / _ref_mw(formula) / m_solvent},
      wrong=lambda formula, m_solute, m_solvent: {"b": m_solute * 1000.0 / molar_mass_g(formula) / (m_solute + m_solvent)},
      rand=lambda rng: {"formula": _rand_formula(rng), "m_solute": _u_(rng, 1e-4, 0.5), "m_solvent": _u_(rng, 0.01, 5.0)},
      kw=[r"\bmolality\b|\bmolal concentration"], nokw=[r"freezing|boiling"],
      text=_texts(_tf("formula", r"(?:kg|g|mg|grams|gram)\s+of")), need=_has("mass", "mass", exactly=True),
      latex=r"b = \frac{m_{\text{solute}} / M}{m_{\text{solvent}}}", method="moles of solute per kilogram of solvent",
      conditions=["masses > 0"],
      templates=["What is the molality of a solution of 10.0 g of NaCl dissolved in 500 g of water?"],
      examples=[{"formula": "NaCl", "m_solute": 0.01, "m_solvent": 0.5}],
      questions=["What is the molality of a solution of 10.0 g of NaCl dissolved in 500 g of water?",
                 "Find the molality when 45.0 g of C6H12O6 is dissolved in 1.00 kg of water."])


def _two_pairs(q, spans):
    pairs = _mass_pairs(q)
    if len(pairs) != 2 or not re.search(r"\b(?:in|and|with)\b", q[pairs[0][2][1]:pairs[1][2][0]]):
        return []
    (m1, f1, s1), (m2, f2, s2) = pairs
    return [("m_solute", m1, s1), ("formula_solute", f1, (s1[1], s1[1])),
            ("m_solvent", m2, s2), ("formula_solvent", f2, (s2[1], s2[1]))]


_TWO_IN = [_inp("formula_solute", "number", "formula of the solute", type="text", min_length=1, max_length=80),
           _inp("m_solute", "mass", "mass of solute"),
           _inp("formula_solvent", "number", "formula of the solvent", type="text", min_length=1, max_length=80),
           _inp("m_solvent", "mass", "mass of solvent")]


def mole_fraction(formula_solute, m_solute, formula_solvent, m_solvent):
    _pos(m_solute=m_solute, m_solvent=m_solvent)
    n1, n2 = m_solute / molar_mass_g(formula_solute), m_solvent / molar_mass_g(formula_solvent)
    return {"x_solute": n1 / (n1 + n2), "x_solvent": n2 / (n1 + n2)}


_proc("mole_fraction", name="Mole fraction in a solution", topic="solutions / concentration", level="intermediate",
      purpose="The mole fractions of solute and solvent from their masses.",
      inputs=_TWO_IN, outputs=[_out("x_solute", "number", "mole fraction of solute"),
                               _out("x_solvent", "number", "mole fraction of solvent")],
      primary="x_solute", run=mole_fraction,
      verify=lambda i, o: _rel(o["x_solute"] / (1 - o["x_solute"]),
                               i["m_solute"] / _mm_rd(i["formula_solute"]) / (i["m_solvent"] / _mm_rd(i["formula_solvent"]))),
      reference=lambda formula_solute, m_solute, formula_solvent, m_solvent: (
          lambda a, b: {"x_solute": a / (a + b), "x_solvent": b / (a + b)})(
          m_solute / _ref_mw(formula_solute), m_solvent / _ref_mw(formula_solvent)),
      wrong=lambda formula_solute, m_solute, formula_solvent, m_solvent: {
          "x_solute": m_solute / (m_solute + m_solvent), "x_solvent": m_solvent / (m_solute + m_solvent)},
      rand=lambda rng: {"formula_solute": _rand_formula(rng), "m_solute": _u_(rng, 1e-4, 1.0),
                        "formula_solvent": rng.choice(["H2O", "C2H5OH", "C6H6", "CH3OH", "C6H12", "CCl4"]),
                        "m_solvent": _u_(rng, 0.01, 5.0)},
      kw=[r"\bmole fractions?\b"], nokw=[r"vapou?r pressure|partial"], text=_texts(_two_pairs), need=lambda t, qs: not qs,
      latex=r"x_{\text{solute}} = \frac{n_{\text{solute}}}{n_{\text{solute}} + n_{\text{solvent}}}",
      method="moles of each from its mass and molar mass", conditions=["masses > 0"],
      templates=["What is the mole fraction of solute in a solution of 20.0 g of C6H12O6 dissolved in 100 g of H2O?"],
      examples=[{"formula_solute": "C6H12O6", "m_solute": 0.02, "formula_solvent": "H2O", "m_solvent": 0.1}],
      questions=["What is the mole fraction of solute in a solution of 20.0 g of C6H12O6 dissolved in 100 g of H2O?",
                 "Find the mole fractions for 46.0 g of C2H5OH in 100 g of H2O."])


def raoult(formula_solute, m_solute, formula_solvent, m_solvent, P0):
    _pos(P0=P0)
    x = mole_fraction(formula_solute, m_solute, formula_solvent, m_solvent)["x_solvent"]
    return {"P": x * P0, "dP": (1 - x) * P0}


_proc("raoult_vapor_pressure", name="Vapor pressure of a solution (Raoult's law)", topic="solutions / colligative properties",
      level="intermediate",
      purpose="The vapor pressure of a solvent above a solution of a nonvolatile solute, from the masses of both.",
      inputs=_TWO_IN + [_inp("P0", "pressure", "vapor pressure of the pure solvent")],
      outputs=[_out("P", "pressure", "vapor pressure of the solution"), _out("dP", "pressure", "vapor pressure lowering")],
      primary="P", run=raoult,
      verify=lambda i, o: _rel(o["P"], i["P0"] * (i["m_solvent"] / _mm_rd(i["formula_solvent"])) /
                               (i["m_solvent"] / _mm_rd(i["formula_solvent"]) + i["m_solute"] / _mm_rd(i["formula_solute"]))),
      reference=lambda formula_solute, m_solute, formula_solvent, m_solvent, P0: (
          lambda a, b: {"P": P0 * b / (a + b)})(m_solute / _ref_mw(formula_solute), m_solvent / _ref_mw(formula_solvent)),
      wrong=lambda formula_solute, m_solute, formula_solvent, m_solvent, P0: {
          "P": P0 * mole_fraction(formula_solute, m_solute, formula_solvent, m_solvent)["x_solute"]},
      rand=lambda rng: {"formula_solute": _rand_formula(rng), "m_solute": _u_(rng, 1e-3, 1.0),
                        "formula_solvent": rng.choice(["H2O", "C2H5OH", "C6H6", "CH3OH"]), "m_solvent": _u_(rng, 0.01, 5.0),
                        "P0": _u_(rng, 500.0, 1e5)},
      kw=[r"\bvapou?r pressure\b"], text=_texts(_two_pairs), need=_has("pressure", exactly=True),
      latex=r"P = x_{\text{solvent}} P^\circ", method="Raoult's law with the solvent mole fraction from the masses",
      conditions=["nonvolatile, non-dissociating solute", "ideal solution"],
      templates=["What is the vapor pressure of a solution of 50.0 g of C12H22O11 in 500 g of H2O if the vapor pressure "
                 "of pure water is 23.8 torr?"],
      examples=[{"formula_solute": "C12H22O11", "m_solute": 0.05, "formula_solvent": "H2O", "m_solvent": 0.5,
                 "P0": 23.8 * ATM / 760}],
      questions=["What is the vapor pressure of a solution of 50.0 g of C12H22O11 in 500 g of H2O if the vapor pressure "
                 "of pure water is 23.8 torr?"])

_I_IN = lambda: _inp("i", "number", "van 't Hoff factor", labels=[r"van\W?t\W? ?hoff factor(?: of| is| i)?", r"\bi"],
                     symbols=["i"], label_only=True)


def _nonelectrolyte(got, q):
    if "i" not in got and re.search(r"non-?electrolyte", q, re.I):
        got["i"] = 1.0
    return got


def _colligative(i, K, b):
    _pos(i=i, K=K, b=b)
    return {"dT": i * K * b}


_DT_LABELS = [r"lower(?:s|ed|ing)? the freezing point by", r"freezing[- ]point depression(?: of| is)?", r"depressed by",
              r"raise(?:s|d)? the boiling point by", r"boiling[- ]point elevation(?: of| is)?", r"elevated by",
              r"\bδt[fb]?", r"decrease(?:s|d)? by", r"increase(?:s|d)? by"]

for _key, _nm, _sym, _word, _kname in (
        ("freezing_point_depression", "Freezing-point depression", ["Kf", "K_f"], "freezing", "cryoscopic constant"),
        ("boiling_point_elevation", "Boiling-point elevation", ["Kb", "K_b"], "boiling", "ebullioscopic constant")):
    _proc(_key, name=_nm, topic="solutions / colligative properties", level="intermediate",
          purpose=f"The change of the {_word} point of a solvent caused by a solute (dT = i K b).",
          inputs=[_I_IN(), _inp("K", "colligative_constant", _kname, symbols=_sym),
                  _inp("b", "molality", "molality of the solute")],
          outputs=[_out("dT", "temperature_difference", f"{_word}-point change")],
          primary="dT", run=_colligative, verify=lambda i, o: _rel(o["dT"], i["i"] * i["K"] * i["b"]),
          reference=lambda i, K, b: {"dT": _brentq(lambda d: d / (K * b) - i, 0.0, 10 * i * K * b)},
          wrong=lambda i, K, b: {"dT": K * b / i},
          rand=lambda rng: {"i": rng.uniform(1.5, 3.0), "K": _u_(rng, 0.5, 40.0), "b": _u_(rng, 1e-3, 5.0)},
          kw=[rf"\b{_word} point\b"], nokw=[r"\bmolar mass\b"], post=_nonelectrolyte,
          need=_has("colligative_constant", "molality", exactly=True),
          latex=r"\Delta T = i K b", method="dT = i K b", conditions=["i, K, b > 0", "dilute solution"],
          templates=[f"What is the {_word} point change of a 0.50 mol/kg aqueous solution of a nonelectrolyte "
                     f"({_sym[0]} = {1.86 if _word == 'freezing' else 0.512} K·kg/mol)?",
                     f"Find the {_word} point change for b = 0.20 mol/kg, i = 2, {_sym[0]} = "
                     f"{1.86 if _word == 'freezing' else 0.512} °C/m."],
          examples=[{"i": 1.0, "K": 1.86 if _word == "freezing" else 0.512, "b": 0.5},
                    {"i": 2.0, "K": 1.86 if _word == "freezing" else 0.512, "b": 0.2}],
          questions=[f"What is the {_word} point change of a 0.50 mol/kg aqueous solution of a nonelectrolyte "
                     f"({_sym[0]} = {1.86 if _word == 'freezing' else 0.512} K·kg/mol)?",
                     f"Find the {_word} point change for b = 0.20 mol/kg, i = 2, {_sym[0]} = "
                     f"{1.86 if _word == 'freezing' else 0.512} °C/m.",
                     f"What is the {_word} point change of a 0.50 mol/kg solution with "
                     f"{_sym[0]} = {1.86 if _word == 'freezing' else 0.512} K·kg/mol?"])


def molar_mass_from_freezing(m_solute, m_solvent, K, dT, i):
    _pos(m_solute=m_solute, m_solvent=m_solvent, K=K, dT=dT, i=i)
    M = i * K * m_solute / (dT * m_solvent)
    return {"M": M, "M_g": M * 1000.0}


_proc("molar_mass_from_freezing_point", name="Molar mass from freezing-point depression",
      topic="solutions / colligative properties", level="advanced",
      purpose="The molar mass of a solute from the freezing-point depression it causes.",
      inputs=[_inp("m_solute", "mass", "mass of solute"), _inp("m_solvent", "mass", "mass of solvent", **_SOLVENT_LABELS),
              _inp("K", "colligative_constant", "cryoscopic constant Kf", symbols=["Kf", "K_f"]),
              _inp("dT", "temperature_difference", "freezing-point depression", labels=_DT_LABELS,
                   symbols=["ΔTf", "ΔT"], label_only=True), _I_IN()],
      outputs=[_out("M", "molar_mass", "molar mass of solute"), _out("M_g", "number", "in g/mol")],
      primary="M", run=molar_mass_from_freezing,
      verify=lambda i, o: _rel(o["M"] * i["dT"] * i["m_solvent"], i["i"] * i["K"] * i["m_solute"]),
      reference=lambda m_solute, m_solvent, K, dT, i: {"M": _brentq(
          lambda M: i * K * (m_solute / M) / m_solvent - dT, 1e-6, 1e6)},
      wrong=lambda m_solute, m_solvent, K, dT, i: {"M": i * K * m_solute * 1000.0 / (dT * m_solvent)},
      rand=lambda rng: {"m_solute": _u_(rng, 1e-3, 0.1), "m_solvent": _u_(rng, 0.05, 2.0), "K": _u_(rng, 1.0, 40.0),
                        "dT": _u_(rng, 0.05, 5.0), "i": rng.choice([1.0, 1.0, 2.0, 3.0])},
      kw=[r"\bmolar mass|molecular weight", r"\bfreezing"], post=_nonelectrolyte,
      need=_has("mass", "mass", "colligative_constant", "temperature", exactly=True),
      latex=r"M = \frac{i K_f m_{\text{solute}}}{\Delta T_f\, m_{\text{solvent}}}", method="b = dT/(i Kf); M = m_solute / (b m_solvent)",
      conditions=["all inputs > 0", "dilute solution"],
      templates=["Dissolving 2.00 g of a nonelectrolyte in 100 g of water lowers the freezing point by 0.372 °C "
                 "(Kf = 1.86 K·kg/mol). What is the molar mass of the solute?"],
      examples=[{"m_solute": 0.002, "m_solvent": 0.1, "K": 1.86, "dT": 0.372, "i": 1.0}],
      questions=["Dissolving 2.00 g of a nonelectrolyte in 100 g of water lowers the freezing point by 0.372 °C "
                 "(Kf = 1.86 K·kg/mol). What is the molar mass of the solute?",
                 "A solution of 1.00 g of a nonelectrolyte in 50.0 g of water freezes at -0.200 °C (Kf = 1.86 K·kg/mol). "
                 "What is the molar mass of the solute?"])


def osmotic_pressure(i, c, T):
    _pos(i=i, c=c, T=T)
    return {"Pi": i * c * R * T, "Pi_atm": i * c * R * T / ATM}


_proc("osmotic_pressure", name="Osmotic pressure", topic="solutions / colligative properties", level="intermediate",
      purpose="The osmotic pressure of a dilute solution (van 't Hoff equation).",
      inputs=[_I_IN(), _inp("c", "molar_concentration", "molar concentration"), _inp("T", "temperature", "temperature")],
      outputs=[_out("Pi", "pressure", "osmotic pressure"), _out("Pi_atm", "number", "in atm")],
      primary="Pi", run=osmotic_pressure, verify=lambda i, o: _rel(o["Pi"], i["i"] * i["c"] * R * i["T"]),
      reference=lambda i, c, T: {"Pi": i * c * _ref_vm(T, 1.0)},
      wrong=lambda i, c, T: {"Pi": i * c * R * (T - 273.15)},
      rand=lambda rng: {"i": rng.choice([1.0, 2.0, 3.0]), "c": _u_(rng, 0.01, 2000.0), "T": rng.uniform(260.0, 400.0)},
      kw=[r"\bosmotic pressure\b"], nokw=[r"\bmolar mass\b"], post=_nonelectrolyte,
      need=_has("molar_concentration", "temperature", exactly=True),
      latex=r"\Pi = i c R T", method="Pi = i c R T", conditions=["i, c, T > 0", "dilute solution"],
      templates=["What is the osmotic pressure of a 0.100 M glucose solution (a nonelectrolyte) at 25 °C?"],
      examples=[{"i": 1.0, "c": 100.0, "T": 298.15}, {"i": 2.0, "c": 150.0, "T": 310.0}],
      questions=["What is the osmotic pressure of a 0.100 M glucose solution (a nonelectrolyte) at 25 °C?",
                 "Find the osmotic pressure of 0.150 M NaCl at 310 K with i = 2.",
                 "What is the osmotic pressure of 0.150 M NaCl at 310 K?"])


def molar_mass_from_osmotic(m, V, Pi, T, i):
    _pos(m=m, V=V, Pi=Pi, T=T, i=i)
    M = i * m * R * T / (Pi * V)
    return {"M": M, "M_g": M * 1000.0}


_proc("molar_mass_from_osmotic_pressure", name="Molar mass from osmotic pressure",
      topic="solutions / colligative properties", level="advanced",
      purpose="The molar mass of a solute (e.g. a protein) from the osmotic pressure of its solution.",
      inputs=[_inp("m", "mass", "mass of solute"), _inp("V", "volume", "volume of solution"),
              _inp("Pi", "pressure", "osmotic pressure"), _inp("T", "temperature", "temperature"), _I_IN()],
      outputs=[_out("M", "molar_mass", "molar mass"), _out("M_g", "number", "in g/mol")],
      primary="M", run=molar_mass_from_osmotic,
      verify=lambda i, o: _rel(o["M"] * i["Pi"] * i["V"], i["i"] * i["m"] * R * i["T"]),
      reference=lambda m, V, Pi, T, i: {"M": _brentq(lambda M: i * (m / M) / V * _ref_vm(T, 1.0) - Pi, 1e-9, 1e9)},
      wrong=lambda m, V, Pi, T, i: {"M": i * m * R * T / (Pi * V) * 1000.0},
      rand=lambda rng: {"m": _u_(rng, 1e-4, 0.1), "V": _u_(rng, 1e-4, 1.0), "Pi": _u_(rng, 10.0, 1e6),
                        "T": rng.uniform(270.0, 330.0), "i": rng.choice([1.0, 1.0, 2.0])},
      kw=[r"\bmolar mass|molecular weight", r"\bosmotic"], post=_nonelectrolyte,
      need=_has("mass", "volume", "pressure", "temperature", exactly=True),
      latex=r"M = \frac{i m R T}{\Pi V}", method="c = Pi/(i R T); M = m / (c V)", conditions=["all inputs > 0"],
      templates=["A solution of 1.00 g of a protein (a nonelectrolyte) in 100 mL of water has an osmotic pressure of "
                 "2.50 torr at 25 °C. What is the molar mass of the protein?"],
      examples=[{"m": 0.001, "V": 1e-4, "Pi": 2.5 * ATM / 760, "T": 298.15, "i": 1.0}],
      questions=["A solution of 1.00 g of a protein (a nonelectrolyte) in 100 mL of water has an osmotic pressure of "
                 "2.50 torr at 25 °C. What is the molar mass of the protein?"])

# ====================================================================================================================
# CHEMICAL EQUILIBRIUM
# ====================================================================================================================
R_LATM = R / 101.325          # L·atm/(mol·K)


def _dn_text(q, spans):
    m = re.search(r"((?:\d+\s*)?[A-Z][A-Za-z0-9()]*\((?:g|s|l|aq)\)(?:\s*\+\s*(?:\d+\s*)?[A-Z][A-Za-z0-9()]*\((?:g|s|l|aq)\))*"
                  r"\s*(?:<=>|⇌|⇄|->|→|=)\s*(?:\d+\s*)?[A-Z][A-Za-z0-9()]*\((?:g|s|l|aq)\)"
                  r"(?:\s*\+\s*(?:\d+\s*)?[A-Z][A-Za-z0-9()]*\((?:g|s|l|aq)\))*)", q)
    if not m:
        return []
    left, right = re.split(r"\s*(?:<=>|⇌|⇄|->|→|=)\s*", m.group(1))
    gas = lambda side: sum(int(t.group(1) or 1) for t in re.finditer(r"(?:(\d+)\s*)?[A-Z][A-Za-z0-9()]*\(g\)", side))
    return [("dn", float(gas(right) - gas(left)), m.span(1))]


_DN = lambda: _inp("dn", "number", "change in moles of gas (products - reactants)",
                   labels=[r"δn(?:_?g)?", r"change in (?:the )?(?:number of )?moles of gas(?:es)?"],
                   symbols=["Δn", "dn", "Δng"], label_only=True)


def kp_from_kc(Kc, T, dn):
    _pos(Kc=Kc, T=T)
    _finite(dn=dn)
    return {"Kp": Kc * (R_LATM * T) ** dn}


def kc_from_kp(Kp, T, dn):
    _pos(Kp=Kp, T=T)
    _finite(dn=dn)
    return {"Kc": Kp / (R_LATM * T) ** dn}


def _ref_kp(Kc, T, dn):
    import scipy.constants as sc
    return Kc * (1000.0 * sc.R * T / sc.atm) ** dn


_proc("kp_from_kc", name="Kp from Kc", topic="equilibrium / equilibrium constants", level="intermediate",
      purpose="The pressure equilibrium constant from the concentration one (Kp = Kc (RT)^dn, atm and mol/L).",
      inputs=[_inp("Kc", "number", "concentration equilibrium constant", symbols=["Kc", "K_c"], label_only=True,
                   labels=[r"\bkc(?: is| of)?"]),
              _inp("T", "temperature", "temperature"), _DN()],
      outputs=[_out("Kp", "number", "pressure equilibrium constant (atm basis)")],
      primary="Kp", run=kp_from_kc, verify=lambda i, o: _rel(o["Kp"], i["Kc"] * (R_LATM * i["T"]) ** i["dn"]),
      reference=lambda Kc, T, dn: {"Kp": _ref_kp(Kc, T, dn)},
      wrong=lambda Kc, T, dn: {"Kp": Kc * (R * T) ** dn},
      rand=lambda rng: {"Kc": _u_(rng, 1e-4, 1e4), "T": rng.uniform(250.0, 1500.0), "dn": float(rng.choice([-2, -1, 1, 2, 3]))},
      kw=[r"\bkp\b", r"\bkc\b"], text=_texts(_dn_text), need=lambda t, qs: "⟦Kc⟧" in t and "⟦Kp⟧" not in t,
      latex=r"K_p = K_c (RT)^{\Delta n}", method="Kp = Kc (RT)^dn with R = 0.082057 L·atm/(mol·K)",
      conditions=["Kc, T > 0", "dn counts gaseous species only"],
      templates=["For N2(g) + 3H2(g) ⇌ 2NH3(g), Kc = 0.50 at 400 K. What is Kp?",
                 "Kc = 4.0 at 500 K and Δn = 1. Find Kp."],
      examples=[{"Kc": 0.5, "T": 400.0, "dn": -2.0}, {"Kc": 4.0, "T": 500.0, "dn": 1.0}],
      questions=["For N2(g) + 3H2(g) ⇌ 2NH3(g), Kc = 0.50 at 400 K. What is Kp?",
                 "Kc = 4.0 at 500 K and Δn = 1. Find Kp.", "Kc = 4.0 at 500 K. Find Kp."])

_proc("kc_from_kp", name="Kc from Kp", topic="equilibrium / equilibrium constants", level="intermediate",
      purpose="The concentration equilibrium constant from the pressure one (Kc = Kp / (RT)^dn).",
      inputs=[_inp("Kp", "number", "pressure equilibrium constant (atm basis)", symbols=["Kp", "K_p"], label_only=True,
                   labels=[r"\bkp(?: is| of)?"]),
              _inp("T", "temperature", "temperature"), _DN()],
      outputs=[_out("Kc", "number", "concentration equilibrium constant")],
      primary="Kc", run=kc_from_kp, verify=lambda i, o: _rel(o["Kc"] * (R_LATM * i["T"]) ** i["dn"], i["Kp"]),
      reference=lambda Kp, T, dn: {"Kc": Kp / _ref_kp(1.0, T, dn)},
      wrong=lambda Kp, T, dn: {"Kc": Kp * (R_LATM * T) ** dn},
      rand=lambda rng: {"Kp": _u_(rng, 1e-4, 1e4), "T": rng.uniform(250.0, 1500.0), "dn": float(rng.choice([-2, -1, 1, 2, 3]))},
      kw=[r"\bkp\b", r"\bkc\b"], text=_texts(_dn_text), need=lambda t, qs: "⟦Kp⟧" in t and "⟦Kc⟧" not in t,
      latex=r"K_c = \frac{K_p}{(RT)^{\Delta n}}", method="Kc = Kp / (RT)^dn with R = 0.082057 L·atm/(mol·K)",
      conditions=["Kp, T > 0", "dn counts gaseous species only"],
      templates=["For PCl5(g) ⇌ PCl3(g) + Cl2(g), Kp = 1.8 at 523 K. What is Kc?"],
      examples=[{"Kp": 1.8, "T": 523.0, "dn": 1.0}],
      questions=["For PCl5(g) ⇌ PCl3(g) + Cl2(g), Kp = 1.8 at 523 K. What is Kc?",
                 "Kp = 0.25 at 700 K and Δn = -1. Find Kc."])


def _ice_terms(equation):
    if re.search(r"\((?:s|l)\)", equation):
        raise ValueError("pure solids and liquids do not enter K: write the equation without them")
    r, p = parse_equation(equation)
    return [-float(c) for c, _f in r] + [float(c) for c, _f in p], [f for _c, f in r + p]


def _ice_bounds(nu, c0):
    lo = max(-c / n for c, n in zip(c0, nu) if n > 0)
    hi = min(c / -n for c, n in zip(c0, nu) if n < 0)
    if not lo < hi:
        raise ValueError("no reaction can take place from these initial concentrations")
    return lo, hi


def _lnq(nu, c0, x):
    s = 0.0
    for c, n in zip(c0, nu):
        v = c + n * x
        if v <= 0:
            return -math.inf if n > 0 else math.inf
        s += n * math.log(v)
    return s


def equilibrium_ice(equation, initial, K):
    nu, sp = _ice_terms(equation)
    if len(initial) != len(sp):
        raise ValueError(f"give one initial concentration per species ({', '.join(sp)})")
    if any(c < 0 for c in initial):
        raise ValueError("concentrations cannot be negative")
    _pos(K=K)
    lo, hi = _ice_bounds(nu, initial)
    lnK = math.log(K)
    x = _bisect(lambda x: _lnq(nu, initial, x) - lnK, lo, hi)
    return {"x": x, "equilibrium": [c + n * x for c, n in zip(initial, nu)]}


def _verify_ice(i, o):
    nu, sp = _ice_terms(i["equation"])
    x = o["x"]
    eq = [c + n * x for c, n in zip(i["initial"], nu)]
    if any(v <= 0 for v in eq):
        return 1.0
    given = o.get("equilibrium", eq)
    return max(abs(_lnq(nu, i["initial"], x) - math.log(i["K"])),
               max(_rel(a, b) for a, b in zip(given, eq)) if len(given) == len(eq) else 1.0)


def _ref_ice(equation, initial, K):
    nu, sp = _ice_terms(equation)
    lo, hi = _ice_bounds(nu, initial)

    def g(x):
        num = den = 1.0
        for c, n in zip(initial, nu):
            v = max(c + n * x, 0.0)
            if n > 0:
                num *= v ** n
            else:
                den *= v ** -n
        return num - K * den
    x = _brentq(g, lo, hi)
    return {"x": x}


_ICE_REACTIONS = ["N2O4 ⇌ 2 NO2", "H2 + I2 ⇌ 2 HI", "N2 + 3 H2 ⇌ 2 NH3", "PCl5 ⇌ PCl3 + Cl2", "CO + H2O ⇌ CO2 + H2",
                  "2 SO2 + O2 ⇌ 2 SO3", "2 HI ⇌ H2 + I2", "N2 + O2 ⇌ 2 NO", "COCl2 ⇌ CO + Cl2", "2 NOCl ⇌ 2 NO + Cl2"]


def _rand_ice(rng):
    eq = rng.choice(_ICE_REACTIONS)
    nu, sp = _ice_terms(eq)
    return {"equation": eq, "initial": [rng.choice([0.0, _u_(rng, 0.01, 2.0)]) for _ in sp], "K": _u_(rng, 1e-3, 1e3)}


def _conc_text(q):
    """The equation and one '[X] = value' per species (all must be stated; a missing one is asked for)."""
    eq, span = _equation_in(q)
    if not eq:
        return {}, []
    preset, spans = {"equation": eq}, [span]
    try:
        nu, sp = _ice_terms(eq)
    except ValueError:
        return preset, spans
    found = {}
    for m in re.finditer(r"\[\s*([A-Z][A-Za-z0-9()]*)\s*\]\s*(?:_?0|₀|_?i|initial)?\s*(?:=|:|is)\s*"
                         r"(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)\s*(?:M\b|mol/L|atm\b)?", q):
        if m.group(1) in sp and m.group(1) not in found and not (span[0] <= m.start() < span[1]):
            found[m.group(1)] = float(m.group(2))
            spans.append(m.span())
    if len(found) == len(sp):
        preset["initial"] = ", ".join(repr(found[s]) for s in sp)
    return preset, spans


_K_IN = lambda: _inp("K", "number", "equilibrium constant", labels=[r"equilibrium constant(?: is| of)?"],
                     symbols=["K", "Kc", "Kp", "K_c", "K_p", "Keq", "K_eq"])

_proc("equilibrium_ice", name="Equilibrium concentrations (ICE table)", topic="equilibrium / ICE tables",
      level="advanced",
      purpose="Equilibrium concentrations (or partial pressures) from the initial ones and K, for any one reaction as written.",
      inputs=[_inp("equation", "number", "the reaction as written (its coefficients define K)", type="text", min_length=4,
                   max_length=200),
              _inp("initial", "number", "initial concentration of each species in the order written (mol/L, or atm for Kp)",
                   type="vector", min_length=2, max_length=10),
              _K_IN()],
      outputs=[_out("x", "number", "extent of reaction (mol/L)"),
               _out("equilibrium", "number", "equilibrium concentrations in the order written")],
      primary="x", run=equilibrium_ice, verify=_verify_ice, reference=_ref_ice,
      wrong=lambda equation, initial, K: (lambda nu, sp: {"x": _bisect(
          lambda x: _lnq([math.copysign(1.0, n) for n in nu], initial, x) - math.log(K),
          *_ice_bounds([math.copysign(1.0, n) for n in nu], initial))})(*_ice_terms(equation)),
      rand=_rand_ice,
      kw=[r"\bequilibrium (?:concentrations?|partial pressures?|amounts?)|\bat equilibrium|\bice table"],
      nokw=[r"quotient|direction|shift|ksp|solubilit|\bph\b"], text=_conc_text, need=lambda t, qs: len(qs) <= 1,
      latex=r"K = \prod_i (c_{i,0} + \nu_i x)^{\nu_i}",
      method="the extent x solved from the mass-action expression by bisection between the limits set by the initial amounts",
      conditions=["one reaction, coefficients as written", "no pure solids or liquids", "K > 0"],
      templates=["For N2O4 ⇌ 2 NO2, Kc = 4.6e-3. Initially [N2O4] = 0.10 M and [NO2] = 0. "
                 "What are the equilibrium concentrations?"],
      examples=[{"equation": "N2O4 ⇌ 2 NO2", "initial": "0.10, 0", "K": 4.6e-3},
                {"equation": "H2 + I2 ⇌ 2 HI", "initial": "0.50, 0.50, 0", "K": 50.0}],
      questions=["For N2O4 ⇌ 2 NO2, Kc = 4.6e-3. Initially [N2O4] = 0.10 M and [NO2] = 0. "
                 "What are the equilibrium concentrations?",
                 "For H2 + I2 ⇌ 2 HI, K = 50.0. Initially [H2] = 0.50 M, [I2] = 0.50 M and [HI] = 0. "
                 "Find the equilibrium concentrations.",
                 "For N2O4 ⇌ 2 NO2, Kc = 4.6e-3. Initially [N2O4] = 0.10 M. What are the equilibrium concentrations?"])


def k_from_dg(dG, T):
    _finite(dG=dG)
    _pos(T=T)
    return {"K": math.exp(-dG / (R * T)), "lnK": -dG / (R * T)}


_DG = lambda: _inp("dG", "molar_energy", "standard Gibbs energy change", symbols=["ΔG°", "ΔG", "ΔGo", "dG"])

_proc("k_from_delta_g", name="Equilibrium constant from the standard Gibbs energy",
      topic="equilibrium / thermodynamics of equilibrium", level="intermediate",
      purpose="The equilibrium constant from the standard Gibbs energy change (K = exp(-dG°/RT)).",
      inputs=[_DG(), _inp("T", "temperature", "temperature")],
      outputs=[_out("K", "number", "equilibrium constant"), _out("lnK", "number", "ln K")],
      primary="lnK", run=k_from_dg, verify=lambda i, o: abs(o["lnK"] * R * i["T"] + i["dG"]) / (R * i["T"]) +
      _rel(o.get("K", math.exp(o["lnK"])), math.exp(o["lnK"])),
      reference=lambda dG, T: {"lnK": _brentq(lambda L: dG + __import__("scipy.constants").constants.R * T * L, -1e4, 1e4)},
      wrong=lambda dG, T: {"lnK": dG / (R * T), "K": math.exp(dG / (R * T))},
      rand=lambda rng: {"dG": rng.uniform(-1e5, 1e5), "T": rng.uniform(250.0, 1500.0)},
      kw=[r"\bequilibrium constant\b|(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])"], nokw=[r"\bδh|enthalpy|\bksp\b|\bq\b|quotient|\be°|potential"],
      need=_has("molar_energy", "temperature", exactly=True),
      latex=r"K = e^{-\Delta G^\circ / RT}", method="ln K = -dG°/(RT)", conditions=["T > 0"],
      templates=["ΔG° = -33.0 kJ/mol at 298 K. What is the equilibrium constant?"],
      examples=[{"dG": -33000.0, "T": 298.0}, {"dG": 10000.0, "T": 500.0}],
      questions=["ΔG° = -33.0 kJ/mol at 298 K. What is the equilibrium constant?",
                 "Find the equilibrium constant K at 500 K if the standard Gibbs energy change is 10.0 kJ/mol."])


def dg_from_k(K, T):
    _pos(K=K, T=T)
    return {"dG": -R * T * math.log(K)}


_proc("delta_g_from_k", name="Standard Gibbs energy from the equilibrium constant",
      topic="equilibrium / thermodynamics of equilibrium", level="intermediate",
      purpose="The standard Gibbs energy change from the equilibrium constant (dG° = -RT ln K).",
      inputs=[_K_IN(), _inp("T", "temperature", "temperature")],
      outputs=[_out("dG", "molar_energy", "standard Gibbs energy change")],
      primary="dG", run=dg_from_k, verify=lambda i, o: abs(o["dG"] / (R * i["T"]) + math.log(i["K"])),
      reference=lambda K, T: {"dG": _brentq(lambda g: -g / (__import__("scipy.constants").constants.R * T) - math.log(K),
                                            -1e8, 1e8)},
      wrong=lambda K, T: {"dG": -R * T * math.log10(K)},
      rand=lambda rng: {"K": _u_(rng, 1e-8, 1e8), "T": rng.uniform(250.0, 1500.0)},
      kw=[r"δg|gibbs"], nokw=[r"\bδh|enthalpy|entropy|\bq\b|quotient|\be°|potential|\bksp\b"],
      need=_has("none", "temperature", exactly=True),
      latex=r"\Delta G^\circ = -RT\ln K", method="dG° = -RT ln K", conditions=["K, T > 0"],
      templates=["K = 1.0e5 at 298 K. What is ΔG°?", "Find the standard Gibbs energy change if the equilibrium constant is 0.25 at 400 K."],
      examples=[{"K": 1.0e5, "T": 298.0}, {"K": 0.25, "T": 400.0}],
      questions=["K = 1.0e5 at 298 K. What is ΔG°?",
                 "Find the standard Gibbs energy change if the equilibrium constant is 0.25 at 400 K."])


def vant_hoff(K1, T1, T2, dH):
    _pos(K1=K1, T1=T1, T2=T2)
    _finite(dH=dH)
    lnK2 = math.log(K1) - dH / R * (1 / T2 - 1 / T1)
    return {"K2": math.exp(lnK2), "lnK2": lnK2}


def _ref_vant_hoff(K1, T1, T2, dH):
    from scipy.integrate import quad
    import scipy.constants as sc
    integral, _e = quad(lambda T: dH / (sc.R * T * T), T1, T2, epsabs=0, epsrel=1e-13, limit=200)
    return {"lnK2": math.log(K1) + integral}


_DH = lambda: _inp("dH", "molar_energy", "standard enthalpy change", symbols=["ΔH°", "ΔH", "ΔHo", "dH", "ΔH_rxn"])

_proc("vant_hoff_k2", name="Van 't Hoff equation: K at another temperature",
      topic="equilibrium / temperature dependence", level="advanced",
      purpose="The equilibrium constant at a second temperature from K at a first one and the reaction enthalpy.",
      inputs=[_inp("K1", "number", "equilibrium constant at T1", symbols=["K1", "K", "K_1"]),
              _inp("T1", "temperature", "first temperature", labels=L_T1, symbols=["T1"], role="initial"),
              _inp("T2", "temperature", "second temperature", labels=L_T2, symbols=["T2"], role="final"), _DH()],
      outputs=[_out("K2", "number", "equilibrium constant at T2"), _out("lnK2", "number", "ln K2")],
      primary="lnK2", run=vant_hoff, verify=lambda i, o: abs(o["lnK2"] - math.log(i["K1"]) + i["dH"] / R *
                                                             (1 / i["T2"] - 1 / i["T1"])),
      reference=_ref_vant_hoff,
      wrong=lambda K1, T1, T2, dH: {"lnK2": math.log(K1) + dH / R * (1 / T2 - 1 / T1)},
      rand=lambda rng: {"K1": _u_(rng, 1e-6, 1e6), "T1": rng.uniform(250.0, 1000.0), "T2": rng.uniform(250.0, 1000.0),
                        "dH": rng.uniform(-2e5, 2e5)},
      kw=[r"(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])|equilibrium constant", r"δh|enthalpy"], nokw=[r"\bδs|entropy|δg|gibbs"],
      need=_has("none", "temperature", "temperature", "molar_energy", exactly=True),
      latex=r"\ln\frac{K_2}{K_1} = -\frac{\Delta H^\circ}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)",
      method="integrated van 't Hoff equation (dH° constant)", conditions=["K1, T1, T2 > 0", "dH° independent of T"],
      templates=["K = 1.8e-5 at 298 K and ΔH° = 50.0 kJ/mol. What is K at 350 K?"],
      examples=[{"K1": 1.8e-5, "T1": 298.0, "T2": 350.0, "dH": 50000.0}],
      questions=["K = 1.8e-5 at 298 K and ΔH° = 50.0 kJ/mol. What is K at 350 K?",
                 "K1 = 0.50, T1 = 400 K, T2 = 500 K, ΔH° = -92.0 kJ/mol. Find the equilibrium constant at T2."])


def dg_nonstandard(dG0, T, Q):
    _finite(dG0=dG0)
    _pos(T=T, Q=Q)
    return {"dG": dG0 + R * T * math.log(Q)}


_proc("delta_g_nonstandard", name="Gibbs energy change at non-standard conditions",
      topic="equilibrium / thermodynamics of equilibrium", level="advanced",
      purpose="The Gibbs energy change of a reaction at a given reaction quotient (dG = dG° + RT ln Q).",
      inputs=[_inp("dG0", "molar_energy", "standard Gibbs energy change", symbols=["ΔG°", "ΔGo"],
                   labels=[r"standard gibbs (?:free )?energy(?: change)?(?: is| of)?"]),
              _inp("T", "temperature", "temperature"),
              _inp("Q", "number", "reaction quotient", symbols=["Q"], labels=[r"reaction quotient(?: is| of)?"])],
      outputs=[_out("dG", "molar_energy", "Gibbs energy change")],
      primary="dG", run=dg_nonstandard,
      verify=lambda i, o: abs(o["dG"] - i["dG0"] - R * i["T"] * math.log(i["Q"])) / max(1.0, abs(i["dG0"]), R * i["T"]),
      reference=lambda dG0, T, Q: {"dG": dG0 + __import__("scipy.constants").constants.R * T * float(__import__("numpy").log(Q))},
      wrong=lambda dG0, T, Q: {"dG": dG0 - R * T * math.log(Q)},
      rand=lambda rng: {"dG0": rng.uniform(-1e5, 1e5), "T": rng.uniform(250.0, 1000.0), "Q": _u_(rng, 1e-6, 1e6)},
      kw=[r"\bq\b|reaction quotient", r"δg|gibbs"], need=_has("molar_energy", "temperature", "none", exactly=True),
      latex=r"\Delta G = \Delta G^\circ + RT\ln Q", method="dG = dG° + RT ln Q", conditions=["T, Q > 0"],
      templates=["ΔG° = -33.0 kJ/mol, T = 298 K and Q = 100. What is ΔG?"],
      examples=[{"dG0": -33000.0, "T": 298.0, "Q": 100.0}],
      questions=["ΔG° = -33.0 kJ/mol, T = 298 K and Q = 100. What is ΔG?",
                 "The standard Gibbs energy change is 5.0 kJ/mol at 350 K and the reaction quotient is 0.010. Find ΔG."])

_METALS = {"Li", "Na", "K", "Rb", "Cs", "Be", "Mg", "Ca", "Sr", "Ba", "Al", "Ga", "In", "Tl", "Sn", "Pb", "Bi", "Sc",
           "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ag", "Cd", "Hg", "Au", "Pt", "Pd", "La", "Ce"}


def salt_ions(formula):
    """'CaF2' -> ('Ca', 1, 'F', 2); 'Ca3(PO4)2' -> ('Ca', 3, 'PO4', 2); None when not a simple salt."""
    m = re.fullmatch(r"([A-Z][a-z]?)(\d*)(.+)", formula)
    if not m or m.group(1) not in _METALS:
        return None
    cat, x, rest = m.group(1), int(m.group(2) or 1), m.group(3)
    if cat == "Hg" and x == 2:
        return None                                    # mercury(I) is Hg2(2+): not read by this rule
    mm = re.fullmatch(r"\(([A-Z][A-Za-z0-9]*)\)(\d+)", rest)
    if mm:
        return cat, x, mm.group(1), int(mm.group(2))
    mm = re.fullmatch(r"([A-Z][a-z]?)(\d*)", rest)
    if mm:
        return cat, x, mm.group(1), int(mm.group(2) or 1)
    if re.fullmatch(r"(?:[A-Z][a-z]?\d*){2,}", rest) and rest[0] not in "(":
        return cat, x, rest, 1
    return None


def _salt_text(cue):
    def g(q, spans):
        f, span = _formula_at(q, cue)
        if not f:
            return []
        ions = salt_ions(f)
        if not ions:
            return None
        return [("x", float(ions[1]), span), ("y", float(ions[3]), (span[1], span[1]))]
    return g


_XY = lambda: [_inp("x", "integer", "cations per formula unit", type="integer", symbols=["x"], label_only=True, min=1, max=4),
               _inp("y", "integer", "anions per formula unit", type="integer", symbols=["y"], label_only=True, min=1, max=4)]
_KSP = lambda: _inp("Ksp", "number", "solubility product", symbols=["Ksp", "K_sp"], labels=[r"\bksp(?: is| of)?"])


def ksp_solubility(Ksp, x, y):
    _pos(Ksp=Ksp)
    x, y = int(x), int(y)
    if x < 1 or y < 1:
        raise ValueError("x and y are whole numbers >= 1")
    s = (Ksp / (x ** x * y ** y)) ** (1.0 / (x + y))
    return {"s": s * 1000.0, "s_molL": s, "log10_s": math.log10(s)}


_proc("ksp_molar_solubility", name="Molar solubility from Ksp", topic="equilibrium / solubility", level="intermediate",
      purpose="The molar solubility of a sparingly soluble salt MxAy in pure water from its solubility product.",
      inputs=[_KSP()] + _XY(),
      outputs=[_out("s", "molar_concentration", "molar solubility"), _out("s_molL", "number", "in mol/L")],
      primary="s", run=ksp_solubility,
      verify=lambda i, o: _rel((i["x"] * o["s"] / 1000.0) ** i["x"] * (i["y"] * o["s"] / 1000.0) ** i["y"], i["Ksp"]),
      reference=lambda Ksp, x, y: (lambda s: {"s": 1000.0 * s, "log10_s": math.log10(s)})(
          math.exp(_brentq(lambda u: x * math.log(x) + y * math.log(y) + (x + y) * u - math.log(Ksp), -800.0, 10.0))),
      wrong=lambda Ksp, x, y: {"s": 1000.0 * Ksp ** (1.0 / (x + y)), "log10_s": math.log10(Ksp) / (x + y)},
      rand=lambda rng: {"Ksp": _u_(rng, 1e-50, 1e-3), "x": rng.choice([1, 1, 2, 3]), "y": rng.choice([1, 2, 2, 3])},
      kw=[r"\bsolubility\b", r"\bksp\b"], nokw=[r"\bin \d|common ion|precipit|\bksp\b[^.?]*\bwhat is the ksp|calculate the ksp|find the ksp|determine the ksp"],
      text=_texts(_salt_text(r"(?:solubility of|ksp of|Ksp of|for)")), need=_has("none", exactly=True),
      latex=r"K_{sp} = (x s)^x (y s)^y", method="s = (Ksp / (x^x y^y))^(1/(x+y))",
      conditions=["Ksp > 0", "no common ion, no side reactions"],
      templates=["What is the molar solubility of CaF2 (Ksp = 3.9e-11)?",
                 "Find the molar solubility of Ag2CrO4 if Ksp = 1.1 × 10^-12."],
      examples=[{"Ksp": 3.9e-11, "x": 1, "y": 2}, {"Ksp": 1.8e-10, "x": 1, "y": 1}],
      questions=["What is the molar solubility of CaF2 (Ksp = 3.9e-11)?",
                 "Find the molar solubility of Ag2CrO4 if Ksp = 1.1 × 10^-12.",
                 "What is the molar solubility of Ca3(PO4)2 (Ksp = 2.0e-29)?", "What is the molar solubility of CaF2?"])


def ksp_from_s(s, x, y):
    _pos(s=s)
    x, y = int(x), int(y)
    sl = s / 1000.0
    Ksp = (x * sl) ** x * (y * sl) ** y
    return {"Ksp": Ksp, "pKsp": -math.log10(Ksp)}


_proc("ksp_from_solubility", name="Ksp from molar solubility", topic="equilibrium / solubility", level="intermediate",
      purpose="The solubility product of a salt MxAy from its molar solubility.",
      inputs=[_inp("s", "molar_concentration", "molar solubility")] + _XY(),
      outputs=[_out("Ksp", "number", "solubility product")],
      primary="Ksp", run=ksp_from_s,
      verify=lambda i, o: _rel((o["Ksp"] / (i["x"] ** i["x"] * i["y"] ** i["y"])) ** (1.0 / (i["x"] + i["y"])) * 1000.0, i["s"]),
      reference=lambda s, x, y: (lambda K: {"Ksp": K, "pKsp": -float(__import__("numpy").log10(K))})(
          float(__import__("numpy").prod(__import__("numpy").power([x * s / 1000.0, y * s / 1000.0], [x, y])))),
      wrong=lambda s, x, y: {"Ksp": (s / 1000.0) ** (x + y), "pKsp": -(x + y) * math.log10(s / 1000.0)},
      rand=lambda rng: {"s": _u_(rng, 1e-6, 100.0), "x": rng.choice([1, 2, 3]), "y": rng.choice([1, 2, 3])},
      kw=[r"\bsolubility\b", r"\b(?:what is|calculate|find|determine)\b[^.?]*\bksp\b"], nokw=[r"common ion"],
      text=_texts(_salt_text(r"(?:solubility of|ksp of|Ksp of|for)")), need=_has("molar_concentration", exactly=True),
      latex=r"K_{sp} = (x s)^x (y s)^y", method="Ksp = (x s)^x (y s)^y", conditions=["s > 0"],
      templates=["The molar solubility of PbI2 is 1.5e-3 M. What is its Ksp?"],
      examples=[{"s": 1.5, "x": 1, "y": 2}],
      questions=["The molar solubility of PbI2 is 1.5e-3 M. What is its Ksp?",
                 "The molar solubility of AgCl is 1.3 × 10^-5 M. Calculate the Ksp of AgCl."])


def common_ion(Ksp, x, y, c_common, common):
    _pos(Ksp=Ksp, c_common=c_common)
    x, y = int(x), int(y)
    c = c_common / 1000.0
    ca, cb = (c, 0.0) if common == "cation" else (0.0, c)
    term = lambda k, c, u: math.log(k) + u if c == 0 else math.log(k * math.exp(u) + c)
    hi = ksp_solubility(Ksp, x, y)["s_molL"]
    u = _bisect(lambda u: x * term(x, ca, u) + y * term(y, cb, u) - math.log(Ksp), math.log(hi) - 600.0, math.log(hi))
    s = math.exp(u)
    return {"s": s * 1000.0, "s_molL": s, "log10_s": u / math.log(10.0)}


def _ref_common(Ksp, x, y, c_common, common):
    import numpy.polynomial.polynomial as P
    c = c_common / 1000.0
    ca, cb = (c, 0.0) if common == "cation" else (0.0, c)
    poly = P.polymul(P.polypow([ca, x], x), P.polypow([cb, y], y))     # expanded (x s + cM)^x (y s + cA)^y
    hi = (Ksp / (x ** x * y ** y)) ** (1.0 / (x + y))
    g = lambda u: math.log(float(P.polyval(math.exp(u), poly))) - math.log(Ksp)
    u = _brentq(g, math.log(hi) - 120.0, math.log(hi) + 1e-9)
    return {"s": 1000.0 * math.exp(u), "log10_s": u / math.log(10.0)}


def _common_text(q, spans):
    out = _salt_text(r"(?:solubility of|ksp of|Ksp of)")(q, spans)
    if not out:
        return out
    salt, _sp = _formula_at(q, r"(?:solubility of|ksp of|Ksp of)")
    m = re.search(r"\bin\s+(?:a\s+)?(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)\s*(M|mol/L|mM)\s+(?:of\s+)?(" + _FTOKEN + r")", q)
    if not m:
        return out
    other = salt_ions(m.group(3).rstrip(".,?)"))
    mine = salt_ions(salt)
    if not other:
        return None
    c = float(m.group(1)) * (1.0 if m.group(2) == "mM" else 1000.0)
    if other[0] == mine[0]:
        return out + [("common", "cation", m.span()), ("c_common", c * other[1], (m.end(), m.end()))]
    if other[2] == mine[2]:
        return out + [("common", "anion", m.span()), ("c_common", c * other[3], (m.end(), m.end()))]
    return None


_proc("common_ion_solubility", name="Molar solubility with a common ion", topic="equilibrium / solubility",
      level="advanced",
      purpose="The molar solubility of a salt MxAy in a solution that already contains one of its ions.",
      inputs=[_KSP()] + _XY() + [
          _inp("c_common", "molar_concentration", "concentration of the common ion", symbols=["c_common"], label_only=True),
          _inp("common", "number", "which ion is common", type="choice", choices=["cation", "anion"])],
      outputs=[_out("s", "molar_concentration", "molar solubility"), _out("s_molL", "number", "in mol/L")],
      primary="s", run=common_ion,
      verify=lambda i, o: (lambda s, ca, cb: abs(i["x"] * math.log(i["x"] * s + ca) + i["y"] * math.log(i["y"] * s + cb)
                                                 - math.log(i["Ksp"])) if s > 0 else 1.0)(
          o["s"] / 1000.0, i["c_common"] / 1000.0 if i["common"] == "cation" else 0.0,
          i["c_common"] / 1000.0 if i["common"] == "anion" else 0.0),
      reference=_ref_common,
      wrong=lambda Ksp, x, y, c_common, common: ksp_solubility(Ksp, x, y),
      rand=lambda rng: {"Ksp": _u_(rng, 1e-30, 1e-6), "x": rng.choice([1, 1, 2]), "y": rng.choice([1, 2]),
                        "c_common": _u_(rng, 1.0, 1000.0), "common": rng.choice(["cation", "anion"])},
      kw=[r"\bsolubility\b", r"\bksp\b", r"\bin\s+(?:a\s+)?\d+(?:\.\d*)?(?:e-?\d+)?\s*(?:m|mol/l|mm)\b"], text=_texts(_common_text), need=_has("none", exactly=True),
      latex=r"K_{sp} = (x s + c_{M})^x (y s + c_{A})^y",
      method="the solubility s solved from the Ksp expression with the common ion's concentration, by bisection",
      conditions=["Ksp > 0", "the common ion comes from a fully dissolved salt"],
      templates=["What is the molar solubility of CaF2 in 0.10 M NaF (Ksp = 3.9e-11)?"],
      examples=[{"Ksp": 3.9e-11, "x": 1, "y": 2, "c_common": 100.0, "common": "anion"}],
      questions=["What is the molar solubility of CaF2 in 0.10 M NaF (Ksp = 3.9e-11)?",
                 "Find the molar solubility of AgCl in 0.010 M AgNO3 (Ksp = 1.8e-10).",
                 "What is the molar solubility of PbI2 in 0.050 M Pb(NO3)2 (Ksp = 9.8e-9)?"])

# ====================================================================================================================
# ACIDS AND BASES
# ====================================================================================================================
def _ph(h_molL):
    return -math.log10(h_molL)


_H_LAB = [r"\[h\+\]", r"\[h3o\+\]", r"hydronium(?: ion)? concentration", r"hydrogen ion concentration",
          r"concentration of (?:h\+|h3o\+|hydronium(?: ions?)?|hydrogen ions?)"]
_OH_LAB = [r"\[oh-\]", r"hydroxide(?: ion)? concentration", r"concentration of (?:oh-|hydroxide(?: ions?)?)"]
_ASK_PH = r"\b(?:what|find|calculate|compute|determine)\b[^?]*\bp(?:o)?h\b"


def ph_from_h(h):
    _pos(h=h)
    pH = _ph(h / 1000.0)
    return {"pH": pH, "pOH": 14.0 - pH, "oh": KW / (h / 1000.0) * 1000.0}


_proc("ph_from_hydronium", name="pH from the hydronium concentration", topic="acids and bases / pH", level="beginner",
      purpose="pH, pOH and [OH-] from the hydronium (hydrogen ion) concentration at 25 °C.",
      inputs=[_inp("h", "molar_concentration", "hydronium ion concentration", labels=_H_LAB)],
      outputs=[_out("pH", "number", "pH"), _out("pOH", "number", "pOH"), _out("oh", "molar_concentration", "[OH-]")],
      primary="pH", run=ph_from_h, verify=lambda i, o: abs(o["pH"] + math.log10(i["h"] / 1000.0)),
      reference=lambda h: {"pH": _brentq(lambda p: math.log(10.0 ** -p) - math.log(h / 1000.0), -10.0, 40.0)},
      wrong=lambda h: {"pH": math.log10(h / 1000.0)},
      rand=lambda rng: {"h": 1000.0 * 10 ** -rng.uniform(-1.0, 15.0)},
      kw=[_ASK_PH, "|".join(_H_LAB)], nokw=[r"\bk_?a|\bpka|\bk_?b|buffer|titrat|hydroxide|\[oh"],
      need=_has("molar_concentration", exactly=True),
      latex=r"\mathrm{pH} = -\log_{10}[\mathrm{H_3O^+}]", method="pH = -log10[H3O+]; pOH = 14 - pH (25 °C)",
      conditions=["[H3O+] > 0 (mol/L)", "25 °C (Kw = 1.0e-14)"],
      templates=["What is the pH of a solution with [H+] = 1.0e-3 M?",
                 "Calculate the pH if the hydronium ion concentration is 2.5 × 10^-5 M."],
      examples=[{"h": 1.0}, {"h": 0.025}],
      questions=["What is the pH of a solution with [H+] = 1.0e-3 M?",
                 "Calculate the pH if the hydronium ion concentration is 2.5 × 10^-5 M."])


def h_from_ph(pH):
    _finite(pH=pH)
    h = 10.0 ** -pH
    return {"h": h * 1000.0, "h_molL": h, "pOH": 14.0 - pH, "oh": KW / h * 1000.0}


_proc("hydronium_from_ph", name="Hydronium concentration from pH", topic="acids and bases / pH", level="beginner",
      purpose="[H3O+], [OH-] and pOH from the pH at 25 °C.",
      inputs=[_inp("pH", "number", "pH", symbols=["pH"], labels=[r"\bph(?: of| is)?"])],
      outputs=[_out("h", "molar_concentration", "[H3O+]"), _out("h_molL", "number", "[H3O+] in mol/L"),
               _out("pOH", "number", "pOH"), _out("oh", "molar_concentration", "[OH-]")],
      primary="h", run=h_from_ph, verify=lambda i, o: _rel(o["h"], 1000.0 * 10.0 ** -i["pH"]),
      reference=lambda pH: {"h": 1000.0 * float(__import__("numpy").power(10.0, -pH))},
      wrong=lambda pH: {"h": 1000.0 * 10.0 ** -(14.0 - pH)},
      rand=lambda rng: {"pH": rng.uniform(0.0, 14.0) if rng.random() < 0.9 else rng.uniform(6.5, 7.5) + 0.31},
      kw=[r"\b(?:what|find|calculate|determine)\b[^?]*(?:" + "|".join(_H_LAB) + r")", r"\bph\b"],
      nokw=[r"\bk_?a|\bpka|\bk_?b|buffer|titrat|\bpoh\s*(?:=|of|is)"],
      need=_has("none", exactly=True),
      latex=r"[\mathrm{H_3O^+}] = 10^{-\mathrm{pH}}", method="[H3O+] = 10^-pH", conditions=["25 °C (Kw = 1.0e-14)"],
      templates=["What is [H+] in a solution of pH 3.20?", "Find the hydronium ion concentration of a solution with pH = 9.5."],
      examples=[{"pH": 3.2}, {"pH": 9.5}],
      questions=["What is [H+] in a solution of pH 3.20?",
                 "Find the hydronium ion concentration of a solution with pH = 9.5."])


def ph_from_oh(oh):
    _pos(oh=oh)
    pOH = _ph(oh / 1000.0)
    return {"pH": 14.0 - pOH, "pOH": pOH}


_proc("ph_from_hydroxide", name="pH from the hydroxide concentration", topic="acids and bases / pH", level="beginner",
      purpose="pOH and pH from the hydroxide ion concentration at 25 °C.",
      inputs=[_inp("oh", "molar_concentration", "hydroxide ion concentration", labels=_OH_LAB)],
      outputs=[_out("pH", "number", "pH"), _out("pOH", "number", "pOH")],
      primary="pH", run=ph_from_oh, verify=lambda i, o: abs(14.0 - o["pH"] + math.log10(i["oh"] / 1000.0)),
      reference=lambda oh: {"pH": 14.0 - _brentq(lambda p: math.log(10.0 ** -p) - math.log(oh / 1000.0), -10.0, 40.0)},
      wrong=lambda oh: {"pH": -math.log10(oh / 1000.0)},
      rand=lambda rng: {"oh": 1000.0 * 10 ** -rng.uniform(-1.0, 15.0)},
      kw=[_ASK_PH, "|".join(_OH_LAB)], nokw=[r"\bk_?a|\bpka|\bk_?b|buffer|titrat|\[h"],
      need=_has("molar_concentration", exactly=True),
      latex=r"\mathrm{pH} = 14 + \log_{10}[\mathrm{OH^-}]", method="pOH = -log10[OH-]; pH = 14 - pOH (25 °C)",
      conditions=["[OH-] > 0", "25 °C"],
      templates=["What is the pH of a solution with [OH-] = 1.0e-4 M?"],
      examples=[{"oh": 0.1}],
      questions=["What is the pH of a solution with [OH-] = 1.0e-4 M?",
                 "Find the pOH and pH if the hydroxide ion concentration is 3.2 × 10^-3 M."])


def ph_strong_acid(c):
    _pos(c=c)
    cl = c / 1000.0
    h = cl / 2 + math.sqrt(cl * cl / 4 + KW)
    return {"pH": _ph(h), "h": h * 1000.0}


def _ref_quad_root(coeffs):
    import numpy as np
    r = [x.real for x in np.roots(coeffs) if abs(x.imag) <= 1e-9 * abs(x) and x.real > 0]
    return max(r)


def _polish(f, x0):
    """A root bracketed around a polynomial root estimate, refined by Brent's method in log space."""
    return math.exp(_brentq(lambda u: f(math.exp(u)), math.log(x0) - 0.01, math.log(x0) + 0.01))


_STRONG_ACID_RX = r"\b(?:hcl|hbr|hi|hno3|hclo4|hclo3|hydrochloric acid|hydrobromic acid|hydroiodic acid|nitric acid|" \
                  r"perchloric acid|strong (?:monoprotic )?acid)\b"
_proc("ph_strong_acid", name="pH of a strong monoprotic acid", topic="acids and bases / strong acids", level="beginner",
      purpose="The pH of a strong monoprotic acid solution (water's own ionization included, so dilute solutions are right).",
      inputs=[_inp("c", "molar_concentration", "acid concentration")],
      outputs=[_out("pH", "number", "pH"), _out("h", "molar_concentration", "[H3O+]")],
      primary="pH", run=ph_strong_acid,
      verify=lambda i, o: (lambda h: abs(h - i["c"] / 1000.0 - KW / h) / h)(10.0 ** -o["pH"]),
      reference=lambda c: {"pH": _ph(_polish(lambda h: h - c / 1000.0 - KW / h,
                                             _ref_quad_root([1.0, -c / 1000.0, -KW])))},
      wrong=lambda c: {"pH": -math.log10(c)},
      rand=lambda rng: {"c": 1000.0 * 10 ** -rng.uniform(-1.0, 9.0)},
      kw=[_ASK_PH, _STRONG_ACID_RX], nokw=[r"\bk_?a|\bpka|buffer|titrat|naoh|koh|hydroxide|\[h|\[oh|\bmix|\badd"],
      need=_has("molar_concentration", exactly=True),
      latex=r"[\mathrm{H_3O^+}] = \frac{c}{2} + \sqrt{\frac{c^2}{4} + K_w}",
      method="charge balance [H3O+] = c + Kw/[H3O+] solved exactly", conditions=["monoprotic strong acid", "25 °C"],
      templates=["What is the pH of 0.010 M HCl?", "Calculate the pH of a 1.0e-8 M HNO3 solution."],
      examples=[{"c": 10.0}, {"c": 1e-5}],
      questions=["What is the pH of 0.010 M HCl?", "Calculate the pH of a 1.0e-8 M HNO3 solution.",
                 "What is the pH of 0.25 M hydrochloric acid?"])

_BASE_OH = {"naoh": 1, "koh": 1, "lioh": 1, "rboh": 1, "csoh": 1, "ca(oh)2": 2, "ba(oh)2": 2, "sr(oh)2": 2,
            "sodium hydroxide": 1, "potassium hydroxide": 1, "lithium hydroxide": 1, "calcium hydroxide": 2,
            "barium hydroxide": 2, "strontium hydroxide": 2}


def _base_text(q, spans):
    ql = q.lower()
    for name in sorted(_BASE_OH, key=len, reverse=True):
        i = ql.find(name)
        if i >= 0:
            return [("n_oh", _BASE_OH[name], (i, i + len(name)))]
    return []


def ph_strong_base(c, n_oh):
    _pos(c=c)
    oh0 = c / 1000.0 * int(n_oh)
    oh = oh0 / 2 + math.sqrt(oh0 * oh0 / 4 + KW)
    return {"pH": 14.0 - _ph(oh), "pOH": _ph(oh)}


_proc("ph_strong_base", name="pH of a strong base", topic="acids and bases / strong bases", level="beginner",
      purpose="The pH and pOH of a strong (group 1 or 2) hydroxide solution.",
      inputs=[_inp("c", "molar_concentration", "base concentration"),
              _inp("n_oh", "integer", "hydroxide ions per formula unit", type="integer", min=1, max=3,
                   symbols=["n_oh"], label_only=True)],
      outputs=[_out("pH", "number", "pH"), _out("pOH", "number", "pOH")],
      primary="pH", run=ph_strong_base,
      verify=lambda i, o: (lambda oh: abs(oh - i["c"] / 1000.0 * i["n_oh"] - KW / oh) / oh)(10.0 ** -(14.0 - o["pH"])),
      reference=lambda c, n_oh: {"pH": 14.0 - _ph(_polish(lambda x: x - c / 1000.0 * n_oh - KW / x,
                                                            _ref_quad_root([1.0, -c / 1000.0 * n_oh, -KW])))},
      wrong=lambda c, n_oh: {"pH": _ph(c / 1000.0 * n_oh)},
      rand=lambda rng: {"c": 1000.0 * 10 ** -rng.uniform(-0.5, 8.0), "n_oh": rng.choice([1, 2])},
      kw=[_ASK_PH, r"naoh|koh|lioh|rboh|csoh|ca\(oh\)2|ba\(oh\)2|sr\(oh\)2|(?:sodium|potassium|lithium|calcium|barium|"
                   r"strontium) hydroxide|strong base"],
      nokw=[r"\bk_?b|\bk_?a|buffer|titrat|\[oh|hydroxide (?:ion )?concentration|\bmix|\badd"],
      text=_texts(_base_text), need=_has("molar_concentration", exactly=True),
      latex=r"[\mathrm{OH^-}] = n c,\quad \mathrm{pH} = 14 - \mathrm{pOH}",
      method="charge balance with water's ionization, [OH-] = n c + Kw/[OH-]", conditions=["strong base fully dissociated", "25 °C"],
      templates=["What is the pH of 0.010 M NaOH?", "Calculate the pH of 0.0050 M Ba(OH)2."],
      examples=[{"c": 10.0, "n_oh": 1}, {"c": 5.0, "n_oh": 2}],
      questions=["What is the pH of 0.010 M NaOH?", "Calculate the pH of 0.0050 M Ba(OH)2.",
                 "What is the pH of a 0.020 M strong base?"])


def _pka_text(sym, word):
    def g(q, spans):
        m = re.search(r"\bp" + word + r"\s*(?:=|:|of|is)?\s*(?:about\s*)?(\d+(?:\.\d*)?)", q, re.I)
        return [(sym, 10.0 ** -float(m.group(1)), m.span())] if m else []
    return g


def _weak_h(Ka, C):
    """[H3O+] of a weak monoprotic acid (water included): h = Ka C/(Ka + h) + Kw/h, by bisection in log space."""
    f = lambda u: (lambda h: h - Ka * C / (Ka + h) - KW / h)(math.exp(u))
    return math.exp(_bisect(f, math.log(1e-7) - 1e-9, math.log(C + 1e-6)))


def ph_weak_acid(Ka, c):
    _pos(Ka=Ka, c=c)
    C = c / 1000.0
    h = _weak_h(Ka, C)
    a = Ka * C / (Ka + h)
    return {"pH": _ph(h), "h": h * 1000.0, "percent_ionized": 100.0 * a / C}


def _ref_weak(Ka, C):
    return _polish(lambda h: h - Ka * C / (Ka + h) - KW / h, _ref_quad_root([1.0, Ka, -(Ka * C + KW), -Ka * KW]))


_KLAB = r"(?:\s+(?:of|for)\s+[a-z0-9() ]{1,30}?)?(?:\s+is|\s+value)?"
_KA = lambda: _inp("Ka", "number", "acid dissociation constant", symbols=["Ka", "K_a"], labels=[r"\bk_?a\b" + _KLAB],
                   label_only=True)
_KB = lambda: _inp("Kb", "number", "base dissociation constant", symbols=["Kb", "K_b"], labels=[r"\bk_?b\b" + _KLAB],
                   label_only=True)
_KA_GIVEN = r"\bk_?a\s*(?:\([^)]*\)\s*)?(?:=|:|of|for|is|value)|\bpk_?a\b"
_KB_GIVEN = r"\bk_?b\s*(?:\([^)]*\)\s*)?(?:=|:|of|for|is|value)|\bpk_?b\b"
_SALT = r"\b(?:salt|sodium|potassium|lithium|conjugate|ammonium|chloride)\b"

_proc("ph_weak_acid", name="pH of a weak acid", topic="acids and bases / weak acids", level="intermediate",
      purpose="The pH and percent ionization of a weak monoprotic acid from Ka (or pKa) and its concentration.",
      inputs=[_KA(), _inp("c", "molar_concentration", "acid concentration")],
      outputs=[_out("pH", "number", "pH"), _out("h", "molar_concentration", "[H3O+]"),
               _out("percent_ionized", "number", "percent ionization")],
      primary="pH", run=ph_weak_acid,
      verify=lambda i, o: (lambda h, C: abs(h - i["Ka"] * C / (i["Ka"] + h) - KW / h) / h)(10.0 ** -o["pH"], i["c"] / 1000.0),
      reference=lambda Ka, c: {"pH": _ph(_ref_weak(Ka, c / 1000.0))},
      wrong=lambda Ka, c: {"pH": _ph(math.sqrt(Ka * c))},
      rand=lambda rng: {"Ka": _u_(rng, 1e-12, 1e-1), "c": 1000.0 * _u_(rng, 1e-6, 2.0)},
      kw=[_ASK_PH + r"|percent ioni[sz]ation|degree of ioni[sz]ation", _KA_GIVEN],
      nokw=[_SALT, r"buffer|titrat|\bk_?b|ka1|ka2|\badd|\bmix"], text=_texts(_pka_text("Ka", "ka")),
      need=_has("molar_concentration", exactly=True),
      latex=r"[\mathrm{H_3O^+}] = \frac{K_a c}{K_a + [\mathrm{H_3O^+}]} + \frac{K_w}{[\mathrm{H_3O^+}]}",
      method="exact charge balance (no 5% approximation), solved by bisection", conditions=["monoprotic acid", "25 °C"],
      templates=["What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?", "Find the pH of a 0.050 M weak acid with pKa = 4.20."],
      examples=[{"Ka": 1.8e-5, "c": 100.0}, {"Ka": 10 ** -4.2, "c": 50.0}],
      questions=["What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?",
                 "Find the pH of a 0.050 M weak acid with pKa = 4.20.",
                 "What is the percent ionization of 0.20 M HF (Ka = 6.8e-4)?", "What is the pH of HF (Ka = 6.8e-4)?"])


def ph_weak_base(Kb, c):
    _pos(Kb=Kb, c=c)
    C = c / 1000.0
    oh = _weak_h(Kb, C)
    return {"pH": 14.0 - _ph(oh), "pOH": _ph(oh), "percent_ionized": 100.0 * Kb / (Kb + oh)}


_proc("ph_weak_base", name="pH of a weak base", topic="acids and bases / weak bases", level="intermediate",
      purpose="The pH, pOH and percent ionization of a weak base from Kb (or pKb) and its concentration.",
      inputs=[_KB(), _inp("c", "molar_concentration", "base concentration")],
      outputs=[_out("pH", "number", "pH"), _out("pOH", "number", "pOH"), _out("percent_ionized", "number", "percent ionization")],
      primary="pH", run=ph_weak_base,
      verify=lambda i, o: (lambda x, C: abs(x - i["Kb"] * C / (i["Kb"] + x) - KW / x) / x)(10.0 ** -(14.0 - o["pH"]), i["c"] / 1000.0),
      reference=lambda Kb, c: {"pH": 14.0 - _ph(_ref_weak(Kb, c / 1000.0))},
      wrong=lambda Kb, c: {"pH": _ph(_ref_weak_plain(Kb, c / 1000.0))},
      rand=lambda rng: {"Kb": _u_(rng, 1e-12, 1e-1), "c": 1000.0 * _u_(rng, 1e-6, 2.0)},
      kw=[_ASK_PH + r"|percent ioni[sz]ation", _KB_GIVEN], nokw=[_SALT, r"buffer|titrat|\bk_?a|\badd|\bmix"],
      text=_texts(_pka_text("Kb", "kb")), need=_has("molar_concentration", exactly=True),
      latex=r"[\mathrm{OH^-}] = \frac{K_b c}{K_b + [\mathrm{OH^-}]} + \frac{K_w}{[\mathrm{OH^-}]}",
      method="exact charge balance for [OH-], solved by bisection; pH = 14 - pOH", conditions=["25 °C"],
      templates=["What is the pH of 0.10 M NH3 (Kb = 1.8 × 10^-5)?"],
      examples=[{"Kb": 1.8e-5, "c": 100.0}],
      questions=["What is the pH of 0.10 M NH3 (Kb = 1.8 × 10^-5)?",
                 "Find the pH of a 0.25 M weak base with pKb = 3.36."])


def _ref_weak_plain(K, C):
    return _weak_h(K, C)          # used by the wrong variant only (it reports the pOH as the pH)


def ka_from_ph(pH, c):
    _finite(pH=pH)
    _pos(c=c)
    C = c / 1000.0
    h = 10.0 ** -pH
    a = h - KW / h
    if a <= 0 or a >= C:
        raise ValueError("this pH is not possible for a weak acid of that concentration")
    Ka = h * a / (C - a)
    return {"Ka": Ka, "pKa": -math.log10(Ka)}


_proc("ka_from_ph", name="Ka from the pH of a weak acid", topic="acids and bases / weak acids", level="intermediate",
      purpose="The acid dissociation constant of a weak monoprotic acid from the pH of a solution of known concentration.",
      inputs=[_inp("pH", "number", "measured pH", symbols=["pH"], labels=[r"\bph(?: of| is)?"]),
              _inp("c", "molar_concentration", "acid concentration")],
      outputs=[_out("pKa", "number", "pKa"), _out("Ka", "number", "Ka")],
      primary="pKa", run=ka_from_ph,
      verify=lambda i, o: abs(-math.log10(_weak_h(10.0 ** -o["pKa"], i["c"] / 1000.0)) - i["pH"]),
      reference=lambda pH, c: {"pKa": _brentq(lambda pk: _ph(_ref_weak(10.0 ** -pk, c / 1000.0)) - pH, -3.0, 20.0)},
      wrong=lambda pH, c: (lambda h: {"pKa": -math.log10(h * h / (c / 1000.0))})(10.0 ** -pH),
      rand=lambda rng: (lambda Ka, c: {"pH": ph_weak_acid(Ka, c)["pH"], "c": c})(_u_(rng, 1e-9, 1e-2), 1000.0 * _u_(rng, 1e-3, 1.0)),
      # the question ASKS for Ka (not "find the pH ... where the acid dissociation constant is 0.0043": that stated
      # Ka was read as the pH and answered, tools/ask_sweep.py 2026-10-23)
      kw=[r"\b(?:what|find|calculate|determine|compute)\s+(?:is\s+|are\s+)?(?:its\s+|the\s+)?(?:value\s+of\s+(?:the\s+|its\s+)?)?"
          r"(?:k_?a|pk_?a|acid dissociation constant)\b", r"\bph\b"],
      nokw=[r"\bk_?a\s*(?:=|:)|\bpk_?a\s*(?:=|:|of\s*\d)|buffer|titrat|\bk_?b|\bk_?a\s+(?:of|for|is|value)\b"
            r"|acid dissociation constant\s+(?:is|of|=|:|equal)"],
      need=_has("molar_concentration", "none", exactly=True),
      latex=r"K_a = \frac{[\mathrm{H_3O^+}][\mathrm{A^-}]}{c - [\mathrm{A^-}]},\ [\mathrm{A^-}] = [\mathrm{H_3O^+}] - K_w/[\mathrm{H_3O^+}]",
      method="[A-] from the charge balance at the measured pH, then the mass-action expression", conditions=["monoprotic weak acid", "25 °C"],
      templates=["A 0.10 M solution of a weak acid has a pH of 2.87. What is its Ka?"],
      examples=[{"pH": 2.87, "c": 100.0}],
      questions=["A 0.10 M solution of a weak acid has a pH of 2.87. What is its Ka?",
                 "The pH of 0.050 M HNO2 is 2.30. Find the acid dissociation constant."])


def buffer_ph(Ka, c_base, c_acid):
    _pos(Ka=Ka, c_base=c_base, c_acid=c_acid)
    return {"pH": -math.log10(Ka) + math.log10(c_base / c_acid)}


_proc("buffer_ph", name="Buffer pH (Henderson-Hasselbalch)", topic="acids and bases / buffers", level="intermediate",
      purpose="The pH of a buffer from the acid's Ka (or pKa) and the concentrations of the weak acid and its conjugate base.",
      inputs=[_KA(), _inp("c_base", "molar_concentration", "conjugate base concentration",
                          labels=[r"\[a-\]", r"conjugate base(?: concentration)?(?: of| is)?", r"\bbase"],
                          suffix_labels=[r"(?:its )?conjugate base|salt|base\b|sodium|potassium|acetate|ammonia|nh3|nh_3"]),
              _inp("c_acid", "molar_concentration", "weak acid concentration", labels=[r"\[ha\]", r"weak acid(?: of| is)?", r"\bacid"],
                   suffix_labels=[r"(?:weak )?acid\b|ch3cooh|hf\b|ammonium|nh4cl|hcooh|hno2"])],
      outputs=[_out("pH", "number", "pH")],
      primary="pH", run=buffer_ph,
      verify=lambda i, o: abs(o["pH"] + math.log10(i["Ka"]) - math.log10(i["c_base"] / i["c_acid"])),
      reference=lambda Ka, c_base, c_acid: {"pH": -float(__import__("numpy").log10(Ka * c_acid / c_base))},
      wrong=lambda Ka, c_base, c_acid: {"pH": -math.log10(Ka) + math.log10(c_acid / c_base)},
      rand=lambda rng: {"Ka": _u_(rng, 1e-11, 1e-2), "c_base": _u_(rng, 1.0, 2000.0), "c_acid": _u_(rng, 1.0, 2000.0)},
      kw=[r"\bbuffer\b|conjugate base", _ASK_PH], nokw=[r"\badd|titrat|\bk_?b"],
      text=_texts(_pka_text("Ka", "ka")), need=_has("molar_concentration", "molar_concentration", exactly=True),
      latex=r"\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{[\mathrm{A^-}]}{[\mathrm{HA}]}",
      method="Henderson-Hasselbalch equation", conditions=["both concentrations well above Ka and [H3O+]"],
      templates=["What is the pH of a buffer that is 0.10 M acetic acid and 0.15 M sodium acetate (pKa = 4.76)?",
                 "Find the pH of a buffer with [HA] = 0.20 M, [A-] = 0.10 M and Ka = 1.8 × 10^-5."],
      examples=[{"Ka": 10 ** -4.76, "c_base": 150.0, "c_acid": 100.0}],
      questions=["What is the pH of a buffer that is 0.10 M acetic acid and 0.15 M sodium acetate (pKa = 4.76)?",
                 "Find the pH of a buffer with [HA] = 0.20 M, [A-] = 0.10 M and Ka = 1.8 × 10^-5."])

_ACID_H = {"HCl": 1, "HBr": 1, "HI": 1, "HNO3": 1, "HClO4": 1, "HClO3": 1, "CH3COOH": 1, "HF": 1, "HCOOH": 1, "HCN": 1,
           "HNO2": 1, "HClO": 1, "C6H5COOH": 1, "HC2H3O2": 1, "H2SO4": 2, "H3PO4": 3, "H2C2O4": 2, "H2CO3": 2}
_BASE_N = {"NaOH": 1, "KOH": 1, "LiOH": 1, "Ca(OH)2": 2, "Ba(OH)2": 2, "Sr(OH)2": 2, "NH3": 1}
_STRONG_A = {"HCl", "HBr", "HI", "HNO3", "HClO4", "HClO3"}
_STRONG_B = {"NaOH", "KOH", "LiOH", "Ca(OH)2", "Ba(OH)2", "Sr(OH)2"}
_WEAK_A = {"CH3COOH", "HF", "HCOOH", "HCN", "HNO2", "HClO", "C6H5COOH", "HC2H3O2"}
_VOLRX = r"(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)\s*(mL|L)\b"
_CONCRX = r"(\d+(?:\.\d*)?(?:[eE][-+]?\d+)?)\s*(M|mol/L|mM)\b"
_VF = {"mL": 1e-6, "L": 1e-3}
_CF = {"M": 1000.0, "mol/L": 1000.0, "mM": 1.0}


def _neut_text(q, spans):
    t = re.search(r"volume of\s+(?:a\s+)?" + _CONCRX + r"\s+(?:of\s+)?(" + _FTOKEN + r")", q)
    a = re.search(_VOLRX + r"\s+of\s+(?:a\s+)?" + _CONCRX + r"\s+(?:of\s+)?(" + _FTOKEN + r")", q)
    if not t or not a:
        return []
    ft, fa = t.group(3).rstrip(".,?"), a.group(5).rstrip(".,?")
    out = [("c_t", float(t.group(1)) * _CF[t.group(2)], t.span()),
           ("V_a", float(a.group(1)) * _VF[a.group(2)], a.span()), ("c_a", float(a.group(3)) * _CF[a.group(4)], (a.end(), a.end()))]
    if ft in _BASE_N and fa in _ACID_H:
        out.append(("ratio", _ACID_H[fa] / _BASE_N[ft], (t.end(), t.end())))
    elif ft in _ACID_H and fa in _BASE_N:
        out.append(("ratio", _BASE_N[fa] / _ACID_H[ft], (t.end(), t.end())))
    return out


def equivalence_volume(c_t, V_a, c_a, ratio):
    _pos(c_t=c_t, V_a=V_a, c_a=c_a, ratio=ratio)
    V = ratio * c_a * V_a / c_t
    return {"V_eq": V, "V_eq_mL": V * 1e6}


_proc("titration_equivalence_volume", name="Titration: equivalence-point volume", topic="acids and bases / titrations",
      level="beginner", purpose="The volume of titrant needed to reach the equivalence point of a neutralization.",
      inputs=[_inp("c_t", "molar_concentration", "titrant concentration", symbols=["c_t"], label_only=True),
              _inp("V_a", "volume", "volume of the analyte", symbols=["V_a"], label_only=True),
              _inp("c_a", "molar_concentration", "analyte concentration", symbols=["c_a"], label_only=True),
              _inp("ratio", "number", "moles of titrant per mole of analyte", symbols=["ratio"], label_only=True)],
      outputs=[_out("V_eq", "volume", "titrant volume at equivalence"), _out("V_eq_mL", "number", "in mL")],
      primary="V_eq", run=equivalence_volume,
      verify=lambda i, o: _rel(o["V_eq"] * i["c_t"], i["ratio"] * i["c_a"] * i["V_a"]),
      reference=lambda c_t, V_a, c_a, ratio: {"V_eq": _brentq(lambda v: c_t * v - ratio * c_a * V_a, 0.0, 1e9)},
      wrong=lambda c_t, V_a, c_a, ratio: {"V_eq": c_a * V_a / (ratio * c_t)},
      rand=lambda rng: {"c_t": _u_(rng, 1.0, 2000.0), "V_a": _u_(rng, 1e-6, 1e-3), "c_a": _u_(rng, 1.0, 2000.0),
                        "ratio": rng.choice([0.5, 2.0, 3.0, 1.0 / 3.0])},
      kw=[r"neutrali[sz]|equivalence point|titrat"], nokw=[r"\bph\b"], text=_texts(_neut_text),
      need=lambda t, qs: not qs,
      latex=r"V_{eq} = \frac{\nu\, c_a V_a}{c_t}", method="moles of titrant = ratio x moles of analyte",
      conditions=["complete neutralization", "the acid's protons and base's hydroxides from the formulas"],
      templates=["What volume of 0.100 M NaOH is needed to neutralize 25.0 mL of 0.150 M HCl?"],
      examples=[{"c_t": 100.0, "V_a": 25e-6, "c_a": 150.0, "ratio": 1.0}, {"c_t": 100.0, "V_a": 2e-5, "c_a": 100.0, "ratio": 2.0}],
      questions=["What volume of 0.100 M NaOH is needed to neutralize 25.0 mL of 0.150 M HCl?",
                 "What volume of 0.100 M NaOH is needed to neutralize 20.0 mL of 0.100 M H2SO4?",
                 "What volume of 0.200 M HCl neutralizes 30.0 mL of 0.100 M Ba(OH)2?"])


def _titr_text(q, spans):
    a = re.search(_VOLRX + r"\s+of\s+(?:a\s+)?" + _CONCRX + r"\s+(?:of\s+)?(" + _FTOKEN + r")", q)
    t = re.search(r"\b(?:with|by)\s+(?:a\s+)?" + _CONCRX + r"\s+(?:of\s+)?(" + _FTOKEN + r")", q)
    v = re.search(r"\b(?:adding|addition of|added|after)\s+(?:the\s+addition of\s+)?" + _VOLRX, q)
    if not (a and t):
        return []
    fa, ft = a.group(5).rstrip(".,?"), t.group(3).rstrip(".,?")
    out = []
    if fa in _ACID_H and _ACID_H[fa] == 1 and ft in _STRONG_B and _BASE_N[ft] == 1:
        out = [("V_a", float(a.group(1)) * _VF[a.group(2)], a.span()), ("c_a", float(a.group(3)) * _CF[a.group(4)], (a.end(), a.end())),
               ("c_b", float(t.group(1)) * _CF[t.group(2)], t.span()), ("_acid", fa, (t.end(), t.end()))]
        if v:
            out.append(("V_b", float(v.group(1)) * _VF[v.group(2)], v.span()))
        return out
    return None


def _strong_strong(c_a, V_a, c_b, V_b):
    _pos(c_a=c_a, V_a=V_a, c_b=c_b)
    if V_b < 0:
        raise ValueError("the added volume cannot be negative")
    D = (c_a * V_a - c_b * V_b) / (V_a + V_b) / 1000.0
    h = D / 2 + math.sqrt(D * D / 4 + KW) if D >= 0 else 2 * KW / (-D + math.sqrt(D * D + 4 * KW))
    return {"pH": _ph(h)}


def _post_strong(got, q):
    acid = got.pop("_acid", None)
    if acid is not None and acid not in _STRONG_A:
        return {"_out_of_scope": "a weak acid titration needs Ka (use the weak-acid titration form)"}
    return got


_proc("titration_ph_strong_acid_strong_base", name="pH during a strong acid - strong base titration",
      topic="acids and bases / titrations", level="intermediate",
      purpose="The pH after adding a volume of strong base to a strong monoprotic acid (exact, any point of the titration).",
      inputs=[_inp("c_a", "molar_concentration", "acid concentration", symbols=["c_a"], label_only=True),
              _inp("V_a", "volume", "acid volume", symbols=["V_a"], label_only=True),
              _inp("c_b", "molar_concentration", "base concentration", symbols=["c_b"], label_only=True),
              _inp("V_b", "volume", "volume of base added", symbols=["V_b"], label_only=True)],
      outputs=[_out("pH", "number", "pH")],
      primary="pH", run=_strong_strong,
      verify=lambda i, o: (lambda h: abs(h - KW / h - (i["c_a"] * i["V_a"] - i["c_b"] * i["V_b"]) / (i["V_a"] + i["V_b"]) / 1000.0)
                           / (h + KW / h))(10.0 ** -o["pH"]),
      reference=lambda c_a, V_a, c_b, V_b: (lambda D: {"pH": _ph(_polish(lambda h: h - KW / h - D,
                                                                          _ref_quad_root([1.0, -D, -KW])))})(
          (c_a * V_a - c_b * V_b) / (V_a + V_b) / 1000.0),
      wrong=lambda c_a, V_a, c_b, V_b: {"pH": _ph(max(abs(c_a * V_a - c_b * V_b) / V_a / 1000.0, 1e-7))
                                        if c_a * V_a >= c_b * V_b else 14 - _ph(abs(c_a * V_a - c_b * V_b) / V_a / 1000.0)},
      rand=lambda rng: (lambda Va, ca, cb: {"c_a": ca, "V_a": Va, "c_b": cb, "V_b": Va * ca / cb * rng.uniform(0.0, 2.0)})(
          _u_(rng, 1e-5, 1e-3), _u_(rng, 10.0, 1000.0), _u_(rng, 10.0, 1000.0)),
      kw=[r"titrat", r"\bph\b"], nokw=[r"\bk_?a|\bpka"], text=_texts(_titr_text), post=_post_strong,
      need=lambda t, qs: not qs,
      latex=r"[\mathrm{H_3O^+}] - \frac{K_w}{[\mathrm{H_3O^+}]} = \frac{c_a V_a - c_b V_b}{V_a + V_b}",
      method="excess strong acid or base after mixing, with water's ionization (exact at the equivalence point too)",
      conditions=["strong monoprotic acid titrated with a strong monohydroxide base", "25 °C"],
      templates=["25.0 mL of 0.100 M HCl is titrated with 0.100 M NaOH. What is the pH after adding 10.0 mL of base?"],
      examples=[{"c_a": 100.0, "V_a": 25e-6, "c_b": 100.0, "V_b": 10e-6}, {"c_a": 100.0, "V_a": 25e-6, "c_b": 100.0, "V_b": 25e-6}],
      questions=["25.0 mL of 0.100 M HCl is titrated with 0.100 M NaOH. What is the pH after adding 10.0 mL of base?",
                 "50.0 mL of 0.200 M HNO3 is titrated with 0.100 M KOH. Find the pH after adding 150 mL of base.",
                 "25.0 mL of 0.100 M HCl is titrated with 0.100 M NaOH. What is the pH?"])


def _titr_weak_text(q, spans):
    a = re.search(_VOLRX + r"\s+of\s+(?:a\s+)?" + _CONCRX + r"\s+(?:of\s+)?(?:weak acid|(" + _FTOKEN + r"))", q)
    t = re.search(r"\b(?:with|by)\s+(?:a\s+)?" + _CONCRX + r"\s+(?:of\s+)?(" + _FTOKEN + r")", q)
    v = re.search(r"\b(?:adding|addition of|added|after)\s+(?:the\s+addition of\s+)?" + _VOLRX, q)
    if not (a and t):
        return []
    fa, ft = (a.group(5) or "").rstrip(".,?"), t.group(3).rstrip(".,?")
    if (fa and fa not in _WEAK_A) or ft not in _STRONG_B or _BASE_N[ft] != 1:
        return None
    out = [("V_a", float(a.group(1)) * _VF[a.group(2)], a.span()), ("c_a", float(a.group(3)) * _CF[a.group(4)], (a.end(), a.end())),
           ("c_b", float(t.group(1)) * _CF[t.group(2)], t.span())]
    if v:
        out.append(("V_b", float(v.group(1)) * _VF[v.group(2)], v.span()))
    return out


def _weak_titr(Ka, c_a, V_a, c_b, V_b):
    _pos(Ka=Ka, c_a=c_a, V_a=V_a, c_b=c_b)
    if V_b < 0:
        raise ValueError("the added volume cannot be negative")
    Vt = V_a + V_b
    C, Na = c_a * V_a / Vt / 1000.0, c_b * V_b / Vt / 1000.0
    f = lambda u: (lambda h: h + Na - KW / h - Ka * C / (Ka + h))(math.exp(u))
    return {"pH": _ph(math.exp(_bisect(f, math.log(1e-20), math.log(10.0))))}


def _ref_weak_titr(Ka, c_a, V_a, c_b, V_b):
    Vt = V_a + V_b
    C, Na = c_a * V_a / Vt / 1000.0, c_b * V_b / Vt / 1000.0
    h0 = _ref_quad_root([1.0, Ka + Na, Na * Ka - KW - Ka * C, -KW * Ka])
    return {"pH": _ph(_polish(lambda h: h + Na - KW / h - Ka * C / (Ka + h), h0))}


_proc("titration_ph_weak_acid_strong_base", name="pH during a weak acid - strong base titration",
      topic="acids and bases / titrations", level="advanced",
      purpose="The pH after adding strong base to a weak monoprotic acid, at any point (exact charge balance).",
      inputs=[_KA(), _inp("c_a", "molar_concentration", "acid concentration", symbols=["c_a"], label_only=True),
              _inp("V_a", "volume", "acid volume", symbols=["V_a"], label_only=True),
              _inp("c_b", "molar_concentration", "base concentration", symbols=["c_b"], label_only=True),
              _inp("V_b", "volume", "volume of base added", symbols=["V_b"], label_only=True)],
      outputs=[_out("pH", "number", "pH")],
      primary="pH", run=_weak_titr,
      verify=lambda i, o: abs(o["pH"] - _weak_titr(**i)["pH"]),
      reference=_ref_weak_titr,
      wrong=lambda Ka, c_a, V_a, c_b, V_b: _weak_titr(Ka, c_a, V_a + V_b, c_b, V_b),
      rand=lambda rng: (lambda Va, ca, cb: {"Ka": _u_(rng, 1e-10, 1e-2), "c_a": ca, "V_a": Va, "c_b": cb,
                                            "V_b": Va * ca / cb * rng.uniform(0.05, 1.8)})(
          _u_(rng, 1e-5, 1e-3), _u_(rng, 10.0, 1000.0), _u_(rng, 10.0, 1000.0)),
      kw=[r"titrat", r"\bph\b", _KA_GIVEN], text=_texts(_titr_weak_text, _pka_text("Ka", "ka")),
      need=lambda t, qs: len(qs) <= 1,
      latex=r"[\mathrm{H_3O^+}] + [\mathrm{Na^+}] = \frac{K_w}{[\mathrm{H_3O^+}]} + \frac{K_a C}{K_a + [\mathrm{H_3O^+}]}",
      method="exact charge balance after mixing, solved by bisection (before, at and after equivalence)",
      conditions=["weak monoprotic acid, strong monohydroxide base", "25 °C"],
      templates=["25.0 mL of 0.100 M CH3COOH (Ka = 1.8e-5) is titrated with 0.100 M NaOH. What is the pH after adding 12.5 mL of base?"],
      examples=[{"Ka": 1.8e-5, "c_a": 100.0, "V_a": 25e-6, "c_b": 100.0, "V_b": 12.5e-6},
                {"Ka": 1.8e-5, "c_a": 100.0, "V_a": 25e-6, "c_b": 100.0, "V_b": 25e-6}],
      questions=["25.0 mL of 0.100 M CH3COOH (Ka = 1.8e-5) is titrated with 0.100 M NaOH. What is the pH after adding "
                 "12.5 mL of base?",
                 "25.0 mL of 0.100 M CH3COOH (Ka = 1.8e-5) is titrated with 0.100 M NaOH. What is the pH after adding "
                 "25.0 mL of base?"])


def ph_diprotic(Ka1, Ka2, c):
    _pos(Ka1=Ka1, Ka2=Ka2, c=c)
    C = c / 1000.0
    f = lambda u: (lambda h: h - C * (Ka1 * h + 2 * Ka1 * Ka2) / (h * h + Ka1 * h + Ka1 * Ka2) - KW / h)(math.exp(u))
    h = math.exp(_bisect(f, math.log(1e-7) - 1e-9, math.log(2 * C + 1e-6)))
    return {"pH": _ph(h)}


def _ref_diprotic(Ka1, Ka2, c):
    C = c / 1000.0
    h0 = _ref_quad_root([1.0, Ka1, Ka1 * Ka2 - C * Ka1 - KW, -(2 * C * Ka1 * Ka2 + KW * Ka1), -KW * Ka1 * Ka2])
    return {"pH": _ph(_polish(lambda h: h - C * (Ka1 * h + 2 * Ka1 * Ka2) / (h * h + Ka1 * h + Ka1 * Ka2) - KW / h, h0))}


_proc("ph_diprotic_acid", name="pH of a diprotic acid", topic="acids and bases / polyprotic acids", level="expert",
      purpose="The pH of a diprotic acid H2A from both dissociation constants, solved exactly.",
      inputs=[_inp("Ka1", "number", "first dissociation constant", symbols=["Ka1", "K_a1"]),
              _inp("Ka2", "number", "second dissociation constant", symbols=["Ka2", "K_a2"]),
              _inp("c", "molar_concentration", "acid concentration")],
      outputs=[_out("pH", "number", "pH")],
      primary="pH", run=ph_diprotic,
      verify=lambda i, o: (lambda h, C: abs(h - C * (i["Ka1"] * h + 2 * i["Ka1"] * i["Ka2"]) / (h * h + i["Ka1"] * h + i["Ka1"] * i["Ka2"])
                                             - KW / h) / h)(10.0 ** -o["pH"], i["c"] / 1000.0),
      reference=_ref_diprotic,
      wrong=lambda Ka1, Ka2, c: ph_weak_acid(Ka1 + Ka2, c),
      rand=lambda rng: (lambda k1: {"Ka1": k1, "Ka2": k1 * _u_(rng, 1e-6, 0.5), "c": 1000.0 * _u_(rng, 1e-4, 1.0)})(
          _u_(rng, 1e-8, 1e-1)),
      kw=[r"\bka1\b|k_a1", r"\bka2\b|k_a2", _ASK_PH], need=_has("molar_concentration", exactly=True),
      latex=r"[\mathrm{H^+}] = C\frac{K_1[\mathrm{H^+}] + 2K_1K_2}{[\mathrm{H^+}]^2 + K_1[\mathrm{H^+}] + K_1K_2} + \frac{K_w}{[\mathrm{H^+}]}",
      method="exact charge balance of H2A, solved by bisection", conditions=["25 °C"],
      templates=["What is the pH of 0.10 M H2CO3 (Ka1 = 4.3e-7, Ka2 = 4.8e-11)?"],
      examples=[{"Ka1": 4.3e-7, "Ka2": 4.8e-11, "c": 100.0}, {"Ka1": 5.9e-2, "Ka2": 6.4e-5, "c": 100.0}],
      questions=["What is the pH of 0.10 M H2CO3 (Ka1 = 4.3e-7, Ka2 = 4.8e-11)?",
                 "Find the pH of 0.10 M oxalic acid with Ka1 = 5.9e-2 and Ka2 = 6.4e-5."])


def ph_salt_weak_acid(Ka, c):
    _pos(Ka=Ka, c=c)
    r = ph_weak_base(KW / Ka, c)
    return {"pH": r["pH"], "Kb": KW / Ka}


_proc("ph_salt_of_weak_acid", name="pH of a salt of a weak acid", topic="acids and bases / salt hydrolysis",
      level="intermediate",
      purpose="The pH of a solution of a salt of a weak acid and a strong base (e.g. sodium acetate) from the acid's Ka.",
      inputs=[_KA(), _inp("c", "molar_concentration", "salt concentration")],
      outputs=[_out("pH", "number", "pH"), _out("Kb", "number", "Kb of the anion (Kw/Ka)")],
      primary="pH", run=ph_salt_weak_acid,
      verify=lambda i, o: (lambda x, C, Kb: abs(x - Kb * C / (Kb + x) - KW / x) / x)(
          10.0 ** -(14.0 - o["pH"]), i["c"] / 1000.0, KW / i["Ka"]),
      reference=lambda Ka, c: {"pH": 14.0 - _ph(_ref_weak(KW / Ka, c / 1000.0))},
      wrong=lambda Ka, c: ph_weak_acid(Ka, c),
      rand=lambda rng: {"Ka": _u_(rng, 1e-11, 1e-2), "c": 1000.0 * _u_(rng, 1e-4, 1.0)},
      kw=[_ASK_PH, _KA_GIVEN, r"\b(?:salt|sodium|potassium|lithium|conjugate base)\b"],
      nokw=[r"buffer|titrat|\badd|\bmix|ka1|ammonium"], text=_texts(_pka_text("Ka", "ka")),
      need=_has("molar_concentration", exactly=True),
      latex=r"K_b = \frac{K_w}{K_a}", method="the anion as a weak base with Kb = Kw/Ka, exact charge balance",
      conditions=["salt of a weak monoprotic acid and a strong base", "25 °C"],
      templates=["What is the pH of 0.10 M sodium acetate (Ka of acetic acid = 1.8e-5)?"],
      examples=[{"Ka": 1.8e-5, "c": 100.0}],
      questions=["What is the pH of 0.10 M sodium acetate (Ka of acetic acid = 1.8e-5)?",
                 "Find the pH of a 0.20 M solution of the salt NaF if Ka for HF is 6.8e-4."])

# ====================================================================================================================
# KINETICS
# ====================================================================================================================
def _ode(rate, A0, t):
    from scipy.integrate import solve_ivp
    s = solve_ivp(lambda _t, y: [-rate(y[0])], (0.0, t), [A0], method="DOP853", rtol=1e-13, atol=A0 * 1e-16)
    return float(s.y[0][-1])


_A0 = lambda: _inp("A0", "molar_concentration", "initial concentration",
                   labels=[r"\[a\]_?0", r"initial concentration", r"\bfrom", r"initially", r"starts? at"],
                   symbols=["A0", "[A]0", "[A]₀", "A_0"], role="initial")
_T_RX = lambda: _inp("t", "time", "elapsed time")


def zero_order_conc(A0, k, t):
    _pos(A0=A0, k=k, t=t)
    A = A0 - k * t
    if A < 0:
        raise ValueError("the reactant is used up before this time (A0 - k t < 0)")
    return {"A": A, "A_molL": A / 1000.0}


_proc("zero_order_concentration", name="Zero-order integrated rate law", topic="kinetics / integrated rate laws",
      level="intermediate", purpose="The concentration of a reactant after a time in a zero-order reaction.",
      inputs=[_A0(), _inp("k", "zero_order_rate_constant", "zero-order rate constant", symbols=["k"]), _T_RX()],
      outputs=[_out("A", "molar_concentration", "concentration at time t"), _out("A_molL", "number", "in mol/L")],
      primary="A", run=zero_order_conc, verify=lambda i, o: _rel(o["A"] + i["k"] * i["t"], i["A0"]),
      reference=lambda A0, k, t: {"A": _ode(lambda a: k, A0, t)},
      wrong=lambda A0, k, t: {"A": A0 * math.exp(-k * t / A0)},
      rand=lambda rng: (lambda A0, k: {"A0": A0, "k": k, "t": A0 / k * rng.uniform(0.05, 0.95)})(
          _u_(rng, 1.0, 2000.0), _u_(rng, 1e-4, 10.0)),
      kw=[r"zero[- ]order", ASK + r"[^.?]*\b(?:concentration|\[a\])"], nokw=[r"half[- ]life"],
      need=_has("molar_concentration", "zero_order_rate_constant", "time", exactly=True),
      latex=r"[A] = [A]_0 - k t", method="[A] = [A]0 - k t", conditions=["[A]0, k, t > 0", "[A] stays >= 0"],
      templates=["A zero-order reaction has k = 0.0020 M/s. If [A]0 = 0.50 M, what is the concentration after 100 s?"],
      examples=[{"A0": 500.0, "k": 2.0, "t": 100.0}],
      questions=["A zero-order reaction has k = 0.0020 M/s. If [A]0 = 0.50 M, what is the concentration after 100 s?",
                 "A zero-order reaction has k = 0.0020 M/s. If [A]0 = 0.50 M, what is the concentration after 1000 s?"])


def first_order_conc(A0, k, t):
    _pos(A0=A0, k=k, t=t)
    A = A0 * math.exp(-k * t)
    return {"A": A, "A_molL": A / 1000.0, "fraction_left": A / A0}


_proc("first_order_concentration", name="First-order integrated rate law", topic="kinetics / integrated rate laws",
      level="intermediate", purpose="The concentration of a reactant after a time in a first-order reaction.",
      inputs=[_A0(), _inp("k", "first_order_rate_constant", "first-order rate constant", symbols=["k"]), _T_RX()],
      outputs=[_out("A", "molar_concentration", "concentration at time t"), _out("A_molL", "number", "in mol/L"),
               _out("fraction_left", "number", "fraction remaining")],
      primary="A", run=first_order_conc, verify=lambda i, o: abs(math.log(o["A"] / i["A0"]) + i["k"] * i["t"]) if o["A"] > 0 else 1.0,
      reference=lambda A0, k, t: {"A": _ode(lambda a: k * a, A0, t)},
      wrong=lambda A0, k, t: {"A": A0 * 10.0 ** (-k * t)},
      rand=lambda rng: (lambda k: {"A0": _u_(rng, 1.0, 2000.0), "k": k, "t": rng.uniform(0.05, 6.0) / k})(_u_(rng, 1e-6, 10.0)),
      kw=[r"first[- ]order", ASK + r"[^.?]*\b(?:concentration|\[a\]|how much (?:remains|is left))"],
      nokw=[r"half[- ]life", r"how long"],
      need=_has("molar_concentration", "per_time", "time", exactly=True),
      latex=r"[A] = [A]_0\, e^{-kt}", method="[A] = [A]0 exp(-k t)", conditions=["[A]0, k, t > 0"],
      templates=["A first-order reaction has k = 0.0150 s^-1. If [A]0 = 0.800 M, what is the concentration after 60.0 s?"],
      examples=[{"A0": 800.0, "k": 0.015, "t": 60.0}],
      questions=["A first-order reaction has k = 0.0150 s^-1. If [A]0 = 0.800 M, what is the concentration after 60.0 s?",
                 "For a first-order decomposition with k = 2.5e-3 min^-1 and an initial concentration of 0.20 M, find the "
                 "concentration after 2.0 h."])


def second_order_conc(A0, k, t):
    _pos(A0=A0, k=k, t=t)
    A = 1.0 / (1.0 / A0 + k * t)
    return {"A": A, "A_molL": A / 1000.0}


_proc("second_order_concentration", name="Second-order integrated rate law", topic="kinetics / integrated rate laws",
      level="intermediate", purpose="The concentration of a reactant after a time in a second-order reaction (rate = k[A]^2).",
      inputs=[_A0(), _inp("k", "second_order_rate_constant", "second-order rate constant", symbols=["k"]), _T_RX()],
      outputs=[_out("A", "molar_concentration", "concentration at time t"), _out("A_molL", "number", "in mol/L")],
      primary="A", run=second_order_conc, verify=lambda i, o: _rel(1.0 / o["A"] - i["k"] * i["t"], 1.0 / i["A0"]),
      reference=lambda A0, k, t: {"A": _ode(lambda a: k * a * a, A0, t)},
      wrong=lambda A0, k, t: {"A": A0 / (1.0 + k * t)},
      rand=lambda rng: (lambda A0, k: {"A0": A0, "k": k, "t": rng.uniform(0.05, 8.0) / (k * A0)})(
          _u_(rng, 1.0, 2000.0), _u_(rng, 1e-7, 1.0)),
      kw=[r"second[- ]order", ASK + r"[^.?]*\b(?:concentration|\[a\])"], nokw=[r"half[- ]life"],
      need=_has("molar_concentration", "second_order_rate_constant", "time", exactly=True),
      latex=r"\frac{1}{[A]} = \frac{1}{[A]_0} + k t", method="1/[A] = 1/[A]0 + k t", conditions=["[A]0, k, t > 0"],
      templates=["A second-order reaction has k = 0.50 M^-1 s^-1. If [A]0 = 0.20 M, what is the concentration after 30 s?"],
      examples=[{"A0": 200.0, "k": 5e-4, "t": 30.0}],
      questions=["A second-order reaction has k = 0.50 M^-1 s^-1. If [A]0 = 0.20 M, what is the concentration after 30 s?"])


def first_order_half_life(k):
    _pos(k=k)
    return {"t_half": math.log(2.0) / k}


def _ref_half_life(k):
    from scipy.integrate import solve_ivp
    s = solve_ivp(lambda _t, y: [-k * y[0]], (0.0, 2.0 / k), [1.0], method="DOP853", rtol=1e-13, atol=1e-16, dense_output=True)
    return {"t_half": _brentq(lambda t: s.sol(t)[0] - 0.5, 0.0, 2.0 / k)}


_proc("first_order_half_life", name="Half-life of a first-order reaction", topic="kinetics / half-life",
      level="beginner", purpose="The half-life of a first-order reaction from its rate constant.",
      inputs=[_inp("k", "first_order_rate_constant", "first-order rate constant", symbols=["k"])],
      outputs=[_out("t_half", "time", "half-life")],
      primary="t_half", run=first_order_half_life, verify=lambda i, o: _rel(o["t_half"] * i["k"], math.log(2.0)),
      reference=_ref_half_life, wrong=lambda k: {"t_half": 0.5 / k},
      rand=lambda rng: {"k": _u_(rng, 1e-8, 100.0)},
      kw=[r"half[- ]life", r"first[- ]order|rate constant"], need=_has("per_time", exactly=True),
      latex=r"t_{1/2} = \frac{\ln 2}{k}", method="t1/2 = ln 2 / k", conditions=["k > 0"],
      templates=["What is the half-life of a first-order reaction with k = 0.0250 s^-1?"],
      examples=[{"k": 0.025}],
      questions=["What is the half-life of a first-order reaction with k = 0.0250 s^-1?",
                 "Find the half-life of a first-order process whose rate constant is 1.2e-4 min^-1."])


def first_order_time(A0, A, k):
    _pos(A0=A0, A=A, k=k)
    if A >= A0:
        raise ValueError("the final concentration must be below the initial one")
    return {"t": math.log(A0 / A) / k}


_proc("first_order_time", name="Time to reach a concentration (first order)", topic="kinetics / integrated rate laws",
      level="intermediate", purpose="How long a first-order reaction takes to bring a reactant from one concentration to another.",
      inputs=[_A0(), _inp("A", "molar_concentration", "final concentration",
                          labels=[r"\bto", r"final concentration", r"falls? to", r"drops? to", r"decreases? to"],
                          symbols=["A", "[A]"], role="final"),
              _inp("k", "first_order_rate_constant", "first-order rate constant", symbols=["k"])],
      outputs=[_out("t", "time", "time needed")],
      primary="t", run=first_order_time, verify=lambda i, o: _rel(o["t"] * i["k"], math.log(i["A0"] / i["A"])),
      reference=lambda A0, A, k: {"t": _brentq(lambda t: _ode(lambda a: k * a, A0, t) - A, 0.0, math.log(A0 / A) / k * 2)},
      wrong=lambda A0, A, k: {"t": math.log10(A0 / A) / k},
      rand=lambda rng: (lambda A0: {"A0": A0, "A": A0 * rng.uniform(0.01, 0.95), "k": _u_(rng, 1e-5, 10.0)})(_u_(rng, 1.0, 2000.0)),
      kw=[r"first[- ]order", r"how long|what time|time (?:needed|required|taken)|how much time"],
      need=_has("molar_concentration", "molar_concentration", "per_time", exactly=True),
      latex=r"t = \frac{1}{k}\ln\frac{[A]_0}{[A]}", method="t = ln([A]0/[A]) / k", conditions=["0 < [A] < [A]0", "k > 0"],
      templates=["A first-order reaction has k = 0.0300 s^-1. How long does it take for the concentration to fall from 0.500 M to 0.100 M?"],
      examples=[{"A0": 500.0, "A": 100.0, "k": 0.03}],
      questions=["A first-order reaction has k = 0.0300 s^-1. How long does it take for the concentration to fall from "
                 "0.500 M to 0.100 M?"])


_K_NUM = lambda sym, meaning, syms, role=None: _inp(sym, "number", meaning, symbols=syms, label_only=False,
                                                     dims=("per_time", "second_order_rate_constant", "zero_order_rate_constant"),
                                                     **({"role": role} if role else {}))


def arrhenius_k2(k1, T1, T2, Ea):
    _pos(k1=k1, T1=T1, T2=T2, Ea=Ea)
    ln = math.log(k1) - Ea / R * (1 / T2 - 1 / T1)
    return {"k2": math.exp(ln), "ratio": math.exp(ln) / k1}


def _ref_arrh(k1, T1, T2, Ea):
    from scipy.integrate import quad
    import scipy.constants as sc
    integral, _e = quad(lambda T: Ea / (sc.R * T * T), T1, T2, epsabs=0, epsrel=1e-13, limit=200)
    return {"k2": k1 * math.exp(integral)}


_EA = lambda: _inp("Ea", "molar_energy", "activation energy", symbols=["Ea", "E_a"],
                   labels=[r"activation energy(?: of| is)?", r"\bea\b"])

_proc("arrhenius_k2", name="Arrhenius equation: rate constant at another temperature", topic="kinetics / temperature dependence",
      level="intermediate",
      purpose="The rate constant at a second temperature from the rate constant at a first one and the activation energy.",
      inputs=[_K_NUM("k1", "rate constant at T1 (any units)", ["k1", "k", "k_1"]),
              _inp("T1", "temperature", "first temperature", labels=L_T1, symbols=["T1"], role="initial"),
              _inp("T2", "temperature", "second temperature", labels=L_T2, symbols=["T2"], role="final"), _EA()],
      outputs=[_out("k2", "number", "rate constant at T2 (units of k1)"), _out("ratio", "number", "k2 / k1")],
      primary="k2", run=arrhenius_k2,
      verify=lambda i, o: abs(math.log(o["k2"] / i["k1"]) + i["Ea"] / R * (1 / i["T2"] - 1 / i["T1"])) if o["k2"] > 0 else 1.0,
      reference=_ref_arrh, wrong=lambda k1, T1, T2, Ea: {"k2": k1 * math.exp(Ea / R * (1 / T2 - 1 / T1))},
      rand=lambda rng: {"k1": _u_(rng, 1e-6, 1e3), "T1": rng.uniform(250.0, 800.0), "T2": rng.uniform(250.0, 800.0),
                        "Ea": rng.uniform(1e4, 2e5)},
      kw=[r"activation energy|\bea\b", r"rate constant|(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])"], nokw=[r"\bpre-?exponential|frequency factor|\bk2\s*="],
      need=lambda t, qs: _dims(qs).get("temperature") == 2 and _dims(qs).get("molar_energy") == 1 and len(qs) == 4,
      latex=r"\ln\frac{k_2}{k_1} = -\frac{E_a}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)",
      method="two-point Arrhenius equation", conditions=["k1, T1, T2, Ea > 0", "Ea independent of T"],
      templates=["A reaction has k = 1.2e-3 s^-1 at 300 K and Ea = 75.0 kJ/mol. What is k at 350 K?"],
      examples=[{"k1": 1.2e-3, "T1": 300.0, "T2": 350.0, "Ea": 75000.0}],
      questions=["A reaction has k = 1.2e-3 s^-1 at 300 K and Ea = 75.0 kJ/mol. What is k at 350 K?",
                 "k1 = 0.050 M^-1 s^-1, T1 = 298 K, T2 = 318 K and the activation energy is 52 kJ/mol. Find the rate constant at T2."])


def activation_energy(k1, T1, k2, T2):
    _pos(k1=k1, T1=T1, k2=k2, T2=T2)
    if T1 == T2:
        raise ValueError("the two temperatures must differ")
    Ea = R * math.log(k2 / k1) / (1 / T1 - 1 / T2)
    return {"Ea": Ea, "Ea_kJ": Ea / 1000.0}


def _ref_ea(k1, T1, k2, T2):
    import numpy as np
    import scipy.constants as sc
    slope = np.polyfit([1 / T1, 1 / T2], [math.log(k1), math.log(k2)], 1)[0]
    return {"Ea": -slope * sc.R}


_proc("activation_energy", name="Activation energy from two rate constants", topic="kinetics / temperature dependence",
      level="advanced", purpose="The Arrhenius activation energy from rate constants measured at two temperatures.",
      inputs=[_inp("k1", "number", "rate constant at T1", symbols=["k1", "k_1"], label_only=True,
                   dims=("per_time", "second_order_rate_constant", "zero_order_rate_constant")),
              _inp("T1", "temperature", "first temperature", symbols=["T1"], label_only=True),
              _inp("k2", "number", "rate constant at T2", symbols=["k2", "k_2"], label_only=True,
                   dims=("per_time", "second_order_rate_constant", "zero_order_rate_constant")),
              _inp("T2", "temperature", "second temperature", symbols=["T2"], label_only=True)],
      outputs=[_out("Ea", "molar_energy", "activation energy"), _out("Ea_kJ", "number", "in kJ/mol")],
      primary="Ea", run=activation_energy,
      verify=lambda i, o: abs(math.log(i["k2"] / i["k1"]) - o["Ea"] / R * (1 / i["T1"] - 1 / i["T2"])),
      reference=_ref_ea, wrong=lambda k1, T1, k2, T2: {"Ea": R * math.log10(k2 / k1) / (1 / T1 - 1 / T2)},
      rand=lambda rng: (lambda T1, T2, Ea, k1: {"k1": k1, "T1": T1, "T2": T2,
                                                 "k2": k1 * math.exp(-Ea / R * (1 / T2 - 1 / T1))})(
          rng.uniform(250.0, 600.0), rng.uniform(610.0, 900.0), rng.uniform(1e4, 2e5), _u_(rng, 1e-6, 1e3)),
      kw=[r"activation energy|\bea\b"],
      need=lambda t, qs: all(s in t for s in ("⟦k1⟧", "⟦k2⟧", "⟦T1⟧", "⟦T2⟧")),
      latex=r"E_a = \frac{R\ln(k_2/k_1)}{1/T_1 - 1/T_2}", method="two-point Arrhenius equation solved for Ea",
      conditions=["k1, k2 > 0", "T1 != T2"],
      templates=["k1 = 1.0e-3 s^-1 at T1 = 300 K and k2 = 5.0e-3 s^-1 at T2 = 320 K. What is the activation energy?"],
      examples=[{"k1": 1.0e-3, "T1": 300.0, "k2": 5.0e-3, "T2": 320.0}],
      questions=["k1 = 1.0e-3 s^-1 at T1 = 300 K and k2 = 5.0e-3 s^-1 at T2 = 320 K. What is the activation energy?"])


def arrhenius_k(A, Ea, T):
    _pos(A=A, Ea=Ea, T=T)
    return {"k": A * math.exp(-Ea / (R * T))}


def _ref_ct_k(A, Ea, T):
    import cantera as ct
    return {"k": ct.ArrheniusRate(A, 0.0, Ea * 1000.0)(T)}


_proc("arrhenius_rate_constant", name="Arrhenius equation: rate constant", topic="kinetics / temperature dependence",
      level="intermediate", purpose="The rate constant from the pre-exponential factor, the activation energy and the temperature.",
      inputs=[_inp("A", "number", "pre-exponential factor (units of k)", symbols=["A"],
                   labels=[r"pre-?exponential factor(?: of| is)?", r"frequency factor(?: of| is)?"],
                   dims=("per_time", "second_order_rate_constant", "zero_order_rate_constant")),
              _EA(), _inp("T", "temperature", "temperature")],
      outputs=[_out("k", "number", "rate constant (units of A)")],
      primary="k", run=arrhenius_k, verify=lambda i, o: abs(math.log(o["k"] / i["A"]) + i["Ea"] / (R * i["T"])) if o["k"] > 0 else 1.0,
      reference=_ref_ct_k, wrong=lambda A, Ea, T: {"k": A * math.exp(-Ea / T)},
      rand=lambda rng: {"A": _u_(rng, 1e3, 1e15), "Ea": rng.uniform(1e4, 1.5e5), "T": rng.uniform(250.0, 2000.0)},
      kw=[r"pre-?exponential|frequency factor|arrhenius", r"rate constant|(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])"],
      need=_has("molar_energy", "temperature", exactly=True),
      latex=r"k = A\,e^{-E_a/RT}", method="k = A exp(-Ea/RT)", conditions=["A, Ea, T > 0"],
      templates=["A reaction has a pre-exponential factor of 1.0e13 s^-1 and Ea = 100 kJ/mol. What is the rate constant at 500 K?"],
      examples=[{"A": 1.0e13, "Ea": 1e5, "T": 500.0}],
      questions=["A reaction has a pre-exponential factor of 1.0e13 s^-1 and Ea = 100 kJ/mol. What is the rate constant at 500 K?"])


def reaction_order(c1, r1, c2, r2):
    _pos(c1=c1, r1=r1, c2=c2, r2=r2)
    if c1 == c2:
        raise ValueError("the two concentrations must differ")
    return {"order": math.log(r2 / r1) / math.log(c2 / c1)}


_proc("reaction_order_from_rates", name="Reaction order from initial rates", topic="kinetics / rate laws", level="intermediate",
      purpose="The order with respect to one reactant from two initial-rate experiments (other concentrations fixed).",
      inputs=[_inp("c1", "molar_concentration", "concentration in experiment 1", symbols=["c1", "[A]1"], label_only=True),
              _inp("r1", "reaction_rate", "initial rate in experiment 1", symbols=["r1", "rate1"], label_only=True),
              _inp("c2", "molar_concentration", "concentration in experiment 2", symbols=["c2", "[A]2"], label_only=True),
              _inp("r2", "reaction_rate", "initial rate in experiment 2", symbols=["r2", "rate2"], label_only=True)],
      outputs=[_out("order", "number", "reaction order")],
      primary="order", run=reaction_order,
      verify=lambda i, o: abs(o["order"] * math.log(i["c2"] / i["c1"]) - math.log(i["r2"] / i["r1"])),
      reference=lambda c1, r1, c2, r2: {"order": float(__import__("numpy").polyfit(
          [math.log(c1), math.log(c2)], [math.log(r1), math.log(r2)], 1)[0])},
      wrong=lambda c1, r1, c2, r2: {"order": (r2 / r1) / (c2 / c1)},
      rand=lambda rng: (lambda c1, c2, n, k: {"c1": c1, "c2": c2, "r1": k * (c1 / 1000) ** n * 1000, "r2": k * (c2 / 1000) ** n * 1000})(
          _u_(rng, 1.0, 1000.0), _u_(rng, 1.0, 1000.0) * 1.37, rng.choice([0.5, 1.0, 2.0, 3.0, 1.5]), _u_(rng, 1e-6, 1.0)),
      kw=[r"\border\b"], need=lambda t, qs: all(s in t for s in ("⟦c1⟧", "⟦c2⟧", "⟦r1⟧", "⟦r2⟧")),
      latex=r"n = \frac{\ln(r_2/r_1)}{\ln(c_2/c_1)}", method="ratio of the two rate laws", conditions=["c1 != c2"],
      templates=["c1 = 0.10 M, r1 = 2.0e-3 M/s, c2 = 0.20 M, r2 = 8.0e-3 M/s. What is the order with respect to A?"],
      examples=[{"c1": 100.0, "r1": 2.0, "c2": 200.0, "r2": 8.0}],
      questions=["c1 = 0.10 M, r1 = 2.0e-3 M/s, c2 = 0.20 M, r2 = 8.0e-3 M/s. What is the order with respect to A?"])


def consecutive(A0, k1, k2, t):
    _pos(A0=A0, k1=k1, k2=k2, t=t)
    A = A0 * math.exp(-k1 * t)
    d = k2 - k1
    B = A0 * k1 * t * math.exp(-k1 * t) if d == 0 else A0 * k1 * math.exp(-k1 * t) * (-math.expm1(-d * t)) / d
    return {"B": B, "A": A, "C": A0 - A - B}


def _ref_consecutive(A0, k1, k2, t):
    from scipy.linalg import expm
    import numpy as np
    y = expm(np.array([[-k1, 0.0], [k1, -k2]]) * t) @ np.array([A0, 0.0])
    return {"A": float(y[0]), "B": float(y[1])}


_proc("consecutive_first_order", name="Consecutive first-order reactions (A -> B -> C)", topic="kinetics / complex reactions",
      level="expert", purpose="Concentrations of the reactant and the intermediate of A -> B -> C (both steps first order) at a time.",
      inputs=[_inp("A0", "molar_concentration", "initial concentration of A (B and C start at 0)"),
              _inp("k1", "first_order_rate_constant", "rate constant of A -> B", symbols=["k1", "k_1"], label_only=True),
              _inp("k2", "first_order_rate_constant", "rate constant of B -> C", symbols=["k2", "k_2"], label_only=True), _T_RX()],
      outputs=[_out("B", "molar_concentration", "intermediate concentration"), _out("A", "molar_concentration", "reactant"),
               _out("C", "molar_concentration", "product")],
      primary="B", run=consecutive,
      verify=lambda i, o: (lambda r: _rel(o["B"], r["B"]))(consecutive(**i)),
      reference=_ref_consecutive,
      wrong=lambda A0, k1, k2, t: {"B": A0 * k1 / (k2 - k1) * (math.exp(-k2 * t) - math.exp(-k1 * t)) if k1 != k2 else 0.0},
      rand=lambda rng: (lambda k1: {"A0": _u_(rng, 10.0, 2000.0), "k1": k1, "k2": k1 * _u_(rng, 0.05, 20.0),
                                    "t": rng.uniform(0.1, 4.0) / k1})(_u_(rng, 1e-4, 10.0)),
      kw=[r"intermediate|consecutive|a\s*(?:->|→)\s*b\s*(?:->|→)\s*c"],
      need=lambda t, qs: "⟦k1⟧" in t and "⟦k2⟧" in t,
      latex=r"[B] = \frac{k_1 [A]_0}{k_2 - k_1}\left(e^{-k_1 t} - e^{-k_2 t}\right)",
      method="analytic solution of the linear rate equations (the k1 = k2 limit included)",
      conditions=["both steps first order", "B and C absent at t = 0"],
      templates=["For A -> B -> C with k1 = 0.10 s^-1, k2 = 0.050 s^-1 and [A]0 = 1.0 M, what is the concentration of the "
                 "intermediate B after 10 s?"],
      examples=[{"A0": 1000.0, "k1": 0.1, "k2": 0.05, "t": 10.0}, {"A0": 1000.0, "k1": 0.1, "k2": 0.1, "t": 10.0}],
      questions=["For A -> B -> C with k1 = 0.10 s^-1, k2 = 0.050 s^-1 and [A]0 = 1.0 M, what is the concentration of the "
                 "intermediate B after 10 s?"])

# ====================================================================================================================
# THERMOCHEMISTRY
# ====================================================================================================================
_CT = lambda: [_inp("T1", "temperature", "initial temperature", labels=L_T1, symbols=["T1"], role="initial"),
               _inp("T2", "temperature", "final temperature", labels=L_T2 + [r"rises? to", r"falls? to"], symbols=["T2"],
                    role="final")]


def calorimetry_heat(m, c, T1, T2):
    _pos(m=m, c=c, T1=T1, T2=T2)
    return {"q": m * c * (T2 - T1)}


_proc("calorimetry_heat", name="Heat from a temperature change", topic="thermochemistry / calorimetry", level="beginner",
      purpose="The heat absorbed (positive) or released (negative) by a mass of substance whose temperature changes.",
      inputs=[_inp("m", "mass", "mass"), _inp("c", "specific_heat", "specific heat capacity")] + _CT(),
      outputs=[_out("q", "energy", "heat absorbed (negative: released)")],
      primary="q", run=calorimetry_heat, verify=lambda i, o: _rel(o["q"], i["m"] * i["c"] * (i["T2"] - i["T1"])),
      reference=lambda m, c, T1, T2: {"q": __import__("scipy.integrate").integrate.quad(lambda T: m * c, T1, T2)[0]},
      wrong=lambda m, c, T1, T2: {"q": m * c * T2},
      rand=lambda rng: {"m": _u_(rng, 1e-3, 10.0), "c": _u_(rng, 100.0, 5000.0), "T1": rng.uniform(250.0, 400.0),
                        "T2": rng.uniform(250.0, 400.0)},
      kw=[r"\bheat\b|\benergy\b"], need=_has("mass", "specific_heat", "temperature", "temperature", exactly=True),
      latex=r"q = m c (T_2 - T_1)", method="q = m c dT", conditions=["no phase change"],
      templates=["How much heat is needed to warm 50.0 g of water from 20.0 °C to 80.0 °C (c = 4.184 J/(g·°C))?"],
      examples=[{"m": 0.05, "c": 4184.0, "T1": 293.15, "T2": 353.15}],
      questions=["How much heat is needed to warm 50.0 g of water from 20.0 °C to 80.0 °C (c = 4.184 J/(g·°C))?",
                 "How much heat is released when 200 g of iron (c = 0.449 J/(g·K)) cools from 400 K to 300 K?"])


def calorimetry_enthalpy(m, c, T1, T2, n):
    _pos(m=m, c=c, T1=T1, T2=T2, n=n)
    q = m * c * (T2 - T1)
    return {"dH": -q / n, "dH_kJ": -q / n / 1000.0, "q_solution": q}


_proc("enthalpy_from_calorimetry", name="Reaction enthalpy from coffee-cup calorimetry", topic="thermochemistry / calorimetry",
      level="intermediate",
      purpose="The molar enthalpy of a reaction from the temperature change of the solution it takes place in.",
      inputs=[_inp("m", "mass", "mass of the solution"), _inp("c", "specific_heat", "specific heat of the solution")] + _CT()
      + [_inp("n", "amount", "moles of the reaction (limiting reactant)")],
      outputs=[_out("dH", "molar_energy", "reaction enthalpy"), _out("dH_kJ", "number", "in kJ/mol"),
               _out("q_solution", "energy", "heat taken up by the solution")],
      primary="dH", run=calorimetry_enthalpy, verify=lambda i, o: _rel(-o["dH"] * i["n"], i["m"] * i["c"] * (i["T2"] - i["T1"])),
      reference=lambda m, c, T1, T2, n: {"dH": -__import__("scipy.integrate").integrate.quad(lambda T: m * c, T1, T2)[0] / n},
      wrong=lambda m, c, T1, T2, n: {"dH": m * c * (T2 - T1) / n},
      rand=lambda rng: {"m": _u_(rng, 0.01, 1.0), "c": _u_(rng, 2000.0, 4500.0), "T1": rng.uniform(280.0, 310.0),
                        "T2": rng.uniform(270.0, 330.0), "n": _u_(rng, 1e-3, 1.0)},
      kw=[r"enthalpy|δh|heat of (?:reaction|neutrali[sz]ation|solution)"],
      need=_has("mass", "specific_heat", "temperature", "temperature", "amount", exactly=True),
      latex=r"\Delta H = -\frac{m c (T_2 - T_1)}{n}", method="heat gained by the solution, with the opposite sign, per mole",
      conditions=["no heat lost to the calorimeter or surroundings", "constant pressure"],
      templates=["When 0.0500 mol of HCl reacts in 100 g of solution (c = 4.18 J/(g·°C)), the temperature rises from 22.0 °C "
                 "to 28.8 °C. What is the enthalpy of reaction?"],
      examples=[{"m": 0.1, "c": 4180.0, "T1": 295.15, "T2": 301.95, "n": 0.05}],
      questions=["When 0.0500 mol of HCl reacts in 100 g of solution (c = 4.18 J/(g·°C)), the temperature rises from 22.0 °C "
                 "to 28.8 °C. What is the enthalpy of reaction?"])


def formation_sum(equation, values):
    coefs, species, nr = _as_written_balanced(equation)
    if len(values) != len(species):
        raise ValueError(f"give one value per species in the order written ({', '.join(species)})")
    tot = math.fsum((c if j >= nr else -c) * v for j, (c, v) in enumerate(zip(coefs, values)))
    return {"dH": tot * 1000.0, "dH_kJ": tot}


def _ref_formation(equation, values):
    t = re.sub(r"\((?:s|l|g|aq)\)", "", equation)
    left, right = re.split(r"\s*(?:<=>|⇌|->|→|=)\s*", t)
    co = lambda side: [float(re.match(r"\s*(\d*)", x).group(1) or 1) for x in side.split("+")]
    nu = [-c for c in co(left)] + co(right)
    import numpy as np
    return {"dH": float(np.dot(nu, values)) * 1000.0}


def _formation_text(q):
    eq, span = _equation_in(q)
    if not eq:
        return {}, []
    preset, spans = {"equation": eq}, [span]
    r, p = parse_equation(eq)
    sp = [f for _c, f in r + p]
    found = {}
    for m in re.finditer(r"(?<![\w(])\(?([A-Z][A-Za-z0-9()]*?)\)?(?:\((?:s|l|g|aq)\))?\s*(?:=|:)\s*(-?\d+(?:\.\d*)?)", q):
        if m.group(1) in sp and m.group(1) not in found and not (span[0] <= m.start() < span[1]):
            found[m.group(1)] = float(m.group(2))
            spans.append(m.span())
    if len(found) == len(sp):
        preset["values"] = ", ".join(repr(found[s]) for s in sp)
    return preset, spans


_proc("reaction_enthalpy_from_formation", name="Reaction enthalpy from enthalpies of formation (Hess's law)",
      topic="thermochemistry / Hess's law", level="advanced",
      purpose="The standard reaction enthalpy from the standard enthalpies of formation of every species (balanced equation).",
      inputs=[_inp("equation", "number", "balanced equation", type="text", min_length=4, max_length=300),
              _inp("values", "number", "standard enthalpy of formation of each species in the order written (kJ/mol)",
                   type="vector", min_length=2, max_length=12)],
      outputs=[_out("dH", "molar_energy", "standard reaction enthalpy"), _out("dH_kJ", "number", "in kJ/mol")],
      primary="dH", run=formation_sum,
      verify=lambda i, o: (lambda c, sp, nr: abs(o["dH"] / 1000.0 - math.fsum((x if j >= nr else -x) * v for j, (x, v) in
                                                                               enumerate(zip(c, i["values"]))))
                           / max(1.0, sum(abs(v) for v in i["values"])))(*_as_written_balanced(i["equation"])),
      reference=_ref_formation,
      wrong=lambda equation, values: (lambda c, sp, nr: {"dH": 1000.0 * math.fsum(
          (1.0 if j >= nr else -1.0) * v for j, v in enumerate(values))})(*_as_written_balanced(equation)),
      rand=lambda rng: (lambda eq: {"equation": eq, "values": [round(rng.uniform(-1500, 300), 1) for _ in
                                                               _as_written_balanced(eq)[1]]})(
          rng.choice(["CH4 + 2 O2 -> CO2 + 2 H2O", "C3H8 + 5 O2 -> 3 CO2 + 4 H2O", "2 H2 + O2 -> 2 H2O",
                      "N2 + 3 H2 -> 2 NH3", "CaCO3 -> CaO + CO2", "2 SO2 + O2 -> 2 SO3", "C2H5OH + 3 O2 -> 2 CO2 + 3 H2O",
                      "Fe2O3 + 3 CO -> 2 Fe + 3 CO2", "4 NH3 + 5 O2 -> 4 NO + 6 H2O"])),
      kw=[r"enthalp|δh", r"formation"], text=_formation_text, need=lambda t, qs: not qs,
      latex=r"\Delta H^\circ_{rxn} = \sum \nu_p \Delta H_f^\circ(p) - \sum \nu_r \Delta H_f^\circ(r)",
      method="coefficient-weighted sum of formation enthalpies, products minus reactants",
      conditions=["the equation as written conserves every element", "one value per species, in kJ/mol"],
      templates=["Using enthalpies of formation CH4 = -74.8, O2 = 0, CO2 = -393.5, H2O = -285.8 (kJ/mol), find ΔH° "
                 "for CH4 + 2 O2 -> CO2 + 2 H2O."],
      examples=[{"equation": "CH4 + 2 O2 -> CO2 + 2 H2O", "values": "-74.8, 0, -393.5, -285.8"}],
      questions=["Using enthalpies of formation CH4 = -74.8, O2 = 0, CO2 = -393.5, H2O = -285.8 (kJ/mol), find ΔH° "
                 "for CH4 + 2 O2 -> CO2 + 2 H2O.",
                 "Using enthalpies of formation CH4 = -74.8, O2 = 0, CO2 = -393.5, H2O = -285.8 (kJ/mol), find ΔH° "
                 "for CH4 + O2 -> CO2 + H2O."])


def gibbs(dH, dS, T):
    _finite(dH=dH, dS=dS)
    _pos(T=T)
    dG = dH - T * dS
    return {"dG": dG, "dG_kJ": dG / 1000.0,
            "spontaneity": "spontaneous" if dG < 0 else ("non-spontaneous" if dG > 0 else "at equilibrium")}


_DS = lambda: _inp("dS", "molar_entropy", "entropy change", symbols=["ΔS°", "ΔS", "dS"])

_proc("gibbs_free_energy", name="Gibbs energy change (dG = dH - T dS)", topic="thermochemistry / spontaneity",
      level="beginner", purpose="The Gibbs energy change of a process from its enthalpy and entropy changes, and whether it is spontaneous.",
      inputs=[_DH(), _DS(), _inp("T", "temperature", "temperature")],
      outputs=[_out("dG", "molar_energy", "Gibbs energy change"), _out("dG_kJ", "number", "in kJ/mol"),
               _out("spontaneity", "text", "spontaneous / non-spontaneous")],
      primary="dG", run=gibbs, verify=lambda i, o: abs(o["dG"] - i["dH"] + i["T"] * i["dS"]) / max(1.0, abs(i["dH"]), abs(i["T"] * i["dS"])),
      reference=lambda dH, dS, T: {"dG": float(Fraction(dH) - Fraction(T) * Fraction(dS))},
      wrong=lambda dH, dS, T: {"dG": dH - T * dS / 1000.0},
      rand=lambda rng: {"dH": rng.uniform(-3e5, 3e5), "dS": rng.uniform(-500.0, 500.0), "T": rng.uniform(200.0, 2000.0)},
      kw=[r"δg|gibbs|spontan"], nokw=[r"(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])|equilibrium constant|\bq\b|quotient|\be°|potential|formation|temperature at which|"
                                     r"above what|below what|crossover"],
      need=_has("molar_energy", "molar_entropy", "temperature", exactly=True),
      latex=r"\Delta G = \Delta H - T\Delta S", method="dG = dH - T dS", conditions=["dH and dS taken as constant"],
      templates=["For a reaction ΔH = -92.2 kJ/mol and ΔS = -198.7 J/(mol·K). What is ΔG at 298 K? Is it spontaneous?"],
      examples=[{"dH": -92200.0, "dS": -198.7, "T": 298.0}],
      questions=["For a reaction ΔH = -92.2 kJ/mol and ΔS = -198.7 J/(mol·K). What is ΔG at 298 K? Is it spontaneous?",
                 "ΔH = 178 kJ/mol, ΔS = 161 J/(mol·K). Find ΔG at 1000 K."])


def crossover_t(dH, dS):
    _finite(dH=dH, dS=dS)
    if dS == 0 or dH == 0 or (dH > 0) != (dS > 0):
        raise ValueError("dH and dS must have the same sign for a temperature where dG = 0 to exist")
    return {"T": dH / dS, "T_C": dH / dS - 273.15}


_proc("crossover_temperature", name="Temperature at which dG = 0", topic="thermochemistry / spontaneity", level="intermediate",
      purpose="The temperature at which a reaction changes between spontaneous and non-spontaneous (dG = 0).",
      inputs=[_DH(), _DS()],
      outputs=[_out("T", "temperature", "temperature where dG = 0"), _out("T_C", "number", "in °C")],
      primary="T", run=crossover_t, verify=lambda i, o: _rel(o["T"] * i["dS"], i["dH"]),
      reference=lambda dH, dS: {"T": _brentq(lambda T: dH - T * dS, 0.0, 10 * dH / dS)},
      wrong=lambda dH, dS: {"T": dS / dH * 1e6},
      rand=lambda rng: (lambda s: {"dH": s * rng.uniform(1e3, 3e5), "dS": s * rng.uniform(10.0, 500.0)})(rng.choice([-1, 1])),
      kw=[r"temperature at which|above what temperature|below what temperature|crossover|becomes? spontaneous|"
          r"what temperature"], need=_has("molar_energy", "molar_entropy", exactly=True),
      latex=r"T = \frac{\Delta H}{\Delta S}", method="T = dH / dS", conditions=["dH and dS of the same sign"],
      templates=["For ΔH = 178 kJ/mol and ΔS = 161 J/(mol·K), above what temperature is the reaction spontaneous?"],
      examples=[{"dH": 178000.0, "dS": 161.0}],
      questions=["For ΔH = 178 kJ/mol and ΔS = 161 J/(mol·K), above what temperature is the reaction spontaneous?",
                 "ΔH = -50 kJ/mol and ΔS = 100 J/(mol·K). At what temperature does the reaction become spontaneous?"])


def heat_from_mass(formula, m, dH):
    _pos(m=m)
    _finite(dH=dH)
    q = m / (molar_mass_g(formula) / 1000.0) * dH
    return {"q": q, "heat_released": -q}


_proc("heat_from_reaction_mass", name="Heat of reaction for a given mass", topic="thermochemistry / reaction enthalpy",
      level="intermediate", purpose="The heat of a reaction for a given mass of a reactant, from its molar enthalpy (per mole of that reactant).",
      inputs=[_inp("formula", "number", "formula of the reactant", type="text", min_length=1, max_length=60),
              _inp("m", "mass", "mass of the reactant"), _DH()],
      outputs=[_out("q", "energy", "enthalpy change (negative: heat released)"),
               _out("heat_released", "energy", "heat released")],
      primary="q", run=heat_from_mass, verify=lambda i, o: _rel(o["q"] * _mm_rd(i["formula"]) / 1000.0, i["m"] * i["dH"]),
      reference=lambda formula, m, dH: {"q": m * 1000.0 / _ref_mw(formula) * dH},
      wrong=lambda formula, m, dH: {"q": m * dH},
      rand=lambda rng: {"formula": _rand_formula(rng), "m": _u_(rng, 1e-3, 10.0), "dH": rng.uniform(-3e6, 1e6)},
      kw=[r"\bheat\b|\benergy\b"], nokw=[r"formation|calorimet|specific heat"],
      text=_texts(_tf("formula", r"(?:kg|g|mg|grams|gram)\s+of")), need=_has("mass", "molar_energy", exactly=True),
      latex=r"q = \frac{m}{M}\Delta H", method="moles of reactant times its molar enthalpy",
      conditions=["dH is per mole of the given reactant"],
      templates=["How much heat is released when 10.0 g of CH4 burns? ΔH = -890 kJ/mol."],
      examples=[{"formula": "CH4", "m": 0.01, "dH": -890000.0}],
      questions=["How much heat is released when 10.0 g of CH4 burns? ΔH = -890 kJ/mol."])


def cc_pressure(P1, T1, T2, dH):
    _pos(P1=P1, T1=T1, T2=T2, dH=dH)
    return {"P2": P1 * math.exp(-dH / R * (1 / T2 - 1 / T1))}


def _ref_cc(P1, T1, T2, dH):
    from scipy.integrate import quad
    import scipy.constants as sc
    integral, _e = quad(lambda T: dH / (sc.R * T * T), T1, T2, epsabs=0, epsrel=1e-13, limit=200)
    return {"P2": P1 * math.exp(integral)}


_proc("clausius_clapeyron_pressure", name="Clausius-Clapeyron: vapor pressure at another temperature",
      topic="thermochemistry / phase equilibria", level="advanced",
      purpose="The vapor pressure of a liquid at a second temperature from one known point and the enthalpy of vaporization.",
      inputs=[_inp("P1", "pressure", "vapor pressure at T1", labels=L_P1, symbols=["P1"], role="initial"),
              _inp("T1", "temperature", "first temperature", labels=L_T1, symbols=["T1"], role="initial"),
              _inp("T2", "temperature", "second temperature", labels=L_T2, symbols=["T2"], role="final"),
              _inp("dH", "molar_energy", "enthalpy of vaporization", symbols=["ΔHvap", "ΔH_vap", "ΔH"])],
      outputs=[_out("P2", "pressure", "vapor pressure at T2")],
      primary="P2", run=cc_pressure,
      verify=lambda i, o: abs(math.log(o["P2"] / i["P1"]) + i["dH"] / R * (1 / i["T2"] - 1 / i["T1"])) if o["P2"] > 0 else 1.0,
      reference=_ref_cc, wrong=lambda P1, T1, T2, dH: {"P2": P1 * math.exp(dH / R * (1 / T2 - 1 / T1))},
      rand=lambda rng: {"P1": _u_(rng, 100.0, 2e5), "T1": rng.uniform(250.0, 450.0), "T2": rng.uniform(250.0, 450.0),
                        "dH": rng.uniform(2e4, 6e4)},
      kw=[r"vapou?r pressure", r"vapori[sz]ation|δh"], nokw=[r"solution|dissolved"],
      need=_has("pressure", "temperature", "temperature", "molar_energy", exactly=True),
      latex=r"\ln\frac{P_2}{P_1} = -\frac{\Delta H_{vap}}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)",
      method="integrated Clausius-Clapeyron equation", conditions=["dHvap constant", "ideal vapor"],
      templates=["Water has a vapor pressure of 23.8 torr at 25 °C and ΔHvap = 43.9 kJ/mol. What is its vapor pressure at 50 °C?"],
      examples=[{"P1": 23.8 * ATM / 760, "T1": 298.15, "T2": 323.15, "dH": 43900.0}],
      questions=["Water has a vapor pressure of 23.8 torr at 25 °C and ΔHvap = 43.9 kJ/mol. What is its vapor pressure at 50 °C?"])


def cc_enthalpy(P1, T1, P2, T2):
    _pos(P1=P1, T1=T1, P2=P2, T2=T2)
    if T1 == T2:
        raise ValueError("the two temperatures must differ")
    return {"dH": R * math.log(P2 / P1) / (1 / T1 - 1 / T2)}


_proc("clausius_clapeyron_enthalpy", name="Clausius-Clapeyron: enthalpy of vaporization", topic="thermochemistry / phase equilibria",
      level="advanced", purpose="The enthalpy of vaporization from vapor pressures at two temperatures.",
      inputs=[_inp("P1", "pressure", "vapor pressure at T1", symbols=["P1"], label_only=True),
              _inp("T1", "temperature", "first temperature", symbols=["T1"], label_only=True),
              _inp("P2", "pressure", "vapor pressure at T2", symbols=["P2"], label_only=True),
              _inp("T2", "temperature", "second temperature", symbols=["T2"], label_only=True)],
      outputs=[_out("dH", "molar_energy", "enthalpy of vaporization")],
      primary="dH", run=cc_enthalpy, verify=lambda i, o: abs(math.log(i["P2"] / i["P1"]) - o["dH"] / R * (1 / i["T1"] - 1 / i["T2"])),
      reference=lambda P1, T1, P2, T2: {"dH": -float(__import__("numpy").polyfit([1 / T1, 1 / T2], [math.log(P1), math.log(P2)], 1)[0])
                                        * __import__("scipy.constants").constants.R},
      wrong=lambda P1, T1, P2, T2: {"dH": R * math.log(P2 / P1) / (T2 - T1)},
      rand=lambda rng: (lambda T1, T2, dH, P1: {"P1": P1, "T1": T1, "T2": T2, "P2": P1 * math.exp(-dH / R * (1 / T2 - 1 / T1))})(
          rng.uniform(250.0, 350.0), rng.uniform(355.0, 450.0), rng.uniform(2e4, 6e4), _u_(rng, 100.0, 1e5)),
      kw=[r"vapori[sz]ation|δh"], need=lambda t, qs: all(s in t for s in ("⟦P1⟧", "⟦P2⟧", "⟦T1⟧", "⟦T2⟧")),
      latex=r"\Delta H_{vap} = \frac{R\ln(P_2/P_1)}{1/T_1 - 1/T_2}", method="two-point Clausius-Clapeyron equation",
      conditions=["T1 != T2"],
      templates=["P1 = 100 torr at T1 = 300 K and P2 = 400 torr at T2 = 340 K. What is the enthalpy of vaporization?"],
      examples=[{"P1": 100 * ATM / 760, "T1": 300.0, "P2": 400 * ATM / 760, "T2": 340.0}],
      questions=["P1 = 100 torr at T1 = 300 K and P2 = 400 torr at T2 = 340 K. What is the enthalpy of vaporization?"])

# ====================================================================================================================
# ELECTROCHEMISTRY
# ====================================================================================================================
_N_E = lambda: _inp("n", "number", "electrons transferred", symbols=["n", "z"], label_only=True,
                    labels=[r"electrons? transferred(?: is| =)?", r"number of electrons(?: is| =)?"],
                    suffix_labels=[r"electrons?"])


def _ion_charge_text(q, spans):
    m = re.search(r"\b([A-Z][a-z]?)\^?\(?(\d?)\+\)?", q)
    if m and m.group(1) in W:
        return [("n", float(m.group(2) or 1), m.span()), ("metal", m.group(1), (m.end(), m.end()))]
    return []


def dg_from_e(n, E):
    _pos(n=n)
    _finite(E=E)
    return {"dG": -n * F * E, "dG_kJ": -n * F * E / 1000.0}


_proc("delta_g_from_cell_potential", name="Gibbs energy from a cell potential", topic="electrochemistry / cell thermodynamics",
      level="intermediate", purpose="The (standard) Gibbs energy change of a cell reaction from its potential (dG = -nFE).",
      inputs=[_N_E(), _inp("E", "voltage", "cell potential")],
      outputs=[_out("dG", "molar_energy", "Gibbs energy change"), _out("dG_kJ", "number", "in kJ/mol")],
      primary="dG", run=dg_from_e, verify=lambda i, o: _rel(o["dG"], -i["n"] * F * i["E"]),
      reference=lambda n, E: {"dG": -n * __import__("scipy.constants").constants.physical_constants["Faraday constant"][0] * E},
      wrong=lambda n, E: {"dG": -F * E},
      rand=lambda rng: {"n": float(rng.choice([2, 3, 4, 6])), "E": rng.uniform(-3.0, 3.0)},
      kw=[r"δg|gibbs"], nokw=[r"(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])|equilibrium constant|\bq\b|quotient|δh|δs|entropy|enthalpy"],
      need=_has("voltage", exactly=True),
      latex=r"\Delta G = -nFE", method="dG = -n F E", conditions=["n > 0"],
      templates=["A cell has E° = 1.10 V and n = 2. What is ΔG°?"],
      examples=[{"n": 2.0, "E": 1.10}],
      questions=["A cell has E° = 1.10 V and n = 2. What is ΔG°?", "A cell has E° = 1.10 V. What is ΔG°?"])


def k_from_e(n, E, T):
    _pos(n=n, T=T)
    _finite(E=E)
    lnK = n * F * E / (R * T)
    return {"lnK": lnK, "K": math.exp(lnK) if lnK < 700 else math.inf}


_proc("k_from_cell_potential", name="Equilibrium constant from a standard cell potential",
      topic="electrochemistry / cell thermodynamics", level="advanced",
      purpose="The equilibrium constant of a cell reaction from its standard potential (ln K = nFE°/RT).",
      inputs=[_N_E(), _inp("E", "voltage", "standard cell potential"), _inp("T", "temperature", "temperature")],
      outputs=[_out("lnK", "number", "ln K"), _out("K", "number", "K")],
      primary="lnK", run=k_from_e, verify=lambda i, o: abs(o["lnK"] * R * i["T"] - i["n"] * F * i["E"]) / (R * i["T"]),
      reference=lambda n, E, T: {"lnK": _brentq(lambda L: -n * 96485.33212331001 * E + 8.31446261815324 * T * L * -1.0, -1e5, 1e5)
                                 if False else n * __import__("scipy.constants").constants.physical_constants["Faraday constant"][0]
                                 * E / (__import__("scipy.constants").constants.R * T)},
      wrong=lambda n, E, T: {"lnK": n * F * E / (R * T) / math.log(10.0)},
      rand=lambda rng: {"n": float(rng.choice([1, 2, 3, 4])), "E": rng.uniform(-1.5, 1.5), "T": rng.uniform(270.0, 400.0)},
      kw=[r"equilibrium constant|(?<!\d )(?<![\d·*/(])\bk\b(?![)/^])"], nokw=[r"\bq\b|quotient|concentration|δg|gibbs"],
      need=_has("voltage", "temperature", exactly=True),
      latex=r"\ln K = \frac{nFE^\circ}{RT}", method="ln K = n F E° / (R T)", conditions=["n, T > 0"],
      templates=["A cell has E° = 0.46 V with n = 2 at 298 K. What is the equilibrium constant?"],
      examples=[{"n": 2.0, "E": 0.46, "T": 298.0}],
      questions=["A cell has E° = 0.46 V with n = 2 at 298 K. What is the equilibrium constant?"])


def nernst(E0, n, Q, T):
    _pos(n=n, Q=Q, T=T)
    _finite(E0=E0)
    return {"E": E0 - R * T / (n * F) * math.log(Q)}


_proc("nernst_potential", name="Nernst equation", topic="electrochemistry / cell potentials", level="advanced",
      purpose="The cell potential at non-standard conditions from E°, the electrons transferred and the reaction quotient.",
      inputs=[_inp("E0", "voltage", "standard cell potential"), _N_E(),
              _inp("Q", "number", "reaction quotient", symbols=["Q"], labels=[r"reaction quotient(?: is| of)?"]),
              _inp("T", "temperature", "temperature")],
      outputs=[_out("E", "voltage", "cell potential")],
      primary="E", run=nernst,
      verify=lambda i, o: abs(o["E"] - i["E0"] + R * i["T"] / (i["n"] * F) * math.log(i["Q"])) / max(1.0, abs(i["E0"])),
      reference=lambda E0, n, Q, T: (lambda sc: {"E": E0 - sc.R * T / (n * sc.physical_constants["Faraday constant"][0])
                                                 * float(__import__("numpy").log(Q))})(__import__("scipy.constants").constants),
      wrong=lambda E0, n, Q, T: {"E": E0 + R * T / (n * F) * math.log(Q)},
      rand=lambda rng: {"E0": rng.uniform(-2.0, 2.0), "n": float(rng.choice([1, 2, 3])), "Q": _u_(rng, 1e-6, 1e6),
                        "T": rng.uniform(270.0, 400.0)},
      kw=[r"nernst|cell potential|\be\b|potential", r"\bq\b|reaction quotient"],
      need=_has("voltage", "none", "none", "temperature", exactly=True),
      latex=r"E = E^\circ - \frac{RT}{nF}\ln Q", method="E = E° - (RT/nF) ln Q", conditions=["n, Q, T > 0"],
      templates=["E° = 1.10 V, n = 2, Q = 0.010 at 298 K. What is the cell potential?"],
      examples=[{"E0": 1.10, "n": 2.0, "Q": 0.01, "T": 298.0}],
      questions=["E° = 1.10 V, n = 2, Q = 0.010 at 298 K. What is the cell potential?",
                 "Use the Nernst equation: E° = 0.80 V, n = 1, reaction quotient 50, T = 310 K. Find E."])


def electrolysis_mass(metal, I, t, n):
    _pos(I=I, t=t, n=n)
    el = _element(metal)
    m = I * t / (n * F) * W[el] / 1000.0
    return {"m": m, "m_g": m * 1000.0}


_proc("electrolysis_mass", name="Mass deposited in electrolysis (Faraday's law)", topic="electrochemistry / electrolysis",
      level="intermediate", purpose="The mass of a metal deposited by a current flowing for a time.",
      inputs=[_inp("metal", "number", "the element deposited (symbol or name)", type="text", min_length=1, max_length=20),
              _inp("I", "current", "current"), _inp("t", "time", "time"), _N_E()],
      outputs=[_out("m", "mass", "mass deposited"), _out("m_g", "number", "in grams")],
      primary="m", run=electrolysis_mass,
      verify=lambda i, o: _rel(o["m"] * i["n"] * F, i["I"] * i["t"] * W[_element(i["metal"])] / 1000.0),
      reference=lambda metal, I, t, n: {"m": I * t / (n * __import__("scipy.constants").constants.physical_constants[
          "Faraday constant"][0]) * _ref_w(_element(metal)) / 1000.0},
      wrong=lambda metal, I, t, n: {"m": I * t / F * W[_element(metal)] / 1000.0},
      rand=lambda rng: {"metal": rng.choice(["Cu", "Ag", "Zn", "Ni", "Al", "Cr", "Au", "Fe"]), "I": _u_(rng, 0.01, 100.0),
                        "t": _u_(rng, 1.0, 1e5), "n": float(rng.choice([2, 3]))},
      kw=[r"deposit|plated|electroly|electroplat"], nokw=[r"how long|what time|time (?:needed|required)"],
      text=_texts(_ion_charge_text, _te("metal", r"(?:mass of|grams of)")), need=_has("current", "time", exactly=True),
      latex=r"m = \frac{I t}{n F} M", method="charge / (n F) gives moles of metal; times its molar mass",
      conditions=["100% current efficiency"],
      templates=["What mass of copper is deposited from a Cu2+ solution by a current of 2.00 A flowing for 30.0 min?"],
      examples=[{"metal": "Cu", "I": 2.0, "t": 1800.0, "n": 2.0}],
      questions=["What mass of copper is deposited from a Cu2+ solution by a current of 2.00 A flowing for 30.0 min?",
                 "What mass of silver is plated from Ag+ by 0.500 A in 2.00 h?"])


def electrolysis_time(metal, m, I, n):
    _pos(m=m, I=I, n=n)
    el = _element(metal)
    return {"t": m * 1000.0 / W[el] * n * F / I}


_proc("electrolysis_time", name="Time for electrolysis (Faraday's law)", topic="electrochemistry / electrolysis",
      level="intermediate", purpose="How long a current must flow to deposit a given mass of a metal.",
      inputs=[_inp("metal", "number", "the element deposited (symbol or name)", type="text", min_length=1, max_length=20),
              _inp("m", "mass", "mass to deposit"), _inp("I", "current", "current"), _N_E()],
      outputs=[_out("t", "time", "time needed")],
      primary="t", run=electrolysis_time,
      verify=lambda i, o: _rel(o["t"] * i["I"], i["m"] * 1000.0 / W[_element(i["metal"])] * i["n"] * F),
      reference=lambda metal, m, I, n: {"t": _brentq(lambda t: I * t / (n * 96485.33212331001) * _ref_w(_element(metal)) / 1000.0 - m,
                                                     0.0, 1e12)},
      wrong=lambda metal, m, I, n: {"t": m * 1000.0 / W[_element(metal)] * F / I},
      rand=lambda rng: {"metal": rng.choice(["Cu", "Ag", "Zn", "Ni", "Al", "Cr", "Au", "Fe"]), "m": _u_(rng, 1e-4, 1.0),
                        "I": _u_(rng, 0.01, 100.0), "n": float(rng.choice([2, 3]))},
      kw=[r"deposit|plated|electroly|electroplat", r"how long|what time|time (?:needed|required)"],
      text=_texts(_ion_charge_text, _te("metal", r"(?:mass of|grams of|g of|kg of|mg of)")),
      need=_has("mass", "current", exactly=True),
      latex=r"t = \frac{m n F}{M I}", method="t = m n F / (M I)", conditions=["100% current efficiency"],
      templates=["How long must a current of 5.00 A flow to deposit 10.0 g of Ni from a Ni2+ solution?"],
      examples=[{"metal": "Ni", "m": 0.01, "I": 5.0, "n": 2.0}],
      questions=["How long must a current of 5.00 A flow to deposit 10.0 g of Ni from a Ni2+ solution?"])

# ====================================================================================================================
# SPECTROSCOPY AND MOLECULAR PROPERTIES
# ====================================================================================================================
def photon_energy(lam):
    _pos(lam=lam)
    E = H_PLANCK * C_LIGHT / lam
    return {"E": E, "E_eV": E / EV, "E_molar": E * NA, "nu": C_LIGHT / lam}


_proc("photon_energy", name="Energy of a photon from its wavelength", topic="spectroscopy / light", level="beginner",
      purpose="The energy (per photon and per mole of photons) and frequency of light of a given wavelength.",
      inputs=[_inp("lam", "length", "wavelength")],
      outputs=[_out("E", "energy", "energy per photon"), _out("E_eV", "number", "in eV"),
               _out("E_molar", "molar_energy", "energy per mole of photons"), _out("nu", "frequency", "frequency")],
      primary="E", run=photon_energy, verify=lambda i, o: _rel(o["E"] * i["lam"], H_PLANCK * C_LIGHT),
      reference=lambda lam: (lambda sc: {"E_molar": sc.h * sc.c / lam * sc.N_A, "E_eV": sc.h * sc.c / lam / sc.e,
                                         "nu": sc.c / lam})(__import__("scipy.constants").constants),
      wrong=lambda lam: {"E": H_PLANCK / lam, "E_eV": H_PLANCK / lam / EV, "E_molar": H_PLANCK / lam * NA, "nu": 1.0 / lam},
      rand=lambda rng: {"lam": _u_(rng, 1e-11, 1e-2)},
      kw=[r"\benergy\b", r"photon|light|radiation|wavelength"], nokw=[r"hydrogen|transition|work function"],
      need=_has("length", exactly=True),
      latex=r"E = \frac{hc}{\lambda}", method="E = h c / lambda", conditions=["lambda > 0"],
      templates=["What is the energy of a photon of wavelength 500 nm?"],
      examples=[{"lam": 5e-7}],
      questions=["What is the energy of a photon of wavelength 500 nm?",
                 "Calculate the energy per mole of photons of light with a wavelength of 254 nm."])


def photon_wavelength(E):
    _pos(E=E)
    lam = H_PLANCK * C_LIGHT / E
    return {"lam": lam, "lam_nm": lam * 1e9}


_proc("photon_wavelength", name="Wavelength of a photon from its energy", topic="spectroscopy / light", level="beginner",
      purpose="The wavelength of a photon of a given energy.",
      inputs=[_inp("E", "energy", "photon energy")],
      outputs=[_out("lam", "length", "wavelength"), _out("lam_nm", "number", "in nm")],
      primary="lam", run=photon_wavelength, verify=lambda i, o: _rel(o["lam"] * i["E"], H_PLANCK * C_LIGHT),
      reference=lambda E: (lambda sc: {"lam_nm": sc.h * sc.c / E * 1e9})(__import__("scipy.constants").constants),
      wrong=lambda E: {"lam": H_PLANCK / E, "lam_nm": H_PLANCK / E * 1e9},
      rand=lambda rng: {"E": _u_(rng, 1e-21, 1e-15)},
      kw=[r"wavelength", r"photon|light|radiation"], nokw=[r"hydrogen|transition"],
      need=_has("energy", exactly=True),
      latex=r"\lambda = \frac{hc}{E}", method="lambda = h c / E", conditions=["E > 0"],
      templates=["What is the wavelength of a photon with an energy of 3.0e-19 J?", "Find the wavelength of a 2.5 eV photon."],
      examples=[{"E": 3.0e-19}, {"E": 2.5 * EV}],
      questions=["What is the wavelength of a photon with an energy of 3.0e-19 J?", "Find the wavelength of a 2.5 eV photon."])


def beer_absorbance(eps, l, c):
    _pos(eps=eps, l=l, c=c)
    A = eps * l * c
    return {"A": A, "T_percent": 100.0 * 10.0 ** -A}


_proc("beer_lambert_absorbance", name="Beer-Lambert law: absorbance", topic="spectroscopy / absorption", level="beginner",
      purpose="The absorbance of a solution from its molar absorptivity, path length and concentration.",
      inputs=[_inp("eps", "molar_absorptivity", "molar absorptivity"), _inp("l", "length", "path length"),
              _inp("c", "molar_concentration", "concentration")],
      outputs=[_out("A", "number", "absorbance"), _out("T_percent", "number", "percent transmittance")],
      primary="A", run=beer_absorbance, verify=lambda i, o: _rel(o["A"], i["eps"] * i["l"] * i["c"]),
      reference=lambda eps, l, c: {"A": -float(__import__("numpy").log10(__import__("numpy").power(10.0, -eps * l * c)))},
      wrong=lambda eps, l, c: {"A": eps * c / l},
      rand=lambda rng: {"eps": _u_(rng, 1.0, 1e4), "l": _u_(rng, 1e-3, 0.1), "c": _u_(rng, 1e-3, 10.0)},
      kw=[r"absorbance"], nokw=[r"transmittance\s*(?:of|is|=)\s*\d"],
      need=_has("molar_absorptivity", "length", "molar_concentration", exactly=True),
      latex=r"A = \varepsilon l c", method="A = epsilon l c", conditions=["dilute solution, monochromatic light"],
      templates=["A 1.00 cm cell holds a 2.0e-5 M solution with molar absorptivity 15000 L/(mol·cm). What is the absorbance?"],
      examples=[{"eps": 1500.0, "l": 0.01, "c": 0.02}],
      questions=["A 1.00 cm cell holds a 2.0e-5 M solution with molar absorptivity 15000 L/(mol·cm). What is the absorbance?"])


def beer_concentration(A, eps, l):
    _pos(A=A, eps=eps, l=l)
    c = A / (eps * l)
    return {"c": c, "c_molL": c / 1000.0}


_proc("beer_lambert_concentration", name="Beer-Lambert law: concentration", topic="spectroscopy / absorption", level="beginner",
      purpose="The concentration of a solution from its absorbance, molar absorptivity and path length.",
      inputs=[_inp("A", "number", "absorbance", symbols=["A"], labels=[r"absorbance(?: of| is)?"]),
              _inp("eps", "molar_absorptivity", "molar absorptivity"), _inp("l", "length", "path length")],
      outputs=[_out("c", "molar_concentration", "concentration"), _out("c_molL", "number", "in mol/L")],
      primary="c", run=beer_concentration, verify=lambda i, o: _rel(o["c"] * i["eps"] * i["l"], i["A"]),
      reference=lambda A, eps, l: {"c": _brentq(lambda c: eps * l * c - A, 0.0, 10 * A / (eps * l))},
      wrong=lambda A, eps, l: {"c": A * eps * l},
      rand=lambda rng: {"A": rng.uniform(0.01, 2.0), "eps": _u_(rng, 1.0, 1e4), "l": _u_(rng, 1e-3, 0.1)},
      kw=[r"absorbance", r"concentration|molarity"],
      need=_has("none", "molar_absorptivity", "length", exactly=True),
      latex=r"c = \frac{A}{\varepsilon l}", method="c = A / (epsilon l)", conditions=["dilute solution"],
      templates=["A solution has an absorbance of 0.450 in a 1.00 cm cell; ε = 1.25e4 L/(mol·cm). What is its concentration?"],
      examples=[{"A": 0.45, "eps": 1250.0, "l": 0.01}],
      questions=["A solution has an absorbance of 0.450 in a 1.00 cm cell; ε = 1.25e4 L/(mol·cm). What is its concentration?"])

R_H = R_INF / (1.0 + M_E / M_P)


def hydrogen_line(n_upper, n_lower):
    n_upper, n_lower = int(n_upper), int(n_lower)
    if n_lower < 1 or n_upper <= n_lower:
        raise ValueError("the levels must be whole numbers with n_upper > n_lower >= 1")
    inv = R_H * (1.0 / n_lower ** 2 - 1.0 / n_upper ** 2)
    return {"lam": 1.0 / inv, "lam_nm": 1e9 / inv, "E": H_PLANCK * C_LIGHT * inv}


def _h_text(q, spans):
    ns = [(int(m.group(1)), m.span()) for m in re.finditer(r"\bn\s*=\s*(\d+)", q)]
    if len(ns) != 2:
        return []
    hi, lo = max(ns[0][0], ns[1][0]), min(ns[0][0], ns[1][0])
    return [("n_upper", hi, ns[0][1]), ("n_lower", lo, ns[1][1])]


def _ref_hline(n_upper, n_lower):
    import scipy.constants as sc
    RH = sc.Rydberg / (1 + sc.m_e / sc.m_p)
    return {"lam_nm": 1e9 / (RH * (1 / n_lower ** 2 - 1 / n_upper ** 2))}


_proc("hydrogen_transition_wavelength", name="Hydrogen atom transition wavelength (Rydberg)", topic="spectroscopy / atomic spectra",
      level="intermediate", purpose="The wavelength of the photon emitted or absorbed in a transition between two levels of hydrogen.",
      inputs=[_inp("n_upper", "integer", "upper level", type="integer", min=2, max=1000, symbols=["n_upper"], label_only=True),
              _inp("n_lower", "integer", "lower level", type="integer", min=1, max=999, symbols=["n_lower"], label_only=True)],
      outputs=[_out("lam", "length", "wavelength"), _out("lam_nm", "number", "in nm"), _out("E", "energy", "photon energy")],
      primary="lam", run=hydrogen_line,
      verify=lambda i, o: _rel(1.0 / o["lam"], R_H * (1.0 / i["n_lower"] ** 2 - 1.0 / i["n_upper"] ** 2)),
      reference=_ref_hline,
      wrong=lambda n_upper, n_lower: (lambda inv: {"lam": 1 / inv, "lam_nm": 1e9 / inv})(R_H * (1.0 / n_lower - 1.0 / n_upper)),
      rand=lambda rng: (lambda lo: {"n_lower": lo, "n_upper": lo + rng.randint(1, 8)})(rng.randint(1, 6)),
      kw=[r"hydrogen", r"wavelength|transition|emit|emission|absorb|photon|line"], text=_texts(_h_text),
      need=lambda t, qs: not qs,
      latex=r"\frac{1}{\lambda} = R_H\left(\frac{1}{n_1^2} - \frac{1}{n_2^2}\right)",
      method="Rydberg formula with the hydrogen Rydberg constant (reduced-mass corrected)", conditions=["hydrogen atom"],
      templates=["What is the wavelength of light emitted when an electron in hydrogen falls from n = 3 to n = 2?"],
      examples=[{"n_upper": 3, "n_lower": 2}, {"n_upper": 2, "n_lower": 1}],
      questions=["What is the wavelength of light emitted when an electron in hydrogen falls from n = 3 to n = 2?",
                 "Find the wavelength of the hydrogen transition from n = 1 to n = 4."])

_DOU_EL = {"C", "H", "N", "O", "S", "P", "F", "Cl", "Br", "I", "Si", "B"}


def degree_of_unsaturation(formula):
    c = formula_counts(formula)
    if set(c) - _DOU_EL:
        raise ValueError("only C, H, N, O, S, P, Si, B and halogens are supported")
    if not c.get("C"):
        raise ValueError("an organic formula (with carbon) is expected")
    tet = c.get("C", 0) + c.get("Si", 0)
    tri = c.get("N", 0) + c.get("P", 0) + c.get("B", 0)
    mono = c.get("H", 0) + sum(c.get(x, 0) for x in ("F", "Cl", "Br", "I"))
    d = Fraction(2 * tet + 2 + tri - mono, 2)
    if d < 0 or d.denominator != 1:
        raise ValueError("this formula has no valid structure (negative or half-integer unsaturation)")
    return {"dou": float(d)}


def _ref_dou(formula):
    from rdkit import Chem
    pt = Chem.GetPeriodicTable()
    counts = _ref_counts(formula)
    return {"dou": 1.0 + sum(n * (pt.GetDefaultValence(el) - 2) for el, n in counts.items()) / 2.0}


_proc("degree_of_unsaturation", name="Degree of unsaturation", topic="molecular properties / structure", level="intermediate",
      purpose="The number of rings plus pi bonds implied by a molecular formula.",
      inputs=[_inp("formula", "number", "molecular formula", type="text", min_length=1, max_length=60)],
      outputs=[_out("dou", "number", "degree of unsaturation (rings + pi bonds)")],
      primary="dou", run=degree_of_unsaturation,
      verify=lambda i, o: (lambda c: abs(o["dou"] - (2 * (c.get("C", 0) + c.get("Si", 0)) + 2 + c.get("N", 0) + c.get("P", 0)
                                                     + c.get("B", 0) - c.get("H", 0) - sum(c.get(x, 0) for x in ("F", "Cl", "Br", "I"))) / 2))(
          _counts_rd(i["formula"])),
      reference=_ref_dou,
      wrong=lambda formula: (lambda c: {"dou": (2 * c.get("C", 0) + 2 - c.get("N", 0) - c.get("H", 0)
                                                - sum(c.get(x, 0) for x in ("F", "Cl", "Br", "I"))) / 2})(formula_counts(formula)),
      rand=lambda rng: (lambda C, N: {"formula": f"C{C}H{2 * C + 2 + N - 2 * rng.randint(0, C)}" + (f"N{N}" if N else "")
                                      + (f"O{rng.randint(1, 3)}" if rng.random() < 0.5 else "")})(rng.randint(2, 12), rng.choice([1, 1, 2, 3])),
      kw=[r"degree of unsaturation|index of hydrogen deficiency|double[- ]bond equivalents?|rings plus (?:pi|double) bonds|"
          r"unsaturation"], text=_texts(_tf("formula", r"(?:of|for)")), need=lambda t, qs: not qs,
      latex=r"\mathrm{DoU} = \frac{2C + 2 + N - H - X}{2}", method="(2C + 2 + N - H - X)/2 (Si as C, P and B as N, O and S ignored)",
      conditions=["a neutral organic formula"],
      templates=["What is the degree of unsaturation of C6H6?", "Find the degree of unsaturation for C5H9NO."],
      examples=[{"formula": "C6H6"}, {"formula": "C5H9NO"}, {"formula": "C4H8Cl2"}],
      questions=["What is the degree of unsaturation of C6H6?", "Find the degree of unsaturation for C5H9NO.",
                 "What is the degree of unsaturation of C2H7?"])


def vib_wavenumber(molecule, k):
    _pos(k=k)
    c = formula_counts(molecule)
    atoms = [el for el, n in c.items() for _ in range(n)]
    if len(atoms) != 2 or any(a not in ISO for a in atoms):
        raise ValueError("a diatomic molecule of H, C, N, O, F, Cl, Br, I, S ... is expected")
    m1, m2 = ISO[atoms[0]] * U_KG, ISO[atoms[1]] * U_KG
    mu = m1 * m2 / (m1 + m2)
    nu = math.sqrt(k / mu) / (2 * math.pi)
    return {"wavenumber": nu / C_LIGHT, "wavenumber_cm": nu / C_LIGHT / 100.0, "nu": nu}


def _ref_vib(molecule, k):
    from rdkit import Chem
    import scipy.constants as sc
    pt = Chem.GetPeriodicTable()
    atoms = [el for el, n in _ref_counts(molecule).items() for _ in range(n)]
    m = [pt.GetMostCommonIsotopeMass(a) * sc.atomic_mass for a in atoms]
    mu = m[0] * m[1] / (m[0] + m[1])
    return {"wavenumber_cm": math.sqrt(k / mu) / (2 * math.pi * sc.c) / 100.0}


_proc("vibrational_wavenumber", name="Vibrational wavenumber of a diatomic molecule", topic="spectroscopy / infrared",
      level="expert", purpose="The harmonic vibrational wavenumber of a diatomic molecule from its bond force constant.",
      inputs=[_inp("molecule", "number", "diatomic molecule, e.g. HCl or CO", type="text", min_length=2, max_length=10),
              _inp("k", "spring_constant", "bond force constant")],
      outputs=[_out("wavenumber", "wavenumber", "wavenumber"), _out("wavenumber_cm", "number", "in cm^-1"),
               _out("nu", "frequency", "vibrational frequency")],
      primary="wavenumber", run=vib_wavenumber,
      verify=lambda i, o: (lambda r: _rel(o["wavenumber"], r["wavenumber"]))(vib_wavenumber(**i)),
      reference=_ref_vib, wrong=lambda molecule, k: (lambda r: {"wavenumber_cm": r["wavenumber_cm"] * 2 * math.pi,
                                                                "wavenumber": r["wavenumber"] * 2 * math.pi})(vib_wavenumber(molecule, k)),
      rand=lambda rng: {"molecule": rng.choice(["HCl", "CO", "H2", "N2", "HF", "NO", "O2", "Cl2", "HBr", "HI", "CN", "BrCl"]),
                        "k": _u_(rng, 100.0, 2500.0)},
      kw=[r"vibrational|stretching|infrared|\bir\b", r"wavenumber|frequency"],
      text=_texts(_tf("molecule", r"(?:of|for)")), need=_has("spring_constant", exactly=True),
      latex=r"\tilde{\nu} = \frac{1}{2\pi c}\sqrt{\frac{k}{\mu}}",
      method="harmonic oscillator with the reduced mass of the most abundant isotopes",
      conditions=["harmonic approximation", "most abundant isotopes (1H, 12C, 16O, 35Cl ...)"],
      templates=["What is the vibrational wavenumber of HCl if its force constant is 516 N/m?"],
      examples=[{"molecule": "HCl", "k": 516.0}, {"molecule": "CO", "k": 1902.0}],
      questions=["What is the vibrational wavenumber of HCl if its force constant is 516 N/m?",
                 "Find the vibrational frequency of CO with a force constant of 1902 N/m."])


# ====================================================================================================================
# worked examples and questions with their outputs (procedure_examples.py holds the outputs, written once from the
# procedures and checked against the reference values of the textbook problems; certification reproduces each)
# ====================================================================================================================
def _attach():
    import ast
    from pathlib import Path
    path = Path(__file__).with_name("procedure_examples.py")
    expect = {}
    if path.exists():
        text = path.read_text(encoding="utf-8")
        expect = ast.literal_eval(text[text.index("{"):])
    for key, (exs, qs) in _EX.items():
        e = expect.get(key, {})
        spec = PROCEDURES[key]
        spec["examples"] = [{"inputs": i, "outputs": o} for i, o in zip(exs, e.get("examples", [])) if o is not None]
        spec["questions"] = [({"q": q, "outcome": o} if isinstance(o, str) else {"q": q, "outputs": o})
                             for q, o in zip(qs, e.get("questions", [])) if o is not None]


_attach()
