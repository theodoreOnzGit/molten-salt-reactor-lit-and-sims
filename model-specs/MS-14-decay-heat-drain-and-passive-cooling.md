# MS-14: Decay heat, drain tanks, freeze valves and passive heat removal

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #56 (part of epic #42)

## Purpose

Predict afterheat after shutdown and where it is deposited in a circulating
fuel system (salt, graphite, off-gas, metal), the thaw and freeze of the
freeze valves, the drain of the fuel to the drain tanks and their heat
removal, and natural circulation in the loops. These set the passive-safety
case of a molten salt reactor.

## Scope

MSRE: fuel drain tanks with bayonet-tube cooling, freeze valves, fill and drain
operations, filling accidents; natural circulation in fuel and coolant loops.
Salt freezing in pipes and the radiator.

## Governing physics (on by default)

- Decay heat from the MS-04 inventory, not from a solid-fuel decay-heat
  standard. A standard (ANS) curve may be an ablation for comparison.
- Decay heat split by location, because noble gases and metals leave the salt.
- Freeze-valve thaw and freeze: conduction with phase change, latent heat from
  MS-01, cooling air and heaters.
- Drain-tank heat removal by boiling water in bayonet tubes; radiation in
  heated cells.
- Natural circulation from MS-05.

## Inputs and sources

| Input | Source |
|---|---|
| Freeze valve development | ORNL-TM-0128 |
| Safety calculations, filling accidents | ORNL-TM-0251, ORNL-TM-0497 |
| Safety analysis report, ²³⁵U and ²³³U | ORNL-TM-0732, ORNL-TM-2111 |
| Operating limits | ORNL-TM-0733 and revisions |
| Calculated fuel radioactivity | ORNL-TM-2970 (code-to-code only) |
| Afterheat radiation transfer in HX | ORNL-TM-3145 |
| Drain tank and systems performance | ORNL-TM-0728, ORNL-TM-3039 |

## OUTRAM PARK starting point

- `teh-o-prke` 23-group 1978 draft-ANS decay heat: a solid-fuel ablation
  reference only.
- `outram-foam-basic-lib` enthalpy-porosity solidification source and
  `outram-foam-appbuilder-lib` meltFoam with a Stefan-problem verification.
- `tuas_boussinesq_solver` natural circulation, validated on CIET/DRACS.
  It has the FLiBe freezing point but no freezing model; the `fhr_sim_v2`
  example notes the solver crashes when salt freezes. On the backend's
  `develop` branch, `fhr_sim_v2` gained a `salt_freeze_guard.rs` that pauses
  the run below the property bound and offers a "melt" its own doc calls a
  cheat. It is a guard, not a solidification model.

## Verification

- Stefan problem for the freeze-valve plug (reuse the meltFoam case).
- Decay heat from the inventory vs ORNL-TM-2970 at the same history.
- Drain-tank energy balance.

## Validation

T-7 (#35) natural circulation; freeze-valve tests in ORNL-TM-0128. Its summary
says only that the valves had "adequate freeze and thaw times"; whether it
tabulates measured times has not been checked.
**No drain or freeze-valve benchmark issue exists yet; this spec's issue tracks
filing one.**

## Known traps

- **A solid-fuel decay-heat curve overstates in-salt heat** because volatile
  and noble-metal fission products leave the salt.
- **A 1-D solver that crashes on freezing** cannot be used for a freeze
  transient. Fix the solver, do not clamp temperature above liquidus.
