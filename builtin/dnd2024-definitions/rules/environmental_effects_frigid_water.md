---
splat_schema: 2
splat_id: 382c25fc-03ac-52bc-98dd-9248b89393ed
splat_kind: rule
splat_key: dnd2024:rules/environmental_effects_frigid_water
splat_name: Frigid Water
splat_origin: dnd2024:reference/pages/195
splat_sources:
- SRD 5.2.1, pp. 195, https://www.dndbeyond.com/srd
---

# Frigid Water

## Source text

<!-- SRD page 195 -->

A creature can be immersed in frigid water for a number of minutes equal to its Constitution score before suffering any ill effects. Each additional minute spent in frigid water requires the creature to succeed on a DC 10 Constitution saving throw or gain 1 Exhaustion level. Creatures with Resistance or Immunity to Cold damage automatically succeed on the save, as do creatures that are naturally adapted to living in ice-cold water.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/environmental_effects_frigid_water ` |
| ` implementation ` | ` dnd2024:rules_environmental_effects_frigid_water ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` environment ` | ` frigid_water ` |
| ` trigger ` | ` each_additional_immersion_minute ` |
| ` automatic_success_any ` | ` ["cold_resistance", "cold_immunity", "naturally_adapted_to_ice_cold_water"] ` |

#### Safe immersion minutes

<!-- splat:values path=splat_rule/parameters/safe_immersion_minutes -->

| Field | Value |
| --- | --- |
| ` var ` | ` subject.constitution_score ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 10 ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` exhaustion_gain ` | ` 1 ` |
