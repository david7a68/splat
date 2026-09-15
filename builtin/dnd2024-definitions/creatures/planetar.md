---
splat_schema: 1
splat_id: 354c2bcd-4b81-50ec-8696-b361255d6067
splat_kind: creature
splat_key: dnd2024:creatures/planetar
splat_name: Planetar
splat_origin:
  key: dnd2024:reference/creature/planetar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 315
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Lawful Good
  tags_text: (Angel)
  movement_text: 40 ft., Fly 120 ft. (hover)
  skills_text: Perception +11
  resistances_text: Radiant
  immunities_text: Charmed, Exhaustion, Frightened
  senses_text: Truesight 120 ft.; Passive Perception 21
  languages_text: All; telepathy 120 ft.
  cr_text: 16 (XP 15,000; PB +5)
  hit_points_text: 262 (21d10 + 147)
splat_stats:
  strength:
    value: 24
  save_strength:
    value: 12
  dexterity:
    value: 20
  save_dexterity:
    value: 5
  constitution:
    value: 24
  save_constitution:
    value: 12
  intelligence:
    value: 19
  save_intelligence:
    value: 4
  wisdom:
    value: 22
  save_wisdom:
    value: 11
  charisma:
    value: 25
  save_charisma:
    value: 12
  armor_class:
    value: 19
  max_hp:
    value: 262
  initiative:
    value: 10
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 120
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/planetar
  implementation: dnd2024:creature_planetar
  parameters:
    features:
      divine_awareness:
        economy: trait
        operation: passive_rule
        detect_lies:
          requires: heard
      exalted_restoration:
        economy: trait
        operation: passive_rule
        trigger: self.dies
        unless:
          plane: mount_celestia
        remove_old_body: true
        new_body:
          delay_seconds: 0
          plane: mount_celestia
          hp: maximum
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: choice
          options:
          - operation: attack_sequence
            feature: radiant_sword
            count: 3
          - operation: repeat
            count: 2
            effect:
              operation: use_feature
              feature: holy_burst
      radiant_sword:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 12
          reach_ft: 10
        hit:
          damage:
          - average: 14
            roll:
              count: 2
              sides: 6
              modifier: 7
            type: slashing
          - average: 18
            roll:
              count: 4
              sides: 8
              modifier: 0
            type: radiant
      holy_burst:
        economy: action
        operation: saving_throw
        ability: dexterity
        dc: 20
        targets:
          kind: enemy
          selection: each
          area:
            shape: sphere
            radius_ft: 20
            center:
              visible: true
              range_ft: 120
        failure:
          damage:
          - average: 24
            roll:
              count: 7
              sides: 6
              modifier: 0
            type: radiant
        success:
          damage_multiplier: 0.5
          other_effects: false
      spellcasting:
        economy: action
        operation: spellcasting
        ability: charisma
        components_waived:
        - material
        save_dc: 20
        options:
        - spell:
            key: dnd2024:spells/detect_evil_and_good
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/commune
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
        - spell:
            key: dnd2024:spells/dispel_evil_and_good
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/raise_dead
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        material_components_available: true
      divine_aid:
        economy: bonus_action
        resource:
          kind: daily
          uses: 2
          recovery: long_rest
        operation: spell_choice
        options:
        - operation: cast_spell
          spell:
            key: dnd2024:spells/cure_wounds
        - operation: cast_spell
          spell:
            key: dnd2024:spells/invisibility
        - operation: cast_spell
          spell:
            key: dnd2024:spells/lesser_restoration
        - operation: cast_spell
          spell:
            key: dnd2024:spells/remove_curse
        ability: charisma
    skill_totals:
      perception: 11
    damage_resistances:
    - radiant
    damage_immunities: []
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    senses:
      truesight: 120
      passive_perception: 21
    languages:
      speaks: all
      understands: []
      telepathy_range_ft: 120
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
- id: dnd2024:creature_planetar
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
  - creature.died
  - rest.completed
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
  - curses
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: c9a4a77f021406da9d002eabe646ab6fcf6da4fdd84a8a0aaf6b9ceb76f19bbb
  source_path: builtin/srd-5.2.1/creature/planetar.md
  pages:
  - 315
  engine_gaps: &id001
  - dnd2024:creature_planetar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: b33857fc15bb227298c033d1bc4b1907ed172505f2dbf08f15516ef9e7e77adf
splat_clauses:
- key: source_rules
  text: '# Planetar Large Celestial (Angel), Lawful Good AC 19 Initiative +10 (20) HP 262 (21d10 + 147)
    Speed 40 ft., Fly 120 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 24 +7 +12 Dex 20 +5 +5 Con 24 +7
    +12 Int 19 +4 +4 Wis 22 +6 +11 Cha 25 +7 +12 Skills Perception +11 Resistances Radiant Immunities
    Charmed, Exhaustion, Frightened Senses Truesight 120 ft.; Passive Perception 21 Languages All; telepathy
    120 ft. CR 16 (XP 15,000; PB +5) ### Traits Divine Awareness. The planetar knows if it hears a lie.
    Exalted Restoration. If the planetar dies outside Mount Celestia, its body disappears, and it gains
    a new body instantly, reviving with all its Hit Points somewhere in Mount Celestia. Magic Resistance.
    The planetar has Advantage on saving throws against spells and other magical effects. ### Actions
    Multiattack. The planetar makes three Radiant Sword attacks or uses Holy Burst twice. Radiant Sword.
    Melee Attack Roll: +12, reach 10 ft. Hit: 14 (2d6 + 7) Slashing damage plus 18 (4d8) Radiant damage.
    Holy Burst. Dexterity Saving Throw: DC 20, each enemy in a 20-foot-radius Sphere centered on a point
    the planetar can see within 120 feet. Failure: 24 (7d6) Radiant damage. Success: Half damage. Spellcasting.
    The planetar casts one of the following spells, requiring no Material components and using Charisma
    as spellcasting ability (spell save DC 20): At Will: Detect Evil and Good 1/Day Each: Commune, Control
    Weather, Dispel Evil and Good, Raise Dead ### Bonus Actions Divine Aid (2/Day). The planetar casts
    Cure Wounds, Invisibility, Lesser Restoration, or Remove Curse, using the same spellcasting ability
    as Spellcasting.'
  sha256: fb533310c5247c874cb46126ce237c0f426350780338e22760a8f1ca0319d110
  status: contract_specified
splat_requires_mechanics: *id001
---

# Planetar

<!-- SRD page 315 -->

Large Celestial (Angel), Lawful Good\
AC 19 Initiative +10 (20)\
HP 262 (21d10 + 147)\
Speed 40 ft., Fly 120 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 24 +7 +12 Dex 20 +5 +5 Con 24 +7 +12\
Int 19 +4 +4 Wis 22 +6 +11 Cha 25 +7 +12\
Skills Perception +11\
Resistances Radiant\
Immunities Charmed, Exhaustion, Frightened\
Senses Truesight 120 ft.; Passive Perception 21\
Languages All; telepathy 120 ft.\
CR 16 (XP 15,000; PB +5)\

### Traits

Divine Awareness. The planetar knows if it hears a lie.\
Exalted Restoration. If the planetar dies outside Mount\
Celestia, its body disappears, and it gains a new body\
instantly, reviving with all its Hit Points somewhere in\
Mount Celestia.\
Magic Resistance. The planetar has Advantage on sav-\
ing throws against spells and other magical effects.\

### Actions

Multiattack. The planetar makes three Radiant Sword\
attacks or uses Holy Burst twice.\
Radiant Sword. Melee Attack Roll: +12, reach 10 ft.\
Hit: 14 (2d6 + 7) Slashing damage plus 18 (4d8) Radi-\
ant damage.\
Holy Burst. Dexterity Saving Throw: DC 20, each en-\
emy in a 20-foot-radius Sphere centered on a point the\
planetar can see within 120 feet. Failure: 24 (7d6) Radi-\
ant damage. Success: Half damage.\
Spellcasting. The planetar casts one of the following\
spells, requiring no Material components and using\
Charisma as spellcasting ability (spell save DC 20):\
At Will: Detect Evil and Good\
1/Day Each: Commune, Control Weather, Dispel Evil\
and Good, Raise Dead\

### Bonus Actions

Divine Aid (2/Day). The planetar casts Cure Wounds,\
Invisibility, Lesser Restoration, or Remove Curse, using\
the same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/315|page 315]].
