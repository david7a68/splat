---
splat_schema: 1
splat_id: 4db58924-8af5-5174-8218-1a90dee028b5
splat_kind: creature
splat_key: dnd2024:creatures/medusa
splat_name: Medusa
splat_origin:
  key: dnd2024:reference/creature/medusa
splat_sources:
- title: SRD 5.2.1
  locator: pp. 306, 307
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Lawful Evil
  movement_text: 30 ft.
  skills_text: Deception +5, Perception +4, Stealth +6
  senses_text: Darkvision 150 ft.; Passive Perception 14
  languages_text: Common plus one other language
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 127 (17d8 + 51)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 13
  save_wisdom:
    value: 4
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 15
  max_hp:
    value: 127
  initiative:
    value: 6
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/medusa
  implementation: dnd2024:creature_medusa
  parameters:
    features:
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 10
          amount:
            dice:
              count: 2
              sides: 6
            add: 3
          damage_type: slashing
        reach_feet: 5
      snake_hair:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 4
            add: 3
          damage_type: piercing
        - op: damage
          average: 14
          amount:
            dice:
              count: 4
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 5
      poison_ray:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 11
          amount:
            dice:
              count: 2
              sides: 8
            add: 2
          damage_type: poison
        range_feet: 150
      multiattack:
        economy: action
        op: choose_sequence
        choices:
        - - op: use_feature
            feature: claw
            count: 2
          - op: use_feature
            feature: snake_hair
            count: 1
        - - op: use_feature
            feature: poison_ray
            count: 3
      petrifying_gaze:
        economy: bonus_action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: constitution
        dc: 13
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: condition
          condition: restrained
          ends:
            event: repeat_save.success
          repeat_save:
            event: target.turn.end
            occurrence: next
            requires_condition: restrained
            ability: constitution
            dc: 13
            success: end_effect
            failure:
            - op: remove_condition
              condition: restrained
            - op: condition
              condition: petrified
        success: []
        additional_target:
          participant: self
          when: self_sees_own_reflection_in_cone
    skill_totals:
      deception: 5
      perception: 4
      stealth: 6
    defenses: {}
    senses:
      passive_perception: 14
      special:
      - sense: darkvision
        range_feet: 150
    communication:
      speaks:
      - common
      understands:
      - common
      additional_languages:
        count: 1
        chooser: gm
        distinct_from_known: true
        can_speak: true
        can_understand: true
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_medusa
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - repeat_save.success
  - target.turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 4a5b47f88f865c6032ad5d1d589150c2c056da6ed6d1bf97d44d48c99239ca63
  source_path: builtin/srd-5.2.1/creature/medusa.md
  pages:
  - 306
  - 307
  engine_gaps: &id001
  - dnd2024:creature_medusa
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: a5bc7ee795264e2321eff01ea922a7031320e0f70a29a2c12541dfd123d9e330
splat_clauses:
- key: source_rules
  text: '# Medusa Medium Monstrosity, Lawful Evil AC 15 Initiative +6 (16) HP 127 (17d8 + 51) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 17 +3 +3 Con 16 +3 +3 Int 12 +1 +1 Wis 13 +1 +4 Cha
    15 +2 +2 Skills Deception +5, Perception +4, Stealth +6 Senses Darkvision 150 ft.; Passive Perception
    14 Languages Common plus one other language CR 6 (XP 2,300; PB +3) ### Actions Multiattack. The medusa
    makes two Claw attacks and one Snake Hair attack, or it makes three Poison Ray attacks. Claw. Melee
    Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. Snake Hair. Melee Attack Roll: +6,
    reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage plus 14 (4d6) Poison damage. Poison Ray. Ranged Attack
    Roll: +5, range 150 ft. Hit: 11 (2d8 + 2) Poison damage. ### Bonus Actions Petrifying Gaze (Recharge
    5–6). Constitution Saving Throw: DC 13, each creature in a 30-foot Cone. If the medusa sees its reflection
    in the Cone, the medusa must make this save. First Failure: The target has the Restrained condition
    and repeats the save at the end of its next turn if it is still Restrained, ending the effect on itself
    on a success. Second Failure: The target has the Petrified condition instead of the Restrained condition.'
  sha256: 3a61f2fbe1b0b323ed00ad88b1322ad8659aa0c1dd3eeb4caf615d0802f166fc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Medusa

<!-- SRD page 306 -->

Medium Monstrosity, Lawful Evil\
AC 15 Initiative +6 (16)\
HP 127 (17d8 + 51)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 17 +3 +3 Con 16 +3 +3\
Int 12 +1 +1 Wis 13 +1 +4 Cha 15 +2 +2\
Skills Deception +5, Perception +4, Stealth +6\
Senses Darkvision 150 ft.; Passive Perception 14\
Languages Common plus one other language\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The medusa makes two Claw attacks\
and one Snake Hair attack, or it makes three Poison\
Ray attacks.\
Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage.\
Snake Hair. Melee Attack Roll: +6, reach 5 ft. Hit:\
5 (1d4 + 3) Piercing damage plus 14 (4d6) Poi-\
son damage.\
Poison Ray. Ranged Attack Roll: +5, range 150 ft. Hit:\
11 (2d8 + 2) Poison damage.\

<!-- SRD page 307 -->


### Bonus Actions

Petrifying Gaze (Recharge 5–6). Constitution Saving\
Throw: DC 13, each creature in a 30-foot Cone. If the\
medusa sees its reflection in the Cone, the medusa\
must make this save. First Failure: The target has the\
Restrained condition and repeats the save at the end of\
its next turn if it is still Restrained, ending the effect on\
itself on a success. Second Failure: The target has the\
Petrified condition instead of the Restrained condition.\

Original layout: [[../../srd-5.2.1/pages/306|page 306]], [[../../srd-5.2.1/pages/307|page 307]].
