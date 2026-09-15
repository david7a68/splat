---
splat_schema: 1
splat_id: 010ab45f-4997-596e-931f-0707f5a0d543
splat_kind: item
splat_key: dnd2024:items/robe_of_useful_items
splat_name: Robe of Useful Items
splat_origin:
  key: dnd2024:reference/magic_item/robe_of_useful_items
splat_sources:
- title: SRD 5.2.1
  locator: pp. 240, 241
  url: https://www.dndbeyond.com/srd
---

# Robe of Useful Items

## Source text

<!-- SRD page 240 -->

Wondrous Item, Uncommon\
This robe has cloth patches of various shapes and\
colors covering it. While wearing the robe, you can\
take a Magic action to detach one of the patches,\
causing it to become the object or creature it rep-\
resents. Once the last patch is removed, the robe\
becomes an ordinary garment.\
The robe has two of each of the following patches:\
• Bullseye Lantern (filled and lit)\
•  Dagger\
•  Mirror\
•  Pole\
•  Rope (coiled)\
•  Sack\
In addition, the robe has 4d4 other patches. The GM\
chooses the patches or determines them randomly\
by rolling on the following table.\
1d100 Patch\
01–08 Bag of 100 GP\
09–15 Silver coffer (1 foot long, 6 inches wide and\
deep) worth 500 GP\
16–22 Iron door (up to 10 feet wide and 10 feet high,\
barred on one side of your choice), which you\
can place in an opening you can reach; it con-\
forms to fit the opening, attaching and hinging\
itself\
23–30 10 gems worth 100 GP each\
31–44 Wooden ladder (24 feet long)\
45–51 Riding Horse with a Riding Saddle\
52–59 Open pit (a 10-foot Cube), which you can\
place on the ground within 10 feet of yourself\
60–68 4 Potions of Healing\
69–75 Rowboat (12 feet long)\
76–83 Spell Scroll containing one spell of level 1, 2, or\
3 (your choice)\

<!-- SRD page 241 -->

1d100 Patch\
84–90 2 Mastiffs\
91–96 Window (2 feet by 4 feet, up to 2 feet deep),\
which you can place on a vertical surface you\
can reach\
97–00 Portable Ram\

Original layout: [[../../srd-5.2.1/pages/240|page 240]], [[../../srd-5.2.1/pages/241|page 241]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/robe_of_useful_items ` |
| ` implementation ` | ` dnd2024:item_robe_of_useful_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` operation ` | ` detach_patch ` |
| ` count ` | ` 1 ` |
| ` result ` | ` represented_object_or_creature ` |
| ` consume_patch ` | ` true ` |
| ` on_last_patch ` | ` robe_becomes_nonmagical ` |

#### Initial patches

<!-- splat:yaml path=splat_rule/parameters/initial_patches -->

```yaml
fixed:
- definition:
    key: dnd2024:equipment/lantern_bullseye_10_gp
  count: 2
  filled: true
  lit: true
- definition:
    key: dnd2024:equipment/dagger
  count: 2
- definition:
    key: dnd2024:equipment/mirror_5_gp
  count: 2
- definition:
    key: dnd2024:equipment/pole_5_cp
  count: 2
- definition:
    key: dnd2024:equipment/rope_1_gp
  count: 2
  coiled: true
- definition:
    key: dnd2024:equipment/sack_1_cp
  count: 2
additional_count:
  dice: 4
  sides: 4
  bonus: 0
additional_selection:
- gm_choice
- d100_per_patch
table:
- min: 1
  max: 8
  patch:
    kind: bag
    contents:
      gp: 100
- min: 9
  max: 15
  patch:
    kind: silver_coffer
    length_ft: 1
    width_inches: 6
    depth_inches: 6
    value_gp: 500
- min: 16
  max: 22
  patch:
    kind: iron_door
    maximum_width_ft: 10
    maximum_height_ft: 10
    barred_side: user_choice
    placement: reachable_opening
    conform_to_opening: true
    attach_and_hinge: true
- min: 23
  max: 30
  patch:
    kind: gems
    count: 10
    value_each_gp: 100
- min: 31
  max: 44
  patch:
    kind: wooden_ladder
    length_ft: 24
- min: 45
  max: 51
  patch:
    creature:
      key: dnd2024:creatures/riding_horse
    equipment:
      kind: riding_saddle
- min: 52
  max: 59
  patch:
    kind: open_pit
    cube_side_ft: 10
    placement: ground
    range_ft: 10
- min: 60
  max: 68
  patch:
    definition:
      key: dnd2024:equipment/potion_of_healing_50_gp
    count: 4
- min: 69
  max: 75
  patch:
    kind: rowboat
    length_ft: 12
- min: 76
  max: 83
  patch:
    definition:
      key: dnd2024:items/spell_scroll
    spell_choice:
      chooser: user
      allowed_levels:
      - 1
      - 2
      - 3
- min: 84
  max: 90
  patch:
    creature:
      key: dnd2024:creatures/mastiff
    count: 2
- min: 91
  max: 96
  patch:
    kind: window
    dimensions_ft:
    - 2
    - 4
    maximum_depth_ft: 2
    placement: reachable_vertical_surface
- min: 97
  max: 100
  patch:
    kind: portable_ram
additional_table_rolls_independent: true
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
