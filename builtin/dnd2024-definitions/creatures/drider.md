---
splat_schema: 1
splat_id: db69f9e6-632e-5775-b339-3f413b618d47
splat_kind: creature
splat_key: dnd2024:creatures/drider
splat_name: Drider
splat_origin:
  key: dnd2024:reference/creature/drider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 281
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Chaotic Evil
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +6, Stealth +10
  senses_text: Darkvision 120 ft.; Passive Perception 16
  languages_text: Elvish, Undercommon
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 123 (13d10 + 52)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 19
  save_dexterity:
    value: 4
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 13
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 19
  max_hp:
    value: 123
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/drider
  implementation: dnd2024:creature_drider
  parameters:
    skill_totals:
      perception: 6
      stealth: 10
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 16
    languages:
      speak: true
      known:
      - elvish
      - undercommon
    features:
      spider_climb:
        economy: passive
        op: movement.climb
        difficult_surfaces: true
        ceilings: true
        ability_check_required: false
      sunlight_sensitivity:
        economy: passive
        op: roll.disadvantage
        when:
          in_sunlight: true
        rolls:
        - ability_check
        - attack
      web_walker:
        economy: passive
        ignore_movement_restrictions_from:
        - web
        locate_creatures_in_same_web: true
      foreleg:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 7
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 8
              add: 4
            average: 13
      poison_burst:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 120
          long: 120
        hit:
        - op: damage
          components:
          - type: poison
            amount:
              dice:
                count: 3
                sides: 6
              add: 3
            average: 13
      magic_of_the_spider_queen:
        economy: bonus_action
        op: spellcasting
        profile:
          ability: wisdom
          waived_components:
          - material
          save_dc: 14
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/darkness
        - op: spell.cast
          spell:
            key: dnd2024:spells/faerie_fire
        - op: spell.cast
          spell:
            key: dnd2024:spells/web
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
        attacks:
          count: 3
          choices:
          - foreleg
          - poison_burst
          combination: any
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
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
- id: dnd2024:creature_drider
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
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: f6770365ce6adb2c7d33ca031a9791005fa07fd163005a0611230c97b2a07858
  source_path: builtin/srd-5.2.1/creature/drider.md
  pages:
  - 281
  engine_gaps: &id001
  - dnd2024:creature_drider
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 7f106696ad1ddf8e2d283d87adeb608b7c3188e9fbe3e46e6a9ca03bb20a5f06
splat_clauses:
- key: source_rules
  text: '# Drider Large Monstrosity, Chaotic Evil AC 19 Initiative +4 (14) HP 123 (13d10 + 52) Speed 30
    ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 19 +4 +4 Con 18 +4 +4 Int 13 +1 +1 Wis
    16 +3 +3 Cha 12 +1 +1 Skills Perception +6, Stealth +10 Senses Darkvision 120 ft.; Passive Perception
    16 Languages Elvish, Undercommon CR 6 (XP 2,300; PB +3) ### Traits Spider Climb. The drider can climb
    difficult surfaces, including along ceilings, without needing to make an ability check. Sunlight Sensitivity.
    While in sunlight, the drider has Disadvantage on ability checks and attack rolls. Web Walker. The
    drider ignores movement restrictions caused by webs, and the drider knows the location of any other
    creature in contact with the same web. ### Actions Multiattack. The drider makes three attacks, using
    Foreleg or Poison Burst in any combination. Foreleg. Melee Attack Roll: +7, reach 10 ft. Hit: 13 (2d8
    + 4) Piercing damage. Poison Burst. Ranged Attack Roll: +6, range 120 ft. Hit: 13 (3d6 + 3) Poison
    damage. ### Bonus Actions Magic of the Spider Queen (Recharge 5–6). The drider casts Darkness, Faerie
    Fire, or Web, requiring no Material components and using Wisdom as the spellcasting ability (spell
    save DC 14).'
  sha256: e17cbd3481f1bf8f53683905b702194db482a187787d35871edfd2df7076f3d4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Drider

<!-- SRD page 281 -->

Large Monstrosity, Chaotic Evil\
AC 19 Initiative +4 (14)\
HP 123 (13d10 + 52)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 19 +4 +4 Con 18 +4 +4\
Int 13 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1\
Skills Perception +6, Stealth +10\
Senses Darkvision 120 ft.; Passive Perception 16\
Languages Elvish, Undercommon\
CR 6 (XP 2,300; PB +3)\

### Traits

Spider Climb. The drider can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\
Sunlight Sensitivity. While in sunlight, the drider has\
Disadvantage on ability checks and attack rolls.\
Web Walker. The drider ignores movement restrictions\
caused by webs, and the drider knows the location of\
any other creature in contact with the same web.\

### Actions

Multiattack. The drider makes three attacks, using\
Foreleg or Poison Burst in any combination.\
Foreleg. Melee Attack Roll: +7, reach 10 ft. Hit: 13 (2d8\
+ 4) Piercing damage.\
Poison Burst. Ranged Attack Roll: +6, range 120 ft. Hit:\
13 (3d6 + 3) Poison damage.\

### Bonus Actions

Magic of the Spider Queen (Recharge 5–6). The drider\
casts Darkness, Faerie Fire, or Web, requiring no Mate-\
rial components and using Wisdom as the spellcasting\
ability (spell save DC 14).\

Original layout: [[../../srd-5.2.1/pages/281|page 281]].
