---
splat_schema: 1
splat_id: 488c2274-8013-558e-ba89-c7f90a86abda
splat_kind: rule
splat_key: dnd2024:class_features/barbarian_level_18_indomitable_might
splat_name: 'Level 18: Indomitable Might'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/barbarian_level_18_indomitable_might
  parameters:
    rolls:
    - ability_check
    - saving_throw
    ability: strength
    optional_total_floor:
      stat: abilities.strength.score
  implementation: dnd2024:class_features_barbarian_level_18_indomitable_might
splat_contracts:
- id: dnd2024:class_features_barbarian_level_18_indomitable_might
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - ability_check.rolled
  - saving_throw.rolled
  capabilities:
  - character_features
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_barbarian_level_18_indomitable_might
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 2117750f05bbbb874f1dfd2f16f7406a6ec481fc9b579e648b5922220b6d6619
  parent_name: Barbarian
  line_span:
    first: 2440
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 18: Indomitable Might If your total for a Strength check or Strength saving throw is
    less than your Strength score, you can use that score in place of the total.'
  sha256: 5857c6200f830f455384ce2a2ee78041029c7144b25d274e08343784da2f96be
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Indomitable Might

<!-- SRD page 30 -->

If your total for a Strength check or Strength saving\
throw is less than your Strength score, you can use\
that score in place of the total.\
