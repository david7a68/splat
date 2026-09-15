---
splat_schema: 1
splat_id: 7d62036e-996d-516f-b89d-a8f1762641c4
splat_kind: creature
splat_key: dnd2024:creatures/vampire_spawn
splat_name: Vampire Spawn
splat_origin:
  key: dnd2024:reference/creature/vampire_spawn
splat_sources:
- title: SRD 5.2.1
  locator: pp. 334, 335
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Neutral Evil
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Perception +3, Stealth +6
  resistances_text: Necrotic
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: Common plus one other language
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 90 (12d8 + 36)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 16
  save_dexterity:
    value: 6
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 16
  max_hp:
    value: 90
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/vampire_spawn
  implementation: dnd2024:creature_vampire_spawn
  parameters:
    features:
      spider_climb:
        economy: trait
        operation: passive_rule
        climb_difficult_surfaces: true
        climb_ceilings: true
        ability_check_required: false
      vampire_weakness:
        economy: trait
        operation: passive_rule
        weaknesses:
        - forbiddance
        - running_water
        - stake_to_the_heart
        - sunlight
      forbiddance:
        economy: trait
        operation: passive_rule
        residence_entry_requires: occupant_invitation
      running_water:
        economy: trait
        operation: passive_rule
        trigger: self.turn_end
        requires:
          in_running_water: true
        damage:
        - roll:
            constant: 20
          type: acid
      stake_to_the_heart:
        economy: trait
        operation: passive_rule
        trigger: piercing_weapon_driven_into_heart
        requires:
          self_condition: incapacitated
        destroy_self: true
      sunlight:
        economy: trait
        operation: passive_rule
        trigger: self.turn_start
        requires:
          in_sunlight: true
        damage:
        - roll:
            constant: 20
          type: radiant
        while_sunlight:
          disadvantage_on:
          - attack_rolls
          - ability_checks
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: claw
          count: 2
        - operation: use_feature
          feature: bite
      claw:
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
          - average: 8
            roll:
              count: 2
              sides: 4
              modifier: 3
            type: slashing
          grapple:
            condition: grappled
            source: self
            escape_dc: 13
            maximum_size: medium
            appendage_pool:
              count: 2
              type: claws
              cost: 1
      bite:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 14
        targets:
          kind: creature
          count: 1
          range_ft: 5
          any_of:
          - willing: true
          - condition: grappled
          - condition: incapacitated
          - condition: restrained
        failure:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 4
              modifier: 3
            type: piercing
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: necrotic
          maximum_hp_reduction:
            equal: necrotic_damage_taken
          heal_self:
            equal: necrotic_damage_taken
        success:
          effects: []
      deathless_agility:
        economy: bonus_action
        operation: choose_action
        count: 1
        options:
        - dash
        - disengage
    skill_totals:
      perception: 3
      stealth: 6
    damage_resistances:
    - necrotic
    senses:
      darkvision: 60
      passive_perception: 13
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 1
        chooser: creator
        can_speak: true
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
- id: dnd2024:creature_vampire_spawn
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
  - damage.resolved
  - attack.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - hit_points
splat_conversion:
  status: specified
  category: creature
  source_sha256: 62fba612d42982e68c5a4797f64b8a2e5e13faa57d1588e959ed5146af008184
  source_path: builtin/srd-5.2.1/creature/vampire_spawn.md
  pages:
  - 334
  - 335
  engine_gaps: &id001
  - dnd2024:creature_vampire_spawn
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 60f0dc57d65fd849f42e15bb722987d5c2338dc9100cbf9f6bac38258d744dcc
splat_clauses:
- key: source_rules
  text: '# Vampire Spawn Medium or Small Undead, Neutral Evil AC 16 Initiative +3 (13) HP 90 (12d8 + 36)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 16 +3 +6 Con 16 +3 +3 Int 11 +0 +0 Wis 10
    +0 +3 Cha 12 +1 +1 Skills Perception +3, Stealth +6 Resistances Necrotic Senses Darkvision 60 ft.;
    Passive Perception 13 Languages Common plus one other language CR 5 (XP 1,800; PB +3) ### Traits Spider
    Climb. The vampire can climb difficult surfaces, including along ceilings, without needing to make
    an ability check. Vampire Weakness. The vampire has these weaknesses: Forbiddance. The vampire can’t
    enter a residence without an invitation from an occupant. Running Water. The vampire takes 20 Acid
    damage if it ends its turn in running water. Stake to the Heart. The vampire is destroyed if a weapon
    that deals Piercing damage is driven into the vampire’s heart while the vampire has the Incapacitated
    condition. Sunlight. The vampire takes 20 Radiant damage if it starts its turn in sunlight. While
    in sunlight, it has Disadvantage on attack rolls and ability checks. ### Actions Multiattack. The
    vampire makes two Claw attacks and uses Bite. Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (2d4
    + 3) Slashing damage. If the target is a Medium or smaller creature, it has the Grappled condition
    (escape DC 13) from one of two claws. Bite. Constitution Saving Throw: DC 14, one creature within
    5 feet that is willing or that has the Grappled, Incapacitated, or Restrained condition. Failure:
    5 (1d4 + 3) Piercing damage plus 10 (3d6) Necrotic damage. The target’s Hit Point maximum decreases
    by an amount equal to the Necrotic damage taken, and the vampire regains Hit Points equal to that
    amount. ### Bonus Actions Deathless Agility. The vampire takes the Dash or Disengage action.'
  sha256: 8f9b270d5a19dd8f93958984cd2bbe80f781adb11cf6c1c01de8500364dce2f2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Vampire Spawn

<!-- SRD page 334 -->

Medium or Small Undead, Neutral Evil\
AC 16 Initiative +3 (13)\
HP 90 (12d8 + 36)\
Speed 30 ft.\

<!-- SRD page 335 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 16 +3 +6 Con 16 +3 +3\
Int 11 +0 +0 Wis 10 +0 +3 Cha 12 +1 +1\
Skills Perception +3, Stealth +6\
Resistances Necrotic\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages Common plus one other language\
CR 5 (XP 1,800; PB +3)\

### Traits

Spider Climb. The vampire can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Vampire Weakness. The vampire has these\
weaknesses:\
Forbiddance. The vampire can’t enter a residence with-\
out an invitation from an occupant.\
Running Water. The vampire takes 20 Acid damage if it\
ends its turn in running water.\
Stake to the Heart. The vampire is destroyed if a\
weapon that deals Piercing damage is driven into the\
vampire’s heart while the vampire has the Incapaci-\
tated condition.\
Sunlight. The vampire takes 20 Radiant damage if it\
starts its turn in sunlight. While in sunlight, it has Dis-\
advantage on attack rolls and ability checks.\

### Actions

Multiattack. The vampire makes two Claw attacks and\
uses Bite.\
Claw. Melee Attack Roll: +6, reach 5 ft. Hit: 8 (2d4 + 3)\
Slashing damage. If the target is a Medium or smaller\
creature, it has the Grappled condition (escape DC 13)\
from one of two claws.\
Bite. Constitution Saving Throw: DC 14, one creature\
within 5 feet that is willing or that has the Grappled,\
Incapacitated, or Restrained condition. Failure: 5 (1d4\
+ 3) Piercing damage plus 10 (3d6) Necrotic dam-\
age. The target’s Hit Point maximum decreases by an\
amount equal to the Necrotic damage taken, and the\
vampire regains Hit Points equal to that amount.\

### Bonus Actions

Deathless Agility. The vampire takes the Dash or Dis-\
engage action.\

Original layout: [[../../srd-5.2.1/pages/334|page 334]], [[../../srd-5.2.1/pages/335|page 335]].
