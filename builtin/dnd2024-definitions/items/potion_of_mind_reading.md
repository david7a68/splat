---
splat_schema: 1
splat_id: 7db614ca-c115-5745-880e-f780e9772a4d
splat_kind: item
splat_key: dnd2024:items/potion_of_mind_reading
splat_name: Potion of Mind Reading
splat_origin:
  key: dnd2024:reference/magic_item/potion_of_mind_reading
splat_sources:
- title: SRD 5.2.1
  locator: pp. 236
  url: https://www.dndbeyond.com/srd
---

# Potion of Mind Reading

## Source text

<!-- SRD page 236 -->

Potion, Rare\
When you drink this potion, you gain the effect of\
the Detect Thoughts spell (save DC 13) for 10 min-\
utes (no Concentration required).\
This potion’s dense, purple liquid has an ovoid\
cloud of pink floating in it.\

Original layout: [[../../srd-5.2.1/pages/236|page 236]].

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
