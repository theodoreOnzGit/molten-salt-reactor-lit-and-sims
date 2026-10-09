> **AI-GENERATED, NOT HUMAN-REVIEWED.** Transcribed and digitised by an AI agent (Claude Sonnet) on 2026-10-09 from public-domain ORNL reports. Treat every number as untrusted draft data until a human has checked it against the cited page. Research, education and V&V only.

# Transients and reactor dynamics: validation data corner

Source PDFs: `literature/moltensalt-org/{ornl,ornl-tm}/`. Page numbers are PDF page indices (printed page in brackets). Tables were checked against page images; figure values were extracted by pixel analysis of renders calibrated on the printed log-log grid lines, and stored linear.

## Datasets

| File | Quantity | Source report | Page / table / figure | Method | Stated measurement uncertainty |
|---|---|---|---|---|---|
| `ORNL-TM-1647_prbs_test_matrix.csv` | PRBS test configuration (bits, bit time, periodicity, min frequency) vs power | ORNL-TM-1647 | PDF p.16, Table 2 | table transcription | none (configuration table) |
| `ORNL-TM-1647_natural_period_vs_power.csv` | Natural oscillation period vs power, measured and theory | ORNL-TM-1647 | PDF p.46, Fig. 29 | figure digitisation | none stated; digitiser reading about 2-3 % |
| `ORNL-TM-2997_freqresp_test_log.csv` | 233U frequency-response test log (date, integrated MW-hr, power, number of tests) | ORNL-TM-2997 | PDF p.22, Table 1 | table transcription | none (test log) |
| `ORNL-TM-2997_fluxreact_8MW_isolated_points.csv` | Flux-to-reactivity magnitude ratio, 8 MW, 233U, 7 isolated low-frequency points | ORNL-TM-2997 | PDF p.20, Fig. 9 | figure digitisation | none stated; digitiser reading about 3 % |
| `ORNL-TM-2571_233U_vs_235U_kinetics_params.csv` | Delayed-neutron data, generation time, temperature coefficients used in 233U vs 235U dynamics models (model inputs, not measurements) | ORNL-TM-2571 | PDF p.7, Table 1 | table transcription | none |
| `ORNL-TM-1626_circulation_reactivity_loss.csv` | Reactivity loss due to fuel circulation, measured vs calculated | ORNL-TM-1626 | PDF p.32 (p.26), text | text transcription | measured +/-0.004 % dk/k |
| `ORNL-1845_ARE_fuel_flow_reactivity.csv` | ARE reactivity change vs fuel flow rate and rod movement (16 runs) | ORNL-1845 | PDF p.65 (p.55), Table 5.7 | table transcription | none stated |
| `ORNL-1845_ARE_calculated_reactivity_rates.csv` | ARE calculated reactivity insertion rates by operation (calculated, not measured) | ORNL-1845 | PDF p.97 (p.87), Table 6.6 | table transcription | none |

Notes. ORNL-TM-2997 and ORNL-TM-1647 state that measured flux-to-reactivity magnitudes were biased or normalised by factors (e.g. x1.75 in 2997 Fig. 6) because of control-rod indication problems; absolute magnitudes should not be treated as precise. ORNL-TM-4179 (Prince, circulating-fuel kinetics) is purely theoretical and contains no experimental data; no dataset taken from it. ORNL-TM-2571 figures 3, 4, 5-7, 10 are theoretical predictions only.

## Figures NOT digitised

| Report | Figure | PDF page | What it shows | Why not |
|---|---|---|---|---|
| ORNL-TM-2997 | Fig. 4 | 11 | 8 MW step response, measured vs theory (delta-MW vs time) | experimental trace is very noisy (circulating voids); any "reading" would be arbitrary |
| ORNL-TM-2997 | Fig. 5 | 15 | flux-to-reactivity response, zero power, stationary fuel, CPSD/FOURCO vs theory | two overlapping scatter marker sets plus theory; log magnitude; not reliably separable |
| ORNL-TM-2997 | Fig. 6 | 16 | same, zero power, circulating fuel (data normalised x1.75) | overlapping marker sets, normalisation; scatter |
| ORNL-TM-2997 | Figs. 7, 8 | 18, 19 | same at intermediate powers | dense overlapping markers; not attempted |
| ORNL-TM-2997 | Fig. 9 | 20 | 8 MW response: phase panel, theory curve, and magnitude cluster 0.07-0.8 rad/s | markers overlap heavily and sit on grid crossings; automated detection unreliable, theory trace contaminated by markers/legend. Only 7 isolated magnitude points were taken |
| ORNL-TM-2997 | Fig. 10 | 24 | outlet temperature-to-power response, 8 MW, three tests plus three scaled theory curves | overlapping markers; theory curves scaled by 0.5 and 0.1 |
| ORNL-TM-1647 | Figs. 21-28 | 38-45 | flux-to-reactivity frequency response at 0, 0.075, 0.465, 1.0, 2.5, 5, 6.7(?), 7.5 MW, five analysis methods per plot | five different marker shapes overlapping on log-log grids; report itself says absolute amplitude is biased between tests |
| ORNL-TM-1647 | Figs. 13-19 | 29-35 | cross-correlation / autocorrelation functions of PRBS tests | linear but dense continuous traces without tabulated values; not attempted |
| ORNL-TM-1626 | Figs. 7, 8 | 33-34 | control rod differential worth from period measurements (fuel stationary, circulating) with reference curve | scattered points, not attempted; Fig. 8 compares to Fig. 7 curve |
| ORNL-TM-2315 | Figs. 5, 6, 7, 9 | 20-25 | neutron noise level (NPSD) vs pump-tank level, He pressure, temperature, net reactivity (5 MW) | log y-axis with mixed marker types (filled/open circles, triangles) and labelled line segments; not attempted in time |
| ORNL-TM-2315 | Fig. 8, Fig. 10 | 23, 26 | noise spectra with min/max void; void fraction vs NPSD | log-log, continuous spectra, no per-point data |
| ORNL-TM-2571 | Figs. 3, 4 | 12-13 | predicted 233U power response (theory only, no measurement) | no measured curve to digitise |
| ORNL-1845 | Fig. 6.14 | ~97-98 | observed ARE reactor periods vs initial power, with lower limits | scattered points on a log scale; not attempted. Fig. 5.11 (excess reactivity vs fuel flow) also not digitised (Table 5.7 gives the tabulated data) |
| ORNL-1845 | other transient figures (e.g. Fig. 6.8, 5.12) | various | chart-recorder traces | raw recorder charts, no usable axis scale |

PDF page numbers for ORNL-TM-1647 figs 21-28, ORNL-TM-2315 Fig. 10, ORNL-TM-2571 Figs. 3-4 and ORNL-1845 Fig. 6.14 are approximate (derived from text search, not each checked visually). Pump start-up/coast-down and natural-circulation datasets were not found in the reports examined.
