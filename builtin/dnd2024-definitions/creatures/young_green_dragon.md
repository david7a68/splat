---
splat_schema: 1
splat_id: 55036dfe-5c31-5350-a624-942d197a4b11
splat_kind: creature
splat_key: dnd2024:creatures/young_green_dragon
splat_name: Young Green Dragon
splat_origin:
  key: dnd2024:reference/creature/young_green_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 293, 294
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Deception +5, Perception +7, Stealth +4
  immunities_text: Poison; Poisoned
  senses_text: Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17
  languages_text: Common, Draconic
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 136 (16d10 + 48)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 12
  save_dexterity:
    value: 4
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 13
  save_wisdom:
    value: 4
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 18
  max_hp:
    value: 136
  initiative:
    value: 4
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
  mechanic: dnd2024:creatures/young_green_dragon
  implementation: dnd2024:creature_young_green_dragon
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
          average: 11
          amount:
            dice:
              count: 2
              sides: 6
            add: 4
          damage_type: slashing
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 10
      poison_breath:
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
        dc: 14
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: damage
          average: 42
          amount:
            dice:
              count: 12
              sides: 6
            add: 0
          damage_type: poison
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
      deception: 5
      perception: 7
      stealth: 4
    defenses:
      immunities:
        damage_types:
        - poison
        conditions:
        - poisoned
    senses:
      passive_perception: 17
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
- id: dnd2024:creature_young_green_dragon
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
  source_sha256: 6e35d3e94c7cd6704c5c5c1be97153c7907c57f382f219773b7a2f82e0eb0c18
  source_path: builtin/srd-5.2.1/creature/young_green_dragon.md
  pages:
  - 293
  - 294
  engine_gaps: &id001
  - dnd2024:creature_young_green_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 57424097495ab6a1a93391ce65fa3c7ef58c2b93c9cd3a48939d6b621f69474a
splat_clauses:
- key: source_rules
  text: '# Young Green Dragon Large Dragon (Chromatic), Lawful Evil AC 18 Initiative +4 (14) HP 136 (16d10
    + 48) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 12 +1 +4 Con
    17 +3 +3 Int 16 +3 +3 Wis 13 +1 +4 Cha 15 +2 +2 Skills Deception +5, Perception +7, Stealth +4 Immunities
    Poison; Poisoned Senses Blindsight 30 ft., Darkvision 120 ft.; Passive Perception 17 Languages Common,
    Draconic CR 8 (XP 3,900; PB +3) ### Traits Amphibious. The dragon can breathe air and water. ### Actions
    Multiattack. The dragon makes three Rend attacks. Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 11
    (2d6 + 4) Slashing damage plus 7 (2d6) Poison damage. Poison Breath (Recharge 5–6). Constitution Saving
    Throw: DC 14, each creature in a 30-foot Cone. Failure: 42 (12d6) Poison damage. Success: Half damage.'
  sha256: 8fe9300bb43ccb9a458e0cbe163d8839239977f51b09b98b683081e3121058d3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Young Green Dragon

<!-- SRD page 293 -->

Large Dragon (Chromatic), Lawful Evil\
AC 18 Initiative +4 (14)\
HP 136 (16d10 + 48)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 12 +1 +4 Con 17 +3 +3\
Int 16 +3 +3 Wis 13 +1 +4 Cha 15 +2 +2\
Skills Deception +5, Perception +7, Stealth +4\
Immunities Poison; Poisoned\
Senses Blindsight 30 ft., Darkvision 120 ft.;\
Passive Perception 17\
Languages Common, Draconic\
CR 8 (XP 3,900; PB +3)\

### Traits

Amphibious. The dragon can breathe air and water.\

<!-- SRD page 294 -->


### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6\
+ 4) Slashing damage plus 7 (2d6) Poison damage.\
Poison Breath (Recharge 5–6). Constitution Saving\
Throw: DC 14, each creature in a 30-foot Cone. Failure:\
42 (12d6) Poison damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/293|page 293]], [[../../srd-5.2.1/pages/294|page 294]].
