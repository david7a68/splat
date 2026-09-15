---
splat_schema: 1
splat_id: 064a002d-1b22-5c6d-aeef-eae338efbded
splat_kind: creature
splat_key: dnd2024:creatures/adult_red_dragon
splat_name: Adult Red Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_red_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 318, 319
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Climb 40 ft., Fly 80 ft.
  skills_text: Perception +13, Stealth +6
  immunities_text: Fire
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 23
  languages_text: Common, Draconic
  cr_text: 17 (XP 18,000, or 20,000 in lair; PB +6)
  hit_points_text: 256 (19d12 + 133)
splat_stats:
  strength:
    value: 27
  save_strength:
    value: 8
  dexterity:
    value: 10
  save_dexterity:
    value: 6
  constitution:
    value: 25
  save_constitution:
    value: 7
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 13
  save_wisdom:
    value: 7
  charisma:
    value: 23
  save_charisma:
    value: 6
  armor_class:
    value: 19
  max_hp:
    value: 256
  initiative:
    value: 12
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/adult_red_dragon
  implementation: dnd2024:creature_adult_red_dragon
  parameters:
    skill_totals:
      perception: 13
      stealth: 6
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 60
    - sense: darkvision
      range_ft: 120
    passive_perception: 23
    languages:
      speak: true
      known:
      - common
      - draconic
    features:
      legendary_resistance:
        economy: passive
        trigger: saving_throw.failed
        optional: true
        op: saving_throw.set_result
        result: success
        uses_in_lair: 4
        uses:
          count: 3
          recovery: long_rest
      rend:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 14
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 1
                sides: 10
              add: 8
            average: 13
          - type: fire
            amount:
              dice:
                count: 2
                sides: 4
              add: 0
            average: 5
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: charisma
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/command
            uses:
              unlimited: true
            cast_level: 2
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/scorching_ray
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/fireball
            uses:
              count: 1
              recovery: long_rest
          save_dc: 20
          attack_bonus: 12
      fire_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 21
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 60
        failure:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 17
                sides: 6
              add: 0
            average: 59
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
      commanding_presence:
        economy: legendary_action
        op: sequence
        steps:
        - op: spell.cast
          spell:
            key: dnd2024:spells/command
          profile: spellcasting
          cast_level: 2
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      fiery_rays:
        economy: legendary_action
        op: sequence
        steps:
        - op: spell.cast
          spell:
            key: dnd2024:spells/scorching_ray
          profile: spellcasting
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      pounce:
        economy: legendary_action
        op: sequence
        steps:
        - op: movement
          maximum:
            multiply:
            - 0.5
            - actor_stat: speed
        - op: feature.use
          feature: rend
        legendary_cost: 1
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 3
          feature: rend
        replace_attack:
          maximum: 1
          choices:
          - op: spell.cast
            spell:
              key: dnd2024:spells/scorching_ray
            profile: spellcasting
    legendary_actions:
      uses: 3
      uses_in_lair: 4
      trigger: another_creature.turn_end_immediately_after
      cost_per_action: 1
      recover_all: actor.turn_start
      maximum_per_trigger: 1
      requires_actions_available: true
      excluded_conditions:
      - incapacitated
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
- id: dnd2024:creature_adult_red_dragon
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
  - another_creature.turn_end_immediately_after
  - creature.feature_requested
  - saving_throw.failed
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
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 42eb4ca527a05334f3b9b84750dd9e0d90cc81d43889d656ff722bca66e1039c
  source_path: builtin/srd-5.2.1/creature/adult_red_dragon.md
  pages:
  - 318
  - 319
  engine_gaps: &id001
  - dnd2024:creature_adult_red_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 80bb583d5907116512afcac5ad88d29e7af4ad3824c9b58cfc47a3fc7444eddc
splat_clauses:
- key: source_rules
  text: '# Adult Red Dragon Huge Dragon (Chromatic), Chaotic Evil AC 19 Initiative +12 (22) HP 256 (19d12
    + 133) Speed 40 ft., Climb 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8 Dex 10 +0 +6
    Con 25 +7 +7 Int 16 +3 +3 Wis 13 +1 +7 Cha 23 +6 +6 Skills Perception +13, Stealth +6 Immunities Fire
    Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 23 Languages Common, Draconic CR
    17 (XP 18,000, or 20,000 in lair; PB +6) ### Traits Legendary Resistance (3/Day, or 4/Day in Lair).
    If the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack. The
    dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Scorching
    Ray. Rend. Melee Attack Roll: +14, reach 10 ft. Hit: 13 (1d10 + 8) Slashing damage plus 5 (2d4) Fire
    damage. Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 21, each creature in a 60-foot Cone.
    Failure: 59 (17d6) Fire damage. Success: Half damage. Spellcasting. The dragon casts one of the following
    spells, requiring no Material components and using Charisma as the spellcasting ability (spell save
    DC 20, +12 to hit with spell attacks): At Will: Command (level 2 version), Detect Magic, Scorching
    Ray 1/Day: Fireball ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after
    another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon
    regains all expended uses at the start of each of its turns. Commanding Presence. The dragon uses
    Spellcasting to cast Command (level 2 version). The dragon can’t take this action again until the
    start of its next turn. Fiery Rays. The dragon uses Spellcasting to cast Scorching Ray. The dragon
    can’t take this action again until the start of its next turn. Pounce. The dragon moves up to half
    its Speed, and it makes one Rend attack.'
  sha256: 661859a5b49729007ddfe480241a4e352eeb0cf765f6318c7c31d3aa28339096
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Red Dragon

<!-- SRD page 318 -->

Huge Dragon (Chromatic), Chaotic Evil\
AC 19 Initiative +12 (22)\
HP 256 (19d12 + 133)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 10 +0 +6 Con 25 +7 +7\
Int 16 +3 +3 Wis 13 +1 +7 Cha 23 +6 +6\
Skills Perception +13, Stealth +6\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 23\
Languages Common, Draconic\
CR 17 (XP 18,000, or 20,000 in lair; PB +6)\

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Scorching Ray.\
Rend. Melee Attack Roll: +14, reach 10 ft. Hit: 13 (1d10\
+ 8) Slashing damage plus 5 (2d4) Fire damage.\

<!-- SRD page 319 -->

Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 21, each creature in a 60-foot Cone. Failure: 59\
(17d6) Fire damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 20,\
+12 to hit with spell attacks):\
At Will: Command (level 2 version), Detect Magic,\
Scorching Ray\
1/Day: Fireball\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Commanding Presence. The dragon uses Spellcasting\
to cast Command (level 2 version). The dragon can’t\
take this action again until the start of its next turn.\
Fiery Rays. The dragon uses Spellcasting to cast Scorch-\
ing Ray. The dragon can’t take this action again until the\
start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/318|page 318]], [[../../srd-5.2.1/pages/319|page 319]].
