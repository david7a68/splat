---
splat_schema: 1
splat_id: 717bc686-4fb8-5665-b30a-c6a2b4a22fa4
splat_kind: creature
splat_key: dnd2024:creatures/skeleton
splat_name: Skeleton
splat_origin:
  key: dnd2024:reference/creature/skeleton
splat_sources:
- title: SRD 5.2.1
  locator: pp. 325, 326
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Lawful Evil
  movement_text: 30 ft.
  vulnerabilities_text: Bludgeoning
  immunities_text: Poison; Exhaustion, Poisoned
  gear_text: Shortbow, Shortsword
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Understands Common plus one other language but can’t speak
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 13 (2d8 + 4)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 14
  max_hp:
    value: 13
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/skeleton
  implementation: dnd2024:creature_skeleton
  parameters:
    features:
      shortsword:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: piercing
        reach_feet: 5
      shortbow:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: piercing
        range_feet: 80
        long_range_feet: 320
    defenses:
      vulnerabilities:
        damage_types:
        - bludgeoning
      immunities:
        damage_types:
        - poison
        conditions:
        - exhaustion
        - poisoned
    senses:
      passive_perception: 9
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Shortbow
      quantity: 1
    - item_name: Shortsword
      quantity: 1
    communication:
      speaks: []
      understands:
      - common
      additional_languages:
        count: 1
        chooser: gm
        distinct_from_known: true
        can_speak: false
        can_understand: true
      can_speak: false
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
- id: dnd2024:creature_skeleton
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
  events: []
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
  source_sha256: a72ca49788c4c3545cbb7765ec850c583e532178d2b99198a0d4643a613d9243
  source_path: builtin/srd-5.2.1/creature/skeleton.md
  pages:
  - 325
  - 326
  engine_gaps: &id001
  - dnd2024:creature_skeleton
  conversion_gaps: []
  execution_profiles:
  - dnd2024:creatures/skeleton
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 3f934e83698ca981bae4bce5446a41642e5f13ca5e2e5c05b12797b82bc33eeb
splat_clauses:
- key: source_rules
  text: '# Skeleton Medium Undead, Lawful Evil AC 14 Initiative +3 (13) HP 13 (2d8 + 4) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 16 +3 +3 Con 15 +2 +2 Int 6 -2 -2 Wis 8 -1 -1 Cha 5 -3 -3
    Vulnerabilities Bludgeoning Immunities Poison; Exhaustion, Poisoned Gear Shortbow, Shortsword Senses
    Darkvision 60 ft.; Passive Perception 9 Languages Understands Common plus one other language but can’t
    speak CR 1/4 (XP 50; PB +2) ### Actions Shortsword. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6
    + 3) Piercing damage. Shortbow. Ranged Attack Roll: +5, range 80/320 ft. Hit: 6 (1d6 + 3) Piercing
    damage.'
  sha256: 69cdda2d2e2e765a7ca7df9b1c5174ee06767304b6e515fe7734cfbc37b3f953
  status: contract_specified
splat_requires_mechanics: *id001
---

# Skeleton

<!-- SRD page 325 -->

Medium Undead, Lawful Evil\
AC 14  Initiative +3 (13)\
HP 13 (2d8 + 4)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 16 +3 +3 Con 15 +2 +2\
Int 6 −2 −2 Wis 8 −1 −1 Cha 5 −3 −3\
Vulnerabilities Bludgeoning\
Immunities Poison; Exhaustion, Poisoned\
Gear Shortbow, Shortsword\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Understands Common plus one other language\
but can’t speak\
CR 1/4 (XP 50; PB +2)\

<!-- SRD page 326 -->


### Actions

Shortsword. Melee Attack Roll: +5, reach 5 ft. Hit: 6\
(1d6 + 3) Piercing damage.\
Shortbow. Ranged Attack Roll: +5, range 80/320 ft. Hit:\
6 (1d6 + 3) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/325|page 325]], [[../../srd-5.2.1/pages/326|page 326]].
