---
splat_schema: 2
splat_id: 840f9425-7fe7-5ec3-be15-31386b8e8a32
splat_kind: item
splat_key: dnd2024:items/folding_boat
splat_name: Folding Boat
splat_origin: dnd2024:reference/magic_item/folding_boat
splat_sources:
- SRD 5.2.1, pp. 223, https://www.dndbeyond.com/srd
---

# Folding Boat


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 223 -->

Wondrous Item, Rare

This object appears as a wooden box that measures 12 inches long, 6 inches wide, and 6 inches deep. It weighs 4 pounds and floats. It can be opened to store items inside. This item also has three command words, each requiring a Magic action to use:

First Command Word. The box unfolds into a Rowboat.

Second Command Word. The box unfolds into a Keelboat.

Third Command Word. The Folding Boat folds back into a box if no creatures are aboard. Any objects in the vessel that can’t fit inside the box remain outside the box as it folds. Any objects in the vessel that can fit inside the box do so.

When the box becomes a vessel, its weight becomes that of a normal vessel its size, and anything that was stored in the box remains in the boat.

Statistics for the Rowboat and Keelboat appear in “Equipment.” If either vessel is reduced to 0 Hit Points, the Folding Boat is destroyed.

Original layout: [[../../srd-5.2.1/pages/223|page 223]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/folding_boat ` |
| ` implementation ` | ` dnd2024:item_folding_boat ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` false ` |

#### Box

<!-- splat:values path=splat_rule/parameters/box -->

| Field | Value |
| --- | --- |
| ` dimensions_inches ` | ` [12, 6, 6] ` |
| ` weight_lb ` | ` 4 ` |
| ` floats ` | ` true ` |
| ` openable_storage ` | ` true ` |

#### Transformation

<!-- splat:yaml path=splat_rule/parameters/transformation -->

```yaml
action: magic
command_words:
  first: rowboat
  second: keelboat
  third: box
box_requires_no_creatures_aboard: true
objects_on_folding:
  fits_box: inside_box
  does_not_fit_box: outside_box
objects_on_unfolding: remain_inside_vessel
vessel_weight: normal_vessel_weight
vessel_statistics:
  rowboat:
    ac: 11
    hp: 50
    speed_mph: 1.5
    crew: 1
    passengers: 3
    cargo_tons: 0
    damage_threshold: 0
    weight_lb: 100
  keelboat:
    ac: 15
    hp: 100
    speed_mph: 1
    crew: 1
    passengers: 6
    cargo_tons: 0.5
    damage_threshold: 10
zero_vessel_hp: item_destroyed
vehicle_rules:
- key: dnd2024:rules/mounts_and_vehicles_large_vehicles
- key: dnd2024:rules/mounts_and_vehicles_speed
- key: dnd2024:rules/mounts_and_vehicles_crew
```
