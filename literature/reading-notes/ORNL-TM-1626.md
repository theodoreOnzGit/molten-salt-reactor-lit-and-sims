# ORNL-TM-1626 — Period Measurements on the MSRE During Fuel Circulation: Theory and Experiment

B. E. Prince, October 1966. PDF: [`../moltensalt-org/ornl-tm/ORNL-TM-1626.pdf`](../moltensalt-org/ornl-tm/ORNL-TM-1626.pdf).

> **AI-written reading note, NOT human-reviewed.** Pages are the report's
> printed numbers, with the PDF page where it was recorded.

**Read how far:** the line-model equations (pp. 7–15), Fig. 4 (p. 21), Table 1
(p. 24) and the comparison with measurement (p. 26), for OUTRAM PARK gh:#1130
and gh:#1330 (2026-11-03 to 11-10). The rest was not read closely.

## What is where

| Item | Where | What it says |
|---|---|---|
| Line model | Eqs. 14 and 24–27, pp. 7–15 | Precursors carried by plug flow through the lower plenum, graphite region, upper plenum and external loop. Precursors are made in proportion to the fission density in the salt, and their neutrons are weighted by the fast adjoint (p. 20). Prince treats the lower plenum as a well-stirred tank. |
| Fig. 4 | p. 21 (PDF p. 27) | Axial fission density in the salt and the fast-group adjoint (calculated curves). Digitised by eye in the backend, reading error about ±0.02 in amplitude. |
| Table 1 | p. 24 (PDF p. 30) | Six delayed families, 10⁴βᵢ = 2.23, 14.57, 13.07, 26.28, 7.66, 2.80 (sum 0.006661). Region residence times 3.8, 9.4, 3.9 and 8.1 s (loop 25.2 s). |
| Measured loss | p. 26 (PDF p. 32) | Reactivity lost to circulation, U-235 fuel, zero power: **0.212 ± 0.004 % δk/k**. Prince's own line model gives 0.222 % (Eq. 14, w = 0, same page). |

## Where it is used

In `outram-park-backend`:

| Code | What it does with TM-1626 | Result |
|---|---|---|
| `outram-park-fork-moltres/tests/msre_n4_precursor_drift_tm1626.rs` | Moltres's precursor drift on Fig. 4 and Table 1, against an exact integral | Moltres 0.2293 % against the exact line model's 0.2293 % |
| `outram-park-digital-twin-engine` `msre::circulation` | The MSRE simulator's default kinetics, the time-domain line model | 0.2291 % |
| `msre::moltres_drift` | Moltres's drift run live beside the plant (gh:#1575) | — |

All of these sit about 8 % above the measured 0.212 %. That gap is recorded
on gh:#1130, not tuned away.
