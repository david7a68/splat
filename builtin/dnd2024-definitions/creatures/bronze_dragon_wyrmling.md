---
splat_schema: 1
splat_id: 4a242baf-74a2-51bb-a6e6-728d5d8ff0e1
splat_kind: creature
splat_key: dnd2024:creatures/bronze_dragon_wyrmling
splat_name: Bronze Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/bronze_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 269, 270
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 30 ft., Fly 60 ft., Swim 30 ft.
  skills_text: Perception +4, Stealth +2
  immunities_text: Lightning
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 39 (6d8 + 12)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 10
  save_dexterity:
    value: 2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 15
  max_hp:
    value: 39
  initiative:
    value: 2
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
  mechanic: dnd2024:creatures/bronze_dragon_wyrmling
  implementation: dnd2024:creature_bronze_dragon_wyrmling
  parameters:
    features:
      amphibious:
        economy: passive
        breathing_media:
        - air
        - water
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 10
            add: 3
          damage_type: slashing
        reach_feet: 5
      lightning_breath:
        economy: action
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
        ability: dexterity
        dc: 12
        targets:
          kind: creature
          count: all
          area:
            shape: line
            size_feet: 40
            origin: self
            width_feet: 5
        failure:
        - op: damage
          average: 16
          amount:
            dice:
              count: 3
              sides: 10
            add: 0
          damage_type: lightning
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count: 2
      repulsion_breath:
        economy: action
        op: save
        ability: strength
        dc: 12
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: push
          distance_max_feet: 30
          direction: straight_away_from_self
        - op: condition
          condition: prone
        success: []
    skill_totals:
      perception: 4
      stealth: 2
    defenses:
      immunities:
        damage_types:
        - lightning
    senses:
      passive_perception: 14
      special:
      - sense: blindsight
        range_feet: 10
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - draconic
      understands:
      - draconic
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
- id: dnd2024:creature_bronze_dragon_wyrmling
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
  source_sha256: f2405db05d6f273886c24278aee5882e9925624a7fb528fd9320003ce28d3b71
  source_path: builtin/srd-5.2.1/creature/bronze_dragon_wyrmling.md
  pages:
  - 269
  - 270
  engine_gaps: &id001
  - dnd2024:creature_bronze_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: f914d1a1f946ebc851ed8ef3412bfc70ac3c0acceaf3cd5c9fa6f79388e7f062
splat_clauses:
- key: source_rules
  text: '# Bronze Dragon Wyrmling Medium Dragon (Metallic), Lawful Good AC 15 Initiative +2 (12) HP 39
    (6d8 + 12) Speed 30 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 10 +0
    +2 Con 15 +2 +2 Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2 Skills Perception +4, Stealth +2 Immunities
    Lightning Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic CR
    2 (XP 450; PB +2) ### Traits Amphibious. The dragon can breathe air and water. ### Actions Multiattack.
    The dragon makes two Rend attacks. Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Slashing
    damage. Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 12, each creature in a 40-foot-long,
    5-footwide Line. Failure: 16 (3d10) Lightning damage. Success: Half damage. Repulsion Breath. Strength
    Saving Throw: DC 12, each creature in a 30-foot Cone. Failure: The target is pushed up to 30 feet
    straight away from the dragon and has the Prone condition.'
  sha256: 8e7375a35f895c0e9c1b6f164a938372d440943b3dea9f12817bd07ca9df9859
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bronze Dragon Wyrmling

<!-- SRD page 269 -->

Medium Dragon (Metallic), Lawful Good\
AC 15 Initiative +2 (12)\
HP 39 (6d8 + 12)\
Speed 30 ft., Fly 60 ft., Swim 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 10 +0 +2 Con 15 +2 +2\
Int 12 +1 +1 Wis 11 +0 +2 Cha 15 +2 +2\
Skills Perception +4, Stealth +2\
Immunities Lightning\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 2 (XP 450; PB +2)\

<!-- SRD page 270 -->


### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10\
+ 3) Slashing damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 12, each creature in a 40-foot-long, 5-foot-\
wide Line. Failure: 16 (3d10) Lightning damage. Suc-\
cess: Half damage.\
Repulsion Breath. Strength Saving Throw: DC 12, each\
creature in a 30-foot Cone. Failure: The target is pushed\
up to 30 feet straight away from the dragon and has the\
Prone condition.\

Original layout: [[../../srd-5.2.1/pages/269|page 269]], [[../../srd-5.2.1/pages/270|page 270]].
