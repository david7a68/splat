---
splat_schema: 1
splat_id: af994011-d082-588c-8cf3-ef3b1e04b0d7
splat_kind: item
splat_key: dnd2024:items/wand_of_magic_detection
splat_name: Wand of Magic Detection
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_magic_detection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
---

# Wand of Magic Detection

## Source text

<!-- SRD page 251 -->

Wand, Uncommon

This wand has 3 charges. While holding it, you can expend 1 charge to cast Detect Magic from it. The wand regains 1d3 expended charges daily at dawn.

Original layout: [[../../srd-5.2.1/pages/251|page 251]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
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
| ` mechanic ` | ` dnd2024:items/wand_of_magic_detection ` |
| ` implementation ` | ` dnd2024:item_wand_of_magic_detection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    expression: 1d3
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wands ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
