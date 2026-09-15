---
splat_schema: 1
splat_id: b02d5610-4e8b-510d-9951-89f98e719302
splat_kind: creature
splat_key: dnd2024:creatures/winter_wolf
splat_name: Winter Wolf
splat_origin:
  key: dnd2024:reference/creature/winter_wolf
splat_sources:
- title: SRD 5.2.1
  locator: pp. 342
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: monstrosity
  alignment: Neutral Evil
  movement_text: 50 ft.
  skills_text: Perception +5, Stealth +5
  immunities_text: Cold
  senses_text: Passive Perception 15
  languages_text: Common, Giant
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 75 (10d10 + 20)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 75
  initiative:
    value: 1
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/winter_wolf
  implementation: dnd2024:creature_winter_wolf
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
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 11
          amount:
            dice:
              count: 2
              sides: 6
            add: 4
          damage_type: piercing
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: prone
        reach_feet: 5
      cold_breath:
        economy: action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: constitution
        dc: 12
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 15
            origin: self
        failure:
        - op: damage
          average: 18
          amount:
            dice:
              count: 4
              sides: 8
            add: 0
          damage_type: cold
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
    skill_totals:
      perception: 5
      stealth: 5
    defenses:
      immunities:
        damage_types:
        - cold
    senses:
      passive_perception: 15
      special: []
    communication:
      speaks:
      - common
      - giant
      understands:
      - common
      - giant
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
- id: dnd2024:creature_winter_wolf
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
  - turn.start
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
  source_sha256: c7697ff3b48468baccea19527e20d694354daf4d3153d73eeb1d948f71c9cc37
  source_path: builtin/srd-5.2.1/creature/winter_wolf.md
  pages:
  - 342
  engine_gaps: &id001
  - dnd2024:creature_winter_wolf
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 7318095ece56ddf36e1c115722637fa1e90c38ea9d400ed6dc6a05ce3f4effb3
splat_clauses:
- key: source_rules
  text: '# Winter Wolf Large Monstrosity, Neutral Evil AC 13 Initiative +1 (11) HP 75 (10d10 + 20) Speed
    50 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 13 +1 +1 Con 14 +2 +2 Int 7 -2 -2 Wis 12 +1 +1
    Cha 8 -1 -1 Skills Perception +5, Stealth +5 Immunities Cold Senses Passive Perception 15 Languages
    Common, Giant CR 3 (XP 700; PB +2) ### Traits Pack Tactics. The wolf has Advantage on an attack roll
    against a creature if at least one of the wolf’s allies is within 5 feet of the creature and the ally
    doesn’t have the Incapacitated condition. ### Actions Bite. Melee Attack Roll: +6, reach 5 ft. Hit:
    11 (2d6 + 4) Piercing damage. If the target is a Large or smaller creature, it has the Prone condition.
    Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 12, each creature in a 15-foot Cone. Failure:
    18 (4d8) Cold damage. Success: Half damage.'
  sha256: 64c16d9b372eb855c8b4872d8cff7ec1692c138e50c77c8a5e0ad8554529eebe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Winter Wolf

<!-- SRD page 342 -->

Large Monstrosity, Neutral Evil\
AC 13 Initiative +1 (11)\
HP 75 (10d10 + 20)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 13 +1 +1 Con 14 +2 +2\
Int 7 −2 −2 Wis 12 +1 +1 Cha 8 −1 −1\
Skills Perception +5, Stealth +5\
Immunities Cold\
Senses Passive Perception 15\
Languages Common, Giant\
CR 3 (XP 700; PB +2)\

### Traits

Pack Tactics. The wolf has Advantage on an attack roll\
against a creature if at least one of the wolf’s allies is\
within 5 feet of the creature and the ally doesn’t have\
the Incapacitated condition.\

### Actions

Bite. Melee Attack Roll: +6, reach 5 ft. Hit: 11 (2d6 + 4)\
Piercing damage. If the target is a Large or smaller crea-\
ture, it has the Prone condition.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 12, each creature in a 15-foot Cone. Failure:\
18 (4d8) Cold damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/342|page 342]].
