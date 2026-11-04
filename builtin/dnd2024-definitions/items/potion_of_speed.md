---
splat_schema: 2
splat_id: dccf5ec1-52af-5ec6-8992-8da6062e975b
splat_kind: item
splat_key: dnd2024:items/potion_of_speed
splat_name: Potion of Speed
splat_origin: dnd2024:reference/magic_item/potion_of_speed
splat_sources:
- SRD 5.2.1, pp. 236, https://www.dndbeyond.com/srd
---

# Potion of Speed


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Very Rare |
| Attunement | No |


## Source text

<!-- SRD page 236 -->

Potion, Very Rare

When you drink this potion, you gain the effect of the Haste spell for 1 minute (no Concentration required) without suffering the wave of lethargy that typically occurs when the effect ends.

This potion’s yellow fluid is streaked with black and swirls on its own.

Original layout: [[../../srd-5.2.1/pages/236|page 236]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_speed ` |
| ` implementation ` | ` dnd2024:item_potion_of_speed ` |

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
  key: dnd2024:spells/haste
duration:
  amount: 1
  unit: minutes
concentration: false
end_lethargy: false
```
