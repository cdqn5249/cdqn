---
title: CDQN Documentation Portal
description: Root documentation portal for the CDQN project, including the SIMEMP constraints and future Qn/cdqn specifications.
version: 0.1.0
updated: 2026-06-16
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
location: docs/index.md
file_repo_path: docs/index.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /
---

<link rel="stylesheet" href="{{ '/assets/css/style.css' | relative_url }}">

# CDQN Documentation Portal

**Project:** cdqn — Chained and Distributed Quang Numbers  
**Author:** Christophe Duy Quang Nguyen  
**License:** Scaling Source License (SSL) 1.0  
**Repository:** [https://github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn)  
**Status:** Thesis / constraint guide — not yet a full engineering specification

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Purpose

This portal is the root documentation index for the CDQN project.

The documents in this repository are intentionally cautious. They are conjectures, constraint systems, and design guides. They are not finalized specifications, implementation manuals, or formal proofs.

Future work will define axioms, derivations, and verification through successful QnLang computation. If the logic is internally consistent and computes successfully within its declared finite boundaries, then the abstraction is considered valid within the Qn universe.

---

## Core documents

| Document | Purpose | Status | HTML | Source |
|---|---|---|---:|---|
| SIMEMP Constraints | Thesis and constraints for the design of the Qn and cdqn stack. | Draft | [simemp.html]({{ '/simemp.html' | relative_url }}) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| Scaling Source License | License governing the Licensed Work, Derivative Works, Paternity References, Open Core Invariants, and Scaling Thresholds. | Stable | External | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Repository | Public source repository for cdqn. | Active | External | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |

---

## Recommended reading order

1. **SIMEMP Constraints**  
   Start with the SIMEMP document. It defines the environmental constraints that will challenge and govern all later Qn/cdqn design work.

2. **LICENSE.md**  
   Read the Scaling Source License to understand attribution, derivative obligations, Open Core Invariants, anti-patent retaliation, and Scaling Usage thresholds.

3. **Future specifications**  
   Later documents may define QnLang, QnIR, cdqn chaining, distributed verification, axioms, derivations, and operational verification.

---

## Project stance

The CDQN project does not attempt to bypass physical computation limits, undecidability, memory walls, semantic gaps, or security assumptions.

Instead, the project designs around them by requiring:

- finite boundaries;
- measurable resource envelopes;
- explicit identity and lineage;
- bounded security assumptions;
- halt-capable computation;
- causal ordering;
- portability constraints;
- modularity constraints;
- efficiency trade-offs governed by the SIMEMP criteria.

---

## License and Open Core Invariants

This documentation portal is provided under the Scaling Source License (SSL) 1.0.

Derivative Works must preserve the required copyright notice and Paternity Reference where applicable:

> Derived from the original work by Christophe Duy Quang Nguyen under the Scaling Source License (SSL). Parent Repository: https://github.com/cdqn5249/cdqn

The technical documentation must not weaken the SSL Open Core Invariants:

1. **Anti-Patent Defense** — automatic license termination upon patent assertion against the Author or project community.
2. **Non-Scaling Open Access** — royalty-free access for non-scaling academic, open-source, personal, research, and small-scale development.

---

## GitHub Pages note

This folder is intended to be used as the GitHub Pages root.

Recommended repository structure:

```text
docs/
  index.md
  simemp.md
  assets/
    css/
      style.css
