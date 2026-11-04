---
splat_schema: 2
splat_id: e3d615d2-9240-5f55-9ac9-75f993be59c0
splat_kind: item
splat_key: dnd2024:items/brazier_of_commanding_fire_elementals
splat_name: Brazier of Commanding Fire Elementals
splat_origin: dnd2024:reference/magic_item/brazier_of_commanding_fire_elementals
splat_sources:
- SRD 5.2.1, pp. 214, https://www.dndbeyond.com/srd
---

# Brazier of Commanding Fire Elementals


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 214 -->

Wondrous Item, Rare

While you are within 5 feet of this brazier, you can take a Magic action to summon a Fire Elemental. The elemental appears in an unoccupied space as close to the brazier as possible, understands your languages, obeys your commands, and takes its turn immediately after you on your Initiative count. The elemental disappears after 1 hour, when it dies, or when you dismiss it as a Bonus Action. The brazier can’t be used this way again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/214|page 214]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/brazier_of_commanding_fire_elementals ` |
| ` implementation ` | ` dnd2024:item_brazier_of_commanding_fire_elementals ` |

### Parameters

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
distance_ft: 5
resource:
  uses: 1
  recovery: dawn
summon:
  creature:
    key: dnd2024:creatures/fire_elemental
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
