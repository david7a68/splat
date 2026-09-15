---
splat_schema: 1
splat_id: d74914a1-1960-56f1-b633-6cc3a498362d
splat_kind: creature
splat_key: dnd2024:creatures/mage
splat_name: Mage
splat_origin:
  key: dnd2024:reference/creature/mage
splat_sources:
- title: SRD 5.2.1
  locator: pp. 305
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  tags_text: (Wizard)
  movement_text: 30 ft.
  skills_text: Arcana +6, History +6, Perception +4
  gear_text: Wand
  senses_text: Passive Perception 14
  languages_text: Common plus three other languages
  cr_text: 6 (XP 2,300; PB +3)
  hit_points_text: 81 (18d8)
splat_stats:
  strength:
    value: 9
  save_strength:
    value: -1
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 17
  save_intelligence:
    value: 6
  wisdom:
    value: 12
  save_wisdom:
    value: 4
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 15
  max_hp:
    value: 81
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mage
  implementation: dnd2024:creature_mage
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: arcane_burst
          count: 3
      arcane_burst:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 6
          reach_ft: 5
          range_ft: 120
          long_range_ft: null
        hit:
          damage:
          - average: 16
            roll:
              count: 3
              sides: 8
              modifier: 3
            type: force
      spellcasting:
        economy: action
        operation: spellcasting
        ability: intelligence
        components_waived: []
        save_dc: 14
        options:
        - spell:
            key: dnd2024:spells/detect_magic
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/light
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/mage_armor
          usage:
            kind: at_will
          included_in_header_ac: true
        - spell:
            key: dnd2024:spells/mage_hand
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/prestidigitation
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/fireball
          usage:
            kind: daily
            uses: 2
            pool: per_spell
            recovery: long_rest
          cast_level: 4
        - spell:
            key: dnd2024:spells/invisibility
          usage:
            kind: daily
            uses: 2
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/cone_of_cold
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/fly
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        material_components_available: true
      misty_step:
        economy: bonus_action
        resource:
          kind: daily
          uses: 3
          recovery: long_rest
        operation: cast_spell
        spell:
          key: dnd2024:spells/misty_step
        ability: intelligence
      protective_magic:
        economy: reaction
        resource:
          kind: daily
          uses: 3
          recovery: long_rest
        operation: spell_choice
        options:
        - operation: cast_spell
          spell:
            key: dnd2024:spells/counterspell
        - operation: cast_spell
          spell:
            key: dnd2024:spells/shield
        trigger: chosen_spell.normal_trigger
        ability: intelligence
        save_dc: 14
    skill_totals:
      arcana: 6
      history: 6
      perception: 4
    gear:
    - item: wand
      quantity: 1
    senses:
      passive_perception: 14
    languages:
      speaks:
      - common
      understands: []
      additional_languages:
        count: 3
        chooser: creator
        can_speak: true
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
- id: dnd2024:creature_mage
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
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - spellcasting
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: 100c75a887b41b7f78fbb5daea234ea6e7ac417a211992244ca1cd5b09f90b55
  source_path: builtin/srd-5.2.1/creature/mage.md
  pages:
  - 305
  engine_gaps: &id001
  - dnd2024:creature_mage
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 5fd74c2ce31d8fe7622cbb28faed94f3ea8df653e537840cae3aa53a3be33039
splat_clauses:
- key: source_rules
  text: '# Mage Medium or Small Humanoid (Wizard), Neutral AC 15 Initiative +2 (12) HP 81 (18d8) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 9 -1 -1 Dex 14 +2 +2 Con 11 +0 +0 Int 17 +3 +6 Wis 12 +1 +4
    Cha 11 +0 +0 Skills Arcana +6, History +6, Perception +4 Gear Wand Senses Passive Perception 14 Languages
    Common plus three other languages CR 6 (XP 2,300; PB +3) ### Actions Multiattack. The mage makes three
    Arcane Burst attacks. Arcane Burst. Melee or Ranged Attack Roll: +6, reach 5 ft. or range 120 ft.
    Hit: 16 (3d8 + 3) Force damage. Spellcasting. The mage casts one of the following spells, using Intelligence
    as the spellcasting ability (spell save DC 14): At Will: Detect Magic, Light, Mage Armor (included
    in AC), Mage Hand, Prestidigitation 2/Day Each: Fireball (level 4 version), Invisibility 1/Day Each:
    Cone of Cold, Fly ### Bonus Actions Misty Step (3/Day). The mage casts Misty Step, using the same
    spellcasting ability as Spellcasting. ### Reactions Protective Magic (3/Day). The mage casts Counterspell
    or Shield in response to the spell’s trigger, using the same spellcasting ability as Spellcasting.'
  sha256: 29124c627e67192586014a240fc0341d8224d9056da5bb1f069a17cb72c12838
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mage

<!-- SRD page 305 -->

Medium or Small Humanoid (Wizard), Neutral\
AC 15 Initiative +2 (12)\
HP 81 (18d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 9 −1 −1 Dex 14 +2 +2 Con 11 +0 +0\
Int 17 +3 +6 Wis 12 +1 +4 Cha 11 +0 +0\
Skills Arcana +6, History +6, Perception +4\
Gear Wand\
Senses Passive Perception 14\
Languages Common plus three other languages\
CR 6 (XP 2,300; PB +3)\

### Actions

Multiattack. The mage makes three Arcane\
Burst attacks.\
Arcane Burst. Melee or Ranged Attack Roll: +6, reach\
5 ft. or range 120 ft. Hit: 16 (3d8 + 3) Force damage.\
Spellcasting. The mage casts one of the following\
spells, using Intelligence as the spellcasting ability (spell\
save DC 14):\
At Will: Detect Magic, Light, Mage Armor (included in\
AC), Mage Hand, Prestidigitation\
2/Day Each: Fireball (level 4 version), Invisibility\
1/Day Each: Cone of Cold, Fly\

### Bonus Actions

Misty Step (3/Day). The mage casts Misty Step, using\
the same spellcasting ability as Spellcasting.\

### Reactions

Protective Magic (3/Day). The mage casts Counterspell\
or Shield in response to the spell’s trigger, using the\
same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/305|page 305]].
