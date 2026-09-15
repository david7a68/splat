---
splat_schema: 1
splat_id: 60947378-529f-5144-b2db-bed2ebe73597
splat_kind: rule
splat_key: dnd2024:rules/paladin_paladin_class_features
splat_name: Paladin Class Features
splat_origin:
  key: dnd2024:reference/pages/053
splat_sources:
- title: SRD 5.2.1
  locator: pp. 53
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/paladin_paladin_class_features
  parameters:
    class:
      key: dnd2024:classes/paladin
    grant_when_class_level_reached:
    - level: 10
      feature:
        key: dnd2024:class_features/paladin_level_10_aura_of_courage
    - level: 11
      feature:
        key: dnd2024:class_features/paladin_level_11_radiant_strikes
    - level: 14
      feature:
        key: dnd2024:class_features/paladin_level_14_restoring_touch
    - level: 18
      feature:
        key: dnd2024:class_features/paladin_level_18_aura_expansion
    - level: 19
      feature:
        key: dnd2024:class_features/paladin_level_19_epic_boon
    - level: 1
      feature:
        key: dnd2024:class_features/paladin_level_1_lay_on_hands
    - level: 1
      feature:
        key: dnd2024:class_features/paladin_level_1_spellcasting
    - level: 1
      feature:
        key: dnd2024:class_features/paladin_level_1_weapon_mastery
    - level: 2
      feature:
        key: dnd2024:class_features/paladin_level_2_fighting_style
    - level: 2
      feature:
        key: dnd2024:class_features/paladin_level_2_paladin_s_smite
    - level: 3
      feature:
        key: dnd2024:class_features/paladin_level_3_channel_divinity
    - level: 3
      feature:
        key: dnd2024:class_features/paladin_level_3_paladin_subclass
    - level: 4
      feature:
        key: dnd2024:class_features/paladin_level_4_ability_score_improvement
    - level: 5
      feature:
        key: dnd2024:class_features/paladin_level_5_extra_attack
    - level: 5
      feature:
        key: dnd2024:class_features/paladin_level_5_faithful_steed
    - level: 6
      feature:
        key: dnd2024:class_features/paladin_level_6_aura_of_protection
    - level: 9
      feature:
        key: dnd2024:class_features/paladin_level_9_abjure_foes
  implementation: dnd2024:rules_paladin_paladin_class_features
splat_contracts:
- id: dnd2024:rules_paladin_paladin_class_features
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
  - 53
  engine_gaps: &id001
  - dnd2024:rules_paladin_paladin_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 3576d1ed80f64d4a2b6bab43e4704d86083e6772d8ac31491b007dea89e655de
  parent_name: Paladin
  line_span:
    first: 4452
    count: 4
splat_clauses:
- key: source_rules
  text: '# Paladin Class Features As a Paladin, you gain the following class features when you reach the
    specified Paladin levels. These features are listed in the Paladin Features table.'
  sha256: 013fccc0b3505ebaf996d4cf4a7638ff8ef46c1a171db337bd10ca594b1786bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Paladin Class Features

<!-- SRD page 53 -->

As a Paladin, you gain the following class features\
when you reach the specified Paladin levels. These\
features are listed in the Paladin Features table.\
