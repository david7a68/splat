---
splat_schema: 1
splat_id: 82c2be54-7083-5c43-9d69-917d6f78556e
splat_kind: spell
splat_key: dnd2024:spells/antimagic_field
splat_name: Antimagic Field
splat_origin:
  key: dnd2024:reference/spell/antimagic_field
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: abjuration
  classes:
  - cleric
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
  material:
    description: iron filings
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/antimagic_field
  implementation: dnd2024:spell_antimagic_field
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 3600
    area:
      shape: emanation
      radius_feet: 10
      origin: caster
      moves_with_origin: true
    forbid_inside:
    - cast_spells
    - magic_actions
    - create_magical_effects
    - teleport_in
    - teleport_out
    - planar_travel
    prevent_affecting_inside:
    - spells
    - magic_actions
    - magical_effects
    - magic_item_properties
    suppress_inside:
      magic_item_properties: true
      portals: temporarily_closed
      spell_and_magic_areas: excluded
    ongoing_spells:
      operation: suppress
      exceptions_by_caster:
      - artifact
      - deity
      duration_elapses_while_suppressed: true
      suppress_does_not_end: true
    exceptions:
      dispel_magic_affects_aura: false
      other_antimagic_fields_nullify_aura: false
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_antimagic_field
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - magic_suppression
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 5c2028816da4d1225f0e2cf9b3985d8b9721cfe54c7e15d59647f414d87eaff8
  source_path: builtin/srd-5.2.1/spell/antimagic_field.md
  pages:
  - 109
  engine_gaps: &id001
  - dnd2024:spell_antimagic_field
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 2af2f2cb6300375b381200586406b1ec1856628db72abeb4bb9c98992873343e
splat_clauses:
- key: source_rules
  text: '# Antimagic Field Level 8 Abjuration (Cleric, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (iron filings) Duration: Concentration, up to 1 hour An aura of antimagic surrounds you in
    10-foot Emanation. No one can cast spells, take Magic actions, or create other magical effects inside
    the aura, and those things can’t target or otherwise affect anything inside it. Magical properties
    of magic items don’t work inside the aura or on anything inside it. Areas of effect created by spells
    or other magic can’t extend into the aura, and no one can teleport into or out of it or use planar
    travel there. Portals close temporarily while in the aura. Ongoing spells, except those cast by an
    Artifact or a deity, are suppressed in the area. While an effect is suppressed, it doesn’t function,
    but the time it spends suppressed counts against its duration. Dispel Magic has no effect on the aura,
    and the auras created by different Antimagic Field spells don’t nullify each other.'
  sha256: 919bd247698b81f3a3b6701926fd8091ee3f832a273d931a7c8185420e1729bf
  status: contract_specified
splat_requires_mechanics: *id001
---

# Antimagic Field

<!-- SRD page 109 -->

Level 8 Abjuration (Cleric, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (iron filings)\
Duration: Concentration, up to 1 hour\
An aura of antimagic surrounds you in 10-foot Em-\
anation. No one can cast spells, take Magic actions,\
or create other magical effects inside the aura, and\
those things can’t target or otherwise affect any-\
thing inside it. Magical properties of magic items\
don’t work inside the aura or on anything inside it.\
Areas of effect created by spells or other magic\
can’t extend into the aura, and no one can teleport\
into or out of it or use planar travel there. Portals\
close temporarily while in the aura.\
Ongoing spells, except those cast by an Artifact or\
a deity, are suppressed in the area. While an effect\
is suppressed, it doesn’t function, but the time it\
spends suppressed counts against its duration.\
Dispel Magic has no effect on the aura, and the au-\
ras created by different Antimagic Field spells don’t\
nullify each other.\

Original layout: [[../../srd-5.2.1/pages/109|page 109]].
