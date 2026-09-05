---
layout: default
title: CDQN Glossary
description: Unified ontological concordance and bidirectional registry for the CDQN project.
version: 1.0.0
updated: 2026-09-05
author: Christophe Duy Quang Nguyen
license: Scaling Source License (SSL) 1.0
license_file: LICENSE.md
license_location: repository root
file_repo_path: docs/glossary.md
parent_repository: https://github.com/cdqn5249/cdqn
permalink: /glossary.html
---

# CDQN Glossary & Ontological Concordance

**Project:** [cdqn]({{ '/glossary.html' | relative_url }}#cdqn) — Chained and Distributed Quang Numbers  
**Author:** Christophe Duy Quang Nguyen  
**License:** [Scaling Source License (SSL) 1.0](https://github.com/cdqn5249/cdqn/blob/main/LICENSE.md)  
**Status:** Canonical Ontological Registry

Copyright (c) 2026 Christophe Duy Quang Nguyen. All rights reserved.

---

## Navigation & Index

Hover over terms across any documentation page for instant in-situ definition tooltips. Click any term to jump directly to its formal specification below.

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
    <a href="#letter-{{ letter }}" class="glossary-index-letter">{{ letter }}</a>
  {% else %}
    <span class="glossary-index-letter inactive">{{ letter }}</span>
  {% endif %}
{% endfor %}
</div>

---

## Canonical Terms by Domain

{% assign categories = site.data.glossary | map: "category" | uniq | sort %}

{% for category in categories %}

## {{ category }}

{% assign terms_in_category = site.data.glossary | where: "category", category | sort: "term" %}

{% for term in terms_in_category %}

<div class="glossary-entry" id="{{ term.slug }}">

<h3 class="glossary-term" id="{{ term.slug }}-title">
  {{ term.term }} 
  <a href="#{{ term.slug }}" class="term-anchor" aria-label="Direct link to {{ term.term }}">#</a>
</h3>

<div class="glossary-definition">
  <p>{{ term.definition }}</p>
</div>

<div class="glossary-backlinks">
  <p class="backlinks-heading"><strong>Mentioned in:</strong></p>
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
