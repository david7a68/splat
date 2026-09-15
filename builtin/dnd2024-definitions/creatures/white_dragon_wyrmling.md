---
splat_schema: 1
splat_id: dd46792d-7e1b-558d-806e-a0264cb53911
splat_kind: creature
splat_key: dnd2024:creatures/white_dragon_wyrmling
splat_name: White Dragon Wyrmling
splat_origin:
  key: dnd2024:reference/creature/white_dragon_wyrmling
splat_sources:
- title: SRD 5.2.1
  locator: pp. 339, 340
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 30 ft., Burrow 15 ft., Fly 60 ft., Swim 30 ft.
  skills_text: Perception +4, Stealth +2
  immunities_text: Cold
  senses_text: Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14
  languages_text: Draconic
  cr_text: 2 (450 XP; PB +2)
  hit_points_text: 32 (5d8 + 10)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 10
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 2
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 16
  max_hp:
    value: 32
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 15
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
  mechanic: dnd2024:creatures/white_dragon_wyrmling
  implementation: dnd2024:creature_white_dragon_wyrmling
  parameters:
    features:
      ice_walk:
        economy: passive
        movement_checks_waived:
          surfaces:
          - ice
          modes:
          - walk
          - climb
        difficult_terrain_extra_cost_ignored:
        - ice
        - snow
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 8
            add: 2
          damage_type: slashing
        - op: damage
          average: 2
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          damage_type: cold
        reach_feet: 5
      cold_breath:
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
        ability: constitution
        dc: 12
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 15
            origin: self
        failure:
        - op: damage
          average: 22
          amount:
            dice:
              count: 5
              sides: 8
            add: 0
          damage_type: cold
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
    skill_totals:
      perception: 4
      stealth: 2
    defenses:
      immunities:
        damage_types:
        - cold
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
- id: dnd2024:creature_white_dragon_wyrmling
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
  source_sha256: 020b8d2582b7e99ea4c63358979888462a5b5b2a274de8d94409235ee0961a95
  source_path: builtin/srd-5.2.1/creature/white_dragon_wyrmling.md
  pages:
  - 339
  - 340
  engine_gaps: &id001
  - dnd2024:creature_white_dragon_wyrmling
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: e217e723bd74091d5fd055cd44e72f7337102504b5c337fec13332262d685386
splat_clauses:
- key: source_rules
  text: '# White Dragon Wyrmling Medium Dragon (Chromatic), Chaotic Evil AC 16 Initiative +2 (12) HP 32
    (5d8 + 10) Speed 30 ft., Burrow 15 ft., Fly 60 ft., Swim 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 14
    +2 +2 Dex 10 +0 +2 Con 14 +2 +2 Int 5 -3 -3 Wis 10 +0 +2 Cha 11 +0 +0 Skills Perception +4, Stealth
    +2 Immunities Cold Senses Blindsight 10 ft., Darkvision 60 ft.; Passive Perception 14 Languages Draconic
    CR 2 (450 XP; PB +2) ### Traits Ice Walk. The dragon can move across and climb icy surfaces without
    needing to make an ability check. Additionally, Difficult Terrain composed of ice or snow doesn’t
    cost it extra movement. ### Actions Multiattack. The dragon makes two Rend attacks. Rend. Melee Attack
    Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2) Slashing damage plus 2 (1d4) Cold damage. Cold Breath (Recharge
    5–6). Constitution Saving Throw: DC 12, each creature in a 15-foot Cone. Failure: 22 (5d8) Cold damage.
    Success: Half damage.'
  sha256: ee3d55d93a39ebbb9c748ed71a8f3e96bff337cb9ba59701b7db83eb15b74ae7
  status: contract_specified
splat_requires_mechanics: *id001
---

# White Dragon Wyrmling

<!-- SRD page 339 -->

Medium Dragon (Chromatic), Chaotic Evil\
AC 16 Initiative +2 (12)\
HP 32 (5d8 + 10)\
Speed 30 ft., Burrow 15 ft., Fly 60 ft., Swim 30 ft.\

<!-- SRD page 340 -->

MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 10 +0 +2 Con 14 +2 +2\
Int 5 −3 −3 Wis 10 +0 +2 Cha 11 +0 +0\
Skills Perception +4, Stealth +2\
Immunities Cold\
Senses Blindsight 10 ft., Darkvision 60 ft.;\
Passive Perception 14\
Languages Draconic\
CR 2 (450 XP; PB +2)\

### Traits

Ice Walk. The dragon can move across and climb icy\
surfaces without needing to make an ability check.\
Additionally, Difficult Terrain composed of ice or snow\
doesn’t cost it extra movement.\

### Actions

Multiattack. The dragon makes two Rend attacks.\
Rend. Melee Attack Roll: +4, reach 5 ft. Hit: 6 (1d8 + 2)\
Slashing damage plus 2 (1d4) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 12, each creature in a 15-foot Cone. Failure:\
22 (5d8) Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/339|page 339]], [[../../srd-5.2.1/pages/340|page 340]].
