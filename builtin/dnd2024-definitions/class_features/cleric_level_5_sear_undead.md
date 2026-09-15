---
splat_schema: 1
splat_id: 9bb79a15-f2ae-5e02-a7b6-b0c01116f75e
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_5_sear_undead
splat_name: 'Level 5: Sear Undead'
splat_origin:
  key: dnd2024:reference/pages/037
splat_sources:
- title: SRD 5.2.1
  locator: pp. 37
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_5_sear_undead
  parameters:
    trigger: turn_undead.used
    optional: true
    damage_roll:
      shared_for_all_targets: true
      roll:
        dice:
          count:
            op: max
            args:
            - 1
            - stat: abilities.wisdom.modifier
          sides: 8
      type: radiant
    targets:
      creature_type: undead
      failed_triggering_save: true
    damage_ends_turn_undead: false
  implementation: dnd2024:class_features_cleric_level_5_sear_undead
splat_contracts:
- id: dnd2024:class_features_cleric_level_5_sear_undead
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - damage
  events:
  - turn_undead.resolved
  capabilities:
  - damage
  - effects
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 37
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_5_sear_undead
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d80a975002eb23a67bfc77b499b12834d9f2d6b1b3ac070344254d3f17bbd5bf
  parent_name: Cleric
  line_span:
    first: 3102
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 5: Sear Undead Whenever you use Turn Undead, you can roll a number of d8s equal to your
    Wisdom modifier (minimum of 1d8) and add the rolls together. Each Undead that fails its saving throw
    against that use of Turn Undead takes Radiant damage equal to the roll’s total. This damage doesn’t
    end the turn effect.'
  sha256: 1a3e7acf3763a397d6f7eae392d5f535019dcfc2bf9cc6f9a48a71b1539c390d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Sear Undead

<!-- SRD page 37 -->

Whenever you use Turn Undead, you can roll a\
number of d8s equal to your Wisdom modifier\
(minimum of 1d8) and add the rolls together. Each\
Undead that fails its saving throw against that use\
of Turn Undead takes Radiant damage equal to the\
roll’s total. This damage doesn’t end the turn effect.\
