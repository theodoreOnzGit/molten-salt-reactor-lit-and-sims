# Thermal-hydraulics benchmarks

**Status, 2026-10-09: nothing is modelled yet.** Each row becomes a V&V record
with **methodology and results** when it is run (see
[`AGENTS.md`](../AGENTS.md) §4).

Tracking issue: #11

## Validation against measurement

| ID | Quantity | Source (in `literature/`) | Data status |
|---|---|---|---|
| TH-1 (#12) | Forced-convection heat transfer of a molten fluoride salt in a smooth tube (Nu vs Re, Pr), including the transition region | ORNL-TM-4079 | Being transcribed into `validation-data/ai-generated/thermal-hydraulics/` |
| TH-2 (#13) | Heat transfer in a forced-convection loop with two fluoride salts | ORNL-TM-5335 | as TH-1 |
| TH-3 (#14) | MSRE power from heat balances; primary heat exchanger and radiator performance | ORNL-TM-3002 | as TH-1 |
| TH-4 (#15) | MSRE core flow distribution (water-model fluid dynamics) | ORNL-TM-3229 | as TH-1 |
| TH-5 (#16) | MSRE fuel and coolant pump head/flow characteristics | ORNL-TM-0079, ORNL-TM-2987 | as TH-1 |
| TH-6 (#17) | Salt thermophysical properties (density, viscosity, conductivity, heat capacity) | ORNL-TM-2316, ORNL-4831 | Being transcribed into `validation-data/ai-generated/salt-properties/` |

TH-6 is an **input**, not a benchmark result. Property correlations must come
from the literature with their stated uncertainty, and never be adjusted to
improve TH-1 to TH-5.

## Code-to-code verification

| ID | Comparison | Reference |
|---|---|---|
| TH-C1 (#18) | Laminar and turbulent pipe heat transfer against analytical and textbook solutions (Graetz, Dittus–Boelter, Gnielinski) | Analytical and published correlations |
| TH-C2 (#19) | Molten Salt Fast Reactor multiphysics benchmark, steady-state steps (lid-driven cavity with salt, power coupling) | Tiberga et al., *Annals of Nuclear Energy* 142 (2020) 107428 |

## Known traps

- **Use the correlation's validity range.** ORNL-TM-4079 spans laminar,
  transition and turbulent flow (its text quotes runs at Re = 597, 4,277 and
  28,104, and separate fits below Re 1,000 and above Re 12,000). Turbulent
  correlations are not valid in the transition band. Report the Re range of
  each comparison.
- **Wall-temperature and property-variation corrections** (μ_b/μ_w) are part
  of the physics. Apply them by default; ablate them visibly.
