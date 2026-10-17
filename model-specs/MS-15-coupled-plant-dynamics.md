# MS-15: Coupled plant dynamics and control (MSRE integrated model)

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #57 (part of epic #42)

## Purpose

Join MS-03 kinetics, MS-05 loops, MS-06 core temperatures, MS-07 heat
exchangers, MS-08 pumps and MS-04 xenon into one time-domain model of the
MSRE, with its control system, and predict frequency response, power
oscillations, load following and operational transients. This is the model
the dynamics measurements validate, and the model a real-time simulator runs.

## Scope

MSRE ²³⁵U and ²³³U at 0–8 MW. Two fidelities: real-time (reduced kinetics,
1-D loops) and offline reference (spatial kinetics). The Aircraft Reactor
Experiment (ARE) as a second circulating-fuel case.

## Governing physics (on by default)

- Every term of the component specs, on by default in the coupled model.
- Fuel and graphite temperature feedback with separate coefficients and
  separate time constants (MS-02, MS-06).
- Transport delays around both loops from flow, varying with pump state.
- Control rod servo and the radiator door and blower control as operated
  (ORNL-TM-0729A/B instrumentation).

## Inputs and sources

| Input | Source |
|---|---|
| Stability analysis | ORNL-TM-1070 |
| Theoretical ²³³U dynamic analysis (1969 model) | ORNL-TM-2571 (model inputs, not measurements) |
| Instrumentation and control | ORNL-TM-0729A, ORNL-TM-0729B |
| Operator simulators of the 1960s | ORNL-TM-1445 |
| Operating techniques | ORNL-TM-3041 |
| ARE operation | ORNL-1845 |
| MSBR control and hybrid simulation (code-to-code only) | ORNL-TM-2489, ORNL-TM-2927, ORNL-TM-3102, ORNL-TM-3767 |

## OUTRAM PARK starting point

- `outram-park-digital-twin-engine`: real-time framework; MSRE appears only
  as a visual archetype label. The backend's `docs/reactor-scoping/msre.md`
  is the integration plan.
- `chem-eng-real-time-process-control-simulator`: PID and transfer functions
  for the controllers.
- `outram-park-fork-moltres` coupled solve takes about 1.2 s against a 1 ms
  real-time budget; hence the reduced model in MS-03.
- `nee_soon` coupling layer: scaffold.

## Verification

- Real-time model vs offline reference for the same transient, with the
  difference stated before the run.
- Linearised model frequency response vs the time-domain model driven by a
  sine.
- Energy conservation over a transient.

## Validation

T-1 (#29) and T-2 (#30) frequency response against the **measured** columns;
T-3 (#31) period; T-4 (#32) void fraction from noise; T-5 (#33) ARE
transients; T-6 (#34) and T-7 (#35) once data are found.

## Known traps

- **ORNL-TM-2571 predictions came from a model.** They are a code-to-code
  reference for that 1969 model only.
- **The low-frequency response depends on loop transit times**, which come
  from MS-05 volumes and flows, never from fitting the response.
- **The MSRE natural oscillation period changes strongly with power**
  (ORNL-TM-1647). A model tuned at one power and checked at that power proves
  nothing; check across power.
