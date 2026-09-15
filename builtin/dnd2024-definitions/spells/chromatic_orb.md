---
splat_schema: 1
splat_id: 53786ff2-4da2-5c8b-8ab6-452fe723c13e
splat_kind: spell
splat_key: dnd2024:spells/chromatic_orb
splat_name: Chromatic Orb
splat_origin:
  key: dnd2024:reference/spell/chromatic_orb
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 90
  material:
    description: a diamond worth 50+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/chromatic_orb
  implementation: dnd2024:spell_chromatic_orb
  parameters:
    targets:
      kind: creature_or_object
      within_spell_range: true
    damage_type_choice:
    - acid
    - cold
    - fire
    - lightning
    - poison
    - thunder
    attack:
      kind: ranged_spell
      hit:
        dice:
          count:
            op: add
            args:
            - 3
            - op: multiply
              args:
              - 1
              - op: subtract
                args:
                - binding: cast_level
                - 1
          sides: 8
          add: 0
        type: chosen
    leap:
      trigger:
        damage_roll_d8_matching_faces_minimum: 2
      distance_from_previous_target_maximum_feet: 30
      target: different_target_caster_choice
      repeat_attack_and_damage_roll: true
      maximum_leaps:
        binding: cast_level
      maximum_targets_per_creature_per_cast: 1
splat_contracts:
- id: dnd2024:spell_chromatic_orb
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - attacks
  events:
  - spell.cast_completed
  - movement.completed
  capabilities:
  - spellcasting
  - effects
  - dice
  - attacks
  - spatial_relationships
splat_conversion:
  status: specified
  category: spell
  source_sha256: 3b50b7129b82ab1a169f2118018d3030f3b3375c8428035808836de137b9e1c4
  source_path: builtin/srd-5.2.1/spell/chromatic_orb.md
  pages:
  - 115
  engine_gaps: &id001
  - dnd2024:spell_chromatic_orb
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: d98d033f24092c84141ac3b8f02208c43a8023e0bb5ec95fabc18560138a6447
splat_clauses:
- key: source_rules
  text: '# Chromatic Orb Level 1 Evocation (Sorcerer, Wizard) Casting Time: Action Range: 90 feet Components:
    V, S, M (a diamond worth 50+ GP) Duration: Instantaneous You hurl an orb of energy at a target within
    range. Choose Acid, Cold, Fire, Lightning, Poison, or Thunder for the type of orb you create, and
    then make a ranged spell attack against the target. On a hit, the target takes 3d8 damage of the chosen
    type. If you roll the same number on two or more of the d8s, the orb leaps to a different target of
    your choice within 30 feet of the target. Make an attack roll against the new target, and make a new
    damage roll. The orb can’t leap again unless you cast the spell with a level 2+ spell slot. Using
    a Higher-Level Spell Slot. The damage increases by 1d8 for each spell slot level above 1. The orb
    can leap a maximum number of times equal to the level of the slot expended, and a creature can be
    targeted only once by each casting of this spell.'
  sha256: b3182319effd401ce2d3cd20274f7df1a141981d1220ae00be4bfb4aeae2bcf1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Chromatic Orb

<!-- SRD page 115 -->

Level 1 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (a diamond worth 50+ GP)\
Duration: Instantaneous\
You hurl an orb of energy at a target within range.\
Choose Acid, Cold, Fire, Lightning, Poison, or Thun-\
der for the type of orb you create, and then make a\
ranged spell attack against the target. On a hit, the\
target takes 3d8 damage of the chosen type.\
If you roll the same number on two or more of\
the d8s, the orb leaps to a different target of your\
choice within 30 feet of the target. Make an attack\
roll against the new target, and make a new damage\
roll. The orb can’t leap again unless you cast the\
spell with a level 2+ spell slot.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 1. The\
orb can leap a maximum number of times equal to\
the level of the slot expended, and a creature can be\
targeted only once by each casting of this spell.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].
