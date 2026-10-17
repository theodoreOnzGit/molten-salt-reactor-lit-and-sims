# What OUTRAM PARK offers a molten salt reactor model

> **AI-generated survey, not human-reviewed.** Written 2026-10-17 by Claude
> from a read of
> [`outram-park-backend`](https://github.com/theodoreOnzGit/outram-park-backend)
> at commit `8d31482`. Each claim names the file it was read from, so it can be
> checked. Crate versions are those in each crate's `Cargo.toml` at that commit.

## Headline

- **Only one crate has human V&V sign-off: `tuas_boussinesq_solver`**
  (README bookkeeping block, "Cleared by maintainer (2026-07-21)", validated
  against CIET and published as doi:10.1016/j.jandt.2025.03.006). Every other
  crate below is unreviewed AI-assisted draft material by its own README.
- **No crate is compared against MSRE measured data.** Searches for ORNL-4233,
  "zero-power physics" and Prince/Haubenreich across the workspace found
  nothing.
- **The molten-salt-specific physics lives in crates that are not on
  crates.io**: `outram-park-fork-moltres` (precursor drift), `-onix`
  (depletion), `-thermochimica` (Gibbs minimisation) and `-offbeat`
  (Hastelloy-N correlations). This repository depends only on published crates
  ([`Cargo.toml`](../Cargo.toml), list checked 2026-10-09; crates.io could not
  be reached from the 2026-10-17 session to recheck). Using them needs either a
  release or a git dependency.
- The backend's own MSRE plan is
  [`docs/reactor-scoping/msre.md`](https://github.com/theodoreOnzGit/outram-park-backend/blob/main/docs/reactor-scoping/msre.md)
  (audit 2026-08-06). Its "missing" list agrees with this survey: a real-time
  circulating-fuel point-kinetics model, MSRE fuel-salt properties, an
  air-cooled radiator, the secondary salt loop, xenon stripping and
  precursor-borne decay heat, and MSRE geometry and kinetics data.

## By reactor-design aspect

State key: **working** = implemented and unit-tested (verification only unless
stated); **partial** = some of the needed physics; **scaffold** = types or
stubs without physics; **absent** = not found.

| Aspect | Crate (path under `crates/`) | State | What is there | What a molten salt model still needs |
|---|---|---|---|---|
| Salt properties | `tuas_boussinesq_solver` (`…/liquid_database/`) | working, human-validated crate | FLiBe and FLiNaK (Romatoski & Hu 2017; Sohal et al. 2010), HITEC, Dowtherm A / Therminol VP-1, YD-325; `CustomLiquid` escape hatch | MSRE fuel salt LiF-BeF₂-ZrF₄-UF₄ and coolant salt from ORNL-TM-2316; uncertainties carried with each correlation |
| | `outram-park-fork-coolprop` | working | 137 Helmholtz fluids, 127 incompressibles incl. Therminol, NaK | No molten salts |
| Steady neutronics (MC) | `outram-mc-libs` | working | CSG k-eigenvalue, delta tracking, S(α,β), tallies, CRAM16 depletion | No delayed-neutron transport (ν̄ total only, `physics/fission.rs`); measured-critical bias +2950 ± 61 pcm on LEU-COMP-THERM-008 by its own record |
| Nuclear data | `njoy-outram-park-fork` | working | RECONR, BROADR, THERMR, ACER, GROUPR and more, code-to-code against NJOY2016 | Li-7 and Zr isotopes not in the NJOY comparison tests (checked 2026-10-09, issue #10) |
| | `reference-data/endf/` (workspace folder) | data | ENDF/B-VIII.0 tapes incl. Li-6, Li-7, Be-9, F-19, C-12, U-234/235/238; graphite S(α,β) | Zr, Mo, Ni, Cr, Fe tapes not checked |
| Deterministic neutronics | `outram-foam-appbuilder-lib` (`src/genfoam/neutronics/`) | partial | Diffusion and SP3 eigenvalue + transient, S_N eigenvalue, Doppler/expansion/coolant-density feedback | Liquid-fuel precursor advection deferred in all four models (`point_kinetics/mod.rs:83` and siblings) |
| | `bedok` | partial, `publish = false` | 3-D SANM nodal diffusion | PWR-oriented; TH coupling 1/9 files, feedback 0/5 |
| Circulating-fuel neutronics | `outram-park-fork-moltres` | working, unpublished | Multigroup diffusion eigenvalue, precursor advection-diffusion-decay, 1-D core-plus-loop ring, salt energy equation with linear feedback | Data are "MSRE-like, not evaluated MSRE" (`src/circulating.rs`); prescribed velocity; steady coupled solve only; 287 pcm at 0.6 m/s compared only in order to the measured ~212 pcm |
| Point kinetics | `teh-o-prke` | working | 6-group PRKE (U-235, U-233, Pu-239), Nordheim-Fuchs, 23-group ANS decay heat, Xe/I solver, linear fuel-temperature feedback | No precursor drift / loop return term (searched `src/` 2026-10-09 and 2026-10-17) |
| | `nee_soon` | scaffold | Pass-through to Nordheim-Fuchs | Coupling workflow returns `NotYetImplemented` |
| Depletion | `outram-park-fork-onix` | working, unpublished | Bateman matrix, CRAM16, verified against closed-form Bateman | No nuclide libraries, no salt-specific removal terms (off-gas, plating) |
| | `outram-mc-libs` (`src/depletion/`) | working | CRAM16, chain, predictor burnup | as above |
| Fission-product transport | `boon-lay` | working for TRISO | Lagrangian decay and diffusion, Booth release | FHR/TRISO only; no salt, gas or graphite pathways |
| Loop thermal hydraulics (1-D) | `tuas_boussinesq_solver` | working, human-validated | Pipe networks (series/parallel), natural circulation, Churchill friction, Gnielinski/laminar Nusselt, shell-and-tube HX, porous media, CIET/DRACS cases | No MSRE loop, no radiator, no pump curves, no salt freezing |
| | `tampines` | partial | Pebble bed, gas phase, drift-flux and two-fluid 1-D; FHR GUI example | Pump, valve, HX components return `NotYetImplemented` |
| | `outram-park-fork-dwsim-libs` | working | LMTD / ε-NTU HX, pump curves and NPSH, valves | No salts |
| Core CFD | `outram-foam-basic-lib`, `-turbulence-lib`, `-appbuilder-lib` | working | FV operators, k-ω SST, k-ε, pimpleFoam, enthalpy-porosity melting source, meltFoam | Not benchmark-validated; no salt properties wired in |
| Process control | `chem-eng-real-time-process-control-simulator` | working | PID, transfer functions, z-domain | No bookkeeping block |
| Salt thermochemistry | `outram-park-fork-thermochimica` | partial, unpublished | Gibbs minimiser, ideal and binary Redlich-Kister solutions, 6 analytic tests | No database parser (so no MSTDB-TC), no redox, speciation or solubility workflow (`src/gem.rs:162-200`) |
| Structural alloy | `outram-park-fork-offbeat` | partial, unpublished | Hastelloy-N conductivity, expansion, Young's modulus, void swelling; Norton/Chaboche creep laws | Hastelloy creep (Zhang) not ported; no helium embrittlement, no irradiated creep-rupture |
| | `farrer-park` | working | FEM elasticity, J2 plasticity, crystal plasticity | No creep, no ASME III-5, no material library |
| Corrosion | — | absent | `offbeat` corrosion is Zircaloy waterside only | Cr depletion, Te cracking, all salt corrosion |
| Graphite | `tuas_boussinesq_solver` | partial | Nuclear graphite thermal properties as a solid | No irradiation dimensional change, gas or salt penetration |
| Tritium | — | absent | — | Everything |
| Uncertainty | `raffles` | partial | 9 distributions, MC / LHS / grid samplers, Pearson, Spearman, Sobol indices | README says "scaffold only" but source has these (a doc defect upstream); surrogates placeholder |
| Reference models | `reference-data/virtual_test_bed/` | data, CC-BY-4.0 | INL VTB MSRE inputs (steady state, multiphysics core, reactivity insertion, lower plenum CFD), MSFR, an MSTDB-TC noble-gas deck | None verified by OUTRAM PARK (`INVENTORY.md`); kinetics sets disagree (β = 2.640e-3, 3.021e-3, 6.525e-3, `docs/reactor-scoping/vtb-findings.md`) |

## Doc defects found upstream

These are claims in the backend that its own code contradicts (backend
`AGENTS.md` treats such a claim as a defect). They are recorded here, not
fixed, because this session has read-only access to the backend.

- `raffles` README and `lib.rs` say nothing is implemented; `src/distributions.rs`,
  `src/samplers.rs` and `src/sensitivity.rs` implement and test distributions,
  samplers and Sobol indices.
- `outram-park-fork-pflotran` `Cargo.toml` description says "no flow mode solves
  yet"; its README says RICHARDS solves.
- `njoy-outram-park-fork` README lists WIMSR as not ported; a WIMSR golden test
  and a V&V document exist.
