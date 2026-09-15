---
splat_schema: 1
splat_id: cfbe1a2b-5d03-5718-a015-b18ad52be8c0
splat_kind: creature
splat_key: dnd2024:creatures/adult_brass_dragon
splat_name: Adult Brass Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_brass_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 268, 269
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 40 ft., Burrow 30 ft., Fly 80 ft.
  skills_text: History +7, Perception +11, Persuasion +8, Stealth +5
  immunities_text: Fire
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21
  languages_text: Common, Draconic
  cr_text: 13 (XP 10,000, or 11,500 in lair; PB +5)
  hit_points_text: 172 (15d12 + 75)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 10
  save_dexterity:
    value: 5
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 13
  save_wisdom:
    value: 6
  charisma:
    value: 17
  save_charisma:
    value: 3
  armor_class:
    value: 18
  max_hp:
    value: 172
  initiative:
    value: 10
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 30
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/adult_brass_dragon
  implementation: dnd2024:creature_adult_brass_dragon
  parameters:
    features:
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 3
          recovery: long_rest
          lair_uses: 4
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
              cast_level: 2
              spellcasting_profile: spellcasting
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 11
          reach_ft: 10
        hit:
          damage:
          - average: 17
            roll:
              count: 2
              sides: 10
              modifier: 6
            type: slashing
          - average: 4
            roll:
              count: 1
              sides: 8
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
        dc: 18
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 60
            origin: self
            width_ft: 5
        failure:
          damage:
          - average: 45
            roll:
              count: 10
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
        dc: 18
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 60
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
        save_dc: 16
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
            key: dnd2024:spells/detect_thoughts
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/control_weather
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
        cast_level: 2
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
        dc: 16
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible: true
        failure:
          damage:
          - average: 27
            roll:
              count: 6
              sides: 8
              modifier: 0
            type: fire
          speed_multiplier: 0.5
          until: target.next_turn_end
        success:
          effects: []
    skill_totals:
      history: 7
      perception: 11
      persuasion: 8
      stealth: 5
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      blindsight: 60
      darkvision: 120
      passive_perception: 21
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
- id: dnd2024:creature_adult_brass_dragon
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
  source_sha256: 3db4cf05974a3fb4d255201866ca495e8c9dc0119b1408e1ff34db1840dec9d8
  source_path: builtin/srd-5.2.1/creature/adult_brass_dragon.md
  pages:
  - 268
  - 269
  engine_gaps: &id001
  - dnd2024:creature_adult_brass_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 2577280b6d0e169cf3c956144ea9be7596e0b04417d9508741d8c14a1ab0f6a3
splat_clauses:
- key: source_rules
  text: '# Adult Brass Dragon Huge Dragon (Metallic), Chaotic Good AC 18 Initiative +10 (20) HP 172 (15d12
    + 75) Speed 40 ft., Burrow 30 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 10 +0 +5
    Con 21 +5 +5 Int 14 +2 +2 Wis 13 +1 +6 Cha 17 +3 +3 Skills History +7, Perception +11, Persuasion
    +8, Stealth +5 Immunities Fire Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21
    Languages Common, Draconic CR 13 (XP 10,000, or 11,500 in lair; PB +5) ### Traits Legendary Resistance
    (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ###
    Actions Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of
    (A) Sleep Breath or (B) Spellcasting to cast Scorching Ray. Rend. Melee Attack Roll: +11, reach 10
    ft. Hit: 17 (2d10 + 6) Slashing damage plus 4 (1d8) Fire damage. Fire Breath (Recharge 5–6). Dexterity
    Saving Throw: DC 18, each creature in a 60-foot-long, 5-footwide Line. Failure: 45 (10d8) Fire damage.
    Success: Half damage. Sleep Breath. Constitution Saving Throw: DC 18, each creature in a 60-foot Cone.
    Failure: The target has the Incapacitated condition until the end of its next turn, at which point
    it repeats the save. Second Failure: The target has the Unconscious condition for 10 minutes. This
    effect ends for the target if it takes damage or a creature within 5 feet of it takes an action to
    wake it. Spellcasting. The dragon casts one of the following spells, requiring no Material components
    and using Charisma as the spellcasting ability (spell save DC 16): At Will: Detect Magic, Minor Illusion,
    Scorching Ray, Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell,
    and no Concentration or Temporary Hit Points required to maintain the spell), Speak with Animals 1/Day
    Each: Detect Thoughts, Control Weather ### Legendary Actions Legendary Action Uses: 3 (4 in Lair).
    Immediately after another creature’s turn, the dragon can expend a use to take one of the following
    actions. The dragon regains all expended uses at the start of each of its turns. Blazing Light. The
    dragon uses Spellcasting to cast Scorching Ray. Pounce. The dragon moves up to half its Speed, and
    it makes one Rend attack. Scorching Sands. Dexterity Saving Throw: DC 16, one creature the dragon
    can see within 120 feet. Failure: 27 (6d8) Fire damage, and the target’s Speed is halved until the
    end of its next turn. Failure or Success: The dragon can’t take this action again until the start
    of its next turn.'
  sha256: f4d46e67d68b75fa06e819b8c2ce8e1b3b7afa7a277a0c96ba89b330f0119409
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Brass Dragon

<!-- SRD page 268 -->

Huge Dragon (Metallic), Chaotic Good\
AC 18 Initiative +10 (20)\
HP 172 (15d12 + 75)\
Speed 40 ft., Burrow 30 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 10 +0 +5 Con 21 +5 +5\
Int 14 +2 +2 Wis 13 +1 +6 Cha 17 +3 +3\
Skills History +7, Perception +11, Persuasion +8, Stealth +5\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 21\
Languages Common, Draconic\
CR 13 (XP 10,000, or 11,500 in lair; PB +5)\

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Sleep Breath or\
(B) Spellcasting to cast Scorching Ray.\
Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 17 (2d10\
+ 6) Slashing damage plus 4 (1d8) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 18, each creature in a 60-foot-long, 5-foot-\
wide Line. Failure: 45 (10d8) Fire damage. Success:\
Half damage.\
Sleep Breath. Constitution Saving Throw: DC 18, each\
creature in a 60-foot Cone. Failure: The target has the\
Incapacitated condition until the end of its next turn, at\
which point it repeats the save. Second Failure: The tar-\
get has the Unconscious condition for 10 minutes. This\
effect ends for the target if it takes damage or a creature\
within 5 feet of it takes an action to wake it.\

<!-- SRD page 269 -->

Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 16):\
At Will: Detect Magic, Minor Illusion, Scorching Ray,\
Shapechange (Beast or Humanoid form only, no\
Temporary Hit Points gained from the spell, and no\
Concentration or Temporary Hit Points required to\
maintain the spell), Speak with Animals\
1/Day Each: Detect Thoughts, Control Weather\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Blazing Light. The dragon uses Spellcasting to cast\
Scorching Ray.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\
Scorching Sands. Dexterity Saving Throw: DC 16, one\
creature the dragon can see within 120 feet. Failure:\
27 (6d8) Fire damage, and the target’s Speed is halved\
until the end of its next turn. Failure or Success: The\
dragon can’t take this action again until the start of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/268|page 268]], [[../../srd-5.2.1/pages/269|page 269]].
