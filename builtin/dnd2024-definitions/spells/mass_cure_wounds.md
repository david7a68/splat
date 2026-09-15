---
splat_schema: 1
splat_id: 65399dfc-e071-52d4-8a0d-953570dfa950
splat_kind: spell
splat_key: dnd2024:spells/mass_cure_wounds
splat_name: Mass Cure Wounds
splat_origin:
  key: dnd2024:reference/spell/mass_cure_wounds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/mass_cure_wounds
  implementation: dnd2024:spell_mass_cure_wounds
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    area:
      shape: sphere
      radius_feet: 30
      origin: chosen_point
      point_visible: true
      range_feet: 60
    targets:
      kind: creature
      in_area: true
      chooser: caster
      maximum_count: 6
    healing:
      dice:
        base_count: 5
        sides: 8
        extra_per_slot: 1
        base_slot: 5
      add:
        binding: spellcasting_ability_modifier
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_mass_cure_wounds
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
    spellcasting_ability_modifier:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - healing
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: 308f077914bad3de0011342d7c780f9495adda7a2884e7384ef2b250e7db5b9f
  source_path: builtin/srd-5.2.1/spell/mass_cure_wounds.md
  pages:
  - 147
  engine_gaps: &id001
  - dnd2024:spell_mass_cure_wounds
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: a2750da90274978473533621dccdc601a816783085d753903a99080ea90180f8
splat_clauses:
- key: source_rules
  text: '# Mass Cure Wounds Level 5 Abjuration (Bard, Cleric, Druid) Casting Time: Action Range: 60 feet
    Components: V, S Duration: Instantaneous A wave of healing energy washes out from a point you can
    see within range. Choose up to six creatures in a 30-foot-radius Sphere centered on that point. Each
    target regains Hit Points equal to 5d8 plus your spellcasting ability modifier. Using a Higher-Level
    Spell Slot. The healing increases by 1d8 for each spell slot level above 5.'
  sha256: fb838af2960f49db594d266ca6a121138c5d69de104689e5a0df0fac553f79b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mass Cure Wounds

<!-- SRD page 147 -->

Level 5 Abjuration (Bard, Cleric, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
A wave of healing energy washes out from a point\
you can see within range. Choose up to six crea-\
tures in a 30-foot-radius Sphere centered on that\
point. Each target regains Hit Points equal to 5d8\
plus your spellcasting ability modifier.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 1d8 for each spell slot level above 5.\

Original layout: [[../../srd-5.2.1/pages/147|page 147]].
