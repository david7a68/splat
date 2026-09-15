---
splat_schema: 1
splat_id: f796a67c-326b-504d-aa54-dab654fef250
splat_kind: spell
splat_key: dnd2024:spells/spike_growth
splat_name: Spike Growth
splat_origin:
  key: dnd2024:reference/spell/spike_growth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 164
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: transmutation
  classes:
  - druid
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 150
  material:
    description: seven thorns
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/spike_growth
  implementation: dnd2024:spell_spike_growth
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    area:
      shape: sphere
      radius_feet: 20
      origin: chosen_point
      range_feet: 150
      affected_surface: ground
    properties:
      difficult_terrain: true
      camouflage: natural_terrain
    damage:
      event: participant.moved
      target_kind: creature
      movement_scopes:
      - into_area
      - within_area
      per_distance_feet: 5
      dice: 2d4
      type: piercing
      saving_throw: false
    hazard_recognition:
      applies_if: did_not_see_area_at_cast
      before_entering: true
      economy: search_action
      check:
        ability: wisdom
        skill_choice:
        - perception
        - survival
        dc: spell_save_dc
      success: recognize_hazard
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_spike_growth
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
  - damage
  events:
  - concentration.ended
  - participant.moved
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - movement
  - areas
  - damage
  - tests
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5074e5b0308fae8bb665f1d6604cadd22561e42963af486796d041d9d5639eda
  source_path: builtin/srd-5.2.1/spell/spike_growth.md
  pages:
  - 164
  engine_gaps: &id001
  - dnd2024:spell_spike_growth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 66a3c9ad0395617b9e33d1ffbd77e04c90801efa3a2cf1a7075c282926405ba6
splat_clauses:
- key: source_rules
  text: '# Spike Growth Level 2 Transmutation (Druid, Ranger) Casting Time: Action Range: 150 feet Components:
    V, S, M (seven thorns) Duration: Concentration, up to 10 minutes The ground in a 20-foot-radius Sphere
    centered on a point within range sprouts hard spikes and thorns. The area becomes Difficult Terrain
    for the duration. When a creature moves into or within the area, it takes 2d4 Piercing damage for
    every 5 feet it travels. The transformation of the ground is camouflaged to look natural. Any creature
    that can’t see the area when the spell is cast must take a Search action and succeed on a Wisdom (Perception
    or Survival) check against your spell save DC to recognize the terrain as hazardous before entering
    it.'
  sha256: 8efd9475387f47686f0bbcd66a152b70269915073d025dbafaaf07e6ed949e75
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spike Growth

<!-- SRD page 164 -->

Level 2 Transmutation (Druid, Ranger)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (seven thorns)\
Duration: Concentration, up to 10 minutes\
The ground in a 20-foot-radius Sphere centered\
on a point within range sprouts hard spikes and\
thorns. The area becomes Difficult Terrain for the\
duration. When a creature moves into or within the\
area, it takes 2d4 Piercing damage for every 5 feet it\
travels.\
The transformation of the ground is camouflaged\
to look natural. Any creature that can’t see the area\
when the spell is cast must take a Search action\
and succeed on a Wisdom (Perception or Survival)\
check against your spell save DC to recognize the\
terrain as hazardous before entering it.\

Original layout: [[../../srd-5.2.1/pages/164|page 164]].
