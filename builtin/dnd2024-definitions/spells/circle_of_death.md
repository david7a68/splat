---
splat_schema: 1
splat_id: 6d81fcb1-cee3-5b8e-8192-2a12d5c40bd8
splat_kind: spell
splat_key: dnd2024:spells/circle_of_death
splat_name: Circle of Death
splat_origin:
  key: dnd2024:reference/spell/circle_of_death
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: necromancy
  classes:
  - sorcerer
  - warlock
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
    feet: 150
  material:
    description: the powder of a crushed black pearl worth 500+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/circle_of_death
  implementation: dnd2024:spell_circle_of_death
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: sphere
      radius_feet: 60
      origin: chosen_point
      range_feet: 150
    targets:
      kind: creature
      selection: all_in_area
    save:
      ability: constitution
      dc: spell_save_dc
    damage:
      dice:
        base_count: 8
        sides: 8
        extra_per_slot: 2
        base_slot: 6
      type: necrotic
    success_damage_scale:
      numerator: 1
      denominator: 2
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_circle_of_death
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
    origin:
      type: location
      required: true
    save_dc:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - damage
  - tests
splat_conversion:
  status: specified
  category: spell
  source_sha256: 93b4964b4706c6576b9e43190d230ef4ec3aa8bcdfd0a1739a610a55b2fcf9ed
  source_path: builtin/srd-5.2.1/spell/circle_of_death.md
  pages:
  - 115
  engine_gaps: &id001
  - dnd2024:spell_circle_of_death
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 77688a0e6e40fab687f5d0023020c047e4753c67113336348272dfb879bb6507
splat_clauses:
- key: source_rules
  text: '# Circle of Death Level 6 Necromancy (Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    150 feet Components: V, S, M (the powder of a crushed black pearl worth 500+ GP) Duration: Instantaneous
    Negative energy ripples out in a 60-foot-radius Sphere from a point you choose within range. Each
    creature in that area makes a Constitution saving throw, taking 8d8 Necrotic damage on a failed save
    or half as much damage on a successful one. Using a Higher-Level Spell Slot. The damage increases
    by 2d8 for each spell slot level above 6.'
  sha256: 5fd8dc378871d19220bfb3561d25b960f944f8cc77eb7398f83a611f3845a71f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Circle of Death

<!-- SRD page 115 -->

Level 6 Necromancy (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (the powder of a crushed black\
pearl worth 500+ GP)\
Duration: Instantaneous\
Negative energy ripples out in a 60-foot-radius\
Sphere from a point you choose within range. Each\
creature in that area makes a Constitution saving\
throw, taking 8d8 Necrotic damage on a failed save\
or half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 2d8 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].
