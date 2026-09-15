---
splat_schema: 1
splat_id: 95d524ef-f1e5-5022-b1f5-8060e6aaf1d8
splat_kind: rule
splat_key: dnd2024:rules/warlock_warlock_class_features
splat_name: Warlock Class Features
splat_origin:
  key: dnd2024:reference/pages/070
splat_sources:
- title: SRD 5.2.1
  locator: pp. 70
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/warlock_warlock_class_features
  parameters:
    class:
      key: dnd2024:classes/warlock
    grant_when_class_level_reached:
    - level: 11
      feature:
        key: dnd2024:class_features/warlock_level_11_mystic_arcanum
    - level: 19
      feature:
        key: dnd2024:class_features/warlock_level_19_epic_boon
    - level: 1
      feature:
        key: dnd2024:class_features/warlock_level_1_eldritch_invocations
    - level: 1
      feature:
        key: dnd2024:class_features/warlock_level_1_pact_magic
    - level: 20
      feature:
        key: dnd2024:class_features/warlock_level_20_eldritch_master
    - level: 2
      feature:
        key: dnd2024:class_features/warlock_level_2_magical_cunning
    - level: 3
      feature:
        key: dnd2024:class_features/warlock_level_3_warlock_subclass
    - level: 4
      feature:
        key: dnd2024:class_features/warlock_level_4_ability_score_improvement
    - level: 9
      feature:
        key: dnd2024:class_features/warlock_level_9_contact_patron
  implementation: dnd2024:rules_warlock_warlock_class_features
splat_contracts:
- id: dnd2024:rules_warlock_warlock_class_features
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - class_features
  events: []
  capabilities:
  - character_features
  - level_advancement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 70
  engine_gaps: &id001
  - dnd2024:rules_warlock_warlock_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 68e62c146f0284201d8f84bdb46e392f10c142777e57d0e0dc2aff19eec6a732
  parent_name: Warlock
  line_span:
    first: 5962
    count: 4
splat_clauses:
- key: source_rules
  text: '# Warlock Class Features As a Warlock, you gain the following class features when you reach the
    specified Warlock levels. These features are listed in the Warlock Features table.'
  sha256: a0a3a2e4bf75d77c34b58a1aa51a2f2c2acbf629440d6b6f2a0af080680195ff
  status: contract_specified
splat_requires_mechanics: *id001
---

# Warlock Class Features

<!-- SRD page 70 -->

As a Warlock, you gain the following class features\
when you reach the specified Warlock levels. These\
features are listed in the Warlock Features table.\
