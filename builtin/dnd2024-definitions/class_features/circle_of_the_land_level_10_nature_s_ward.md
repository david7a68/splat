---
splat_schema: 1
splat_id: 2a7aed1d-4207-5101-aac3-ec846256c8cc
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward
splat_name: 'Level 10: Nature’s Ward'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
---

# Level 10: Nature’s Ward

## Source text

<!-- SRD page 46 -->

You are immune to the Poisoned condition, and you\
have Resistance to a damage type associated with\
your current land choice in the Circle Spells feature,\
as shown in the Nature’s Ward table.\
Nature’s Ward\
Land Type Resistance Land Type Resistance\
Arid Fire Temperate Lightning\
Polar Cold Tropical Poison\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/circle_of_the_land_level_10_nature_s_ward ` |
| ` implementation ` | ` dnd2024:class_features_circle_of_the_land_level_10_nature_s_ward ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition_immunities ` | ` ["poisoned"] ` |

#### Damage resistance by current circle land

<!-- splat:values path=splat_rule/parameters/damage_resistance_by_current_circle_land -->

| Field | Value |
| --- | --- |
| ` arid ` | ` fire ` |
| ` polar ` | ` cold ` |
| ` temperate ` | ` lightning ` |
| ` tropical ` | ` poison ` |
