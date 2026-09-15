---
splat_schema: 1
splat_id: 6a5f535d-ceda-554a-b6af-4db9fc9e48e1
splat_kind: creature
splat_key: dnd2024:creatures/adult_copper_dragon
splat_name: Adult Copper Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_copper_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 276, 277
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 40 ft., Climb 40 ft., Fly 80 ft.
  skills_text: Deception +9, Perception +12, Stealth +6
  immunities_text: Acid
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22
  languages_text: Common, Draconic
  cr_text: 14 (XP 11,500, or 13,000 in lair; PB +5)
  hit_points_text: 184 (16d12 + 80)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 12
  save_dexterity:
    value: 6
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 15
  save_wisdom:
    value: 7
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 18
  max_hp:
    value: 184
  initiative:
    value: 11
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
  mechanic: dnd2024:creatures/adult_copper_dragon
  implementation: dnd2024:creature_adult_copper_dragon
  parameters:
    skill_totals:
      deception: 9
      perception: 12
      stealth: 6
    damage_immunities:
    - acid
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 60
    - sense: darkvision
      range_ft: 120
    passive_perception: 22
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
        attack_bonus: 11
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
                count: 2
                sides: 10
              add: 6
            average: 17
          - type: acid
            amount:
              dice:
                count: 1
                sides: 8
              add: 0
            average: 4
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: charisma
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/mind_spike
            uses:
              unlimited: true
            cast_level: 4
          - spell:
              key: dnd2024:spells/minor_illusion
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/shapechange
            uses:
              unlimited: true
            overrides:
              forms:
              - beast
              - humanoid
              temporary_hp_granted: 0
              maintain_requires_concentration: false
              maintain_requires_temporary_hp: false
          - spell:
              key: dnd2024:spells/greater_restoration
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/major_image
            uses:
              count: 1
              recovery: long_rest
          save_dc: 17
      acid_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 18
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 60
            width_ft: 5
        failure:
        - op: damage
          components:
          - type: acid
            amount:
              dice:
                count: 12
                sides: 8
              add: 0
            average: 54
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
      slowing_breath:
        economy: action
        op: saving_throw
        ability: constitution
        dc: 18
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 60
        failure:
        - op: effect.apply
          duration:
            until: target.next_turn_end
          rules:
            reactions_allowed: false
            speed_multiplier: 0.5
            turn_economy_choice:
            - action
            - bonus_action
            both_allowed: false
        success: []
      giggling_magic:
        economy: legendary_action
        op: saving_throw
        ability: charisma
        dc: 17
        targets:
          kind: creature
          count: 1
          range_ft: 90
          visible_to_actor: true
        failure:
        - op: damage
          components:
          - type: psychic
            amount:
              dice:
                count: 7
                sides: 6
              add: 0
            average: 24
        - op: roll.add
          amount:
            negate:
              dice:
                count: 1
                sides: 6
              add: 0
          rolls:
          - ability_check
          - attack
          roll_each_test: true
          duration:
            until: target.next_turn_end
        success: []
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      mind_jolt:
        economy: legendary_action
        op: sequence
        steps:
        - op: spell.cast
          spell:
            key: dnd2024:spells/mind_spike
          profile: spellcasting
          cast_level: 4
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
          - op: feature.use
            feature: slowing_breath
          - op: spell.cast
            spell:
              key: dnd2024:spells/mind_spike
            profile: spellcasting
            cast_level: 4
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
- id: dnd2024:creature_adult_copper_dragon
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
  source_sha256: e73cdcf73ba5338df34eb10202ac817ab7777a0613f420b106e5d1f29c79eaf7
  source_path: builtin/srd-5.2.1/creature/adult_copper_dragon.md
  pages:
  - 276
  - 277
  engine_gaps: &id001
  - dnd2024:creature_adult_copper_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: a2f4ebff2c8d111a73b7fd5020957fc01b4e74e19db6f32d2da3fddd67e63361
splat_clauses:
- key: source_rules
  text: '# Adult Copper Dragon Huge Dragon (Metallic), Chaotic Good AC 18 Initiative +11 (21) HP 184 (16d12
    + 80) Speed 40 ft., Climb 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 12 +1 +6
    Con 21 +5 +5 Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +4 Skills Deception +9, Perception +12, Stealth +6
    Immunities Acid Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22 Languages Common,
    Draconic CR 14 (XP 11,500, or 13,000 in lair; PB +5) ### Traits Legendary Resistance (3/Day, or 4/Day
    in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack.
    The dragon makes three Rend attacks. It can replace one attack with a use of (A) Slowing Breath or
    (B) Spellcasting to cast Mind Spike (level 4 version). Rend. Melee Attack Roll: +11, reach 10 ft.
    Hit: 17 (2d10 + 6) Slashing damage plus 4 (1d8) Acid damage. Acid Breath (Recharge 5–6). Dexterity
    Saving Throw: DC 18, each creature in an 60-foot-long, 5-footwide Line. Failure: 54 (12d8) Acid damage.
    Success: Half damage. Slowing Breath. Constitution Saving Throw: DC 18, each creature in a 60-foot
    Cone. Failure: The target can’t take Reactions; its Speed is halved; and it can take either an action
    or a Bonus Action on its turn, not both. This effect lasts until the end of its next turn. Spellcasting.
    The dragon casts one of the following spells, requiring no Material components and using Charisma
    as the spellcasting ability (spell save DC 17): At Will: Detect Magic, Mind Spike (level 4 version),
    Minor Illusion, Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the
    spell, and no Concentration or Temporary Hit Points required to maintain the spell) 1/Day Each: Greater
    Restoration, Major Image ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after
    another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon
    regains all expended uses at the start of each of its turns. Giggling Magic. Charisma Saving Throw:
    DC 17, one creature the dragon can see within 90 feet. Failure: 24 (7d6) Psychic damage. Until the
    end of its next turn, the target rolls 1d6 whenever it makes an ability check or attack roll and subtracts
    the number rolled from the D20 Test. Failure or Success: The dragon can’t take this action again until
    the start of its next turn. Mind Jolt. The dragon uses Spellcasting to cast Mind Spike (level 4 version).
    The dragon can’t take this action again until the start of its next turn. Pounce. The dragon moves
    up to half its Speed, and it makes one Rend attack.'
  sha256: c98612a0b79e9a604ea49e5ab69140e96340930c32d0659943696cc59dc56d50
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Copper Dragon

<!-- SRD page 276 -->

Huge Dragon (Metallic), Chaotic Good\
AC 18 Initiative +11 (21)\
HP 184 (16d12 + 80)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 12 +1 +6 Con 21 +5 +5\
Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +4\
Skills Deception +9, Perception +12, Stealth +6\
Immunities Acid\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 22\
Languages Common, Draconic\
CR 14 (XP 11,500, or 13,000 in lair; PB +5)\

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Slowing Breath\
or (B) Spellcasting to cast Mind Spike (level 4 version).\
Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 17 (2d10\
+ 6) Slashing damage plus 4 (1d8) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 18, each creature in an 60-foot-long, 5-foot-\
wide Line. Failure: 54 (12d8) Acid damage. Success:\
Half damage.\
Slowing Breath. Constitution Saving Throw: DC 18,\
each creature in a 60-foot Cone. Failure: The target\
can’t take Reactions; its Speed is halved; and it can take\
either an action or a Bonus Action on its turn, not both.\
This effect lasts until the end of its next turn.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17):\
At Will: Detect Magic, Mind Spike (level 4 version),\
Minor Illusion, Shapechange (Beast or Humanoid\
form only, no Temporary Hit Points gained from the\
spell, and no Concentration or Temporary Hit Points\
required to maintain the spell)\
1/Day Each: Greater Restoration, Major Image\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Giggling Magic. Charisma Saving Throw: DC 17, one\
creature the dragon can see within 90 feet. Failure: 24\
(7d6) Psychic damage. Until the end of its next turn,\
the target rolls 1d6 whenever it makes an ability check\
or attack roll and subtracts the number rolled from the\
D20 Test. Failure or Success: The dragon can’t take this\
action again until the start of its next turn.\
Mind Jolt. The dragon uses Spellcasting to cast Mind\
Spike (level 4 version). The dragon can’t take this action\
again until the start of its next turn.\

<!-- SRD page 277 -->

Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/276|page 276]], [[../../srd-5.2.1/pages/277|page 277]].
