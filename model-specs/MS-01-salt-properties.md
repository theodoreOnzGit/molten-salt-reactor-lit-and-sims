# MS-01: Fuel, coolant and flush salt thermophysical properties

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #43 (part of epic #42)

## Purpose

Supply density, viscosity, heat capacity, thermal conductivity, liquidus and
heat of fusion for every salt the other models carry, **with the uncertainty
each correlation states**. Every thermal-hydraulic, kinetic and chemical model
reads these. They are inputs, never tuning knobs.

## Scope

| Salt | Composition (mol %) | Used in |
|---|---|---|
| MSRE fuel, ²³⁵U | LiF-BeF₂-ZrF₄-UF₄ 65-29.1-5-0.9 (nominal) | MS-02 to MS-06, MS-14, MS-15 |
| MSRE fuel, ²³³U | LiF-BeF₂-ZrF₄-UF₄ with ²³³U loading | as above |
| MSRE coolant | LiF-BeF₂ 66-34 | MS-05, MS-07 |
| MSRE flush | LiF-BeF₂ 66-34 | MS-14 (fill/drain) |
| Heat-transfer test salts | LiF-BeF₂-ThF₄-UF₄ 67.5-20-12-0.5 (ORNL-TM-4079), LiF-BeF₂-ThF₄-UF₄ 72-16-12-0.3 and NaBF₄-NaF 92-8 (ORNL-TM-5335), HITEC checkout runs | MS-07, for TH-1 and TH-2. ORNL-TM-5335 states its own property uncertainties (k ±15 %, μ ±10 %); use them |

Compositions must be taken from the cited report for each run, not from the
nominal row above. Out of scope: chloride salts, salt vapour pressure (MS-09).

## Governing relations

Each property is a function of temperature (and composition where the source
gives it), returned with its range of validity and its stated uncertainty.
Outside the range the call returns an error, not a clamped value: a silent
clamp hides an extrapolation from the model that asked for it.

## Inputs and sources

| Property | Source (in `literature/`) | Note |
|---|---|---|
| ρ, μ, k, c_p, ΔH_fus for fuel, coolant and flush salts | ORNL-TM-2316 (Cantor et al. 1968), Tables 1–6 | Transcribed and AI-audited in `validation-data/ai-generated/salt-properties/`. **Needs human review (#38) before use** |
| Liquidus and phase equilibria | ORNL-2548, ORNL-2295, ORNL-2896 | Phase diagrams; not yet transcribed |
| Fluoride-salt thermal conductivity, variable-gap method | Not located. ORNL-4831 defers the salt results to a later report (#41) | Until found, state which k correlation is used and its uncertainty |
| Electrical conductivity | ORNL-TM-2180 | Only for instrumentation models |

## OUTRAM PARK starting point

`tuas_boussinesq_solver` (`…/liquid_database/`) has FLiBe and FLiNaK from
Romatoski & Hu (2017) and Sohal et al. (2010), and a `CustomLiquid` hook that
takes four property functions plus bounds with no solver changes
(`docs/reactor-scoping/msre.md` in the backend). **Reuse `CustomLiquid` for the
MSRE salts.** Do not substitute the FLiBe correlation for the MSRE coolant
without showing the two agree within their uncertainties over 500–700 °C; the
compositions differ.

## Verification

1. Each implemented correlation reproduces every tabulated point of its source
   table to the printed precision (unit test per table).
2. Out-of-range calls return an error (unit test).
3. Unit consistency through `uom` types.

## Validation

There is no separate validation: a property correlation **is** a fit to
measurement. TH-6 (#17) records which correlation is quoted and the scatter of
the source data about it.

## Known traps

- **Composition drift.** The MSRE fuel was adjusted over its life (ORNL-4658).
  Take the composition for the run being modelled.
- **ORNL-TM-2316 gives estimated and measured values side by side.** Its
  heat capacity and conductivity for several salts are estimates. Carry the
  label through.
- **Never adjust a property to improve TH-1 to TH-5.** If a heat-transfer
  comparison misses, the property uncertainty belongs in the error budget
  (MS-16), not in a new coefficient.

## Open questions

- Where are the measured fluoride-salt conductivities ORNL-4831 promised?
