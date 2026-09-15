---
splat_schema: 1
splat_id: fed2df36-1e53-5e3f-8b77-aef7a2b16ae5
splat_kind: creature
splat_key: dnd2024:creatures/water_elemental
splat_name: Water Elemental
splat_origin:
  key: dnd2024:reference/creature/water_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 337
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  movement_text: 30 ft., Swim 90 ft.
  resistances_text: Acid, Fire
  immunities_text: Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Aquan)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 114 (12d10 + 48)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 14
  max_hp:
    value: 114
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/water_elemental
  implementation: dnd2024:creature_water_elemental
  parameters:
    features:
      freeze:
        economy: trait
        operation: passive_rule
        trigger: self.cold_damage_taken
        speed_modifier_ft: -20
        until: self.next_turn_end
      water_form:
        economy: trait
        operation: passive_rule
        occupy_enemy_space: true
        minimum_gap_inches: 1
        extra_movement_cost: 0
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: slam
          count: 2
      slam:
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
          - average: 13
            roll:
              count: 2
              sides: 8
              modifier: 4
            type: bludgeoning
          prone:
            condition: prone
            maximum_size: medium
      whelm:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 4
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: strength
        dc: 15
        targets:
          kind: creature
          selection: each
          location: self.space
        failure:
          damage:
          - roll:
              count: 4
              sides: 8
              modifier: 4
            type: bludgeoning
          grapple:
            condition: grappled
            source: self
            escape_dc: 14
            maximum_size: large
            while_grappled:
              conditions:
              - restrained
              suffocating_unless_water_breathing: true
              damage_each_self_turn_start:
              - roll:
                  count: 2
                  sides: 8
                  modifier: 0
                type: bludgeoning
        success:
          damage_multiplier: 0.5
          other_effects: false
        capacity:
          choose_one:
          - size: large
            count: 1
          - maximum_size: medium
            count: 2
        pull_out:
          economy: action
          actor_within_ft: 5
          check: strength_athletics
          dc: 14
    damage_resistances:
    - acid
    - fire
    damage_immunities:
    - poison
    condition_immunities:
    - exhaustion
    - grappled
    - paralyzed
    - petrified
    - poisoned
    - prone
    - restrained
    - unconscious
    senses:
      darkvision: 60
      passive_perception: 10
    languages:
      speaks:
      - language: primordial
        dialects:
        - aquan
      understands: []
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
- id: dnd2024:creature_water_elemental
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
  - turn.ended
  - damage.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 26e790a049fb7095477b1813c63ae5da679667c955bae3ec03978c704d336a8a
  source_path: builtin/srd-5.2.1/creature/water_elemental.md
  pages:
  - 337
  engine_gaps: &id001
  - dnd2024:creature_water_elemental
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: f814c4cc0a22d91d3543095f4210846cc5667ce8f6222510ae3e04874550d65a
splat_clauses:
- key: source_rules
  text: '# Water Elemental Large Elemental, Neutral AC 14 Initiative +2 (12) HP 114 (12d10 + 48) Speed
    30 ft., Swim 90 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 14 +2 +2 Con 18 +4 +4 Int 5 -3 -3
    Wis 10 +0 +0 Cha 8 -1 -1 Resistances Acid, Fire Immunities Poison; Exhaustion, Grappled, Paralyzed,
    Petrified, Poisoned, Prone, Restrained, Unconscious Senses Darkvision 60 ft.; Passive Perception 10
    Languages Primordial (Aquan) CR 5 (XP 1,800; PB +3) ### Traits Freeze. If the elemental takes Cold
    damage, its Speed decreases by 20 feet until the end of its next turn. Water Form. The elemental can
    enter an enemy’s space and stop there. It can move through a space as narrow as 1 inch without expending
    extra movement to do so. ### Actions Multiattack. The elemental makes two Slam attacks. Slam. Melee
    Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4) Bludgeoning damage. If the target is a Medium or smaller
    creature, it has the Prone condition. Whelm (Recharge 4–6). Strength Saving Throw: DC 15, each creature
    in the elemental’s space. Failure: 22 (4d8 + 4) Bludgeoning damage. If the target is a Large or smaller
    creature, it has the Grappled condition (escape DC 14). Until the grapple ends, the target has the
    Restrained condition, is suffocating unless it can breathe water, and takes 9 (2d8) Bludgeoning damage
    at the start of each of the elemental’s turns. The elemental can grapple one Large creature or up
    to two Medium or smaller creatures at a time with Whelm. As an action, a creature within 5 feet of
    the elemental can pull a creature out of it by succeeding on a DC 14 Strength (Athletics) check. Success:
    Half damage only.'
  sha256: 32fca2ff0fb124ab269a3a5c345e2e6ca27f4cfc20548d0b99751855c7bf4ae0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Water Elemental

<!-- SRD page 337 -->

Large Elemental, Neutral\
AC 14 Initiative +2 (12)\
HP 114 (12d10 + 48)\
Speed 30 ft., Swim 90 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 14 +2 +2 Con 18 +4 +4\
Int 5 −3 −3 Wis 10 +0 +0 Cha 8 −1 −1\
Resistances Acid, Fire\
Immunities Poison; Exhaustion, Grappled, Paralyzed,\
Petrified, Poisoned, Prone, Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Aquan)\
CR 5 (XP 1,800; PB +3)\

### Traits

Freeze. If the elemental takes Cold damage, its Speed\
decreases by 20 feet until the end of its next turn.\
Water Form. The elemental can enter an enemy’s space\
and stop there. It can move through a space as narrow\
as 1 inch without expending extra movement to do so.\

### Actions

Multiattack. The elemental makes two Slam attacks.\
Slam. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8\
+ 4) Bludgeoning damage. If the target is a Medium or\
smaller creature, it has the Prone condition.\
Whelm (Recharge 4–6). Strength Saving Throw: DC 15,\
each creature in the elemental’s space. Failure: 22 (4d8\
+ 4) Bludgeoning damage. If the target is a Large or\
smaller creature, it has the Grappled condition (escape\
DC 14). Until the grapple ends, the target has the Re-\
strained condition, is suffocating unless it can breathe\
water, and takes 9 (2d8) Bludgeoning damage at the\
start of each of the elemental’s turns. The elemental\
can grapple one Large creature or up to two Medium\
or smaller creatures at a time with Whelm. As an ac-\
tion, a creature within 5 feet of the elemental can pull\
a creature out of it by succeeding on a DC 14 Strength\
(Athletics) check. Success: Half damage only.\

Original layout: [[../../srd-5.2.1/pages/337|page 337]].
