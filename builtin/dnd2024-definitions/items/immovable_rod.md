---
splat_schema: 2
splat_id: b72b7339-8954-5e4e-acee-5e8da4331a86
splat_kind: item
splat_key: dnd2024:items/immovable_rod
splat_name: Immovable Rod
splat_origin: dnd2024:reference/magic_item/immovable_rod
splat_sources:
- SRD 5.2.1, pp. 227, https://www.dndbeyond.com/srd
---

# Immovable Rod


## Item

| Field | Value |
| --- | --- |
| Type | Rod |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 227 -->

Rod, Uncommon

This iron rod has a button on one end. You can take a Utilize action to press the button, which causes the rod to become magically fixed in place. Until you or another creature takes a Utilize action to push the button again, the rod doesn’t move, even if it defies gravity. The rod can hold up to 8,000 pounds of weight. More weight causes the rod to deactivate and fall. A creature can take a Utilize action to make a DC 30 Strength (Athletics) check, moving the fixed rod up to 10 feet on a successful check.

Original layout: [[../../srd-5.2.1/pages/227|page 227]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/immovable_rod ` |
| ` implementation ` | ` dnd2024:item_immovable_rod ` |

### Parameters

#### Toggle

<!-- splat:values path=splat_rule/parameters/toggle -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` actor ` | ` any_creature ` |
| ` operation ` | ` press_button ` |
| ` states ` | ` ["fixed", "unfixed"] ` |

#### Fixed

<!-- splat:yaml path=splat_rule/parameters/fixed -->

```yaml
gravity_ignored: true
maximum_load_lb: 8000
overload:
- deactivate
- fall
move:
  action: utilize
  check:
    ability: strength
    skill: athletics
    dc: 30
  success_distance_max_ft: 10
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
