---
splat_schema: 1
splat_id: f910f9d1-71dd-568c-894a-d4fea2b45e0a
splat_kind: item
splat_key: dnd2024:items/winged_boots
splat_name: Winged Boots
splat_origin:
  key: dnd2024:reference/magic_item/winged_boots
splat_sources:
- title: SRD 5.2.1
  locator: pp. 253
  url: https://www.dndbeyond.com/srd
---

# Winged Boots

## Source text

<!-- SRD page 253 -->

Wondrous Item, Uncommon (Requires Attunement)

These boots have 4 charges and regain 1d4 expended charges daily at dawn. While wearing the boots, you can take a Magic action to expend 1 charge, gaining a Fly Speed of 30 feet for 1 hour. If you are flying when the duration expires, you descend at a rate of 30 feet per round until you land.

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
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/winged_boots ` |
| ` implementation ` | ` dnd2024:item_winged_boots ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 4
initial: 4
recovery:
  event: dawn
  amount:
    expression: 1d4
    scope: use
  cap: maximum
```

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: magic
charge_cost: 1
fly_speed_ft: 30
duration:
  amount: 1
  unit: hour
```

#### Expires while flying

<!-- splat:values path=splat_rule/parameters/expires_while_flying -->

| Field | Value |
| --- | --- |
| ` descent_ft_per_round ` | ` 30 ` |
| ` until ` | ` landed ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
