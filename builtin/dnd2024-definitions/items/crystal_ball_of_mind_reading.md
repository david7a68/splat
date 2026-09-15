---
splat_schema: 1
splat_id: 423f448d-388d-5cc1-9cc0-cbce88f96c2e
splat_kind: item
splat_key: dnd2024:items/crystal_ball_of_mind_reading
splat_name: Crystal Ball of Mind Reading
splat_origin:
  key: dnd2024:reference/magic_item/crystal_ball_of_mind_reading
splat_sources:
- title: SRD 5.2.1
  locator: pp. 216
  url: https://www.dndbeyond.com/srd
---

# Crystal Ball of Mind Reading

## Source text

<!-- SRD page 216 -->

Wondrous Item, Legendary (Requires Attunement)\
While touching this crystal orb, you can cast Scrying\
(save DC 17) with it. In addition, you can cast Detect\
Thoughts (save DC 17) targeting creatures you can\
see within 30 feet of the spell’s sensor. You don’t\
need to concentrate on this Detect Thoughts spell\
to maintain it during its duration, but it ends if the\
Scrying spell ends.\

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
| ` mechanic ` | ` dnd2024:items/crystal_ball_of_mind_reading ` |
| ` implementation ` | ` dnd2024:item_crystal_ball_of_mind_reading ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` touching ` |
| ` attunement_required ` | ` true ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc | usage | concentration | ends_on |
| --- | --- | --- | --- | --- |
| ` 0 ` | ` 17 ` | ` at_will ` |  |  |
| ` 1 ` | ` 17 ` | ` at_will ` | ` false ` | ` linked_scrying_ends ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_thoughts ` |

##### Targets

<!-- splat:values path=splat_rule/parameters/spells/1/targets -->

| Field | Value |
| --- | --- |
| ` visible_to_actor ` | ` true ` |
| ` maximum_distance_from_scrying_sensor_ft ` | ` 30 ` |
