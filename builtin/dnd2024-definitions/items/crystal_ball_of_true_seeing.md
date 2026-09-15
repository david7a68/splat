---
splat_schema: 1
splat_id: dba28332-7e38-5d3e-9fcd-0a534695c20a
splat_kind: item
splat_key: dnd2024:items/crystal_ball_of_true_seeing
splat_name: Crystal Ball of True Seeing
splat_origin:
  key: dnd2024:reference/magic_item/crystal_ball_of_true_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Crystal Ball of True Seeing

## Source text

<!-- SRD page 216 -->

Wondrous Item, Legendary (Requires Attunement)\
While touching this crystal orb, you can cast Scrying\
(save DC 17) with it. In addition, you have Truesight\
with a range of 120 feet centered on the spell’s\
sensor.\

Original layout: [[../../srd-5.2.1/pages/216|page 216]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/crystal_ball_of_true_seeing ` |
| ` implementation ` | ` dnd2024:item_crystal_ball_of_true_seeing ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` touching ` |

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration | save_dc |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 17 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

#### While scrying

<!-- splat:values path=splat_rule/parameters/while_scrying -->

| Field | Value |
| --- | --- |
| ` truesight_ft ` | ` 120 ` |
| ` origin ` | ` spell_sensor ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
