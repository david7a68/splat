---
splat_schema: 1
splat_id: 6baed617-b0e7-5653-835c-59bd1b1418b7
splat_kind: item
splat_key: dnd2024:items/medallion_of_thoughts
splat_name: Medallion of Thoughts
splat_origin:
  key: dnd2024:reference/magic_item/medallion_of_thoughts
splat_sources:
- title: SRD 5.2.1
  locator: pp. 230
  url: https://www.dndbeyond.com/srd
---

# Medallion of Thoughts

## Source text

<!-- SRD page 230 -->

Wondrous Item, Uncommon (Requires Attunement)

The medallion has 5 charges. While wearing it, you can expend 1 charge to cast Detect Thoughts (save DC 13) from it. The medallion regains 1d4 expended charges daily at dawn.

Original layout: [[../../srd-5.2.1/pages/230|page 230]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/medallion_of_thoughts ` |
| ` implementation ` | ` dnd2024:item_medallion_of_thoughts ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 5
maximum: 5
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 4
  cap_at_maximum: true
```

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc | charge_cost |
| --- | --- | --- |
| ` 0 ` | ` 13 ` | ` 1 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_thoughts ` |
