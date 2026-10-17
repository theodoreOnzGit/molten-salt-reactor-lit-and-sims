# MS-07: Primary heat exchanger and salt-to-air radiator

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #49 (part of epic #42)

## Purpose

Model heat transfer from fuel salt to coolant salt in the primary heat
exchanger, and from coolant salt to air in the radiator, including the radiator
doors and blowers that were the MSRE's actual load-control element.

## Scope

MSRE U-tube shell-and-tube primary heat exchanger; MSRE radiator tube bank
with doors and blowers. The forced-convection tube tests of ORNL-TM-4079
(MSBR fuel salt) and ORNL-TM-5335 (MSBR fuel salt and NaBF₄-NaF coolant) are
the separate-effects base for the salt-side coefficients. Neither tested the
MSRE salts, so carrying their fits to MSRE conditions is an extrapolation in
Pr and must be stated.

## Governing physics (on by default)

- Tube-side and shell-side heat transfer from correlations valid for the local
  Re and Pr, with the μ_b/μ_w correction, laminar and transition included.
- Wall conduction through Hastelloy-N (k from MS-10).
- Fouling resistance only if a measured value exists, stated as such.
- Radiator: air-side correlation for the tube bank, air flow from blower
  curves and door position, radiation from hot tubes to the enclosure.
- Thermal inertia of tubes and shells.

## Inputs and sources

| Input | Source |
|---|---|
| HX and radiator design | ORNL-TM-0728 |
| HX overall U history, radiator U vs air ΔP, heat balances | ORNL-TM-3002 (in `validation-data/ai-generated/thermal-hydraulics/`) |
| Salt forced-convection heat transfer in a smooth tube | ORNL-TM-4079 |
| Two-salt forced-convection loop | ORNL-TM-5335 |
| Tube plugging, vibration | ORNL-TM-1023, ORNL-TM-2098 |
| MSBR HX design methods (code-to-code only) | ORNL-TM-2815, ORNL-TM-2952 |

## OUTRAM PARK starting point

- `tuas_boussinesq_solver` `SimpleShellAndTubeHeatExchanger`, validated by
  the maintainer against a HITEC/YD-325 heat exchanger (Du et al. 2018). The
  base for the primary HX.
- **No radiator exists.** The backend scoping document suggests building on
  TUAS's air-cooled tube-bank component.
- `outram-park-fork-dwsim-libs` has LMTD and ε-NTU for cross-checks.

## Verification

- ε-NTU analytical solution for counter-flow and the U-tube geometry.
- TH-C1 (#18) for the tube-side correlation.

## Validation

TH-1 (#12) and TH-2 (#13) for the salt-side coefficient; TH-3 (#14) for MSRE
HX and radiator performance.

## Known traps

- **The ORNL-TM-3002 measured U values assume a nominal 8.0 MW** (report
  p. 24). They are tied to the heat-balance power, so a U comparison and a
  power comparison are not independent. The report gives measured and
  calculated U side by side; validate against the measured column only.
- **Transition-region Nu.** ORNL-TM-4079 gives separate fits below Re 1,000
  and above Re 12,000. A turbulent correlation is not valid between them.
- **Do not fit an HX multiplier to TH-3.** A miss in U goes in the record.
