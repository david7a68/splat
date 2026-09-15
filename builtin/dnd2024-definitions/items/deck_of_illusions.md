---
splat_schema: 1
splat_id: 12a23b10-d2a5-5f8f-b36c-8c5fe9570cb3
splat_kind: item
splat_key: dnd2024:items/deck_of_illusions
splat_name: Deck of Illusions
splat_origin:
  key: dnd2024:reference/magic_item/deck_of_illusions
splat_sources:
- title: SRD 5.2.1
  locator: pp. 217, 218
  url: https://www.dndbeyond.com/srd
---

# Deck of Illusions

## Source text

<!-- SRD page 217 -->

Wondrous Item, Uncommon\
This box contains a set of cards. A full deck has 34\
cards: 32 depicting specific creatures and two with\
a mirrored surface. A deck found as treasure is usu-\
ally missing 1d20 − 1 cards.\
The magic of the deck functions only if its cards\
are drawn at random. You can take a Magic action\
to draw a card at random from the deck and throw\
it to the ground at a point within 30 feet of yourself.\
An illusion of a creature, determined by rolling on\
the Deck of Illusions table, forms over the thrown\
card and remains until dispelled. The illusory crea-\
ture created by the card looks and behaves like a\
real creature of its kind, except that it can do no\
harm. While you are within 120 feet of the illusory\
creature and can see it, you can take a Magic action\
to move it anywhere within 30 feet of its card.\
Any physical interaction with the illusory crea-\
ture reveals it to be false, because objects pass\
through it. A creature that takes a Study action to\
visually inspect the illusory creature identifies it as\
an illusion with a successful DC 15 Intelligence (In-\
vestigation) check. The illusion lasts until its card\
is moved or the illusion is dispelled (using a Dispel\
Magic spell or a similar effect). When the illusion\
ends, the image on its card disappears, and that\
card can’t be used again.\
Deck of Illusions\
1d100 Illusion*\
01–03 Adult Red Dragon\
04–06 Archmage\
07–09 Assassin\
10–12 Bandit Captain\
13–15 Basilisk\
16–18 Berserker\
19–21 Bugbear Warrior\
22–24 Cloud Giant\
25–27 Druid\
28–30 Erinyes\
31–33 Ettin\
34–36 Fire Giant\
37–39 Frost Giant\
40–42 Gnoll Warrior\
43–45 Goblin Warrior\
46–48 Guardian Naga\
49–51 Hill Giant\
52–54 Hobgoblin Warrior\
55–57 Incubus\
58–60 Iron Golem\
61–63 Knight\
64–66 Kobold Warrior\
67–69 Lich\
70–72 Medusa\
73–75 Night Hag\
76–78 Ogre\
79–81 Oni\

<!-- SRD page 218 -->

1d100 Illusion*\
82–84 Priest\
85–87 Succubus\
88–90 Troll\
91–93 Veteran Warrior\
94–96 Wyvern\
97–00 The card drawer\
*Stat blocks for these creatures (except the card drawer) appear in\
“Monsters.”\

Original layout: [[../../srd-5.2.1/pages/217|page 217]], [[../../srd-5.2.1/pages/218|page 218]].

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
| ` mechanic ` | ` dnd2024:items/deck_of_illusions ` |
| ` implementation ` | ` dnd2024:item_deck_of_illusions ` |

### Parameters

#### Full deck

<!-- splat:values path=splat_rule/parameters/full_deck -->

| Field | Value |
| --- | --- |
| ` cards ` | ` 34 ` |
| ` specific_creatures ` | ` 32 ` |
| ` mirrored_cards ` | ` 2 ` |

#### Found deck missing

<!-- splat:values path=splat_rule/parameters/found_deck_missing -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d20-1 ` |
| ` scope ` | ` use ` |

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` draw ` | ` random_only ` |
| ` throw_range_ft ` | ` 30 ` |

##### Illusion

###### Roll

<!-- splat:values path=splat_rule/parameters/activate/illusion/roll -->

| Field | Value |
| --- | --- |
| ` expression ` | ` 1d100 ` |
| ` scope ` | ` use ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/activate/illusion/rows
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
- row_17
- row_18
- row_19
- row_20
- row_21
- row_22
- row_23
- row_24
- row_25
- row_26
- row_27
- row_28
- row_29
- row_30
- row_31
- row_32
- row_33
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Details:
    type: section
    optional: true
-->

| Key | Roll | Details |
| --- | --- | --- |
| ` row_1 ` | ` 1–3 ` | [Rules](#^splat-d415bdef-row_1 "section:row_1") |
| ` row_2 ` | ` 4–6 ` | [Rules](#^splat-d415bdef-row_2 "section:row_2") |
| ` row_3 ` | ` 7–9 ` | [Rules](#^splat-d415bdef-row_3 "section:row_3") |
| ` row_4 ` | ` 10–12 ` | [Rules](#^splat-d415bdef-row_4 "section:row_4") |
| ` row_5 ` | ` 13–15 ` | [Rules](#^splat-d415bdef-row_5 "section:row_5") |
| ` row_6 ` | ` 16–18 ` | [Rules](#^splat-d415bdef-row_6 "section:row_6") |
| ` row_7 ` | ` 19–21 ` | [Rules](#^splat-d415bdef-row_7 "section:row_7") |
| ` row_8 ` | ` 22–24 ` | [Rules](#^splat-d415bdef-row_8 "section:row_8") |
| ` row_9 ` | ` 25–27 ` | [Rules](#^splat-d415bdef-row_9 "section:row_9") |
| ` row_10 ` | ` 28–30 ` | [Rules](#^splat-d415bdef-row_10 "section:row_10") |
| ` row_11 ` | ` 31–33 ` | [Rules](#^splat-d415bdef-row_11 "section:row_11") |
| ` row_12 ` | ` 34–36 ` | [Rules](#^splat-d415bdef-row_12 "section:row_12") |
| ` row_13 ` | ` 37–39 ` | [Rules](#^splat-d415bdef-row_13 "section:row_13") |
| ` row_14 ` | ` 40–42 ` | [Rules](#^splat-d415bdef-row_14 "section:row_14") |
| ` row_15 ` | ` 43–45 ` | [Rules](#^splat-d415bdef-row_15 "section:row_15") |
| ` row_16 ` | ` 46–48 ` | [Rules](#^splat-d415bdef-row_16 "section:row_16") |
| ` row_17 ` | ` 49–51 ` | [Rules](#^splat-d415bdef-row_17 "section:row_17") |
| ` row_18 ` | ` 52–54 ` | [Rules](#^splat-d415bdef-row_18 "section:row_18") |
| ` row_19 ` | ` 55–57 ` | [Rules](#^splat-d415bdef-row_19 "section:row_19") |
| ` row_20 ` | ` 58–60 ` | [Rules](#^splat-d415bdef-row_20 "section:row_20") |
| ` row_21 ` | ` 61–63 ` | [Rules](#^splat-d415bdef-row_21 "section:row_21") |
| ` row_22 ` | ` 64–66 ` | [Rules](#^splat-d415bdef-row_22 "section:row_22") |
| ` row_23 ` | ` 67–69 ` | [Rules](#^splat-d415bdef-row_23 "section:row_23") |
| ` row_24 ` | ` 70–72 ` | [Rules](#^splat-d415bdef-row_24 "section:row_24") |
| ` row_25 ` | ` 73–75 ` | [Rules](#^splat-d415bdef-row_25 "section:row_25") |
| ` row_26 ` | ` 76–78 ` | [Rules](#^splat-d415bdef-row_26 "section:row_26") |
| ` row_27 ` | ` 79–81 ` | [Rules](#^splat-d415bdef-row_27 "section:row_27") |
| ` row_28 ` | ` 82–84 ` | [Rules](#^splat-d415bdef-row_28 "section:row_28") |
| ` row_29 ` | ` 85–87 ` | [Rules](#^splat-d415bdef-row_29 "section:row_29") |
| ` row_30 ` | ` 88–90 ` | [Rules](#^splat-d415bdef-row_30 "section:row_30") |
| ` row_31 ` | ` 91–93 ` | [Rules](#^splat-d415bdef-row_31 "section:row_31") |
| ` row_32 ` | ` 94–96 ` | [Rules](#^splat-d415bdef-row_32 "section:row_32") |
| ` row_33 ` | ` 97–100 ` | [Rules](#^splat-d415bdef-row_33 "section:row_33") |

###### Row 1

^splat-d415bdef-row_1

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_1/result -->

```yaml
appearance:
  key: dnd2024:creatures/adult_red_dragon
```

###### Row 2

^splat-d415bdef-row_2

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_2/result -->

```yaml
appearance:
  key: dnd2024:creatures/archmage
```

###### Row 3

^splat-d415bdef-row_3

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_3/result -->

```yaml
appearance:
  key: dnd2024:creatures/assassin
```

###### Row 4

^splat-d415bdef-row_4

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_4/result -->

```yaml
appearance:
  key: dnd2024:creatures/bandit_captain
```

###### Row 5

^splat-d415bdef-row_5

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_5/result -->

```yaml
appearance:
  key: dnd2024:creatures/basilisk
```

###### Row 6

^splat-d415bdef-row_6

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_6/result -->

```yaml
appearance:
  key: dnd2024:creatures/berserker
```

###### Row 7

^splat-d415bdef-row_7

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_7/result -->

```yaml
appearance:
  key: dnd2024:creatures/bugbear_warrior
```

###### Row 8

^splat-d415bdef-row_8

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_8/result -->

```yaml
appearance:
  key: dnd2024:creatures/cloud_giant
```

###### Row 9

^splat-d415bdef-row_9

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_9/result -->

```yaml
appearance:
  key: dnd2024:creatures/druid
```

###### Row 10

^splat-d415bdef-row_10

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_10/result -->

```yaml
appearance:
  key: dnd2024:creatures/erinyes
```

###### Row 11

^splat-d415bdef-row_11

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_11/result -->

```yaml
appearance:
  key: dnd2024:creatures/ettin
```

###### Row 12

^splat-d415bdef-row_12

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_12/result -->

```yaml
appearance:
  key: dnd2024:creatures/fire_giant
```

###### Row 13

^splat-d415bdef-row_13

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_13/result -->

```yaml
appearance:
  key: dnd2024:creatures/frost_giant
```

###### Row 14

^splat-d415bdef-row_14

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_14/result -->

```yaml
appearance:
  key: dnd2024:creatures/gnoll_warrior
```

###### Row 15

^splat-d415bdef-row_15

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_15/result -->

```yaml
appearance:
  key: dnd2024:creatures/goblin_warrior
```

###### Row 16

^splat-d415bdef-row_16

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_16/result -->

```yaml
appearance:
  key: dnd2024:creatures/guardian_naga
```

###### Row 17

^splat-d415bdef-row_17

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_17/result -->

```yaml
appearance:
  key: dnd2024:creatures/hill_giant
```

###### Row 18

^splat-d415bdef-row_18

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_18/result -->

```yaml
appearance:
  key: dnd2024:creatures/hobgoblin_warrior
```

###### Row 19

^splat-d415bdef-row_19

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_19/result -->

```yaml
appearance:
  key: dnd2024:creatures/incubus
```

###### Row 20

^splat-d415bdef-row_20

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_20/result -->

```yaml
appearance:
  key: dnd2024:creatures/iron_golem
```

###### Row 21

^splat-d415bdef-row_21

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_21/result -->

```yaml
appearance:
  key: dnd2024:creatures/knight
```

###### Row 22

^splat-d415bdef-row_22

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_22/result -->

```yaml
appearance:
  key: dnd2024:creatures/kobold_warrior
```

###### Row 23

^splat-d415bdef-row_23

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_23/result -->

```yaml
appearance:
  key: dnd2024:creatures/lich
```

###### Row 24

^splat-d415bdef-row_24

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_24/result -->

```yaml
appearance:
  key: dnd2024:creatures/medusa
```

###### Row 25

^splat-d415bdef-row_25

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_25/result -->

```yaml
appearance:
  key: dnd2024:creatures/night_hag
```

###### Row 26

^splat-d415bdef-row_26

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_26/result -->

```yaml
appearance:
  key: dnd2024:creatures/ogre
```

###### Row 27

^splat-d415bdef-row_27

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_27/result -->

```yaml
appearance:
  key: dnd2024:creatures/oni
```

###### Row 28

^splat-d415bdef-row_28

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_28/result -->

```yaml
appearance:
  key: dnd2024:creatures/priest
```

###### Row 29

^splat-d415bdef-row_29

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_29/result -->

```yaml
appearance:
  key: dnd2024:creatures/succubus
```

###### Row 30

^splat-d415bdef-row_30

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_30/result -->

```yaml
appearance:
  key: dnd2024:creatures/troll
```

###### Row 31

^splat-d415bdef-row_31

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_31/result -->

```yaml
appearance:
  key: dnd2024:creatures/warrior_veteran
```

###### Row 32

^splat-d415bdef-row_32

###### Result

<!-- splat:yaml path=splat_rule/parameters/activate/illusion/rows/@row_32/result -->

```yaml
appearance:
  key: dnd2024:creatures/wyvern
```

###### Row 33

^splat-d415bdef-row_33

###### Result

<!-- splat:values path=splat_rule/parameters/activate/illusion/rows/@row_33/result -->

| Field | Value |
| --- | --- |
| ` appearance ` | ` card_drawer ` |

#### Illusion

<!-- splat:yaml path=splat_rule/parameters/illusion -->

```yaml
behavior: imitate_creature
can_harm: false
move:
  action: magic
  user_maximum_distance_ft: 120
  user_must_see: true
  destination_maximum_distance_from_card_ft: 30
physical_interaction:
  objects_pass_through: true
  reveals_false: true
inspect:
  action: study
  check:
    ability: intelligence
    skill: investigation
    dc: 15
  success: identify_illusion
ends_on:
- card_moved
- dispelled
dispel_examples:
- key: dnd2024:spells/dispel_magic
on_end:
  card_image: disappears
  card_reusable: false
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
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
