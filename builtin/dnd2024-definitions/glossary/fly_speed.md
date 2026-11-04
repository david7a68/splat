---
splat_schema: 2
splat_id: 06a93d1d-494d-59a7-9d12-c3156f10b120
splat_kind: rule
splat_key: dnd2024:glossary/fly_speed
splat_name: Fly Speed
splat_origin: dnd2024:reference/glossary/fly_speed
splat_sources:
- SRD 5.2.1, pp. 182, https://www.dndbeyond.com/srd
---

# Fly Speed

## Source text

<!-- SRD page 182 -->

A Fly Speed can be used to travel through the air. While you have a Fly Speed, you can stay aloft until you land, fall, or die. See also “Flying” and “Speed.”

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/fly_speed ` |
| ` implementation ` | ` dnd2024:glossary_fly_speed ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` movement_mode ` | ` fly ` |
| ` medium ` | ` air ` |
| ` stay_aloft_until_any ` | ` ["land", "fall", "die"] ` |
