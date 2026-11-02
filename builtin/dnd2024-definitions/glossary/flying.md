---
splat_schema: 1
splat_id: f8b2a361-4eb9-5095-a2fc-165a470cecb6
splat_kind: rule
splat_key: dnd2024:glossary/flying
splat_name: Flying
splat_origin:
  key: dnd2024:reference/glossary/flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
---

# Flying

## Source text

<!-- SRD page 182 -->

A variety of effects allow a creature to fly. While flying, you fall if you have the Incapacitated or Prone condition or your Fly Speed is reduced to 0. You can stay aloft in those circumstances if you can hover. See also “Falling” and “Fly Speed.”

Original layout: [[../../srd-5.2.1/pages/182|page 182]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/flying ` |
| ` implementation ` | ` dnd2024:glossary_flying ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` fall_when_any ` | ` ["incapacitated", "prone", "fly_speed_zero"] ` |
| ` hover_prevents_these_falls ` | ` true ` |
