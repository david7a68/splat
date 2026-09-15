---
splat_schema: 1
splat_id: a3cdd71e-c01d-5b58-8196-b24aaa70472a
splat_kind: creature
splat_key: dnd2024:creatures/goblin_boss
splat_name: Goblin Boss
splat_origin:
  key: dnd2024:reference/creature/goblin_boss
splat_sources:
- title: SRD 5.2.1
  locator: pp. 290
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: small
  type: fey
  alignment: Chaotic Neutral
  tags_text: (Goblinoid)
  movement_text: 30 ft.
  skills_text: Stealth +6
  gear_text: Chain Shirt, Scimitar, Shield, Shortbow
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Common, Goblin
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 21 (6d6)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 17
  max_hp:
    value: 21
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/goblin_boss
  implementation: dnd2024:creature_goblin_boss
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - scimitar
          - shortbow
      scimitar:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 6
              modifier: 2
            type: slashing
          conditional_damage:
            if:
              attack_had_advantage: true
            damage:
            - roll:
                count: 1
                sides: 4
                modifier: 0
              type: slashing
      shortbow:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          range_ft: 80
          long_range_ft: 320
        hit:
          damage:
          - average: 5
            roll:
              count: 1
              sides: 6
              modifier: 2
            type: piercing
          conditional_damage:
            if:
              attack_had_advantage: true
            damage:
            - roll:
                count: 1
                sides: 4
                modifier: 0
              type: piercing
      nimble_escape:
        economy: bonus_action
        operation: choose_action
        options:
        - disengage
        - hide
        count: 1
      redirect_attack:
        economy: reaction
        operation: redirect_attack
        trigger: visible_creature.attack_roll_against_self
        ally:
          within_ft: 5
          size:
          - small
          - medium
          chooser: self
        swap_positions: true
        new_target: chosen_ally
    skill_totals:
      stealth: 6
    gear:
    - item: chain_shirt
      quantity: 1
    - item: scimitar
      quantity: 1
    - item: shield
      quantity: 1
    - item: shortbow
      quantity: 1
    senses:
      darkvision: 60
      passive_perception: 9
    languages:
      speaks:
      - common
      - goblin
      understands: []
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
- id: dnd2024:creature_goblin_boss
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
  - attack.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 44daf7dd0db816bd8bb38569725529b09a558a7ab18f07088b8dec056378cd90
  source_path: builtin/srd-5.2.1/creature/goblin_boss.md
  pages:
  - 290
  engine_gaps: &id001
  - dnd2024:creature_goblin_boss
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 6dc1da35d7be3aae5cbfde705b52109b0efa8f780b692f8d118478c5adff4c48
splat_clauses:
- key: source_rules
  text: '# Goblin Boss Small Fey (Goblinoid), Chaotic Neutral AC 17 Initiative +2 (12) HP 21 (6d6) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 15 +2 +2 Con 10 +0 +0 Int 10 +0 +0 Wis 8 -1 -1
    Cha 10 +0 +0 Skills Stealth +6 Gear Chain Shirt, Scimitar, Shield, Shortbow Senses Darkvision 60 ft.;
    Passive Perception 9 Languages Common, Goblin CR 1 (XP 200; PB +2) ### Actions Multiattack. The goblin
    makes two attacks, using Scimitar or Shortbow in any combination. Scimitar. Melee Attack Roll: +4,
    reach 5 ft. Hit: 5 (1d6 + 2) Slashing damage, plus 2 (1d4) Slashing damage if the attack roll had
    Advantage. Shortbow. Ranged Attack Roll: +4, range 80/320 ft. Hit: 5 (1d6 + 2) Piercing damage, plus
    2 (1d4) Piercing damage if the attack roll had Advantage. ### Bonus Actions Nimble Escape. The goblin
    takes the Disengage or Hide action. ### Reactions Redirect Attack. Trigger: A creature the goblin
    can see makes an attack roll against it. Response: The goblin chooses a Small or Medium ally within
    5 feet of itself. The goblin and that ally swap places, and the ally becomes the target of the attack
    instead.'
  sha256: c4a0b69f95e3e73a98b977ca312f226318c1c80bd59c3939e2a806fc592d75b1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Goblin Boss

<!-- SRD page 290 -->

Small Fey (Goblinoid), Chaotic Neutral\
AC 17 Initiative +2 (12)\
HP 21 (6d6)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 15 +2 +2 Con 10 +0 +0\
Int 10 +0 +0 Wis 8 −1 −1 Cha 10 +0 +0\
Skills Stealth +6\
Gear Chain Shirt, Scimitar, Shield, Shortbow\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Common, Goblin\
CR 1 (XP 200; PB +2)\

### Actions

Multiattack. The goblin makes two attacks, using Scim-\
itar or Shortbow in any combination.\
Scimitar. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Slashing damage, plus 2 (1d4) Slashing damage if\
the attack roll had Advantage.\
Shortbow. Ranged Attack Roll: +4, range 80/320 ft.\
Hit: 5 (1d6 + 2) Piercing damage, plus 2 (1d4) Piercing\
damage if the attack roll had Advantage.\

### Bonus Actions

Nimble Escape. The goblin takes the Disengage or\
Hide action.\

### Reactions

Redirect Attack. Trigger: A creature the goblin can see\
makes an attack roll against it. Response: The goblin\
chooses a Small or Medium ally within 5 feet of itself.\
The goblin and that ally swap places, and the ally be-\
comes the target of the attack instead.\

Original layout: [[../../srd-5.2.1/pages/290|page 290]].
