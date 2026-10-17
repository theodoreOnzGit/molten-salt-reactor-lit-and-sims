# MS-03: Circulating-fuel neutron kinetics

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #45 (part of epic #42)

## Purpose

Predict power and reactivity in time when delayed-neutron precursors are
carried out of the core by the fuel and decay partly in the external loop. This
is the physics that sets MSRE apart from solid-fuelled reactors, and the reason
standard point kinetics is not valid for it.

## Scope

Two fidelities, the cheap one verified against the expensive one:

1. **Spatial**: multigroup diffusion with precursor advection-diffusion-decay
   on a core-plus-loop geometry.
2. **Reduced**: point kinetics with a loop-transit return term, fast enough for
   the real-time plant model (MS-15).

Out of scope: xenon (MS-04), thermal feedback models (supplied by MS-05/MS-06).

## Governing physics (on by default)

Precursor balance per group *i*:

∂C_i/∂t + ∇·(u C_i) − ∇·(D_C ∇C_i) + λ_i C_i = β_i ν Σ_f φ

Reduced form: core precursors lose λ_i C_i + C_i/τ_C and gain the return
(1/τ_C)·C_i(t − τ_L)·e^{−λ_i τ_L}, with core and loop transit times τ_C and
τ_L from MS-05 flow and volumes. Precursor drift is physics, not a correction:
the circulation loss must come out of this transport.

- Effective delayed-neutron fractions from MS-02 importance weighting
  (ORNL-TM-0380 gives the 1962 effective yields as a comparison).
- Transit times from loop volumes and flow at each pump state, varying in time
  when flow varies (MS-08).

## Inputs and sources

| Input | Source |
|---|---|
| Delayed-neutron data, ²³⁵U and ²³³U | ENDF/B-VIII.0; ORNL-TM-2571 for the 1969 ²³³U set as a comparison |
| Effective yields, 1962 calculation | ORNL-TM-0380 |
| Core and loop volumes, flow | ORNL-TM-0728; ORNL-TM-3007 (extended hydraulic model) |
| Improved circulating-fuel kinetics representation | ORNL-TM-4179 |
| Period theory with circulation | ORNL-TM-1626 |

## OUTRAM PARK starting point

- `outram-park-fork-moltres` (unpublished): `CirculatingFuelSolver` on a 1-D
  ring with precursor drift, verified against a static solver at u → 0
  (|Δk| ≈ 2e-16). Its data are "MSRE-like, not evaluated MSRE"
  (`src/circulating.rs`). Steady coupled solve only; about 1.2 s per solve.
- `teh-o-prke`: 6-group PRKE for U-235/U-233, no drift. **The reduced model
  is new physics to add here**, as the backend's MSRE scoping document
  proposes.
- `outram-foam-appbuilder-lib` GeN-Foam (published crate): ~~liquid-fuel
  precursor advection deferred in all four neutronics models~~ **CORRECTED
  2026-10-17 (backend `develop`, `21a93d9`):** the diffusion **eigenvalue**
  has precursor drift (`src/genfoam/neutronics/diffusion/precursor_drift/`),
  code-to-code against upstream GeN-Foam's `2D_MSFR` tutorial at +0.2 pcm on a
  drift worth of −177.8 pcm. Transients, point kinetics, SP3 and S_N still
  defer it. This is a second, independent spatial reference for the
  circulation loss, and the only one in a crate this repository can already
  depend on.
- `outram-mc-libs` (develop): β_eff by k-ratio and adjoint-weighted IFP, and
  Λ (`src/physics/kinetics.rs`, `src/physics/ifp.rs`). A source of
  stationary-fuel β_eff for the reduced model, once a salt-fuelled MC model can
  be built (MS-02 blocker).

## Verification

- T-C1 (#36): reduced model vs the analytical circulating-fuel solution
  (ORNL-TM-4179).
- Reduced model vs the spatial models (moltres ring, GeN-Foam diffusion with
  drift): effective delayed fraction and circulation loss at several flows. The
  two spatial models should first agree with each other on one geometry. State the predicted difference from the
  slug-flow assumption before running.
- Zero-flow limit recovers stationary-fuel point kinetics exactly.
- T-C2 (#37): MSFR benchmark transient steps (Tiberga et al. 2020).

## Validation

N-4 (#5) circulation reactivity loss; T-3 (#31) period with circulating fuel;
T-1 (#29) and T-2 (#30) frequency response (with MS-15 feedback); T-6 (#34)
pump transients once data are found.

## Known traps

- **The measured circulation loss must not be an input.** Subtracting it as a
  constant makes N-4 unable to fail.
- **Loop transit times set the low-frequency response.** Derive them from
  design volumes and flows, not from the T-1 fit.
- **Mixing in the core and heads is not slug flow.** The residence-time
  distribution (MS-06, TH-4) changes the loss; state which is assumed.
- **Three β values circulate in the INL VTB inputs** (2.640e-3, 3.021e-3,
  6.525e-3, per the backend's `docs/reactor-scoping/vtb-findings.md`). Do not
  pick one; derive β_eff from data and say why.
