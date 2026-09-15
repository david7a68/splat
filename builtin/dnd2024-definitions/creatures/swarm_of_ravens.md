---
splat_schema: 1
splat_id: 8533ab51-d920-544c-ac08-8d714f8de7fe
splat_kind: creature
splat_key: dnd2024:creatures/swarm_of_ravens
splat_name: Swarm of Ravens
splat_origin:
  key: dnd2024:reference/creature/swarm_of_ravens
splat_sources:
- title: SRD 5.2.1
  locator: pp. 362
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  size_options_text: Medium Swarm of Tiny
  tags_text: s
  movement_text: 10 ft., Fly 50 ft.
  skills_text: Perception +5
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned
  senses_text: Passive Perception 15
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 6
  save_strength:
    value: -2
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 12
  max_hp:
    value: 11
  initiative:
    value: 2
  speed:
    value: 10
    unit: ft
  fly_speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/swarm_of_ravens
  implementation: dnd2024:creature_swarm_of_ravens
  parameters:
    features:
      swarm:
        economy: passive
        space_sharing: bidirectional
        minimum_passage_size: tiny
        regain_hit_points: false
        gain_temporary_hit_points: false
      beaks:
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
            self_bloodied: true
          then:
          - op: damage
            average: 2
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            damage_type: piercing
          else:
          - op: damage
            average: 5
            amount:
              dice:
                count: 1
                sides: 6
              add: 2
            damage_type: piercing
        reach_feet: 5
      cacophony:
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
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: wisdom
        dc: 10
        targets:
          kind: creature
          count: 1
          location: self_space
        failure:
        - op: condition
          condition: deafened
          ends:
            event: turn.start
            participant: self
            occurrence: next
          while_active:
            disadvantage:
            - ability_check
            - attack
        success: []
    skill_totals:
      perception: 5
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
      passive_perception: 15
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
- id: dnd2024:creature_swarm_of_ravens
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
  - grappling
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: cd099394d5250683f7fba31d84d6f8e0085aba8272342024764dc3a83c19bb24
  source_path: builtin/srd-5.2.1/creature/swarm_of_ravens.md
  pages:
  - 362
  engine_gaps: &id001
  - dnd2024:creature_swarm_of_ravens
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: fcacdd36228aa5353ab21caded8ac6b55c2120c1c6264eada9a444ea81ef0a84
splat_clauses:
- key: source_rules
  text: '# Swarm of Ravens Medium Swarm of Tiny Beasts, Unaligned AC 12 Initiative +2 (12) HP 11 (2d8
    + 2) Speed 10 ft., Fly 50 ft. MOD SAVE MOD SAVE MOD SAVE Str 6 -2 -2 Dex 14 +2 +2 Con 12 +1 +1 Int
    5 -3 -3 Wis 12 +1 +1 Cha 6 -2 -2 Skills Perception +5 Resistances Bludgeoning, Piercing, Slashing
    Immunities Charmed, Frightened, Grappled, Paralyzed, Petrified, Prone, Restrained, Stunned Senses
    Passive Perception 15 Languages None CR 1/4 (XP 50; PB +2) ### Traits Swarm. The swarm can occupy
    another creature’s space and vice versa, and the swarm can move through any opening large enough for
    a Tiny raven. The swarm can’t regain Hit Points or gain Temporary Hit Points. ### Actions Beaks. Melee
    Attack Roll: +4, reach 5 ft. Hit: 5 (1d6 + 2) Piercing damage, or 2 (1d4) Piercing damage if the swarm
    is Bloodied. Cacophony (Recharge 6). Wisdom Saving Throw: DC 10, one creature in the swarm’s space.
    Failure: The target has the Deafened condition until the start of the swarm’s next turn. While Deafened,
    the target also has Disadvantage on ability checks and attack rolls.'
  sha256: 7c9a851104d2b97f041aadd8239ea1a169ddd82384da762f568d46088968c075
  status: contract_specified
splat_requires_mechanics: *id001
---

# Swarm of Ravens

<!-- SRD page 362 -->

Medium Swarm of Tiny Beasts, Unaligned\
AC 12 Initiative +2 (12)\
HP 11 (2d8 + 2)\
Speed 10 ft., Fly 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 6 −2 −2 Dex 14 +2 +2 Con 12 +1 +1\
Int  5 −3 −3 Wis 12 +1 +1 Cha 6 −2 −2\
Skills Perception +5\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Charmed, Frightened, Grappled, Paralyzed,\
Petrified, Prone, Restrained, Stunned\
Senses Passive Perception 15\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Traits

Swarm. The swarm can occupy another creature’s\
space and vice versa, and the swarm can move through\
any opening large enough for a Tiny raven. The swarm\
can’t regain Hit Points or gain Temporary Hit Points.\

### Actions

Beaks. Melee Attack Roll: +4, reach 5 ft. Hit: 5 (1d6\
+ 2) Piercing damage, or 2 (1d4) Piercing damage if the\
swarm is Bloodied.\
Cacophony (Recharge 6). Wisdom Saving Throw: DC\
10, one creature in the swarm’s space. Failure: The\
target has the Deafened condition until the start of the\
swarm’s next turn. While Deafened, the target also has\
Disadvantage on ability checks and attack rolls.\

Original layout: [[../../srd-5.2.1/pages/362|page 362]].
