---
splat_schema: 1
splat_id: 247e28ca-ee08-53c5-846e-4fcd417019fd
splat_kind: creature
splat_key: dnd2024:creatures/hyena
splat_name: Hyena
splat_origin:
  key: dnd2024:reference/creature/hyena
splat_sources:
- title: SRD 5.2.1
  locator: pp. 356
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +3
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 5 (1d8 + 1)
splat_stats:
  strength:
    value: 11
  save_strength:
    value: 0
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 11
  max_hp:
    value: 5
  initiative:
    value: 1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hyena
  implementation: dnd2024:creature_hyena
  parameters:
    features:
      pack_tactics:
        economy: passive
        advantage:
          roll: attack
          target_kind: creature
          requires:
            ally_distance_to_target_max_feet: 5
            ally_incapacitated: false
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 2
        targets:
          count: 1
        hit:
        - op: damage
          average: 3
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          damage_type: piercing
        reach_feet: 5
    skill_totals:
      perception: 3
    defenses: {}
    senses:
      passive_perception: 13
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks: []
      understands: []
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
- id: dnd2024:creature_hyena
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
  events: []
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 2d459566b16fda54eba55115465f9b20bbc48ca07de093213933c42d83732979
  source_path: builtin/srd-5.2.1/creature/hyena.md
  pages:
  - 356
  engine_gaps: &id001
  - dnd2024:creature_hyena
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 58d200ebf092c7ee72f998a83b82b61153846f5d3fbbf1d0ce49f7e50c688778
splat_clauses:
- key: source_rules
  text: '# Hyena Medium Beast, Unaligned AC 11 Initiative +1 (11) HP 5 (1d8 + 1) Speed 50 ft. MOD SAVE
    MOD SAVE MOD SAVE Str 11 +0 +0 Dex 13 +1 +1 Con 12 +1 +1 Int 2 -4 -4 Wis 12 +1 +1 Cha 5 -3 -3 Skills
    Perception +3 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR 0 (XP 10; PB +2) ###
    Traits Pack Tactics. The hyena has Advantage on an attack roll against a creature if at least one
    of the hyena’s allies is within 5 feet of the creature and the ally doesn’t have the Incapacitated
    condition. ### Actions Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 3 (1d6) Piercing damage.'
  sha256: 0bf2283a41d4d4b9f0b7c740bc76c0247cf7c5f8543fcd0872e8e20cdfc26cc6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hyena

<!-- SRD page 356 -->

Medium Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 5 (1d8 + 1)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 11 +0 +0 Dex 13 +1 +1 Con 12 +1 +1\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Skills Perception +3\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Pack Tactics. The hyena has Advantage on an attack\
roll against a creature if at least one of the hyena’s allies\
is within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +2, reach 5 ft. Hit: 3 (1d6)\
Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/356|page 356]].
