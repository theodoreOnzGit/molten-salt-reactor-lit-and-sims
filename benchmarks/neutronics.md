# Neutronics benchmarks

**Status, 2026-10-09: nothing is modelled yet.** This page lists the
benchmarks to build and where their measured data come from. Each row becomes
a V&V record with **methodology and results** when it is run (see
[`AGENTS.md`](../AGENTS.md) §4).

Tracking issue: #1

## Validation against measurement (MSRE)

| ID | Quantity | Source (in `literature/`) | Data status |
|---|---|---|---|
| N-1 (#2) | First criticality, ²³⁵U loading: critical uranium concentration, excess reactivity | ORNL-4233 (Prince et al. 1968) | Being transcribed into `validation-data/ai-generated/neutronics/` |
| N-2 (#3) | Control-rod worth and calibration curves (period and rod-bump methods) | ORNL-4233 | as N-1 |
| N-3 (#4) | Isothermal temperature coefficient of reactivity | ORNL-4233, ORNL-TM-0379 | as N-1 |
| N-4 (#5) | Reactivity loss from fuel circulation (delayed-neutron precursor drift) | ORNL-4233, ORNL-TM-1626 | as N-1 |
| N-5 (#6) | ²³³U zero-power experiments: critical loading, rod worth, temperature coefficient | ORNL-TM-3963 | as N-1 |
| N-6 (#7) | Reactivity balance over power operation (burn-up, samarium, xenon) | ORNL-TM-1796 | as N-1 |
| N-7 (#8) | Xenon poisoning vs power and helium void fraction | ORNL-TM-3464 | as N-1 |

**The reference for N-1 to N-5 is the measurement, not a code.** The
evaluated MSRE critical benchmark in the IRPhEP handbook (MSRE first
criticality, 2019 edition) is the best-documented model specification. It is
distributed under the OECD/NEA handbook terms and may be **cited but not
copied** here. Inputs built from it must be derived from its published
description, with the citation.

## Code-to-code verification

| ID | Comparison | Reference | Notes |
|---|---|---|---|
| N-C1 (#9) | k_eff of a simplified MSRE geometry, OUTRAM PARK `outram-mc-libs` vs OpenMC | OpenMC with the same ENDF/B-VIII.0 library | Certifies transport and data processing, not the physics model |
| N-C2 (#10) | Cross-section processing for the salt nuclides: `njoy-outram-park-fork` vs NJOY2016 | NJOY2016 | Be-9, F-19 and Li-6 already appear in the backend's NJOY comparison tests (`pendf_stages_vs_njoy2016.rs`, `ace_reconstructed_total_light_nuclides.rs`, checked 2026-10-09). Li-7 and the Zr isotopes were not found there; extend those tests rather than writing new ones |

## Known traps

- **Precursor drift is physics, not a correction.** The circulating-fuel
  reactivity loss (N-4) must come out of the model's delayed-neutron transport.
  Do not subtract a measured loss as a fitted constant.
- **Graphite and salt temperatures differ.** The MSRE coefficient (N-3) is
  isothermal: compare against an isothermal calculation.
- **Thermal scattering data.** Graphite S(α,β) is applied by default. Report
  an ablation without it, not the other way round.
