---
splat_schema: 1
splat_id: 4544c113-2676-582c-898a-9b6ad5031d8b
splat_kind: creature
splat_key: dnd2024:creatures/young_white_dragon
splat_name: Young White Dragon
splat_origin:
  key: dnd2024:reference/creature/young_white_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 340
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Perception +6, Stealth +3
  immunities_text: Cold
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16
  languages_text: Common, Draconic
  cr_text: 6 (2,300 XP; PB +3)
  hit_points_text: 123 (13d10 + 52)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 3
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 6
  save_intelligence:
    value: 2
  wisdom:
    value: 11
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 17
  max_hp:
    value: 123
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 20
    unit: ft
  fly_speed:
    value: 80
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/young_white_dragon
  implementation: dnd2024:creature_young_white_dragon
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
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 4
            add: 4
          damage_type: slashing
        - op: damage
          average: 2
          amount:
            dice:
              count: 1
              sides: 4
            add: 0
          damage_type: cold
        reach_feet: 10
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
        dc: 15
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: damage
          average: 40
          amount:
            dice:
              count: 9
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
          count: 3
    skill_totals:
      perception: 6
      stealth: 3
    defenses:
      immunities:
        damage_types:
        - cold
    senses:
      passive_perception: 16
      special:
      - sense: blindsight
        range_feet: 30
      - sense: darkvision
        range_feet: 120
    communication:
      speaks:
      - common
      - draconic
      understands:
      - common
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
- id: dnd2024:creature_young_white_dragon
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
  source_sha256: de45456b6855dad80c7f504a26c2052291d1c034cfe78f90441dd86b2d1288ac
  source_path: builtin/srd-5.2.1/creature/young_white_dragon.md
  pages:
  - 340
  engine_gaps: &id001
  - dnd2024:creature_young_white_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 71239144f4ef8b328af8755d3098e2e7e3d1f7eb1fb6621cfd8b36eb982356f7
splat_clauses:
- key: source_rules
  text: '# Young White Dragon Large Dragon (Chromatic), Chaotic Evil AC 17 Initiative +3 (13) HP 123 (13d10
    + 52) Speed 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4
    Dex 10 +0 +3 Con 18 +4 +4 Int 6 -2 2 Wis 11 +0 +3 Cha 12 +1 +1 Skills Perception +6, Stealth +3 Immunities
    Cold Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16 Languages Common, Draconic
    CR 6 (2,300 XP; PB +3) ### Traits Ice Walk. The dragon can move across and climb icy surfaces without
    needing to make an ability check. Additionally, Difficult Terrain composed of ice or snow doesn’t
    cost it extra movement. ### Actions Multiattack. The dragon makes three Rend attacks. Rend. Melee
    Attack Roll: +7, reach 10 ft. Hit: 9 (2d4 + 4) Slashing damage plus 2 (1d4) Cold damage. Cold Breath
    (Recharge 5–6). Constitution Saving Throw: DC 15, each creature in a 30-foot Cone. Failure: 40 (9d8)
    Cold damage. Success: Half damage.'
  sha256: 949c1e21ce10adc5c7ddf91fcdf577587865d8f13ed275c4c16f6506ba1b9864
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young White Dragon

<!-- SRD page 340 -->

Large Dragon (Chromatic), Chaotic Evil\
AC 17 Initiative +3 (13)\
HP 123 (13d10 + 52)\
Speed 40 ft., Burrow 20 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 10 +0 +3 Con 18 +4 +4\
Int 6 −2 2 Wis 11 +0 +3 Cha 12 +1 +1\
Skills Perception +6, Stealth +3\
Immunities Cold\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 16\
Languages Common, Draconic\
CR 6 (2,300 XP; PB +3)\

### Traits

Ice Walk. The dragon can move across and climb icy\
surfaces without needing to make an ability check.\
Additionally, Difficult Terrain composed of ice or snow\
doesn’t cost it extra movement.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 9 (2d4\
+ 4) Slashing damage plus 2 (1d4) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 15, each creature in a 30-foot Cone. Failure:\
40 (9d8) Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/340|page 340]].
