---
splat_schema: 1
splat_id: 04f1b738-4d86-580c-b194-c0193acaf6a2
splat_kind: rule
splat_key: dnd2024:rules/exploration_travel
splat_name: Travel
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Travel

## Source text

<!-- SRD page 12 -->

During an adventure, the characters might travel long distances on trips that could take hours or days. The GM can summarize this travel without calculating exact distances or travel times, or the GM might have you use the travel pace rules below.

If you need to know how fast you can move when every second matters, see the movement rules in “Combat” later in “Playing the Game.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_travel ` |
| ` implementation ` | ` dnd2024:rules_exploration_travel ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` time_critical_resolution ` | ` combat_movement ` |

#### Long distance resolution

<!-- splat:values path=splat_rule/parameters/long_distance_resolution -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` choices ` | ` ["summarized", "travel_pace"] ` |

#### Travel pace rule

<!-- splat:values path=splat_rule/parameters/travel_pace_rule -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/exploration_travel_pace ` |
