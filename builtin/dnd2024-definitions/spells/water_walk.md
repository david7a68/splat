---
splat_schema: 1
splat_id: ae4dc889-df0f-5d43-b166-eff1b6d007a8
splat_kind: spell
splat_key: dnd2024:spells/water_walk
splat_name: Water Walk
splat_origin:
  key: dnd2024:reference/spell/water_walk
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173, 174
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - cleric
  - druid
  - ranger
  - sorcerer
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action or Ritual
  ritual: true
  range:
    feet: 30
  material:
    description: a piece of cork
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/water_walk
  implementation: dnd2024:spell_water_walk
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 3600
    targets:
      kind: creature
      willing: true
      chooser: caster
      maximum_count: 10
      range_feet: 30
    movement:
      surface_types:
      - any_liquid
      - water
      - acid
      - mud
      - snow
      - quicksand
      - lava
      treat_surface_as: harmless_solid_ground
      lava_heat_damage: still_possible
    cross_surface:
      directions:
      - surface_to_liquid
      - liquid_to_surface
      economy: bonus_action
    falling_into_liquid:
      passes_through_surface: true
      enters_liquid: true
      bonus_action_required: false
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_water_walk
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
  - movement
splat_conversion:
  status: specified
  category: spell
  source_sha256: 79dbf99132536bd8f8c26634626abb2aa79ed375d3065e4559853bc3a5c89b3a
  source_path: builtin/srd-5.2.1/spell/water_walk.md
  pages:
  - 173
  - 174
  engine_gaps: &id001
  - dnd2024:spell_water_walk
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: f025b223dbeed13936bb1e9642961fd188727c62726cd5b64ee9ac590b698a0c
splat_clauses:
- key: source_rules
  text: '# Water Walk Level 3 Transmutation (Cleric, Druid, Ranger, Sorcerer) Casting Time: Action or
    Ritual Range: 30 feet Components: V, S, M (a piece of cork) Duration: 1 hour This spell grants the
    ability to move across any liquid surface—such as water, acid, mud, snow, quicksand, or lava—as if
    it were harmless solid ground (creatures crossing molten lava can still take damage from the heat).
    Up to ten willing creatures of your choice within range gain this ability for the duration. An affected
    target must take a Bonus Action to pass from the liquid’s surface into the liquid itself and vice
    versa, but if the target falls into the liquid, the target passes through the surface into the liquid
    below.'
  sha256: 0cece24ff19da6c134ab0af1fd40cee1d2f85bfb9f76ef98756e9b444e4ecdfb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Water Walk

<!-- SRD page 173 -->

Level 3 Transmutation (Cleric, Druid, Ranger, Sorcerer)\
Casting Time: Action or Ritual\
Range: 30 feet\

<!-- SRD page 174 -->

Components: V, S, M (a piece of cork)\
Duration: 1 hour\
This spell grants the ability to move across any liquid\
surface—such as water, acid, mud, snow, quicksand,\
or lava—as if it were harmless solid ground (crea-\
tures crossing molten lava can still take damage\
from the heat). Up to ten willing creatures of your\
choice within range gain this ability for the duration.\
An affected target must take a Bonus Action to\
pass from the liquid’s surface into the liquid itself\
and vice versa, but if the target falls into the liquid,\
the target passes through the surface into the liquid\
below.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]], [[../../srd-5.2.1/pages/174|page 174]].
