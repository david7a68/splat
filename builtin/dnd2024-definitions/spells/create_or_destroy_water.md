---
splat_schema: 1
splat_id: 09578d72-3c2f-562e-a002-15dd09897499
splat_kind: spell
splat_key: dnd2024:spells/create_or_destroy_water
splat_name: Create or Destroy Water
splat_origin:
  key: dnd2024:reference/spell/create_or_destroy_water
splat_sources:
- title: SRD 5.2.1
  locator: pp. 121
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a mix of water and sand
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/create_or_destroy_water
  implementation: dnd2024:spell_create_or_destroy_water
  parameters:
    targeting:
      within_range: true
    choice:
    - create_container_water
    - rain
    - destroy_container_water
    - destroy_fog
    create_container_water:
      container: open
      clean: true
      max_gallons: 10
    rain:
      shape: cube
      side_feet: 30
      extinguishes: exposed_flames
    destroy_container_water:
      container: open
      max_gallons: 10
    destroy_fog:
      shape: cube
      side_feet: 30
    slot_scaling:
      above: 1
      gallons_per_level: 10
      cube_side_feet_per_level: 5
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_create_or_destroy_water
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      type: location_list
      required: true
    objects:
      type: object_list
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 530a8693a4a2e09cced0fe07e3e5aebf8e7fa9c2f40e0e433f3433ed6d7f964a
  source_path: builtin/srd-5.2.1/spell/create_or_destroy_water.md
  pages:
  - 121
  engine_gaps: &id001
  - dnd2024:spell_create_or_destroy_water
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 0ad096579777baf2e2945f09f5402a5d57ce8d568832a8ec82db8150dabf5a3c
splat_clauses:
- key: source_rules
  text: '# Create or Destroy Water Level 1 Transmutation (Cleric, Druid) Casting Time: Action Range: 30
    feet Components: V, S, M (a mix of water and sand) Duration: Instantaneous You do one of the following:
    Create Water. You create up to 10 gallons of clean water within range in an open container. Alternatively,
    the water falls as rain in a 30-foot Cube within range, extinguishing exposed flames there. Destroy
    Water. You destroy up to 10 gallons of water in an open container within range. Alternatively, you
    destroy fog in a 30-foot Cube within range. Using a Higher-Level Spell Slot. You create or destroy
    10 additional gallons of water, or the size of the Cube increases by 5 feet, for each spell slot level
    above 1.'
  sha256: b40d8b5ffc5c81303785127f733c8412e64d1619e6acac3ed281d2110e301ff6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Create or Destroy Water

<!-- SRD page 121 -->

Level 1 Transmutation (Cleric, Druid)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a mix of water and sand)\
Duration: Instantaneous\
You do one of the following:\
Create Water. You create up to 10 gallons of clean\
water within range in an open container. Alter-\
natively, the water falls as rain in a 30-foot Cube\
within range, extinguishing exposed flames there.\
Destroy Water. You destroy up to 10 gallons of\
water in an open container within range. Alter-\
natively, you destroy fog in a 30-foot Cube within\
range.\
Using a Higher-Level Spell Slot. You create or\
destroy 10 additional gallons of water, or the size of\
the Cube increases by 5 feet, for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/121|page 121]].
