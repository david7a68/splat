---
splat_schema: 1
splat_id: 6621b501-072a-50cb-bc24-e3d31afd6ee9
splat_kind: rule
splat_key: dnd2024:rules/druid_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/041
splat_sources:
- title: SRD 5.2.1
  locator: pp. 41
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/druid_as_a_level_1_character
  parameters:
    class_definition:
      key: dnd2024:classes/druid
    grant_core_traits: all
    grant_features:
      class_level: 1
      from: class_definition
  implementation: dnd2024:rules_druid_as_a_level_1_character
splat_contracts:
- id: dnd2024:rules_druid_as_a_level_1_character
  inputs:
    actor:
      type: participant
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - character_updates
  events: []
  capabilities:
  - character_creation
  - progression
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 41
  engine_gaps: &id001
  - dnd2024:rules_druid_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: b3a75833e3ea469e4039cae34b49d39b6d316c90d73eb167441572adb3295c5f
  parent_name: Druid
  line_span:
    first: 3384
    count: 4
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Druid Traits table. • Gain the Druid’s
    level 1 features, which are listed in the Druid Features table.'
  sha256: 5b9ff350389000f80c8d792ea35e2c0a8bb55fe62c76ebbfeb1fd0f0ae4124f9
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 41 -->

•  Gain all the traits in the Core Druid Traits table.\
•  Gain the Druid’s level 1 features, which are listed\
in the Druid Features table.\
