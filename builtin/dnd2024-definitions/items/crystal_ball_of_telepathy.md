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

Wondrous Item, Legendary (Requires Attunement)\
While touching this crystal orb, you can cast Scry-\
ing (save DC 17) with it. In addition, you can com-\
municate telepathically with creatures you can see\
within 30 feet of the spell’s sensor. You can also cast\
Suggestion (save DC 17) through the sensor on one\
of those creatures. You don’t need to concentrate on\
this Suggestion to maintain it during its duration,\
but it ends if Scrying ends. You can’t cast Suggestion\
in this way again until the next dawn.\

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

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc | origin | concentration | end_on |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` 17 ` |  |  |  |
| ` 1 ` | ` 17 ` | ` scrying_sensor ` | ` false ` | ` ["scrying_ends"] ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/suggestion ` |

##### Resource

<!-- splat:values path=splat_rule/parameters/spells/1/resource -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 1 ` |
| ` recovery ` | ` dawn ` |

##### Target

<!-- splat:values path=splat_rule/parameters/spells/1/target -->

| Field | Value |
| --- | --- |
| ` seen ` | ` true ` |
| ` within_sensor_ft ` | ` 30 ` |

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
