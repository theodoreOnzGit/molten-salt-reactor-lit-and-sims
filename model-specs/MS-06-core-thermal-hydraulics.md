# MS-06: Core thermal hydraulics and flow distribution

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #48 (part of epic #42)

## Purpose

Resolve flow and temperature inside the reactor vessel: the volute and
downcomer, the lower head, the graphite channels and the upper head. It gives
graphite and salt temperatures for reactivity feedback, hot-channel factors,
the residence-time distribution that MS-03 needs, and the graphite heating that
MS-12 needs.

## Scope

Two fidelities: a channel model (parallel graphite channels with radial power
shape from MS-02), and CFD of the vessel (porous-core or resolved). Out of
scope: two-phase helium-bubble transport beyond a void-fraction field.

## Governing physics (on by default)

- Incompressible flow with buoyancy; turbulence model chosen for the
  Re in each region and stated.
- Conjugate heat transfer between salt and graphite, with gamma and neutron
  heating deposited in graphite (fraction from MS-02).
- Property variation from MS-01.

## Inputs and sources

| Input | Source |
|---|---|
| Vessel and core geometry | ORNL-TM-0728 |
| Core flow distribution, water model | ORNL-TM-3229 (velocities, head losses, lower-head pressure; in `validation-data/ai-generated/thermal-hydraulics/`) |
| Steady-state core temperatures, calculated | ORNL-TM-0378 |
| Temperature and reactivity-coefficient averaging | ORNL-TM-0379 |
| Graphite heating fraction | ORNL-TM-0730 |

## OUTRAM PARK starting point

`outram-foam-basic-lib`, `-turbulence-lib` (k-ω SST, k-ε) and
`-appbuilder-lib` (pimpleFoam, GeN-Foam one-phase porous solver), none
benchmark-validated. GeN-Foam porous solver uses constant properties. The INL
VTB MSRE lower-plenum CFD input is in the backend's
`reference-data/virtual_test_bed/` as a code-to-code reference (CC-BY-4.0).

## Verification

- TH-C2 (#19): MSFR benchmark steady-state steps (lid-driven cavity with salt).
- Mesh convergence on the quantities compared, with the observed order.
- Channel model vs CFD for the same power shape: mixed-mean outlet temperature.

## Validation

TH-4 (#15) core flow distribution against the ORNL-TM-3229 water model, in
water, at the water test's Re. Salt-condition predictions are then an
extrapolation, stated as such.

## Known traps

- **The water model is not salt.** Validate at the water test conditions; do
  not carry a water-matched coefficient to salt without a similarity argument.
- **The graphite-salt temperature difference drives the power coefficient.**
  Report it with its uncertainty; it is not a free parameter.
