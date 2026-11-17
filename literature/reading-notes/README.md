# Reading notes: breadcrumbs through the literature

> **AI-written reading notes, NOT human-reviewed.** Every claim names the
> report and page so a reader can check it in a minute. Treat these notes as
> pointers into the reports, never as a substitute for them. Nothing here is
> for operation, licensing or safety decisions.

The maintainer asked agents to leave breadcrumbs as they read (2026-11-17).
A note records **where** something is, **what it says**, and **how far it was
read**, so the next reader (human or agent) starts from the page instead of
from the top of a 500-page report.

## Notes

| Report | What the note holds | Read how far |
|---|---|---|
| [ORNL-TM-0728](ORNL-TM-0728.md) | MSRE plant sizes and elevations (cell, vessel, pumps, heat exchanger, radiator, drain tanks), the coolant loop order, "drain by gravity" | Targeted search of the full text; Fig. 5.2 read in detail |
| [ORNL-TM-1626](ORNL-TM-1626.md) | Circulating-fuel reactivity loss: the line model, Table 1 residence times and delayed families, Fig. 4 shapes, the measured 0.212 % | Eqs. 14 and 24–27 and Table 1, Fig. 4 digitised |
| [ORNL-4449](ORNL-4449.md) | U-233 operation of the MSRE; the switch from two-fluid to one-fluid breeder; where the thorium and protactinium chemistry is | Introduction (p. x) read in full; contents scanned |
| [ORNL-4396](ORNL-4396.md) | MSBR one-fluid reference design and the thorium / protactinium chemistry programme, early 1969 | Contents and introduction scanned |
| [ORNL-TM-2571 and ORNL-TM-2997](ORNL-TM-2571-and-2997.md) | MSRE dynamics with U-233 fuel: theory and experiment | Title pages only |

## Trails

### Thorium, U-233 and breeding (towards a LFTR)

1. **The MSRE never bred.** Its fuel salt held no thorium and it had no
   blanket. It **ran on U-233** bred elsewhere (Savannah River reactors) in
   1968–69: [ORNL-4449, p. x](ORNL-4449.md#u-233-operation).
2. **Why one fluid.** ORNL moved its breeder work from two-fluid (graphite
   tubes separating fuel and fertile salt) to one-fluid in late 1967, for two
   reasons given on [ORNL-4449, p. x](ORNL-4449.md#why-the-breeder-became-one-fluid).
3. **The thorium chemistry** (ThF₄ phase diagrams, protactinium and thorium
   extraction into bismuth or lead, rare-earth removal) is lab work for the
   MSBR, spread through the semiannual reports:
   [ORNL-4396](ORNL-4396.md#thorium-and-protactinium-sections) and
   [ORNL-4449](ORNL-4449.md#thorium-and-protactinium-sections).
4. **The breeder design itself** is ORNL-4541, *Conceptual Design Study of a
   Single-Fluid Molten-Salt Breeder Reactor* (1971), mirrored at
   [`../moltensalt-org/ornl/ORNL-4541.pdf`](../moltensalt-org/ornl/ORNL-4541.pdf).
   **Not yet read.** It is the next stop on this trail.
5. **What OUTRAM PARK lacks for it** (checked 2026-11-17 in the backend's
   `reference-data/endf/`): no Th-232, Pa-233 or U-233 evaluations. It has a
   depletion module (`outram_mc_libs::depletion`: Bateman equations, CRAM16,
   a burnup loop); whether it handles continuous salt processing (Pa-233
   removal) was **not checked**.

### MSRE plant geometry

[ORNL-TM-0728](ORNL-TM-0728.md) for sizes and elevations. The backend's
`bugis::components::msre_plant` widget is drawn from these numbers, and
`outram_mc_libs::vv::msre_n1` builds the core from TM-0728's tables.

### Circulating-fuel kinetics

[ORNL-TM-1626](ORNL-TM-1626.md), then TM-1070 (frequency response) and
TM-1647 (measured periods), which have no notes yet.

## How to add a breadcrumb

- One file per report, named by its report number. Link it in the table above.
- For each item: the page (printed page, and the PDF page where they differ),
  what it says in a sentence, and any number with its units as printed.
- Say **how far you read**. "Contents only" is a useful note; a note that
  hides how little was read is not.
- If the report contradicts itself or another source, record both and say
  which you trust and why. Do not fix the report.
- Keep the AI-written label until a human has checked the page references.
