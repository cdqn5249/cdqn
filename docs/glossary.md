---
layout: default
title: CDQN Glossary
description: Unified ontological concordance and dynamic bidirectional registry for the CDQN project.
version: 1.1.1
updated: 2026-10-08
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

{%- assign all_terms = "" | split: "" -%}
{%- if site.data.glossary.first.slug -%}
  {%- assign all_terms = site.data.glossary -%}
{%- else -%}
  {%- for category in site.data.glossary -%}
    {%- for item in category[1] -%}
      {%- assign all_terms = all_terms | push: item -%}
    {%- endfor -%}
  {%- endfor -%}
{%- endif -%}
{%- assign all_terms = all_terms | sort: "term" -%}
{%- assign categories = all_terms | map: "category" | uniq | sort -%}
{%- assign letters = "A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,R,S,T,U,V,W,X,Y,Z" | split: "," -%}

<div class="category-index" style="margin-bottom: 1rem;">
  <strong>Domains:</strong>
  {% for category in categories %}
    <a href="#{{ category | slugify }}" class="badge">{{ category }}</a>
  {% endfor %}
</div>

<div class="glossary-index">
{% for letter in letters %}
  {% assign target_term = nil %}
  {% for t in all_terms %}
    {% assign first_char = t.term | slice: 0 | upcase %}
    {% if first_char == letter and target_term == nil %}
      {% assign target_term = t %}
    {% endif %}
  {% endfor %}
  {% if target_term %}
    <a href="#{{ target_term.slug }}" class="glossary-index-letter" title="Jump to {{ target_term.term }}">{{ letter }}</a>
  {% else %}
    <span class="glossary-index-letter inactive">{{ letter }}</span>
  {% endif %}
{% endfor %}
</div>

---

## Canonical Terms by Domain

{% for category in categories %}

<h2 class="glossary-category" id="{{ category | slugify }}">{{ category }}</h2>

{% assign terms_in_category = all_terms | where: "category", category | sort: "term" %}

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
  {%- assign mentioning_pages = "" | split: "" -%}
  {%- for p in site.pages -%}
    {%- if p.terms_used and p.terms_used contains term.slug -%}
      {%- assign mentioning_pages = mentioning_pages | push: p -%}
    {%- endif -%}
  {%- endfor -%}
  {%- assign mentioning_pages = mentioning_pages | sort: "title" -%}

  {%- if mentioning_pages.size > 0 -%}
    <ul class="backlinks-list">
      {%- for p in mentioning_pages -%}
        <li class="backlinks-doc">
          <a href="{{ p.url | relative_url }}#ref-{{ term.slug }}"><strong>{{ p.title | default: p.name }}</strong></a>
        </li>
      {%- endfor -%}
    </ul>
  {%- elsif term.sources -%}
    {%- comment -%} Legacy fallback during transition {%- endcomment -%}
    {%- assign legacy_docs = term.sources | map: "doc" | uniq | sort -%}
    <ul class="backlinks-list">
      {%- for doc in legacy_docs -%}
        <li class="backlinks-doc">
          <strong>{{ doc }}</strong>
        </li>
      {%- endfor -%}
    </ul>
  {%- else -%}
    <p class="backlinks-none"><small><em>Referenced system-wide</em></small></p>
  {%- endif -%}
</div>

</div>

{% endfor %}

{% endfor %}
