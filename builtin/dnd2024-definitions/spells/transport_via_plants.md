---
splat_schema: 1
splat_id: b3344347-6f56-5da9-962b-ee4c8eb3c8c6
splat_kind: spell
splat_key: dnd2024:spells/transport_via_plants
splat_name: Transport via Plants
splat_origin:
  key: dnd2024:reference/spell/transport_via_plants
splat_sources:
- title: SRD 5.2.1
  locator: pp. 170
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: conjuration
  classes:
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 10
splat_rule:
  mechanic: dnd2024:spells/transport_via_plants
  implementation: dnd2024:spell_transport_via_plants
  parameters:
    targeting:
      origin:
        kind: inanimate_plant
        size_min: large
        within_range: true
      destination:
        kind: plant
        same_plane: true
        distance_limit: none
        previously_seen_or_touched: true
    link:
      duration: spell_duration
      eligible_traveler: any_creature
      direction: origin_to_destination
      movement_cost_feet: 5
      enter: origin_plant
      exit: destination_plant
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: minute
splat_contracts:
- id: dnd2024:spell_transport_via_plants
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    locations:
      type: location_list
      required: true
    objects:
      type: object_list
      required: true
    campaign:
      type: campaign
      required: true
  outputs:
  - effects
  - locations
  events:
  - spell.cast_completed
  - area.entered
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - spatial_effects
  - campaign_state
  - planar_travel
splat_conversion:
  status: specified
  category: spell
  source_sha256: aa1d9c89aa5b2dac43ac4b971a93dd263dc3e1baa51f028bf5e531c052c2e9f7
  source_path: builtin/srd-5.2.1/spell/transport_via_plants.md
  pages:
  - 170
  engine_gaps: &id001
  - dnd2024:spell_transport_via_plants
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: afd260e8a0a358baaf453fe77595f86a3fe9bddeccfed8f2fb65061a3f20d467
splat_clauses:
- key: source_rules
  text: '# Transport via Plants Level 6 Conjuration (Druid) Casting Time: Action Range: 10 feet Components:
    V, S Duration: 1 minute This spell creates a magical link between a Large or larger inanimate plant
    within range and another plant, at any distance, on the same plane of existence. You must have seen
    or touched the destination plant at least once before. For the duration, any creature can step into
    the target plant and exit from the destination plant by using 5 feet of movement.'
  sha256: c99b7578865949c635d2ae893fba913d1bdf0fd9e4619ebb0614b34e66722fb0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Transport via Plants

<!-- SRD page 170 -->

Level 6 Conjuration (Druid)\
Casting Time: Action\
Range: 10 feet\
Components: V, S\
Duration: 1 minute\
This spell creates a magical link between a Large\
or larger inanimate plant within range and another\
plant, at any distance, on the same plane of exis-\
tence. You must have seen or touched the destina-\
tion plant at least once before. For the duration, any\
creature can step into the target plant and exit from\
the destination plant by using 5 feet of movement.\

Original layout: [[../../srd-5.2.1/pages/170|page 170]].
