---
splat_schema: 1
splat_id: c58ffaf5-abf4-5105-bae0-03f347059b4c
splat_kind: spell
splat_key: dnd2024:spells/find_the_path
splat_name: Find the Path
splat_origin:
  key: dnd2024:reference/spell/find_the_path
splat_sources:
- title: SRD 5.2.1
  locator: pp. 131
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: divination
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 day
  casting_time: 1 minute
  ritual: false
  range:
    mode: self
  material:
    description: a set of divination tools—such as cards or runes—worth 100+ GP
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/find_the_path
  implementation: dnd2024:spell_find_the_path
  parameters:
    spell_lifetime:
      concentration: true
      maximum_seconds: 86400
    destination:
      chooser: caster
      named: true
      familiar: true
      must_be_specific: true
      must_be_stationary: true
      same_plane_at_cast: true
      violation: spell_fails
    output: most_direct_physical_route
    ongoing_information:
      requires: same_plane_as_destination
      outputs:
      - distance
      - direction
      at_path_choice: most_direct_path
    supplied_world_facts:
    - physical_route_network
    - destination_familiarity
    - destination_motion
    - destination_plane
splat_contracts:
- id: dnd2024:spell_find_the_path
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    destination:
      type: location
      required: true
    route_facts:
      type: resolution
      required: true
  outputs:
  - effects
  - information
  events:
  - concentration.ended
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - information
  - concentration
splat_conversion:
  status: specified
  category: spell
  source_sha256: 60a972c54053e154c1a9cd00a76f03caff365378896468251ba11612feb01aa5
  source_path: builtin/srd-5.2.1/spell/find_the_path.md
  pages:
  - 131
  engine_gaps: &id001
  - dnd2024:spell_find_the_path
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: dfa1b6e923de191aff03cacc71d1ad3acb93f1098b378e611869e83dd9569ec4
splat_clauses:
- key: source_rules
  text: '# Find the Path Level 6 Divination (Bard, Cleric, Druid) Casting Time: 1 minute Range: Self Components:
    V, S, M (a set of divination tools—such as cards or runes—worth 100+ GP) Duration: Concentration,
    up to 1 day You magically sense the most direct physical route to a location you name. You must be
    familiar with the location, and the spell fails if you name a destination on another plane of existence,
    a moving destination (such as a mobile fortress), or an unspecific destination (such as “a green dragon’s
    lair”). For the duration, as long as you are on the same plane of existence as the destination, you
    know how far it is and in what direction it lies. Whenever you face a choice of paths along the way
    there, you know which path is the most direct.'
  sha256: 79b61456869aef81eea05913d7a7b06ffba49bca30c20a61b0dd68797cf8c683
  status: contract_specified
splat_requires_mechanics: *id001
---

# Find the Path

<!-- SRD page 131 -->

Level 6 Divination (Bard, Cleric, Druid)\
Casting Time: 1 minute\
Range: Self\
Components: V, S, M (a set of divination tools—such as\
cards or runes—worth 100+ GP)\
Duration: Concentration, up to 1 day\
You magically sense the most direct physical route\
to a location you name. You must be familiar with\
the location, and the spell fails if you name a desti-\
nation on another plane of existence, a moving des-\
tination (such as a mobile fortress), or an unspecific\
destination (such as “a green dragon’s lair”).\
For the duration, as long as you are on the same\
plane of existence as the destination, you know how\
far it is and in what direction it lies. Whenever you\
face a choice of paths along the way there, you know\
which path is the most direct.\

Original layout: [[../../srd-5.2.1/pages/131|page 131]].
