---
splat_schema: 1
splat_id: 628203ef-4479-58ab-b84a-0d5d2be14f57
splat_kind: item
splat_key: dnd2024:items/censer_of_controlling_air_elementals
splat_name: Censer of Controlling Air Elementals
splat_origin:
  key: dnd2024:reference/magic_item/censer_of_controlling_air_elementals
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Censer of Controlling Air Elementals

## Source text

<!-- SRD page 215 -->

Wondrous Item, Rare\
While gently swinging this censer, you can take a\
Magic action to summon an Air Elemental. The\
elemental appears in an unoccupied space as close\
to the censer as possible, understands your lan-\
guages, obeys your commands, and takes its turn\
immediately after you on your Initiative count. The\
elemental disappears after 1 hour, when it dies, or\
when you dismiss it as a Bonus Action. The censer\
can’t be used this way again until the next dawn.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/censer_of_controlling_air_elementals ` |
| ` implementation ` | ` dnd2024:item_censer_of_controlling_air_elementals ` |

### Parameters

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
requires: gently_swinging_censer
resource:
  uses: 1
  recovery: dawn
summon:
  creature:
    key: dnd2024:creatures/air_elemental
  placement:
    unoccupied: true
    selection: nearest_item
  understands: summoner_languages
  commands: obey
  initiative: immediately_after_summoner_same_count
  duration:
    value: 1
    unit: hour
  end_on:
  - death
  - summoner_bonus_action_dismissal
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
