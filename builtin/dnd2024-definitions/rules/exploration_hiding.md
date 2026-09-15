---
splat_schema: 1
splat_id: 2a6db38c-9dd5-595a-9cd7-3288f0a5f7d5
splat_kind: rule
splat_key: dnd2024:rules/exploration_hiding
splat_name: Hiding
splat_origin:
  key: dnd2024:reference/pages/011
splat_sources:
- title: SRD 5.2.1
  locator: pp. 11
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_hiding
  parameters:
    apply_definitions:
    - key: dnd2024:glossary/hide_action
    circumstances_appropriate:
      actor: gm
      type: boolean
  implementation: dnd2024:rule_exploration_hiding
splat_contracts:
- id: dnd2024:rule_exploration_hiding
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - resolution
  events: []
  capabilities:
  - stealth
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 11
  engine_gaps: &id001
  - dnd2024:rule_exploration_hiding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ee1b4fab58419f42edb4a9488b77c97b7ee1af11bad0e768cad3836398016a11
  parent_name: Exploration
  line_span:
    first: 654
    count: 6
splat_clauses:
- key: source_rules
  text: '# Hiding Adventurers and monsters often hide, whether to spy on one another, sneak past a guardian,
    or set an ambush. The Game Master decides when circumstances are appropriate for hiding. When you
    try to hide, you take the Hide action.'
  sha256: 64c27dbb06f5aa191c04095b83307609132d402e6575eb4ded7b77cff24908e5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hiding

<!-- SRD page 11 -->

Adventurers and monsters often hide, whether to\
spy on one another, sneak past a guardian, or set an\
ambush. The Game Master decides when circum-\
stances are appropriate for hiding. When you try to\
hide, you take the Hide action.\
