---
splat_schema: 1
splat_id: cb8eacdd-70dc-5c61-a8f3-6c6e12cefb0e
splat_kind: item
splat_key: dnd2024:items/potion_of_animal_friendship
splat_name: Potion of Animal Friendship
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_animal_friendship
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
---

# Potion of Animal Friendship

## Source text

<!-- SRD page 235 -->

Potion, Uncommon\
When you drink this potion, you can cast the level\
3 version of the Animal Friendship spell (save DC 13).\
Agitating this potion’s muddy liquid brings little\
bits into view: a fish scale, a hummingbird feather, a\
cat claw, or a squirrel hair.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].

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
| ` mechanic ` | ` dnd2024:items/potion_of_animal_friendship ` |
| ` implementation ` | ` dnd2024:item_potion_of_animal_friendship ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` potion_drunk ` |
| ` consumed ` | ` true ` |
| ` cast_timing ` | ` on_drinking ` |
| ` optional_cast ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Cast

<!-- splat:yaml path=splat_rule/parameters/cast -->

```yaml
spell:
  key: dnd2024:spells/animal_friendship
level: 3
save_dc: 13
```
