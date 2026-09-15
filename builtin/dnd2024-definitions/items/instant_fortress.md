---
splat_schema: 1
splat_id: 23e0b3bc-2f00-5740-b0c8-97b445edede9
splat_kind: item
splat_key: dnd2024:items/instant_fortress
splat_name: Instant Fortress
splat_origin:
  key: dnd2024:reference/magic_item/instant_fortress
splat_sources:
- title: SRD 5.2.1
  locator: pp. 227
  url: https://www.dndbeyond.com/srd
---

# Instant Fortress

## Source text

<!-- SRD page 227 -->

Wondrous Item, Rare (Requires Attunement)\
As a Magic action, you can place this 1-inch\
adamantine statuette on the ground and, using\
a command word, cause it to grow rapidly into a\
square adamantine tower. Repeating the command\
word causes the tower to revert to statuette form,\
which works only if the tower is empty. Each crea-\
ture in the area where the tower appears is pushed\
to an unoccupied space outside but next to the\
tower. Objects in the area that aren’t being worn or\
carried are also pushed clear of the tower.\
The tower is 20 feet on a side and 30 feet high,\
with arrow slits on all sides and a battlement atop\
it. Its interior is divided into two floors, with a lad-\
der, staircase, or ramp (your choice) connecting\
them. This ladder, staircase, or ramp ends at a trap-\
door leading to the roof. When created, the tower\
has a single door at ground level on the side facing\
you. The door opens only at your command, which\
you can issue as a Bonus Action. It is immune to the\
Knock spell and similar magic.\
Magic prevents the tower from being tipped over.\
The roof, the door, and the walls each have AC 20;\
HP 100; Immunity to Bludgeoning, Piercing, and\
Slashing damage except that which is dealt by siege\
equipment; and Resistance to all other damage.\
Shrinking the tower back down to statuette form\
doesn’t repair damage to the tower. Only a Wish\
spell can repair the tower (this use of the spell\
counts as replicating a spell of level 8 or lower).\
Each casting of Wish causes the tower to regain all\
its Hit Points.\

Original layout: [[../../srd-5.2.1/pages/227|page 227]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/instant_fortress ` |
| ` implementation ` | ` dnd2024:item_instant_fortress ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` statuette_size_inches ` | ` 1 ` |
| ` material ` | ` adamantine ` |

#### Expand

<!-- splat:yaml path=splat_rule/parameters/expand -->

```yaml
action: magic
place_on_ground: true
command_word: true
displace:
  creatures: unoccupied_adjacent_outside_space
  objects: outside_tower
  exclude_worn_carried_objects: true
```

#### Shrink

<!-- splat:values path=splat_rule/parameters/shrink -->

| Field | Value |
| --- | --- |
| ` command_word ` | ` true ` |
| ` requires ` | ` tower_empty ` |
| ` preserve_damage ` | ` true ` |

#### Tower

<!-- splat:yaml path=splat_rule/parameters/tower -->

```yaml
width_ft: 20
length_ft: 20
height_ft: 30
arrow_slits: all_sides
battlement: true
floors: 2
connector:
  chooser: user
  options:
  - ladder
  - staircase
  - ramp
roof_access: trapdoor
ground_door:
  faces: user_at_creation
  open:
    actor: attuned_user
    action: bonus_action
    command: true
  immune_to:
  - key: dnd2024:spells/knock
  immune_to_similar_opening_magic: true
cannot_tip_over: true
component_statistics:
  components:
  - roof
  - door
  - walls
  ac: 20
  hp: 100
  immunity:
  - bludgeoning
  - piercing
  - slashing
  immunity_exception: siege_equipment
  resistance: all_other_damage
```

#### Repair

<!-- splat:yaml path=splat_rule/parameters/repair -->

```yaml
only_spell:
  key: dnd2024:spells/wish
counts_as: replicate_spell_level_8_or_lower
hp_restored: all
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
