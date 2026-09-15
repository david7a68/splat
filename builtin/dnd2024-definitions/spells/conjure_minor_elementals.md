---
splat_schema: 1
splat_id: b5d274e6-266a-512f-be8e-9db637d17892
splat_kind: spell
splat_key: dnd2024:spells/conjure_minor_elementals
splat_name: Conjure Minor Elementals
splat_origin:
  key: dnd2024:reference/spell/conjure_minor_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 118
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: conjuration
  classes:
  - druid
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/conjure_minor_elementals
  implementation: dnd2024:spell_conjure_minor_elementals
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 600
    area:
      shape: emanation
      radius_feet: 15
      origin: caster
      moves_with_origin: true
    damage_trigger:
      event: attack.hit
      attacker: caster
      target_kind: creature
      target_in_area: true
    extra_damage:
      dice:
        base_count: 2
        sides: 8
        extra_per_slot: 1
        base_slot: 4
      type:
        chooser: caster
        timing: attack_made
        options:
        - acid
        - cold
        - fire
        - lightning
    terrain:
      scope: ground_in_area
      affected: caster_enemies
      difficult: true
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_conjure_minor_elementals
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    damage_type_choices:
      type: choice
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - damage
  events:
  - attack.hit
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - attacks
  - damage
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 45ae0fd9f779d6510023032c4c093f41ba376894af74fa0c635d94638138051b
  source_path: builtin/srd-5.2.1/spell/conjure_minor_elementals.md
  pages:
  - 118
  engine_gaps: &id001
  - dnd2024:spell_conjure_minor_elementals
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: a973e08deff33a63e35ec6020dbf28a5d4ecb8458c54aa228eac17b89f26ec64
splat_clauses:
- key: source_rules
  text: '# Conjure Minor Elementals Level 4 Conjuration (Druid, Wizard) Casting Time: Action Range: Self
    Components: V, S Duration: Concentration, up to 10 minutes You conjure spirits from the Elemental
    Planes that flit around you in a 15-foot Emanation for the duration. Until the spell ends, any attack
    you make deals an extra 2d8 damage when you hit a creature in the Emanation. This damage is Acid,
    Cold, Fire, or Lightning (your choice when you make the attack). In addition, the ground in the Emanation
    is Difficult Terrain for your enemies. Using a Higher-Level Spell Slot. The damage increases by 1d8
    for each spell slot level above 4.'
  sha256: 6c4a74d9ca9e246b877ce815c2acbc5f7ebf7c7c577af5923a452978f3e6df06
  status: contract_specified
splat_requires_mechanics: *id001
---

# Conjure Minor Elementals

<!-- SRD page 118 -->

Level 4 Conjuration (Druid, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You conjure spirits from the Elemental Planes that\
flit around you in a 15-foot Emanation for the dura-\
tion. Until the spell ends, any attack you make deals\
an extra 2d8 damage when you hit a creature in the\
Emanation. This damage is Acid, Cold, Fire, or Light-\
ning (your choice when you make the attack).\
In addition, the ground in the Emanation is Diffi-\
cult Terrain for your enemies.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d8 for each spell slot level above 4.\

Original layout: [[../../srd-5.2.1/pages/118|page 118]].
