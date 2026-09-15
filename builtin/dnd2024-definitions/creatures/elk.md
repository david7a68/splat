---
splat_schema: 1
splat_id: 837bc152-ee71-5eff-bc33-6edc96f9beab
splat_kind: creature
splat_key: dnd2024:creatures/elk
splat_name: Elk
splat_origin:
  key: dnd2024:reference/creature/elk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 348
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 50 ft.
  skills_text: Perception +2
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d10)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 10
  max_hp:
    value: 11
  initiative:
    value: 0
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/elk
  implementation: dnd2024:creature_elk
  parameters:
    features:
      ram:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: large
            immediately_preceding_movement:
              minimum_feet: 20
              direction: straight_toward_target
          then:
          - op: damage
            average: 3
            amount:
              dice:
                count: 1
                sides: 6
              add: 0
            damage_type: bludgeoning
          - op: condition
            condition: prone
        reach_feet: 5
    skill_totals:
      perception: 2
    defenses: {}
    senses:
      passive_perception: 12
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
- id: dnd2024:creature_elk
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
  source_sha256: 4de3cc7c562f5a7c7a3727849b72d7c52dc7c81744e413ef48926cf10df3a6c1
  source_path: builtin/srd-5.2.1/creature/elk.md
  pages:
  - 348
  engine_gaps: &id001
  - dnd2024:creature_elk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 19d385a3efd03391dbfc1a04a8127d5f98372edb473a6f2ed98f6506edf6d25b
splat_clauses:
- key: source_rules
  text: '# Elk Large Beast, Unaligned AC 10 Initiative +0 (10) HP 11 (2d10) Speed 50 ft. MOD SAVE MOD
    SAVE MOD SAVE Str 16 +3 +3 Dex 10 +0 +0 Con 11 +0 +0 Int 2 -4 -4 Wis 10 +0 +0 Cha 6 -2 -2 Skills Perception
    +2 Senses Darkvision 60 ft.; Passive Perception 12 Languages None CR 1/4 (XP 50; PB +2) ### Actions
    Ram. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Bludgeoning damage. If the target is a Large
    or smaller creature and the elk moved 20+ feet straight toward it immediately before the hit, the
    target takes an extra 3 (1d6) Bludgeoning damage and has the Prone condition.'
  sha256: 3ab55b5a5c26ef5ff20559216c50a453320f154148eab276c78fa5f826bc7ba8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Elk

<!-- SRD page 348 -->

Large Beast, Unaligned\
AC 10  Initiative +0 (10)\
HP 11 (2d10)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 10 +0 +0 Con 11 +0 +0\
Int 2 −4 −4 Wis 10 +0 +0 Cha 6 −2 −2\
Skills Perception +2\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Ram. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Bludgeoning damage. If the target is a Large or\
smaller creature and the elk moved 20+ feet straight\
toward it immediately before the hit, the target takes an\
extra 3 (1d6) Bludgeoning damage and has the Prone\
condition.\

Original layout: [[../../srd-5.2.1/pages/348|page 348]].
