---
splat_schema: 1
splat_id: 9faba9cc-ccd6-51b2-9585-817d222f1256
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
splat_name: 'Level 3: Open Hand Technique'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_3_open_hand_technique
  parameters:
    trigger: actor.flurry_of_blows_attack_hits_creature
    optional: true
    choose: 1
    save_dc:
      op: add
      args:
      - 8
      - var: actor.abilities.wisdom.modifier
      - var: actor.proficiency_bonus
    options:
      addle:
        opportunity_attacks: false
        end: target.next_turn.start
      push:
        save: strength
        failure:
          push:
            maximum_feet: 15
            direction: away_from_actor
      topple:
        save: dexterity
        failure:
          condition: prone
    class_scope:
      class: monk
      minimum_level: 3
  implementation: dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique
splat_contracts:
- id: dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique
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
  - turn.start
  capabilities:
  - character_features
  - conditions
  - proficiencies
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_warrior_of_the_open_hand_level_3_open_hand_technique
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 1985f5f232874d1c15b6e11aac2674a27a5ebf13ad60d47f68840b93975ab597
  parent_name: Warrior of the Open Hand
  line_span:
    first: 4356
    count: 10
splat_clauses:
- key: source_rules
  text: '# Level 3: Open Hand Technique Whenever you hit a creature with an attack granted by your Flurry
    of Blows, you can impose one of the following effects on that target. Addle. The target can’t make
    Opportunity Attacks until the start of its next turn. Push. The target must succeed on a Strength
    saving throw or be pushed up to 15 feet away from you. Topple. The target must succeed on a Dexterity
    saving throw or have the Prone condition.'
  sha256: e571ec3c4ef55d9a2c09b303950ddea4de635d6e483871692198524a82665653
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Open Hand Technique

<!-- SRD page 52 -->

Whenever you hit a creature with an attack granted\
by your Flurry of Blows, you can impose one of the\
following effects on that target.\
Addle. The target can’t make Opportunity Attacks\
until the start of its next turn.\
Push. The target must succeed on a Strength sav-\
ing throw or be pushed up to 15 feet away from you.\
Topple. The target must succeed on a Dexterity\
saving throw or have the Prone condition.\
