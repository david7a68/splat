---
splat_schema: 2
splat_id: 09e38628-990e-50a7-87be-311aa6df7f73
splat_kind: item
splat_key: dnd2024:items/bowl_of_commanding_water_elementals
splat_name: Bowl of Commanding Water Elementals
splat_origin: dnd2024:reference/magic_item/bowl_of_commanding_water_elementals
splat_sources:
- SRD 5.2.1, pp. 214, https://www.dndbeyond.com/srd
---

# Bowl of Commanding Water Elementals


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 214 -->

Wondrous Item, Rare

While this bowl is filled with water and you are within 5 feet of it, you can take a Magic action to summon a Water Elemental. The elemental appears in an unoccupied space as close to the bowl as possible, understands your languages, obeys your commands, and takes its turn immediately after you on your Initiative count. The elemental disappears after 1 hour, when it dies, or when you dismiss it as a Bonus Action. The bowl can’t be used this way again until the next dawn.

The bowl is about 1 foot in diameter and half as deep. It holds about 3 gallons.

Original layout: [[../../srd-5.2.1/pages/214|page 214]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bowl_of_commanding_water_elementals ` |
| ` implementation ` | ` dnd2024:item_bowl_of_commanding_water_elementals ` |

### Parameters

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
distance_ft: 5
requires: bowl_filled_with_water
resource:
  uses: 1
  recovery: dawn
summon:
  creature:
    key: dnd2024:creatures/water_elemental
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

#### Container

<!-- splat:values path=splat_rule/parameters/container -->

| Field | Value |
| --- | --- |
| ` diameter_ft ` | ` 1 ` |
| ` depth_ft ` | ` 0.5 ` |
| ` capacity_gallons ` | ` 3 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
