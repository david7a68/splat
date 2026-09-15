---
splat_schema: 1
splat_id: aad2d730-166b-50f4-acd0-c355c6b3b9e5
splat_kind: creature
splat_key: dnd2024:creatures/roper
splat_name: Roper
splat_origin:
  key: dnd2024:reference/creature/roper
splat_sources:
- title: SRD 5.2.1
  locator: pp. 320
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: aberration
  alignment: Neutral Evil
  movement_text: 10 ft., Climb 20 ft.
  skills_text: Perception +6, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 16
  languages_text: None
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 93 (11d10 + 33)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 20
  max_hp:
    value: 93
  initiative:
    value: 5
  speed:
    value: 10
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/roper
  implementation: dnd2024:creature_roper
  parameters:
    features:
      spider_climb:
        economy: passive
        climb_ability_check_required: false
        surfaces:
        - difficult
        - ceiling
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 17
          amount:
            dice:
              count: 3
              sides: 8
            add: 4
          damage_type: piercing
        reach_feet: 5
      tentacle:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: condition
          condition: grappled
          escape_dc: 14
          source: self
          appendages:
            kind: tentacle
            available: 6
            used: 1
        - op: condition
          condition: poisoned
          ends:
            event: grapple.end
            source: self
        reach_feet: 60
        appendage:
          ac: 20
          hp: 10
          damage_immunities:
          - poison
          - psychic
          on_destroyed:
          - free_grappled_creature
          damage_transferred_to_self: false
          regrow:
            event: turn.start
            participant: self
            occurrence: next
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: tentacle
          count: 2
        - op: use_feature
          feature: reel
          count: 1
        - op: use_feature
          feature: bite
          count: 2
      reel:
        economy: action
        op: pull
        targets:
          count: all
          grappled_by: self
        distance_max_feet: 30
        direction: straight_toward_self
    skill_totals:
      perception: 6
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 16
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
- id: dnd2024:creature_roper
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
  - grapple.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - grappling
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 88c8b85a1319a8f6b0e75fc6b92c5667ed51145639ee988bc8ed413c5b4fc616
  source_path: builtin/srd-5.2.1/creature/roper.md
  pages:
  - 320
  engine_gaps: &id001
  - dnd2024:creature_roper
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 56da6f520278b51fc46c2e298c453fd9efb5a84c4bbf9338b4b9e8e99756c2ee
splat_clauses:
- key: source_rules
  text: '# Roper Large Aberration, Neutral Evil AC 20 Initiative +5 (15) HP 93 (11d10 + 33) Speed 10 ft.,
    Climb 20 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 8 -1 -1 Con 17 +3 +3 Int 7 -2 -2 Wis 16 +3
    +3 Cha 6 -2 -2 Skills Perception +6, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 16 Languages
    None CR 5 (XP 1,800; PB +3) ### Traits Spider Climb. The roper can climb difficult surfaces, including
    along ceilings, without needing to make an ability check. ### Actions Multiattack. The roper makes
    two Tentacle attacks, uses Reel, and makes two Bite attacks. Bite. Melee Attack Roll: +7, reach 5
    ft. Hit: 17 (3d8 + 4) Piercing damage. Tentacle. Melee Attack Roll: +7, reach 60 ft. Hit: The target
    has the Grappled condition (escape DC 14) from one of six tentacles, and the target has the Poisoned
    condition until the grapple ends. The tentacle can be damaged, freeing a creature it has Grappled
    when destroyed (AC 20, HP 10, Immunity to Poison and Psychic damage). Damaging the tentacle deals
    no damage to the roper, and a destroyed tentacle regrows at the start of the roper’s next turn. Reel.
    The roper pulls each creature Grappled by it up to 30 feet straight toward it.'
  sha256: 1c9014364052f94c08279d1d5794cb40b1b57f95d1bc026c9bf354f26e5561c2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Roper

<!-- SRD page 320 -->

Large Aberration, Neutral Evil\
AC 20  Initiative +5 (15)\
HP 93 (11d10 + 33)\
Speed 10 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 8 −1 −1 Con 17 +3 +3\
Int 7 −2 −2 Wis 16 +3 +3 Cha 6 −2 −2\
Skills Perception +6, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 16\
Languages None\
CR 5 (XP 1,800; PB +3)\

### Traits

Spider Climb. The roper can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Multiattack. The roper makes two Tentacle attacks,\
uses Reel, and makes two Bite attacks.\
Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 17 (3d8 + 4)\
Piercing damage.\
Tentacle. Melee Attack Roll: +7, reach 60 ft. Hit: The\
target has the Grappled condition (escape DC 14) from\
one of six tentacles, and the target has the Poisoned\
condition until the grapple ends.\
The tentacle can be damaged, freeing a creature it has\
Grappled when destroyed (AC 20, HP 10, Immunity to\
Poison and Psychic damage). Damaging the tentacle\
deals no damage to the roper, and a destroyed tentacle\
regrows at the start of the roper’s next turn.\
Reel. The roper pulls each creature Grappled by it up\
to 30 feet straight toward it.\

Original layout: [[../../srd-5.2.1/pages/320|page 320]].
