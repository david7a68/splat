---
splat_schema: 1
splat_id: e0b35dea-1113-5224-abe1-792ab8accccf
splat_kind: item
splat_key: dnd2024:items/bag_of_tricks
splat_name: Bag of Tricks
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_tricks
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212
  url: https://www.dndbeyond.com/srd
---

# Bag of Tricks

## Source text

<!-- SRD page 212 -->

Wondrous Item, Uncommon\
This bag made from gray, rust, or tan cloth appears\
empty. Reaching inside the bag, however, reveals\
the presence of a small, fuzzy object.\
You can take a Magic action to pull the fuzzy ob-\
ject from the bag and throw it up to 20 feet. When\
the object lands, it transforms into a creature you\
determine by rolling on the table that corresponds\
to the bag’s color. See “Monsters” for the creature’s\
stat block. The creature vanishes at the next dawn\
or when it is reduced to 0 Hit Points.\
The creature is Friendly to you and your allies,\
and it acts immediately after you on your Initiative\
count. You can take a Bonus Action to command\
how the creature moves and what action it takes on\
its next turn, such as attacking an enemy. In the ab-\
sence of such orders, the creature acts in a fashion\
appropriate to its nature.\
Once three fuzzy objects have been pulled from\
the bag, the bag can’t be used again until the next\
dawn.\
Gray Bag of Tricks\
1d8 Creature 1d8 Creature\
1 Weasel 5 Panther\
2 Giant Rat 6 Giant Badger\
3 Badger 7 Dire Wolf\
4 Boar 8 Giant Elk\
Rust Bag of Tricks\
1d8 Creature 1d8 Creature\
1 Rat 5 Giant Goat\
2 Owl 6 Giant Boar\
3 Mastiff 7 Lion\
4 Goat 8 Brown Bear\
Tan Bag of Tricks\
1d8 Creature 1d8 Creature\
1 Jackal 5 Black Bear\
2 Ape 6 Giant Weasel\
3 Baboon 7 Giant Hyena\
4 Axe Beak 8 Tiger\

Original layout: [[../../srd-5.2.1/pages/212|page 212]].

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

<!-- splat:yaml path=splat_rule/parameters/variants -->

```yaml
gray:
  roll:
    expression: 1d8
    scope: use
  rows:
  - min: 1
    max: 1
    result:
      creature:
        key: dnd2024:creatures/weasel
  - min: 2
    max: 2
    result:
      creature:
        key: dnd2024:creatures/giant_rat
  - min: 3
    max: 3
    result:
      creature:
        key: dnd2024:creatures/badger
  - min: 4
    max: 4
    result:
      creature:
        key: dnd2024:creatures/boar
  - min: 5
    max: 5
    result:
      creature:
        key: dnd2024:creatures/panther
  - min: 6
    max: 6
    result:
      creature:
        key: dnd2024:creatures/giant_badger
  - min: 7
    max: 7
    result:
      creature:
        key: dnd2024:creatures/dire_wolf
  - min: 8
    max: 8
    result:
      creature:
        key: dnd2024:creatures/giant_elk
rust:
  roll:
    expression: 1d8
    scope: use
  rows:
  - min: 1
    max: 1
    result:
      creature:
        key: dnd2024:creatures/rat
  - min: 2
    max: 2
    result:
      creature:
        key: dnd2024:creatures/owl
  - min: 3
    max: 3
    result:
      creature:
        key: dnd2024:creatures/mastiff
  - min: 4
    max: 4
    result:
      creature:
        key: dnd2024:creatures/goat
  - min: 5
    max: 5
    result:
      creature:
        key: dnd2024:creatures/giant_goat
  - min: 6
    max: 6
    result:
      creature:
        key: dnd2024:creatures/giant_boar
  - min: 7
    max: 7
    result:
      creature:
        key: dnd2024:creatures/lion
  - min: 8
    max: 8
    result:
      creature:
        key: dnd2024:creatures/brown_bear
tan:
  roll:
    expression: 1d8
    scope: use
  rows:
  - min: 1
    max: 1
    result:
      creature:
        key: dnd2024:creatures/jackal
  - min: 2
    max: 2
    result:
      creature:
        key: dnd2024:creatures/ape
  - min: 3
    max: 3
    result:
      creature:
        key: dnd2024:creatures/baboon
  - min: 4
    max: 4
    result:
      creature:
        key: dnd2024:creatures/axe_beak
  - min: 5
    max: 5
    result:
      creature:
        key: dnd2024:creatures/black_bear
  - min: 6
    max: 6
    result:
      creature:
        key: dnd2024:creatures/giant_weasel
  - min: 7
    max: 7
    result:
      creature:
        key: dnd2024:creatures/giant_hyena
  - min: 8
    max: 8
    result:
      creature:
        key: dnd2024:creatures/tiger
```

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
