---
splat_schema: 2
splat_id: 010ab45f-4997-596e-931f-0707f5a0d543
splat_kind: item
splat_key: dnd2024:items/robe_of_useful_items
splat_name: Robe of Useful Items
splat_origin: dnd2024:reference/magic_item/robe_of_useful_items
splat_sources:
- SRD 5.2.1, pp. 240, 241, https://www.dndbeyond.com/srd
---

# Robe of Useful Items


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 240 -->

Wondrous Item, Uncommon

This robe has cloth patches of various shapes and colors covering it. While wearing the robe, you can take a Magic action to detach one of the patches, causing it to become the object or creature it represents. Once the last patch is removed, the robe becomes an ordinary garment.

The robe has two of each of the following patches:

• Bullseye Lantern (filled and lit)

•  Dagger

•  Mirror

•  Pole

•  Rope (coiled)

•  Sack

In addition, the robe has 4d4 other patches. The GM chooses the patches or determines them randomly by rolling on the following table.

| 1d100 | Patch |
| --- | --- |
| 01–08 | Bag of 100 GP |
| 09–15 | Silver coffer (1 foot long, 6 inches wide and deep) worth 500 GP |
| 16–22 | Iron door (up to 10 feet wide and 10 feet high, barred on one side of your choice), which you can place in an opening you can reach; it conforms to fit the opening, attaching and hinging itself |
| 23–30 | 10 gems worth 100 GP each |
| 31–44 | Wooden ladder (24 feet long) |
| 45–51 | Riding Horse with a Riding Saddle |
| 52–59 | Open pit (a 10-foot Cube), which you can place on the ground within 10 feet of yourself |
| 60–68 | 4 Potions of Healing |
| 69–75 | Rowboat (12 feet long) |
| 76–83 | Spell Scroll containing one spell of level 1, 2, or 3 (your choice) |

<!-- SRD page 241 -->

| 1d100 | Patch |
| --- | --- |
| 84–90 | 2 Mastiffs |
| 91–96 | Window (2 feet by 4 feet, up to 2 feet deep), which you can place on a vertical surface you can reach |
| 97–00 | Portable Ram |

Original layout: [[../../srd-5.2.1/pages/240|page 240]], [[../../srd-5.2.1/pages/241|page 241]].





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

<!-- splat:values path=splat_rule/parameters/initial_patches -->

| Field | Value |
| --- | --- |
| ` additional_selection ` | ` ["gm_choice", "d100_per_patch"] ` |
| ` additional_table_rolls_independent ` | ` true ` |

##### Fixed

<!-- splat:table
path: splat_rule/parameters/initial_patches/fixed
key: Item
order:
- lantern_bullseye_10_gp
- dagger
- mirror_5_gp
- pole_5_cp
- rope_1_gp
- sack_1_cp
columns:
  Item:
    path: definition
    type: reference
  Count:
    path: count
    type: integer
  Filled:
    path: filled
    type: boolean
    optional: true
  Lit:
    path: lit
    type: boolean
    optional: true
  Coiled:
    path: coiled
    type: boolean
    optional: true
-->

| Item | Count | Filled | Lit | Coiled |
| --- | --- | --- | --- | --- |
| [Lantern, Bullseye (10 GP)](<../equipment/lantern_bullseye_10_gp.md> "dnd2024:equipment/lantern_bullseye_10_gp") | ` 2 ` | true | true |  |
| [Dagger](<../equipment/dagger.md> "dnd2024:equipment/dagger") | ` 2 ` |  |  |  |
| [Mirror (5 GP)](<../equipment/mirror_5_gp.md> "dnd2024:equipment/mirror_5_gp") | ` 2 ` |  |  |  |
| [Pole (5 CP)](<../equipment/pole_5_cp.md> "dnd2024:equipment/pole_5_cp") | ` 2 ` |  |  |  |
| [Rope (1 GP)](<../equipment/rope_1_gp.md> "dnd2024:equipment/rope_1_gp") | ` 2 ` |  |  | true |
| [Sack (1 CP)](<../equipment/sack_1_cp.md> "dnd2024:equipment/sack_1_cp") | ` 2 ` |  |  |  |

##### Additional count

<!-- splat:values path=splat_rule/parameters/initial_patches/additional_count -->

| Field | Value |
| --- | --- |
| ` dice ` | ` 4 ` |
| ` sides ` | ` 4 ` |
| ` bonus ` | ` 0 ` |

##### Table

<!-- splat:rows path=splat_rule/parameters/initial_patches/table -->

| Row | min | max |
| --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 8 ` |
| ` 1 ` | ` 9 ` | ` 15 ` |
| ` 2 ` | ` 16 ` | ` 22 ` |
| ` 3 ` | ` 23 ` | ` 30 ` |
| ` 4 ` | ` 31 ` | ` 44 ` |
| ` 5 ` | ` 45 ` | ` 51 ` |
| ` 6 ` | ` 52 ` | ` 59 ` |
| ` 7 ` | ` 60 ` | ` 68 ` |
| ` 8 ` | ` 69 ` | ` 75 ` |
| ` 9 ` | ` 76 ` | ` 83 ` |
| ` 10 ` | ` 84 ` | ` 90 ` |
| ` 11 ` | ` 91 ` | ` 96 ` |
| ` 12 ` | ` 97 ` | ` 100 ` |

###### Patch

<!-- splat:yaml path=splat_rule/parameters/initial_patches/table/0/patch -->

```yaml
kind: bag
contents:
  gp: 100
```

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/1/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` silver_coffer ` |
| ` length_ft ` | ` 1 ` |
| ` width_inches ` | ` 6 ` |
| ` depth_inches ` | ` 6 ` |
| ` value_gp ` | ` 500 ` |

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/2/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` iron_door ` |
| ` maximum_width_ft ` | ` 10 ` |
| ` maximum_height_ft ` | ` 10 ` |
| ` barred_side ` | ` user_choice ` |
| ` placement ` | ` reachable_opening ` |
| ` conform_to_opening ` | ` true ` |
| ` attach_and_hinge ` | ` true ` |

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/3/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` gems ` |
| ` count ` | ` 10 ` |
| ` value_each_gp ` | ` 100 ` |

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/4/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` wooden_ladder ` |
| ` length_ft ` | ` 24 ` |

###### Patch

<!-- splat:yaml path=splat_rule/parameters/initial_patches/table/5/patch -->

```yaml
creature:
  key: dnd2024:creatures/riding_horse
equipment:
  kind: riding_saddle
```

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/6/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` open_pit ` |
| ` cube_side_ft ` | ` 10 ` |
| ` placement ` | ` ground ` |
| ` range_ft ` | ` 10 ` |

###### Patch

<!-- splat:yaml path=splat_rule/parameters/initial_patches/table/7/patch -->

```yaml
definition:
  key: dnd2024:equipment/potion_of_healing_50_gp
count: 4
```

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/8/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` rowboat ` |
| ` length_ft ` | ` 12 ` |

###### Patch

<!-- splat:yaml path=splat_rule/parameters/initial_patches/table/9/patch -->

```yaml
definition:
  key: dnd2024:items/spell_scroll
spell_choice:
  chooser: user
  allowed_levels:
  - 1
  - 2
  - 3
```

###### Patch

<!-- splat:yaml path=splat_rule/parameters/initial_patches/table/10/patch -->

```yaml
creature:
  key: dnd2024:creatures/mastiff
count: 2
```

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/11/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` window ` |
| ` dimensions_ft ` | ` [2, 4] ` |
| ` maximum_depth_ft ` | ` 2 ` |
| ` placement ` | ` reachable_vertical_surface ` |

###### Patch

<!-- splat:values path=splat_rule/parameters/initial_patches/table/12/patch -->

| Field | Value |
| --- | --- |
| ` kind ` | ` portable_ram ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
