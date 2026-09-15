---
splat_schema: 1
splat_id: 36badd6b-3407-5b39-9565-dd4d74569f8c
splat_kind: rule
splat_key: dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence
splat_name: 'Level 14: Intimidating Presence'
splat_origin:
  key: dnd2024:reference/pages/030
splat_sources:
- title: SRD 5.2.1
  locator: pp. 30
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/path_of_the_berserker_level_14_intimidating_presence
  parameters:
    action: bonus_action
    resource:
      id: berserker.intimidating_presence
      maximum: 1
      recovery:
      - event: rest.long_completed
        amount: all
    cost: 1
    recharge_alternative:
      action: none
      cost:
        resource: barbarian.rage
        amount: 1
      restore_uses: 1
    area:
      shape: emanation
      radius_feet: 30
      origin: actor
    targets:
      type: creature
      selection: actor_choice
    save:
      ability: wisdom
      dc:
        op: add
        args:
        - 8
        - var: actor.abilities.strength.modifier
        - var: actor.proficiency_bonus
      failure:
        condition: frightened
        source: actor
        duration:
          minutes: 1
      repeat:
        event: target.turn.end
        success: end_effect
    class_scope:
      class: barbarian
      minimum_level: 14
  implementation: dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence
splat_contracts:
- id: dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence
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
  - action.bonus
  - turn.end
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - conditions
  - proficiencies
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 30
  engine_gaps: &id001
  - dnd2024:class_features_path_of_the_berserker_level_14_intimidating_presence
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 8485bbf3fddf3e0c5f5b17c61910d568f6953229c08727c075fb3b083a4325a9
  parent_name: Path of the Berserker
  line_span:
    first: 2477
    count: 15
splat_clauses:
- key: source_rules
  text: '# Level 14: Intimidating Presence As a Bonus Action, you can strike terror into others with your
    menacing presence and primal power. When you do so, each creature of your choice in a 30-foot Emanation
    originating from you must make a Wisdom saving throw (DC 8 plus your Strength modifier and Proficiency
    Bonus). On a failed save, a creature has the Frightened condition for 1 minute. At the end of each
    of the Frightened creature’s turns, the creature repeats the save, ending the effect on itself on
    a success. Once you use this feature, you can’t use it again until you finish a Long Rest unless you
    expend a use of your Rage (no action required) to restore your use of it.'
  sha256: 8fa500c3c421200c36f1c654f19038601641836808c47957511d8a458ad6decc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Intimidating Presence

<!-- SRD page 30 -->

As a Bonus Action, you can strike terror into others\
with your menacing presence and primal power.\
When you do so, each creature of your choice in a\
30-foot Emanation originating from you must make\
a Wisdom saving throw (DC 8 plus your Strength\
modifier and Proficiency Bonus). On a failed save,\
a creature has the Frightened condition for 1 min-\
ute. At the end of each of the Frightened creature’s\
turns, the creature repeats the save, ending the ef-\
fect on itself on a success.\
Once you use this feature, you can’t use it again\
until you finish a Long Rest unless you expend a use\
of your Rage (no action required) to restore your\
use of it.\
