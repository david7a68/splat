---
splat_schema: 1
splat_id: 0ffb9a54-fb7c-569e-b970-25d02576365e
splat_kind: creature
splat_key: dnd2024:creatures/homunculus
splat_name: Homunculus
splat_origin:
  key: dnd2024:reference/creature/homunculus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 299
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: tiny
  type: construct
  alignment: Neutral
  movement_text: 20 ft., Fly 40 ft.
  immunities_text: Poison; Charmed, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Understands Common plus one other language but can’t speak
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 4 (1d4 + 2)
splat_stats:
  strength:
    value: 4
  save_strength:
    value: -3
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 2
  charisma:
    value: 7
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 4
  initiative:
    value: 2
  speed:
    value: 20
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/homunculus
  implementation: dnd2024:creature_homunculus
  parameters:
    features:
      telepathic_bond:
        economy: trait
        operation: passive_rule
        telepathy:
          with: master
          requires_same_plane: true
          range: unlimited
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          reach_ft: 5
        hit:
          damage:
          - average: 1
            roll:
              constant: 1
            type: piercing
          save:
            operation: saving_throw
            ability: constitution
            dc: 12
            targets:
              kind: target
            failure:
              conditions:
              - condition: poisoned
                until: self.next_turn_end
            failure_by_5_or_more:
              conditions:
              - condition: poisoned
                duration_seconds: 60
              while_poisoned:
                condition: unconscious
                ends_on: target.damage_taken
    damage_immunities:
    - poison
    condition_immunities:
    - charmed
    - poisoned
    senses:
      darkvision: 60
      passive_perception: 10
    languages:
      speaks: []
      understands:
      - common
      additional_languages:
        count: 1
        chooser: creator
        can_speak: false
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
- id: dnd2024:creature_homunculus
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
  - turn.ended
  - damage.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: df67d41c5cc7313f0fc172cf6384d6d1cb822f0b8f9e55fb2a96fbaf17e5dde6
  source_path: builtin/srd-5.2.1/creature/homunculus.md
  pages:
  - 299
  engine_gaps: &id001
  - dnd2024:creature_homunculus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: be768f7082a9bd799888890c0cd7614668c40648797cf12e63f869108a5b298d
splat_clauses:
- key: source_rules
  text: '# Homunculus Tiny Construct, Neutral AC 13 Initiative +2 (12) HP 4 (1d4 + 2) Speed 20 ft., Fly
    40 ft. MOD SAVE MOD SAVE MOD SAVE Str 4 -3 -3 Dex 15 +2 +2 Con 14 +2 +2 Int 10 +0 +0 Wis 10 +0 +2
    Cha 7 -2 +0 Immunities Poison; Charmed, Poisoned Senses Darkvision 60 ft.; Passive Perception 10 Languages
    Understands Common plus one other language but can’t speak CR 0 (XP 10; PB +2) ### Traits Telepathic
    Bond. While the homunculus is on the same plane of existence as its master, the two of them can communicate
    telepathically with each other. ### Actions Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing
    damage, and the target is subjected to the following effect. Constitution Saving Throw: DC 12. Failure:
    The target has the Poisoned condition until the end of the homunculus’s next turn. Failure by 5 or
    More: The target has the Poisoned condition for 1 minute. While Poisoned, the target has the Unconscious
    condition, which ends early if the target takes any damage.'
  sha256: 155bc927b88ecff2c7cdd53ba751f118274348818ad70cc8e299b6ef85fac8e2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Homunculus

<!-- SRD page 299 -->

Tiny Construct, Neutral\
AC 13 Initiative +2 (12)\
HP 4 (1d4 + 2)\
Speed 20 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 4 −3 −3 Dex 15 +2 +2 Con 14 +2 +2\
Int 10 +0 +0 Wis 10 +0 +2 Cha 7 −2 +0\
Immunities Poison; Charmed, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Understands Common plus one other language\
but can’t speak\
CR 0 (XP 10; PB +2)\

### Traits

Telepathic Bond. While the homunculus is on the same\
plane of existence as its master, the two of them can\
communicate telepathically with each other.\

### Actions

Bite. Melee Attack Roll: +4, reach 5 ft. Hit: 1 Piercing\
damage, and the target is subjected to the following\
effect. Constitution Saving Throw: DC 12. Failure: The\
target has the Poisoned condition until the end of the\
homunculus’s next turn. Failure by 5 or More: The tar-\
get has the Poisoned condition for 1 minute. While Poi-\
soned, the target has the Unconscious condition, which\
ends early if the target takes any damage.\

Original layout: [[../../srd-5.2.1/pages/299|page 299]].
