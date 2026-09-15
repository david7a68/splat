---
splat_schema: 1
splat_id: ba35052c-b52f-5c7e-8261-63005e16e998
splat_kind: item
splat_key: dnd2024:items/potion_of_vitality
splat_name: Potion of Vitality
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_vitality
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Vitality

## Source text

<!-- SRD page 236 -->

Potion, Very Rare\
When you drink this potion, it removes any\
Exhaustion levels you have and ends the Poisoned\
condition on you. For the next 24 hours, you regain\
the maximum number of Hit Points for any Hit Point\
Die you spend.\
This potion’s crimson liquid regularly pulses with\
dull light, calling to mind a heartbeat.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_vitality ` |
| ` implementation ` | ` dnd2024:item_potion_of_vitality ` |

### Parameters

#### Consumption

<!-- splat:values path=splat_rule/parameters/consumption -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` drink ` |
| ` consume_item ` | ` true ` |
| ` remove_exhaustion ` | ` all_levels ` |
| ` end_conditions ` | ` ["poisoned"] ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
duration:
  value: 24
  unit: hour
hit_point_dice_spent: maximum_healing
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
