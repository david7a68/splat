---
splat_schema: 1
splat_id: 4a526619-386c-5f7a-b204-951f64027eef
splat_kind: creature
splat_key: dnd2024:creatures/young_black_dragon
splat_name: Young Black Dragon
splat_origin:
  key: dnd2024:reference/creature/young_black_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 264
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Perception +6, Stealth +5
  immunities_text: Acid
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16
  languages_text: Common, Draconic
  cr_text: 7 (XP 2,900; PB +3)
  hit_points_text: 127 (15d10 + 45)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 5
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 11
  save_wisdom:
    value: 3
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 18
  max_hp:
    value: 127
  initiative:
    value: 5
  speed:
    value: 40
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
  mechanic: dnd2024:creatures/young_black_dragon
  implementation: dnd2024:creature_young_black_dragon
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
          average: 3
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          damage_type: acid
        reach_feet: 10
      acid_breath:
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
        dc: 14
        targets:
          kind: creature
          count: all
          area:
            shape: line
            size_feet: 30
            origin: self
            width_feet: 5
        failure:
        - op: damage
          average: 49
          amount:
            dice:
              count: 14
              sides: 6
            add: 0
          damage_type: acid
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
      stealth: 5
    defenses:
      immunities:
        damage_types:
        - acid
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
- id: dnd2024:creature_young_black_dragon
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
  source_sha256: 4fb34795875ca29084a6dbabb33df11020a0195685f91a488804c50bc00f5791
  source_path: builtin/srd-5.2.1/creature/young_black_dragon.md
  pages:
  - 264
  engine_gaps: &id001
  - dnd2024:creature_young_black_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: f78280b2dbc28c8ad4605d020db8b7c8b9590a85ab6665d1ab25007df4267448
splat_clauses:
- key: source_rules
  text: '# Young Black Dragon Large Dragon (Chromatic), Chaotic Evil AC 18 Initiative +5 (15) HP 127 (15d10
    + 45) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 14 +2 +5 Con
    17 +3 +3 Int 12 +1 +1 Wis 11 +0 +3 Cha 15 +2 +2 Skills Perception +6, Stealth +5 Immunities Acid Senses
    Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 16 Languages Common, Draconic CR 7 (XP 2,900;
    PB +3) ### Traits Amphibious. The dragon can breathe air and water. ### Actions Multiattack. The dragon
    makes three Rend attacks. Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 9 (2d4 + 4) Slashing damage
    plus 3 (1d6) Acid damage. Acid Breath (Recharge 5–6). Dexterity Saving Throw: DC 14, each creature
    in a 30-foot-long, 5-footwide Line. Failure: 49 (14d6) Acid damage. Success: Half damage.'
  sha256: d60b50552a04988970b5538c4d9312e86e4b88a3c65d91214131959f0e7d9a27
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Black Dragon

<!-- SRD page 264 -->

Large Dragon (Chromatic), Chaotic Evil\
AC 18 Initiative +5 (15)\
HP 127 (15d10 + 45)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 14 +2 +5 Con 17 +3 +3\
Int 12 +1 +1 Wis 11 +0 +3 Cha 15 +2 +2\
Skills Perception +6, Stealth +5\
Immunities Acid\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 16\
Languages Common, Draconic\
CR 7 (XP 2,900; PB +3)\

### Traits

Amphibious. The dragon can breathe air and water.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 9 (2d4\
+ 4) Slashing damage plus 3 (1d6) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 14, each creature in a 30-foot-long, 5-foot-\
wide Line. Failure: 49 (14d6) Acid damage. Success:\
Half damage.\

Original layout: [[../../srd-5.2.1/pages/264|page 264]].
