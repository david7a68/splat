---
splat_schema: 1
splat_id: ab1a5798-8113-5d7d-9f4c-b409d5b76886
splat_kind: rule
splat_key: dnd2024:rules/cleric_cleric_class_features
splat_name: Cleric Class Features
splat_origin:
  key: dnd2024:reference/pages/036
splat_sources:
- title: SRD 5.2.1
  locator: pp. 36
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/cleric_cleric_class_features
  parameters:
    class_definition:
      key: dnd2024:classes/cleric
    grant_features:
      level_basis: class_level
      unlock_when: class_level_gte_feature_level
      from: class_progression
    feature_unlocks:
    - class_level: 10
      feature:
        key: dnd2024:class_features/cleric_level_10_divine_intervention
    - class_level: 14
      feature:
        key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
    - class_level: 19
      feature:
        key: dnd2024:class_features/cleric_level_19_epic_boon
    - class_level: 1
      feature:
        key: dnd2024:class_features/cleric_level_1_divine_order
    - class_level: 1
      feature:
        key: dnd2024:class_features/cleric_level_1_spellcasting
    - class_level: 20
      feature:
        key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
    - class_level: 2
      feature:
        key: dnd2024:class_features/cleric_level_2_channel_divinity
    - class_level: 3
      feature:
        key: dnd2024:class_features/cleric_level_3_cleric_subclass
    - class_level: 4
      feature:
        key: dnd2024:class_features/cleric_level_4_ability_score_improvement
    - class_level: 5
      feature:
        key: dnd2024:class_features/cleric_level_5_sear_undead
    - class_level: 7
      feature:
        key: dnd2024:class_features/cleric_level_7_blessed_strikes
  implementation: dnd2024:rules_cleric_cleric_class_features
splat_contracts:
- id: dnd2024:rules_cleric_cleric_class_features
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
  - 36
  engine_gaps: &id001
  - dnd2024:rules_cleric_cleric_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: f63db7b9f44c93b4ad00809019784852e4e32987b48af0615528a0b22e9db38e
  parent_name: Cleric
  line_span:
    first: 2965
    count: 4
splat_clauses:
- key: source_rules
  text: '# Cleric Class Features As a Cleric, you gain the following class features when you reach the
    specified Cleric levels. These features are listed in the Cleric Features table.'
  sha256: c0140201f96c004b350f8f68cd5e2191b159a0b8f081fc3095beb15ff53b9f72
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cleric Class Features

<!-- SRD page 36 -->

As a Cleric, you gain the following class features\
when you reach the specified Cleric levels. These\
features are listed in the Cleric Features table.\
