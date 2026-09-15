---
splat_schema: 1
splat_id: 433c4671-5681-5c3f-9a38-f7eca923b6d5
splat_kind: creature
splat_key: dnd2024:creatures/otyugh
splat_name: Otyugh
splat_origin:
  key: dnd2024:reference/creature/otyugh
splat_sources:
- title: SRD 5.2.1
  locator: pp. 313
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: aberration
  alignment: Neutral
  movement_text: 30 ft.
  senses_text: Darkvision 120 ft.; Passive Perception 11
  languages_text: Otyugh; telepathy 120 ft. (doesn’t allow the receiving creature to respond telepathically)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 104 (11d10 + 44)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 19
  save_constitution:
    value: 7
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 104
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/otyugh
  implementation: dnd2024:creature_otyugh
  parameters:
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 11
    languages:
      speak: true
      known:
      - otyugh
    telepathy:
      range_ft: 120
      two_way: false
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 8
              add: 3
            average: 12
        - op: condition.apply
          condition: poisoned
          on_target_long_rest_finished:
            op: saving_throw
            ability: constitution
            dc: 15
            targets:
              kind: target
            failure:
            - op: hp_maximum.reduce
              amount:
                dice:
                  count: 1
                  sides: 10
                add: 0
              average: 5
              recovery: this_poisoned_condition_ends
            success:
            - op: effect.end
      tentacle:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 8
              add: 3
            average: 12
        - op: grapple
          escape_dc: 13
          when:
            target_kind: creature
            target_maximum_size: medium
          limb_slots: 2
          slots_per_target: 1
      tentacle_slam:
        economy: action
        op: saving_throw
        ability: constitution
        dc: 14
        targets:
          kind: creature
          count: all
          grappled_by: actor
        failure:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 8
              add: 3
            average: 16
        - op: condition.apply
          condition: stunned
          duration:
            until: actor.next_turn_start
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 1
          feature: bite
        - count: 2
          feature: tentacle
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_otyugh
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 7df039a0501bd31fdd1c1323973047d6dcc705fd97c309efcd7b8948c8323d72
  source_path: builtin/srd-5.2.1/creature/otyugh.md
  pages:
  - 313
  engine_gaps: &id001
  - dnd2024:creature_otyugh
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: f9c34c8182854c47794706f95ae83a7d5fe00707fdeb0d048eaf93ecb301b1db
splat_clauses:
- key: source_rules
  text: '# Otyugh Large Aberration, Neutral AC 14 Initiative +0 (10) HP 104 (11d10 + 44) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 11 +0 +0 Con 19 +4 +7 Int 6 -2 -2 Wis 13 +1 +1 Cha 6 -2
    -2 Senses Darkvision 120 ft.; Passive Perception 11 Languages Otyugh; telepathy 120 ft. (doesn’t allow
    the receiving creature to respond telepathically) CR 5 (XP 1,800; PB +3) ### Actions Multiattack.
    The otyugh makes one Bite attack and two Tentacle attacks. Bite. Melee Attack Roll: +6, reach 5 ft.
    Hit: 12 (2d8 + 3) Piercing damage, and the target has the Poisoned condition. Whenever the Poisoned
    target finishes a Long Rest, it is subjected to the following effect. Constitution Saving Throw: DC
    15. Failure: The target’s Hit Point maximum decreases by 5 (1d10) and doesn’t return to normal until
    the Poisoned condition ends on the target. Success: The Poisoned condition ends. Tentacle. Melee Attack
    Roll: +6, reach 10 ft. Hit: 12 (2d8 + 3) Piercing damage. If the target is a Medium or smaller creature,
    it has the Grappled condition (escape DC 13) from one of two tentacles. Tentacle Slam. Constitution
    Saving Throw: DC 14, each creature Grappled by the otyugh. Failure: 16 (3d8 + 3) Bludgeoning damage,
    and the target has the Stunned condition until the start of the otyugh’s next turn. Success: Half
    damage only.'
  sha256: 32cef42de27fbd202fcd98635a17aae84a528dbb9be8ae710b21bdcc36de22a4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Otyugh

<!-- SRD page 313 -->

Large Aberration, Neutral\
AC 14 Initiative +0 (10)\
HP 104 (11d10 + 44)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 11 +0 +0 Con 19 +4 +7\
Int 6 −2 −2 Wis 13 +1 +1 Cha 6 −2 −2\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Otyugh; telepathy 120 ft. (doesn’t allow the\
receiving creature to respond telepathically)\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The otyugh makes one Bite attack and two\
Tentacle attacks.\
Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 12 (2d8 + 3)\
Piercing damage, and the target has the Poisoned con-\
dition. Whenever the Poisoned target finishes a Long\
Rest, it is subjected to the following effect. Constitution\
Saving Throw: DC 15. Failure: The target’s Hit Point\
maximum decreases by 5 (1d10) and doesn’t return to\
normal until the Poisoned condition ends on the target.\
Success: The Poisoned condition ends.\
Tentacle. Melee Attack Roll: +6, reach 10 ft. Hit: 12\
(2d8 + 3) Piercing damage. If the target is a Medium or\
smaller creature, it has the Grappled condition (escape\
DC 13) from one of two tentacles.\
Tentacle Slam. Constitution Saving Throw: DC 14, each\
creature Grappled by the otyugh. Failure: 16 (3d8 + 3)\
Bludgeoning damage, and the target has the Stunned\
condition until the start of the otyugh’s next turn. Suc-\
cess: Half damage only.\

Original layout: [[../../srd-5.2.1/pages/313|page 313]].
