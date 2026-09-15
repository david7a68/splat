---
splat_schema: 1
splat_id: ac56109a-cc47-5b67-a56c-99fc5d3a86fc
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_as_a_level_1_character
  parameters:
    requires_total_character_level: 1
    core_traits:
      key: dnd2024:classes/sorcerer
    grant_all_core_traits: true
    features:
    - level: 1
      feature:
        key: dnd2024:class_features/sorcerer_level_1_innate_sorcery
    - level: 1
      feature:
        key: dnd2024:class_features/sorcerer_level_1_spellcasting
  implementation: dnd2024:rules_sorcerer_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_sorcerer_as_a_level_1_character
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
  - 64
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 4898fb75ec7f1c597dacf3acc716a41e8e4321a27bfd7b73804cb30cf68e3151
  parent_name: Sorcerer
  line_span:
    first: 5418
    count: 5
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Sorcerer Traits table. • Gain the
    Sorcerer’s level 1 features, which are listed in the Sorcerer Features table.'
  sha256: 41ffb951b97784298307f3a56b7f31356aa9b6fd578f442e6f2041f4487a059b
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 64 -->

•  Gain all the traits in the Core Sorcerer Traits\
table.\
•  Gain the Sorcerer’s level 1 features, which are\
listed in the Sorcerer Features table.\
