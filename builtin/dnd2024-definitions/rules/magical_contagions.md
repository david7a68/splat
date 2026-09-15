---
splat_schema: 1
splat_id: 5a44188d-30f9-5c58-b7f0-069f048599ce
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions
splat_name: Magical Contagions
splat_origin:
  key: dnd2024:reference/pages/193
splat_sources:
- title: SRD 5.2.1
  locator: pp. 193
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magical_contagions
  parameters:
    curse_duration: specified_time_or_until_ended
    magical_contagion:
      magical_origin: true
      adverse_effect: true
      contagious: true
  implementation: dnd2024:rule_magical_contagions
splat_contracts:
- id: dnd2024:rule_magical_contagions
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - effects
  events: []
  capabilities:
  - curses
  - contagions
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 193
  engine_gaps: &id001
  - dnd2024:rule_magical_contagions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 2884a6dc9259a61c25fba35984188f11735180b07297f321d46b3f728130761c
  parent_name: Magical Contagions
  line_span:
    first: 9433
    count: 7
splat_clauses:
- key: source_rules
  text: '# Magical Contagions A curse is a magical burden that lasts for a specified time or until it
    is ended by some means. A magical contagion is an adverse effect of magical origin that is contagious
    by definition. The following sections discuss curses and magical contagions in detail.'
  sha256: db9c869ea025a538af84e211cd0351997ff5bd31ac154cdbc2f006511a329522
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magical Contagions

<!-- SRD page 193 -->

A curse is a magical burden that lasts for a specified\
time or until it is ended by some means. A magical\
contagion is an adverse effect of magical origin that\
is contagious by definition.\
The following sections discuss curses and magical\
contagions in detail.\
