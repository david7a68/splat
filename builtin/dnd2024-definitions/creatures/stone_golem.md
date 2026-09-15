---
splat_schema: 1
splat_id: a2dd7d7d-f61c-5fa6-a5d9-253d6a4e52de
splat_kind: creature
splat_key: dnd2024:creatures/stone_golem
splat_name: Stone Golem
splat_origin:
  key: dnd2024:reference/creature/stone_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 330
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: construct
  alignment: Unaligned
  movement_text: 30 ft.
  immunities_text: Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Darkvision 120 ft.; Passive Perception 10
  languages_text: Understands Common plus two other languages but can’t speak
  cr_text: 10 (XP 5,900; PB +4)
  hit_points_text: 220 (21d10 + 105)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 6
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 18
  max_hp:
    value: 220
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/stone_golem
  implementation: dnd2024:creature_stone_golem
  parameters:
    damage_immunities:
    - poison
    - psychic
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    - paralyzed
    - petrified
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 10
    languages:
      speak: false
      known:
      - common
      additional_choice_count: 2
    features:
      immutable_form:
        economy: passive
        op: shapechange.prohibit
      magic_resistance:
        economy: passive
        op: roll.advantage
        rolls:
        - saving_throw
        against:
        - spell
        - magical_effect
      slam:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 10
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 2
                sides: 8
              add: 6
            average: 15
          - type: force
            amount:
              dice:
                count: 2
                sides: 8
              add: 0
            average: 9
      force_bolt:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 9
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 120
          long: 120
        hit:
        - op: damage
          components:
          - type: force
            amount:
              dice:
                count: 4
                sides: 10
              add: 0
            average: 22
      slow:
        economy: bonus_action
        op: spellcasting
        profile:
          ability: constitution
          waived_components:
          - verbal
          - somatic
          - material
          save_dc: 17
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/slow
        recharge:
          die_sides: 6
          success_min: 5
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 2
          choices:
          - slam
          - force_bolt
          combination: any
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_stone_golem
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: e13d5cfa07064dd2a50740b668344b3cbc7a4a3f4e8afb2eb1c0baeb60c2562d
  source_path: builtin/srd-5.2.1/creature/stone_golem.md
  pages:
  - 330
  engine_gaps: &id001
  - dnd2024:creature_stone_golem
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 3dbec564d9524fec37e4ab2364b968ff67dd4d3d575084a3a35b0b66fd721c50
splat_clauses:
- key: source_rules
  text: '# Stone Golem Large Construct, Unaligned AC 18 Initiative +3 (13) HP 220 (21d10 + 105) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 22 +6 +6 Dex 9 -1 -1 Con 20 +5 +5 Int 3 -4 -4 Wis 11 +0 +0 Cha
    1 -5 -5 Immunities Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
    Senses Darkvision 120 ft.; Passive Perception 10 Languages Understands Common plus two other languages
    but can’t speak CR 10 (XP 5,900; PB +4) ### Traits Immutable Form. The golem can’t shape-shift. Magic
    Resistance. The golem has Advantage on saving throws against spells and other magical effects. ###
    Actions Multiattack. The golem makes two attacks, using Slam or Force Bolt in any combination. Slam.
    Melee Attack Roll: +10, reach 5 ft. Hit: 15 (2d8 + 6) Bludgeoning damage plus 9 (2d8) Force damage.
    Force Bolt. Ranged Attack Roll: +9, range 120 ft. Hit: 22 (4d10) Force damage. ### Bonus Actions Slow
    (Recharge 5–6). The golem casts the Slow spell, requiring no spell components and using Constitution
    as the spellcasting ability (spell save DC 17).'
  sha256: 7037884bd14ffbd1ffd1aa2b726e4ea8e5ce251130405e76bbabf5d0bd0b2b85
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stone Golem

<!-- SRD page 330 -->

Large Construct, Unaligned\
AC 18 Initiative +3 (13)\
HP 220 (21d10 + 105)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 9 −1 −1 Con 20 +5 +5\
Int 3 −4 −4 Wis 11 +0 +0 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 120 ft.; Passive Perception 10\
Languages Understands Common plus two other\
languages but can’t speak\
CR 10 (XP 5,900; PB +4)\

### Traits

Immutable Form. The golem can’t shape-shift.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two attacks, using Slam\
or Force Bolt in any combination.\
Slam. Melee Attack Roll: +10, reach 5 ft. Hit: 15 (2d8\
+ 6) Bludgeoning damage plus 9 (2d8) Force damage.\
Force Bolt. Ranged Attack Roll: +9, range 120 ft. Hit:\
22 (4d10) Force damage.\

### Bonus Actions

Slow (Recharge 5–6). The golem casts the Slow spell,\
requiring no spell components and using Constitution\
as the spellcasting ability (spell save DC 17).\

Original layout: [[../../srd-5.2.1/pages/330|page 330]].
