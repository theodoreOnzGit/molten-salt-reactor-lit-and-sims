# MS-10: Structural alloy (Hastelloy N / INOR-8) mechanical behaviour

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #52 (part of epic #42)

## Purpose

Predict tensile, creep and creep-rupture behaviour of Hastelloy N before and
after irradiation, and the stresses in vessel, pump tank and heat-exchanger
components. Lifetime of the metal boundary is a design limit for any molten
salt reactor.

## Scope

Unirradiated and irradiated INOR-8 / Hastelloy N, including titanium-modified
variants; component stress for the reactor vessel, pump tanks and HX.
Out of scope: ASME III-5 design-by-rule (a later step once the material model
exists), welding.

## Governing physics (on by default)

- Elastic-plastic response with temperature-dependent properties.
- Creep by a law derived from the ORNL data, run **uncalibrated first**
  (from published constants), then calibrated with the parameters, ranges and
  data stated, then ablated (AGENTS.md §2).
- Helium embrittlement from thermal-neutron ¹⁰B(n,α) and Ni reactions: loss
  of rupture life and ductility as a function of thermal fluence and boron
  content.
- Thermal stress from MS-05/MS-06 temperature fields.

## Inputs and sources

| Input | Source |
|---|---|
| Tensile and creep, unirradiated | ORNL-TM-1017 (transcribed and AI-audited; human review #38) |
| Surveillance specimens, groups 1–4 | ORNL-TM-1997 (partly transcribed), ORNL-TM-2359, -2647, -3063 (not transcribed) |
| Post-irradiation examination | ORNL-TM-4174 |
| Modified alloys (Ti, Zr, Hf) | ORNL-TM-3064, ORNL-TM-3321 |
| Welded after irradiation | ORNL-TM-2858 |
| Pump tank thermal stress and fatigue | ORNL-TM-0078 |
| Nine years' exposure to salt and air | ORNL-TM-4189 |
| Elastic and thermal properties | Correlations in `outram-park-fork-offbeat`, each traced to its upstream reference |

## OUTRAM PARK starting point

- `outram-park-fork-offbeat` (unpublished): Hastelloy-N conductivity,
  thermal expansion, Young's modulus and void swelling (the swelling goes
  negative below about 1 dpa, as upstream does; check before use). Norton,
  Chaboche and code_aster creep laws. Hastelloy creep (Zhang) not ported.
- `farrer-park`: FEM elasticity and J2 plasticity, verified by MMS; no creep,
  no material library.
- No helium embrittlement model anywhere.

## Verification

- `farrer-park` patch and MMS tests on the component geometry.
- Creep law integrates to the analytical strain-time curve at constant stress.
- M-C2 (#27): Larson–Miller fit reported with an ablation of the fitted
  constant.

## Validation

M-1 (#21) unirradiated tensile and creep-rupture; M-2 (#22) irradiated
surveillance specimens; M-3 (#23) post-irradiation examination.

## Known traps

- **Heat number and boron content belong in every M-2 comparison.**
- **A fitted creep law is a calibration**, so report it as one.
- **The Haiku transcription invented values for blank cells** before audit
  (`validation-data/ai-generated/AUDIT-2026-10-09.md`). Use only rows a human
  has checked.
