---
splat_schema: 1
splat_id: ea893ce1-636c-54cb-8ddd-fde7402372fc
splat_kind: rule
splat_key: dnd2024:glossary/attitude
splat_name: Attitude
splat_origin:
  key: dnd2024:reference/glossary/attitude
splat_sources:
- title: SRD 5.2.1
  locator: pp. 177
  url: https://www.dndbeyond.com/srd
---

# Attitude

## Source text

<!-- SRD page 177 -->

A monster has a starting attitude toward a player character: Friendly, Hostile, or Indifferent. See also “Friendly,” “Hostile,” “Indifferent,” and “Influence.”

Original layout: [[../../srd-5.2.1/pages/177|page 177]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/attitude ` |
| ` implementation ` | ` dnd2024:glossary_attitude ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` scope ` | ` monster_toward_player_character ` |
| ` states ` | ` ["friendly", "hostile", "indifferent"] ` |
| ` default ` | ` indifferent ` |
