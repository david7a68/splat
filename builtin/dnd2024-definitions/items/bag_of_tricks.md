---
splat_schema: 2
splat_id: e0b35dea-1113-5224-abe1-792ab8accccf
splat_kind: item
splat_key: dnd2024:items/bag_of_tricks
splat_name: Bag of Tricks
splat_origin: dnd2024:reference/magic_item/bag_of_tricks
splat_sources:
- SRD 5.2.1, pp. 212, https://www.dndbeyond.com/srd
---

# Bag of Tricks


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 212 -->

Wondrous Item, Uncommon

This bag made from gray, rust, or tan cloth appears empty. Reaching inside the bag, however, reveals the presence of a small, fuzzy object.

You can take a Magic action to pull the fuzzy object from the bag and throw it up to 20 feet. When the object lands, it transforms into a creature you determine by rolling on the table that corresponds to the bag’s color. See “Monsters” for the creature’s stat block. The creature vanishes at the next dawn or when it is reduced to 0 Hit Points.

The creature is Friendly to you and your allies, and it acts immediately after you on your Initiative count. You can take a Bonus Action to command how the creature moves and what action it takes on its next turn, such as attacking an enemy. In the absence of such orders, the creature acts in a fashion appropriate to its nature.

Once three fuzzy objects have been pulled from the bag, the bag can’t be used again until the next dawn.

**Gray Bag of Tricks**

| 1d8 | Creature | 1d8 | Creature |
| --- | --- | --- | --- |
| 1 | Weasel | 5 | Panther |
| 2 | Giant Rat | 6 | Giant Badger |
| 3 | Badger | 7 | Dire Wolf |
| 4 | Boar | 8 | Giant Elk |

**Rust Bag of Tricks**

| 1d8 | Creature | 1d8 | Creature |
| --- | --- | --- | --- |
| 1 | Rat | 5 | Giant Goat |
| 2 | Owl | 6 | Giant Boar |
| 3 | Mastiff | 7 | Lion |
| 4 | Goat | 8 | Brown Bear |

**Tan Bag of Tricks**

| 1d8 | Creature | 1d8 | Creature |
| --- | --- | --- | --- |
| 1 | Jackal | 5 | Black Bear |
| 2 | Ape | 6 | Giant Weasel |
| 3 | Baboon | 7 | Giant Hyena |
| 4 | Axe Beak | 8 | Tiger |

Original layout: [[../../srd-5.2.1/pages/212|page 212]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/bag_of_tricks ` |
| ` implementation ` | ` dnd2024:item_bag_of_tricks ` |

### Parameters

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    expression: '3'
    scope: use
  cap: maximum
```

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` throw_maximum_ft ` | ` 20 ` |
| ` draw_and_throw ` | ` true ` |

#### Variants

##### Gray

###### Roll

<!-- splat:values path=splat_rule/parameters/variants/gray/roll -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d8 ` |
| ` scope ` | ` use ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/variants/gray/rows
key: Creature
order:
- weasel
- giant_rat
- badger
- boar
- panther
- giant_badger
- dire_wolf
- giant_elk
columns:
  Roll:
    path: ''
    type: range
  Creature:
    path: result/creature
    type: reference
-->

| Roll | Creature |
| --- | --- |
| ` 1 ` | [Weasel](<../creatures/weasel.md> "dnd2024:creatures/weasel") |
| ` 2 ` | [Giant Rat](<../creatures/giant_rat.md> "dnd2024:creatures/giant_rat") |
| ` 3 ` | [Badger](<../creatures/badger.md> "dnd2024:creatures/badger") |
| ` 4 ` | [Boar](<../creatures/boar.md> "dnd2024:creatures/boar") |
| ` 5 ` | [Panther](<../creatures/panther.md> "dnd2024:creatures/panther") |
| ` 6 ` | [Giant Badger](<../creatures/giant_badger.md> "dnd2024:creatures/giant_badger") |
| ` 7 ` | [Dire Wolf](<../creatures/dire_wolf.md> "dnd2024:creatures/dire_wolf") |
| ` 8 ` | [Giant Elk](<../creatures/giant_elk.md> "dnd2024:creatures/giant_elk") |

##### Rust

###### Roll

<!-- splat:values path=splat_rule/parameters/variants/rust/roll -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d8 ` |
| ` scope ` | ` use ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/variants/rust/rows
key: Creature
order:
- rat
- owl
- mastiff
- goat
- giant_goat
- giant_boar
- lion
- brown_bear
columns:
  Roll:
    path: ''
    type: range
  Creature:
    path: result/creature
    type: reference
-->

| Roll | Creature |
| --- | --- |
| ` 1 ` | [Rat](<../creatures/rat.md> "dnd2024:creatures/rat") |
| ` 2 ` | [Owl](<../creatures/owl.md> "dnd2024:creatures/owl") |
| ` 3 ` | [Mastiff](<../creatures/mastiff.md> "dnd2024:creatures/mastiff") |
| ` 4 ` | [Goat](<../creatures/goat.md> "dnd2024:creatures/goat") |
| ` 5 ` | [Giant Goat](<../creatures/giant_goat.md> "dnd2024:creatures/giant_goat") |
| ` 6 ` | [Giant Boar](<../creatures/giant_boar.md> "dnd2024:creatures/giant_boar") |
| ` 7 ` | [Lion](<../creatures/lion.md> "dnd2024:creatures/lion") |
| ` 8 ` | [Brown Bear](<../creatures/brown_bear.md> "dnd2024:creatures/brown_bear") |

##### Tan

###### Roll

<!-- splat:values path=splat_rule/parameters/variants/tan/roll -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d8 ` |
| ` scope ` | ` use ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/variants/tan/rows
key: Creature
order:
- jackal
- ape
- baboon
- axe_beak
- black_bear
- giant_weasel
- giant_hyena
- tiger
columns:
  Roll:
    path: ''
    type: range
  Creature:
    path: result/creature
    type: reference
-->

| Roll | Creature |
| --- | --- |
| ` 1 ` | [Jackal](<../creatures/jackal.md> "dnd2024:creatures/jackal") |
| ` 2 ` | [Ape](<../creatures/ape.md> "dnd2024:creatures/ape") |
| ` 3 ` | [Baboon](<../creatures/baboon.md> "dnd2024:creatures/baboon") |
| ` 4 ` | [Axe Beak](<../creatures/axe_beak.md> "dnd2024:creatures/axe_beak") |
| ` 5 ` | [Black Bear](<../creatures/black_bear.md> "dnd2024:creatures/black_bear") |
| ` 6 ` | [Giant Weasel](<../creatures/giant_weasel.md> "dnd2024:creatures/giant_weasel") |
| ` 7 ` | [Giant Hyena](<../creatures/giant_hyena.md> "dnd2024:creatures/giant_hyena") |
| ` 8 ` | [Tiger](<../creatures/tiger.md> "dnd2024:creatures/tiger") |

#### Summon

<!-- splat:yaml path=splat_rule/parameters/summon -->

```yaml
attitude: friendly_to_user_and_allies
initiative: immediately_after_user_same_count
end:
- next_dawn
- zero_hit_points
command:
  action: bonus_action
  controls:
  - next_turn_movement
  - next_turn_action
without_command: nature_appropriate_behavior
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
