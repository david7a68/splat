---
splat_schema: 2
splat_id: 450aaf9d-fa3a-5c68-b39f-96530f4dd743
splat_kind: item
splat_key: dnd2024:items/potion_of_poison
splat_name: Potion of Poison
splat_origin: dnd2024:reference/magic_item/potion_of_poison
splat_sources:
- SRD 5.2.1, pp. 236, https://www.dndbeyond.com/srd
---

# Potion of Poison


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 236 -->

Potion, Uncommon

This concoction looks, smells, and tastes like a Potion of Healing or another beneficial potion. However, it is actually poison masked by illusion magic. Identify reveals its true nature.

If you drink this potion, you take 4d6 Poison damage and must succeed on a DC 13 Constitution saving throw or have the Poisoned condition for 1 hour.

Original layout: [[../../srd-5.2.1/pages/236|page 236]].





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
