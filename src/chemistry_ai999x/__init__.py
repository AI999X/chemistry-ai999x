"""chemistry-ai999x — offline chemistry answers with their formula, steps and verification.

    import chemistry_ai999x as m
    m.ask("What is the pH of 0.10 M acetic acid (Ka = 1.8 × 10^-5)?").show()      # pH 2.875 (exact charge balance)
    m.solve("balance_equation", equation="C3H8 + O2 -> CO2 + H2O").values         # C3H8 + 5 O2 -> 3 CO2 + 4 H2O
    m.capabilities()                   # every certified capability
    m.templates()                      # the documented question forms

Covers stoichiometry (molar mass from a formula with brackets and hydrates, mole conversions, percent composition,
empirical and molecular formulas, equation balancing, limiting reagent, theoretical and percent yield), gases (ideal
gas law, gas laws, density and molar mass, Dalton, Graham, van der Waals, molecular speeds), solutions (molarity,
dilution, molality, mole fraction, Raoult's law, colligative properties, osmotic pressure), chemical equilibrium (Kp/Kc,
ICE tables, K from dG°, van 't Hoff, Ksp and the common-ion effect), acids and bases (pH of strong, weak, diprotic acids
and bases, salts, buffers, titrations), kinetics (integrated rate laws, half-life, Arrhenius, reaction order,
consecutive reactions), thermochemistry (calorimetry, Hess's law, dG = dH - TdS, Clausius-Clapeyron),
electrochemistry (dG and K from E°, Nernst, Faraday electrolysis) and spectroscopy (photon energy, Beer-Lambert,
hydrogen spectrum, IR wavenumber, degree of unsaturation). Values are SI (a concentration in mol/m^3, with a mol/L
companion output). Built by the AI999X model machine; every capability is certified against independent libraries,
and a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("chemistry", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
