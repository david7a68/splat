---
splat_schema: 1
splat_id: 7b7dd526-12b4-5cff-94fd-749cd6d6a42e
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_1_primal_order
splat_name: 'Level 1: Primal Order'
splat_origin:
  key: dnd2024:reference/pages/042
splat_sources:
- title: SRD 5.2.1
  locator: pp. 42
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_1_primal_order
  parameters:
    choose: 1
    options:
      magician:
        extra_cantrip:
          count: 1
          list: druid
        ability_check_bonus:
          ability: intelligence
          skills:
          - arcana
          - nature
          amount:
            op: max
            args:
            - 1
            - var: actor.abilities.wisdom.modifier
      warden:
        weapon_proficiencies:
        - martial
        armor_training:
        - medium
    class_scope:
      class: druid
      minimum_level: 1
  implementation: dnd2024:class_features_druid_level_1_primal_order
splat_contracts:
- id: dnd2024:class_features_druid_level_1_primal_order
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
  - ability.check
  capabilities:
  - character_features
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 42
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_1_primal_order
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: c859061d0abbcdeaa5077f840a12ff3d23f5d44ae44c87404705d69b5f7b3fc1
  parent_name: Druid
  line_span:
    first: 3483
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 1: Primal Order You have dedicated yourself to one of the following sacred roles of your
    choice. Magician. You know one extra cantrip from the Druid spell list. In addition, your mystical
    connection to nature gives you a bonus to your Intelligence (Arcana or Nature) checks. The bonus equals
    your Wisdom modifier (minimum bonus of +1). Warden. Trained for battle, you gain proficiency with
    Martial weapons and training with Medium armor.'
  sha256: 3d8c66d64b347e95ad4a5a7b83b6c942b5618b14588eaa7fffa8173a74044e7c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Primal Order

<!-- SRD page 42 -->

You have dedicated yourself to one of the following\
sacred roles of your choice.\
Magician. You know one extra cantrip from the\
Druid spell list. In addition, your mystical connec-\
tion to nature gives you a bonus to your Intelligence\
(Arcana or Nature) checks. The bonus equals your\
Wisdom modifier (minimum bonus of +1).\
Warden. Trained for battle, you gain proficiency\
with Martial weapons and training with Medium\
armor.\
