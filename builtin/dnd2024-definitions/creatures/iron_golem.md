---
splat_schema: 1
splat_id: 294e6c1c-6e74-574f-a834-775f3ca30093
splat_kind: creature
splat_key: dnd2024:creatures/iron_golem
splat_name: Iron Golem
splat_origin:
  key: dnd2024:reference/creature/iron_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 302
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: construct
  alignment: Unaligned
  movement_text: 30 ft.
  immunities_text: Fire, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Darkvision 120 ft.; Passive Perception 10
  languages_text: Understands Common plus two other languages but can’t speak
  cr_text: 16 (XP 15,000; PB +5)
  hit_points_text: 252 (24d10 + 120)
splat_stats:
  strength:
    value: 24
  save_strength:
    value: 7
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
    value: 20
  max_hp:
    value: 252
  initiative:
    value: 9
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/iron_golem
  implementation: dnd2024:creature_iron_golem
  parameters:
    features:
      immutable_form:
        economy: passive
        shape_shift_allowed: false
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      bladed_arm:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 12
        targets:
          count: 1
        hit:
        - op: damage
          average: 20
          amount:
            dice:
              count: 3
              sides: 8
            add: 7
          damage_type: slashing
        - op: damage
          average: 10
          amount:
            dice:
              count: 3
              sides: 6
            add: 0
          damage_type: fire
        reach_feet: 10
      fiery_bolt:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 10
        targets:
          count: 1
        hit:
        - op: damage
          average: 36
          amount:
            dice:
              count: 8
              sides: 8
            add: 0
          damage_type: fire
        range_feet: 120
      poison_breath:
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
        ability: constitution
        dc: 18
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 60
            origin: self
        failure:
        - op: damage
          average: 55
          amount:
            dice:
              count: 10
              sides: 10
            add: 0
          damage_type: poison
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: bladed_arm
        - feature: fiery_bolt
      fire_absorption:
        economy: passive
        trigger: damage.subjected
        damage_type: fire
        op: heal
        amount:
          event_value: incoming_damage_of_matching_type
    defenses:
      immunities:
        damage_types:
        - fire
        - poison
        - psychic
        conditions:
        - charmed
        - exhaustion
        - frightened
        - paralyzed
        - petrified
        - poisoned
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 120
    communication:
      speaks: []
      understands:
      - common
      additional_languages:
        count: 2
        chooser: gm
        distinct_from_known: true
        can_speak: false
        can_understand: true
      can_speak: false
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
- id: dnd2024:creature_iron_golem
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
  - damage.subjected
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - hit_points
  - resources
  - shapechanging
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: f51691ca96fb6eb7671072caf862b7eda7877df3d6d3a29b08d1efb304063b66
  source_path: builtin/srd-5.2.1/creature/iron_golem.md
  pages:
  - 302
  engine_gaps: &id001
  - dnd2024:creature_iron_golem
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: d1b1ad7d14ebbc558b812252a71761ab6ed5863e12873cf2e0c921501039bc45
splat_clauses:
- key: source_rules
  text: '# Iron Golem Large Construct, Unaligned AC 20 Initiative +9 (19) HP 252 (24d10 + 120) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 24 +7 +7 Dex 9 -1 -1 Con 20 +5 +5 Int 3 -4 -4 Wis 11 +0 +0 Cha
    1 -5 -5 Immunities Fire, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
    Senses Darkvision 120 ft.; Passive Perception 10 Languages Understands Common plus two other languages
    but can’t speak CR 16 (XP 15,000; PB +5) ### Traits Fire Absorption. Whenever the golem is subjected
    to Fire damage, it regains a number of Hit Points equal to the Fire damage dealt. Immutable Form.
    The golem can’t shape-shift. Magic Resistance. The golem has Advantage on saving throws against spells
    and other magical effects. ### Actions Multiattack. The golem makes two attacks, using Bladed Arm
    or Fiery Bolt in any combination. Bladed Arm. Melee Attack Roll: +12, reach 10 ft. Hit: 20 (3d8 +
    7) Slashing damage plus 10 (3d6) Fire damage. Fiery Bolt. Ranged Attack Roll: +10, range 120 ft. Hit:
    36 (8d8) Fire damage. Poison Breath (Recharge 6). Constitution Saving Throw: DC 18, each creature
    in a 60-foot Cone. Failure: 55 (10d10) Poison damage. Success: Half damage.'
  sha256: 52f4fc3ec29967adc4f0eced071c4ec8b074cc30f30dcd5ce11c9a6ab2e72f29
  status: contract_specified
splat_requires_mechanics: *id001
---

# Iron Golem

<!-- SRD page 302 -->

Large Construct, Unaligned\
AC 20 Initiative +9 (19)\
HP 252 (24d10 + 120)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 24 +7 +7 Dex 9 −1 −1 Con 20 +5 +5\
Int 3 −4 −4 Wis  11 +0 +0 Cha  1 −5 −5\
Immunities Fire, Poison, Psychic; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 120 ft.; Passive Perception 10\
Languages Understands Common plus two other\
languages but can’t speak\
CR 16 (XP 15,000; PB +5)\

### Traits

Fire Absorption. Whenever the golem is subjected to\
Fire damage, it regains a number of Hit Points equal to\
the Fire damage dealt.\
Immutable Form. The golem can’t shape-shift.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two attacks, using\
Bladed Arm or Fiery Bolt in any combination.\
Bladed Arm. Melee Attack Roll: +12, reach 10 ft.\
Hit: 20 (3d8 + 7) Slashing damage plus 10 (3d6)\
Fire damage.\
Fiery Bolt. Ranged Attack Roll: +10, range 120 ft. Hit:\
36 (8d8) Fire damage.\
Poison Breath (Recharge 6). Constitution Saving\
Throw: DC 18, each creature in a 60-foot Cone. Fail-\
ure: 55 (10d10) Poison damage. Success: Half damage.\

Original layout: [[../../srd-5.2.1/pages/302|page 302]].
