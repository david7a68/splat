---
splat_schema: 1
splat_id: 9fdaa056-6bc5-5ad5-b58e-23b119e5f15a
splat_kind: rule
splat_key: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
splat_name: 'Level 17: Quivering Palm'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/warrior_of_the_open_hand_level_17_quivering_palm
  parameters:
    apply:
      trigger: unarmed_strike.hit_creature
      focus_cost: 4
      duration:
        days:
          stat: classes.monk.level
      perceptible: false
      passive_damage: 0
    maximum_active_creatures: 1
    detonate:
      action_options:
      - action: action
      - action: replace_one_attack
        requires_action: attack
        turn: own
      requires_same_plane: true
      save:
        ability: constitution
        dc:
          op: add
          args:
          - 8
          - stat: abilities.wisdom.modifier
          - stat: proficiency_bonus
        failure_damage:
          dice:
            count: 10
            sides: 12
        success_damage_multiplier: 0.5
        damage_type: force
      ends_vibrations: true
    dismiss:
      action: none
      harmless: true
  implementation: dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm
splat_contracts:
- id: dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm
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
  - resource_changes
  - damage
  events:
  - attack.hit
  - action.attack
  - action.taken
  capabilities:
  - effects
  - resources
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_warrior_of_the_open_hand_level_17_quivering_palm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d039cec35c05836cb113a814a3a9afd9eb332c231b4e8b313a02ecce8e532b0d
  parent_name: Warrior of the Open Hand
  line_span:
    first: 4380
    count: 18
splat_clauses:
- key: source_rules
  text: '# Level 17: Quivering Palm You gain the ability to set up lethal vibrations in someone’s body.
    When you hit a creature with an Unarmed Strike, you can expend 4 Focus Points to start these imperceptible
    vibrations, which last for a number of days equal to your Monk level. The vibrations are harmless
    unless you take an action to end them. Alternatively, when you take the Attack action on your turn,
    you can forgo one of the attacks to end the vibrations. To end them, you and the target must be on
    the same plane of existence. When you end them, the target must make a Constitution saving throw,
    taking 10d12 Force damage on a failed save or half as much damage on a successful one. You can have
    only one creature under the effect of this feature at a time. You can end the vibrations harmlessly
    (no action required).'
  sha256: b5ba64b2c8914def5f586f5c51b4b71a37697b41bb3a1eb0f96f02971eb2a7f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 17: Quivering Palm

<!-- SRD page 52 -->

You gain the ability to set up lethal vibrations in\
someone’s body. When you hit a creature with an\
Unarmed Strike, you can expend 4 Focus Points\
to start these imperceptible vibrations, which last\
for a number of days equal to your Monk level. The\
vibrations are harmless unless you take an action\
to end them. Alternatively, when you take the At-\
tack action on your turn, you can forgo one of the\
attacks to end the vibrations. To end them, you and\
the target must be on the same plane of existence.\
When you end them, the target must make a Consti-\
tution saving throw, taking 10d12 Force damage on\
a failed save or half as much damage on a successful\
one.\
You can have only one creature under the effect\
of this feature at a time. You can end the vibrations\
harmlessly (no action required).\
