---
splat_schema: 1
splat_id: dd57d273-fc5e-53a3-8a5e-f5a6158cb96c
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_13_relentless_hunter
splat_name: 'Level 13: Relentless Hunter'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_13_relentless_hunter
  parameters:
    spell:
      key: dnd2024:spells/hunter_s_mark
    concentration_end_prevention:
      cause: damage_taken
    class_scope:
      class: ranger
      minimum_level: 13
  implementation: dnd2024:class_features_ranger_level_13_relentless_hunter
splat_contracts:
- id: dnd2024:class_features_ranger_level_13_relentless_hunter
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
  - concentration.damage_check
  capabilities:
  - character_features
  - spellcasting
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_13_relentless_hunter
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0c04184635672c3a950db344b8f8a6eef3ea7f79840e5d26e119f10acebb976f
  parent_name: Ranger
  line_span:
    first: 4988
    count: 3
splat_clauses:
- key: source_rules
  text: '# Level 13: Relentless Hunter Taking damage can’t break your Concentration on Hunter’s Mark.'
  sha256: c7150cfd636b6103d3266c45b93fbebea7c0dd4497f898bd40d6f2012d1c1695
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 13: Relentless Hunter

<!-- SRD page 59 -->

Taking damage can’t break your Concentration on\
Hunter’s Mark.\
