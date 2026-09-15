---
splat_schema: 1
splat_id: 095ce437-b53b-536e-a720-5fa6a0397580
splat_kind: spell
splat_key: dnd2024:spells/flame_strike
splat_name: Flame Strike
splat_origin:
  key: dnd2024:reference/spell/flame_strike
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: evocation
  classes:
  - cleric
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: a pinch of sulfur
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/flame_strike
  implementation: dnd2024:spell_flame_strike
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: cylinder
      radius_feet: 10
      height_feet: 40
      origin: chosen_point
      range_feet: 60
    targets:
      kind: creature
      selection: all_in_area
    save:
      ability: dexterity
      dc: spell_save_dc
    damage:
    - dice:
        base_count: 5
        sides: 6
        extra_per_slot: 1
        base_slot: 5
      type: fire
    - dice:
        base_count: 5
        sides: 6
        extra_per_slot: 1
        base_slot: 5
      type: radiant
    success_damage_scale:
      numerator: 1
      denominator: 2
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_flame_strike
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
  source_sha256: 7059e3787d8fb8248101ddb10dcdcbb557ce6cdba73228886729a8cbf712f294
  source_path: builtin/srd-5.2.1/spell/flame_strike.md
  pages:
  - 132
  engine_gaps: &id001
  - dnd2024:spell_flame_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: bd51b4630b0ada7fb3b40944694907156b17bd55f03bbc54c77af25710ed7969
splat_clauses:
- key: source_rules
  text: '# Flame Strike Level 5 Evocation (Cleric) Casting Time: Action Range: 60 feet Components: V,
    S, M (a pinch of sulfur) Duration: Instantaneous A vertical column of brilliant fire roars down from
    above. Each creature in a 10-foot-radius, 40-foothigh Cylinder centered on a point within range makes
    a Dexterity saving throw, taking 5d6 Fire damage and 5d6 Radiant damage on a failed save or half as
    much damage on a successful one. Using a Higher-Level Spell Slot. The Fire damage and the Radiant
    damage increase by 1d6 for each spell slot level above 5.'
  sha256: 1e2c8201658573ac57cedcab61de1c44ebba3b8352c7b3d388152431ccdcbd38
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flame Strike

<!-- SRD page 132 -->

Level 5 Evocation (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a pinch of sulfur)\
Duration: Instantaneous\
A vertical column of brilliant fire roars down from\
above. Each creature in a 10-foot-radius, 40-foot-\
high Cylinder centered on a point within range\
makes a Dexterity saving throw, taking 5d6 Fire\
damage and 5d6 Radiant damage on a failed save or\
half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The Fire damage\
and the Radiant damage increase by 1d6 for each\
spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].
