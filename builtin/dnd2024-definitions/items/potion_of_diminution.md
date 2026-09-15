---
splat_schema: 1
splat_id: 96634ed3-2452-5736-a380-1fde627182b0
splat_kind: item
splat_key: dnd2024:items/potion_of_diminution
splat_name: Potion of Diminution
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_diminution
splat_sources:
- title: SRD 5.2.1
  locator: pp. 235
  url: https://www.dndbeyond.com/srd
---

# Potion of Diminution

## Source text

<!-- SRD page 235 -->

Potion, Rare\
When you drink this potion, you gain the “reduce”\
effect of the Enlarge/Reduce spell for 1d4 hours (no\
Concentration required).\
The red in the potion’s liquid continuously con-\
tracts to a tiny bead and then expands to color the\
clear liquid around it. Shaking the bottle fails to in-\
terrupt this process.\

Original layout: [[../../srd-5.2.1/pages/235|page 235]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_diminution ` |
| ` implementation ` | ` dnd2024:item_potion_of_diminution ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` potion_drunk ` |
| ` consumed ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
spell:
  key: dnd2024:spells/enlarge_reduce
mode: reduce
duration:
  amount:
    roll:
      count: 1
      sides: 4
  unit: hours
concentration: false
```
