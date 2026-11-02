---
splat_schema: 1
splat_id: 9cfc86c9-7bfb-5579-a4f0-30ab90c2b223
splat_kind: rule
splat_key: dnd2024:glossary/burning_hazard
splat_name: Burning [Hazard]
splat_origin:
  key: dnd2024:reference/glossary/burning_hazard
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
---

# Burning [Hazard]

## Source text

<!-- SRD page 178 -->

A burning creature or object takes 1d4 Fire damage at the start of each of its turns. As an action, you can extinguish fire on yourself by giving yourself the Prone condition and rolling on the ground. The fire also goes out if it is doused, submerged, or suffocated.

Original layout: [[../../srd-5.2.1/pages/178|page 178]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/burning_hazard ` |
| ` implementation ` | ` dnd2024:glossary_burning_hazard ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligible_targets ` | ` ["creature", "object"] ` |
| ` also_ends_when ` | ` ["doused", "submerged", "suffocated"] ` |

#### Damage

<!-- splat:values path=splat_rule/parameters/damage -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 1d4 ` |
| ` type ` | ` fire ` |
| ` timing ` | ` subject_turn_start ` |

#### Self extinguish

<!-- splat:values path=splat_rule/parameters/self_extinguish -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` apply_condition ` | ` prone ` |
| ` required_activity ` | ` roll_on_ground ` |
| ` ends_burning ` | ` true ` |
