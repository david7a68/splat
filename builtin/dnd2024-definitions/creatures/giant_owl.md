---
splat_schema: 1
splat_id: d7f4119e-2381-5843-a7a9-5a90087a19fe
splat_kind: creature
splat_key: dnd2024:creatures/giant_owl
splat_name: Giant Owl
splat_origin:
  key: dnd2024:reference/creature/giant_owl
splat_sources:
- title: SRD 5.2.1
  locator: pp. 352
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: celestial
  alignment: Neutral
  movement_text: 5 ft., Fly 60 ft.
  skills_text: Perception +6, Stealth +6
  resistances_text: Necrotic, Radiant
  senses_text: Darkvision 120 ft.; Passive Perception 16
  languages_text: Celestial; understands Common, Elvish, and Sylvan but can’t speak them
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 19 (3d10 + 3)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 4
  charisma:
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 19
  initiative:
    value: 2
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_owl
  implementation: dnd2024:creature_giant_owl
  parameters:
    features:
      flyby:
        economy: passive
        opportunity_attack_immunity:
          trigger: fly_out_of_enemy_reach
      talons:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 10
            add: 2
          damage_type: slashing
        reach_feet: 5
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: wisdom
        spells:
        - spell:
            key: dnd2024:spells/detect_evil_and_good
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/clairvoyance
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - verbal
        - somatic
        - material
    skill_totals:
      perception: 6
      stealth: 6
    defenses:
      resistances:
        damage_types:
        - necrotic
        - radiant
    senses:
      passive_perception: 16
      special:
      - sense: darkvision
        range_feet: 120
    communication:
      speaks:
      - celestial
      understands:
      - celestial
      - common
      - elvish
      - sylvan
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_giant_owl
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - fly_out_of_enemy_reach
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - movement
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6980860ca2d80b402ef482f44c4f063a506b87d95501946e9e085d5d03fe2db6
  source_path: builtin/srd-5.2.1/creature/giant_owl.md
  pages:
  - 352
  engine_gaps: &id001
  - dnd2024:creature_giant_owl
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: c7a2fe3229efc1054f4aaa55d5b5dd09e07ecc59cd5fbfe810ab1ead97adfaa5
splat_clauses:
- key: source_rules
  text: '# Giant Owl Large Celestial, Neutral AC 12 Initiative +2 (12) HP 19 (3d10 + 3) Speed 5 ft., Fly
    60 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 15 +2 +2 Con 12 +1 +1 Int 10 +0 +0 Wis 14 +2 +4
    Cha 10 +0 +0 Skills Perception +6, Stealth +6 Resistances Necrotic, Radiant Senses Darkvision 120
    ft.; Passive Perception 16 Languages Celestial; understands Common, Elvish, and Sylvan but can’t speak
    them CR 1/4 (XP 50; PB +2) ### Traits Flyby. The owl doesn’t provoke an Opportunity Attack when it
    flies out of an enemy’s reach. ### Actions Talons. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10
    + 2) Slashing damage. Spellcasting. The owl casts one of the following spells, requiring no spell
    components and using Wisdom as the spellcasting ability: At Will: Detect Evil and Good, Detect Magic
    1/Day: Clairvoyance'
  sha256: aade668a67f7282b37ffa86188c3a505dbda0b20d20c015fc2236a9e3af308c9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Owl

<!-- SRD page 352 -->

Large Celestial, Neutral\
AC 12 Initiative +2 (12)\
HP 19 (3d10 + 3)\
Speed 5 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 15 +2 +2 Con 12 +1 +1\
Int 10 +0 +0 Wis 14 +2 +4 Cha 10 +0 +0\
Skills Perception +6, Stealth +6\
Resistances Necrotic, Radiant\
Senses Darkvision 120 ft.; Passive Perception 16\
Languages Celestial; understands Common, Elvish, and\
Sylvan but can’t speak them\
CR 1/4 (XP 50; PB +2)\

### Traits

Flyby. The owl doesn’t provoke an Opportunity Attack\
when it flies out of an enemy’s reach.\

### Actions

Talons. Melee Attack Roll: +4, reach 5 ft. Hit: 7 (1d10\
+ 2) Slashing damage.\
Spellcasting. The owl casts one of the following spells,\
requiring no spell components and using Wisdom as\
the spellcasting ability:\
At Will: Detect Evil and Good, Detect Magic\
1/Day: Clairvoyance\

Original layout: [[../../srd-5.2.1/pages/352|page 352]].
