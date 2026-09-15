---
splat_schema: 1
splat_id: 9484799d-b608-508a-b072-41698346c374
splat_kind: spell
splat_key: dnd2024:spells/sunburst
splat_name: Sunburst
splat_origin:
  key: dnd2024:reference/spell/sunburst
splat_sources:
- title: SRD 5.2.1
  locator: pp. 167
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: evocation
  classes:
  - cleric
  - druid
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
    feet: 150
  material:
    description: a piece of sunstone
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/sunburst
  implementation: dnd2024:spell_sunburst
  parameters:
    area:
      shape: sphere
      radius_feet: 60
      origin: chosen_point_within_range
      illumination: sunlight
    targets:
      kind: creature
      each_in_area: true
    save:
      ability: constitution
      dc:
        binding: save_dc
      failure:
        damage:
          dice:
            count: 12
            sides: 6
            add: 0
          type: radiant
        condition: blinded
        duration_minutes: 1
      success:
        damage:
          dice:
            count: 12
            sides: 6
            add: 0
          type: radiant
        damage_fraction:
          numerator: 1
          denominator: 2
    repeat_save:
      requires: blinded_by_spell
      event: target_turn_end
      ability: constitution
      success: end_effect_on_target
    dispel:
      in_area: true
      effect: darkness
      created_by: any_spell
splat_contracts:
- id: dnd2024:spell_sunburst
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      required: false
      type: location_list
    campaign:
      required: false
      type: campaign
    save_dc:
      type: integer
      required: true
    choices:
      required: false
      type: choice
  outputs:
  - effects
  - damage
  events:
  - spell.cast_completed
  - turn.end
  - movement.completed
  - save.before
  - save.after
  capabilities:
  - spellcasting
  - effects
  - dice
  - damage
  - spatial_relationships
  - conditions
  - saving_throws
splat_conversion:
  status: specified
  category: spell
  source_sha256: 2687bcee2ffc6ffce56de263e80ef5ab966e73c2e88c4e6d9b59806d7fa2ac86
  source_path: builtin/srd-5.2.1/spell/sunburst.md
  pages:
  - 167
  engine_gaps: &id001
  - dnd2024:spell_sunburst
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: 4fedd1bd6fce3d7bef340e893c758037c4061f6e473233b2050094883e57c4da
splat_clauses:
- key: source_rules
  text: '# Sunburst Level 8 Evocation (Cleric, Druid, Sorcerer, Wizard) Casting Time: Action Range: 150
    feet Components: V, S, M (a piece of sunstone) Duration: Instantaneous Brilliant sunlight flashes
    in a 60-foot-radius Sphere centered on a point you choose within range. Each creature in the Sphere
    makes a Constitution saving throw. On a failed save, a creature takes 12d6 Radiant damage and has
    the Blinded condition for 1 minute. On a successful save, it takes half as much damage only. A creature
    Blinded by this spell makes another Constitution saving throw at the end of each of its turns, ending
    the effect on itself on a success. This spell dispels Darkness in its area that was created by any
    spell.'
  sha256: 8f00229b9e581746c9587c64cb5fe1761c19e990f90351ee9775676ca2f3c1f3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sunburst

<!-- SRD page 167 -->

Level 8 Evocation (Cleric, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a piece of sunstone)\
Duration: Instantaneous\
Brilliant sunlight flashes in a 60-foot-radius Sphere\
centered on a point you choose within range. Each\
creature in the Sphere makes a Constitution saving\
throw. On a failed save, a creature takes 12d6 Ra-\
diant damage and has the Blinded condition for 1\
minute. On a successful save, it takes half as much\
damage only.\
A creature Blinded by this spell makes another\
Constitution saving throw at the end of each of its\
turns, ending the effect on itself on a success.\
This spell dispels Darkness in its area that was\
created by any spell.\

Original layout: [[../../srd-5.2.1/pages/167|page 167]].
