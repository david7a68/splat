---
splat_schema: 1
splat_id: d0f2dd2f-9c3d-592f-85f4-8423d591d3ce
splat_kind: item
splat_key: dnd2024:items/helm_of_telepathy
splat_name: Helm of Telepathy
splat_origin:
  key: dnd2024:reference/magic_item/helm_of_telepathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
---

# Helm of Telepathy

## Source text

<!-- SRD page 226 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this helm, you have telepathy with a range of 30 feet, and you can cast Detect Thoughts or Suggestion (save DC 13) from the helm. Once either spell is cast from the helm, that spell can’t be cast from it again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

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
| ` mechanic ` | ` dnd2024:items/helm_of_telepathy ` |
| ` implementation ` | ` dnd2024:item_helm_of_telepathy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` telepathy_range_ft ` | ` 30 ` |
| ` attunement_required ` | ` true ` |

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- detect_thoughts
- suggestion
columns:
  Spell:
    path: spell
    type: reference
  Save dc:
    path: save_dc
    type: integer
  Uses maximum:
    path: uses/maximum
    type: integer
  Uses recovery event:
    path: uses/recovery_event
    type: string
  Usage pool:
    path: usage_pool
    type: symbol
-->

| Spell | Save dc | Uses maximum | Uses recovery event | Usage pool |
| --- | --- | --- | --- | --- |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` 13 ` | ` 1 ` | ` time.dawn ` | ` Independent per spell ` |
| [Suggestion](<../spells/suggestion.md> "dnd2024:spells/suggestion") | ` 13 ` | ` 1 ` | ` time.dawn ` | ` Independent per spell ` |
