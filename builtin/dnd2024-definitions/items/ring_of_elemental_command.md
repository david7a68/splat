---
splat_schema: 1
splat_id: b780ade2-e0a3-5faa-9abf-90701505a948
splat_kind: item
splat_key: dnd2024:items/ring_of_elemental_command
splat_name: Ring of Elemental Command
splat_origin:
  key: dnd2024:reference/magic_item/ring_of_elemental_command
splat_sources:
- title: SRD 5.2.1
  locator: pp. 237, 238
  url: https://www.dndbeyond.com/srd
---

# Ring of Elemental Command

## Source text

<!-- SRD page 237 -->

Ring, Legendary (Requires Attunement)\
Each Ring of Elemental Command is linked to one of\
the four Elemental Planes. The GM chooses or ran-\
domly determines the linked plane. For example, a\
Ring of Elemental Command (air) is linked to the Ele-\
mental Plane of Air.\
Every Ring of Elemental Command has the follow-\
ing two properties:\
Elemental Bane. While wearing the ring, you have\
Advantage on attack rolls against Elementals and\
they have Disadvantage on attack rolls against\
you.\
Elemental Compulsion. While wearing the ring,\
you can take a Magic action to try to compel an\
Elemental you see within 60 feet of yourself. The\
Elemental makes a DC 18 Wisdom saving throw.\
On a failed save, the Elemental has the Charmed\
condition until the start your next turn, and you\
determine what it does with its move and action\
on its next turn.\
Elemental Focus. While wearing the ring, you\
benefit from additional properties corresponding to\
the ring’s linked Elemental Plane:\
Air. You know Auran, you have Resistance to Light-\
ning damage, and you have a Fly Speed equal to\
your Speed and can hover.\
Earth. You know Terran, and you have Resistance\
to Acid damage. Terrain composed of rubble,\
rocks, or dirt isn’t Difficult Terrain for you. In ad-\
dition, you can move through solid earth or rock\
as if those areas were Difficult Terrain without\
disturbing the matter through which you pass. If\
you end your turn in solid earth or rock, you are\
shunted out to the nearest unoccupied space you\
last occupied.\
Fire. You know Ignan, and you have Immunity to\
Fire damage.\
Water. You know Aquan, you gain a Swim Speed of\
60 feet, and you can breathe underwater.\
Spellcasting. The ring has 5 charges and re-\
gains 1d4 + 1 expended charges daily at dawn.\
While wearing the ring, you can cast a spell from\
it. Choose the spell from the list of available spells\
based on the Elemental Plane the ring is linked to,\
as shown in the following table. The table indicates\
how many charges you must expend to cast the\
spell, which has a save DC of 18.\

<!-- SRD page 238 -->

Plane Spells (Charges)\
Air Chain Lightning (3 charges), Feather Fall (0\
charges), Gust of Wind (2 charges), Wind\
Wall (1 charge)\
Earth Earthquake (5 charges), Stone Shape (2\
charges), Stoneskin (3 charges), Wall of Stone\
(3 charges)\
Fire Burning Hands (1 charge), Fireball (2 charges),\
Fire Storm (4 charges), Wall of Fire (3 charges)\
Water Create or Destroy Water (1 charge), Ice Storm\
(2 charges), Tsunami (5 charges), Wall of Ice\
(3 charges), Water Walk (2 charges)\

Original layout: [[../../srd-5.2.1/pages/237|page 237]], [[../../srd-5.2.1/pages/238|page 238]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` ring ` |
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

<!-- splat:yaml path=splat_rule/parameters/variants -->

```yaml
air:
  focus:
    language: auran
    resistance:
    - lightning
    fly_speed: walking_speed
    hover: true
  spells:
  - spell:
      key: dnd2024:spells/chain_lightning
    cost: 3
    save_dc: 18
  - spell:
      key: dnd2024:spells/feather_fall
    cost: 0
    save_dc: 18
  - spell:
      key: dnd2024:spells/gust_of_wind
    cost: 2
    save_dc: 18
  - spell:
      key: dnd2024:spells/wind_wall
    cost: 1
    save_dc: 18
earth:
  focus:
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
  spells:
  - spell:
      key: dnd2024:spells/earthquake
    cost: 5
    save_dc: 18
  - spell:
      key: dnd2024:spells/stone_shape
    cost: 2
    save_dc: 18
  - spell:
      key: dnd2024:spells/stoneskin
    cost: 3
    save_dc: 18
  - spell:
      key: dnd2024:spells/wall_of_stone
    cost: 3
    save_dc: 18
fire:
  focus:
    language: ignan
    immunity:
    - fire
  spells:
  - spell:
      key: dnd2024:spells/burning_hands
    cost: 1
    save_dc: 18
  - spell:
      key: dnd2024:spells/fireball
    cost: 2
    save_dc: 18
  - spell:
      key: dnd2024:spells/fire_storm
    cost: 4
    save_dc: 18
  - spell:
      key: dnd2024:spells/wall_of_fire
    cost: 3
    save_dc: 18
water:
  focus:
    language: aquan
    swim_speed_ft: 60
    underwater_breathing: true
  spells:
  - spell:
      key: dnd2024:spells/create_or_destroy_water
    cost: 1
    save_dc: 18
  - spell:
      key: dnd2024:spells/ice_storm
    cost: 2
    save_dc: 18
  - spell:
      key: dnd2024:spells/tsunami
    cost: 5
    save_dc: 18
  - spell:
      key: dnd2024:spells/wall_of_ice
    cost: 3
    save_dc: 18
  - spell:
      key: dnd2024:spells/water_walk
    cost: 2
    save_dc: 18
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
