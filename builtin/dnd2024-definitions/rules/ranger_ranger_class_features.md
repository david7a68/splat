---
splat_schema: 1
splat_id: 6bb212a0-e1a9-5a31-a937-8be18be81053
splat_kind: rule
splat_key: dnd2024:rules/ranger_ranger_class_features
splat_name: Ranger Class Features
splat_origin:
  key: dnd2024:reference/pages/057
splat_sources:
- title: SRD 5.2.1
  locator: pp. 57
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/ranger_ranger_class_features
  parameters:
    class_definition:
      key: dnd2024:classes/ranger
    grant_features:
      level_basis: class_level
      unlock_when: class_level_gte_feature_level
      from: class_progression
    feature_unlocks:
    - class_level: 10
      feature:
        key: dnd2024:class_features/ranger_level_10_tireless
    - class_level: 13
      feature:
        key: dnd2024:class_features/ranger_level_13_relentless_hunter
    - class_level: 14
      feature:
        key: dnd2024:class_features/ranger_level_14_nature_s_veil
    - class_level: 17
      feature:
        key: dnd2024:class_features/ranger_level_17_precise_hunter
    - class_level: 18
      feature:
        key: dnd2024:class_features/ranger_level_18_feral_senses
    - class_level: 19
      feature:
        key: dnd2024:class_features/ranger_level_19_epic_boon
    - class_level: 1
      feature:
        key: dnd2024:class_features/ranger_level_1_favored_enemy
    - class_level: 1
      feature:
        key: dnd2024:class_features/ranger_level_1_spellcasting
    - class_level: 1
      feature:
        key: dnd2024:class_features/ranger_level_1_weapon_mastery
    - class_level: 20
      feature:
        key: dnd2024:class_features/ranger_level_20_foe_slayer
    - class_level: 2
      feature:
        key: dnd2024:class_features/ranger_level_2_deft_explorer
    - class_level: 2
      feature:
        key: dnd2024:class_features/ranger_level_2_fighting_style
    - class_level: 3
      feature:
        key: dnd2024:class_features/ranger_level_3_ranger_subclass
    - class_level: 4
      feature:
        key: dnd2024:class_features/ranger_level_4_ability_score_improvement
    - class_level: 5
      feature:
        key: dnd2024:class_features/ranger_level_5_extra_attack
    - class_level: 6
      feature:
        key: dnd2024:class_features/ranger_level_6_roving
    - class_level: 9
      feature:
        key: dnd2024:class_features/ranger_level_9_expertise
  implementation: dnd2024:rules_ranger_ranger_class_features
splat_contracts:
- id: dnd2024:rules_ranger_ranger_class_features
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
  - 57
  engine_gaps: &id001
  - dnd2024:rules_ranger_ranger_class_features
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 93dc1ad1e1ec77f2a32710077ac73e3388387c1cad19b2ed3f2b614a78d24a3d
  parent_name: Ranger
  line_span:
    first: 4825
    count: 4
splat_clauses:
- key: source_rules
  text: '# Ranger Class Features As a Ranger, you gain the following class features when you reach the
    specified Ranger levels. These features are listed in the Ranger Features table.'
  sha256: fcd5252a89ce5ec006dd9f7d34acdeaf9dca75fd2f501e16be6d6aa23eed8a4e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ranger Class Features

<!-- SRD page 57 -->

As a Ranger, you gain the following class features\
when you reach the specified Ranger levels. These\
features are listed in the Ranger Features table.\
