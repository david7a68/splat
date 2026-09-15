---
splat_schema: 1
splat_id: 1a191a34-0ce9-5f59-8e13-07ecfba84ef5
splat_kind: rule
splat_key: dnd2024:rules/monk_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/049
splat_sources:
- title: SRD 5.2.1
  locator: pp. 49
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/monk_as_a_level_1_character
  parameters:
    requires_total_character_level: 1
    core_traits:
      key: dnd2024:classes/monk
    grant_all_core_traits: true
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/monk_level_1_martial_arts
    - level: 1
      feature:
        key: dnd2024:class_features/monk_level_1_unarmored_defense
  implementation: dnd2024:rules_monk_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_monk_as_a_level_1_character
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - core_traits
  - class_features
  events: []
  capabilities:
  - character_creation
  - character_features
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 49
  engine_gaps: &id001
  - dnd2024:rules_monk_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: cec783bafcac40ba645a1984f01917c8bd97db7ea98f5ee63d415981f410fa9c
  parent_name: Monk
  line_span:
    first: 4111
    count: 4
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Monk Traits table. • Gain the Monk’s
    level 1 features, which are listed in the Monk Features table.'
  sha256: b0ece5ce31c35118aa0e64e026423ab49fd1667287995c28fad66861b38815a3
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 49 -->

•  Gain all the traits in the Core Monk Traits table.\
•  Gain the Monk’s level 1 features, which are listed\
in the Monk Features table.\
