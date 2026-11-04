---
splat_schema: 2
splat_id: b68a6254-8080-5093-abb4-54380a137949
splat_kind: item
splat_key: dnd2024:items/eversmoking_bottle
splat_name: Eversmoking Bottle
splat_origin: dnd2024:reference/magic_item/eversmoking_bottle
splat_sources:
- SRD 5.2.1, pp. 220, 221, https://www.dndbeyond.com/srd
---

# Eversmoking Bottle


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 220 -->

Wondrous Item, Uncommon

As a Magic action, you can open or close this bottle.

Opening the bottle causes thick smoke to billow out, forming a cloud that fills a 60-foot Emanation originating from the bottle. The area within the smoke is Heavily Obscured.

Each minute the bottle remains open, the size of the Emanation increases by 10 feet until it reaches its maximum size of 120 feet.

<!-- SRD page 221 -->

Closing the bottle causes the cloud to become fixed in place until it disperses after 10 minutes. A strong wind (such as that created by the Gust of Wind spell) disperses the cloud after 1 minute.

Original layout: [[../../srd-5.2.1/pages/220|page 220]], [[../../srd-5.2.1/pages/221|page 221]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/eversmoking_bottle ` |
| ` implementation ` | ` dnd2024:item_eversmoking_bottle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` toggle_action ` | ` magic ` |

#### Open

<!-- splat:yaml path=splat_rule/parameters/open -->

```yaml
cloud:
  shape: emanation
  origin: bottle
  radius_ft: 60
  obscurity: heavily_obscured
  growth_ft_per_minute: 10
  maximum_radius_ft: 120
```

#### Closed

<!-- splat:yaml path=splat_rule/parameters/closed -->

```yaml
cloud_location: fixed_at_closing
disperse_after:
  amount: 10
  unit: minute
```

#### Strong wind

<!-- splat:yaml path=splat_rule/parameters/strong_wind -->

```yaml
disperse_after:
  amount: 1
  unit: minute
examples:
- key: dnd2024:spells/gust_of_wind
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
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
