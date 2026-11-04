---
splat_schema: 2
splat_id: ed23dedf-b0f0-541b-bead-65706afdde4c
splat_kind: rule
splat_key: dnd2024:rules/the_next_dawn
splat_name: “The Next Dawn”
splat_origin: dnd2024:reference/pages/206
splat_sources:
- SRD 5.2.1, pp. 206, https://www.dndbeyond.com/srd
---

# “The Next Dawn”

## Source text

<!-- SRD page 206 -->

Magic items often have charges or properties that recharge at the next dawn or some other specified time. If such an item is on a world or plane of existence where the specified event doesn’t occur, the GM determines when the item recharges.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/the_next_dawn ` |
| ` implementation ` | ` dnd2024:rules_the_next_dawn ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` recharge_event ` | ` item_specified ` |

#### If recharge event absent on world or plane

<!-- splat:values path=splat_rule/parameters/if_recharge_event_absent_on_world_or_plane -->

| Field | Value |
| --- | --- |
| ` replacement_time_authority ` | ` gm ` |
| ` replacement_time_input ` | ` duration ` |
