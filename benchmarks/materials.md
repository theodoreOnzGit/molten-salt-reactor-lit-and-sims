# Materials benchmarks

**Status, 2026-10-09: nothing is modelled yet.** Each row becomes a V&V record
with **methodology and results** when it is run (see
[`AGENTS.md`](../AGENTS.md) §4).

Tracking issue: #20

## Validation against measurement (Hastelloy N / INOR-8)

| ID | Quantity | Source (in `literature/`) | Data status |
|---|---|---|---|
| M-1 (#21) | Tensile and creep-rupture properties of unirradiated INOR-8 | ORNL-TM-1017 | Being transcribed into `validation-data/ai-generated/materials/` |
| M-2 (#22) | Irradiation embrittlement: creep-rupture life and ductility of MSRE surveillance specimens vs fluence | ORNL-TM-1997, -2359, -2647, -3063 | as M-1 |
| M-3 (#23) | Post-irradiation examination of MSRE components | ORNL-TM-4174 | as M-1 |
| M-4 (#24) | Tellurium intergranular cracking: crack depth and frequency | ORNL-4829 | as M-1 |
| M-5 (#25) | Corrosion mass transfer in thermal-convection and forced-circulation loops (weight change vs time and temperature) | ORNL-TM-4188, ORNL-TM-3866, ORNL-4575 | Not started |

## Model checks

| ID | Check | Reference |
|---|---|---|
| M-C1 (#26) | Chromium solid-state diffusion corrosion model vs the analytical diffusion-limited solution | ORNL-4575 (Evans et al. 1971) |
| M-C2 (#27) | Larson–Miller fit of M-1 rupture data, reported **with** an ablation of the fitted constant | M-1 data |

## Known traps

- **Helium embrittlement depends on thermal fluence and boron content.** The
  heat number and its boron content belong in every M-2 comparison.
- **A fitted creep law is a calibration.** It must follow the
  uncalibrated → calibrated → ablation order in [`AGENTS.md`](../AGENTS.md) §2.
