# MS-09: Salt chemistry and thermochemistry

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #51 (part of epic #42)

## Purpose

Predict phase equilibria (liquidus, freezing, precipitation), redox state
(U⁴⁺/U³⁺ ratio), oxide tolerance, solubility of actinides and fission-product
fluorides, and vapour pressure of the salts. Redox drives corrosion (MS-11)
and tellurium attack; liquidus sets freeze margins (MS-14); solubility limits
fuel loading.

## Scope

LiF-BeF₂-ZrF₄-UF₄ (MSRE fuel), LiF-BeF₂ (coolant, flush), LiF-BeF₂-ThF₄-UF₄
(MSBR), with UF₃, oxides, CeF₃/rare earths and PuF₃ as solutes. Out of scope:
process chemistry of fluorination and reductive extraction.

## Governing physics (on by default)

- Gibbs energy minimisation with a solution model that represents the
  non-ideality of these melts; an ideal-mixing ablation must be visible and
  labelled.
- Redox as an equilibrium of U⁴⁺/U³⁺ with the metal walls (Cr, Fe, Ni) and
  with HF/H₂ in cover gas.
- Oxide chemistry from H₂O-HF equilibration data.

## Inputs and sources

| Input | Source |
|---|---|
| Phase diagrams | ORNL-2295, ORNL-2548, ORNL-2896, ORNL-3373, ORNL-TM-2596 |
| EMF and activity in LiF-BeF₂ | ORNL-4257 |
| Oxide chemistry, H₂O-HF equilibration | ORNL-TM-1129 |
| Liquid-vapour equilibria | ORNL-4415 |
| CeF₃ solubility | ORNL-TM-2335 |
| PuF₃ fuelling | ORNL-TM-2256 |
| UF₃ equilibria with graphite | ORNL-TM-4056 |
| Chemical aspects of MSRE operation, measured redox history | ORNL-4658 |
| Radiation chemistry, fluorine in frozen salt | ORNL-TM-0500, ORNL-TM-3144 |
| Modern thermodynamic database | MSTDB-TC. **Distribution is controlled**; whether an open subset exists is unresolved (backend `docs/melcor-parity-scoping.md`). Do not commit it here |

## OUTRAM PARK starting point

`outram-park-fork-thermochimica` (unpublished): a Gibbs minimiser with ideal
and binary Redlich-Kister solutions and six analytic verification tests. No
database parser, so no MSTDB-TC, no quasichemical or sublattice models, no
ternary interactions, no redox or solubility workflow (`src/gem.rs:162-200`).
The backend's `reference-data/virtual_test_bed/msr/msfr/thermochemistry/`
holds an MSTDB-TC-family noble-gas deck the crate cannot read.

## Verification

- Reuse the six analytic tests; add a ternary Redlich-Kister identity.
- Code-to-code against upstream Thermochimica on an open database for the same
  system (the VTB deck if its licence allows).

## Validation

Liquidus temperatures against the phase diagrams; redox and oxide tolerance
against ORNL-4658 and ORNL-TM-1129. **No benchmark issue exists for salt
chemistry yet; this spec's issue tracks filing one.**

## Known traps

- **Ideal mixing is not the default physics** for BeF₂-containing melts.
- **Phase diagram points are read from 1950s figures**; carry the reading
  uncertainty.
- **Database provenance.** A Gibbs-energy dataset whose licence is unclear
  cannot enter this repository (AGENTS.md §6).
