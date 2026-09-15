---
splat_schema: 1
splat_id: 3c3ee465-62cb-54df-b868-1a436b108091
splat_kind: creature
splat_key: dnd2024:creatures/adult_blue_dragon
splat_name: Adult Blue Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_blue_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266, 267
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Burrow 30 ft., Fly 80 ft.
  skills_text: Perception +12, Stealth +5
  immunities_text: Lightning
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22
  languages_text: Common, Draconic
  cr_text: 16 (XP 15,000, or 18,000 in lair; PB +5)
  hit_points_text: 212 (17d12 + 102)
splat_stats:
  strength:
    value: 25
  save_strength:
    value: 7
  dexterity:
    value: 10
  save_dexterity:
    value: 5
  constitution:
    value: 23
  save_constitution:
    value: 6
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 15
  save_wisdom:
    value: 7
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 19
  max_hp:
    value: 212
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
  mechanic: dnd2024:creatures/adult_blue_dragon
  implementation: dnd2024:creature_adult_blue_dragon
  parameters:
    skill_totals:
      perception: 12
      stealth: 5
    damage_immunities:
    - lightning
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
        attack_bonus: 12
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
                sides: 8
              add: 7
            average: 16
          - type: lightning
            amount:
              dice:
                count: 1
                sides: 10
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
          save_dc: 18
      lightning_breath:
        economy: action
        op: saving_throw
        ability: dexterity
        dc: 19
        targets:
          kind: creature
          count: all
          area:
            shape: line
            origin: actor
            size_ft: 90
            width_ft: 5
        failure:
        - op: damage
          components:
          - type: lightning
            amount:
              dice:
                count: 11
                sides: 10
              add: 0
            average: 60
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
- id: dnd2024:creature_adult_blue_dragon
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
  source_sha256: 1a3771a1a548c5339230e6df7e2ac399fa62f317803daba845a3293622e7e226
  source_path: builtin/srd-5.2.1/creature/adult_blue_dragon.md
  pages:
  - 266
  - 267
  engine_gaps: &id001
  - dnd2024:creature_adult_blue_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 9077d4e90b58939a0a78d27bf33a7a874133cb60664c34e58b6211f48b20c65d
splat_clauses:
- key: source_rules
  text: '# Adult Blue Dragon Huge Dragon (Chromatic), Lawful Evil AC 19 Initiative +10 (20) HP 212 (17d12
    + 102) Speed 40 ft., Burrow 30 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 25 +7 +7 Dex 10 +0 +5
    Con 23 +6 +6 Int 16 +3 +3 Wis 15 +2 +7 Cha 20 +5 +5 Skills Perception +12, Stealth +5 Immunities Lightning
    Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22 Languages Common, Draconic CR
    16 (XP 15,000, or 18,000 in lair; PB +5) ### Traits Legendary Resistance (3/Day, or 4/Day in Lair).
    If the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack. The
    dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Shatter.
    Rend. Melee Attack Roll: +12, reach 10 ft. Hit: 16 (2d8 + 7) Slashing damage plus 5 (1d10) Lightning
    damage. Lightning Breath (Recharge 5–6). Dexterity Saving Throw: DC 19, each creature in a 90-foot-long,
    5-footwide Line. Failure: 60 (11d10) Lightning damage. Success: Half damage. Spellcasting. The dragon
    casts one of the following spells, requiring no Material components and using Charisma as the spellcasting
    ability (spell save DC 18): At Will: Detect Magic, Invisibility, Mage Hand, Shatter 1/Day Each: Scrying,
    Sending ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s
    turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended
    uses at the start of each of its turns. Cloaked Flight. The dragon uses Spellcasting to cast Invisibility
    on itself, and it can fly up to half its Fly Speed. The dragon can’t take this action again until
    the start of its next turn. Sonic Boom. The dragon uses Spellcasting to cast Shatter. The dragon can’t
    take this action again until the start of its next turn. Tail Swipe. The dragon makes one Rend attack.'
  sha256: e921f096c822c295341e5a37f538b7c00e01f253f68d1c7ffc33c4a77eb2e38f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Blue Dragon

<!-- SRD page 266 -->

Huge Dragon (Chromatic), Lawful Evil\
AC 19 Initiative +10 (20)\
HP 212 (17d12 + 102)\
Speed 40 ft., Burrow 30 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 25 +7 +7 Dex 10 +0 +5 Con 23 +6 +6\
Int 16 +3 +3 Wis 15 +2 +7 Cha 20 +5 +5\
Skills Perception +12, Stealth +5\
Immunities Lightning\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 22\
Languages Common, Draconic\
CR 16 (XP 15,000, or 18,000 in lair; PB +5)\

### Traits

Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to\
cast Shatter.\
Rend. Melee Attack Roll: +12, reach 10 ft. Hit: 16 (2d8\
+ 7) Slashing damage plus 5 (1d10) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 19, each creature in a 90-foot-long, 5-foot-\

<!-- SRD page 267 -->

wide Line. Failure: 60 (11d10) Lightning damage. Suc-\
cess: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 18):\
At Will: Detect Magic, Invisibility, Mage Hand, Shatter\
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
ter. The dragon can’t take this action again until the\
start of its next turn.\
Tail Swipe. The dragon makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/266|page 266]], [[../../srd-5.2.1/pages/267|page 267]].
