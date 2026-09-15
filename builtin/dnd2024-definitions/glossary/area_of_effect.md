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
---

# Area of Effect

## Source text

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

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/area_of_effect ` |
| ` implementation ` | ` dnd2024:glossary_area_of_effect ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` shapes ` | ` ["cone", "cube", "cylinder", "emanation", "line", "sphere"] ` |
| ` origin ` | ` shape_defined ` |

#### Location excluded when

<!-- splat:values path=splat_rule/parameters/location_excluded_when -->

| Field | Value |
| --- | --- |
| ` all_straight_lines_from_origin_blocked ` | ` true ` |
| ` blocking_obstruction_cover ` | ` total ` |

#### Unseen origin behind obstruction

<!-- splat:values path=splat_rule/parameters/unseen_origin_behind_obstruction -->

| Field | Value |
| --- | --- |
| ` replace_origin ` | ` near_side_of_obstruction ` |

#### Geometry assumptions

<!-- splat:values path=splat_rule/parameters/geometry_assumptions -->

| Field | Value |
| --- | --- |
| ` provider ` | ` external_geometry ` |
| ` required_facts ` | ` ["straight_line_blockage", "total_cover", "creator_visibility", "nearest_obstruction_near_side"] ` |
