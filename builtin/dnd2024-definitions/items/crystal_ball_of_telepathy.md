---
splat_schema: 1
splat_id: 35b045cf-cf79-5cba-a8d6-201145db8236
splat_kind: item
splat_key: dnd2024:items/crystal_ball_of_telepathy
splat_name: Crystal Ball of Telepathy
splat_origin:
  key: dnd2024:reference/magic_item/crystal_ball_of_telepathy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Crystal Ball of Telepathy

## Source text

<!-- SRD page 216 -->

Wondrous Item, Legendary (Requires Attunement)

While touching this crystal orb, you can cast Scrying (save DC 17) with it. In addition, you can communicate telepathically with creatures you can see within 30 feet of the spell’s sensor. You can also cast Suggestion (save DC 17) through the sensor on one of those creatures. You don’t need to concentrate on this Suggestion to maintain it during its duration, but it ends if Scrying ends. You can’t cast Suggestion in this way again until the next dawn.

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
| ` mechanic ` | ` dnd2024:items/crystal_ball_of_telepathy ` |
| ` implementation ` | ` dnd2024:item_crystal_ball_of_telepathy ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` touching ` |

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- scrying
- suggestion
columns:
  Spell:
    path: spell
    type: reference
  Save dc:
    path: save_dc
    type: integer
  Resource uses:
    path: resource/uses
    type: integer
    optional: true
  Resource recovery:
    path: resource/recovery
    type: symbol
    optional: true
  Target seen:
    path: target/seen
    type: boolean
    optional: true
  Target within sensor ft:
    path: target/within_sensor_ft
    type: integer
    optional: true
  Origin:
    path: origin
    type: symbol
    optional: true
  Concentration:
    path: concentration
    type: boolean
    optional: true
  End on:
    path: end_on
    type: yaml
    optional: true
-->

| Spell | Save dc | Resource uses | Resource recovery | Target seen | Target within sensor ft | Origin | Concentration | End on |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` 17 ` |  |  |  |  |  |  |  |
| [Suggestion](<../spells/suggestion.md> "dnd2024:spells/suggestion") | ` 17 ` | ` 1 ` | ` Dawn ` | true | ` 30 ` | ` Scrying sensor ` | false | ` ["scrying_ends"] ` |

#### Telepathy

<!-- splat:values path=splat_rule/parameters/telepathy -->

| Field | Value |
| --- | --- |
| ` targets ` | ` seen_creatures ` |
| ` within_sensor_ft ` | ` 30 ` |
| ` requires ` | ` scrying_active ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
