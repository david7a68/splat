---
splat_schema: 1
splat_id: dccf5ec1-52af-5ec6-8992-8da6062e975b
splat_kind: item
splat_key: dnd2024:items/potion_of_speed
splat_name: Potion of Speed
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_speed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Speed

## Source text

<!-- SRD page 236 -->

Potion, Very Rare\
When you drink this potion, you gain the effect of\
the Haste spell for 1 minute (no Concentration re-\
quired) without suffering the wave of lethargy that\
typically occurs when the effect ends.\
This potion’s yellow fluid is streaked with black\
and swirls on its own.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` potion ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

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
