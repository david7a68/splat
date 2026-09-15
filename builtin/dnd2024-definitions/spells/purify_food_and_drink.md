---
splat_schema: 1
splat_id: eb1d3055-55bf-59f6-838b-15c1b6a3fe18
splat_kind: spell
splat_key: dnd2024:spells/purify_food_and_drink
splat_name: Purify Food and Drink
splat_origin:
  key: dnd2024:reference/spell/purify_food_and_drink
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - cleric
  - druid
  - paladin
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 10
splat_rule:
  mechanic: dnd2024:spells/purify_food_and_drink
  implementation: dnd2024:spell_purify_food_and_drink
  parameters:
    area:
      shape: sphere
      radius_feet: 5
    eligible:
      kinds:
      - food
      - drink
      nonmagical: true
    remove:
    - poison
    - rot
splat_contracts:
- id: dnd2024:spell_purify_food_and_drink
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    locations:
      required: false
      type: location_list
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
  - purified_food_and_drink
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - spatial_areas
  - movement
splat_conversion:
  status: specified
  category: spell
  source_sha256: d04e05175b1c462431574dfbaedd3425ceb45f2ca28d76fb7d1401b6bacc66cc
  source_path: builtin/srd-5.2.1/spell/purify_food_and_drink.md
  pages:
  - 157
  engine_gaps: &id001
  - dnd2024:spell_purify_food_and_drink
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: c0e945918cf3c46530eb9e8ff64af06a0c74e266823ec1cf3b1f5019a11babb9
splat_clauses:
- key: source_rules
  text: '# Purify Food and Drink Level 1 Transmutation (Cleric, Druid, Paladin) Casting Time: Action or
    Ritual Range: 10 feet Components: V, S Duration: Instantaneous You remove poison and rot from nonmagical
    food and drink in a 5-foot-radius Sphere centered on a point within range.'
  sha256: 83136db0dfaa42be701f8d23d77f507f23c2505ea7e6b32b9d1bb2c87749868d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Purify Food and Drink

<!-- SRD page 157 -->

Level 1 Transmutation (Cleric, Druid, Paladin)\
Casting Time: Action or Ritual\
Range: 10 feet\
Components: V, S\
Duration: Instantaneous\
You remove poison and rot from nonmagical food\
and drink in a 5-foot-radius Sphere centered on a\
point within range.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
