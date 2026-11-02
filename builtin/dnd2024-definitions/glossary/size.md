---
splat_schema: 1
splat_id: e4478f08-398a-5c14-a125-dfe9cac2c924
splat_kind: rule
splat_key: dnd2024:glossary/size
splat_name: Size
splat_origin:
  key: dnd2024:reference/glossary/size
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
---

# Size

## Source text

<!-- SRD page 188 -->

A creature or an object belongs to a size category: Tiny, Small, Medium, Large, Huge, or Gargantuan. A creature’s size determines how much space the creature occupies in combat. An object’s size affects its Hit Points. See also “Breaking Objects” and “Playing the Game” (“Combat”).

Original layout: [[../../srd-5.2.1/pages/188|page 188]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/size ` |
| ` implementation ` | ` dnd2024:glossary_size ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ordered_categories ` | ` ["tiny", "small", "medium", "large", "huge", "gargantuan"] ` |
| ` applies_to ` | ` ["creature", "object"] ` |
| ` creature_mechanical_use ` | ` combat_occupied_space ` |
| ` object_mechanical_use ` | ` hit_points ` |
