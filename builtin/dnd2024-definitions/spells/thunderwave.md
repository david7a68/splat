---
splat_schema: 1
splat_id: b7643b11-2a43-559e-971f-aae50cc8f01f
splat_kind: spell
splat_key: dnd2024:spells/thunderwave
splat_name: Thunderwave
splat_origin:
  key: dnd2024:reference/spell/thunderwave
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - bard
  - druid
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/thunderwave
  implementation: dnd2024:spell_thunderwave
  parameters:
    targeting:
      kind: creatures_in_area
      shape: cube
      side_feet: 15
      origin: caster
    save:
      ability: constitution
      failure:
        damage:
          dice: 2d8
          type: thunder
        push:
          distance_feet: 10
          direction: away_from_caster
      success:
        damage_fraction:
          numerator: 1
          denominator: 2
    objects:
      unsecured: true
      entirely_within_area: true
      push_feet: 10
      direction: away_from_caster
    sound:
      kind: thunderous_boom
      audible_radius_feet: 300
    slot_scaling:
      above: 1
      damage_per_level: 1d8
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_thunderwave
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
    save_dc:
      type: integer
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - damage
  - spatial_effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: 1e86f4349dae61c0e2350d6b07e7a554a49b1cbf608abe66d88cf8844edb24a9
  source_path: builtin/srd-5.2.1/spell/thunderwave.md
  pages:
  - 169
  engine_gaps: &id001
  - dnd2024:spell_thunderwave
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: c6048c59ab0402ebd4c16a4407cdf31c4d8e0ec5e3b13ecc1ddea59a2e04c9a7
splat_clauses:
- key: source_rules
  text: '# Thunderwave Level 1 Evocation (Bard, Druid, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S Duration: Instantaneous You unleash a wave of thunderous energy. Each creature in
    a 15-foot Cube originating from you makes a Constitution saving throw. On a failed save, a creature
    takes 2d8 Thunder damage and is pushed 10 feet away from you. On a successful save, a creature takes
    half as much damage only. In addition, unsecured objects that are entirely within the Cube are pushed
    10 feet away from you, and a thunderous boom is audible within 300 feet. Using a Higher-Level Spell
    Slot. The damage increases by 1d8 for each spell slot level above 1.'
  sha256: 82ce3e84b96202945529a5736e3ba5034bd5278fcb99faa4490da17f8740902b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Thunderwave

<!-- SRD page 169 -->

Level 1 Evocation (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Instantaneous\
You unleash a wave of thunderous energy. Each\
creature in a 15-foot Cube originating from you\
makes a Constitution saving throw. On a failed\
save, a creature takes 2d8 Thunder damage and is\
pushed 10 feet away from you. On a successful save,\
a creature takes half as much damage only.\
In addition, unsecured objects that are entirely\
within the Cube are pushed 10 feet away from you,\
and a thunderous boom is audible within 300 feet.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].
