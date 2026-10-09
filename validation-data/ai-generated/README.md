# AI-generated validation data corner

> **AI-GENERATED, NOT HUMAN-REVIEWED.** Every number below was transcribed or
> digitised by an AI agent on 2026-10-09 from the public-domain reports in
> [`literature/`](../../literature/moltensalt-org/README.md). Treat it as
> untrusted draft data until a human has checked it against the cited page.
> Research, education and V&V only.

| Topic | Datasets | Agent | AI audit | Figures listed as not digitised |
|---|---|---|---|---|
| [Neutronics](neutronics/README.md) | 16 | Claude Sonnet | none | yes, in the README |
| [Transients](transients/README.md) | 8 | Claude Sonnet | none | 14 |
| [Thermal hydraulics](thermal-hydraulics/README.md) | 21 | Claude Sonnet | none | about 17 |
| [Materials](materials/README.md) | 6 | Claude Haiku | Sonnet, cell by cell ([log](AUDIT-2026-10-09.md)) | ORNL-TM-1017 Figs. 1–19; ORNL-TM-1997 listed only in general terms |
| [Salt properties](salt-properties/README.md) | 18 | Claude Haiku, then Claude Sonnet | Sonnet, cell by cell ([log](AUDIT-2026-10-09.md)) | yes, in the README |

Each topic README opens with the same label. It lists its datasets with source
page, table or figure, and method, and has a **"Figures NOT digitised"**
section giving the reason for each figure skipped. Each CSV repeats the source
and label in its `#` header.

## What to know before using any of it

- **Haiku's first pass was not trustworthy.** The audit found 44 wrong value
  cells in 23 rows of the materials tables. Some of those cells were blank in
  the original and had values **invented** for them. It also found 9 omitted
  rows, made-up temperature ranges, and mislabelled salt compositions. All of
  these were fixed, and the log records each old → new value. **A second AI
  pass can share the first one's blind spots.** Have a human spot-check these
  folders first.
- **Sonnet's folders were not audited.** The agents checked their own tables
  against page images, but no independent pass has been made.
- **Digitised figures carry a reading uncertainty** in their CSV header.
  Values come from page images of 1960s scans, so treat them as ± a gridline
  fraction, not as tabulated data.
- **Measured, calculated and estimated values are labelled per dataset or per
  row.** Validate only against measured values (see
  [`AGENTS.md`](../../AGENTS.md) §3).
- **ORNL-4831 has no fluoride-salt conductivity data.** Its salt results were
  deferred to a later publication; only the calibration fluids are here.

## Known gaps

- Pump start-up and coast-down transients, and natural-circulation tests, were
  not found in the reports searched (benchmarks T-6, T-7).
- Materials: ORNL-TM-2359, -2647, -3063 (surveillance groups 2–4), ORNL-TM-4174
  and ORNL-4829 are not yet transcribed.
- Dense frequency-response figures (ORNL-TM-1647 Figs. 21–28, ORNL-TM-2997
  Figs. 5–10) need manual marker picking.
