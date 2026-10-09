> **AI-GENERATED, NOT HUMAN-REVIEWED.** Transcribed and digitised by an AI agent (Claude Sonnet) on 2026-10-09 from public-domain ORNL reports. Treat every number as untrusted draft data until a human has checked it against the cited page. Research, education and V&V only.

# Thermal-hydraulics validation data corner (ORNL MSR reports)

Raw measured quantities (flows, temperatures, heat fluxes, heads, powers) are kept in the report's original US-customary units; no values were tuned, smoothed or corrected. Page numbers are the numbers printed on the report pages (not PDF page indices). Each CSV starts with a `#` header giving source, page, units, method, uncertainty, test conditions and the AI-generated label. Known printing anomalies are transcribed as printed and flagged in the CSV headers. Salt thermophysical properties were deliberately not extracted. Figure digitisations were done by image processing (deskew, marker-centroid detection, least-squares axis calibration on printed gridlines) and every one carries a reading uncertainty; digitised values were cross-checked against numbers quoted in the report text where such numbers exist (the cross-checks were not used to calibrate).

## Datasets

| File | Quantity | Source report | Page / table / figure | Method | Stated measurement uncertainty |
|---|---|---|---|---|---|
| `ORNL-TM-4079_msbr-salt-forced-convection.csv` | Forced-convection h, Nu, Re, Pr; T_in, T_out, flow, heat flux; MSBR fuel salt LiF-BeF2-ThF4-UF4 67.5-20-12-0.5, 0.18-in. ID tube, 70 runs | ORNL-TM-4079 | pp. 39-41, Table B-1 | table transcription | none overall; Vidar TC system better than +-0.5 F; correlation scatter 4-7% |
| `ORNL-TM-4079_hitec-salt-forced-convection.csv` | Same, Hitec nitrate salt checkout runs (not fluoride) | ORNL-TM-4079 | p. 42, Table B-2 | table transcription | none |
| `ORNL-TM-5335_fuel-salt-forced-convection.csv` | Local h, Nu, Re, Pr at 1.3 and 1.6 m; LiF-BeF2-ThF4-UF4 72-16-12-0.3, 10.5 mm ID, L/D 331 | ORNL/TM-5335 | pp. 17-18, Table 6 | table transcription | DAS +-0.10% FS; property uncertainties given (k +-15%, mu +-10%) |
| `ORNL-TM-5335_coolant-salt-forced-convection.csv` | Same, NaBF4-NaF 92-8 coolant salt | ORNL/TM-5335 | p. 19, Table 7 | table transcription | as above |
| `ORNL-TM-3002_msre-heat-balance-full-power.csv` | MSRE full-power salt heat balance terms (MW) | ORNL-TM-3002 | p. 5, Table I | table transcription | cp +-1.4%, dT error 0.4%, flow-cal error 2.9% (text) |
| `ORNL-TM-3002_msre-air-heat-balances.csv` | Radiator air-side heat balances, two cases | ORNL-TM-3002 | p. 13, Table III | table transcription | none; inconsistent between cases (text) |
| `ORNL-TM-3002_msre-coolant-system-head-loss.csv` | CALCULATED coolant-system head loss by component at 850 gpm (model target, not measurement) | ORNL-TM-3002 | p. 10, Table II | table transcription | +-15% viscosity band quoted (94-99 ft total) |
| `ORNL-TM-3002_msre-hx-radiator-summary-values.csv` | MSRE heat-exchanger overall U (measured and calculated), radiator U, flows, powers | ORNL-TM-3002 | pp. 4-8, 20, 23-25, Table IV (coefficient rows only) | table/text transcription | none for U |
| `ORNL-TM-3002_msre-radiator-U-vs-air-dP.csv` | Radiator overall U vs air pressure drop, one and two blowers | ORNL-TM-3002 | p. 26, Fig. 6 | figure digitisation (log-log axes) | none stated; reading +-2% |
| `ORNL-TM-3002_msre-hx-overall-U-history.csv` | Heat-exchanger overall U vs time, 8 measurements 1967-69 | ORNL-TM-3002 | p. 22, Fig. 5 | figure digitisation | none stated; reading +-3 Btu/hr-ft2-F |
| `ORNL-TM-3229_volute-centreline-velocity.csv` | Volute centreline water velocity vs angle, 300/600/1200 gpm | ORNL-TM-3229 | p. 11, Fig. 6 | figure digitisation | none; reading +-0.1 ft/s, +-1 deg |
| `ORNL-TM-3229_core-wall-annulus-vertical-velocity.csv` | Core-wall cooling annulus centreline velocity vs height | ORNL-TM-3229 | p. 12, Fig. 7 | figure digitisation | none; reading +-0.03 ft/s, +-0.2 in. |
| `ORNL-TM-3229_core-wall-annulus-angular-velocity.csv` | Annulus bottom velocity vs azimuth | ORNL-TM-3229 | p. 13, Fig. 8 | figure digitisation | none; reading +-0.02 ft/s |
| `ORNL-TM-3229_lower-head-wall-pressure.csv` | Wall static pressure at 5 taps, lower head, 1200 gpm (ordinates only) | ORNL-TM-3229 | p. 16, Fig. 9 | figure digitisation | none; +-0.005 ft |
| `ORNL-TM-3229_moderator-assembly-head-loss.csv` | Head loss across moderator assembly vs flow | ORNL-TM-3229 | p. 21, Fig. 12 | figure digitisation (log-log) | none; reading +-2% |
| `ORNL-TM-3229_overall-core-head-loss.csv` | Overall vessel head loss, 5-in. inlet to 5-in. outlet | ORNL-TM-3229 | p. 28, Fig. 15 | figure digitisation (log-log) | none; reading +-2% |
| `ORNL-TM-3229_lower-head-solids-settling.csv` | Fraction of iron filings recovered from lower head | ORNL-TM-3229 | p. 30, "Fig. 16" (table) | table transcription | none |
| `ORNL-TM-0079_pump-13in-impeller-head-flow-power.csv` | MSRE fuel pump water test, 13-in. impeller: speed, power, discharge pressure, venturi/stripper/fountain flow, total flow, head (30 rows) | ORNL-TM-79 | p. 39, Table II | table transcription | none |
| `ORNL-TM-0079_pump-11in-impeller-head-flow-power.csv` | Same, 11-in. impeller, three inlet-baffle configurations, plus pump power, water hp, efficiency (67 rows) | ORNL-TM-79 | pp. 40-41, Table III | table transcription | none |
| `ORNL-TM-2987_fuel-pump-13in-molten-salt-head-flow.csv` | Prototype fuel pump, 13-in. impeller, molten salt 1200 F head vs flow at 700/860/1030 rpm | ORNL-TM-2987 | p. 13, Fig. 6 | figure digitisation | none; reading +-8 gpm, +-0.15 ft |
| `ORNL-TM-2987_mark2-fuel-pump-molten-salt-head-flow.csv` | Mark-2 fuel pump, 11.5-in. impeller, molten salt 1200 F | ORNL-TM-2987 | p. 37, Fig. 18 | figure digitisation | none; two points have ambiguous rpm labels |

## Doubtful items (see CSV headers)

- TM-4079: heat-flux column multiplier read as 1e-5 (scan hard to read; consistent with abstract range 22,000-560,000); the printed dT column does not equal T_out - T_in for runs 210, 211, 212, 220.
- TM-5335: row order 1.3 m then 1.6 m is an inference from the table footnote.
- TM-0079 Table III: several printed total-flow values do not equal venturi + stripper + fountain (1641, 2171 are probable misprints); one stripper flow (48.3) and one kW (7.2) look wrong; kept as printed.
- TM-2987 Fig. 18: rpm labels of the top three points.
- The unlabelled block structure in TM-0079 Table II (six blocks of five speeds) is reported by index only.

## Figures NOT digitised

| Report | Figure | Page | What it shows | Why not digitised |
|---|---|---|---|---|
| ORNL-TM-4079 | Figs. 9, 10 | 20-22 (approx.) | Axial wall/fluid temperature profiles and local h along the tube for single runs | Not examined in detail; many closely spaced points per run; underlying run data not tabulated. Not attempted |
| ORNL-TM-4079 | Figs. 11, 13, 14 | 24-31 (approx.) | Nu vs Re measured vs correlations; temperature-profile comparisons | Measured points are already tabulated in Table B-1; correlation lines not data; not attempted |
| ORNL/TM-5335 | Figs. 8, 9 | 22-23 | Nu vs Re for all wall thermocouples vs Sieder-Tate / Hausen curves, both salts | Hundreds of overlapping points (all thermocouples on the heated section), only the 1.3/1.6 m values are tabulated; not reliably separable |
| ORNL/TM-5335 | Fig. 7 | 21-22 | Fluid and wall temperature along the section for a run | Not digitised; the report notes bulk temperature was assumed linear (not measured) |
| ORNL-TM-3002 | Fig. 2 | 12 | Pitot-venturi air-flowmeter calibration (laboratory, stack, manufacturer) | Calibration of an instrument not a salt result; fit equation V = 1254.78 dP^0.53092 + 181 printed on figure (not transcribed); points not digitised |
| ORNL-TM-3002 | Fig. 3 | 14 | Radiator-stack air velocity and temperature-rise traverses (two diameters) | Many overlapping points on two series with hand-drawn curves; not reliably separable |
| ORNL-TM-3002 | Fig. 5 (HTI bars) | 22 | Heat transfer index bars (MW/F) vs time | Thick bars, no individual values; the overall-U dots were digitised |
| ORNL-TM-3229 | Fig. 10 | 17 | Wall-jet velocity profile in lower head at 17 in. radius, four azimuths | Four overlapping symbol types (open/filled circle and triangle) many coincident; not reliably separable |
| ORNL-TM-3229 | Fig. 11 | 19 | Heat-transfer coefficients in lower head vs flow (log-log, four symbol types) | Overlapping symbols and points under legend box; coefficients converted water-to-salt by the author |
| ORNL-TM-3229 | Fig. 13 | 23 | Flow in 77 fuel channels vs radius (two channel groups) | About 77 heavily overlapping points of two similar symbols; not reliably separable |
| ORNL-TM-3229 | Fig. 14 | 27 | Upper-head velocity profiles by fluid-age technique | Drawn as short broken line fragments; no clear data points |
| ORNL-TM-0079 | Figs. 9, 10, 12, 13 | 16, 17, 20, 21 | Pump head vs flow, 13-in. and 11-in. impellers, power, efficiency contours | Underlying data are tabulated in Tables II and III (transcribed); efficiency-contour plot (Fig. 13) not examined |
| ORNL-TM-0079 | Fig. 7, 8 | 11, 13 | Venturi and motor calibration | Instrument calibration; Fig. 8 lines nearly overlapping |
| ORNL-TM-0079 | Figs. 14-16, 18 | 23-28 | CO2 stripping effectiveness, fountain flow vs speed | Out of the heat-transfer/flow-performance scope (chemistry/gas stripping); Table I (stripping tests) not transcribed |
| ORNL-TM-2987 | Fig. 6 (water curves), Fig. 18 (vendor curves) | 13, 37 | Water/vendor head-flow lines | Lines only, no data points |
| ORNL-TM-2987 | Fig. 9 | 20 | Visicorder traces of undissolved gas by gamma densitometry vs time | Analogue strip-chart traces with noise; not digitised (gas content values 4.6% and 1.7% printed on figure) |
| ORNL-TM-2987 | Figs. 8, Tables 2-3 | ~16-18 | Back-diffusion of radioactive gas vs purge flow | Out of scope (gas transport, not thermal hydraulics); not examined |

## Not examined / worth a second pass

- ORNL-TM-4079 figures were only scanned from text; Table B-1 transcription was checked via image and the h/Nu constant-ratio test, but the figure pages were not viewed.
- ORNL-TM-2987 (coolant pump data, Table 1 operating history, pump performance in MSRE, p. 38-40) and ORNL-TM-0079 (coastdown, priming, Tables not extracted) have further quantitative content not extracted.
- Other manifest reports on MSRE heat transfer, flow, radiator and heat-exchanger performance were not searched.
