---
splat_schema: 2
splat_id: 5ac9961d-7125-5059-baa5-0d8bd2321e82
splat_kind: item
splat_key: dnd2024:items/potion_of_flying
splat_name: Potion of Flying
splat_origin: dnd2024:reference/magic_item/potion_of_flying
splat_sources:
- SRD 5.2.1, pp. 235, https://www.dndbeyond.com/srd
---

# Potion of Flying


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Very Rare |
| Attunement | No |


## Source text

<!-- SRD page 235 -->

Potion, Very Rare

When you drink this potion, you gain a Fly Speed equal to your Speed for 1 hour and can hover. If you’re in the air when the potion wears off, you fall unless you have some other means of staying aloft.

This potion’s clear liquid floats at the top of its container and has cloudy white impurities drifting in it.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].





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
