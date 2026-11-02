---
splat_schema: 1
splat_id: d2c2ad7c-5657-584a-839d-b729a774bf16
splat_kind: rule_group
splat_key: dnd2024:subclasses/druid_circle_of_the_land
splat_name: Circle of the Land
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
---

# Circle of the Land

## Source text

<!-- SRD page 46 -->

### Celebrate Connection to the Natural World

The Circle of the Land comprises mystics and sages who safeguard ancient knowledge and rites. These Druids meet within sacred circles of trees or standing stones to whisper primal secrets in Druidic. The circle’s wisest members preside as the chief priests of their communities.

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

### Members

<!-- splat:rows path=splat_rule_group/members -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward ` |
| ` 1 ` | ` dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary ` |
| ` 2 ` | ` dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells ` |
| ` 3 ` | ` dnd2024:class_features/circle_of_the_land_level_3_land_s_aid ` |
| ` 4 ` | ` dnd2024:class_features/circle_of_the_land_level_6_natural_recovery ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/druid_circle_of_the_land ` |
| ` implementation ` | ` dnd2024:subclasses_druid_circle_of_the_land ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` subclass ` | ` circle_of_the_land ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Feature
order:
- circle_of_the_land_level_10_nature_s_ward
- circle_of_the_land_level_14_nature_s_sanctuary
- circle_of_the_land_level_3_circle_of_the_land_spells
- circle_of_the_land_level_3_land_s_aid
- circle_of_the_land_level_6_natural_recovery
columns:
  Feature:
    path: feature
    type: reference
  Minimum class level:
    path: minimum_class_level
    type: integer
-->

| Feature | Minimum class level |
| --- | --- |
| [Level 10: Nature’s Ward](<../class_features/circle_of_the_land_level_10_nature_s_ward.md> "dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward") | ` 10 ` |
| [Level 14: Nature’s Sanctuary](<../class_features/circle_of_the_land_level_14_nature_s_sanctuary.md> "dnd2024:class_features/circle_of_the_land_level_14_nature_s_sanctuary") | ` 14 ` |
| [Level 3: Circle of the Land Spells](<../class_features/circle_of_the_land_level_3_circle_of_the_land_spells.md> "dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells") | ` 3 ` |
| [Level 3: Land’s Aid](<../class_features/circle_of_the_land_level_3_land_s_aid.md> "dnd2024:class_features/circle_of_the_land_level_3_land_s_aid") | ` 3 ` |
| [Level 6: Natural Recovery](<../class_features/circle_of_the_land_level_6_natural_recovery.md> "dnd2024:class_features/circle_of_the_land_level_6_natural_recovery") | ` 6 ` |
