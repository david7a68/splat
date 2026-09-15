---
splat_schema: 1
splat_id: 1ec276d2-f908-58a1-9831-340fff07ff4d
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_7_feral_instinct
splat_name: 'Level 7: Feral Instinct'
splat_origin:
  key: dnd2024:reference/pages/029
splat_sources:
- title: SRD 5.2.1
  locator: pp. 29
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_7_feral_instinct
  parameters:
    advantage:
    - initiative
    class_scope:
      class: barbarian
      minimum_level: 7
  implementation: dnd2024:class_features_barbarian_level_7_feral_instinct
splat_contracts:
- id: dnd2024:class_features_barbarian_level_7_feral_instinct
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - initiative.roll
  capabilities:
  - character_features
  - d20_tests
  - initiative
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 29
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_7_feral_instinct
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 45c3711e89fed437e42849f8f5744d3cb961c29464350f88934d180b8eb258d0
  parent_name: Barbarian
  line_span:
    first: 2380
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 7: Feral Instinct Your instincts are so honed that you have Advantage on Initiative rolls.'
  sha256: 28ba52993ca636cac93840caf6c93abc08c019034e259697fdb14688557fc478
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Feral Instinct

<!-- SRD page 29 -->

Your instincts are so honed that you have Advan-\
tage on Initiative rolls.\
