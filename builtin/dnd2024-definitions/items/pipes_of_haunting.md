---
splat_schema: 1
splat_id: 82fa3bfa-6986-51ce-b6cf-60eef480f825
splat_kind: item
splat_key: dnd2024:items/pipes_of_haunting
splat_name: Pipes of Haunting
splat_origin:
  key: dnd2024:reference/magic_item/pipes_of_haunting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
---

# Pipes of Haunting

## Source text

<!-- SRD page 234 -->

Wondrous Item, Uncommon\
These pipes have 3 charges and regain 1d3 ex-\
pended charges daily at dawn. You can take a Magic\
action to play them and expend 1 charge to create\
an eerie, spellbinding tune. Each creature of your\
choice within 30 feet of you must succeed on a DC\
15 Wisdom saving throw or have the Frightened\
condition for 1 minute. A creature that fails the save\
repeats it at the end of each of its turns, ending the\
effect on itself on a success. A creature that suc-\
ceeds on its save is immune to the effect of these\
pipes for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].

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
| ` mechanic ` | ` dnd2024:items/pipes_of_haunting ` |
| ` implementation ` | ` dnd2024:item_pipes_of_haunting ` |

### Parameters

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    expression: 1d3
    scope: use
  cap: maximum
```

#### Activate

<!-- splat:yaml path=splat_rule/parameters/activate -->

```yaml
action: magic
play_pipes: true
charge_cost: 1
targets:
  chooser: user
  range_ft: 30
save:
  ability: wisdom
  dc: 15
  failure:
    condition: frightened
    duration:
      amount: 1
      unit: minute
  success:
    immune_to_same_pipes_effect:
      amount: 24
      unit: hour
```

#### Repeat save

<!-- splat:yaml path=splat_rule/parameters/repeat_save -->

```yaml
event: target_turn_end
ability: wisdom
dc: 15
success:
  end_effect: true
  immune_to_same_pipes_effect:
    amount: 24
    unit: hour
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
