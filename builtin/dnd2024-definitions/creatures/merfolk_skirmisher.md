---
splat_schema: 1
splat_id: 4e85999e-74ec-59f9-83c4-f05e2801c75a
splat_kind: creature
splat_key: dnd2024:creatures/merfolk_skirmisher
splat_name: Merfolk Skirmisher
splat_origin:
  key: dnd2024:reference/creature/merfolk_skirmisher
splat_sources:
- title: SRD 5.2.1
  locator: pp. 308
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: elemental
  alignment: Neutral
  movement_text: 10 ft., Swim 40 ft.
  senses_text: Passive Perception 12
  languages_text: Common, Primordial (Aquan)
  cr_text: 1/8 (XP 25; PB +2)
  hit_points_text: 11 (2d8 + 2)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 11
  max_hp:
    value: 11
  initiative:
    value: 1
  speed:
    value: 10
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/merfolk_skirmisher
  implementation: dnd2024:creature_merfolk_skirmisher
  parameters:
    senses: []
    passive_perception: 12
    languages:
      speak: true
      known:
      - common
      - primordial
      primordial_dialects:
      - aquan
    features:
      amphibious:
        economy: passive
        op: breathing.allow
        media:
        - air
        - water
      ocean_spear:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 2
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        range_ft:
          normal: 20
          long: 60
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              add: 0
            average: 3
          - type: cold
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            average: 2
        - op: speed.add
          amount_ft: -10
          duration:
            until: target.next_turn_end
          when:
            target_kind: creature
        after_attack:
          when: ranged_attack_hit_or_miss
          op: item.return
          destination: actor_hand
          magical: true
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
- id: dnd2024:creature_merfolk_skirmisher
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
  - item_state
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 32c7e1747f2eecaf407b1339680b938c04e9895f27937d4b7ffa5afcae7b8f13
  source_path: builtin/srd-5.2.1/creature/merfolk_skirmisher.md
  pages:
  - 308
  engine_gaps: &id001
  - dnd2024:creature_merfolk_skirmisher
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 2cf4847057743b86b51495ea5b303a2f7c0e94767e3d7e4d024a2e97726954c2
splat_clauses:
- key: source_rules
  text: '# Merfolk Skirmisher Medium Elemental, Neutral AC 11 Initiative +1 (11) HP 11 (2d8 + 2) Speed
    10 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 13 +1 +1 Con 12 +1 +1 Int 11 +0 +0
    Wis 14 +2 +2 Cha 12 +1 +1 Senses Passive Perception 12 Languages Common, Primordial (Aquan) CR 1/8
    (XP 25; PB +2) ### Traits Amphibious. The merfolk can breathe air and water. ### Actions Ocean Spear.
    Melee or Ranged Attack Roll: +2, reach 5 ft. or range 20/60 ft. Hit: 3 (1d6) Piercing damage plus
    2 (1d4) Cold damage. If the target is a creature, its Speed decreases by 10 feet until the end of
    its next turn. Hit or Miss: The spear magically returns to the merfolk’s hand immediately after a
    ranged attack.'
  sha256: 90d56794288f6d2d7ff4a50dfa54d0a3a3c35c39f47ed5b30a49e689759473b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Merfolk Skirmisher

<!-- SRD page 308 -->

Medium Elemental, Neutral\
AC 11 Initiative +1 (11)\
HP 11 (2d8 + 2)\
Speed 10 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 13 +1 +1 Con 12 +1 +1\
Int 11 +0 +0 Wis 14 +2 +2 Cha 12 +1 +1\
Senses Passive Perception 12\
Languages Common, Primordial (Aquan)\
CR 1/8 (XP 25; PB +2)\

### Traits

Amphibious. The merfolk can breathe air and water.\

### Actions

Ocean Spear. Melee or Ranged Attack Roll: +2, reach 5\
ft. or range 20/60 ft. Hit: 3 (1d6) Piercing damage plus 2\
(1d4) Cold damage. If the target is a creature, its Speed\
decreases by 10 feet until the end of its next turn. Hit or\
Miss: The spear magically returns to the merfolk’s hand\
immediately after a ranged attack.\

Original layout: [[../../srd-5.2.1/pages/308|page 308]].
