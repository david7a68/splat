---
splat_schema: 1
splat_id: 7e9cb656-6ad5-5a5c-9c31-0effa594bc32
splat_kind: creature
splat_key: dnd2024:creatures/succubus
splat_name: Succubus
splat_origin:
  key: dnd2024:reference/creature/succubus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 331
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Neutral Evil
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7
  resistances_text: Cold, Fire, Poison, Psychic
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: Abyssal, Common, Infernal; telepathy 60 ft.
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 71 (13d8 + 13)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 15
  save_intelligence:
    value: 2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 15
  max_hp:
    value: 71
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/succubus
  implementation: dnd2024:creature_succubus
  parameters:
    features:
      fiendish_touch:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 16
          amount:
            dice:
              count: 2
              sides: 10
            add: 5
          damage_type: psychic
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: fiendish_touch
          count: 1
        - op: choose
          choices:
          - op: use_feature
            feature: charm
            count: 1
          - op: use_feature
            feature: draining_kiss
            count: 1
      charm:
        economy: action
        op: cast_spell
        spell:
          key: dnd2024:spells/dominate_person
        ability: charisma
        save_dc: 15
        components_waived:
        - verbal
        - somatic
        - material
        cast_level: 8
      draining_kiss:
        economy: action
        op: save
        ability: constitution
        dc: 15
        targets:
          kind: creature
          count: 1
          range_feet: 5
          charmed_by: self
        failure:
        - op: damage
          average: 13
          amount:
            dice:
              count: 3
              sides: 8
            add: 0
          damage_type: psychic
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
        after_save:
        - op: reduce_hit_point_maximum
          amount:
            event_value: damage_taken
      shape_shift:
        economy: bonus_action
        op: shape_shift
        forms:
        - form: humanoid
          size_choice:
          - medium
          - small
          fly_speed_available: false
        - form: 'true'
          fly_speed_available: true
        retain_statistics_except:
        - fly_speed
        equipment_transformed: false
      incubus_form:
        economy: passive
        trigger: self.long_rest.finished
        optional: true
        op: replace_stat_block
        definition:
          key: dnd2024:creatures/incubus
    skill_totals:
      deception: 9
      insight: 5
      perception: 5
      persuasion: 9
      stealth: 7
    defenses:
      resistances:
        damage_types:
        - cold
        - fire
        - poison
        - psychic
    senses:
      passive_perception: 15
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - abyssal
      - common
      - infernal
      understands:
      - abyssal
      - common
      - infernal
      telepathy:
        range_feet: 60
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
- id: dnd2024:creature_succubus
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
  - self.long_rest.finished
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - shapechanging
  - spellcasting
  - stat_block_replacement
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 2b4961508737adf7d9e522f7d8043240014d53cc27e6842c688ac7a481a801ec
  source_path: builtin/srd-5.2.1/creature/succubus.md
  pages:
  - 331
  engine_gaps: &id001
  - dnd2024:creature_succubus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: be827794a18cb3a0acf003220c930ba43d172cc0a171a581b8e929aeb5f76209
splat_clauses:
- key: source_rules
  text: '# Succubus Medium Fiend, Neutral Evil AC 15 Initiative +3 (13) HP 71 (13d8 + 13) Speed 30 ft.,
    Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 17 +3 +3 Con 13 +1 +1 Int 15 +2 +2 Wis 12 +1
    +1 Cha 20 +5 +5 Skills Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7 Resistances
    Cold, Fire, Poison, Psychic Senses Darkvision 60 ft.; Passive Perception 15 Languages Abyssal, Common,
    Infernal; telepathy 60 ft. CR 4 (XP 1,100; PB +2) ### Traits Incubus Form. When the succubus finishes
    a Long Rest, it can shape-shift into an Incubus, using that stat block instead of this one. ### Actions
    Multiattack. The succubus makes one Fiendish Touch attack and uses Charm or Draining Kiss. Fiendish
    Touch. Melee Attack Roll: +7, reach 5 ft. Hit: 16 (2d10 + 5) Psychic damage. Charm. The succubus casts
    Dominate Person (level 8 version), requiring no spell components and using Charisma as the spellcasting
    ability (spell save DC 15). Draining Kiss. Constitution Saving Throw: DC 15, one creature Charmed
    by the succubus within 5 feet. Failure: 13 (3d8) Psychic damage. Success: Half damage. Failure or
    Success: The target’s Hit Point maximum decreases by an amount equal to the damage taken. ### Bonus
    Actions Shape-Shift. The succubus shape-shifts into a Medium or Small Humanoid, or it returns to its
    true form. Its game statistics are the same in each form, except its Fly Speed is available only in
    its true form. Any equipment it is wearing or carrying isn’t transformed.'
  sha256: 864d24d35442d3e80944626122eb26952716678c2b7cbc63ac18542b37ee6304
  status: contract_specified
splat_requires_mechanics: *id001
---

# Succubus

<!-- SRD page 331 -->

Medium Fiend, Neutral Evil\
AC 15 Initiative +3 (13)\
HP 71 (13d8 + 13)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 17 +3 +3 Con 13 +1 +1\
Int 15 +2 +2 Wis 12 +1 +1 Cha 20 +5 +5\
Skills Deception +9, Insight +5, Perception +5, Persuasion\
+9, Stealth +7\
Resistances Cold, Fire, Poison, Psychic\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Abyssal, Common, Infernal; telepathy 60 ft.\
CR 4 (XP 1,100; PB +2)\

### Traits

Incubus Form. When the succubus finishes a Long\
Rest, it can shape-shift into an Incubus, using that stat\
block instead of this one.\

### Actions

Multiattack. The succubus makes one Fiendish Touch\
attack and uses Charm or Draining Kiss.\
Fiendish Touch. Melee Attack Roll: +7, reach 5 ft. Hit:\
16 (2d10 + 5) Psychic damage.\
Charm. The succubus casts Dominate Person (level 8\
version), requiring no spell components and using Cha-\
risma as the spellcasting ability (spell save DC 15).\
Draining Kiss. Constitution Saving Throw: DC 15, one\
creature Charmed by the succubus within 5 feet. Fail-\
ure: 13 (3d8) Psychic damage. Success: Half damage.\
Failure or Success: The target’s Hit Point maximum de-\
creases by an amount equal to the damage taken.\

### Bonus Actions

Shape-Shift. The succubus shape-shifts into a Medium\
or Small Humanoid, or it returns to its true form. Its\
game statistics are the same in each form, except its Fly\
Speed is available only in its true form. Any equipment\
it is wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/331|page 331]].
