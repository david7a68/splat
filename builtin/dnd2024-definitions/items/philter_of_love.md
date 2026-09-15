---
splat_schema: 1
splat_id: aba2f30b-16d4-5ef4-aa45-cf129d5841ca
splat_kind: item
splat_key: dnd2024:items/philter_of_love
splat_name: Philter of Love
splat_origin:
  key: dnd2024:reference/magic_item/philter_of_love
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
---

# Philter of Love

## Source text

<!-- SRD page 234 -->

Potion, Uncommon\
The next time you see a creature within 10 minutes\
after drinking this philter, you are charmed by that\
creature and have the Charmed condition for 1\
hour.\
This rose-hued, effervescent liquid contains one\
easy-to-miss bubble shaped like a heart.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
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
| ` mechanic ` | ` dnd2024:items/philter_of_love ` |
| ` implementation ` | ` dnd2024:item_philter_of_love ` |

### Parameters

#### Consumption

<!-- splat:values path=splat_rule/parameters/consumption -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` drink ` |
| ` consume_item ` | ` true ` |

#### Delayed effect

<!-- splat:yaml path=splat_rule/parameters/delayed_effect -->

```yaml
trigger: next_creature_seen
window:
  value: 10
  unit: minute
condition: charmed
source: seen_creature
duration:
  value: 1
  unit: hour
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
