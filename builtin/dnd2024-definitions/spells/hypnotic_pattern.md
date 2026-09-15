---
splat_schema: 1
splat_id: fc4b0931-2173-59ab-a1df-e4807fc8aad3
splat_kind: spell
splat_key: dnd2024:spells/hypnotic_pattern
splat_name: Hypnotic Pattern
splat_origin:
  key: dnd2024:reference/spell/hypnotic_pattern
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: illusion
  classes:
  - bard
  - sorcerer
  - warlock
  - wizard
  components:
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 120
  material:
    description: a pinch of confetti
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/hypnotic_pattern
  implementation: dnd2024:spell_hypnotic_pattern
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 60
    area:
      shape: cube
      side_feet: 30
      range_feet: 120
    pattern:
      appears: momentarily
      then: vanishes
    targets:
      kind: creature
      in_area: true
      sees_pattern: true
    save:
      ability: wisdom
      dc: spell_save_dc
      success: unaffected
    failure:
      condition: charmed
      while_charmed:
        condition: incapacitated
        speed_feet: 0
    end_target_triggers:
    - event: damage.applied
      subject: target
    - event: action.completed
      actor: another_creature
      action: shake_target_awake
      economy: action
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_hypnotic_pattern
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    targets:
      type: participant_list
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - action.completed
  - concentration.ended
  - damage.applied
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - conditions
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 6d353a60f476a7e2a9287f557a6bd561e882f7c25c3af37b8586e8baf76ec1fc
  source_path: builtin/srd-5.2.1/spell/hypnotic_pattern.md
  pages:
  - 141
  engine_gaps: &id001
  - dnd2024:spell_hypnotic_pattern
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 6230d6ef3054ae07c3c31f94543b326403d6005aff7b60bc961cd83c44a81d54
splat_clauses:
- key: source_rules
  text: '# Hypnotic Pattern Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    120 feet Components: S, M (a pinch of confetti) Duration: Concentration, up to 1 minute You create
    a twisting pattern of colors in a 30-foot Cube within range. The pattern appears for a moment and
    vanishes. Each creature in the area who can see the pattern must succeed on a Wisdom saving throw
    or have the Charmed condition for the duration. While Charmed, the creature has the Incapacitated
    condition and a Speed of 0. The spell ends for an affected creature if it takes any damage or if someone
    else uses an action to shake the creature out of its stupor.'
  sha256: b2e2649c4eb76580e4f69ad49dfa5ccfd7be830d34046fd953a2c7c7baf63943
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hypnotic Pattern

<!-- SRD page 141 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: S, M (a pinch of confetti)\
Duration: Concentration, up to 1 minute\
You create a twisting pattern of colors in a 30-foot\
Cube within range. The pattern appears for a mo-\
ment and vanishes. Each creature in the area who\
can see the pattern must succeed on a Wisdom\
saving throw or have the Charmed condition for the\
duration. While Charmed, the creature has the Inca-\
pacitated condition and a Speed of 0.\
The spell ends for an affected creature if it takes\
any damage or if someone else uses an action to\
shake the creature out of its stupor.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].
