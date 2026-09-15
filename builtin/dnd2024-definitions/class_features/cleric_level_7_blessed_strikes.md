---
splat_schema: 1
splat_id: 7ff00766-dac4-5411-9bd6-882b50c54b8d
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_7_blessed_strikes
splat_name: 'Level 7: Blessed Strikes'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_7_blessed_strikes
  parameters:
    choice:
      count: 1
      options:
        divine_strike:
          trigger: actor.weapon_attack_hits_creature
          limit:
            count: 1
            period: actor.turn
          damage:
            dice:
              count: 1
              sides: 8
            type_choice:
            - necrotic
            - radiant
        potent_spellcasting:
          class: cleric
          spell_level: 0
          damage_bonus:
            var: actor.abilities.wisdom.modifier
    legacy_subclass_same_options: only_selected_option_applies
    class_scope:
      class: cleric
      minimum_level: 7
  implementation: dnd2024:class_features_cleric_level_7_blessed_strikes
splat_contracts:
- id: dnd2024:class_features_cleric_level_7_blessed_strikes
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
  - character.feature_acquired
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
  - dnd2024:class_features_cleric_level_7_blessed_strikes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 9c9e2d85dd72fecc13fd627ebf145448efe51fd92b7045965d2e8ab34fd70cad
  parent_name: Cleric
  line_span:
    first: 3109
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 7: Blessed Strikes Divine power infuses you in battle. You gain one of the following
    options of your choice (if you get either option from a Cleric subclass in an older book, use only
    the option you choose for this feature). Divine Strike. Once on each of your turns when you hit a
    creature with an attack roll using a weapon, you can cause the target to take an extra 1d8 Necrotic
    or Radiant damage (your choice). Potent Spellcasting. Add your Wisdom modifier to the damage you deal
    with any Cleric cantrip.'
  sha256: 8137dae44ebe0651496a7d53a795b7a57a6767cc4b878a878a1e78f8d0556e5a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Blessed Strikes

<!-- SRD page 38 -->

Divine power infuses you in battle. You gain one of\
the following options of your choice (if you get ei-\
ther option from a Cleric subclass in an older book,\
use only the option you choose for this feature).\
Divine Strike. Once on each of your turns when\
you hit a creature with an attack roll using a\
weapon, you can cause the target to take an extra\
1d8 Necrotic or Radiant damage (your choice).\
Potent Spellcasting. Add your Wisdom modifier\
to the damage you deal with any Cleric cantrip.\
