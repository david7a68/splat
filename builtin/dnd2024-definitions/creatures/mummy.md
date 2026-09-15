---
splat_schema: 1
splat_id: 006a2999-e089-576e-94db-c2a47aca2c61
splat_kind: creature
splat_key: dnd2024:creatures/mummy
splat_name: Mummy
splat_origin:
  key: dnd2024:reference/creature/mummy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 309
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Lawful Evil
  size_options_text: Medium or Small
  movement_text: 20 ft.
  vulnerabilities_text: Fire
  immunities_text: Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Common plus two other languages
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 58 (9d8 + 18)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 12
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 11
  max_hp:
    value: 58
  initiative:
    value: -1
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/mummy
  implementation: dnd2024:creature_mummy
  parameters:
    damage_vulnerabilities:
    - fire
    damage_immunities:
    - necrotic
    - poison
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    - paralyzed
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 11
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 2
    features:
      rotting_fist:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
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
                count: 1
                sides: 10
              add: 3
            average: 8
          - type: necrotic
            amount:
              dice:
                count: 3
                sides: 6
              add: 0
            average: 10
        - op: curse.apply
          when:
            target_kind: creature
          rules:
            healing_allowed: false
            long_rest_restores_hp_maximum: false
          periodic:
            interval:
              amount: 24
              unit: hour
            effect:
              op: hp_maximum.reduce
              amount:
                dice:
                  count: 3
                  sides: 6
                add: 0
              average: 10
        on_reduce_target_to_zero_hp:
          when:
            target_kind: creature
          op: death
          corpse: dust
      dreadful_glare:
        economy: action
        op: saving_throw
        ability: wisdom
        dc: 11
        targets:
          kind: creature
          count: 1
          range_ft: 60
          visible_to_actor: true
        failure:
        - op: condition.apply
          condition: frightened
          duration:
            until: actor.next_turn_end
        success:
        - op: feature.immunity
          source: actor
          feature: dreadful_glare
          duration:
            amount: 24
            unit: hour
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: rotting_fist
          count: 2
        - feature: dreadful_glare
          count: 1
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
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
- id: dnd2024:creature_mummy
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
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 441e7e9ba703e95b61b6cb9a575fef4cb2106f309e0b99d2420d2413223c1552
  source_path: builtin/srd-5.2.1/creature/mummy.md
  pages:
  - 309
  engine_gaps: &id001
  - dnd2024:creature_mummy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 74815586752b10708676cd416767212678d4883b9b6c5cfc933936ca0fdb1c79
splat_clauses:
- key: source_rules
  text: '# Mummy Medium or Small Undead, Lawful Evil AC 11 Initiative -1 (9) HP 58 (9d8 + 18) Speed 20
    ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 8 -1 -1 Con 15 +2 +2 Int 6 -2 -2 Wis 12 +1 +3 Cha
    12 +1 +1 Vulnerabilities Fire Immunities Necrotic, Poison; Charmed, Exhaustion, Frightened, Paralyzed,
    Poisoned Senses Darkvision 60 ft.; Passive Perception 11 Languages Common plus two other languages
    CR 3 (XP 700; PB +2) ### Actions Multiattack. The mummy makes two Rotting Fist attacks and uses Dreadful
    Glare. Rotting Fist. Melee Attack Roll: +5, reach 5 ft. Hit: 8 (1d10 + 3) Bludgeoning damage plus
    10 (3d6) Necrotic damage. If the target is a creature, it is cursed. While cursed, the target can’t
    regain Hit Points, its Hit Point maximum doesn’t return to normal when finishing a Long Rest, and
    its Hit Point maximum decreases by 10 (3d6) every 24 hours that elapse. A creature dies and turns
    to dust if reduced to 0 Hit Points by this attack. Dreadful Glare. Wisdom Saving Throw: DC 11, one
    creature the mummy can see within 60 feet. Failure: The target has the Frightened condition until
    the end of the mummy’s next turn. Success: The target is immune to this mummy’s Dreadful Glare for
    24 hours.'
  sha256: 41e8d2c8cb057801c331e01111e01ad4728218d52a7d2222ecabe50e213b3cf0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mummy

<!-- SRD page 309 -->

Medium or Small Undead, Lawful Evil\
AC 11 Initiative −1 (9)\
HP 58 (9d8 + 18)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 8 −1 −1 Con 15 +2 +2\
Int 6 −2 −2 Wis 12 +1 +3 Cha 12 +1 +1\
Vulnerabilities Fire\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common plus two other languages\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The mummy makes two Rotting Fist at-\
tacks and uses Dreadful Glare.\
Rotting Fist. Melee Attack Roll: +5, reach 5 ft. Hit: 8\
(1d10 + 3) Bludgeoning damage plus 10 (3d6) Necrotic\
damage. If the target is a creature, it is cursed. While\
cursed, the target can’t regain Hit Points, its Hit Point\
maximum doesn’t return to normal when finishing a\
Long Rest, and its Hit Point maximum decreases by 10\
(3d6) every 24 hours that elapse. A creature dies and\
turns to dust if reduced to 0 Hit Points by this attack.\
Dreadful Glare. Wisdom Saving Throw: DC 11, one\
creature the mummy can see within 60 feet. Failure:\
The target has the Frightened condition until the end of\
the mummy’s next turn. Success: The target is immune\
to this mummy’s Dreadful Glare for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/309|page 309]].
