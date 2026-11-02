---
splat_schema: 1
splat_id: 99965f86-2526-5f91-9b40-1114afbf6486
splat_kind: item
splat_key: dnd2024:items/belt_of_dwarvenkind
splat_name: Belt of Dwarvenkind
splat_origin:
  key: dnd2024:reference/magic_item/belt_of_dwarvenkind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 213
  url: https://www.dndbeyond.com/srd
---

# Belt of Dwarvenkind

## Source text

<!-- SRD page 213 -->

Wondrous Item, Rare (Requires Attunement)

While wearing this belt, you gain the following benefits:

Dwarvish. You know Dwarvish.

Friend of Dwarvenkind. You have Advantage on Charisma (Persuasion) checks made to interact with dwarves and duergar.

Toughness. Your Constitution increases by 2, to a maximum of 20.

In addition, while attuned to the belt, you have a 50 percent chance each day at dawn of growing a full beard if you can grow one, or a thicker beard if you already have one.

If you aren’t a dwarf or duergar, you gain the following additional benefits while wearing the belt:

Darkvision. You have Darkvision with a range of 60 feet.

Resilience. You have Resistance to Poison damage. You also have Advantage on saving throws you make to avoid or end the Poisoned condition.

Original layout: [[../../srd-5.2.1/pages/213|page 213]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/belt_of_dwarvenkind ` |
| ` implementation ` | ` dnd2024:item_belt_of_dwarvenkind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |
| ` language ` | ` dwarvish ` |

#### Checks

<!-- splat:rows path=splat_rule/parameters/checks -->

| Row | ability | skill | target_ancestries | advantage |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` charisma ` | ` persuasion ` | ` ["dwarf", "duergar"] ` | ` true ` |

#### Ability

<!-- splat:yaml path=splat_rule/parameters/ability -->

```yaml
constitution:
  increase: 2
  maximum: 20
```

#### Beard

<!-- splat:yaml path=splat_rule/parameters/beard -->

```yaml
requires: attuned
event: dawn
chance_percent: 50
requires_can_grow_beard: true
result_by_existing_beard:
  absent: full
  present: thicker
```

#### Non dwarf non duergar

<!-- splat:yaml path=splat_rule/parameters/non_dwarf_non_duergar -->

```yaml
darkvision_ft: 60
damage_resistance: poison
save_advantage:
  avoid_or_end_condition: poisoned
```

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
