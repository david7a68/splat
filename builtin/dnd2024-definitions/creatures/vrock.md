---
splat_schema: 1
splat_id: ba1b2ff5-3b5f-5f57-8ee9-517931cc2356
splat_kind: creature
splat_key: dnd2024:creatures/vrock
splat_name: Vrock
splat_origin:
  key: dnd2024:reference/creature/vrock
splat_sources:
- title: SRD 5.2.1
  locator: pp. 336
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 40 ft., Fly 60 ft.
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Poisoned
  senses_text: Darkvision 120 ft.; Passive Perception 11
  languages_text: Abyssal; telepathy 120 ft.
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 152 (16d10 + 64)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 15
  save_dexterity:
    value: 5
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 13
  save_wisdom:
    value: 4
  charisma:
    value: 8
  save_charisma:
    value: 2
  armor_class:
    value: 15
  max_hp:
    value: 152
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/vrock
  implementation: dnd2024:creature_vrock
  parameters:
    features:
      demonic_restoration:
        economy: trait
        operation: passive_rule
        trigger: self.dies
        unless:
          plane: abyss
        remove_old_body: true
        new_body:
          delay_seconds: 0
          plane: abyss
          hp: maximum
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: shred
          count: 2
      shred:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
        hit:
          damage:
          - average: 10
            roll:
              count: 2
              sides: 6
              modifier: 3
            type: piercing
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: poison
      spores:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: constitution
        dc: 15
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 20
            origin: self
        failure:
          conditions:
          - condition: poisoned
          repeat_save:
            event: target.turn_end
            success: end_effect
          while_poisoned:
            damage_each_target_turn_start:
            - roll:
                count: 1
                sides: 10
                modifier: 0
              type: poison
          ends_on:
            operation: empty_holy_water_flask_on_target
        success:
          effects: []
      stunning_screech:
        economy: action
        resource:
          kind: daily
          uses: 1
          recovery: long_rest
        operation: saving_throw
        ability: constitution
        dc: 15
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 20
            origin: self
        failure:
          damage:
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: thunder
          conditions:
          - condition: stunned
            until: self.next_turn_end
        automatic_success_tags:
        - demon
        success:
          effects: []
    damage_resistances:
    - cold
    - fire
    - lightning
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
      darkvision: 120
      passive_perception: 11
    languages:
      speaks:
      - abyssal
      understands: []
      telepathy_range_ft: 120
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_vrock
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - turn.ended
  - creature.died
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 36bc9307f0e10dd22e714bbf4cd86866286c468c9ed0f5ddfeffce8575e21b4a
  source_path: builtin/srd-5.2.1/creature/vrock.md
  pages:
  - 336
  engine_gaps: &id001
  - dnd2024:creature_vrock
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 3785f620376a2bf7cd6dcde0f2bb49e722d460e8dac8a87202a0800c7db4cd93
splat_clauses:
- key: source_rules
  text: '# Vrock Large Fiend (Demon), Chaotic Evil AC 15 Initiative +2 (12) HP 152 (16d10 + 64) Speed
    40 ft., Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 15 +2 +5 Con 18 +4 +4 Int 8 -1 -1 Wis
    13 +1 +4 Cha 8 -1 +2 Resistances Cold, Fire, Lightning Immunities Poison; Poisoned Senses Darkvision
    120 ft.; Passive Perception 11 Languages Abyssal; telepathy 120 ft. CR 6 (XP 2,300; PB +3) ### Traits
    Demonic Restoration. If the vrock dies outside the Abyss, its body dissolves into ichor, and it gains
    a new body instantly, reviving with all its Hit Points somewhere in the Abyss. Magic Resistance. The
    vrock has Advantage on saving throws against spells and other magical effects. ### Actions Multiattack.
    The vrock makes two Shred attacks. Shred. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 + 3) Piercing
    damage plus 10 (3d6) Poison damage. Spores (Recharge 6). Constitution Saving Throw: DC 15, each creature
    in a 20-foot Emanation originating from the vrock. Failure: The target has the Poisoned condition
    and repeats the save at the end of each of its turns, ending the effect on itself on a success. While
    Poisoned, the target takes 5 (1d10) Poison damage at the start of each of its turns. Emptying a flask
    of Holy Water on the target ends the effect early. Stunning Screech (1/Day). Constitution Saving Throw:
    DC 15, each creature in a 20-foot Emanation originating from the vrock (demons succeed automatically).
    Failure: 10 (3d6) Thunder damage, and the target has the Stunned condition until the end of the vrock’s
    next turn.'
  sha256: ddf4bf374c6c0420b1f4097d546049ad3fd9d5880e96c0be5b4e52060899fd90
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vrock

<!-- SRD page 336 -->

Large Fiend (Demon), Chaotic Evil\
AC 15 Initiative +2 (12)\
HP 152 (16d10 + 64)\
Speed 40 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 15 +2 +5 Con 18 +4 +4\
Int 8 −1 −1 Wis 13 +1 +4 Cha 8 −1 +2\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Abyssal; telepathy 120 ft.\
CR 6 (XP 2,300; PB +3)\

### Traits

Demonic Restoration. If the vrock dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The vrock has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The vrock makes two Shred attacks.\
Shred. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6\
+ 3) Piercing damage plus 10 (3d6) Poison damage.\
Spores (Recharge 6). Constitution Saving Throw: DC\
15, each creature in a 20-foot Emanation originating\
from the vrock. Failure: The target has the Poisoned\
condition and repeats the save at the end of each of its\
turns, ending the effect on itself on a success. While\
Poisoned, the target takes 5 (1d10) Poison damage at\
the start of each of its turns. Emptying a flask of Holy\
Water on the target ends the effect early.\
Stunning Screech (1/Day). Constitution Saving Throw:\
DC 15, each creature in a 20-foot Emanation originating\
from the vrock (demons succeed automatically). Fail-\
ure: 10 (3d6) Thunder damage, and the target has the\
Stunned condition until the end of the vrock’s next turn.\

Original layout: [[../../srd-5.2.1/pages/336|page 336]].
