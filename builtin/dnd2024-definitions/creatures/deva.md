---
splat_schema: 1
splat_id: 0f3a54cb-d789-586c-8fa8-eb65317e6314
splat_kind: creature
splat_key: dnd2024:creatures/deva
splat_name: Deva
splat_origin:
  key: dnd2024:reference/creature/deva
splat_sources:
- title: SRD 5.2.1
  locator: pp. 279
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: celestial
  alignment: Lawful Good
  tags_text: (Angel)
  movement_text: 30 ft., Fly 90 ft. (hover)
  skills_text: Insight +9, Perception +9
  resistances_text: Radiant
  immunities_text: Charmed, Exhaustion, Frightened
  senses_text: Darkvision 120 ft.; Passive Perception 19
  languages_text: All; telepathy 120 ft.
  cr_text: 10 (XP 5,900; PB +4)
  hit_points_text: 229 (27d8 + 108)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 18
  save_dexterity:
    value: 4
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 17
  save_intelligence:
    value: 3
  wisdom:
    value: 20
  save_wisdom:
    value: 9
  charisma:
    value: 20
  save_charisma:
    value: 9
  armor_class:
    value: 17
  max_hp:
    value: 229
  initiative:
    value: 4
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/deva
  implementation: dnd2024:creature_deva
  parameters:
    features:
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
        - operation: attack_sequence
          feature: holy_mace
          count: 2
      holy_mace:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 8
          reach_ft: 5
        hit:
          damage:
          - average: 7
            roll:
              count: 1
              sides: 6
              modifier: 4
            type: bludgeoning
          - average: 18
            roll:
              count: 4
              sides: 8
              modifier: 0
            type: radiant
      spellcasting:
        economy: action
        operation: spellcasting
        ability: charisma
        components_waived:
        - material
        save_dc: 17
        options:
        - spell:
            key: dnd2024:spells/detect_evil_and_good
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
            key: dnd2024:spells/commune
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
            key: dnd2024:spells/lesser_restoration
        - operation: cast_spell
          spell:
            key: dnd2024:spells/remove_curse
        ability: charisma
    skill_totals:
      insight: 9
      perception: 9
    damage_resistances:
    - radiant
    damage_immunities: []
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    senses:
      darkvision: 120
      passive_perception: 19
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
- id: dnd2024:creature_deva
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
  source_sha256: 46bf94357eb2c71406b8be9a8837b01de4f88faa445dbc111eb528697540de88
  source_path: builtin/srd-5.2.1/creature/deva.md
  pages:
  - 279
  engine_gaps: &id001
  - dnd2024:creature_deva
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: fb20ee50dead79bc9e3b04d3dcb7f306dbbdd7cec0fd000537495d106c0074f1
splat_clauses:
- key: source_rules
  text: '# Deva Medium Celestial (Angel), Lawful Good AC 17 Initiative +4 (14) HP 229 (27d8 + 108) Speed
    30 ft., Fly 90 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 18 +4 +4 Con 18 +4 +4 Int 17
    +3 +3 Wis 20 +5 +9 Cha 20 +5 +9 Skills Insight +9, Perception +9 Resistances Radiant Immunities Charmed,
    Exhaustion, Frightened Senses Darkvision 120 ft.; Passive Perception 19 Languages All; telepathy 120
    ft. CR 10 (XP 5,900; PB +4) ### Traits Exalted Restoration. If the deva dies outside Mount Celestia,
    its body disappears, and it gains a new body instantly, reviving with all its Hit Points somewhere
    in Mount Celestia. Magic Resistance. The deva has Advantage on saving throws against spells and other
    magical effects. ### Actions Multiattack. The deva makes two Holy Mace attacks. Holy Mace. Melee Attack
    Roll: +8, reach 5 ft. Hit: 7 (1d6 + 4) Bludgeoning damage plus 18 (4d8) Radiant damage. Spellcasting.
    The deva casts one of the following spells, requiring no Material components and using Charisma as
    the spellcasting ability (spell save DC 17): At Will: Detect Evil and Good, Shapechange (Beast or
    Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration or Temporary
    Hit Points required to maintain the spell) 1/Day Each: Commune, Raise Dead ### Bonus Actions Divine
    Aid (2/Day). The deva casts Cure Wounds, Lesser Restoration, or Remove Curse, using the same spellcasting
    ability as Spellcasting.'
  sha256: bf78d1fd26b88e5aef39ce11bfe2390128e9dec232dfd395fa46af7feaf26400
  status: contract_specified
splat_requires_mechanics: *id001
---

# Deva

<!-- SRD page 279 -->

Medium Celestial (Angel), Lawful Good\
AC 17  Initiative +4 (14)\
HP 229 (27d8 + 108)\
Speed 30 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 18 +4 +4 Con 18 +4 +4\
Int 17 +3 +3 Wis 20 +5 +9 Cha 20 +5 +9\
Skills Insight +9, Perception +9\
Resistances Radiant\
Immunities Charmed, Exhaustion, Frightened\
Senses Darkvision 120 ft.; Passive Perception 19\
Languages All; telepathy 120 ft.\
CR 10 (XP 5,900; PB +4)\

### Traits

Exalted Restoration. If the deva dies outside Mount\
Celestia, its body disappears, and it gains a new body\
instantly, reviving with all its Hit Points somewhere in\
Mount Celestia.\
Magic Resistance. The deva has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The deva makes two Holy Mace attacks.\
Holy Mace. Melee Attack Roll: +8, reach 5 ft. Hit:\
7 (1d6 + 4) Bludgeoning damage plus 18 (4d8) Radi-\
ant damage.\
Spellcasting. The deva casts one of the following spells,\
requiring no Material components and using Charisma\
as the spellcasting ability (spell save DC 17):\
At Will: Detect Evil and Good, Shapechange (Beast or\
Humanoid form only, no Temporary Hit Points gained\
from the spell, and no Concentration or Temporary\
Hit Points required to maintain the spell)\
1/Day Each: Commune, Raise Dead\

### Bonus Actions

Divine Aid (2/Day). The deva casts Cure Wounds,\
Lesser Restoration, or Remove Curse, using the same\
spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/279|page 279]].
