# MS-11: Corrosion and tellurium intergranular cracking

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #53 (part of epic #42)

## Purpose

Predict wall attack by the salt: chromium depletion and mass transfer from hot
to cold legs, and the shallow intergranular cracking by fission-product
tellurium found in MSRE Hastelloy N.

## Scope

Hastelloy N and stainless steels in fluoride salts (thermal-convection and
forced-circulation loops, MSRE); tellurium cracking depth and frequency vs
time, temperature and redox. Fluoroborate-salt corrosion is included as test
data but is not an MSRE condition.

## Governing physics (on by default)

- Chromium solid-state diffusion in the alloy coupled to the salt redox
  potential (MS-09) and to the temperature profile around the loop
  (MS-05): dissolution in hot legs, deposition in cold legs.
- Liquid-film mass transfer as a series resistance, with its own ablation
  switch (`without_liquid_film`) as in ORNL-4575.
- Tellurium supply from MS-04 fission-product deposition; crack growth
  dependence on redox as reported in ORNL-TM-6002.

## Inputs and sources

| Input | Source |
|---|---|
| Solid-state diffusion mechanism with and without liquid film | ORNL-4575 (Evans, Koger, DeVan 1971) |
| Thermal-convection loop with FeF₂ addition | ORNL-TM-4188 |
| NaBF₄-NaF loops | ORNL-TM-3866, ORNL-TM-4221, ORNL-TM-4271, ORNL-TM-3863 |
| Corrosion behaviour in fluoride salts, general | ORNL-TM-0328, ORNL-TM-0202, ORNL-TM-4286, ORNL-TM-5783 |
| Forced-circulation test loop design | ORNL-TM-0528, ORNL-TM-5540 |
| 316 SS in Li₂BeF₄ | ORNL-TM-5782 |
| Tellurium cracking in MSRE | ORNL-4829 |
| Tellurium-Hastelloy studies, redox effect | ORNL-TM-6002 |
| In-reactor convection loops | ORNL-TM-1960 |

## OUTRAM PARK starting point

**Absent.** `outram-park-fork-offbeat` `corrosion` is Zircaloy waterside
oxidation only. This is new physics, written from ORNL-4575 with the
equations cited.

## Verification

M-C1 (#26): chromium diffusion model vs the analytical diffusion-limited
solution (ORNL-4575). Mass conservation of Cr around a closed loop.

## Validation

M-5 (#25) weight change vs time and temperature in loops; M-4 (#24)
tellurium crack depth and frequency.

## Known traps

- **Corrosion rate is set by redox, not by the alloy alone.** A model run at
  the wrong U⁴⁺/U³⁺ ratio can match a weight-loss curve for the wrong reason.
- **Fluoroborate data are a different chemistry.** Do not pool them with
  LiF-BeF₂ data in one fit.
- **Effective diffusivity is the classic fitted constant.** Take it from
  independent diffusion measurements first and report the uncalibrated miss.
