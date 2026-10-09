# molten-salt-reactor-lit-and-sims

Repo for storage of public domain molten salt literature and open source
molten salt reactor models, both solid and liquid fuelled, built on the
[OUTRAM PARK](https://github.com/theodoreOnzGit/outram-park-backend) crates.

> **Research, education and verification & validation only.** Nothing here is
> for facility operation, reactor control, licensing, safety-critical or
> emergency decisions.

| Folder | What it holds |
|---|---|
| [`literature/`](literature/moltensalt-org/README.md) | 313 public-domain AEC-era reports (1949–1977), with the copyright basis and the list of what was excluded |
| [`validation-data/ai-generated/`](validation-data/ai-generated/README.md) | Measured data transcribed and digitised **by AI agents, not yet human-reviewed**, including a list of the figures that could not be digitised |
| [`benchmarks/`](benchmarks/) | Benchmark specifications: [neutronics](benchmarks/neutronics.md), [thermal hydraulics](benchmarks/thermal-hydraulics.md), [materials](benchmarks/materials.md), [transients](benchmarks/transients.md) |
| [`crates/`](crates/msr-sims/) | Rust workspace that depends on every OUTRAM PARK crate on crates.io |

Agent and contributor principles: [`AGENTS.md`](AGENTS.md).

## Building

```bash
cargo test --release
```

`tuas_boussinesq_solver` is pinned to `=0.1.6` until a `tampines` release
builds against 0.1.7
([outram-park-backend#813](https://github.com/theodoreOnzGit/outram-park-backend/issues/813)).

## Licence

The code and documents in this repository are AGPL-3.0. The mirrored reports
in `literature/` are U.S. government-sponsored works treated as public domain;
the basis is in their README.
