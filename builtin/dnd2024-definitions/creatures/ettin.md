---
splat_schema: 1
splat_id: fc14b342-51b7-59a4-9a62-b0680cb2fdd5
splat_kind: creature
splat_key: dnd2024:creatures/ettin
splat_name: Ettin
splat_origin:
  key: dnd2024:reference/creature/ettin
splat_sources:
- title: SRD 5.2.1
  locator: pp. 284
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: giant
  alignment: Chaotic Evil
  movement_text: 40 ft.
  skills_text: Perception +4
  immunities_text: Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious
  gear_text: Battleaxe, Morningstar
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Giant
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 85 (10d10 + 30)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 85
  initiative:
    value: -1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ettin
  implementation: dnd2024:creature_ettin
  parameters:
    features:
      battleaxe:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 14
          amount:
            dice:
              count: 2
              sides: 8
            add: 5
          damage_type: slashing
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: prone
        reach_feet: 5
      morningstar:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 14
          amount:
            dice:
              count: 2
              sides: 8
            add: 5
          damage_type: piercing
        - op: disadvantage
          roll: attack
          count: 1
          ends:
            event: turn.end
            participant: target
            occurrence: next
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: battleaxe
          count: 1
        - op: use_feature
          feature: morningstar
          count: 1
    skill_totals:
      perception: 4
    defenses:
      immunities:
        conditions:
        - blinded
        - charmed
        - deafened
        - frightened
        - stunned
        - unconscious
    senses:
      passive_perception: 14
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Battleaxe
      quantity: 1
    - item_name: Morningstar
      quantity: 1
    communication:
      speaks:
      - giant
      understands:
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
- id: dnd2024:creature_ettin
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
  - turn.end
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
  source_sha256: 4746642135bffe05aba4ae97dea132f388e4cebc4e5e1b9ef5a06a937e922c04
  source_path: builtin/srd-5.2.1/creature/ettin.md
  pages:
  - 284
  engine_gaps: &id001
  - dnd2024:creature_ettin
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 5163f59df108ebde49a82356fd02a695800d7a8f120c0ccd4224d62718291798
splat_clauses:
- key: source_rules
  text: '# Ettin Large Giant, Chaotic Evil AC 12 Initiative -1 (9) HP 85 (10d10 + 30) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 8 -1 -1 Con 17 +3 +3 Int 6 -2 -2 Wis 10 +0 +0 Cha 8 -1 -1
    Skills Perception +4 Immunities Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious Gear
    Battleaxe, Morningstar Senses Darkvision 60 ft.; Passive Perception 14 Languages Giant CR 4 (XP 1,100;
    PB +2) ### Actions Multiattack. The ettin makes one Battleaxe attack and one Morningstar attack. Battleaxe.
    Melee Attack Roll: +7, reach 5 ft. Hit: 14 (2d8 + 5) Slashing damage. If the target is a Large or
    smaller creature, it has the Prone condition. Morningstar. Melee Attack Roll: +7, reach 5 ft. Hit:
    14 (2d8 + 5) Piercing damage, and the target has Disadvantage on the next attack roll it makes before
    the end of its next turn.'
  sha256: b2cf33a53f885c0032b8ef1df48e1137667a50d807c45e72011f6b61adaea480
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ettin

<!-- SRD page 284 -->

Large Giant, Chaotic Evil\
AC 12  Initiative −1 (9)\
HP 85 (10d10 + 30)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 8 −1 −1 Con 17 +3 +3\
Int 6 −2 −2 Wis 10 +0 +0 Cha 8 −1 −1\
Skills Perception +4\
Immunities Blinded, Charmed, Deafened, Frightened,\
Stunned, Unconscious\
Gear Battleaxe, Morningstar\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Giant\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The ettin makes one Battleaxe attack and\
one Morningstar attack.\
Battleaxe. Melee Attack Roll: +7, reach 5 ft. Hit: 14\
(2d8 + 5) Slashing damage. If the target is a Large or\
smaller creature, it has the Prone condition.\
Morningstar. Melee Attack Roll: +7, reach 5 ft. Hit: 14\
(2d8 + 5) Piercing damage, and the target has Disad-\
vantage on the next attack roll it makes before the end\
of its next turn.\

Original layout: [[../../srd-5.2.1/pages/284|page 284]].
