---
splat_schema: 1
splat_id: e5e91d65-5f33-5e24-aa7f-dc5c4154bb28
splat_kind: spell
splat_key: dnd2024:spells/fire_storm
splat_name: Fire Storm
splat_origin:
  key: dnd2024:reference/spell/fire_storm
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: evocation
  classes:
  - cleric
  - druid
  - sorcerer
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 150
splat_rule:
  mechanic: dnd2024:spells/fire_storm
  implementation: dnd2024:spell_fire_storm
  parameters:
    area:
      shape: cubes
      maximum_count: 10
      side_feet: 10
      each_contiguous_with_another: true
      arrangement_chooser: caster
    save:
      ability: dexterity
      dc:
        binding: save_dc
      failure:
        damage:
          amount:
            dice:
              count: 7
              sides: 10
              add: 0
          type: fire
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    objects:
      flammable: true
      not_worn_or_carried: true
      effect: burning
splat_contracts:
- id: dnd2024:spell_fire_storm
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    save_dc:
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
  - saves
  - damage
  - burning_objects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - saving_throws
  - spatial_areas
  - inventory
  - choices
splat_conversion:
  status: specified
  category: spell
  source_sha256: 786949484c9f504db1d6be917cd757db64cdb5f0839e3083eeead94fd6b6b53c
  source_path: builtin/srd-5.2.1/spell/fire_storm.md
  pages:
  - 132
  engine_gaps: &id001
  - dnd2024:spell_fire_storm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_1.md
  specification_sha256: a4220a6418ca76afa7d26c37a4201513db47ed46b4a31d00b0608b1936ef1df9
splat_clauses:
- key: source_rules
  text: '# Fire Storm Level 7 Evocation (Cleric, Druid, Sorcerer) Casting Time: Action Range: 150 feet
    Components: V, S Duration: Instantaneous A storm of fire appears within range. The area of the storm
    consists of up to ten 10-foot Cubes, which you arrange as you like. Each Cube must be contiguous with
    at least one other Cube. Each creature in the area makes a Dexterity saving throw, taking 7d10 Fire
    damage on a failed save or half as much damage on a successful one. Flammable objects in the area
    that aren’t being worn or carried start burning.'
  sha256: b6e2085ab02803d4623ad5893ccba47d075a8942319fa10e7de0e5be1e5d9770
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fire Storm

<!-- SRD page 132 -->

Level 7 Evocation (Cleric, Druid, Sorcerer)\
Casting Time: Action\
Range: 150 feet\
Components: V, S\
Duration: Instantaneous\
A storm of fire appears within range. The area of\
the storm consists of up to ten 10-foot Cubes, which\
you arrange as you like. Each Cube must be contig-\
uous with at least one other Cube. Each creature\
in the area makes a Dexterity saving throw, taking\
7d10 Fire damage on a failed save or half as much\
damage on a successful one.\
Flammable objects in the area that aren’t being\
worn or carried start burning.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].
