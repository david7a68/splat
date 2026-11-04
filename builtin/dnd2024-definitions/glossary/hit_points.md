---
splat_schema: 2
splat_id: 72531fa2-5662-5996-89fd-07c5b8af784c
splat_kind: rule
splat_key: dnd2024:glossary/hit_points
splat_name: Hit Points
splat_origin: dnd2024:reference/glossary/hit_points
splat_sources:
- SRD 5.2.1, pp. 183, https://www.dndbeyond.com/srd
---

# Hit Points

## Source text

<!-- SRD page 183 -->

Hit Points (HP) are a measure of how difficult it is to kill or destroy a creature or an object. Damage reduces Hit Points, and healing restores them. You can’t have more Hit Points than your Hit Point maximum, and you can’t have less than 0. See also “Breaking Objects” and “Playing the Game” (“Damage and Healing”).

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/hit_points ` |
| ` implementation ` | ` dnd2024:glossary_hit_points ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligible ` | ` ["creature", "object"] ` |
| ` minimum ` | ` 0 ` |
| ` damage_operation ` | ` subtract ` |
| ` healing_operation ` | ` add ` |

#### Maximum

<!-- splat:values path=splat_rule/parameters/maximum -->

| Field | Value |
| --- | --- |
| ` var ` | ` hit_point_maximum ` |
