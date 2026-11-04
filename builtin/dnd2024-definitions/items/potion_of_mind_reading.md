---
splat_schema: 2
splat_id: 7db614ca-c115-5745-880e-f780e9772a4d
splat_kind: item
splat_key: dnd2024:items/potion_of_mind_reading
splat_name: Potion of Mind Reading
splat_origin: dnd2024:reference/magic_item/potion_of_mind_reading
splat_sources:
- SRD 5.2.1, pp. 236, https://www.dndbeyond.com/srd
---

# Potion of Mind Reading


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 236 -->

Potion, Rare

When you drink this potion, you gain the effect of the Detect Thoughts spell (save DC 13) for 10 minutes (no Concentration required).

This potion’s dense, purple liquid has an ovoid cloud of pink floating in it.

Original layout: [[../../srd-5.2.1/pages/236|page 236]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_mind_reading ` |
| ` implementation ` | ` dnd2024:item_potion_of_mind_reading ` |

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
  key: dnd2024:spells/detect_thoughts
save_dc: 13
duration:
  amount: 10
  unit: minutes
concentration: false
```
