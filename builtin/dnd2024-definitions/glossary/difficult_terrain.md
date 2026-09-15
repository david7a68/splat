---
splat_schema: 1
splat_id: fbd3a3b4-5d1f-5725-b7a3-4b41d20ffdb5
splat_kind: rule
splat_key: dnd2024:glossary/difficult_terrain
splat_name: Difficult Terrain
splat_origin:
  key: dnd2024:reference/glossary/difficult_terrain
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/difficult_terrain
  parameters:
    additional_cost_feet_per_foot: 1
    stacks: false
    qualifying_any:
    - creature_present: true
      creature_not_tiny: true
      creature_not_ally: true
    - furniture_size_at_least: traveler_size
    - terrain_any:
      - heavy_snow
      - ice
      - rubble
      - undergrowth
    - liquid_depth_min: shin
      liquid_depth_max: waist
    - narrow_opening_size_difference: -1
    - slope_degrees_min: 20
    similar_terrain:
      chooser: gm
      result_type: boolean
      similar_to_listed_terrain_required: true
  implementation: dnd2024:glossary_difficult_terrain
splat_contracts:
- id: dnd2024:glossary_difficult_terrain
  inputs:
    subject:
      type: participant
      required: true
    location:
      type: location
      required: true
    terrain_facts:
      type: resolution
      required: true
  outputs:
  - is_difficult_terrain
  - movement_cost
  events: []
  capabilities:
  - movement
  - geometry
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 059766833265a60b476846c205279b29d1b1a934727519abda8b6ce67cb12541
  source_path: builtin/srd-5.2.1/glossary/difficult_terrain.md
  pages:
  - 181
  engine_gaps: &id001
  - dnd2024:glossary_difficult_terrain
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: dcff24e38e4dd97eb526ab3c7adb2a38470d1737974aa331429386cd00c0f88a
splat_clauses:
- key: source_rules
  text: '# Difficult Terrain If a space is Difficult Terrain, every foot of movement in that space costs
    1 extra foot. For example, moving 5 feet through Difficult Terrain costs 10 feet of movement. Difficult
    Terrain isn’t cumulative; either a space is Difficult Terrain or it isn’t. A space is Difficult Terrain
    if the space contains any of the following or something similar: • A creature that isn’t Tiny or your
    ally • Furniture that is sized for creatures of your size or larger • Heavy snow, ice, rubble, or
    undergrowth • Liquid that’s between shinand waist-deep • A narrow opening sized for a creature one
    size smaller than you • A slope of 20 degrees or more'
  sha256: 8e4015478847743ff0c85e7e649e402583106d1a83409f4c59b1b1399085f128
  status: contract_specified
splat_requires_mechanics: *id001
---

# Difficult Terrain

<!-- SRD page 181 -->

If a space is Difficult Terrain, every foot of move-\
ment in that space costs 1 extra foot. For example,\
moving 5 feet through Difficult Terrain costs 10 feet\
of movement. Difficult Terrain isn’t cumulative; ei-\
ther a space is Difficult Terrain or it isn’t.\
A space is Difficult Terrain if the space contains\
any of the following or something similar:\
• A creature that isn’t Tiny or your ally\
• Furniture that is sized for creatures of your size or\
larger\
• Heavy snow, ice, rubble, or undergrowth\
• Liquid that’s between shin- and waist-deep\
• A narrow opening sized for a creature one size\
smaller than you\
• A slope of 20 degrees or more\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].
