---
splat_schema: 1
splat_id: ea3231b8-486e-50f6-b3a8-0802bac13882
splat_kind: item
splat_key: dnd2024:items/wind_fan
splat_name: Wind Fan
splat_origin:
  key: dnd2024:reference/magic_item/wind_fan
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
---

# Wind Fan

## Source text

<!-- SRD page 253 -->

Wondrous Item, Uncommon

While holding this fan, you can cast Gust of Wind (save DC 13) from it. Each subsequent time the fan is used before the next dawn, it has a cumulative 20 percent chance of not working; if the fan fails to work, it tears into useless, nonmagical tatters.

Original layout: [[../../srd-5.2.1/pages/253|page 253]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wind_fan ` |
| ` implementation ` | ` dnd2024:item_wind_fan ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc |
| --- | --- |
| ` 0 ` | ` 13 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/gust_of_wind ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
usage_count_reset: dawn
first_use_failure_chance: 0
subsequent_cumulative_increment: 0.2
probability:
  minimum: 0
  maximum: 1
  operation: multiply
  operands:
  - value_from: previous_uses_since_dawn
  - 0.2
on_failure:
- no_spell
- destroy_magic
- tear_item_unusable
roll_before_cast: true
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
