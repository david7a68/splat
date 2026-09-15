---
splat_schema: 1
splat_id: 58b4c331-4c93-59f2-91fc-df243d8d664c
splat_kind: creature
splat_key: dnd2024:creatures/death_dog
splat_name: Death Dog
splat_origin:
  key: dnd2024:reference/creature/death_dog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 279
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral Evil
  movement_text: 40 ft.
  skills_text: Perception +5, Stealth +4
  immunities_text: Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious
  senses_text: Darkvision 120 ft.; Passive Perception 15
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 39 (6d8 + 12)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 39
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/death_dog
  implementation: dnd2024:creature_death_dog
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 4
            add: 2
          damage_type: piercing
        - op: if
          test:
            target_kind: creature
          then:
          - op: save
            ability: constitution
            dc: 12
            targets:
              participant: target
            failure:
            - op: choose_by_predicate
              test:
                target_has_source_poison_effect: true
              then:
              - op: reduce_hit_point_maximum
                average: 5
                amount:
                  dice:
                    count: 1
                    sides: 10
                  add: 0
              else:
              - op: condition
                condition: poisoned
                ends:
                  event: repeated_save.success
                while_active:
                  long_rest_restores_hp_maximum: false
                  repeat_save:
                    ability: constitution
                    dc: 12
                    interval:
                      hours: 24
                    success: end_effect
                    failure:
                    - op: reduce_hit_point_maximum
                      average: 5
                      amount:
                        dice:
                          count: 1
                          sides: 10
                        add: 0
            success: []
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: bite
          count: 2
    skill_totals:
      perception: 5
      stealth: 4
    defenses:
      immunities:
        conditions:
        - blinded
        - charmed
        - deafened
        - frightened
        - stunned
        - unconscious
    senses:
      passive_perception: 15
      special:
      - sense: darkvision
        range_feet: 120
    communication:
      speaks: []
      understands: []
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
- id: dnd2024:creature_death_dog
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
  - repeated_save.success
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
  source_sha256: 6c7269d6ccf79f229b940ad16bc3d6a77a5f81cf704bbd53172fc213b51e3717
  source_path: builtin/srd-5.2.1/creature/death_dog.md
  pages:
  - 279
  engine_gaps: &id001
  - dnd2024:creature_death_dog
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 555813805c4553fb06505fa85e5161985bfc1d8cfa5b245e195151905ec3487f
splat_clauses:
- key: source_rules
  text: '# Death Dog Medium Monstrosity, Neutral Evil AC 12 Initiative +2 (12) HP 39 (6d8 + 12) Speed
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 14 +2 +2 Con 14 +2 +2 Int 3 -4 -4 Wis 13 +1 +1
    Cha 6 -2 -2 Skills Perception +5, Stealth +4 Immunities Blinded, Charmed, Deafened, Frightened, Stunned,
    Unconscious Senses Darkvision 120 ft.; Passive Perception 15 Languages None CR 1 (XP 200; PB +2) ###
    Actions Multiattack. The death dog makes two Bite attacks. Bite. Melee Attack Roll: +4, reach 5 ft.
    Hit: 4 (1d4 + 2) Piercing damage. If the target is a creature, it is subjected to the following effect.
    Constitution Saving Throw: DC 12. First Failure: The target has the Poisoned condition. While Poisoned,
    the target’s Hit Point maximum doesn’t return to normal when finishing a Long Rest, and it repeats
    the save every 24 hours that elapse, ending the effect on itself on a success. Subsequent Failures:
    The Poisoned target’s Hit Point maximum decreases by 5 (1d10).'
  sha256: d1fce97bccab648b2af928eb2892c566c103aa12e45b74d78f71351385582bd6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Death Dog

<!-- SRD page 279 -->

Medium Monstrosity, Neutral Evil\
AC 12 Initiative +2 (12)\
HP 39 (6d8 + 12)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +2 Con 14 +2 +2\
Int 3 −4 −4 Wis 13 +1 +1 Cha 6 −2 −2\
Skills Perception +5, Stealth +4\
Immunities Blinded, Charmed, Deafened, Frightened,\
Stunned, Unconscious\
Senses Darkvision 120 ft.; Passive Perception 15\
Languages None\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The death dog makes two Bite attacks.\
Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 4 (1d4\
+ 2) Piercing damage. If the target is a creature, it is\
subjected to the following effect. Constitution Saving\
Throw: DC 12. First Failure: The target has the Poisoned\
condition. While Poisoned, the target’s Hit Point max-\
imum doesn’t return to normal when finishing a Long\
Rest, and it repeats the save every 24 hours that elapse,\
ending the effect on itself on a success. Subsequent\
Failures: The Poisoned target’s Hit Point maximum de-\
creases by 5 (1d10).\

Original layout: [[../../srd-5.2.1/pages/279|page 279]].
