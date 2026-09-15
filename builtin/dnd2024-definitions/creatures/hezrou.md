---
splat_schema: 1
splat_id: a959432d-bbcc-5cab-810d-29dd255b407b
splat_kind: creature
splat_key: dnd2024:creatures/hezrou
splat_name: Hezrou
splat_origin:
  key: dnd2024:reference/creature/hezrou
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  tags_text: (Demon)
  movement_text: 30 ft.
  resistances_text: Cold, Fire, Lightning
  immunities_text: Poison; Poisoned
  senses_text: Darkvision 120 ft.; Passive Perception 11
  languages_text: Abyssal; telepathy 120 ft.
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 157 (15d10 + 75)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 7
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 20
  save_constitution:
    value: 8
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 12
  save_wisdom:
    value: 4
  charisma:
    value: 13
  save_charisma:
    value: 1
  armor_class:
    value: 18
  max_hp:
    value: 157
  initiative:
    value: 6
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hezrou
  implementation: dnd2024:creature_hezrou
  parameters:
    features:
      demonic_restoration:
        economy: trait
        operation: passive_rule
        trigger: self.dies
        unless:
          plane: abyss
        remove_old_body: true
        new_body:
          delay_seconds: 0
          plane: abyss
          hp: maximum
      magic_resistance:
        economy: trait
        operation: passive_rule
        saving_throw_advantage_against:
        - spells
        - magical_effects
      stench:
        economy: trait
        operation: saving_throw
        ability: constitution
        dc: 16
        targets:
          kind: creature
          selection: each
          area:
            shape: emanation
            size_ft: 10
            origin: self
        failure:
          conditions:
          - condition: poisoned
            until: target.next_turn_start
        trigger: target.turn_start
        success:
          effects: []
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 3
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 5
        hit:
          damage:
          - average: 6
            roll:
              count: 1
              sides: 4
              modifier: 4
            type: slashing
          - average: 9
            roll:
              count: 2
              sides: 8
              modifier: 0
            type: poison
      leap:
        economy: bonus_action
        operation: jump
        maximum_ft: 30
        movement_cost_ft: 10
    damage_resistances:
    - cold
    - fire
    - lightning
    damage_immunities:
    - poison
    condition_immunities:
    - poisoned
    senses:
      darkvision: 120
      passive_perception: 11
    languages:
      speaks:
      - abyssal
      understands: []
      telepathy_range_ft: 120
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_hezrou
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - creature.died
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 3ee3c32c6802f1de2ee7323a548440c6af0a5334d5953ac86fa97e211cfd4a90
  source_path: builtin/srd-5.2.1/creature/hezrou.md
  pages:
  - 298
  engine_gaps: &id001
  - dnd2024:creature_hezrou
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: a2fafb050cd5b7e72b251ee1838b4ad03c1c42961a373060d672591427a2ac54
splat_clauses:
- key: source_rules
  text: '# Hezrou Large Fiend (Demon), Chaotic Evil AC 18 Initiative +6 (16) HP 157 (15d10 + 75) Speed
    30 ft. MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +7 Dex 17 +3 +3 Con 20 +5 +8 Int 5 -3 -3 Wis 12 +1 +4
    Cha 13 +1 +1 Resistances Cold, Fire, Lightning Immunities Poison; Poisoned Senses Darkvision 120 ft.;
    Passive Perception 11 Languages Abyssal; telepathy 120 ft. CR 8 (XP 3,900; PB +3) ### Traits Demonic
    Restoration. If the hezrou dies outside the Abyss, its body dissolves into ichor, and it gains a new
    body instantly, reviving with all its Hit Points somewhere in the Abyss. Magic Resistance. The hezrou
    has Advantage on saving throws against spells and other magical effects. Stench. Constitution Saving
    Throw: DC 16, any creature that starts its turn in a 10-foot Emanation originating from the hezrou.
    Failure: The target has the Poisoned condition until the start of its next turn. ### Actions Multiattack.
    The hezrou makes three Rend attacks. Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 6 (1d4 + 4) Slashing
    damage plus 9 (2d8) Poison damage. ### Bonus Actions Leap. The hezrou jumps up to 30 feet by spending
    10 feet of movement.'
  sha256: e81d51fe7900e065ebe13080f7ab6a47ea5e9916fad115b64a6573428ee18612
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hezrou

<!-- SRD page 298 -->

Large Fiend (Demon), Chaotic Evil\
AC 18 Initiative +6 (16)\
HP 157 (15d10 + 75)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +7 Dex 17 +3 +3 Con 20 +5 +8\
Int 5 −3 −3 Wis 12 +1 +4 Cha 13 +1 +1\
Resistances Cold, Fire, Lightning\
Immunities Poison; Poisoned\
Senses Darkvision 120 ft.; Passive Perception 11\
Languages Abyssal; telepathy 120 ft.\
CR 8 (XP 3,900; PB +3)\

### Traits

Demonic Restoration. If the hezrou dies outside the\
Abyss, its body dissolves into ichor, and it gains a new\
body instantly, reviving with all its Hit Points some-\
where in the Abyss.\
Magic Resistance. The hezrou has Advantage on saving\
throws against spells and other magical effects.\
Stench. Constitution Saving Throw: DC 16, any creature\
that starts its turn in a 10-foot Emanation originating\
from the hezrou. Failure: The target has the Poisoned\
condition until the start of its next turn.\

### Actions

Multiattack. The hezrou makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 5 ft. Hit: 6 (1d4 + 4)\
Slashing damage plus 9 (2d8) Poison damage.\

### Bonus Actions

Leap. The hezrou jumps up to 30 feet by spending 10\
feet of movement.\

Original layout: [[../../srd-5.2.1/pages/298|page 298]].
