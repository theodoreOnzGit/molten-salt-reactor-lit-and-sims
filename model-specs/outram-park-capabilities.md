# What OUTRAM PARK offers a molten salt reactor model

> **AI-generated survey, not human-reviewed.** Written 2026-10-17 by Claude.
> **Baseline: the backend's `develop` branch at commit `21a93d9`
> (2026-10-17 22:48 +0800).** Each claim names the file it was read from, so it
> can be checked. Crate versions are those in each crate's `Cargo.toml` on
> `develop`; they match the crates.io versions this repository pins.
>
> ~~The first version surveyed `main` at commit `8d31482`.~~ **CORRECTED
> 2026-10-17:** `main` was last updated 2026-09-14 and is about 490 commits
> behind `develop`, so that survey was a month stale. Every claim was rechecked
> against `develop`; claims that changed are struck through below with the new
> reading beside them.

## Headline

- **Only one crate has human V&V sign-off at crate level:
  `tuas_boussinesq_solver`** (README bookkeeping block, "Cleared by maintainer
  (2026-07-21)", validated against CIET and published as
  doi:10.1016/j.jandt.2025.03.006). Every other crate below is unreviewed
  AI-assisted draft material by its own README. On `develop`, `boon-lay`
  additionally carries 31 function-level reviews signed by the maintainer in
  `src/fuel_failure/review.md` (rung 3, 2026-10-13); they cover TRISO fuel
  failure, not anything this repository needs.
- **No crate is compared against MSRE measured data.** Searches for ORNL-4233,
  "zero-power physics" and Prince/Haubenreich across `develop` found only
  bibliography entries.
- **The molten-salt-specific physics lives mostly in crates that are not on
  crates.io**: `outram-park-fork-moltres` (precursor drift), `-onix`
  (depletion), `-thermochimica` (Gibbs minimisation) and `-offbeat`
  (Hastelloy-N correlations). This repository depends only on published crates
  ([`Cargo.toml`](../Cargo.toml), list checked 2026-10-09; crates.io could not
  be reached from the 2026-10-17 session to recheck). Using them needs either a
  release or a git dependency. The one exception: GeN-Foam diffusion in the
  published `outram-foam-appbuilder-lib` now has steady liquid-fuel precursor
  drift (below).
- The backend's own MSRE plan is
  [`docs/reactor-scoping/msre.md`](https://github.com/theodoreOnzGit/outram-park-backend/blob/develop/docs/reactor-scoping/msre.md)
  (audit 2026-08-06, unchanged on `develop`). Its "missing" list agrees with
  this survey: a real-time circulating-fuel point-kinetics model, MSRE
  fuel-salt properties, an air-cooled radiator, the secondary salt loop, xenon
  stripping and precursor-borne decay heat, and MSRE geometry and kinetics
  data.

## By reactor-design aspect

State key: **working** = implemented and unit-tested (verification only unless
stated); **partial** = some of the needed physics; **scaffold** = types or
stubs without physics; **absent** = not found.

| Aspect | Crate (path under `crates/`) | State | What is there | What a molten salt model still needs |
|---|---|---|---|---|
| Salt properties | `tuas_boussinesq_solver` 0.1.7 (`…/liquid_database/`) | working, human-validated crate | FLiBe and FLiNaK (Romatoski & Hu 2017; Sohal et al. 2010), HITEC, Dowtherm A / Therminol VP-1, YD-325; `CustomLiquid` escape hatch | MSRE fuel salt LiF-BeF₂-ZrF₄-UF₄ and coolant salt from ORNL-TM-2316; uncertainties carried with each correlation |
| | `outram-park-fork-coolprop` | working | 137 Helmholtz fluids, ~~127~~ 126 incompressibles (README, `develop`) incl. Therminol, NaK | No molten salts |
| Steady neutronics (MC) | `outram-mc-libs` 0.0.5 | working | CSG k-eigenvalue, delta tracking, S(α,β), tallies, CRAM16 depletion. **On `develop`:** delayed-neutron energies drawn from the delayed group spectra since 2026-10-03 (`src/physics/fission.rs:24-29`); β_eff by the k-ratio route and Λ (`src/physics/kinetics.rs`); OpenMC's adjoint-weighted IFP ported (`src/physics/ifp.rs`) | No time-dependent precursor transport (delayed neutrons still born at the fission instant). ~~Measured-critical bias +2950 ± 61 pcm on LEU-COMP-THERM-008 by its own record~~ **CORRECTED 2026-10-17:** that bias was a geometry defect (surface normals computed about the wrong centre inside translated lattices), fixed by `Geometry::cross_surface_in_frame`; paired A/B −2304 ± 267 pcm (`examples/lct008_keff.rs:59-86`). Latest full 35-nuclide model, case 1: +48 to +67 ± ~20 pcm vs benchmark k 1.0007, with OpenMC on the same deck at +98 ± 23 (`verification_and_validation/icsbep/five_route_keff/data/lct008_full_five_routes_2026_10_14.md`). The ±600 pcm benchmark band used there is a stand-in; the ICSBEP case uncertainty is still unobtained. Godiva still reads +1107 pcm in `verification_and_validation/gpu_cpu_benchmarks.md` (LOW tier). **`FuelForm::MoltenSalt` exists but fails to resolve as unsupported** (`src/model/material.rs:194-200`, upstream issue #937), so no salt-fuelled model can be built yet |
| Nuclear data | `njoy-outram-park-fork` 0.0.4 | working | RECONR, BROADR, THERMR, ACER, GROUPR and more, code-to-code against NJOY2016 | ~~Li-7 and Zr isotopes not in the NJOY comparison tests~~ **CORRECTED 2026-10-17:** Li-7 is in the 57-tape 0 K ACER comparison against NJOY2016 (`verification_and_validation/acer_ce_vs_njoy2016_multi_nuclide.md:378`), a V&V document, not a gated test. Zr is still absent: no Zr tape exists (issue #10) |
| | `reference-data/endf/` (workspace folder) | data | ENDF/B-VIII.0 tapes incl. Li-6, Li-7, Be-9, F-19, C-12, U-234/235/238; graphite S(α,β). **On `develop`:** 89 tapes (from 41), adding Cr-50/52/53/54 and Fe-54/56/57; Mo-95 present | ~~Zr, Mo, Ni, Cr, Fe tapes not checked~~ **No Zr and no Ni tapes** — both needed for MSRE fuel salt and Hastelloy N |
| Deterministic neutronics | `outram-foam-appbuilder-lib` 0.1.4 (`src/genfoam/neutronics/`) | partial | Diffusion and SP3 eigenvalue + transient, S_N eigenvalue, Doppler/expansion/coolant-density feedback | ~~Liquid-fuel precursor advection deferred in all four models~~ **CORRECTED 2026-10-17:** the diffusion **eigenvalue** now has precursor drift (`diffusion/precursor_drift/mod.rs`, wired by `set_precursor_drift`), code-to-code against upstream GeN-Foam's `2D_MSFR` tutorial at +0.2 pcm on a drift worth of −177.8 pcm (`tests/genfoam_tutorial_keff.rs:1066-1072`). Still deferred: diffusion transients, point kinetics, SP3, S_N |
| | `bedok` | partial, `publish = false` | 3-D SANM nodal diffusion. **On `develop`:** TH 9/9 files and coupling 6/6; NEACRP A2 and D1 match the MATLAB reference (`README.md:34-38,209-211`) | PWR/BWR-oriented. ~~TH coupling 1/9 files, feedback 0/5~~ |
| Circulating-fuel neutronics | `outram-park-fork-moltres` | working, unpublished | Multigroup diffusion eigenvalue, precursor advection-diffusion-decay, 1-D core-plus-loop ring, salt energy equation with linear feedback | Data are "MSRE-like, not evaluated MSRE" (`src/circulating.rs:329-333`); prescribed velocity; steady coupled solve only; 287 pcm at 0.6 m/s compared only in order to the measured ~212 pcm; still no dependent crate (unchanged on `develop`) |
| Point kinetics | `teh-o-prke` 0.1.6 | working | 6-group PRKE (U-235, U-233, Pu-239), Nordheim-Fuchs, 23-group ANS decay heat, Xe/I solver, linear fuel-temperature feedback; 5-group set rescalable to a given total β (`delayed_neutron_layer.rs`, new on `develop`) | No precursor drift / loop return term (searched `src/` on `main` and `develop`) |
| | `nee_soon` 0.0.3 | partial | ~~Pass-through to Nordheim-Fuchs~~ On `develop`: HTR-10/HTTR MC, MC-to-GeN-Foam coupling, MGXS, plus the Nordheim-Fuchs pass-through | Xin Wang workflow still returns `NotYetImplemented`; nothing MSR-specific |
| Depletion | `outram-park-fork-onix` | working, unpublished | Bateman matrix, CRAM16, verified against closed-form Bateman | No nuclide libraries, no salt-specific removal terms (off-gas, plating) |
| | `outram-mc-libs` (`src/depletion/`) | working | CRAM16, chain, predictor burnup | as above |
| Fission-product transport | `boon-lay` | working for TRISO | Lagrangian decay and diffusion, Booth release; on `develop` also graphite oxidation and kernel hydrolysis chemistry, fuel failure, TRISO accident modules | FHR/TRISO only; no salt, gas or graphite pathways for an MSR |
| Loop thermal hydraulics (1-D) | `tuas_boussinesq_solver` | working, human-validated | Pipe networks (series/parallel), natural circulation, Churchill friction, Gnielinski/laminar Nusselt, shell-and-tube HX, porous media, CIET/DRACS cases | No MSRE loop, no radiator, no pump curves, no salt freezing (unchanged on `develop`). UW-Madison FLiBe loop still `#[cfg(test)]`-only |
| | `tampines` | partial | Pebble bed, gas phase, drift-flux and two-fluid 1-D; FHR GUI example; on `develop` a working helical-coil once-through steam generator | Pump, valve, HX components return `NotYetImplemented` |
| | `outram-park-fork-dwsim-libs` | working | LMTD / ε-NTU HX, pump curves and NPSH, valves | No salts |
| Core CFD | `outram-foam-basic-lib`, `-turbulence-lib`, `-appbuilder-lib` | working | FV operators, k-ω SST, k-ε, pimpleFoam, enthalpy-porosity melting source, meltFoam (`src/solvers/melt_foam/`) | Not benchmark-validated; no salt properties wired in |
| Salt freezing | `outram-park-digital-twin-engine` (`examples/fhr_sim_v2/…/salt_freeze_guard.rs`, new on `develop`) | guard only | Pauses the run when salt falls below its property bound and offers a "melt" its own doc calls a cheat | Not a solidification model; MS-14 still needs one |
| Process control | `chem-eng-real-time-process-control-simulator` | working | PID, transfer functions, z-domain | No bookkeeping block |
| Salt thermochemistry | `outram-park-fork-thermochimica` | partial, unpublished | Gibbs minimiser, ideal and binary Redlich-Kister solutions, 6 analytic tests | No database parser (so no MSTDB-TC), no redox, speciation or solubility workflow (`src/gem.rs:166-168`; no source change on `develop`) |
| Structural alloy | `outram-park-fork-offbeat` | partial, unpublished | Hastelloy-N conductivity, expansion, Young's modulus, void swelling; Norton/Chaboche creep laws | Hastelloy creep (Zhang) not ported; no helium embrittlement, no irradiated creep-rupture |
| | `farrer-park` | working | FEM elasticity, J2 plasticity, crystal plasticity; on `develop` a PRISMS-Fatigue indicator port (calibrated for Al7075) | No creep, no ASME III-5, no material library |
| Corrosion | — | absent | `offbeat` corrosion is Zircaloy waterside only | Cr depletion, Te cracking, all salt corrosion |
| Graphite | `tuas_boussinesq_solver` | partial | Nuclear graphite thermal properties as a solid | No irradiation dimensional change, gas or salt penetration |
| Tritium | — | absent | Tritium appears on `develop` only in HTR-10/LWR dose and dispersion code (`buangkok`, `changi`) | All production and transport |
| Uncertainty | `raffles` 0.0.1 | partial | 9 distributions, MC / LHS / grid samplers, Pearson, Spearman, Sobol indices; on `develop` also Bayesian, ABC, model selection, fault trees, polynomial and neural surrogates | ~~README says "scaffold only"; surrogates placeholder~~ README fixed on `develop` ("implemented in part, with no human V&V") |
| Reference models | `reference-data/virtual_test_bed/` | data, CC-BY-4.0 | INL VTB MSRE inputs (steady state, multiphysics core, reactivity insertion, lower plenum CFD), MSFR, an MSTDB-TC noble-gas deck | None verified by OUTRAM PARK (`INVENTORY.md`); kinetics sets disagree (β = 2.640e-3, 3.021e-3, 6.525e-3, `docs/reactor-scoping/vtb-findings.md:97-99`) |

### Crates new on `develop`

`bishan`, `buangkok`, `changi`, `dhoby-ghaut`, `dover`, `kaki-bukit`,
`kovan-web`, `pasir-ris`, `redhill`, `sembawang`. None holds molten-salt
physics. `dhoby-ghaut` (GUI workbench) lists an "MSR, liquid fuel" card as
partially supported (`src/workbench/catalogue.rs`) with no physics of its own;
`buangkok`'s ported pyDOSEIA dose code is generic and could later take MSR
release terms.

## Doc defects found upstream

These are claims in the backend that its own code contradicts (backend
`AGENTS.md` treats such a claim as a defect). They are recorded here, not
fixed, because this session has read-only access to the backend. Checked on
`develop`.

- ~~`raffles` README and `lib.rs` say nothing is implemented~~ Fixed on
  `develop`. A leftover remains: `README.md:192` refers to `src/surrogate.rs`,
  which is now a directory that does use `burn`.
- `outram-park-fork-pflotran` `Cargo.toml` description says "no flow mode solves
  yet"; its README says RICHARDS solves.
- `njoy-outram-park-fork` README (line 88) lists WIMSR as not ported; a WIMSR
  golden test, a V&V document and `docs/porting-plan.md` say it is ported.
- `outram-foam-appbuilder-lib` `src/genfoam/neutronics/diffusion/mod.rs:86`
  still lists liquid-fuel advection as deferred, and backend
  `docs/reactor-scoping/msre.md:102-103` says the same; the diffusion
  eigenvalue now has it.
