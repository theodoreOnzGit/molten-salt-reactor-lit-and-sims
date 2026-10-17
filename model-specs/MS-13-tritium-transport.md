# MS-13: Tritium production and transport

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #55 (part of epic #42)

## Purpose

Predict tritium production from lithium and beryllium in the salt and its
distribution: off-gas, graphite, permeation through heat-exchanger walls into
the coolant and onward to the environment. Tritium release is a key
licensing-relevant source term for any lithium-bearing fluoride salt reactor.

## Scope

MSRE fuel and coolant loops; MSBR steady-state distribution as a code-to-code
case; the Coolant-Salt Technology Facility as a separate-effects case.

## Governing physics (on by default)

- Production by ⁶Li(n,α)T, ⁷Li(n,n′α)T and ⁹Be(n,α)⁶He→⁶Li, with ⁶Li
  depletion and regeneration from MS-04.
- Speciation as T₂/TF set by redox (MS-09).
- Transport in salt, mass transfer to gas, absorption in graphite (MS-12),
  permeation through metal walls (diffusion with surface effects).

## Inputs and sources

| Input | Source |
|---|---|
| Steady-state tritium distribution method, MSBR | ORNL-TM-4804 |
| Tritium in the Coolant-Salt Technology Facility, measured | ORNL-TM-5759 |
| Third salt loop for tritium control (design) | ORNL-TM-3428 |
| Tritium removal from molten-salt blankets | ORNL-TM-5104 |
| MSRE tritium observations | ORNL MSR Program semiannual progress reports (search needed; none transcribed) |
| Cross sections | ENDF/B-VIII.0 |

## OUTRAM PARK starting point

**Absent.** No tritium production or permeation model exists in the backend.
`outram-mc-libs` tallies can give reaction rates; `outram-park-fork-onix` can
deplete ⁶Li.

## Verification

- Production rate from MC tallies vs a hand calculation with one-group rates.
- Permeation through a wall vs the analytical diffusion-limited solution.
- Mass balance: produced = decayed + inventory + every outflow.
- Code-to-code against the ORNL-TM-4804 method for its MSBR case.

## Validation

Against ORNL-TM-5759 measurements, and MSRE tritium data once located. **No
tritium benchmark issue exists yet; this spec's issue tracks filing one and
locating the MSRE data.**

## Known traps

- **Surface-limited vs diffusion-limited permeation** changes the answer by
  orders of magnitude. Test which regime holds; do not choose the one that
  fits.
- **⁶Li content of the salt must be known** (enrichment and burn-out), not
  assumed.
