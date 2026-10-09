# Public-domain molten salt reactor reports (mirrored from moltensalt.org)

293 reports from the U.S. Atomic Energy Commission era, 1949–1977, mirrored from
<https://moltensalt.org/references/static/downloads/pdf/index.html> on
2026-10-09. Titles, dates, sizes and SHA-256 hashes are in
[`MANIFEST.md`](MANIFEST.md).

| Folder | Count | Series |
|---|---|---|
| [`ornl/`](ornl/) | 101 | ORNL numbered reports (MSRE design and operations, MSBR studies, MSRP and ANP progress reports) |
| [`ornl-tm/`](ornl-tm/) | 183 | ORNL technical memoranda |
| [`other-us-gov/`](other-us-gov/) | 9 | ANL, LA, MIT, TID and WASH reports |

## Why these are treated as public domain

**This is a reasoned copyright assessment, not legal advice.** If you know of a
reason any file here should not be redistributed, open an issue.

- **Government-sponsored work published before 1978.** Each report was
  prepared for the AEC by a contractor (Union Carbide Nuclear Division for
  ORNL, and others) or by AEC staff. Under the U.S. 1909 Copyright Act, a work
  published without a copyright notice before 1 January 1978 entered the
  public domain. These reports carry the AEC "prepared as an account of
  Government sponsored work" disclaimer instead of a notice.
- **Every file was scanned for a notice.** The text of the first six and last
  two pages of each PDF was searched for `copyright`, `©`,
  `all rights reserved` and `royalty-free`. Two hits were real notices and
  those files were **excluded** (below). One hit (ORNL-3470) was OCR noise and
  the file was kept.
- **Limits of that scan.** Many PDFs are scans with an OCR text layer, and OCR
  can miss a notice. The scan looks only at the front and back pages.
- **Only clear cases are kept.** Every file is an AEC laboratory or
  AEC-issued report series (ORNL, ANL, LA, MIT under AEC contract, TID, WASH).
  Twenty files whose claim was weaker were removed and erased from the git
  history on 2026-10-09 (below).

## What was deliberately left out

From the same source page (412 PDFs in total):

| Excluded | Reason |
|---|---|
| *Fluid Fuel Reactors* (Addison-Wesley 1958, 28 files) | © 1958, assigned to the AEC General Manager; all rights reserved. DOE granted a non-exclusive republication licence to one person in 2018 ([fluidfuelreactors.com](https://fluidfuelreactors.com/)) |
| *Nuclear Applications & Technology* 1970 (9 files) | ANS journal; renewal was automatic for 1964–77 works |
| *Nuclear Science & Engineering* 1957 (4 files) | ANS journal; renewal status not checked |
| ORNL reports dated 1978 onwards (17 files) | Post-1977 contractor works; the government holds only a licence |
| JPL-TR-32-198 | "Copyright © 1962 Jet Propulsion Laboratory, California Institute of Technology" |
| NAS-NS-3060, NAS-NS-3063 | Copyright licence notice found (3060); dated 1986 (3063) |
| **Removed 2026-10-09, weak claim:** ORNL Central Files memoranda (11 files, `ORNL-CF-*`) | Internal memoranda; whether they were ever *published* in the 1909-Act sense is doubtful |
| **Removed 2026-10-09, weak claim:** NAS-NS-3004, -3013, -3016, -3025, -3050, -3058 | Written and published by the National Academy of Sciences (a private body); the AEC only sponsored printing, and NAS-NS-3060 in the same series carries a copyright-licence notice |
| **Removed 2026-10-09, weak claim:** SL-1954 | Report by a private firm (Sargent & Lundy) to ORNL; no government notice found |
| **Removed 2026-10-09, weak claim:** ORAU-IEA-77-13 | Oak Ridge Associated Universities is not a federal agency; no sponsorship notice found |
| **Removed 2026-10-09, weak claim:** ORNL-MIT-117 | MIT Practice School student report; no notice found |
| IAEA TECDOCs | © IAEA |
| EIR (Swiss), AEEW (UKAEA), CNRS, ICAPP, UCLA, MSR-FUJI and undated papers | Not U.S.-government works; no public-domain basis |

## Intended use

Research, education and verification & validation only. Nothing here is
authoritative for facility operation, licensing or safety decisions.
