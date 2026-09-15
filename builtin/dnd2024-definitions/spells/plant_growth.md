---
splat_schema: 1
splat_id: 6f0a27ec-46de-51ee-952c-eac79c4f4d4b
splat_kind: spell
splat_key: dnd2024:spells/plant_growth
splat_name: Plant Growth
splat_origin:
  key: dnd2024:reference/spell/plant_growth
splat_sources:
- title: SRD 5.2.1
  locator: pp. 153
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - bard
  - druid
  - ranger
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action (Overgrowth) or 8 hours (Enrichment)
  ritual: false
  range:
    feet: 150
splat_rule:
  mechanic: dnd2024:spells/plant_growth
  implementation: dnd2024:spell_plant_growth
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    mode_choice:
      caster_choice: true
      determined_by: casting_time
    overgrowth:
      casting_economy: action
      origin:
        chosen_point: true
        range_feet: 150
      shape: sphere
      radius_feet: 100
      affected_plants: normal
      growth: thick_overgrown
      movement_feet_cost_per_foot: 4
      exclusions:
        chooser: caster
        count: any
        area_size: any
        within_spell_area: true
    enrichment:
      casting_seconds: 28800
      origin:
        chosen_point: true
        range_feet: 150
      radius_miles: 0.5
      affected_plants: all
      duration_days: 365
      harvest_food_multiplier: 2
      benefit_limit:
        count: 1
        per: year
        scope: plant
    spatial_resolution:
      source: supplied_world_geometry
      unknown_facts: require_resolution
      infer_unprovided_geometry: false
splat_contracts:
- id: dnd2024:spell_plant_growth
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    origin:
      type: location
      required: true
    mode:
      type: choice
      required: true
    excluded_areas:
      required: false
      type: location_list
    campaign:
      type: campaign
      required: true
    spatial_facts:
      required: false
      type: resolution
  outputs:
  - effects
  - objects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - world_objects
  - areas
  - campaign_time
splat_conversion:
  status: specified
  category: spell
  source_sha256: bdee6214c8cfbd300873b2f20c9aa266ed8c0ff74870d90aa661c7274f857427
  source_path: builtin/srd-5.2.1/spell/plant_growth.md
  pages:
  - 153
  engine_gaps: &id001
  - dnd2024:spell_plant_growth
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: 706f63e677c68bb70b2dd300e6c6bfed2167ebf1414911049dc4b966297fccb7
splat_clauses:
- key: source_rules
  text: '# Plant Growth Level 3 Transmutation (Bard, Druid, Ranger) Casting Time: Action (Overgrowth)
    or 8 hours (Enrichment) Range: 150 feet Components: V, S Duration: Instantaneous This spell channels
    vitality into plants. The casting time you use determines whether the spell has the Overgrowth or
    the Enrichment effect below. Overgrowth. Choose a point within range. All normal plants in a 100-foot-radius
    Sphere centered on that point become thick and overgrown. A creature moving through that area must
    spend 4 feet of movement for every 1 foot it moves. You can exclude one or more areas of any size
    within the spell’s area from being affected. Enrichment. All plants in a half-mile radius centered
    on a point within range become enriched for 365 days. The plants yield twice the normal amount of
    food when harvested. They can benefit from only one Plant Growth per year.'
  sha256: 722ff3892a2df1ebd6f8f35e85fd750ab78bed5b89177eda8a7abab2b37edd68
  status: contract_specified
splat_requires_mechanics: *id001
---

# Plant Growth

<!-- SRD page 153 -->

Level 3 Transmutation (Bard, Druid, Ranger)\
Casting Time: Action (Overgrowth) or\
8 hours (Enrichment)\
Range: 150 feet\
Components: V, S\
Duration: Instantaneous\
This spell channels vitality into plants. The casting\
time you use determines whether the spell has the\
Overgrowth or the Enrichment effect below.\
Overgrowth. Choose a point within range. All\
normal plants in a 100-foot-radius Sphere centered\
on that point become thick and overgrown. A crea-\
ture moving through that area must spend 4 feet of\
movement for every 1 foot it moves. You can exclude\
one or more areas of any size within the spell’s area\
from being affected.\
Enrichment. All plants in a half-mile radius cen-\
tered on a point within range become enriched for\
365 days. The plants yield twice the normal amount\
of food when harvested. They can benefit from only\
one Plant Growth per year.\

Original layout: [[../../srd-5.2.1/pages/153|page 153]].
