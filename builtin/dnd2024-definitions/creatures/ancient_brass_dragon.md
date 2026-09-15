---
splat_schema: 1
splat_id: b7c31d5e-b3b4-507c-8d2e-690f7d334b7d
splat_kind: creature
splat_key: dnd2024:creatures/ancient_brass_dragon
splat_name: Ancient Brass Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_brass_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 269
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 40 ft., Burrow 40 ft., Fly 80 ft.
  skills_text: History +9, Perception +14, Persuasion +12, Stealth +6
  immunities_text: Fire
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 24
  languages_text: Common, Draconic
  cr_text: 20 (XP 25,000, or 33,000 in lair; PB +6)
  hit_points_text: 332 (19d20 + 133)
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
    value: 15
  save_wisdom:
    value: 8
  charisma:
    value: 22
  save_charisma:
    value: 6
  armor_class:
    value: 20
  max_hp:
    value: 332
  initiative:
    value: 12
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ancient_brass_dragon
  implementation: dnd2024:creature_ancient_brass_dragon
  parameters:
    features:
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 4
          recovery: long_rest
          lair_uses: 5
        operation: replace_save_result
        trigger: self.save_failed
        optional: true
        result: success
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: choice
            options:
            - operation: use_feature
              feature: sleep_breath
            - operation: cast_spell
              spell:
                key: dnd2024:spells/scorching_ray
              cast_level: 3
              spellcasting_profile: spellcasting
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 14
          reach_ft: 15
        hit:
          damage:
          - average: 19
            roll:
              count: 2
              sides: 10
              modifier: 8
            type: slashing
          - average: 7
            roll:
              count: 2
              sides: 6
              modifier: 0
            type: fire
      fire_breath:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: dexterity
        dc: 21
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 90
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 58
            roll:
              count: 13
              sides: 8
              modifier: 0
            type: fire
        success:
          damage_multiplier: 0.5
          other_effects: false
      sleep_breath:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 21
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 90
            origin: self
        failure:
          conditions:
          - condition: incapacitated
            until: target.next_turn_end
          repeat_save:
            event: target.next_turn_end
            failure:
              conditions:
              - condition: unconscious
                duration_seconds: 600
              ends_on:
              - event: target.damage_taken
              - event: creature.action
                operation: wake_target
                within_ft: 5
            success: end_effect
        success:
          effects: []
      spellcasting:
        economy: action
        operation: spellcasting
        ability: charisma
        components_waived:
        - material
        save_dc: 20
        options:
        - spell:
            key: dnd2024:spells/detect_magic
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/minor_illusion
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/scorching_ray
          usage:
            kind: at_will
          cast_level: 3
        - spell:
            key: dnd2024:spells/shapechange
          usage:
            kind: at_will
          overrides:
            allowed_types:
            - beast
            - humanoid
            gain_temporary_hp: false
            maintain_requires_concentration: false
            maintain_requires_temporary_hp: false
        - spell:
            key: dnd2024:spells/speak_with_animals
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/control_weather
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/detect_thoughts
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        material_components_available: true
      blazing_light:
        economy: legendary_action
        legendary_cost: 1
        operation: cast_spell
        spell:
          key: dnd2024:spells/scorching_ray
        cast_level: 3
        spellcasting_profile: spellcasting
      pounce:
        economy: legendary_action
        legendary_cost: 1
        operation: sequence
        effects:
        - operation: move
          maximum_ft:
            multiply:
            - stat: speed
            - 0.5
        - operation: attack_sequence
          feature: rend
          count: 1
      scorching_sands:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: saving_throw
        ability: dexterity
        dc: 20
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible: true
        failure:
          damage:
          - average: 36
            roll:
              count: 8
              sides: 8
              modifier: 0
            type: fire
          speed_multiplier: 0.5
          until: target.next_turn_end
        success:
          effects: []
    skill_totals:
      history: 9
      perception: 14
      persuasion: 12
      stealth: 6
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      blindsight: 60
      darkvision: 120
      passive_perception: 24
    languages:
      speaks:
      - common
      - draconic
      understands: []
    legendary_actions:
      uses: 3
      recovery: self.turn_start
      trigger: other_creature.turn_end
      cost_per_action: 1
      lair_uses: 4
      requires:
        not_incapacitated: true
      maximum_actions_per_trigger: 1
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_ancient_brass_dragon
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - turn.ended
  - damage.resolved
  - rest.completed
  - saving_throw.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spellcasting
  - spatial_queries
  - limited_resources
  - legendary_actions
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 38a2b2152c0db20f1c8a5a90f8d30c20a62858dd128f967230921425c00a0d6f
  source_path: builtin/srd-5.2.1/creature/ancient_brass_dragon.md
  pages:
  - 269
  engine_gaps: &id001
  - dnd2024:creature_ancient_brass_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 473186ef53f952c88f21f5d301f96d0da6965d5c1519b8edfb4610e57e8fe3ea
splat_clauses:
- key: source_rules
  text: '# Ancient Brass Dragon Gargantuan Dragon (Metallic), Chaotic Good AC 20 Initiative +12 (22) HP
    332 (19d20 + 133) Speed 40 ft., Burrow 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8
    Dex 10 +0 +6 Con 25 +7 +7 Int 16 +3 +3 Wis 15 +2 +8 Cha 22 +6 +6 Skills History +9, Perception +14,
    Persuasion +12, Stealth +6 Immunities Fire Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception
    24 Languages Common, Draconic CR 20 (XP 25,000, or 33,000 in lair; PB +6) ### Traits Legendary Resistance
    (4/Day, or 5/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ###
    Actions Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of
    (A) Sleep Breath or (B) Spellcasting to cast Scorching Ray (level 3 version). Rend. Melee Attack Roll:
    +14, reach 15 ft. Hit: 19 (2d10 + 8) Slashing damage plus 7 (2d6) Fire damage. Fire Breath (Recharge
    5–6). Dexterity Saving Throw: DC 21, each creature in a 90-foot-long, 5-footwide Line. Failure: 58
    (13d8) Fire damage. Success: Half damage. Sleep Breath. Constitution Saving Throw: DC 21, each creature
    in a 90-foot Cone. Failure: The target has the Incapacitated condition until the end of its next turn,
    at which point it repeats the save. Second Failure: The target has the Unconscious condition for 10
    minutes. This effect ends for the target if it takes damage or a creature within 5 feet of it takes
    an action to wake it. Spellcasting. The dragon casts one of the following spells, requiring no Material
    components and using Charisma as the spellcasting ability (spell save DC 20): At Will: Detect Magic,
    Minor Illusion, Scorching Ray (level 3 version), Shapechange (Beast or Humanoid form only, no Temporary
    Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain
    the spell), Speak with Animals 1/Day Each: Control Weather, Detect Thoughts ### Legendary Actions
    Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend
    a use to take one of the following actions. The dragon regains all expended uses at the start of each
    of its turns. Blazing Light. The dragon uses Spellcasting to cast Scorching Ray (level 3 version).
    Pounce. The dragon moves up to half its Speed, and it makes one Rend attack. Scorching Sands. Dexterity
    Saving Throw: DC 20, one creature the dragon can see within 120 feet. Failure: 36 (8d8) Fire damage,
    and the target’s Speed is halved until the end of its next turn. Failure or Success: The dragon can’t
    take this action again until the start of its next turn.'
  sha256: e672f23308280e00e478e8bd44093f8cc3e88aa1f4bcf9c4627d9b9e259453d1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Brass Dragon

<!-- SRD page 269 -->

Gargantuan Dragon (Metallic), Chaotic Good\
AC 20  Initiative +12 (22)\
HP 332 (19d20 + 133)\
Speed 40 ft., Burrow 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 10 +0 +6 Con 25 +7 +7\
Int 16 +3 +3 Wis 15 +2 +8 Cha 22 +6 +6\
Skills History +9, Perception +14, Persuasion +12, Stealth\
+6\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 24\
Languages Common, Draconic\
CR 20 (XP 25,000, or 33,000 in lair; PB +6)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Sleep Breath or\
(B) Spellcasting to cast Scorching Ray (level 3 version).\
Rend. Melee Attack Roll: +14, reach 15 ft. Hit: 19 (2d10\
+ 8) Slashing damage plus 7 (2d6) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 21, each creature in a 90-foot-long, 5-foot-\
wide Line. Failure: 58 (13d8) Fire damage. Success:\
Half damage.\
Sleep Breath. Constitution Saving Throw: DC 21, each\
creature in a 90-foot Cone. Failure: The target has the\
Incapacitated condition until the end of its next turn, at\
which point it repeats the save. Second Failure: The tar-\
get has the Unconscious condition for 10 minutes. This\
effect ends for the target if it takes damage or a creature\
within 5 feet of it takes an action to wake it.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 20):\
At Will: Detect Magic, Minor Illusion, Scorching Ray\
(level 3 version), Shapechange (Beast or Humanoid\
form only, no Temporary Hit Points gained from the\
spell, and no Concentration or Temporary Hit Points\
required to maintain the spell), Speak with Animals\
1/Day Each: Control Weather, Detect Thoughts\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Blazing Light. The dragon uses Spellcasting to cast\
Scorching Ray (level 3 version).\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\
Scorching Sands. Dexterity Saving Throw: DC 20, one\
creature the dragon can see within 120 feet. Failure:\
36 (8d8) Fire damage, and the target’s Speed is halved\
until the end of its next turn. Failure or Success: The\
dragon can’t take this action again until the start of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/269|page 269]].
