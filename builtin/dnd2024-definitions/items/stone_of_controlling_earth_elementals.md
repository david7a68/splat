---
splat_schema: 2
splat_id: d3eece81-23d6-5ce1-b296-e84b65be3d85
splat_kind: item
splat_key: dnd2024:items/stone_of_controlling_earth_elementals
splat_name: Stone of Controlling Earth Elementals
splat_origin: dnd2024:reference/magic_item/stone_of_controlling_earth_elementals
splat_sources:
- SRD 5.2.1, pp. 248, https://www.dndbeyond.com/srd
---

# Stone of Controlling Earth Elementals


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 248 -->

Wondrous Item, Rare

While touching this 5-pound stone to the ground, you can take a Magic action to summon an Earth Elemental. The elemental appears in an unoccupied space you choose within 30 feet of yourself, obeys your commands, and takes its turn immediately after you on your Initiative count. The elemental disappears after 1 hour, when it dies, or when you dismiss it as a Bonus Action. The stone can’t be used this way again until the next dawn.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/stone_of_controlling_earth_elementals ` |
| ` implementation ` | ` dnd2024:item_stone_of_controlling_earth_elementals ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` weight_lb ` | ` 5 ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
requires: stone_touching_ground
resource:
  uses: 1
  recovery: dawn
summon:
  creature:
    key: dnd2024:creatures/earth_elemental
  placement:
    unoccupied: true
    chooser: user
    range_ft: 30
  commands: obey
  initiative: immediately_after_user_same_count
  duration:
    value: 1
    unit: hour
  end_on:
  - death
  - user_bonus_action_dismissal
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
