---
splat_schema: 1
splat_id: eb0c302e-d1ce-5d59-a29c-6ca17b77f484
splat_kind: creature
splat_key: dnd2024:creatures/ancient_blue_dragon
splat_name: Ancient Blue Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_blue_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 267
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Burrow 40 ft., Fly 80 ft.
  skills_text: Perception +17, Stealth +7
  immunities_text: Lightning
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27
  languages_text: Common, Draconic
  cr_text: 23 (XP 50,000, or 62,000 in lair; PB +7)
  hit_points_text: 481 (26d20 + 208)
splat_stats:
  strength:
    value: 29
  save_strength:
    value: 9
  dexterity:
    value: 10
  save_dexterity:
    value: 7
  constitution:
    value: 27
  save_constitution:
    value: 8
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 17
  save_wisdom:
    value: 10
  charisma:
    value: 25
  save_charisma:
    value: 7
  armor_class:
    value: 22
  max_hp:
    value: 481
  initiative:
    value: 14
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
  mechanic: dnd2024:creatures/ancient_blue_dragon
  implementation: dnd2024:creature_ancient_blue_dragon
  parameters:
    skill_totals:
      perception: 17
      stealth: 7
    damage_immunities:
    - lightning
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
        attack_bonus: 16
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
              add: 9
            average: 18
          - type: lightning
            amount:
              dice:
                count: 2
                sides: 10
              add: 0
            average: 11
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
              key: dnd2024:spells/invisibility
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/mage_hand
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/shatter
            uses:
              unlimited: true
            cast_level: 3
          - spell:
              key: dnd2024:spells/scrying
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/sending
            uses:
              count: 1
              recovery: long_rest
          save_dc: 22
      lightning_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 23
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 120
            width_ft: 10
        failure:
        - op: damage
          components:
          - type: lightning
            amount:
              dice:
                count: 16
                sides: 10
              add: 0
            average: 88
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
      cloaked_flight:
        economy: legendary_action
        op: sequence
        steps:
        - op: spell.cast
          spell:
            key: dnd2024:spells/invisibility
          profile: spellcasting
          target: self
        - op: movement
          maximum:
            multiply:
            - 0.5
            - actor_stat: fly_speed
          optional: true
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      sonic_boom:
        economy: legendary_action
        op: sequence
        steps:
        - op: spell.cast
          spell:
            key: dnd2024:spells/shatter
          profile: spellcasting
          cast_level: 3
        lockout_until: actor.next_turn_start
        legendary_cost: 1
      tail_swipe:
        economy: legendary_action
        op: feature.use
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
              key: dnd2024:spells/shatter
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
- id: dnd2024:creature_ancient_blue_dragon
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
  source_sha256: e652475efd616b5d81edac03e3c6e177e49b6c1656677457eb2d70483460bc78
  source_path: builtin/srd-5.2.1/creature/ancient_blue_dragon.md
  pages:
  - 267
  engine_gaps: &id001
  - dnd2024:creature_ancient_blue_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: c18ecea5af93c73399abd2264611b599a7e1311ceba48f95be9902fc384e7ad7
splat_clauses:
- key: source_rules
  text: '# Ancient Blue Dragon Gargantuan Dragon (Chromatic), Lawful Evil AC 22 Initiative +14 (24) HP
    481 (26d20 + 208) Speed 40 ft., Burrow 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 29 +9 +9
    Dex 10 +0 +7 Con 27 +8 +8 Int 18 +4 +4 Wis 17 +3 +10 Cha 25 +7 +7 Skills Perception +17, Stealth +7
    Immunities Lightning Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27 Languages
    Common, Draconic CR 23 (XP 50,000, or 62,000 in lair; PB +7) ### Traits Legendary Resistance (4/Day,
    or 5/Day in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ### Actions
    Multiattack. The dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting
    to cast Shatter (level 3 version). Rend. Melee Attack Roll: +16, reach 15 ft. Hit: 18 (2d8 + 9) Slashing
    damage plus 11 (2d10) Lightning damage. Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC
    23, each creature in a 120-foot-long, 10-foot-wide Line. Failure: 88 (16d10) Lightning damage. Success:
    Half damage. Spellcasting. The dragon casts one of the following spells, requiring no Material components
    and using Charisma as the spellcasting ability (spell save DC 22): At Will: Detect Magic, Invisibility,
    Mage Hand, Shatter (level 3 version) 1/Day Each: Scrying, Sending ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use
    to take one of the following actions. The dragon regains all expended uses at the start of each of
    its turns. Cloaked Flight. The dragon uses Spellcasting to cast Invisibility on itself, and it can
    fly up to half its Fly Speed. The dragon can’t take this action again until the start of its next
    turn. Sonic Boom. The dragon uses Spellcasting to cast Shatter (level 3 version). The dragon can’t
    take this action again until the start of its next turn. Tail Swipe. The dragon makes one Rend attack.'
  sha256: a5c6e6eab2f9af779d296580b37c29f73b0757dd62877f377cff6788b24d5566
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Blue Dragon

<!-- SRD page 267 -->

Gargantuan Dragon (Chromatic), Lawful Evil\
AC 22 Initiative +14 (24)\
HP 481 (26d20 + 208)\
Speed 40 ft., Burrow 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 29 +9 +9 Dex 10 +0 +7 Con 27 +8 +8\
Int 18 +4 +4 Wis 17 +3 +10 Cha 25 +7 +7\
Skills Perception +17, Stealth +7\
Immunities Lightning\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 27\
Languages Common, Draconic\
CR 23 (XP 50,000, or 62,000 in lair; PB +7)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Shatter (level 3 version).\
Rend. Melee Attack Roll: +16, reach 15 ft. Hit: 18 (2d8\
+ 9) Slashing damage plus 11 (2d10) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 23, each creature in a 120-foot-long,\
10-foot-wide Line. Failure: 88 (16d10) Lightning dam-\
age. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 22):\
At Will: Detect Magic, Invisibility, Mage Hand, Shatter\
(level 3 version)\
1/Day Each: Scrying, Sending\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Cloaked Flight. The dragon uses Spellcasting to cast In-\
visibility on itself, and it can fly up to half its Fly Speed.\
The dragon can’t take this action again until the start of\
its next turn.\
Sonic Boom. The dragon uses Spellcasting to cast Shat-\
ter (level 3 version). The dragon can’t take this action\
again until the start of its next turn.\
Tail Swipe. The dragon makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/267|page 267]].
