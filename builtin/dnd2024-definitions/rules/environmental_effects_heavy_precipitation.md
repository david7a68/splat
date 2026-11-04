---
splat_schema: 2
splat_id: 9e214780-fd56-5641-8e5c-a47eebb51296
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_heavy_precipitation
splat_name: Heavy Precipitation
splat_origin: dnd2024:reference/pages/195
splat_sources:
- SRD 5.2.1, pp. 195, https://www.dndbeyond.com/srd
---

# Heavy Precipitation

## Source text

<!-- SRD page 195 -->

Everything within an area of heavy rain or heavy snowfall is Lightly Obscured, and creatures in the area have Disadvantage on all Wisdom (Perception) checks. Heavy rain also extinguishes open flames.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_heavy_precipitation ` |
| ` implementation ` | ` dnd2024:rule_environmental_effects_heavy_precipitation ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weather ` | ` ["heavy_rain", "heavy_snow"] ` |
| ` area_obscurity ` | ` lightly_obscured ` |
| ` wisdom_perception_checks ` | ` disadvantage ` |
| ` heavy_rain_extinguishes_open_flames ` | ` true ` |
