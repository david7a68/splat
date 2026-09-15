---
splat_schema: 1
splat_id: d8c7cab5-1a7f-5f44-99f9-2e2de6e3a51d
splat_kind: creature
splat_key: dnd2024:creatures/dire_wolf
splat_name: Dire Wolf
splat_origin:
  key: dnd2024:reference/creature/dire_wolf
splat_sources:
- title: SRD 5.2.1
  locator: pp. 347
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +5, Stealth +4
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: None
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (3d10 + 6)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 22
  initiative:
    value: 2
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/dire_wolf
  implementation: dnd2024:creature_dire_wolf
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
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 10
            add: 3
          damage_type: piercing
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: prone
        reach_feet: 5
    skill_totals:
      perception: 5
      stealth: 4
    defenses: {}
    senses:
      passive_perception: 15
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
- id: dnd2024:creature_dire_wolf
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
  source_sha256: d481ba6f9732987688748997f115bbab4f77b2e612fada174e5a8bb107f72957
  source_path: builtin/srd-5.2.1/creature/dire_wolf.md
  pages:
  - 347
  engine_gaps: &id001
  - dnd2024:creature_dire_wolf
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d86f2052a6a6bf82f4873a32d4fde6d00c5c14be33c2367dcdbab1c9297034c8
splat_clauses:
- key: source_rules
  text: '# Dire Wolf Large Beast, Unaligned AC 14 Initiative +2 (12) HP 22 (3d10 + 6) Speed 50 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 15 +2 +2 Con 15 +2 +2 Int 3 -4 -4 Wis 12 +1 +1 Cha 7 -2 -2
    Skills Perception +5, Stealth +4 Senses Darkvision 60 ft.; Passive Perception 15 Languages None CR
    1 (XP 200; PB +2) ### Traits Pack Tactics. The wolf has Advantage on an attack roll against a creature
    if at least one of the wolf’s allies is within 5 feet of the creature and the ally doesn’t have the
    Incapacitated condition. ### Actions Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Piercing
    damage. If the target is a Large or smaller creature, it has the Prone condition.'
  sha256: ea6aed0c05a9feb63ae9ebbd7ba1087b84ba3c3830787382703291a654e355e4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dire Wolf

<!-- SRD page 347 -->

Large Beast, Unaligned\
AC 14  Initiative +2 (12)\
HP 22 (3d10 + 6)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 15 +2 +2 Con 15 +2 +2\
Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +5, Stealth +4\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages None\
CR 1 (XP 200; PB +2)\

### Traits

Pack Tactics. The wolf has Advantage on an attack roll\
against a creature if at least one of the wolf’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3)\
Piercing damage. If the target is a Large or smaller crea-\
ture, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/347|page 347]].
