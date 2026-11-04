---
splat_schema: 2
splat_id: a91e7a5c-29f3-5ae1-bd9f-fcc4b5fbc863
splat_kind: item
splat_key: dnd2024:items/potion_of_growth
splat_name: Potion of Growth
splat_origin: dnd2024:reference/magic_item/potion_of_growth
splat_sources:
- SRD 5.2.1, pp. 235, https://www.dndbeyond.com/srd
---

# Potion of Growth


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 235 -->

Potion, Uncommon

When you drink this potion, you gain the “enlarge” effect of the Enlarge/Reduce spell for 10 minutes (no Concentration required).

The red in the potion’s liquid continuously expands from a tiny bead to color the clear liquid around it and then contracts. Shaking the bottle fails to interrupt this process.

Original layout: [[../../srd-5.2.1/pages/235|page 235]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_growth ` |
| ` implementation ` | ` dnd2024:item_potion_of_growth ` |

### Parameters

#### Consumption

<!-- splat:yaml path=splat_rule/parameters/consumption -->

```yaml
trigger: drink
consume_item: true
gain_spell_effect:
  spell:
    key: dnd2024:spells/enlarge_reduce
  mode: enlarge
  duration:
    value: 10
    unit: minute
  concentration: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
