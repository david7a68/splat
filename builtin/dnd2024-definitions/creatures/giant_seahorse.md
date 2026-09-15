---
splat_schema: 1
splat_id: 2c3d32aa-4010-5638-a265-553da3cc8030
splat_kind: creature
splat_key: dnd2024:creatures/giant_seahorse
splat_name: Giant Seahorse
splat_origin:
  key: dnd2024:reference/creature/giant_seahorse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 353
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 5 ft., Swim 40 ft.
  senses_text: Passive Perception 11
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 16 (3d10)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 11
  save_constitution:
    value: 0
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
    value: 14
  max_hp:
    value: 16
  initiative:
    value: 1
  speed:
    value: 5
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_seahorse
  implementation: dnd2024:creature_giant_seahorse
  parameters:
    features:
      water_breathing:
        economy: passive
        breathing_media:
        - water
      ram:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 4
        targets:
          count: 1
        hit:
        - op: choose_by_predicate
          test:
            immediately_preceding_movement:
              minimum_feet: 20
              direction: straight_toward_target
          then:
          - op: damage
            average: 11
            amount:
              dice:
                count: 2
                sides: 8
              add: 2
            damage_type: bludgeoning
          else:
          - op: damage
            average: 9
            amount:
              dice:
                count: 2
                sides: 6
              add: 2
            damage_type: bludgeoning
        reach_feet: 5
      bubble_dash:
        economy: bonus_action
        requires:
          underwater: true
        op: move
        distance_max:
          multiply:
          - stat: swim_speed
          - numerator: 1
            denominator: 2
        provokes_opportunity_attacks: false
    defenses: {}
    senses:
      passive_perception: 11
      special: []
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
- id: dnd2024:creature_giant_seahorse
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
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: c0e415697a0f66934d9432514817fe6a8123815a9cd0844cd160f75ab1ffb08e
  source_path: builtin/srd-5.2.1/creature/giant_seahorse.md
  pages:
  - 353
  engine_gaps: &id001
  - dnd2024:creature_giant_seahorse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: ba170fe78028a7f1e47584c5c8f901de4dbc084dbf44f125db6e48c07315656d
splat_clauses:
- key: source_rules
  text: '# Giant Seahorse Large Beast, Unaligned AC 14 Initiative +1 (11) HP 16 (3d10) Speed 5 ft., Swim
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 12 +1 +1 Con 11 +0 +0 Int 2 -4 -4 Wis 12 +1 +1
    Cha 5 -3 -3 Senses Passive Perception 11 Languages None CR 1/2 (XP 100; PB +2) ### Traits Water Breathing.
    The seahorse can breathe only underwater. ### Actions Ram. Melee Attack Roll: +4, reach 5 ft. Hit:
    9 (2d6 + 2) Bludgeoning damage, or 11 (2d8 + 2) Bludgeoning damage if the seahorse moved 20+ feet
    straight toward the target immediately before the hit. ### Bonus Actions Bubble Dash. While underwater,
    the seahorse moves up to half its Swim Speed without provoking Opportunity Attacks.'
  sha256: 6e85f7f740631d9252337e2591a0a2aae30a087415d26d9148c066ac8fcd86f8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Seahorse

<!-- SRD page 353 -->

Large Beast, Unaligned\
AC 14  Initiative +1 (11)\
HP 16 (3d10)\
Speed 5 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 12 +1 +1 Con 11 +0 +0\
Int 2 −4 −4 Wis 12 +1 +1 Cha 5 −3 −3\
Senses Passive Perception 11\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Water Breathing. The seahorse can breathe only\
underwater.\

### Actions

Ram. Melee Attack Roll: +4, reach 5 ft. Hit: 9 (2d6\
+ 2) Bludgeoning damage, or 11 (2d8 + 2) Bludgeoning\
damage if the seahorse moved 20+ feet straight toward\
the target immediately before the hit.\

### Bonus Actions

Bubble Dash. While underwater, the seahorse moves\
up to half its Swim Speed without provoking Opportu-\
nity Attacks.\

Original layout: [[../../srd-5.2.1/pages/353|page 353]].
