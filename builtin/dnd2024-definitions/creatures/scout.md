---
splat_schema: 1
splat_id: 05dd58d8-2e0e-5ff9-81e5-ee246a24acf4
splat_kind: creature
splat_key: dnd2024:creatures/scout
splat_name: Scout
splat_origin:
  key: dnd2024:reference/creature/scout
splat_sources:
- title: SRD 5.2.1
  locator: pp. 322
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  skills_text: Nature +4, Perception +5, Stealth +6, Survival +5
  gear_text: Leather Armor, Longbow, Shortsword
  senses_text: Passive Perception 15
  languages_text: Common plus one other language
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 16 (3d8 + 3)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 16
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/scout
  implementation: dnd2024:creature_scout
  parameters:
    skill_totals:
      nature: 4
      perception: 5
      stealth: 6
      survival: 5
    gear:
    - item: leather_armor
      count: 1
    - item: longbow
      count: 1
    - item: shortsword
      count: 1
    senses: []
    passive_perception: 15
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 1
    features:
      shortsword:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
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
                count: 1
                sides: 6
              add: 2
            average: 5
      longbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 150
          long: 600
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 2
            average: 6
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 2
          choices:
          - shortsword
          - longbow
          combination: any
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
- id: dnd2024:creature_scout
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
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 173c396bb136620798c87aedee0a0184287b084e50be443b043bfdb4cf0ed9a4
  source_path: builtin/srd-5.2.1/creature/scout.md
  pages:
  - 322
  engine_gaps: &id001
  - dnd2024:creature_scout
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 925029aeefaac104b3cba98c89e09facf7ee61e5e6df3a0104193f220b010043
splat_clauses:
- key: source_rules
  text: '# Scout Medium or Small Humanoid, Neutral AC 13 Initiative +2 (12) HP 16 (3d8 + 3) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 11 +0 +0 Dex 14 +2 +2 Con 12 +1 +1 Int 11 +0 +0 Wis 13 +1 +1 Cha 11
    +0 +0 Skills Nature +4, Perception +5, Stealth +6, Survival +5 Gear Leather Armor, Longbow, Shortsword
    Senses Passive Perception 15 Languages Common plus one other language CR 1/2 (XP 100; PB +2) ### Actions
    Multiattack. The scout makes two attacks, using Shortsword and Longbow in any combination. Shortsword.
    Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage. Longbow. Ranged Attack Roll:
    +4, range 150/600 ft. Hit: 6 (1d8 + 2) Piercing damage.'
  sha256: aabd13ed7af54246e21a15e3c4ba71b9cd2cbcfe42a47c3b7226a78d08ecbfcf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scout

<!-- SRD page 322 -->

Medium or Small Humanoid, Neutral\
AC 13  Initiative +2 (12)\
HP 16 (3d8 + 3)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 14 +2 +2 Con 12 +1 +1\
Int 11 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Nature +4, Perception +5, Stealth +6, Survival +5\
Gear Leather Armor, Longbow, Shortsword\
Senses Passive Perception 15\
Languages Common plus one other language\
CR 1/2 (XP 100; PB +2)\

### Actions

Multiattack. The scout makes two attacks, using Shorts-\
word and Longbow in any combination.\
Shortsword. Melee Attack Roll: +4, reach 5 ft. Hit: 5\
(1d6 + 2) Piercing damage.\
Longbow. Ranged Attack Roll: +4, range 150/600 ft.\
Hit: 6 (1d8 + 2) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/322|page 322]].
