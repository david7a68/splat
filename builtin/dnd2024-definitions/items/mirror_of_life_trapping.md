---
splat_schema: 2
splat_id: 30a92d9d-5865-5439-9f5c-d270d95eeef9
splat_kind: item
splat_key: dnd2024:items/mirror_of_life_trapping
splat_name: Mirror of Life Trapping
splat_origin: dnd2024:reference/magic_item/mirror_of_life_trapping
splat_sources:
- SRD 5.2.1, pp. 230, https://www.dndbeyond.com/srd
---

# Mirror of Life Trapping


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | No |


## Source text

<!-- SRD page 230 -->

Wondrous Item, Very Rare

When this 4-foot-tall, 2-foot-wide mirror is viewed indirectly, its surface shows faint images of creatures. The mirror weighs 50 pounds, and it has AC 11, HP 10, Immunity to Poison and Psychic damage, and Vulnerability to Bludgeoning damage. It shatters and is destroyed when reduced to 0 Hit Points.

If the mirror is hanging on a vertical surface and you are within 5 feet of it, you can take a Magic action and use a command word to activate it. It remains activated until you take a Magic action and repeat the command word to deactivate it.

Any creature other than you that sees its reflection in the activated mirror while within 30 feet of the mirror must succeed on a DC 15 Charisma saving throw or be trapped, along with anything it is wearing or carrying, in one of the mirror’s twelve extradimensional cells. A creature that knows the mirror’s nature makes the save with Advantage, and Constructs succeed on the save automatically.

An extradimensional cell is an infinite expanse filled with thick fog that reduces visibility to 10 feet. Creatures trapped in the mirror’s cells don’t age, and they don’t need to eat, drink, or sleep. A creature trapped within a cell can escape using magic that permits planar travel. Otherwise, the creature is confined to the cell until freed.

If the mirror traps a creature but its twelve extradimensional cells are already occupied, the mirror frees one trapped creature at random to accommodate the new prisoner. A freed creature appears in an unoccupied space within sight of the mirror but facing away from it. If the mirror is shattered, all creatures it contains are freed and appear in unoccupied spaces near it.

While within 5 feet of the mirror, you can take a Magic action to name one creature trapped in it or call out a particular cell by number. The creature named or contained in the named cell appears as an image on the mirror’s surface. You and the creature can then communicate.

In a similar way, you can take a Magic action and use a second command word to free one creature trapped in the mirror. The freed creature appears, along with its possessions, in the unoccupied space nearest to the mirror and facing away from it.

Placing the mirror inside an extradimensional space created by a Bag of Holding, Portable Hole, or similar item instantly destroys both items and opens a gate to the Astral Plane. The gate originates where the one item was placed inside the other. Any creature within 10 feet of the gate and not behind Total Cover is sucked through it to a random location on the Astral Plane. The gate then closes. The gate is one-way only and can’t be reopened.

Original layout: [[../../srd-5.2.1/pages/230|page 230]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/mirror_of_life_trapping ` |
| ` implementation ` | ` dnd2024:item_mirror_of_life_trapping ` |

### Parameters

#### Object

<!-- splat:values path=splat_rule/parameters/object -->

| Field | Value |
| --- | --- |
| ` height_ft ` | ` 4 ` |
| ` width_ft ` | ` 2 ` |
| ` weight_lb ` | ` 50 ` |
| ` armor_class ` | ` 11 ` |
| ` hit_points ` | ` 10 ` |
| ` immunity ` | ` ["poison", "psychic"] ` |
| ` vulnerability ` | ` ["bludgeoning"] ` |
| ` at_zero_hp ` | ` ["shatter", "destroy"] ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
command_word: activation_word
range_ft: 5
requires: hanging_on_vertical_surface
duration: until_deactivated
deactivation:
  action: magic
  command_word: activation_word
```

#### Capture

<!-- splat:yaml path=splat_rule/parameters/capture -->

```yaml
trigger: creature_sees_own_reflection_while_mirror_active
exclude: activator
range_ft: 30
save:
  ability: charisma
  dc: 15
  advantage_if: knows_mirror_nature
  automatic_success_types:
  - construct
failure:
  move_to: empty_cell
  include: worn_and_carried_possessions
if_all_cells_occupied:
  release: random_prisoner
  then: capture_new_prisoner
trigger_scope: each_reflection_exposure
```

#### Cells

<!-- splat:values path=splat_rule/parameters/cells -->

| Field | Value |
| --- | --- |
| ` count ` | ` 12 ` |
| ` capacity_each ` | ` 1 ` |
| ` space ` | ` infinite ` |
| ` fog_visibility_ft ` | ` 10 ` |
| ` aging ` | ` false ` |
| ` needs_food ` | ` false ` |
| ` needs_water ` | ` false ` |
| ` needs_sleep ` | ` false ` |
| ` escape_methods ` | ` ["planar_travel_magic", "release"] ` |

#### Release placement

<!-- splat:yaml path=splat_rule/parameters/release_placement -->

```yaml
random_replacement:
  unoccupied: true
  within_sight_of: mirror
  facing: away_from_mirror
shattered:
  release: all_prisoners
  unoccupied: true
  near: mirror
command:
  unoccupied: true
  selection: nearest_mirror
  facing: away_from_mirror
  include: possessions
```

#### Communication

<!-- splat:values path=splat_rule/parameters/communication -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` range_ft ` | ` 5 ` |
| ` selector ` | ` ["creature_name", "cell_number"] ` |
| ` display ` | ` prisoner_image ` |
| ` two_way_communication ` | ` true ` |

#### Release command

<!-- splat:values path=splat_rule/parameters/release_command -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` range_ft ` | ` 5 ` |
| ` word ` | ` second_command_word ` |
| ` count ` | ` 1 ` |

#### Nested extradimensional

<!-- splat:yaml path=splat_rule/parameters/nested_extradimensional -->

```yaml
trigger: mirror_placed_inside_item_created_extradimensional_space
examples:
- key: dnd2024:items/bag_of_holding
- key: dnd2024:items/portable_hole
includes_similar_items: true
destroy: both_items
gate:
  origin: nesting_location
  radius_ft: 10
  exclude_total_cover: true
  targets: creatures
  destination:
    plane: astral
    location: random
  one_way: true
  closes_immediately_after_transport: true
  reopenable: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
