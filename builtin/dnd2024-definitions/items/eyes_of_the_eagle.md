---
splat_schema: 2
splat_id: 5542e450-360d-50dd-a176-b461a8cf4654
splat_kind: item
splat_key: dnd2024:items/eyes_of_the_eagle
splat_name: Eyes of the Eagle
splat_origin: dnd2024:reference/magic_item/eyes_of_the_eagle
splat_sources:
- SRD 5.2.1, pp. 221, https://www.dndbeyond.com/srd
---

# Eyes of the Eagle


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 221 -->

Wondrous Item, Uncommon

These crystal lenses fit over the eyes. While wearing them, you have Advantage on Wisdom (Perception) checks that rely on sight. In conditions of clear visibility, you can make out details of even extremely distant creatures and objects as small as 2 feet across.

Original layout: [[../../srd-5.2.1/pages/221|page 221]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/eyes_of_the_eagle ` |
| ` implementation ` | ` dnd2024:item_eyes_of_the_eagle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_over_eyes ` |

#### Check advantage

<!-- splat:values path=splat_rule/parameters/check_advantage -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` sense ` | ` sight ` |

#### Clear visibility

<!-- splat:yaml path=splat_rule/parameters/clear_visibility -->

```yaml
minimum_detail_object_width_ft: 2
distance_limit: extremely_distant
distance_adjudication:
  chooser: gm
  type: number
  scope: visibility_distance
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
