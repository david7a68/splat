---
splat_schema: 1
splat_id: 86bad936-8264-59f8-8911-7f25a067cb15
splat_kind: creature
splat_key: dnd2024:creatures/ancient_red_dragon
splat_name: Ancient Red Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_red_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 319
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Climb 40 ft., Fly 80 ft.
  skills_text: Perception +16, Stealth +7
  immunities_text: Fire
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26
  languages_text: Common, Draconic
  cr_text: 24 (XP 62,000, or 75,000 in lair; PB +7)
  hit_points_text: 507 (26d20 + 234)
splat_stats:
  strength:
    value: 30
  save_strength:
    value: 10
  dexterity:
    value: 10
  save_dexterity:
    value: 7
  constitution:
    value: 29
  save_constitution:
    value: 9
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 15
  save_wisdom:
    value: 9
  charisma:
    value: 27
  save_charisma:
    value: 8
  armor_class:
    value: 22
  max_hp:
    value: 507
  initiative:
    value: 14
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
  mechanic: dnd2024:creatures/ancient_red_dragon
  implementation: dnd2024:creature_ancient_red_dragon
  parameters:
    skill_totals:
      perception: 16
      stealth: 7
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 60
    - sense: darkvision
      range_ft: 120
    passive_perception: 26
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
        uses_in_lair: 5
        uses:
          count: 4
          recovery: long_rest
      rend:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 17
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 15
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 8
              add: 10
            average: 19
          - type: fire
            amount:
              dice:
                count: 3
                sides: 6
              add: 0
            average: 10
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
            cast_level: 3
          - spell:
              key: dnd2024:spells/fireball
            uses:
              count: 1
              recovery: long_rest
            cast_level: 6
          - spell:
              key: dnd2024:spells/scrying
            uses:
              count: 1
              recovery: long_rest
          save_dc: 23
          attack_bonus: 15
      fire_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 24
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 90
        failure:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 26
                sides: 6
              add: 0
            average: 91
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
          cast_level: 3
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
            cast_level: 3
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
- id: dnd2024:creature_ancient_red_dragon
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
  source_sha256: 9a084e9283b29b8cc84318f2ae32c6301b0ca6e1f474e459b95f618cf396d086
  source_path: builtin/srd-5.2.1/creature/ancient_red_dragon.md
  pages:
  - 319
  engine_gaps: &id001
  - dnd2024:creature_ancient_red_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 4c4a0a16b06865524943d61f905afe030aa8cba275b9cfa85ad61cc8741c9b28
splat_clauses:
- key: source_rules
  text: '# Ancient Red Dragon Gargantuan Dragon (Chromatic), Chaotic Evil AC 22 Initiative +14 (24) HP
    507 (26d20 + 234) Speed 40 ft., Climb 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 30 +10 +10
    Dex 10 +0 +7 Con 29 +9 +9 Int 18 +4 +4 Wis 15 +2 +9 Cha 27 +8 +8 Skills Perception +16, Stealth +7
    Immunities Fire Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26 Languages Common,
    Draconic CR 24 (XP 62,000, or 75,000 in lair; PB +7) ### Traits Legendary Resistance (4/Day, or 5/Day
    in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack.
    The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast
    Scorching Ray (level 3 version). Rend. Melee Attack Roll: +17, reach 15 ft. Hit: 19 (2d8 + 10) Slashing
    damage plus 10 (3d6) Fire damage. Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 24, each
    creature in a 90-foot Cone. Failure: 91 (26d6) Fire damage. Success: Half damage. Spellcasting. The
    dragon casts one of the following spells, requiring no Material components and using Charisma as the
    spellcasting ability (spell save DC 23, +15 to hit with spell attacks): At Will: Command (level 2
    version), Detect Magic, Scorching Ray (level 3 version) 1/Day Each: Fireball (level 6 version), Scrying
    ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn,
    the dragon can expend a use to take one of the following actions. The dragon regains all expended
    uses at the start of each of its turns. Commanding Presence. The dragon uses Spellcasting to cast
    Command (level 2 version). The dragon can’t take this action again until the start of its next turn.
    Fiery Rays. The dragon uses Spellcasting to cast Scorching Ray (level 3 version). The dragon can’t
    take this action again until the start of its next turn. Pounce. The dragon moves up to half its Speed,
    and it makes one Rend attack.'
  sha256: 0f6e7d161e4bbef81002ed83c312c5b4a2334acb41be1fd63469ef715276ac4c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Red Dragon

<!-- SRD page 319 -->

Gargantuan Dragon (Chromatic), Chaotic Evil\
AC 22 Initiative +14 (24)\
HP 507 (26d20 + 234)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +10 Dex 10 +0 +7 Con 29 +9 +9\
Int 18 +4 +4 Wis 15 +2 +9 Cha 27 +8 +8\
Skills Perception +16, Stealth +7\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 26\
Languages Common, Draconic\
CR 24 (XP 62,000, or 75,000 in lair; PB +7)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Scorching Ray (level 3 version).\
Rend. Melee Attack Roll: +17, reach 15 ft. Hit: 19 (2d8\
+ 10) Slashing damage plus 10 (3d6) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 24, each creature in a 90-foot Cone. Failure: 91\
(26d6) Fire damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 23,\
+15 to hit with spell attacks):\
At Will: Command (level 2 version), Detect Magic,\
Scorching Ray (level 3 version)\
1/Day Each: Fireball (level 6 version), Scrying\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Commanding Presence. The dragon uses Spellcasting\
to cast Command (level 2 version). The dragon can’t\
take this action again until the start of its next turn.\
Fiery Rays. The dragon uses Spellcasting to cast Scorch-\
ing Ray (level 3 version). The dragon can’t take this ac-\
tion again until the start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/319|page 319]].
