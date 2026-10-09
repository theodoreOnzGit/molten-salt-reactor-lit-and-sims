# Transient and dynamics data

**Status, 2026-10-09: nothing is modelled yet.** Each row becomes a V&V record
with **methodology and results** when it is run (see
[`AGENTS.md`](../AGENTS.md) §4).

Tracking issue: #28

## Validation against measurement (MSRE)

| ID | Quantity | Source (in `literature/`) | Data status |
|---|---|---|---|
| T-1 (#29) | Frequency response (power/reactivity amplitude and phase) at several power levels, ²³⁵U | ORNL-TM-1647 | Being transcribed into `validation-data/ai-generated/transients/` |
| T-2 (#30) | Frequency response with ²³³U fuel, measured vs predicted | ORNL-TM-2997, ORNL-TM-2571 | as T-1 |
| T-3 (#31) | Reactor period with circulating fuel | ORNL-TM-1626 | as T-1 |
| T-4 (#32) | Helium void fraction from neutron noise | ORNL-TM-2315 | as T-1 |
| T-5 (#33) | Aircraft Reactor Experiment operation: power and temperature transients | ORNL-1845 | as T-1 |
| T-6 (#34) | Fuel-pump start-up and coast-down (flow transients and the reactivity they cause) | Search the ORNL MSRE operation reports | Not located yet |
| T-7 (#35) | Natural-circulation heat removal | Search the ORNL MSRE operation reports | Not located yet |

## Code-to-code verification

| ID | Comparison | Reference |
|---|---|---|
| T-C1 (#36) | Point kinetics with precursor drift vs the analytical circulating-fuel solution | ORNL-TM-4179. OUTRAM PARK `teh-o-prke` has stationary-fuel point kinetics but no circulating-fuel precursor drift (searched its `src/` 2026-10-09), so this is new physics to add, not a reuse |
| T-C2 (#37) | Molten Salt Fast Reactor multiphysics benchmark, transient steps (power-coupling perturbation) | Tiberga et al., *Annals of Nuclear Energy* 142 (2020) 107428 |

## Known traps

- **The MSRE's predicted frequency response came from a model.** T-2 has both
  measured and predicted columns. Validate against the **measured** column;
  the predicted one is a code-to-code reference for the 1969 ORNL model only.
- **Precursor transit times in the external loop set the low-frequency
  response.** Derive loop volumes and flow rates from the design reports, not
  from the fit.
