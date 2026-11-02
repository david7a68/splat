---
splat_schema: 1
splat_id: 2335a66c-5d9e-568f-bcc0-994e8ea396d0
splat_kind: item
splat_key: dnd2024:items/ring_of_three_wishes
splat_name: Ring of Three Wishes
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_three_wishes
splat_sources:
- title: SRD 5.2.1
  locator: pp. 239
  url: https://www.dndbeyond.com/srd
---

# Ring of Three Wishes

## Source text

<!-- SRD page 239 -->

Ring, Legendary

While wearing this ring, you can expend 1 of its 3 charges to cast Wish from it. The ring becomes nonmagical when you use the last charge.

Original layout: [[../../srd-5.2.1/pages/239|page 239]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_three_wishes ` |
| ` implementation ` | ` dnd2024:item_ring_of_three_wishes ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |

#### Charges

<!-- splat:values path=splat_rule/parameters/charges -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 3 ` |
| ` initial ` | ` 3 ` |

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wish ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

#### Last charge

<!-- splat:values path=splat_rule/parameters/last_charge -->

| Field | Value |
| --- | --- |
| ` magical ` | ` false ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_rings ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
