---
splat_schema: 1
splat_id: 53f5b970-f413-563d-92e5-e8c7759f9df6
splat_kind: rule
splat_key: dnd2024:class_features/hunter_level_3_hunter_s_lore
splat_name: 'Level 3: Hunter’s Lore'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/hunter_level_3_hunter_s_lore
  parameters:
    while_target_marked_by: own_hunters_mark
    reveal_target_properties:
    - immunities
    - resistances
    - vulnerabilities
    reveal_absence: true
  implementation: dnd2024:class_features_hunter_level_3_hunter_s_lore
splat_contracts:
- id: dnd2024:class_features_hunter_level_3_hunter_s_lore
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - spell.effect_applied
  - spell.effect_ended
  capabilities:
  - knowledge
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  engine_gaps: &id001
  - dnd2024:class_features_hunter_level_3_hunter_s_lore
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: ff759a651f33f0191497090ebb2682104445664b36610b2a5a1fdc12195b8acb
  parent_name: Hunter
  line_span:
    first: 5083
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 3: Hunter’s Lore You can call on the forces of nature to reveal certain strengths and
    weaknesses of your prey. While a creature is marked by your Hunter’s Mark, you know whether that creature
    has any Immunities, Resistances, or Vulnerabilities, and if the creature has any, you know what they
    are.'
  sha256: cb1787ad10e2573511082c4f6e83270e40985f6f50bd694be62fc40995203140
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Hunter’s Lore

<!-- SRD page 61 -->

You can call on the forces of nature to reveal certain\
strengths and weaknesses of your prey. While a\
creature is marked by your Hunter’s Mark, you know\
whether that creature has any Immunities, Resis-\
tances, or Vulnerabilities, and if the creature has\
any, you know what they are.\
