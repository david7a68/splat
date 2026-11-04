---
splat_schema: 2
splat_id: 96634ed3-2452-5736-a380-1fde627182b0
splat_kind: item
splat_key: dnd2024:items/potion_of_diminution
splat_name: Potion of Diminution
splat_origin: dnd2024:reference/magic_item/potion_of_diminution
splat_sources:
- SRD 5.2.1, pp. 235, https://www.dndbeyond.com/srd
---

# Potion of Diminution


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 235 -->

Potion, Rare

When you drink this potion, you gain the “reduce” effect of the Enlarge/Reduce spell for 1d4 hours (no Concentration required).

The red in the potion’s liquid continuously contracts to a tiny bead and then expands to color the clear liquid around it. Shaking the bottle fails to interrupt this process.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].





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
