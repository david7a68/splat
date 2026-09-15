---
splat_schema: 1
splat_id: aaf99226-28f3-581e-b5d9-cf306445bff7
splat_kind: rule
splat_key: dnd2024:rules/exploration_light
splat_name: Light
splat_origin:
  key: dnd2024:reference/pages/011
splat_sources:
- title: SRD 5.2.1
  locator: pp. 11
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/exploration_light
  parameters:
    illumination:
      bright_light: normal_vision
      dim_light: lightly_obscured
      darkness: heavily_obscured
  implementation: dnd2024:rules_exploration_light
splat_contracts:
- id: dnd2024:rules_exploration_light
  inputs:
    area:
      type: location
      required: true
    illumination:
      type: choice
      required: true
  outputs:
  - visibility
  events: []
  capabilities:
  - lighting
  - perception
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 11
  engine_gaps: &id001
  - dnd2024:rules_exploration_light
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: ad998bb859981d421fc886f67e069ede623aac3e0d0f158e89ba7aaaf1eca4c5
  parent_name: Exploration
  line_span:
    first: 629
    count: 17
splat_clauses:
- key: source_rules
  text: '# Light The presence or absence of light determines the category of illumination in an area,
    as defined below. Bright Light. Bright Light lets most creatures see normally. Even gloomy days provide
    Bright Light, as do torches, lanterns, fires, and other sources of illumination within a specific
    radius. Dim Light. Dim Light, also called shadows, creates a Lightly Obscured area. An area of Dim
    Light is usually a boundary between Bright Light and surrounding Darkness. The soft light of twilight
    and dawn also counts as Dim Light. A full moon might bathe the land in Dim Light. Darkness. Darkness
    creates a Heavily Obscured area. Characters face Darkness outdoors at night (even most moonlit nights),
    within the confines of an unlit dungeon, or in an area of magical Darkness.'
  sha256: d887391ffba954969bedb8460f4a341409bd2f9bf05c77f8e773e9b7547e87f1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Light

<!-- SRD page 11 -->

The presence or absence of light determines the cat-\
egory of illumination in an area, as defined below.\
Bright Light. Bright Light lets most creatures see\
normally. Even gloomy days provide Bright Light, as\
do torches, lanterns, fires, and other sources of illu-\
mination within a specific radius.\
Dim Light. Dim Light, also called shadows, cre-\
ates a Lightly Obscured area. An area of Dim Light\
is usually a boundary between Bright Light and\
surrounding Darkness. The soft light of twilight and\
dawn also counts as Dim Light. A full moon might\
bathe the land in Dim Light.\
Darkness. Darkness creates a Heavily Obscured\
area. Characters face Darkness outdoors at night\
(even most moonlit nights), within the confines of\
an unlit dungeon, or in an area of magical Darkness.\
