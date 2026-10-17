# MS-05: Primary and secondary loop thermal hydraulics (1-D system)

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #47 (part of epic #42)

## Purpose

A 1-D network model of the MSRE fuel loop and coolant loop: flow, pressure
drop, temperatures and transit times at steady state and in transients. It
gives MS-03 its transit times, MS-15 its thermal feedback and MS-14 its
natural-circulation path.

## Scope

Fuel loop: reactor vessel (downcomer, lower head, core channels, upper head),
fuel pump bowl, primary heat exchanger shell side, piping. Coolant loop:
heat exchanger tube side, radiator, coolant pump, piping. Forced and natural
circulation. Components are specified separately in MS-07 (heat exchangers)
and MS-08 (pumps).

## Governing physics (on by default)

- Mass, momentum and energy conservation per control volume, with the
  Boussinesq approximation only where its validity is shown for the salt and
  temperature range.
- Friction from the correlation valid for the local Re, including laminar and
  transition bands; form losses from design data.
- **Variable properties** from MS-01, including the μ_b/μ_w wall correction.
  Ablation switch: `without_wall_viscosity_correction`.
- Heat losses to the cell through insulation, with the electric heaters as
  a source.
- Thermal inertia of pipe walls and vessel metal.

## Inputs and sources

| Input | Source |
|---|---|
| Loop geometry, pipe sizes, elevations, volumes | ORNL-TM-0728 |
| Extended hydraulic model of the fuel system | ORNL-TM-3007 |
| Coolant-system head loss by component, **calculated** at 850 gpm (a 1970 model result, not a measurement) | ORNL-TM-3002 Table II (in `validation-data/ai-generated/thermal-hydraulics/`) |
| Core head loss, water model | ORNL-TM-3229 |
| Systems and components performance | ORNL-TM-3039 |
| Temperatures at steady power | ORNL-TM-0378 (calculated) |

## OUTRAM PARK starting point

`tuas_boussinesq_solver` is the human-validated base: pipe networks in series
and parallel, natural circulation, Churchill friction, Gnielinski and laminar
Nusselt, insulated pipes, validated against CIET. Its UW-Madison FLiBe loop is
the closest template but is `#[cfg(test)]`-only, not public API. No MSRE loop
exists. Use `CustomLiquid` (MS-01) for the salts.

## Verification

- TH-C1 (#18): pipe heat transfer vs Graetz, Dittus–Boelter, Gnielinski.
- Energy balance to round-off in a closed loop at steady state.
- Transit times from the network vs volume/flow by hand.

## Validation

TH-3 (#14) heat balances; T-7 (#35) natural circulation once data are found.
The ORNL-TM-3002 coolant head-loss table is a code-to-code reference for the
1970 calculation only. Measured loop pressure drops still need locating
(ORNL-TM-3039 is the first place to look).

## Known traps

- **ORNL-TM-3002 power is itself a heat balance.** Do not validate power
  against a number derived from the same balance the model computes.
- **Bypass flows** (downcomer to upper head, fountain flow in the pump bowl)
  change transit times. Represent them from ORNL-TM-3007 and the water tests;
  do not tune them to match T-1.
- **TUAS form-loss recalibration history** (pipe 38, K 0.8 → 17.8) shows how
  easily a K becomes a fit. Every K needs a source.
