# MS-02: Core neutronics, steady state

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #44 (part of epic #42)

## Purpose

Compute k_eff, critical uranium concentration, control-rod worth, temperature
coefficients and flux and power shapes for the MSRE with stationary fuel. This
is the reference that the kinetics (MS-03), depletion (MS-04) and coupled plant
(MS-15) models are built from or checked against.

## Scope

MSRE graphite-moderated core with ²³⁵U and with ²³³U fuel, zero power,
isothermal. Out of scope here: circulating fuel (MS-03), burn-up (MS-04).

## Governing physics (on by default)

- Continuous-energy Monte Carlo transport as the reference; multigroup
  diffusion or SP3 for fast models, with group constants generated from the MC
  model, never fitted to a measurement.
- Graphite S(α,β) thermal scattering. Ablation switch: `without_graphite_sab`.
- Doppler broadening at the model temperature.
- Explicit control-rod geometry (three rods in thimbles, ORNL-4123).
- Salt in the graphite channels and in the downcomer, lower and upper heads.

## Inputs and sources

| Input | Source | Note |
|---|---|---|
| Core geometry, graphite stringers, vessel, heads | ORNL-TM-0728 (Design and Operations Report Part I) | Derive from the report; the IRPhEP MSRE evaluation may be **cited, not copied** |
| Control elements | ORNL-4123 | |
| Fuel composition and density | MS-01; ORNL-4233, ORNL-TM-3963 for the as-run loadings | |
| Graphite density, impurities, salt penetration | ORNL-TM-0730 (Part III, Nuclear Analysis) | Boron equivalent is an input with uncertainty, not a free parameter |
| Nuclear data | ENDF/B-VIII.0 via `njoy-outram-park-fork` | State the library and processing temperature |
| 1960s calculated values for comparison | ORNL-TM-0730, ORNL-TM-1946, ORNL-TM-0379 | Code-to-code reference for the 1960s method only |

## OUTRAM PARK starting point

- `outram-mc-libs`: CSG k-eigenvalue, S(α,β), tallies. Its own records show
  a +2950 ± 61 pcm bias on ICSBEP LEU-COMP-THERM-008 and +1107 pcm on Godiva.
  **That bias must be resolved or bounded before an MSRE k_eff means anything.**
- `njoy-outram-park-fork`: processing for Be-9, F-19, Li-6 is in its NJOY2016
  comparison tests; Li-7 and Zr isotopes are not (#10).
- `outram-foam-appbuilder-lib` GeN-Foam diffusion/SP3 for the fast path.

## Verification

- N-C1 (#9): k_eff of a simplified MSRE geometry, `outram-mc-libs` vs OpenMC
  with the same library.
- N-C2 (#10): cross sections for Li-7, Be-9, F-19, Zr, U vs NJOY2016.
- Diffusion/SP3 group constants reproduce the MC k_eff of the same geometry
  within the stated homogenisation error.

## Validation

N-1 (#2) critical concentration and excess reactivity; N-2 (#3) rod worth;
N-3 (#4) isothermal temperature coefficient; N-5 (#6) ²³³U zero-power
experiments. Data in `validation-data/ai-generated/neutronics/` (needs human
review, #39).

## Pass criteria and uncertainty

Report ρ_calc − ρ_meas in pcm with the MC statistical uncertainty, the
nuclear-data uncertainty (MS-16) and the measurement uncertainty given in
ORNL-4233. No pass threshold is set in advance of an uncertainty budget; set
it in the V&V record before the first run.

## Known traps

- **The +2950 pcm measured-critical bias in `outram-mc-libs`.** Do not
  interpret an MSRE result until it is explained.
- **Graphite and salt temperatures differ at power.** N-3 is isothermal;
  compare against an isothermal calculation.
- **Rod worth depends on the measurement method** (period vs rod-bump,
  ORNL-4233). Compare like with like. The transcribed differential-worth curve
  is the authors' fitted curve, not the measured points; compare integral
  worth first.
- **The ORNL-4233 delayed-neutron table holds adopted values, not
  measurements.** It is an input to the 1968 analysis, not a target.
- **Do not tune graphite boron or salt density to hit critical.** If k_eff
  misses, report the miss.
