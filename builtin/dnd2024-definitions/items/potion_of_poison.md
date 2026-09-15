---
splat_schema: 1
splat_id: 450aaf9d-fa3a-5c68-b39f-96530f4dd743
splat_kind: item
splat_key: dnd2024:items/potion_of_poison
splat_name: Potion of Poison
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_poison
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Poison

## Source text

<!-- SRD page 236 -->

Potion, Uncommon\
This concoction looks, smells, and tastes like a\
Potion of Healing or another beneficial potion. How-\
ever, it is actually poison masked by illusion magic.\
Identify reveals its true nature.\
If you drink this potion, you take 4d6 Poison dam-\
age and must succeed on a DC 13 Constitution sav-\
ing throw or have the Poisoned condition for 1 hour.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

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
| ` mechanic ` | ` dnd2024:items/potion_of_poison ` |
| ` implementation ` | ` dnd2024:item_potion_of_poison ` |

### Parameters

#### Illusion

<!-- splat:yaml path=splat_rule/parameters/illusion -->

```yaml
senses:
- appearance
- smell
- taste
disguise: beneficial_potion
identified_by:
  key: dnd2024:spells/identify
```

#### Consumption

<!-- splat:yaml path=splat_rule/parameters/consumption -->

```yaml
trigger: drink
consume_item: true
damage:
  roll:
    dice: 4
    sides: 6
    bonus: 0
  type: poison
save:
  ability: constitution
  dc: 13
failure:
  condition: poisoned
  duration:
    value: 1
    unit: hour
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
