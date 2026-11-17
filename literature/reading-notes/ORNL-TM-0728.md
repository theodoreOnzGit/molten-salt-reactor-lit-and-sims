# ORNL-TM-0728 — MSRE Design and Operations Report, Part I: Description of Reactor Design

R. C. Robertson, January 1965. PDF: [`../moltensalt-org/ornl-tm/ORNL-TM-0728.pdf`](../moltensalt-org/ornl-tm/ORNL-TM-0728.pdf) (575 PDF pages).

> **AI-written reading note, NOT human-reviewed.** "PDF p." is the page in
> the PDF; printed page numbers differ. Found by searching the PDF's text
> layer (an OCR of a scan, with broken spacing), then reading the sentence
> around each hit.

**Read how far (2026-11-14):** targeted searches for plant sizes and
elevations, for the OUTRAM PARK MSRE plant drawing (`bugis::components::
msre_plant`). Fig. 5.2 (reactor cell elevation) was read in detail earlier,
for the backend's Monte Carlo model (gh:#1128). The rest of the report was not
read.

## Plant sizes and elevations

| Item | As printed | Where |
|---|---|---|
| Reactor cell (containment vessel) | 24 ft in diameter, 33 ft overall height, hemispherical bottom and flat top; el. 819–852 ft | §4.3.1, PDF p. 75 |
| Reactor vessel | 59⅛ in OD | Table 5.2 |
| Graphite core | 55¼ in diameter, 64 in high | Table 5.4 |
| Thermal shield | about 10.4 ft OD × 7.8 ft ID × 12.5 ft high overall; 18 in base, 151 in cylinder, 16 in cover | §5.3.10.1, PDF p. 162 |
| Fuel pump | bowl about 36 in in diameter; pump and 75 hp motor about 8 ft high | §2.4.2, PDF p. 44 |
| Fuel pump salt level | overflow pipe top 1½ in above the normal level, "an elevation of about 840 ft 3 in" | §5.4.7.1, PDF p. 195 |
| Fuel pump bowl | "at the highest elevation in the primary circulating system" | PDF p. 96 |
| Heat exchanger | horizontal shell-and-tube, about 16 in in diameter and 8 ft long, 163 U-tubes of ½ in OD | §2.4.3, PDF p. 46 |
| Heat exchanger and fuel piping | pitch downward about 3° to drain salt to the reactor vessel | PDF p. 96 |
| Coolant pump | bowl centreline at 833 ft 6 in, the highest point in the coolant system | §8, PDF p. 315 |
| Radiator | bottom at el. 836 ft 1¾ in | §4.3, PDF p. 86 |
| Radiator tubes | 120 tubes, ¾ in OD × 30 ft long, in serpentine coils between two headers | §8, PDF p. 317 |
| Drain tank cell | 17 ft 7 in × 21 ft 2½ in × 20 ft 6 in high | PDF p. 478 |
| Drain tank cell floor | 814 ft at the highest point | §4.3.2, PDF p. 84 |
| Fuel drain tanks | 50 in in diameter, about 86 in high, not including the steam dome | §6, PDF p. 266 |
| Coolant drain tank | at the 820 ft elevation, "almost directly beneath the radiator" | §9.1, PDF p. 353; §8.1 |
| Drainage | cells laid out at "relative elevations so that the reactor could drain by gravity" | PDF p. 259 |

## Coolant loop order

§8.2, PDF p. 317:
- The coolant pump discharges into **line 200**, which enters the reactor
  cell to the fuel heat exchanger.
- **Line 201** carries the salt from the heat exchanger to the radiator's
  top header.
- The salt flows down through the radiator and back to the pump.

## Not found in the text

These were searched for and not found:
- the heat exchanger's elevation;
- the flush tank's size;
- the coolant drain tank's size;
- the radiator housing's size.

Fig. 5.2 and the drawings it cites may have them. The backend widget marks
all four as indicative.
