---
splat_schema: 1
splat_id: cd9719c9-693a-55fd-9cc5-065882e41c16
splat_kind: spell
splat_key: dnd2024:spells/tiny_hut
splat_name: Tiny Hut
splat_origin:
  key: dnd2024:reference/spell/tiny_hut
splat_sources:
- title: SRD 5.2.1
  locator: pp. 169
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: evocation
  classes:
  - bard
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: 1 minute or Ritual
  ritual: true
  range:
    mode: self
  material:
    description: a crystal bead
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/tiny_hut
  implementation: dnd2024:spell_tiny_hut
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 28800
    area:
      shape: emanation
      radius_feet: 10
      origin: caster_at_cast
      stationary: true
    cast_failure:
      if: any_creature_in_area_not_fully_encapsulated
    crossing:
      creatures_and_objects_present_at_cast: freely_allowed
      all_other_creatures_and_objects: barred
    spell_barrier:
      maximum_spell_level_blocked: 3
      cast_through: forbidden
      effects_extend_inside: forbidden
    interior:
      atmosphere: comfortable_and_dry
      weather_independent: true
      light_choice:
        chooser: caster
        economy: none
        options:
        - dim
        - darkness
    appearance:
      outside: opaque
      color: caster_choice
      inside: transparent
    early_end_conditions:
    - caster_leaves_area
    - caster_casts_tiny_hut_again
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_tiny_hut
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    color:
      type: choice
      required: true
    lighting_choice:
      type: choice
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - areas
  - magic_suppression
splat_conversion:
  status: specified
  category: spell
  source_sha256: a5ebc50fac2e7218b601772e360cf00a3f002730876290fe299c162afe0e30eb
  source_path: builtin/srd-5.2.1/spell/tiny_hut.md
  pages:
  - 169
  engine_gaps: &id001
  - dnd2024:spell_tiny_hut
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 6625da3e253cc2998b24fabbe5cc3f020f0f7bc076c44f3ad49014c470448753
splat_clauses:
- key: source_rules
  text: '# Tiny Hut Level 3 Evocation (Bard, Wizard) Casting Time: 1 minute or Ritual Range: Self Components:
    V, S, M (a crystal bead) Duration: 8 hours A 10-foot Emanation springs into existence around you and
    remains stationary for the duration. The spell fails when you cast it if the Emanation isn’t big enough
    to fully encapsulate all creatures in its area. Creatures and objects within the Emanation when you
    cast the spell can move through it freely. All other creatures and objects are barred from passing
    through it. Spells of level 3 or lower can’t be cast through it, and the effects of such spells can’t
    extend into it. The atmosphere inside the Emanation is comfortable and dry, regardless of the weather
    outside. Until the spell ends, you can command the interior to have Dim Light or Darkness (no action
    required). The Emanation is opaque from the outside and of any color you choose, but it’s transparent
    from the inside. The spell ends early if you leave the Emanation or if you cast it again.'
  sha256: 22ecc468a47aa8778230e9e1a0395a09fdc02eb6ebd40ce9cee3d29ee30a0e58
  status: contract_specified
splat_requires_mechanics: *id001
---

# Tiny Hut

<!-- SRD page 169 -->

Level 3 Evocation (Bard, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (a crystal bead)\
Duration: 8 hours\
A 10-foot Emanation springs into existence around\
you and remains stationary for the duration. The\
spell fails when you cast it if the Emanation isn’t big\
enough to fully encapsulate all creatures in its area.\
Creatures and objects within the Emanation\
when you cast the spell can move through it freely.\
All other creatures and objects are barred from\
passing through it. Spells of level 3 or lower can’t be\
cast through it, and the effects of such spells can’t\
extend into it.\
The atmosphere inside the Emanation is comfort-\
able and dry, regardless of the weather outside. Until\
the spell ends, you can command the interior to have\
Dim Light or Darkness (no action required). The Em-\
anation is opaque from the outside and of any color\
you choose, but it’s transparent from the inside.\
The spell ends early if you leave the Emanation or\
if you cast it again.\

Original layout: [[../../srd-5.2.1/pages/169|page 169]].
