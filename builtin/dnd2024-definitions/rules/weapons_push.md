---
splat_schema: 1
splat_id: 6815bcf2-fb7e-51d4-9c32-5fb0bf8105ef
splat_kind: rule
splat_key: dnd2024:rules/weapons_push
splat_name: Push
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
---

# Push

## Source text

<!-- SRD page 90 -->

If you hit a creature with this weapon, you can push the creature up to 10 feet straight away from yourself if it is Large or smaller.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/weapons_push ` |
| ` implementation ` | ` dnd2024:rules_weapons_push ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` target_size_max ` | ` large ` |

#### Trigger

<!-- splat:values path=splat_rule/parameters/trigger -->

| Field | Value |
| --- | --- |
| ` weapon_hit ` | ` true ` |
| ` target_kind ` | ` creature ` |

#### Push

<!-- splat:values path=splat_rule/parameters/push -->

| Field | Value |
| --- | --- |
| ` distance_ft_min ` | ` 0 ` |
| ` distance_ft_max ` | ` 10 ` |
| ` direction ` | ` straight_away_from_attacker ` |
