---
splat_schema: 1
splat_id: 49f5b590-7cb3-502f-87a2-1ea109302e29
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
splat_name: 'Level 14: Improved Blessed Strikes'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_14_improved_blessed_strikes
  parameters:
    modify_selected_blessed_strikes:
      divine_strike:
        damage_dice:
          count: 2
          sides: 8
      potent_spellcasting:
        trigger: cleric.cantrip_cast_deals_damage_to_creature
        optional: true
        target:
          type: creature
          self_allowed: true
          range_feet: 60
          count: 1
        temporary_hit_points:
          op: multiply
          args:
          - 2
          - var: actor.abilities.wisdom.modifier
    class_scope:
      class: cleric
      minimum_level: 14
  implementation: dnd2024:class_features_cleric_level_14_improved_blessed_strikes
splat_contracts:
- id: dnd2024:class_features_cleric_level_14_improved_blessed_strikes
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - attack.hit
  - spell.damage
  capabilities:
  - character_features
  - spellcasting
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 38
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_14_improved_blessed_strikes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 43f4d462737f30acfab51950aa43db85a7a44d47564545c9c6f0fa1d1e81be2f
  parent_name: Cleric
  line_span:
    first: 3128
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 14: Improved Blessed Strikes The option you chose for Blessed Strikes grows more powerful.
    Divine Strike. The extra damage of your Divine Strike increases to 2d8. Potent Spellcasting. When
    you cast a Cleric cantrip and deal damage to a creature with it, you can give vitality to yourself
    or another creature within 60 feet of yourself, granting a number of Temporary Hit Points equal to
    twice your Wisdom modifier.'
  sha256: d7ee203427ac5cd86a7b16bbe0f2694d7fdec97ced0e8f1b9502852f638d1910
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Improved Blessed Strikes

<!-- SRD page 38 -->

The option you chose for Blessed Strikes grows\
more powerful.\
Divine Strike. The extra damage of your Divine\
Strike increases to 2d8.\
Potent Spellcasting. When you cast a Cleric can-\
trip and deal damage to a creature with it, you can\
give vitality to yourself or another creature within\
60 feet of yourself, granting a number of Temporary\
Hit Points equal to twice your Wisdom modifier.\
