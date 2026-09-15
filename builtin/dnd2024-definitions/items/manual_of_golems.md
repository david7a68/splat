---
splat_schema: 1
splat_id: aa4e9ca8-b2d8-52e1-9d3b-35342f1a6965
splat_kind: item
splat_key: dnd2024:items/manual_of_golems
splat_name: Manual of Golems
splat_origin:
  key: dnd2024:reference/magic_item/manual_of_golems
splat_sources:
- title: SRD 5.2.1
  locator: pp. 229, 230
  url: https://www.dndbeyond.com/srd
---

# Manual of Golems

## Source text

<!-- SRD page 229 -->

Wondrous Item, Very Rare\
This tome contains information and incantations\
necessary to make a particular type of golem. The\
GM chooses the type or determines it randomly by\
rolling on the accompanying table. To decipher and\
use the manual, you must be a spellcaster with at\
least two level 5 spell slots. A creature that can’t\
use a Manual of Golems and attempts to read it takes\
6d6 Psychic damage.\
To create a golem, you must spend the time shown\
on the table, working without interruption with the\
manual at hand and resting no more than 8 hours\
per day. You must also pay the specified cost to pur-\
chase supplies.\
Once you finish creating the golem, the book is\
consumed in eldritch flames. The golem becomes\
animate when the ashes of the manual are sprinkled\
on it. See “Monsters” for the golem’s stat block. The\
golem is under your control, and it understands and\
obeys your commands.\

<!-- SRD page 230 -->

1d20 Golem Time Cost\
1–5 Clay Golem 30 days 65,000 GP\
6–17 Flesh Golem 60 days 50,000 GP\
18 Iron Golem 120 days 100,000 GP\
19–20 Stone Golem 90 days 80,000 GP\

Original layout: [[../../srd-5.2.1/pages/229|page 229]], [[../../srd-5.2.1/pages/230|page 230]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/manual_of_golems ` |
| ` implementation ` | ` dnd2024:item_manual_of_golems ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` type_selection ` | ` gm_choice_or_roll ` |
| ` type_die ` | ` 20 ` |
| ` attunement_required ` | ` false ` |

#### Eligibility

<!-- splat:values path=splat_rule/parameters/eligibility -->

| Field | Value |
| --- | --- |
| ` spellcaster ` | ` true ` |
| ` level_5_spell_slots_at_least ` | ` 2 ` |

#### Ineligible read attempt

<!-- splat:yaml path=splat_rule/parameters/ineligible_read_attempt -->

```yaml
damage:
  amount:
    roll:
      count: 6
      sides: 6
  type: psychic
```

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | roll | work_days | cost_gp |
| --- | --- | --- | --- |
| ` 0 ` | ` [1, 5] ` | ` 30 ` | ` 65000 ` |
| ` 1 ` | ` [6, 17] ` | ` 60 ` | ` 50000 ` |
| ` 2 ` | ` [18, 18] ` | ` 120 ` | ` 100000 ` |
| ` 3 ` | ` [19, 20] ` | ` 90 ` | ` 80000 ` |

##### Creature

<!-- splat:values path=splat_rule/parameters/variants/0/creature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/clay_golem ` |

##### Creature

<!-- splat:values path=splat_rule/parameters/variants/1/creature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/flesh_golem ` |

##### Creature

<!-- splat:values path=splat_rule/parameters/variants/2/creature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/iron_golem ` |

##### Creature

<!-- splat:values path=splat_rule/parameters/variants/3/creature -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:creatures/stone_golem ` |

#### Crafting

<!-- splat:values path=splat_rule/parameters/crafting -->

| Field | Value |
| --- | --- |
| ` manual_at_hand ` | ` true ` |
| ` uninterrupted ` | ` true ` |
| ` maximum_daily_rest_hours ` | ` 8 ` |
| ` supplies_cost_required ` | ` true ` |
| ` completion ` | ` consume_book_in_flames ` |
| ` animate ` | ` sprinkle_book_ashes_on_golem ` |
| ` control ` | ` creator ` |
| ` understands_and_obeys_commands ` | ` true ` |
