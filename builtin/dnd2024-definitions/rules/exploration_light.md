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
---

# Light

## Source text

<!-- SRD page 11 -->

The presence or absence of light determines the category of illumination in an area, as defined below.

Bright Light. Bright Light lets most creatures see normally. Even gloomy days provide Bright Light, as do torches, lanterns, fires, and other sources of illumination within a specific radius.

Dim Light. Dim Light, also called shadows, creates a Lightly Obscured area. An area of Dim Light is usually a boundary between Bright Light and surrounding Darkness. The soft light of twilight and dawn also counts as Dim Light. A full moon might bathe the land in Dim Light.

Darkness. Darkness creates a Heavily Obscured area. Characters face Darkness outdoors at night (even most moonlit nights), within the confines of an unlit dungeon, or in an area of magical Darkness.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_light ` |
| ` implementation ` | ` dnd2024:rules_exploration_light ` |

### Parameters

#### Illumination

<!-- splat:values path=splat_rule/parameters/illumination -->

| Field | Value |
| --- | --- |
| ` bright_light ` | ` normal_vision ` |
| ` dim_light ` | ` lightly_obscured ` |
| ` darkness ` | ` heavily_obscured ` |
