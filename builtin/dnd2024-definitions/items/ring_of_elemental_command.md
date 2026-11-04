---
splat_schema: 2
splat_id: b780ade2-e0a3-5faa-9abf-90701505a948
splat_kind: item
splat_key: dnd2024:items/ring_of_elemental_command
splat_name: Ring of Elemental Command
splat_origin: dnd2024:reference/magic_item/ring_of_elemental_command
splat_sources:
- SRD 5.2.1, pp. 237, 238, https://www.dndbeyond.com/srd
---

# Ring of Elemental Command


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Legendary |
| Attunement | Yes |


## Source text

<!-- SRD page 237 -->

Ring, Legendary (Requires Attunement)

Each Ring of Elemental Command is linked to one of the four Elemental Planes. The GM chooses or randomly determines the linked plane. For example, a Ring of Elemental Command (air) is linked to the Elemental Plane of Air. Every Ring of Elemental Command has the following two properties:

Elemental Bane. While wearing the ring, you have Advantage on attack rolls against Elementals and they have Disadvantage on attack rolls against you.

Elemental Compulsion. While wearing the ring, you can take a Magic action to try to compel an Elemental you see within 60 feet of yourself. The Elemental makes a DC 18 Wisdom saving throw. On a failed save, the Elemental has the Charmed condition until the start your next turn, and you determine what it does with its move and action on its next turn.

Elemental Focus. While wearing the ring, you benefit from additional properties corresponding to the ring’s linked Elemental Plane:

Air. You know Auran, you have Resistance to Lightning damage, and you have a Fly Speed equal to your Speed and can hover.

Earth. You know Terran, and you have Resistance to Acid damage. Terrain composed of rubble, rocks, or dirt isn’t Difficult Terrain for you. In addition, you can move through solid earth or rock as if those areas were Difficult Terrain without disturbing the matter through which you pass. If you end your turn in solid earth or rock, you are shunted out to the nearest unoccupied space you last occupied.

Fire. You know Ignan, and you have Immunity to Fire damage.

Water. You know Aquan, you gain a Swim Speed of 60 feet, and you can breathe underwater.

Spellcasting. The ring has 5 charges and regains 1d4 + 1 expended charges daily at dawn. While wearing the ring, you can cast a spell from it. Choose the spell from the list of available spells based on the Elemental Plane the ring is linked to, as shown in the following table. The table indicates how many charges you must expend to cast the spell, which has a save DC of 18.

<!-- SRD page 238 -->

| Plane | Spells (Charges) |
| --- | --- |
| Air | Chain Lightning (3 charges), Feather Fall (0 charges), Gust of Wind (2 charges), Wind Wall (1 charge) |
| Earth | Earthquake (5 charges), Stone Shape (2 charges), Stoneskin (3 charges), Wall of Stone (3 charges) |
| Fire | Burning Hands (1 charge), Fireball (2 charges), Fire Storm (4 charges), Wall of Fire (3 charges) |
| Water | Create or Destroy Water (1 charge), Ice Storm (2 charges), Tsunami (5 charges), Wall of Ice (3 charges), Water Walk (2 charges) |

Original layout: [[../../srd-5.2.1/pages/237|page 237]], [[../../srd-5.2.1/pages/238|page 238]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_elemental_command ` |
| ` implementation ` | ` dnd2024:item_ring_of_elemental_command ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Variant selection

<!-- splat:values path=splat_rule/parameters/variant_selection -->

| Field | Value |
| --- | --- |
| ` method ` | ` ["gm_choice", "random"] ` |
| ` options ` | ` ["air", "earth", "fire", "water"] ` |
| ` random_distribution ` | ` uniform_four_planes ` |

#### Elemental bane

<!-- splat:values path=splat_rule/parameters/elemental_bane -->

| Field | Value |
| --- | --- |
| ` attacks_against_elementals ` | ` advantage ` |
| ` elemental_attacks_against_user ` | ` disadvantage ` |

#### Elemental compulsion

<!-- splat:yaml path=splat_rule/parameters/elemental_compulsion -->

```yaml
action: magic
target:
  type: elemental
  seen: true
  range_ft: 60
save:
  ability: wisdom
  dc: 18
failure:
  condition: charmed
  duration: until_user_next_turn_start
  user_controls_target_next_turn:
  - move
  - action
```

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 5
initial: 5
recovery:
  event: dawn
  amount:
    dice: 1
    sides: 4
    bonus: 1
```

#### Variants

##### Air

###### Focus

<!-- splat:values path=splat_rule/parameters/variants/air/focus -->

| Field | Value |
| --- | --- |
| ` language ` | ` auran ` |
| ` resistance ` | ` ["lightning"] ` |
| ` fly_speed ` | ` walking_speed ` |
| ` hover ` | ` true ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/variants/air/spells
key: Spell
order:
- chain_lightning
- feather_fall
- gust_of_wind
- wind_wall
columns:
  Spell:
    path: spell
    type: reference
  Cost:
    path: cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Spell | Cost | Save dc |
| --- | --- | --- |
| [Chain Lightning](<../spells/chain_lightning.md> "dnd2024:spells/chain_lightning") | ` 3 ` | ` 18 ` |
| [Feather Fall](<../spells/feather_fall.md> "dnd2024:spells/feather_fall") | ` 0 ` | ` 18 ` |
| [Gust of Wind](<../spells/gust_of_wind.md> "dnd2024:spells/gust_of_wind") | ` 2 ` | ` 18 ` |
| [Wind Wall](<../spells/wind_wall.md> "dnd2024:spells/wind_wall") | ` 1 ` | ` 18 ` |

##### Earth

###### Focus

<!-- splat:yaml path=splat_rule/parameters/variants/earth/focus -->

```yaml
language: terran
resistance:
- acid
ignore_difficult_terrain_materials:
- rubble
- rocks
- dirt
earth_glide:
  materials:
  - solid_earth
  - rock
  movement_cost: difficult_terrain
  disturb_material: false
  if_end_turn_inside: shunt_to_nearest_unoccupied_space_last_occupied
```

###### Spells

<!-- splat:table
path: splat_rule/parameters/variants/earth/spells
key: Spell
order:
- earthquake
- stone_shape
- stoneskin
- wall_of_stone
columns:
  Spell:
    path: spell
    type: reference
  Cost:
    path: cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Spell | Cost | Save dc |
| --- | --- | --- |
| [Earthquake](<../spells/earthquake.md> "dnd2024:spells/earthquake") | ` 5 ` | ` 18 ` |
| [Stone Shape](<../spells/stone_shape.md> "dnd2024:spells/stone_shape") | ` 2 ` | ` 18 ` |
| [Stoneskin](<../spells/stoneskin.md> "dnd2024:spells/stoneskin") | ` 3 ` | ` 18 ` |
| [Wall of Stone](<../spells/wall_of_stone.md> "dnd2024:spells/wall_of_stone") | ` 3 ` | ` 18 ` |

##### Fire

###### Focus

<!-- splat:values path=splat_rule/parameters/variants/fire/focus -->

| Field | Value |
| --- | --- |
| ` language ` | ` ignan ` |
| ` immunity ` | ` ["fire"] ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/variants/fire/spells
key: Spell
order:
- burning_hands
- fireball
- fire_storm
- wall_of_fire
columns:
  Spell:
    path: spell
    type: reference
  Cost:
    path: cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Spell | Cost | Save dc |
| --- | --- | --- |
| [Burning Hands](<../spells/burning_hands.md> "dnd2024:spells/burning_hands") | ` 1 ` | ` 18 ` |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` 2 ` | ` 18 ` |
| [Fire Storm](<../spells/fire_storm.md> "dnd2024:spells/fire_storm") | ` 4 ` | ` 18 ` |
| [Wall of Fire](<../spells/wall_of_fire.md> "dnd2024:spells/wall_of_fire") | ` 3 ` | ` 18 ` |

##### Water

###### Focus

<!-- splat:values path=splat_rule/parameters/variants/water/focus -->

| Field | Value |
| --- | --- |
| ` language ` | ` aquan ` |
| ` swim_speed_ft ` | ` 60 ` |
| ` underwater_breathing ` | ` true ` |

###### Spells

<!-- splat:table
path: splat_rule/parameters/variants/water/spells
key: Spell
order:
- create_or_destroy_water
- ice_storm
- tsunami
- wall_of_ice
- water_walk
columns:
  Spell:
    path: spell
    type: reference
  Cost:
    path: cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Spell | Cost | Save dc |
| --- | --- | --- |
| [Create or Destroy Water](<../spells/create_or_destroy_water.md> "dnd2024:spells/create_or_destroy_water") | ` 1 ` | ` 18 ` |
| [Ice Storm](<../spells/ice_storm.md> "dnd2024:spells/ice_storm") | ` 2 ` | ` 18 ` |
| [Tsunami](<../spells/tsunami.md> "dnd2024:spells/tsunami") | ` 5 ` | ` 18 ` |
| [Wall of Ice](<../spells/wall_of_ice.md> "dnd2024:spells/wall_of_ice") | ` 3 ` | ` 18 ` |
| [Water Walk](<../spells/water_walk.md> "dnd2024:spells/water_walk") | ` 2 ` | ` 18 ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
