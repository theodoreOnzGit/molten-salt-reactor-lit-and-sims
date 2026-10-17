# MS-08: Fuel and coolant salt pumps

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #50 (part of epic #42)

## Purpose

Give head and power vs flow and speed for the MSRE fuel and coolant pumps,
and the speed history in start-up and coast-down. Flow sets precursor transit
times (MS-03), so pump transients are reactivity transients.

## Scope

MSRE sump-type centrifugal fuel pump (with its bowl, spray ring, fountain and
xenon-stripping flows) and coolant pump. Steady curves and start-up/coast-down
dynamics.

## Governing physics (on by default)

- Head and torque from homologous curves derived from test data, with
  similarity scaling between speeds and between water and salt stated.
- Rotor inertia and motor torque for start-up and coast-down.
- Internal flows in the fuel pump bowl (spray, fountain, bypass) as separate
  paths in MS-05, because they carry the xenon stripping (MS-04).

## Inputs and sources

| Input | Source |
|---|---|
| Fuel pump water tests, 11-in. and 13-in. impellers | ORNL-TM-0079 (in `validation-data/ai-generated/thermal-hydraulics/`; water, not salt) |
| Prototype and Mark-2 pumps in molten salt at 1200 °F | ORNL-TM-2987 (digitised; two points have ambiguous rpm labels) |
| High-temperature pump experience | ORNL-TM-1993 |
| Pump tank thermal stress | ORNL-TM-0078 (for MS-10) |
| Rotor inertia, motor data | ORNL-TM-0728; ORNL-TM-3039 |

## OUTRAM PARK starting point

`outram-park-fork-dwsim-libs` has pump curves and NPSH. `tampines`
`components/pump` returns `NotYetImplemented`. `tuas_boussinesq_solver` has no
pump-curve component. Build the curve in TUAS, cross-check with DWSIM.

## Verification

- Affinity laws reproduce the curve at another speed exactly for an ideal
  homologous pump (unit test).
- Coast-down vs the analytical inertia-plus-quadratic-friction solution.

## Validation

TH-5 (#16) head-flow curves; T-6 (#34) start-up and coast-down once data are
found (#41).

## Known traps

- **Water and salt curves differ at the same speed** through viscosity and
  Reynolds number. Validate each against its own fluid.
- **The ambiguous rpm points in the ORNL-TM-2987 digitisation** must not be
  silently assigned. Leave them out or show both readings.
