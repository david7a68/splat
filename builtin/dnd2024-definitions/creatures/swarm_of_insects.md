---
splat_schema: 1
splat_id: e2376c5f-c7b9-5a91-aab6-787c9980d3f7
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_insects
splat_name: Swarm of Insects
splat_origin:
  key: dnd2024:reference/creature/swarm_of_insects
splat_sources:
- title: SRD 5.2.1
  locator: pp. 361
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  size_options_text: Medium Swarm of Tiny
  tags_text: s
  movement_text: 20 ft., Climb or Fly 20 ft. (GM’s choice)
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned
  senses_text: Blindsight 30 ft.; Passive Perception 8
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 19 (3d8 + 6)
splat_stats:
  strength:
    value: 3
  save_strength:
    value: -4
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 7
  save_wisdom:
    value: -2
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/swarm_of_insects
  implementation: dnd2024:creature_swarm_of_insects
  parameters:
    features:
      spider_climb:
        economy: passive
        climb_ability_check_required: false
        surfaces:
        - difficult
        - ceiling
        requires:
          has_climb_speed: true
      swarm:
        economy: passive
        space_sharing: bidirectional
        minimum_passage_size: tiny
        regain_hit_points: false
        gain_temporary_hit_points: false
      bites:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: choose_by_predicate
          test:
            self_bloodied: true
          then:
          - op: damage
            average: 3
            amount:
              dice:
                count: 1
                sides: 4
              add: 1
            damage_type: poison
          else:
          - op: damage
            average: 6
            amount:
              dice:
                count: 2
                sides: 4
              add: 1
            damage_type: poison
        reach_feet: 5
    defenses:
      resistances:
        damage_types:
        - bludgeoning
        - piercing
        - slashing
      immunities:
        conditions:
        - charmed
        - frightened
        - grappled
        - paralyzed
        - petrified
        - prone
        - restrained
        - stunned
    senses:
      passive_perception: 8
      special:
      - sense: blindsight
        range_feet: 30
    communication:
      speaks: []
      understands: []
    movement_variant:
      chooser: gm
      choose_one:
      - climb_speed_feet: 20
      - fly_speed_feet: 20
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
- id: dnd2024:creature_swarm_of_insects
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
  - grappling
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: de8e0220562ac27bafcd352dce99df0c0e5ca24bd1bbaea1daab3706534d6a75
  source_path: builtin/srd-5.2.1/creature/swarm_of_insects.md
  pages:
  - 361
  engine_gaps: &id001
  - dnd2024:creature_swarm_of_insects
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 41a07d7350897dbfc4adfa5628e5b15ffa8786eeb59cf8fbc78011e5334a2449
splat_clauses:
- key: source_rules
  text: '# Swarm of Insects Medium Swarm of Tiny Beasts, Unaligned AC 11 Initiative +1 (11) HP 19 (3d8
    + 6) Speed 20 ft., Climb or Fly 20 ft. (GM’s choice) MOD SAVE MOD SAVE MOD SAVE Str 3 -4 -4 Dex 13
    +1 +1 Con 14 +2 +2 Int 1 -5 -5 Wis 7 -2 -2 Cha 1 -5 -5 Resistances Bludgeoning, Piercing, Slashing
    Immunities Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned Senses
    Blindsight 30 ft.; Passive Perception 8 Languages None CR 1/2 (XP 100; PB +2) ### Traits Spider Climb.
    If the swarm has a Climb Speed, the swarm can climb difficult surfaces, including along ceilings,
    without needing to make an ability check. Swarm. The swarm can occupy another creature’s space and
    vice versa, and the swarm can move through any opening large enough for a Tiny insect. The swarm can’t
    regain Hit Points or gain Temporary Hit Points. ### Actions Bites. Melee Attack Roll: +3, reach 5
    ft. Hit: 6 (2d4 + 1) Poison damage, or 3 (1d4 + 1) Poison damage if the swarm is Bloodied.'
  sha256: 06517a72265ccfa282e5396e30fe47d016943708aabc5f131d055b4158597b2f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swarm of Insects

<!-- SRD page 361 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 11 Initiative +1 (11)\
HP 19 (3d8 + 6)\
Speed 20 ft., Climb or Fly 20 ft. (GM’s choice)\
MOD SAVE MOD SAVE MOD SAVE\
Str 3 −4 −4 Dex 13 +1 +1 Con 14 +2 +2\
Int 1 −5 −5 Wis 7 −2 −2 Cha 1 −5 −5\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Blindsight 30 ft.; Passive Perception 8\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Spider Climb. If the swarm has a Climb Speed, the\
swarm can climb difficult surfaces, including along ceil-\
ings, without needing to make an ability check.\
Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny insect. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Bites. Melee Attack Roll: +3, reach 5 ft. Hit: 6 (2d4 + 1)\
Poison damage, or 3 (1d4 + 1) Poison damage if the\
swarm is Bloodied.\

Original layout: [[../../srd-5.2.1/pages/361|page 361]].
