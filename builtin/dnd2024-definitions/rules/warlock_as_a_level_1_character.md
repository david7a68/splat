---
splat_schema: 1
splat_id: e661b70d-3f6f-5fcb-ba3e-3506cf801465
splat_kind: rule
splat_key: dnd2024:rules/warlock_as_a_level_1_character
splat_name: As a Level 1 Character
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_as_a_level_1_character
  parameters:
    class:
      key: dnd2024:classes/warlock
    entry_mode: level_one_character
    grant_all_core_traits: true
    grant_features_at_class_level: 1
    embedded_feature_progression_source:
      key: dnd2024:tables/warlock_progression
    level_one_features:
    - key: dnd2024:class_features/warlock_level_1_eldritch_invocations
    - key: dnd2024:class_features/warlock_level_1_pact_magic
  implementation: dnd2024:rule_warlock_as_a_level_1_character
splat_contracts:
- id: dnd2024:rule_warlock_as_a_level_1_character
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - classes
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:rule_warlock_as_a_level_1_character
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 4b02c9325bce1817ede73405d54a9a5484222b7bb50494db8030acceb0690538
  parent_name: Warlock
  line_span:
    first: 5950
    count: 4
splat_clauses:
- key: source_rules
  text: '# As a Level 1 Character • Gain all the traits in the Core Warlock Traits table. • Gain the Warlock’s
    level 1 features, which are listed in the Warlock Features table.'
  sha256: 0ab827656165d64e5d2666d8ba26400d0dbcfc6da743e99d38fd74e5c8a04b25
  status: contract_specified
splat_requires_mechanics: *id001
---

# As a Level 1 Character

<!-- SRD page 70 -->

•  Gain all the traits in the Core Warlock Traits table.\
•  Gain the Warlock’s level 1 features, which are\
listed in the Warlock Features table.\
