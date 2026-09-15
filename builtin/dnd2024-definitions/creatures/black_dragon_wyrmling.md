---
splat_schema: 1
splat_id: d84fe7b2-40c2-553a-b531-dacc33a2079d
splat_kind: creature
splat_key: dnd2024:creatures/black_dragon_wyrmling
splat_name: Black Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/black_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 263, 264
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 30 ft., Fly 60 ft., Swim 30 ft.
  skills_text: Perception +4, Stealth +4
  immunities_text: Acid
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 33 (6d8 + 6)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 14
  save_dexterity:
    value: 4
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 17
  max_hp:
    value: 33
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
  swim_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/black_dragon_wyrmling
  implementation: dnd2024:creature_black_dragon_wyrmling
  parameters:
    skill_totals:
      perception: 4
      stealth: 4
    damage_immunities:
    - acid
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 10
    - sense: darkvision
      range_ft: 60
    passive_perception: 14
    languages:
      speak: true
      known:
      - draconic
    features:
      amphibious:
        economy: passive
        op: breathing.allow
        media:
        - air
        - water
      rend:
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
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 6
              add: 2
            average: 5
          - type: acid
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            average: 2
      acid_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 11
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 15
            width_ft: 5
        failure:
        - op: damage
          components:
          - type: acid
            amount:
              dice:
                count: 5
                sides: 8
              add: 0
            average: 22
        success:
        - op: damage
          from: failure_damage
          multiplier: 0.5
          round: down
        recharge:
          die_sides: 6
          success_min: 5
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: rend
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
- id: dnd2024:creature_black_dragon_wyrmling
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
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6482129d271145beffd9285c6fb981490f195274c56b8ad6cb4f564b842a2725
  source_path: builtin/srd-5.2.1/creature/black_dragon_wyrmling.md
  pages:
  - 263
  - 264
  engine_gaps: &id001
  - dnd2024:creature_black_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: ed98777a2ac53e91d5cab545353ce298a939ca140000dbc2d120735ce8a8527d
splat_clauses:
- key: source_rules
  text: '# Black Dragon Wyrmling Medium Dragon (Chromatic), Chaotic Evil AC 17 Initiative +4 (14) HP 33
    (6d8 + 6) Speed 30 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 14 +2
    +4 Con 13 +1 +1 Int 10 +0 +0 Wis 11 +0 +2 Cha 13 +1 +1 Skills Perception +4, Stealth +4 Immunities
    Acid Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR 2 (XP
    450; PB +2) ### Traits Amphibious. The dragon can breathe air and water. ### Actions Multiattack.
    The dragon makes two Rend attacks. Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Slashing
    damage plus 2 (1d4) Acid damage. Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 11, each creature
    in a 15-foot-long, 5-footwide Line. Failure: 22 (5d8) Acid damage. Success: Half damage.'
  sha256: c322da51542da287799e196cf36f799ed5f07cbfc8295fbeffd63900353e5ed2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Black Dragon Wyrmling

<!-- SRD page 263 -->

Medium Dragon (Chromatic), Chaotic Evil\
AC 17 Initiative +4 (14)\
HP 33 (6d8 + 6)\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\

<!-- SRD page 264 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 14 +2 +4 Con 13 +1 +1\
Int 10 +0 +0 Wis 11 +0 +2 Cha 13 +1 +1\
Skills Perception +4, Stealth +4\
Immunities Acid\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 2 (XP 450; PB +2)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2)\
Slashing damage plus 2 (1d4) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 11, each creature in a 15-foot-long, 5-foot-\
wide Line. Failure: 22 (5d8) Acid damage. Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/263|page 263]], [[../../srd-5.2.1/pages/264|page 264]].
