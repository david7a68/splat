---
splat_schema: 1
splat_id: 6ab88a22-af92-500a-9df9-44cebb0110d0
splat_kind: creature
splat_key: dnd2024:creatures/spirit_naga
splat_name: Spirit Naga
splat_origin:
  key: dnd2024:reference/creature/spirit_naga
splat_sources:
- title: SRD 5.2.1
  locator: pp. 329
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  movement_text: 40 ft.
  immunities_text: Poison; Charmed, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: Abyssal, Common
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 135 (18d10 + 36)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 17
  save_dexterity:
    value: 6
  constitution:
    value: 14
  save_constitution:
    value: 5
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 15
  save_wisdom:
    value: 5
  charisma:
    value: 16
  save_charisma:
    value: 6
  armor_class:
    value: 17
  max_hp:
    value: 135
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/spirit_naga
  implementation: dnd2024:creature_spirit_naga
  parameters:
    damage_immunities:
    - poison
    condition_immunities:
    - charmed
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 12
    languages:
      speak: true
      known:
      - common
      - abyssal
    features:
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 7
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 6
              add: 4
            average: 7
          - type: poison
            amount:
              dice:
                count: 4
                sides: 6
              add: 0
            average: 14
      necrotic_ray:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 60
          long: 60
        hit:
        - op: damage
          components:
          - type: necrotic
            amount:
              dice:
                count: 6
                sides: 6
              add: 0
            average: 21
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: intelligence
          waived_components:
          - somatic
          - material
          spells:
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/mage_hand
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/minor_illusion
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/water_breathing
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/detect_thoughts
            uses:
              count: 2
              recovery: long_rest
          - spell:
              key: dnd2024:spells/dimension_door
            uses:
              count: 2
              recovery: long_rest
          - spell:
              key: dnd2024:spells/hold_person
            uses:
              count: 2
              recovery: long_rest
            cast_level: 3
          - spell:
              key: dnd2024:spells/lightning_bolt
            uses:
              count: 2
              recovery: long_rest
            cast_level: 4
          save_dc: 14
      fiendish_restoration:
        economy: passive
        trigger: actor.died
        op: revive
        hp: maximum
        delay:
          amount:
            dice:
              count: 1
              sides: 6
            add: 0
          unit: day
        only_prevention_spell:
          key: dnd2024:spells/wish
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 3
          choices:
          - bite
          - necrotic_ray
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
- id: dnd2024:creature_spirit_naga
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
  - actor.died
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
  source_sha256: 62490a5e30eaa311262c36daebeb4448d8d1f7bb797e1f8cb27d78beaed2906c
  source_path: builtin/srd-5.2.1/creature/spirit_naga.md
  pages:
  - 329
  engine_gaps: &id001
  - dnd2024:creature_spirit_naga
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 65bcbbb1403cdad8caad1b2716a2dd4c01ddd17f55f3a9e6ac594bf9d067ce5d
splat_clauses:
- key: source_rules
  text: '# Spirit Naga Large Fiend, Chaotic Evil AC 17 Initiative +3 (13) HP 135 (18d10 + 36) Speed 40
    ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 17 +3 +6 Con 14 +2 +5 Int 16 +3 +3 Wis 15 +2 +5 Cha
    16 +3 +6 Immunities Poison; Charmed, Poisoned Senses Darkvision 60 ft.; Passive Perception 12 Languages
    Abyssal, Common CR 8 (XP 3,900; PB +3) ### Traits Fiendish Restoration. If it dies, the naga returns
    to life in 1d6 days and regains all its Hit Points. Only a Wish spell can prevent this trait from
    functioning. ### Actions Multiattack. The naga makes three attacks, using Bite or Necrotic Ray in
    any combination. Bite. Melee Attack Roll: +7, reach 10 ft. Hit: 7 (1d6 + 4) Piercing damage plus 14
    (4d6) Poison damage. Necrotic Ray. Ranged Attack Roll: +6, range 60 ft. Hit: 21 (6d6) Necrotic damage.
    Spellcasting. The naga casts one of the following spells, requiring no Somatic or Material components
    and using Intelligence as the spellcasting ability (spell save DC 14): At Will: Detect Magic, Mage
    Hand, Minor Illusion, Water Breathing 2/Day Each: Detect Thoughts, Dimension Door, Hold Person (level
    3 version), Lightning Bolt (level 4 version)'
  sha256: d66192d0fe589a92775f4cb1492f53a10131ea56a1449ca53497c18a4f926299
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spirit Naga

<!-- SRD page 329 -->

Large Fiend, Chaotic Evil\
AC 17 Initiative +3 (13)\
HP 135 (18d10 + 36)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 17 +3 +6 Con 14 +2 +5\
Int 16 +3 +3 Wis 15 +2 +5 Cha 16 +3 +6\
Immunities Poison; Charmed, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Abyssal, Common\
CR 8 (XP 3,900; PB +3)\

### Traits

Fiendish Restoration. If it dies, the naga returns to life\
in 1d6 days and regains all its Hit Points. Only a Wish\
spell can prevent this trait from functioning.\

### Actions

Multiattack. The naga makes three attacks, using Bite\
or Necrotic Ray in any combination.\
Bite. Melee Attack Roll: +7, reach 10 ft. Hit: 7 (1d6 + 4)\
Piercing damage plus 14 (4d6) Poison damage.\
Necrotic Ray. Ranged Attack Roll: +6, range 60 ft. Hit:\
21 (6d6) Necrotic damage.\
Spellcasting. The naga casts one of the following\
spells, requiring no Somatic or Material components\
and using Intelligence as the spellcasting ability (spell\
save DC 14):\
At Will: Detect Magic, Mage Hand, Minor Illusion, Wa-\
ter Breathing\
2/Day Each: Detect Thoughts, Dimension Door,\
Hold Person (level 3 version), Lightning Bolt (level\
4 version)\

Original layout: [[../../srd-5.2.1/pages/329|page 329]].
