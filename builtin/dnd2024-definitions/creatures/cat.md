---
splat_schema: 1
splat_id: 16552e43-3ddf-5ebd-a18e-b9ff80cedf44
splat_kind: creature
splat_key: dnd2024:creatures/cat
splat_name: Cat
splat_origin:
  key: dnd2024:reference/creature/cat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 346
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft.
  skills_text: Perception +3, Stealth +4
  senses_text: Darkvision 60 ft.; Passive Perception 13
  languages_text: None
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 2 (1d4)
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 15
  save_dexterity:
    value: 4
  constitution:
    value: 10
  save_constitution:
    value: 0
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
    value: 12
  max_hp:
    value: 2
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/cat
  implementation: dnd2024:creature_cat
  parameters:
    features:
      scratch:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: damage
          average: 1
          amount:
            constant: 1
          damage_type: slashing
        reach_feet: 5
      jumper:
        economy: passive
        jump_distance_ability: dexterity
    skill_totals:
      perception: 3
      stealth: 4
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
- id: dnd2024:creature_cat
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
  source_sha256: 62f069fc1eff16b90a8e1378c0d8649c0b2540e9dae4e7864e3f348873038e1d
  source_path: builtin/srd-5.2.1/creature/cat.md
  pages:
  - 346
  engine_gaps: &id001
  - dnd2024:creature_cat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 1fea583ee7ea1044bae80f4b22d9324bce5efb6ae1c47e9e412e82f75b39c7be
splat_clauses:
- key: source_rules
  text: '# Cat Tiny Beast, Unaligned AC 12 Initiative +2 (12) HP 2 (1d4) Speed 40 ft., Climb 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 3 -4 -4 Dex 15 +2 +4 Con 10 +0 +0 Int 3 -4 -4 Wis 12 +1 +1 Cha 7 -2 -2
    Skills Perception +3, Stealth +4 Senses Darkvision 60 ft.; Passive Perception 13 Languages None CR
    0 (XP 10; PB +2) ### Traits Jumper. The cat’s jump distance is determined using its Dexterity rather
    than its Strength. ### Actions Scratch. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Slashing damage.'
  sha256: 18f4e6891804369a1e388774da423214fd41b9f545525d9992d88810c1ed4542
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cat

<!-- SRD page 346 -->

Tiny Beast, Unaligned\
AC 12  Initiative +2 (12)\
HP 2 (1d4)\
Speed 40 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 15 +2 +4 Con 10 +0 +0\
Int 3 −4 −4 Wis 12 +1 +1 Cha 7 −2 −2\
Skills Perception +3, Stealth +4\
Senses Darkvision 60 ft.; Passive Perception 13\
Languages None\
CR 0 (XP 10; PB +2)\

### Traits

Jumper. The cat’s jump distance is determined using its\
Dexterity rather than its Strength.\

### Actions

Scratch. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Slash-\
ing damage.\

Original layout: [[../../srd-5.2.1/pages/346|page 346]].
