# Model specifications

> **AI-drafted 2026-10-17, not human-reviewed.** Research, education and V&V
> only (see [`AGENTS.md`](../AGENTS.md)).

**Status, 2026-10-17: nothing is modelled yet.** Each file below says what a
molten salt reactor model must contain before it is worth validating: the
physics that is on by default, the inputs and the report each comes from, the
OUTRAM PARK crate to start from and what it lacks, the verification tests, and
the [benchmarks](../benchmarks/) that validate it.

The benchmarks say **what to compare against**. These specs say **what model
is compared**. A benchmark is never an input to the model it checks.

Tracking issue (epic): #42

## Specs

| ID | Aspect | Main OUTRAM PARK start | Validated by | Issue |
|---|---|---|---|---|
| [MS-01](MS-01-salt-properties.md) | Fuel, coolant and flush salt properties | `tuas_boussinesq_solver` `CustomLiquid` | TH-6 (#17) | #43 |
| [MS-02](MS-02-core-neutronics-steady-state.md) | Core neutronics, steady state | `outram-mc-libs`, `njoy-outram-park-fork` | N-1, N-2, N-3, N-5 | #44 |
| [MS-03](MS-03-circulating-fuel-kinetics.md) | Circulating-fuel kinetics | `outram-park-fork-moltres` (unpublished), `teh-o-prke` | N-4, T-1 to T-3, T-C1 | #45 |
| [MS-04](MS-04-depletion-fission-products-xenon.md) | Depletion, fission products, xenon, off-gas | `outram-park-fork-onix` (unpublished) | N-6, N-7 | #46 |
| [MS-05](MS-05-loop-thermal-hydraulics.md) | Primary and secondary loop TH (1-D) | `tuas_boussinesq_solver` | TH-3, T-7 | #47 |
| [MS-06](MS-06-core-thermal-hydraulics.md) | Core TH and flow distribution | `outram-foam-*` | TH-4, TH-C2 | #48 |
| [MS-07](MS-07-heat-exchangers-and-radiator.md) | Heat exchanger and radiator | `tuas_boussinesq_solver` shell-and-tube | TH-1, TH-2, TH-3 | #49 |
| [MS-08](MS-08-pumps.md) | Fuel and coolant pumps | `outram-park-fork-dwsim-libs` | TH-5, T-6 | #50 |
| [MS-09](MS-09-salt-chemistry.md) | Salt chemistry and thermochemistry | `outram-park-fork-thermochimica` (unpublished) | none yet | #51 |
| [MS-10](MS-10-structural-alloy.md) | Hastelloy N mechanical behaviour | `outram-park-fork-offbeat` (unpublished), `farrer-park` | M-1, M-2, M-3, M-C2 | #52 |
| [MS-11](MS-11-corrosion-and-tellurium.md) | Corrosion and tellurium cracking | absent | M-4, M-5, M-C1 | #53 |
| [MS-12](MS-12-graphite-moderator.md) | Graphite moderator | `tuas_boussinesq_solver` (thermal only) | none yet | #54 |
| [MS-13](MS-13-tritium-transport.md) | Tritium production and transport | absent | none yet | #55 |
| [MS-14](MS-14-decay-heat-drain-and-passive-cooling.md) | Decay heat, drain tanks, freeze valves, passive cooling | `teh-o-prke`, `outram-foam-appbuilder-lib` meltFoam | T-7 | #56 |
| [MS-15](MS-15-coupled-plant-dynamics.md) | Coupled plant dynamics and control | `outram-park-digital-twin-engine` | T-1 to T-7 | #57 |
| [MS-16](MS-16-uncertainty-quantification.md) | Uncertainty quantification | `raffles` | supplies error bars to all | #58 |

What each crate offers, crate by crate and aspect by aspect:
[`outram-park-capabilities.md`](outram-park-capabilities.md).

## Dependencies between models

```
MS-01 properties ──► MS-05, MS-06, MS-07, MS-08, MS-09, MS-14
MS-02 steady neutronics ──► MS-03 (β_eff, Λ), MS-04 (rates), MS-13 (rates)
MS-05 loops ──► MS-03 (transit times), MS-14 (natural circulation)
MS-09 chemistry ──► MS-11 (redox), MS-13 (T₂/TF)
MS-04 inventory ──► MS-11 (tellurium), MS-14 (decay heat)
MS-03 + MS-04 + MS-05 + MS-06 + MS-07 + MS-08 ──► MS-15 plant model
MS-16 ──► error bars on every comparison
```

## What every spec contains

1. **Purpose** and the design question the model answers.
2. **Scope**, including what is left out.
3. **Governing physics, on by default.** A term the physics supplies is
   applied unless a visible `without_*` switch ablates it (AGENTS.md §2).
4. **Inputs and sources**, each from a report in [`literature/`](../literature/moltensalt-org/README.md)
   or another citable open source.
5. **OUTRAM PARK starting point**: what to reuse and what is missing
   (search before building, AGENTS.md §7).
6. **Verification**: analytical, conservation and code-to-code tests.
7. **Validation**: the benchmark IDs and issues that check it against
   measurement.
8. **Known traps**: the places where a model is most easily tuned into
   agreement.

When a model is built, its V&V record adds the predicted outcome (written
before the run), the measured numbers with uncertainty, the date and data
version (AGENTS.md §4).

## Gaps these specs found

- **Not on crates.io.** `outram-park-fork-moltres`, `-onix`,
  `-thermochimica` and `-offbeat` hold the molten-salt physics but are not
  among this repository's dependencies, which are published crates only.
- **New physics needed in the backend:** circulating-fuel point kinetics
  (MS-03), salt-specific fission-product removal (MS-04), radiator (MS-07),
  pump curves in TUAS (MS-08), thermochemistry database and redox (MS-09),
  helium embrittlement (MS-10), all of corrosion (MS-11), graphite irradiation
  (MS-12), all of tritium (MS-13), freezing in 1-D TH (MS-14).
- **No benchmark yet** for salt chemistry, graphite, tritium, drain and freeze
  valves, or fission-product distribution. Each spec's issue tracks filing one.
- **The `outram-mc-libs` measured-critical bias** (+2950 ± 61 pcm on
  LEU-COMP-THERM-008, by its own record) must be resolved before MSRE
  criticality comparisons mean anything.
