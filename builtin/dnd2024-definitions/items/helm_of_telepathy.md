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

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this helm, you have telepathy with a\
range of 30 feet, and you can cast Detect Thoughts\
or Suggestion (save DC 13) from the helm. Once ei-\
ther spell is cast from the helm, that spell can’t be\
cast from it again until the next dawn.\

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

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc | usage_pool |
| --- | --- | --- |
| ` 0 ` | ` 13 ` | ` independent_per_spell ` |
| ` 1 ` | ` 13 ` | ` independent_per_spell ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_thoughts ` |

##### Uses

<!-- splat:values path=splat_rule/parameters/spells/0/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/suggestion ` |

##### Uses

<!-- splat:values path=splat_rule/parameters/spells/1/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` recovery_event ` | ` time.dawn ` |
