> **AI-GENERATED, NOT HUMAN-REVIEWED.** Transcribed and digitised by an AI agent (Claude Sonnet) on 2026-10-09 from public-domain ORNL reports. Treat every number as untrusted draft data until a human has checked it against the cited page. Research, education and V&V only.

# MSRE neutronics / reactor-physics validation data (AI-generated corner)

Sources (all under `literature/moltensalt-org/`): ORNL-4233 (235U zero-power experiments, 1967), ORNL-TM-3963 (233U zero-power experiments), ORNL-TM-1796 (reactivity balance, 1967), ORNL-TM-3464 (xenon behaviour), ORNL-TM-3151 (gamma spectrometry of fission products, 1972), ORNL-TM-0379 (checked, nothing to extract: see below).
Page numbers are the printed report page, with the PDF page in brackets. Every table transcription was compared with the page image; OCR text layers in these scans are unreliable for digits (several were wrong and were corrected from the image). Original units are kept (F, in., lb/ft3, % dk/k). Where a conversion is given it is a plain factor (1/F to 1/K: x1.8).

## Datasets

| File | Quantity | Source (page, table/figure) | Method | Stated uncertainty |
|---|---|---|---|---|
| `ORNL-4233_critical-concentration.csv` | Critical 235U concentration, measured (two bases) vs calculated | ORNL-4233 p.11 (PDF 21), Table 1 | table transcription | +/-0.007 wt %, +/-1 lb/ft3, +/-0.3 g/L |
| `ORNL-4233_delayed-neutron-fractions.csv` | Six-group delayed-neutron data used (adopted values, not measured) | p.16 (PDF 26), Table 2 | table transcription | none stated |
| `ORNL-4233_rod-bank-worth.csv` | Banked critical positions and derived worth of rod banks 1-2, 1-2-3 | p.22 (PDF 32), Table 3 | table transcription | none in table; asymmetry <0.005 % dk/k |
| `ORNL-4233_rod-worth-measured-vs-calc.csv` | Control-rod worth, measured vs calculated (51 in. travel) | p.35 (PDF 45), Table 4 | table transcription | none in table; 5 % self-consistency in text |
| `ORNL-4233_rod1-integral-worth.csv` | Integral worth of regulating rod 1 vs position, two loadings | p.18 (PDF 28), Fig. 6 | figure digitisation | read +/-0.03 % dk/k, +/-0.1 in |
| `ORNL-4233_rod1-differential-worth.csv` | Differential worth of rod 1 vs position (authors' fitted curve only) | p.17 (PDF 27), Fig. 5 | figure digitisation | read +/-0.002 % dk/k/in; report rms scatter 2.8e-4 % dk/k/in |
| `ORNL-4233_reactivity-coefficients.csv` | Concentration coefficient, circulation loss, temperature coefficients (measured and calculated) | pp.19-23, 37-42 (PDF 29-52), text, Fig. 7/16 values | text transcription | as quoted per row |
| `ORNL-TM-3963_critical-concentration.csv` | 233U critical loading/concentration with corrections | pp.19-22, 40 (PDF 27-30) | text transcription | 15.11 +/- 0.10 g U/L |
| `ORNL-TM-3963_nuclear-characteristics.csv` | 233U rod worth, temperature and concentration coefficients, measured vs calculated | p.54 (PDF 62), Table 7; p.45 Table 6 | table transcription | none in table |
| `ORNL-TM-3963_critical-corrections-calc.csv` | CALCULATED corrections to 233U critical loading (not measured) | p.22 (PDF 30), Table 2 | table transcription | none stated |
| `ORNL-TM-1796_reactivity-balance-terms.csv` | Reactivity-balance terms at start of power and after 16,450 Mwhr | p.46 (PDF 52), Table 2 | table transcription | per-term, 0.006 to 0.04 % dk/k; residual +0.047 +/-0.04 |
| `ORNL-TM-3464_xenon-poisoning-measured.csv` | Measured steady-state 135Xe poisoning, 235U and 233U operation | pp.15, 29 (PDF 21, 35), Tables 2, 3 | table transcription | none stated |
| `ORNL-TM-3464_core-void-vs-conditions.csv` | Core void fraction change vs pressure/temperature/level | p.11 (PDF 17), Table 1 | table transcription | none stated |
| `ORNL-TM-3464_xenon-vs-void-fig19.csv` | Measured xenon poisoning vs core void fraction, helium and argon (points only) | p.72 (PDF 78), Fig. 19 (cf. Figs 17, 18) | figure digitisation | read +/-0.015 % dk/k, +/-0.01 vol % |
| `ORNL-TM-3151_offgas-jumper-activity.csv` | Fission-product activities in off-gas jumper line at shutdown 1969-06-01 | p.65 (PDF 73), Table 7.1 | table transcription | std dev of average per row |
| `ORNL-TM-3151_fuel-line-relative-activity.csv` | Relative activities in fuel line 102, shutdown 1969-06-01 | p.66 (PDF 74), Table 7.2 | table transcription | none; report urges restraint |

## Caveats

- The Fig. 5 and Fig. 6 digitisations are the authors' smooth curves, not raw measurements; the Fig. 6 curves are an integral of the Fig. 5 fit.
- In `ORNL-4233_rod1-differential-worth.csv` the report's text layer says the rms deviation is "4.7 %" of the mean differential worth while the page image reads about 0.7 % (2.8e-4 against ~0.045 supports 0.7 %); not resolved.
- Fig. 19 plotted curves in ORNL-TM-3464 are tuned model fits ("best fit"); only measured points were taken. Fig. 19 void-fraction values are themselves inferred, not directly measured (report says no direct void measurement).
- ORNL-TM-3963's text-layer sentence on the temperature coefficient with no circulating bubbles is garbled; only the Table 7 values were recorded.
- Tables 2 in ORNL-TM-3963 and the delayed-neutron fractions are calculated/adopted inputs, labelled so in the file headers.
- ORNL-TM-0379 (1962) is a calculation-only report (importance-averaged temperature coefficients: fuel -4.4e-5, graphite -7.3e-5 per F as quoted in its abstract; nothing measured) and was not turned into a dataset.
- Many tables in ORNL-TM-3151 (Tables 7.4 to 7.14, 8.1 to 8.3: nuclide chains, Nb/Zr, Ru, I ratios) were not transcribed; only the first two quantitative tables were done.

## Figures NOT digitised

| Report | Figure | Page (PDF) | What it shows | Why not |
|---|---|---|---|---|
| ORNL-4233 | Fig. 7 | 19 (29) | Reactivity vs 235U mass in loop, fuel circulating and not circulating | Near-straight lines with closely spaced points; reading to useful precision (<=0.03 % dk/k) not reliable at this scale. Slope and 0.212 +/- 0.004 % circulation effect are in the scalar CSV |
| ORNL-4233 | Fig. 16 | 38 (48) | Reactivity vs fuel temperature, three loadings | Curves are translated vertically (arbitrary offsets) so only slopes mean anything; slopes are given in the text and recorded |
| ORNL-4233 | Figs 8, 10 | 21, 25 (30, 35) | Shim/regulating rod shadowing curves; differential worth with fuel circulating | Not attempted in this pass; Fig. 8 curves are three loadings interleaved, Fig. 10 is scattered points |
| ORNL-4233 | Figs 11-14 | 30-32 (39-41) | Rod-drop integrated count vs time (measured vs calculated) | Not attempted; counts on scales that need dense point reading, overlapping curves/points, rod-drop data not tabulated |
| ORNL-4233 | Fig. 3, Figs 1-2 | 8, 5-6 (17, 15-16) | Subcritical count-rate ratio approach to critical; schematics | Not attempted (extrapolation plots, low priority) |
| ORNL-4233 | Figs 18-24 | 44-61 | Pressure-release transients, frequency responses, pump start/coastdown | Not attempted (dynamics, outside this topic pass) |
| ORNL-TM-3963 | Figs 3-5, 6-8 | 33-52 (approx. PDF 40-52) | Subcritical count rate in drain tank; rod-drop results; reactivity vs uranium mass; calibration fit; rod-shadowing | Not examined at image level in this pass; no digitisation attempted |
| ORNL-TM-3963 | Fig. 9 | PDF 58 | Neutron flux-to-reactivity frequency response | Not attempted; log-frequency axes |
| ORNL-TM-1796 | Fig. 15 | 41 (47) | Complete reactivity balance vs date, Oct 1966 to Jan 1967, with power | Hundreds of dense noisy points on multiple date axes, gaps for computer outage; not reliably digitisable |
| ORNL-TM-1796 | Figs 3-13 | various | Calculated xenon build-up/decay vs bubble volume and stripping efficiency | Calculated only |
| ORNL-TM-3464 | Fig. 28 | 90 (96) | Observed vs calculated xenon removal transient (argon, 0.7 vol % voids) | Observed points are dense and overlap the curves (about 40 points); fraction-of-initial scale only; skipped to avoid unreliable values |
| ORNL-TM-3464 | Figs 17, 18 | 67, 70 (73, 76) | Observed vs calculated xenon vs void (argon; helium) | Observed points covered by Fig. 19 digitisation; calculated curves not digitised |
| ORNL-TM-3464 | Figs 3, 5, 7, 9, 20-27, 29 | various | Void fraction vs pump speed; model sensitivity | Mostly calculated; Fig. 3 (void vs pump speed, measured) not attempted |
| ORNL-TM-3151 | all spectra figures | pp.57-140+ | Gamma spectra and activity profiles along lines | Not attempted; 214-page report only sampled |
