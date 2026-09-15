---
splat_schema: 1
splat_id: 34d57125-2ab3-533f-8cfc-df44ed4a96ee
splat_kind: creature
splat_key: dnd2024:creatures/bandit_captain
splat_name: Bandit Captain
splat_origin:
  key: dnd2024:reference/creature/bandit_captain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 261, 262
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Athletics +4, Deception +4
  gear_text: Pistol, Scimitar, Studded Leather Armor
  senses_text: Passive Perception 10
  languages_text: Common, Thieves’ Cant
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 52 (8d8 + 16)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 4
  dexterity:
    value: 16
  save_dexterity:
    value: 5
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 14
  save_charisma:
    value: 2
  armor_class:
    value: 15
  max_hp:
    value: 52
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bandit_captain
  implementation: dnd2024:creature_bandit_captain
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
          - pistol
      scimitar:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 6
            roll:
              count: 1
              sides: 6
              modifier: 3
            type: slashing
      pistol:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          range_ft: 30
          long_range_ft: 90
        hit:
          damage:
          - average: 8
            roll:
              count: 1
              sides: 10
              modifier: 3
            type: piercing
      parry:
        economy: reaction
        operation: modify_ac
        trigger: self.hit_by_melee_attack
        requires:
          holding_weapon: true
        bonus: 2
        scope: triggering_attack
        reevaluate_hit: true
    skill_totals:
      athletics: 4
      deception: 4
    gear:
    - item: pistol
      quantity: 1
    - item: scimitar
      quantity: 1
    - item: studded_leather_armor
      quantity: 1
    senses:
      passive_perception: 10
    languages:
      speaks:
      - common
      - thieves_cant
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
- id: dnd2024:creature_bandit_captain
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 75aca46eb6e14679647c8201353608826e6e70ba00a056c374678024c7074a13
  source_path: builtin/srd-5.2.1/creature/bandit_captain.md
  pages:
  - 261
  - 262
  engine_gaps: &id001
  - dnd2024:creature_bandit_captain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 64824d28ef6dc7c6b231284ac5a61fabb062249e4410114d6264943ee1fa1270
splat_clauses:
- key: source_rules
  text: '# Bandit Captain Medium or Small Humanoid, Neutral AC 15 Initiative +3 (13) HP 52 (8d8 + 16)
    Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +4 Dex 16 +3 +5 Con 14 +2 +2 Int 14 +2 +2 Wis 11
    +0 +2 Cha 14 +2 +2 Skills Athletics +4, Deception +4 Gear Pistol, Scimitar, Studded Leather Armor
    Senses Passive Perception 10 Languages Common, Thieves’ Cant CR 2 (XP 450; PB +2) ### Actions Multiattack.
    The bandit makes two attacks, using Scimitar and Pistol in any combination. Scimitar. Melee Attack
    Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Slashing damage. Pistol. Ranged Attack Roll: +5, range 30/90
    ft. Hit: 8 (1d10 + 3) Piercing damage. ### Reactions Parry. Trigger: The bandit is hit by a melee
    attack roll while holding a weapon. Response: The bandit adds 2 to its AC against that attack, possibly
    causing it to miss.'
  sha256: 0c9593a4e62126a9b867a4738e62002abe6be1bfa85db5c19cd771d25503c264
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bandit Captain

<!-- SRD page 261 -->

Medium or Small Humanoid, Neutral\
AC 15 Initiative +3 (13)\
HP 52 (8d8 + 16)\
Speed 30 ft.\

<!-- SRD page 262 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +4 Dex 16 +3 +5 Con 14 +2 +2\
Int 14 +2 +2 Wis 11 +0 +2 Cha 14 +2 +2\
Skills Athletics +4, Deception +4\
Gear Pistol, Scimitar, Studded Leather Armor\
Senses Passive Perception 10\
Languages Common, Thieves’ Cant\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The bandit makes two attacks, using Scim-\
itar and Pistol in any combination.\
Scimitar. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Slashing damage.\
Pistol. Ranged Attack Roll: +5, range 30/90 ft. Hit: 8\
(1d10 + 3) Piercing damage.\

### Reactions

Parry. Trigger: The bandit is hit by a melee attack roll\
while holding a weapon. Response: The bandit adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/261|page 261]], [[../../srd-5.2.1/pages/262|page 262]].
