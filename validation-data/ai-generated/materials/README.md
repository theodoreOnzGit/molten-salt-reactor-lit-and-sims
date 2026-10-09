# AI-Generated Materials Validation Data

**AI-GENERATED, NOT HUMAN-REVIEWED.** Transcribed by an AI agent (Claude Haiku) on 2026-10-09 from public-domain ORNL reports. Treat every number as untrusted draft data until a human has checked it against the cited page. Research, education and V&V only.

## Datasets

| CSV File | Rows | Quantity | Source Report | Table # |
|----------|------|----------|---|---|
| ORNL-TM-1017_table2_tensile_properties.csv | 8 | Average tensile properties vs temperature (21–982°C), heats 5075/5081 | ORNL-TM-1017 | 2 |
| ORNL-TM-1017_table3_creep_rupture_all_temps.csv | 21 | Creep-rupture data for Heat 5055 at 593, 704, 816°C | ORNL-TM-1017 | 3 |
| ORNL-TM-1017_table4_creep_rupture_heat5075.csv | 23 | Creep-rupture data for Heat 5075 at 593, 704, 816°C | ORNL-TM-1017 | 4 |
| ORNL-TM-1017_table5_creep_rupture_heat5081.csv | 21 | Creep-rupture data for Heat 5081 at 593, 704, 816°C | ORNL-TM-1017 | 5 |
| ORNL-TM-1997_table9_creep_rupture_heat5081_650C.csv | 10 | Creep-rupture of unirradiated Heat 5081 at 650°C (annealed; annealed + 4800 hr in MSRE salt) | ORNL-TM-1997 | 9 |
| ORNL-TM-1997_table10_creep_rupture_heat5085_650C.csv | 17 | Creep-rupture of unirradiated Heat 5085 at 650°C (as-received; annealed; annealed + 4800 hr in MSRE salt) | ORNL-TM-1997 | 10 |

## Sources

- **ORNL-TM-1017** (1965): "Tensile and Creep Properties of INOR-8 for the MSRE," J. T. Venard. Three heats (5055, 5075, 5081) tested at room temperature to 982°C.
- **ORNL-TM-1997** (1967): "An Evaluation of the MSRE Hastelloy N Surveillance Specimens – First Group," H. E. McCoy, Jr. First group removed after 7823 Mwhr of reactor operation (Sept 1965–July 1966). Heats 5081 and 5085, tested at 650°C.

## Data Format

- Machine-readable scientific notation: `3.5e-2` for `3.5 × 10⁻²`
- Original units preserved (psi for stress, hours for time, percent for strain/elongation, hr⁻¹ for creep rate)
- Blank cells indicate no value recorded or test discontinued
- All rows validated to have correct number of fields per CSV header

## Figures NOT Digitised

**ORNL-TM-1017** (19 figures):
- Figs. 1–3: Metallography (as-received microstructure, 100x, heats 5055/5075/5081)
- Fig. 4: Technical drawing (creep and tensile specimen dimensions)
- Figs. 5–8: Property plots (ultimate strength, 0.2% yield, elongation, reduction of area vs temperature, scatter bands)
- Figs. 9–19: Stress-rupture life plots (log stress vs log time to rupture, three temperatures, three heats, various strain levels)

**ORNL-TM-1997**: Microstructure images, facility diagrams, surveillance fixture schematic, tensile property plots, post-irradiation examination micrographs not extracted.

## Known Data Notes

- **Table 3, row 2273** (704°C, 52 ksi) and row 2267: the 0.2 % to 2.0 % strain-time cells are blank in the original (~~the earlier note and values were wrong~~ **CORRECTED 2026-10-09**, see `../AUDIT-2026-10-09.md`)
- **Tables 4 and 5**: blank cells are blank in the printed tables; the earlier note that rupture cells of rows 3142/2547 were blank was false (values 8.9 and 78.6 hr are printed)
- **Table 5, row 2466**: all strain-time cells blank in the original; only rupture time, creep rate and elongation are printed
- **ORNL-TM-1997 Tables 9 and 10**: tests 6160 and 6154 carry footnote a (discontinued prior to failure), marked with an `a` suffix on the test number
- **ORNL-TM-1997, Tables 9 & 10**: Creep rate units shown as %/hr in original (preserved as written)

## Incomplete / Pending Transcription

The following reports identified but not yet transcribed:
- **ORNL-TM-2359, ORNL-TM-2647, ORNL-TM-3063**: MSRE Hastelloy-N Surveillance Groups 2–4 (irradiated specimens)
- **ORNL-TM-4174**: Post-Irradiation Examination of Materials from the MSRE
- **ORNL-4829**: Intergranular Cracking of INOR-8 in the MSRE (corrosion/cracking depth data)

---

**Generated**: 2026-10-09  
**Total CSV files**: 6  
**Total data rows**: 100 (after the 2026-10-09 audit added 9 omitted rows)  
**Figures not digitised**: 19 (metallography, technical drawings, stress-rupture and property plots)  
**Status**: Draft – Requires human verification against source PDFs before use in publications

**Audited:** 2026-10-09 by Claude Sonnet (AI, not a human review); every cell compared with the page images, errors corrected in place. See `../AUDIT-2026-10-09.md`.
