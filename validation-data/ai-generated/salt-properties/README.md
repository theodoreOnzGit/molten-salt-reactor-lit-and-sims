# Molten Salt Thermophysical Properties — Validation Data

> **AI-GENERATED, NOT HUMAN-REVIEWED.** Transcribed by an AI agent (Claude Haiku) on 2026-10-09 from public-domain ORNL reports. Treat every number as untrusted draft data until a human has checked it against the cited page. Research, education and V&V only.

## Dataset Index

| CSV File | Property | Salt & Composition (mol %) | Source Report | Printed page | Table |
|----------|----------|---------------------------|-----------------|------|---------|
| `ORNL-TM-2316_table1_heat_capacity.csv` | Heat capacity Cp = a + b*t (constant pressure) | F1-F4 (LiF-BeF2-ThF4-UF4), L2B (LiF-BeF2 66-34), C1 (NaBF4-NaF 92-8), C2 (NaBF4 100) | ORNL-TM-2316 (1968) | 22 | unnumbered in report |
| `ORNL-TM-2316_table2_density_liquid.csv` | Liquid density rho(t) | F1-F4, L2B, C1, C2 | ORNL-TM-2316 (1968) | 28 | unnumbered |
| `ORNL-TM-2316_table3_viscosity.csv` | Viscosity eta(T) exponential equation | F1-F4, L2B, C1, C2 | ORNL-TM-2316 (1968) | 8 | unnumbered |
| `ORNL-TM-2316_table4_thermal_conductivity.csv` | Thermal conductivity (mostly estimated) | F1-F4, L2B, C1, C2 | ORNL-TM-2316 (1968) | 11 | unnumbered |
| `ORNL-TM-2316_table5_electrical_conductivity.csv` | Electrical conductivity kappa(t) linear equation | F1-F4, L2B, C1, C2 | ORNL-TM-2316 (1968) | 14 | unnumbered |
| `ORNL-TM-2316_table6_heat_of_fusion.csv` | Heat of fusion and solid-transition enthalpy | F1-F4, L2B, C1, C2 | ORNL-TM-2316 (1968) | 25 | unnumbered |
| `ORNL-4831_table2_calibration_k_vs_T.csv` | Measured k vs T, variable-gap apparatus (31 runs) | CALIBRATION fluids only: H2O, Hg, HTS (KNO3-NaNO2-NaNO3 44-49-7), He, Ar | ORNL-4831 (Cooke, 1973) | 41 | 2 |
| `ORNL-4831_tableB12_H2O_calibration_I-A.csv` ... `ORNL-4831_tableB22_HTS_calibration_III-B.csv` (11 files: B-12 H2O, B-13 Hg, B-14 HTS, B-15 He, B-16 vacuum, B-17 HTS, B-18 He, B-19 Ar, B-20 vacuum, B-21 He, B-22 HTS) | Reduced variable-gap data: thickness, T_avg, delta T, guard and shunting factors, heat flux, total thermal resistance | CALIBRATION fluids/vacuum, no fluoride salt | ORNL-4831 Appendix B | 87-98 | B-12 to B-22 |

**ORNL-4831 contains no fluoride-salt measurements** (checked 2026-10-09: the report's conclusions, printed page 45, say the fluoride-salt results "will be reported in a later publication"; Tables B-1 to B-22 cover only H2O, Hg, HTS, He, Ar and vacuum). The files above are calibration data, every row carries `role = calibration`. Raw (uncorrected) Tables B-1 to B-11 are not transcribed.

**Bases.** The ORNL-TM-2316 CSVs carry a per-row `basis` column (`estimated`, `measured`, `preliminary_measurement`, `see_note`); most F1-F4 values and many C1/C2 values are estimates, not measurements. The ORNL-TM-2316 report has no numbered tables; the earlier "Table 22.1"-style numbers were invented and have been removed. The CSVs were audited on 2026-10-09: see `../AUDIT-2026-10-09.md`.

---

## Salt Compositions

### MSRE Fuel and Related Mixtures

| Symbol | LiF | BeF₂ | ThF₄ | UF₄ | Liquidus Temp. (°C) | Notes |
|--------|-----|------|------|-----|-------------------|-------|
| **F1** | 73  | 16   | 10.7 | 0.3 | 500 ± 5   | Single-region fuel design |
| **F2** | 72  | 21   | 6.7  | 0.3 | 500 ± 5   | Single-region fuel design, lower Th |
| **F3** | 68  | 20   | 11.7 | 0.3 | 480 ± 5   | Single-region fuel design, lower LiF |
| **F4** | 63  | 25   | 11.7 | 0.3 | 500 ± 5   | Single-region fuel design, higher Be |

### Coolant and Flush Salts

| Symbol | LiF | BeF₂ | NaBF₄ | NaF | Liquidus Temp. (°C) | Notes |
|--------|-----|------|-------|-----|-------------------|-------|
| **L₂B** | 66  | 34   | —     | —   | 458 ± 1 (peritectic) | Present MSRE coolant; flush salt |
| **C₁** | —   | —    | 92    | 8   | 385 ± 1 (eutectic)  | Fluoroborate coolant candidate |
| **C₂** | —   | —    | 100   | —   | 407 ± 1 (melting point) | Pure NaBF₄ (theoretical only) |

---

## Reported Correlations & Measured Values

### 1. **Heat Capacity** (ORNL-TM-2316, page 22)

#### Measured Data (Table 1)

Liquid and solid heat capacities at constant pressure, with temperature-dependent linear terms.

- **F₁–F₄ liquid:** estimated by mole-fraction additivity  
  - Cp = 0.34 – 0.39 cal/(g·°C)  
  - Liquid Cp constant; uncertainties ±4 %
  - Solid phase shows temperature dependence: Cp,solid = a + b·t  

- **F₁–F₄ solid:** from F₁–F₄  
  - Example: F₁ solid Cp = 0.22 + 12.7×10⁻⁵·t cal/(g·°C)  
  - Applicable 25–440 °C

- **L₂B (LiF-BeF₂):** liquid Cp is the average of two measurement sets (Hoffman 0.577, Douglas and Payne 0.56); the table prints 0.57  
  - Liquid: 0.57 cal/(g·°C) ± 3%  
  - Solid: 0.317 + 3.61×10⁻⁴·t (25–472°C) ± 3%

- **C₁, C₂ (fluoroborate):**  
  - C₁ liquid: 0.360 cal/(g·°C) ± 2%  
  - Solids with two ranges (25–243, 243–406°C): see CSV

> **Not re-checked (2026-10-09 audit):** the heat-content equations below, the per-salt liquidus temperatures in the composition tables, the solid-Cp example text and the "Predicted Range" viscosity numbers were not compared against the page images; the audit covered the CSVs and the statements corrected in this README. Treat them as unverified.

#### Heat Content Equations (for total enthalpy from room temperature)

**LiF-BeF₂-ThF₄ (72-16-12 mol %):**  
```
Solid (25–440°C):
  H_L - H₂₅ = -5.28 + 0.207t + 6.33×10⁻⁵·t²
  
Liquid (500–750°C):
  H_L - H₂₅ = 11.34 + 0.324t
```

**LiF-BeF₂ (66-34 mol %):**  
```
Solid (0–472°C):
  H_L - H₀₀C = 0.3179t - 1.806×10⁻⁴·t²
  
Liquid (472–600°C):
  H_L - H₀₀C = 32.632 + 0.561t
  Or (from alternative source):
  H_L - H₃₀ = 33.62 + 0.577(t - 30)
```

**NaBF₄-NaF (92-8 mol %):**  
```
Solid (25–243°C):
  H_L - H₂₅ = -5.90 + 0.230t + 2.90×10⁻⁴·t²
  
Solid (243–381°C):
  H_L - H₂₅ = 0.40 + 0.337t
  
Liquid (381–600°C):
  H_L - H₂₅ = 22.1 + 0.360t
```

**Source & Notes:**  
- Measured on defined compositions; constants valid only for stated compositions
- Uncertainties reflect combination of systematic and random errors
- T in °C; H in cal/g

---

### 2. **Liquid Density** (ORNL-TM-2316, page 28)

#### Correlation: ρ(T) Linear, ρ = a − b·T

| Salt | Composition (mol %) | a (g/cm³) | b (10⁻⁴/°C) | Uncertainty |
|------|---------------------|-----------|------------|-------------|
| **F₁** | 73-16-10.7-0.3 | 3.628 | 6.6 | ± 3% |
| **F₂** | 72-21-6.7-0.3 | 3.153 | 5.8 | ± 3% |
| **F₃** | 68-20-11.7-0.3 | 3.687 | 6.5 | ± 3% |
| **F₄** | 63-25-11.7-0.3 | 3.644 | 6.3 | ± 3% |
| **L₂B** | 66-34 | 2.214 | 4.2 | ± 2% |
| **C₁** | 92-8 | 2.27 | 7.4 | ± 5% |
| **C₂** | 100-0 | 2.26 | 7.4 | ± 5% |

**Equation:**  
ρ(in g/cm³) = a − b×10⁻⁴·(t in °C)

**Method:**  
- F₁–F₄: estimated by additivity of molar volumes at 600 and 800°C
- L₂B: equation derived from additive molar volumes; it gives densities approximately the average of two experimental sets (refs 5, 6); three determinations exist
- C₁, C₂: preliminary pycnometric measurements

**Notes:**
- Three L₂B sources showed 3% scatter; ref. 6 (~3% high)
- Reference 4 (MSR Program, 1965) reported C₁ densities at 649°C varying 1.87–2.02 g/cm³
- NaF molar-volume contribution subtracted from C₁ to obtain the C₂ equation (not the other way round)

---

### 3. **Viscosity** (ORNL-TM-2316, page 8)

#### Correlation: η = A·exp(B/T), T in °K

| Salt | A (cP) | B (°K) | Uncertainty | Source |
|------|--------|--------|------------|--------|
| **F₁** | 0.084 | 4340 | ± 25% | Estimated from LiF-BeF₂-UF₄ data |
| **F₂** | 0.072 | 4370 | ± 25% | Estimated from LiF-BeF₂-UF₄ data |
| **F₃** | 0.077 | 4430 | ± 25% | Estimated from LiF-BeF₂-UF₄ data |
| **F₄** | 0.0444 | 5030 | ± 25% | Estimated from LiF-BeF₂-UF₄ data |
| **L₂B** | 0.116 | 3755 | ± 15% | **Measured** |
| **C₁, C₂** | 0.04 | 3000 | ± 50% | Estimated (preliminary NaBF₄ measurements plus NaI analogy) |

**Method:**
- F₁–F₄: empirical correlation from viscosities in related systems; assumed UF₄ effect ≈ ThF₄ effect
- L₂B: measured values
- C₁, C₂: derived from preliminary NaBF₄ measurements and analogy to NaI

**Predicted Range (assuming 62–73 mol % LiF, 15–30 mol % BeF₂):**
- At 600°C: 9–16 cP
- At 700°C: 5–9 cP

---

### 4. **Thermal Conductivity** (ORNL-TM-2316, page 11)

#### Measured/Estimated Values

| Salt | k (W/(cm·°C)) | Uncertainty | Status |
|------|--------------|------------|--------|
| **F₁** | 0.010 | ≥ ± 25% | Estimated from theory |
| **F₂** | 0.011 | ≥ ± 25% | Estimated from theory |
| **F₃** | 0.0083 | ≥ ± 25% | Estimated from theory |
| **F₄** | 0.0070 | ≥ ± 25% | Estimated from theory |
| **L₂B** | 0.010 | ± 10% | **Measured** |
| **C₁** | 0.0052 | ± 50% | see CSV note: report says only that a very preliminary measurement on C₂ agrees with the theoretical expression |
| **C₂** | 0.0051 | ± 50% | see CSV note (as C₁) |

**Method (F₁–F₄):**  
Theory (Rao, adapted by Turnbull):
```
k = 11.9×10⁻³ · T_m^(1/2) · ρ_m^(2/3) / (M/n)^(7/6)
```
where  
- T_m = melting point (°K)
- ρ_m = liquid density (g/cm³) at T_m  
- M = average molar mass (g/mol)  
- n = average number of discrete ions per molecule

For two fluoride melts tested (L₂B and LiF-BeF₂-ThF₄-UF₄ 71.2-23-5-0.8), the theoretical expression gave values ~25% lower than experiment (printed page 12); 15% was added to the estimated F₁–F₄ values. (~~15% lower~~ **CORRECTED 2026-10-09**, checked against the page image.)

**Notes:**
- 15% was added to the F₁–F₄ estimates because of the ~25% shortfall seen for two fluoride melts  
- L₂B value is reported as measured (ref 3); the report does not call it an average
- C₁, C₂: "Very preliminary measurement on C₂ agrees with the theoretical expression"

**Important:** Thermal conductivity of F₁–F₄ values are theoretical estimates and should not be treated as measured. Relative ordering (F₁ > F₂ > F₃ > F₄) is estimated.

---

### 5. **Electrical Conductivity** (ORNL-TM-2316, page 14)

#### Correlation: κ = κ₀ + a(t − 500), t in °C

| Salt | κ₀ (ohm·cm)⁻¹ | a (×10⁻³/(ohm·cm·°C)) | Uncertainty |
|------|--------|-------|------------|
| **F₁** | 1.72 | 8.0 | ± 20% |
| **F₂** | 1.63 | 7.3 | ± 20% |
| **F₃** | 1.66 | 6.4 | ± 20% |
| **F₄** | 1.94 | 7.1 | ± 20% |
| **L₂B** | 1.54 | 6.0 | ± 10% |
| **C₁** | 2.7 | 13 | ± 50% |
| **C₂** | 1.92 | 2.6 | ± 20% |

**Method:**  
- F₁–F₄: estimated empirically from related salt systems; self-consistent correlation attempted
- L₂B: preliminary measurements (ref 4); report: 6 of 7 salts estimated
- C₁: interpolated from literature NaF-NaBF₄ data; C₂: KBF₄/NaI analogy

**Equation:**  
κ(in (ohm·cm)⁻¹) = κ₀ + 10⁻³·a·(t °C − 500)

**Notes:**
- Number of significant figures indicates relative differences between salts, not precision
- Significant uncertainties reflect data scatter and estimation methodology

---

### 6. **Heat of Fusion** (ORNL-TM-2316, page 25)

#### Measured & Estimated Values

| Salt | ΔH_fus (cal/g) | Uncertainty | Status | Notes |
|------|---------------|------------|--------|-------|
| **F₁** | 62 | ± 10% | Estimated | Assuming isothermal melt at 500°C |
| **F₂** | 67 | ± 15% | Estimated | |
| **F₃** | 58 | ± 15% | Estimated | |
| **F₄** | 63 | ± 15% | Estimated | |
| **L₂B** | 107 | ± 3% | **Measured** | Report says only "Measured" (the "average of two" claim belongs to the L₂B heat capacity) |
| **C₁** | 31 | ± 2% | **Measured** | |
| **C₂** | 29 | ± 2% | **Measured** | |

**Solid Phase Transitions:**
- C₁ at 243°C: ΔH = 14.5 cal/g (± 2%)
- C₂ at 243°C: ΔH = 14.7 cal/g (± 2%)

**Method (F₁–F₄):**  
- Treated as additive mixtures of components
- Individual component heats of fusion used:
  - Li₂BeF₄: 10,600 cal/mol
  - Li₃ThF₇: 13,960 cal/mol  
  - LiF: 6,470 cal/mol
  - ThF₄: 11,000 cal/mol (estimated)

**Important Note:**  
For F₁–F₄ to fully melt from solidus to liquidus, **add 10–15 cal/g** to the above values (non-isothermal transition).

---

## Figures NOT Digitised

The following figures and graphs from ORNL-TM-2316 and related reports contain important data and are referenced but not transcribed to CSV:

| Report | Fig. # | Page | Title | Content |
|--------|--------|------|-------|---------|
| ORNL-TM-2316 | Fig. 1 (unlabeled) | 18 | Phase diagram | LiF-BeF₂-ThF₄ ternary phase diagram |
| ORNL-TM-2316 | Fig. 2 (unlabeled) | 21 | Phase diagram | NaBF₄-NaF binary phase diagram |
| ORNL-TM-2316 | Appendix A | 43 | Isochoric heat capacity graph | C_v, C_p/C_v, sonic velocity vs. temperature |
| ORNL-TM-2316 | Appendix B | 44 | Derived properties graph | Thermal diffusivity, kinematic viscosity, Prandtl number |
| ORNL-4831 | Fig. 2 | 10 | Radiative function graph | Y vs optical thickness τ for various emissivities |
| ORNL-4831 | Fig. 3 | 12 | Thermal resistance graph | Thermal resistance vs. specimen thickness (infrared absorbing fluid) |
| ORNL-4831 | Figs. 16–20 | 35–37 | Calibration curves | Total thermal resistance vs. specimen thickness for Ar, He, H₂O, HTS, Hg |

**Count:** 7 figure groups / figure ranges not digitised. (Experimental data tables are tables, not figures: ORNL-4831 Table 2 and the reduced Appendix B Tables B-12 to B-22 are transcribed above; raw Tables B-1 to B-11 are not.)

---

## Data Quality & Limitations

### Measured vs. Estimated

- **Measured:** heat capacity (L₂B, C₁, C₂), viscosity (L₂B), heat of fusion (L₂B, C₁, C₂), thermal conductivity (L₂B). L₂B density is a molar-volume-additivity equation fitted near measured data, not a direct fit.
- **Estimated from theory/correlations:** F₁–F₄ thermal conductivity, all F₁–F₄ electrical conductivity, most F₁–F₄ viscosity
- **Preliminary/uncertain:** C₁ density (preliminary pycnometric), C₁/C₂ thermal conductivity and viscosity, L₂B electrical conductivity

### Uncertainties

Uncertainties listed in the original document represent:
> "the largest probable combination of systematic and random errors associated with the value given for the property."

They are **not** goodness-of-fit statistics for interpolations or uncertainties for values computed from correlations at temperatures within stated range.

### OCR Artifacts

Transcription was performed from OCR'd scan images with spot checks against PDF images. Possible OCR errors:
- Small numerical digits (0, 1, 6, 8) in subscripts and exponents
- Greek letters (Δ, η, κ, ρ, σ, τ) mapped to Latin equivalents with notes
- Superscripts and subscripts may be incorrectly positioned

**Check page images before use in calculations.**

---

## References

1. **ORNL-TM-2316.pdf** (August 1968)  
   *Physical Properties of Molten-Salt Reactor Fuel, Coolant, and Flush Salts*  
   Edited by S. Cantor; Contributors: S. Cantor, J. W. Cooke, A. S. Dworkin, G. D. Robbins, R. E. Thoma, G. M. Watson  
   Oak Ridge National Laboratory, operated by Union Carbide Corporation for the U.S. Atomic Energy Commission.

2. **ORNL-4831.pdf** (February 1973)  
   *Development of the Variable-Gap Technique for Measuring the Thermal Conductivity of Fluoride Salt Mixtures*  
   By J. W. Cooke  
   Oak Ridge National Laboratory. Reactor Technology Program.

3. **ORNL-TM-3777.pdf** (December 1972)  
   *Heat Transfer Salt for High-Temperature Steam Generation*  
   [Not yet transcribed; reference placeholder]

---

## Recommended Citation

```
Cantor, S. et al. (1968). Physical Properties of Molten-Salt Reactor Fuel, Coolant, 
and Flush Salts. ORNL-TM-2316. Oak Ridge National Laboratory.

[AI transcription by Claude Haiku, 2026-10-09, for research/education/V&V only]
```

---

**Last updated:** 2026-10-09  
**Transcribed by:** Claude Haiku 4.5  
**Audited:** 2026-10-09 by Claude Sonnet (AI, not a human review), see `../AUDIT-2026-10-09.md`  
**Status:** DRAFT — NOT HUMAN-REVIEWED  
**Intended use:** Research, education, and verification & validation only  
**License:** Original reports are public domain (U.S. Government documents)
