---
splat_schema: 1
splat_id: 5ac9961d-7125-5059-baa5-0d8bd2321e82
splat_kind: item
splat_key: dnd2024:items/potion_of_flying
splat_name: Potion of Flying
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
---

# Potion of Flying

## Source text

<!-- SRD page 235 -->

Potion, Very Rare

When you drink this potion, you gain a Fly Speed equal to your Speed for 1 hour and can hover. If you’re in the air when the potion wears off, you fall unless you have some other means of staying aloft.

This potion’s clear liquid floats at the top of its container and has cloudy white impurities drifting in it.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].

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
| ` mechanic ` | ` dnd2024:items/potion_of_flying ` |
| ` implementation ` | ` dnd2024:item_potion_of_flying ` |

### Parameters

#### Consumption

<!-- splat:values path=splat_rule/parameters/consumption -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` drink ` |
| ` consume_item ` | ` true ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
duration:
  value: 1
  unit: hour
fly_speed: walking_speed
hover: true
expiry_airborne: fall_unless_other_means_of_staying_aloft
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
