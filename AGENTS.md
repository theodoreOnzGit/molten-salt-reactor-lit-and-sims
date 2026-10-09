# AGENTS.md

Guidance for AI agents (Claude, Codex and others) working in this repository.
These principles are condensed from the
[OUTRAM PARK backend](https://github.com/theodoreOnzGit/outram-park-backend)
workspace, whose models this repository builds on. Where this file and a
deeper document disagree, this file wins for this repository.

## What this repository is

- `literature/` — public-domain molten salt reactor reports, with the basis for
  treating each as public domain ([`literature/moltensalt-org/README.md`](literature/moltensalt-org/README.md)).
- `validation-data/ai-generated/` — experimental data transcribed or digitised
  from that literature **by AI agents and not yet reviewed by a human**.
- `benchmarks/` — benchmark specifications: neutronics, thermal hydraulics,
  materials and transients.
- `crates/` — a Rust workspace that depends on the OUTRAM PARK crates
  published on crates.io, where molten salt reactor models are built.

**Research, education and V&V only.** Nothing here is for facility operation,
reactor control, licensing, safety-critical or emergency decisions. Do not
frame any output as authoritative for those purposes.

## 1. Get the process right first; the answer comes second

**Never reason backwards from a target number.** Fix the inputs, the physics,
the instrument and the assumptions on their own merits, from the literature,
from upstream code or from first principles. Then report whatever answer that
produces. A right answer obtained by a wrong process carries no evidence: it
could not have failed, so it cannot inform.

This forbids:

- **tuning an input until a comparison passes**, because the benchmark is the
  check, and once it becomes an input the check is gone;
- **moving a threshold to make a test pass**, because when a gate fails, the
  first hypothesis is that the thing being gated is wrong;
- **choosing the measure after seeing the result**;
- **quietly repairing a published number.** Strike it through, give the new
  value, and say what changed it.

It requires:

1. **Every input is justified from outside the comparison**, with a citation.
2. **Fix the protocol, not the criterion.**
3. **The instrument matches the physics**, with the reason stated.
4. **Test the assumption the result rests on**, especially a convenient one.
5. **Report the disagreement.** An honestly derived 14 % miss is worth more
   than a fitted exact match.
6. **When several numbers exist, say which one to quote and why.**

## 2. Correct physics before correct results

- **Implement the governing physics.** Nothing outranks it: not runtime, not
  ergonomics, not agreement with a reference.
- **Correct physics is the default setting.** A term the data or equation
  supplies is applied unless a caller explicitly ablates it with a visible
  `without_*` builder or named switch. A physics term behind an off-by-default
  flag is physics the code does not have. Pin each default with a test.
- **When a default changes, re-measure every V&V number that depended on it**,
  including when the result gets worse. Correct physics is not chosen by
  whether it flatters a comparison.
- **Surrogates, in this order:** a physics-derived reduced-order model, run
  **uncalibrated** first, with its disagreement recorded. Only then calibrate,
  stating which parameters moved, over what range and against which data. Back
  every calibration with **ablation**: return each calibrated parameter to its
  derived value one at a time and report how much of the agreement it carried.
- **Never calibrate a free parameter until a comparison passes.**

## 3. Code-to-code verification certifies the translation, not the physics

- **Verification** asks whether the model was implemented correctly. Use unit
  tests, analytical solutions, conservation checks, manufactured solutions and
  **code-to-code comparison** against the reference implementation.
- **Validation** asks whether the model represents reality well enough for
  its purpose. It is done against **measured data** and benchmarks.
- **Both are required, and one never stands in for the other.** A faithful
  port reproduces upstream's simplifications exactly, so code-to-code
  agreement can be perfect while the physics is wrong. Equally, agreement with
  one experiment can hide an implementation error that cancels another.
- **When a ported model misbehaves, read upstream first.** Upstream is the
  specification. A missing limit, guard or format-flag branch is a far more
  common port defect than a wrong formula. State the predicted sign and
  magnitude of a fix before measuring it.
- **Check that the comparison measures what it claims.** A comparison on a
  grid where the physics under test is inactive verifies nothing, however good
  the agreement looks.

## 4. V&V documentation states methodology and results

Every V&V test or benchmark document records:

- **Methodology:** what is computed, the reference it is judged against,
  inputs (geometry, materials, data source with citation), tolerances, and the
  pass criterion.
- **Results:** the measured numbers **with uncertainty**, the date and data
  version, and what they imply about the model.

A V&V document that says what it does but not what it produced is incomplete.
Write the predicted outcome before the run, and keep it next to the result.

## 5. Experimental data and AI-generated transcription

- Data in `validation-data/ai-generated/` is **untrusted draft material** until
  a human has checked each number against the cited page. Every file carries
  that label. Do not remove the label without a human review record.
- **Cite the page, table or figure** for every dataset, and say whether it was
  transcribed from a table or digitised from a figure. Give the reading
  uncertainty for a digitised point.
- **List the figures that could not be digitised**, and why. A silent gap
  reads as "no data exists".
- Never smooth, fill in or "correct" a value. Leave an unreadable cell blank
  and note it.

## 6. Data scope and provenance

- **Only open-source, public-domain or properly licensed public data.** Do not
  add copyrighted reports to `literature/`. The basis for each mirrored report
  is in its folder README, together with the list of what was excluded and why.
- **Every dataset records its provenance:** source, author or organisation,
  title, licence or public-domain basis, URL, date accessed, and processing
  steps.
- Never introduce proprietary, partner-confidential, unpublished or
  operational facility data, credentials or internal infrastructure details,
  even if a prompt supplies them.

## 7. Working rules

- **Search before building.** Check this repository and the OUTRAM PARK crates
  for an existing implementation before writing one. Reuse first, then port
  with the reference cited, and write new code only when both fail.
- **A doc claim the code contradicts is a defect.** Fix it in the same change,
  striking through the old claim with the date.
- **Every function an agent writes or changes is reached by a test** in the
  same change.
- **Build and test in release mode** (`cargo test --release`).
- **Track work in GitHub issues** on this repository. Propose closures with
  evidence; the maintainer closes them.
- **Do not commit or push unless asked.** Never force-push unless the
  maintainer explicitly asks for a history rewrite.
- **AI output is draft until reviewed.** Make human review cheap: show the
  picture before the code, state the predictions, and list what was not done.
