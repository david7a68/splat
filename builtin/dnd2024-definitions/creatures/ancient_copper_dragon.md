---
splat_schema: 1
splat_id: 4f924e40-6e9d-500c-b56a-b90483966022
splat_kind: creature
splat_key: dnd2024:creatures/ancient_copper_dragon
splat_name: Ancient Copper Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_copper_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 277
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Chaotic Good
  tags_text: (Metallic)
  movement_text: 40 ft., Climb 40 ft., Fly 80 ft.
  skills_text: Deception +13, Perception +17, Stealth +8
  immunities_text: Acid
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27
  languages_text: Common, Draconic
  cr_text: 21 (XP 33,000, or 41,000 in lair; PB +7)
  hit_points_text: 367 (21d20 + 147)
splat_stats:
  strength:
    value: 27
  save_strength:
    value: 8
  dexterity:
    value: 12
  save_dexterity:
    value: 8
  constitution:
    value: 25
  save_constitution:
    value: 7
  intelligence:
    value: 20
  save_intelligence:
    value: 5
  wisdom:
    value: 17
  save_wisdom:
    value: 10
  charisma:
    value: 22
  save_charisma:
    value: 6
  armor_class:
    value: 21
  max_hp:
    value: 367
  initiative:
    value: 15
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
  mechanic: dnd2024:creatures/ancient_copper_dragon
  implementation: dnd2024:creature_ancient_copper_dragon
  parameters:
    skill_totals:
      deception: 13
      perception: 17
      stealth: 8
    damage_immunities:
    - acid
    condition_immunities: []
    senses:
    - sense: blindsight
      range_ft: 60
    - sense: darkvision
      range_ft: 120
    passive_perception: 27
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
        attack_bonus: 15
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
                sides: 10
              add: 8
            average: 19
          - type: acid
            amount:
              dice:
                count: 2
                sides: 8
              add: 0
            average: 9
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
            cast_level: 5
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
          - spell:
              key: dnd2024:spells/project_image
            uses:
              count: 1
              recovery: long_rest
          save_dc: 21
      acid_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 22
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 90
            width_ft: 10
        failure:
        - op: damage
          components:
          - type: acid
            amount:
              dice:
                count: 14
                sides: 8
              add: 0
            average: 63
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
        dc: 22
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 90
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
        dc: 21
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible_to_actor: true
        failure:
        - op: damage
          components:
          - type: psychic
            amount:
              dice:
                count: 9
                sides: 6
              add: 0
            average: 31
        - op: roll.add
          amount:
            negate:
              dice:
                count: 1
                sides: 8
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
          cast_level: 5
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
            cast_level: 5
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
- id: dnd2024:creature_ancient_copper_dragon
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
  source_sha256: 40e60c7d1d0b948aa526cb76cb8cad8870dc857515ad843ebfeed67fb076f887
  source_path: builtin/srd-5.2.1/creature/ancient_copper_dragon.md
  pages:
  - 277
  engine_gaps: &id001
  - dnd2024:creature_ancient_copper_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: ab5510a180d2522185d2ebbb07de914da7019b6bb1a2ba722469ec3e65f704b6
splat_clauses:
- key: source_rules
  text: '# Ancient Copper Dragon Gargantuan Dragon (Metallic), Chaotic Good AC 21 Initiative +15 (25)
    HP 367 (21d20 + 147) Speed 40 ft., Climb 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8
    Dex 12 +1 +8 Con 25 +7 +7 Int 20 +5 +5 Wis 17 +3 +10 Cha 22 +6 +6 Skills Deception +13, Perception
    +17, Stealth +8 Immunities Acid Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27
    Languages Common, Draconic CR 21 (XP 33,000, or 41,000 in lair; PB +7) ### Traits Legendary Resistance
    (4/Day, or 5/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ###
    Actions Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of
    (A) Slowing Breath or (B) Spellcasting to cast Mind Spike (level 5 version). Rend. Melee Attack Roll:
    +15, reach 15 ft. Hit: 19 (2d10 + 8) Slashing damage plus 9 (2d8) Acid damage. Acid Breath (Recharge
    5–6). Dexterity Saving Throw: DC 22, each creature in an 90-foot-long, 10-footwide Line. Failure:
    63 (14d8) Acid damage. Success: Half damage. Slowing Breath. Constitution Saving Throw: DC 22, each
    creature in a 90-foot Cone. Failure: The target can’t take Reactions; its Speed is halved; and it
    can take either an action or a Bonus Action on its turn, not both. This effect lasts until the end
    of its next turn. Spellcasting. The dragon casts one of the following spells, requiring no Material
    components and using Charisma as the spellcasting ability (spell save DC 21): At Will: Detect Magic,
    Mind Spike (level 5 version), Minor Illusion, Shapechange (Beast or Humanoid form only, no Temporary
    Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain
    the spell) 1/Day Each: Greater Restoration, Major Image, Project Image ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use
    to take one of the following actions. The dragon regains all expended uses at the start of each of
    its turns. Giggling Magic. Charisma Saving Throw: DC 21, one creature the dragon can see within 120
    feet. Failure: 31 (9d6) Psychic damage. Until the end of its next turn, the target rolls 1d8 whenever
    it makes an ability check or attack roll and subtracts the number rolled from the D20 Test. Failure
    or Success: The dragon can’t take this action again until the start of its next turn. Mind Jolt. The
    dragon uses Spellcasting to cast Mind Spike (level 5 version). The dragon can’t take this action again
    until the start of its next turn. Pounce. The dragon moves up to half its Speed, and it makes one
    Rend attack.'
  sha256: c15e549ef12bae3eb0561ec99636af3e5312bbac7c25beee8e0a77c230a3f991
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Copper Dragon

<!-- SRD page 277 -->

Gargantuan Dragon (Metallic), Chaotic Good\
AC 21 Initiative +15 (25)\
HP 367 (21d20 + 147)\
Speed 40 ft., Climb 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 12 +1 +8 Con 25 +7 +7\
Int 20 +5 +5 Wis 17 +3 +10 Cha 22 +6 +6\
Skills Deception +13, Perception +17, Stealth +8\
Immunities Acid\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 27\
Languages Common, Draconic\
CR 21 (XP 33,000, or 41,000 in lair; PB +7)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Slowing Breath\
or (B) Spellcasting to cast Mind Spike (level 5 version).\
Rend. Melee Attack Roll: +15, reach 15 ft. Hit: 19 (2d10\
+ 8) Slashing damage plus 9 (2d8) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 22, each creature in an 90-foot-long, 10-foot-\
wide Line. Failure: 63 (14d8) Acid damage. Success:\
Half damage.\
Slowing Breath. Constitution Saving Throw: DC 22,\
each creature in a 90-foot Cone. Failure: The target\
can’t take Reactions; its Speed is halved; and it can take\
either an action or a Bonus Action on its turn, not both.\
This effect lasts until the end of its next turn.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 21):\
At Will: Detect Magic, Mind Spike (level 5 version),\
Minor Illusion, Shapechange (Beast or Humanoid\
form only, no Temporary Hit Points gained from the\
spell, and no Concentration or Temporary Hit Points\
required to maintain the spell)\
1/Day Each: Greater Restoration, Major Image,\
Project Image\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Giggling Magic. Charisma Saving Throw: DC 21, one\
creature the dragon can see within 120 feet. Failure:\
31 (9d6) Psychic damage. Until the end of its next turn,\
the target rolls 1d8 whenever it makes an ability check\
or attack roll and subtracts the number rolled from the\
D20 Test. Failure or Success: The dragon can’t take this\
action again until the start of its next turn.\
Mind Jolt. The dragon uses Spellcasting to cast Mind\
Spike (level 5 version). The dragon can’t take this action\
again until the start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/277|page 277]].
