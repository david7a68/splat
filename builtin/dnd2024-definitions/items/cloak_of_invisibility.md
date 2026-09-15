---
splat_schema: 1
splat_id: f3314c8d-5295-5e5d-8945-3625bab644fc
splat_kind: item
splat_key: dnd2024:items/cloak_of_invisibility
splat_name: Cloak of Invisibility
splat_origin:
  key: dnd2024:reference/magic_item/cloak_of_invisibility
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215, 216
  url: https://www.dndbeyond.com/srd
---

# Cloak of Invisibility

## Source text

<!-- SRD page 215 -->

Wondrous Item, Legendary (Requires Attunement)\
This cloak has 3 charges and regains 1d3 expended\
charges daily at dawn. While wearing the cloak, you\
can take a Magic action to pull its hood over your\

<!-- SRD page 216 -->

head and expend 1 charge to give yourself the Invis-\
ible condition for 1 hour. The effect ends early if you\
pull the hood down (no action required) or cease\
wearing the cloak.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]], [[../../srd-5.2.1/pages/216|page 216]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/cloak_of_invisibility ` |
| ` implementation ` | ` dnd2024:item_cloak_of_invisibility ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 3
    bonus: 0
```

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
gesture: pull_hood_up
cost: 1
condition: invisible
duration:
  value: 1
  unit: hour
end_on:
- hood_lowered
- cloak_removed
lower_hood_action: none
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
