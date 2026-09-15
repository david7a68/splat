---
splat_schema: 1
splat_id: 7ab8f18b-a827-56dc-af7d-035bbb3eb42d
splat_kind: rule
splat_key: dnd2024:glossary/area_of_effect
splat_name: Area of Effect
splat_origin:
  key: dnd2024:reference/glossary/area_of_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/area_of_effect
  parameters:
    shapes:
    - cone
    - cube
    - cylinder
    - emanation
    - line
    - sphere
    origin: shape_defined
    location_excluded_when:
      all_straight_lines_from_origin_blocked: true
      blocking_obstruction_cover: total
    unseen_origin_behind_obstruction:
      replace_origin: near_side_of_obstruction
    geometry_assumptions:
      provider: external_geometry
      required_facts:
      - straight_line_blockage
      - total_cover
      - creator_visibility
      - nearest_obstruction_near_side
  implementation: dnd2024:glossary_area_of_effect
splat_contracts:
- id: dnd2024:glossary_area_of_effect
  inputs:
    creator:
      type: participant
      required: true
    origin:
      type: location
      required: true
    area:
      type: effect
      required: true
    geometry:
      type: resolution
      required: true
  outputs:
  - effective_origin
  - included_locations
  events: []
  capabilities:
  - areas
  - geometry
  - visibility
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d03fc30c78ef5d0a888c911c5bf81114784909036aa468000c4a0d0231eab63f
  source_path: builtin/srd-5.2.1/glossary/area_of_effect.md
  pages:
  - 177
  engine_gaps: &id001
  - dnd2024:glossary_area_of_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 741fd9bb686f4809e1ac5ae7f484f5505cff9b91366b5ab9d6d842bc338e643c
splat_clauses:
- key: source_rules
  text: '# Area of Effect The descriptions of many spells and other features specify that they have an
    area of effect, which typically has one of six shapes. These shapes are defined elsewhere in this
    glossary: Cone Cylinder Line Cube Emanation Sphere An area of effect has a point of origin, a location
    from which the effect’s energy erupts. The rules for each shape specify how to position its point
    of origin. If all straight lines extending from the point of origin to a location in the area of effect
    are blocked, that location isn’t included in the area of effect. To block a line, an obstruction must
    provide Total Cover. See also “Cover.” If the creator of an area of effect places it at an unseen
    point and an obstruction—such as a wall— is between the creator and that point, the point of origin
    comes into being on the near side of the obstruction.'
  sha256: a439368f2972ba9931487a31b8adfc6eda76952acab6ddee833b569395ff18b4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Area of Effect

<!-- SRD page 177 -->

The descriptions of many spells and other features\
specify that they have an area of effect, which typ-\
ically has one of six shapes. These shapes are de-\
fined elsewhere in this glossary:\
Cone Cylinder Line\
Cube Emanation Sphere\
An area of effect has a point of origin, a location\
from which the effect’s energy erupts. The rules for\
each shape specify how to position its point of ori-\
gin. If all straight lines extending from the point of\
origin to a location in the area of effect are blocked,\
that location isn’t included in the area of effect.\
To block a line, an obstruction must provide Total\
Cover. See also “Cover.”\
If the creator of an area of effect places it at an\
unseen point and an obstruction—such as a wall—\
is between the creator and that point, the point\
of origin comes into being on the near side of the\
obstruction.\

Original layout: [[../../srd-5.2.1/pages/177|page 177]].
