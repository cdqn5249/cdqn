---
title: CDQN Glossary
description: Unified glossary for the CDQN project — canonical definitions with bidirectional source links.
version: 1.0.0
updated: 2026-08-30
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/glossary.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /glossary.html
---

<link rel="stylesheet" href="{{ '/assets/css/style.css' | relative_url }}">

# CDQN Glossary

**Project:** cdqn — Chained and Distributed Quang Numbers  
**Author:** Christophe Duy Quang Nguyen  
**License:** [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)  
**Status:** Canonical reference — single source of truth for all specialized terms

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Quick Navigation

| Document | Published page | Source file |
|---|---|---|
| Documentation portal | [index.html]({{ '/index.html' | relative_url }}) | [docs/index.md](https://github.com/cdqn5249/cdqn/blob/main/docs/index.md) |
| SIMEMP constraints | [simemp.html]({{ '/simemp.html' | relative_url }}) | [docs/simemp.md](https://github.com/cdqn5249/cdqn/blob/main/docs/simemp.md) |
| Abstraction layers | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) | [docs/abstractionLayers.md](https://github.com/cdqn5249/cdqn/blob/main/docs/abstractionLayers.md) |
| Qn Primitive Envelope | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) | [docs/qnPrimitive.md](https://github.com/cdqn5249/cdqn/blob/main/docs/qnPrimitive.md) |
| This glossary | [glossary.html]({{ '/glossary.html' | relative_url }}) | [docs/glossary.md](https://github.com/cdqn5249/cdqn/blob/main/docs/glossary.md) |
| Scaling Source License | Not published from docs/ | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Public repository | [Repository](https://github.com/cdqn5249/cdqn) | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |

> **License location note:**  
> `LICENSE.md` is located at the root of the `cdqn` repository, not inside `docs/`. Because GitHub Pages is published from `docs/`, the license is linked through the GitHub repository rather than served as a local GitHub Pages file.

---

## How to Use This Glossary

**Forward navigation:** Click any term heading to copy its anchor link. Use that link from any document to point readers to the canonical definition.

**Backward navigation:** Every term lists the exact document sections where it is used. Click any section link to see how the term is applied in context.

**Categories:** Terms are grouped by conceptual domain. Use the alphabetical index below to jump directly to a term.

---

## Alphabetical Index

{% assign all_terms = site.data.glossary | sort: "term" %}
{% assign letters = "A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z" | split: "," %}

<div class="glossary-index">
{% for letter in letters %}
  {% assign has_term = false %}
  {% for t in all_terms %}
    {% assign first_char = t.term | slice: 0 | upcase %}
    {% if first_char == letter %}{% assign has_term = true %}{% endif %}
  {% endfor %}
  {% if has_term %}
    <a href="#index-{{ letter }}" class="glossary-index-letter">{{ letter }}</a>
  {% else %}
    <span class="glossary-index-letter inactive">{{ letter }}</span>
  {% endif %}
{% endfor %}
</div>

---

## Terms by Category

{% assign categories = site.data.glossary | map: "category" | uniq | sort %}

{% for category in categories %}

### {{ category }}

{% assign terms_in_category = site.data.glossary | where: "category", category | sort: "term" %}

{% for term in terms_in_category %}

<div class="glossary-entry" id="{{ term.slug }}">

<h2 class="glossary-term">{{ term.term }} <a href="#{{ term.slug }}" class="term-anchor" aria-label="Anchor link for {{ term.term }}">#</a></h2>

<div class="glossary-definition">
{{ term.definition }}
</div>

<div class="glossary-backlinks">
  <p class="backlinks-heading">Mentioned in:</p>
  {% assign docs = term.sources | map: "doc" | uniq | sort %}
  <ul class="backlinks-list">
  {% for doc in docs %}
    <li class="backlinks-doc">
      <strong>{{ doc }}</strong>
      <ul class="backlinks-sections">
      {% for source in term.sources %}
        {% if source.doc == doc %}
        <li>
          <a href="{% if source.doc == 'LICENSE.md' %}https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md{{ source.anchor }}{% else %}{{ '/' | append: source.doc | replace: '.md', '.html' | relative_url }}{{ source.anchor }}{% endif %}">
            {{ source.section }}
          </a>
        </li>
        {% endif %}
      {% endfor %}
      </ul>
    </li>
  {% endfor %}
  </ul>
</div>

</div>

{% endfor %}

{% endfor %}

---

## Document Metadata

| Field | Value |
|---|---|
| Document Title | CDQN Glossary |
| Version | 1.0.0 |
| Last Updated | 2026-08-30 |
| Author | Christophe Duy Quang Nguyen |
| License | Scaling Source License (SSL) 1.0 |
| Repository Path | docs/glossary.md |
| Parent Repository | https://github.com/cdqn5249/cdqn |
| Status | Canonical reference |
| Depends On | docs/_data/glossary.yml |

---

## Bottom Navigation

| Destination | Link |
|---|---|
| Documentation portal | [index.html]({{ '/index.html' | relative_url }}) |
| SIMEMP constraints | [simemp.html]({{ '/simemp.html' | relative_url }}) |
| Abstraction layers | [abstractionLayers.html]({{ '/abstractionLayers.html' | relative_url }}) |
| Qn Primitive Envelope | [qnPrimitive.html]({{ '/qnPrimitive.html' | relative_url }}) |
| This glossary | [glossary.html]({{ '/glossary.html' | relative_url }}) |
| License source | [LICENSE.md](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md) |
| Public repository | [github.com/cdqn5249/cdqn](https://github.com/cdqn5249/cdqn) |
