---
splat_schema: 1
splat_id: 98ff1bc5-c043-5f84-8d6e-b036199c09a6
splat_kind: item
splat_key: dnd2024:items/wand_of_wonder
splat_name: Wand of Wonder
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_wonder
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251, 252
  url: https://www.dndbeyond.com/srd
---

# Wand of Wonder

## Source text

<!-- SRD page 251 -->

Wand, Rare (Requires Attunement)\
This wand has 7 charges. While holding it, you can\
take a Magic action to expend 1 charge while choos-\
ing a point within 120 feet of yourself. That loca-\
tion becomes the point of origin of a spell or other\
magical effect determined by rolling on the Wand\
of Wonder Effects table. Spells cast from the wand\
have a save DC of 15. If a spell’s maximum range\
is normally less than 120 feet, it becomes 120 feet\
when cast from the wand. If an effect has multiple\
possible subjects, the GM determines randomly\
which among them are affected.\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into dust and is destroyed.\

<!-- SRD page 252 -->

Wand of Wonder Effects\
1d100 Effect\
01–20 You cast a spell originating from the chosen\
point. Roll 1d10 to determine the spell: on a\
1–2, Darkness; on a 3–4, Faerie Fire; on a 5–6,\
Fireball; on a 7–8, Slow; on a 9–10,Stinking\
Cloud.\
21–25 Nothing happens at the chosen point of origin.\
Instead, you have the Stunned condition until\
the start of your next turn, believing some-\
thing awesome just happened.\
26–30 You cast Gust of Wind. The Line created by the\
spell extends from you to the chosen point of\
origin.\
31–35 Nothing happens at the chosen point of origin.\
Instead, you take 1d6 Psychic damage.\
36–40 Heavy rain falls for 1 minute in a 120-foot-\
high, 60-foot-radius Cylinder centered on the\
chosen point of origin. During that time, the\
area of effect is Lightly Obscured.\
41–45 A cloud of 600 oversized butterflies fills a\
60-foot-high, 30-foot-radius Cylinder centered\
on the chosen point of origin. The butterflies\
remain for 10 minutes, during which time the\
area of effect is Heavily Obscured.\
46–50 You cast Lightning Bolt. The Line created by the\
spell extends from you to the chosen point of\
origin.\
51–55 The creature closest to the chosen point of\
origin is enlarged as if you had cast Enlarge/\
Reduce on it. If the target isn’t you and can’t be\
affected by that spell, you become the target\
instead.\
56–60 A magically formed creature appears in an\
unoccupied space as close to the chosen point\
of origin as possible. The creature isn’t under\
your control, acts as it normally would, and\
disappears after 1 hour or when it drops to 0\
Hit Points. Roll 1d4 to determine which crea-\
ture appears. On a 1, a Rhinoceros appears;\
on a 2, an Elephant appears; and on a 3–4, a\
Rat appears.\
61–64 Grass covers a 60-foot-radius circle of ground,\
with the center of that circle as close to the\
chosen point of origin as possible. Grass that’s\
already there grows to ten times its normal\
size and remains overgrown for 1 minute.\
1d100 Effect\
65–68 An object of the GM’s choice disappears into\
the Ethereal Plane. The object must be neither\
worn nor carried, within 120 feet of the cho-\
sen point of origin, and no larger than 10 feet\
in any dimension. If there are no such objects\
in range, nothing happens.\
69–72 Nothing happens at the chosen point of origin.\
Instead, you shrink as if you had cast Enlarge/\
Reduce on yourself and remain in that state for\
1 minute.\
73–77 Leaves grow from the creature nearest to the\
chosen point of origin. Unless they are picked\
off, the leaves turn brown and fall off after 24\
hours.\
78–82 Nothing happens at the chosen point of origin.\
Instead, a burst of colorful, shimmering light\
extends from you in a 30-foot Emanation.\
Each creature in the area must succeed on a\
DC 15 Constitution saving throw or have the\
Blinded condition for 1 minute. A creature re-\
peats the save at the end of each of its turns,\
ending the effect on itself on a success.\
83–87 Nothing happens at the chosen point of origin.\
Instead, you cast Invisibility on yourself.\
88–92 Nothing happens at the chosen point of ori-\
gin. Instead, a stream of 1d4 × 10 gems, each\
worth 1 GP, shoots from the wand’s tip in a\
Line 30 feet long and 5 feet wide toward the\
chosen point of origin. Each gem deals 1 Blud-\
geoning damage, and the total damage of the\
gems is divided equally among all creatures in\
the Line.\
93–97 You cast Polymorph, targeting the creature\
closest to the chosen point of origin. Roll 1d4\
to determine the target’s new form. On a 1,\
the new form is a Black Bear; on a 2, the\
new form is a Giant Wasp; on a 3–4, the new\
form is a Frog.\
98–00 The creature closest to the chosen point of\
origin makes a DC 15 Constitution saving\
throw. On a failed save, the creature has the\
Restrained condition and begins to turn to\
stone. While Restrained in this way, the crea-\
ture repeats the save at the end of its next\
turn. On a successful save, the effect ends.\
On a failed save, the creature has the Petrified\
condition instead of the Restrained condition.\
The petrification lasts until the creature is\
freed by the Greater Restoration spell or similar\
magic.\

Original layout: [[../../srd-5.2.1/pages/251|page 251]], [[../../srd-5.2.1/pages/252|page 252]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
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
| ` mechanic ` | ` dnd2024:items/wand_of_wonder ` |
| ` implementation ` | ` dnd2024:item_wand_of_wonder ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
| ` action ` | ` magic ` |
| ` charge_cost ` | ` 1 ` |
| ` effect_die ` | ` 100 ` |
| ` attunement_required ` | ` true ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 7
maximum: 7
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 6
      add: 1
  cap_at_maximum: true
last_charge:
  roll:
    roll:
      count: 1
      sides: 20
  'on': 1
  result: destroy_to_dust
```

#### Chosen origin

<!-- splat:values path=splat_rule/parameters/chosen_origin -->

| Field | Value |
| --- | --- |
| ` type ` | ` point ` |
| ` distance_from_user_ft_max ` | ` 120 ` |

#### Spell overrides

<!-- splat:values path=splat_rule/parameters/spell_overrides -->

| Field | Value |
| --- | --- |
| ` save_dc ` | ` 15 ` |
| ` range_ft_minimum ` | ` 120 ` |

#### Multiple possible subjects

<!-- splat:values path=splat_rule/parameters/multiple_possible_subjects -->

| Field | Value |
| --- | --- |
| ` selection ` | ` random ` |
| ` adjudicator ` | ` gm ` |

#### Effects

<!-- splat:rows path=splat_rule/parameters/effects -->

| Row | range |
| --- | --- |
| ` 0 ` | ` [1, 20] ` |
| ` 1 ` | ` [21, 25] ` |
| ` 2 ` | ` [26, 30] ` |
| ` 3 ` | ` [31, 35] ` |
| ` 4 ` | ` [36, 40] ` |
| ` 5 ` | ` [41, 45] ` |
| ` 6 ` | ` [46, 50] ` |
| ` 7 ` | ` [51, 55] ` |
| ` 8 ` | ` [56, 60] ` |
| ` 9 ` | ` [61, 64] ` |
| ` 10 ` | ` [65, 68] ` |
| ` 11 ` | ` [69, 72] ` |
| ` 12 ` | ` [73, 77] ` |
| ` 13 ` | ` [78, 82] ` |
| ` 14 ` | ` [83, 87] ` |
| ` 15 ` | ` [88, 92] ` |
| ` 16 ` | ` [93, 97] ` |
| ` 17 ` | ` [98, 100] ` |

##### Effect

<!-- splat:values path=splat_rule/parameters/effects/0/effect -->

| Field | Value |
| --- | --- |
| ` cast_at_origin ` | ` true ` |

###### Spell roll

<!-- splat:values path=splat_rule/parameters/effects/0/effect/spell_roll -->

| Field | Value |
| --- | --- |
| ` die ` | ` 10 ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/effects/0/effect/spell_roll/rows
key: Spell
order:
- darkness
- faerie_fire
- fireball
- slow
- stinking_cloud
columns:
  Spell:
    path: spell
    type: reference
  Range:
    path: range
    type: yaml
-->

| Spell | Range |
| --- | --- |
| [Darkness](<../spells/darkness.md> "dnd2024:spells/darkness") | ` [1, 2] ` |
| [Faerie Fire](<../spells/faerie_fire.md> "dnd2024:spells/faerie_fire") | ` [3, 4] ` |
| [Fireball](<../spells/fireball.md> "dnd2024:spells/fireball") | ` [5, 6] ` |
| [Slow](<../spells/slow.md> "dnd2024:spells/slow") | ` [7, 8] ` |
| [Stinking Cloud](<../spells/stinking_cloud.md> "dnd2024:spells/stinking_cloud") | ` [9, 10] ` |

##### Effect

<!-- splat:values path=splat_rule/parameters/effects/1/effect -->

| Field | Value |
| --- | --- |
| ` target ` | ` user ` |
| ` condition ` | ` stunned ` |
| ` ends ` | ` user.next_turn_start ` |
| ` chosen_origin_effect ` | ` false ` |

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/2/effect -->

```yaml
spell:
  key: dnd2024:spells/gust_of_wind
line:
  from: user
  to: chosen_origin
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/3/effect -->

```yaml
target: user
damage:
  amount:
    roll:
      count: 1
      sides: 6
  type: psychic
chosen_origin_effect: false
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/4/effect -->

```yaml
weather: heavy_rain
area:
  shape: cylinder
  origin: chosen_origin
  height_ft: 120
  radius_ft: 60
duration:
  amount: 1
  unit: minutes
obscuration: lightly_obscured
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/5/effect -->

```yaml
create: oversized_butterflies
count: 600
area:
  shape: cylinder
  origin: chosen_origin
  height_ft: 60
  radius_ft: 30
duration:
  amount: 10
  unit: minutes
obscuration: heavily_obscured
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/6/effect -->

```yaml
spell:
  key: dnd2024:spells/lightning_bolt
line:
  from: user
  to: chosen_origin
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/7/effect -->

```yaml
spell:
  key: dnd2024:spells/enlarge_reduce
mode: enlarge
target: closest_creature_to_chosen_origin
ineligible_other_target_fallback: user
```

##### Effect

<!-- splat:values path=splat_rule/parameters/effects/8/effect -->

| Field | Value |
| --- | --- |
| ` placement ` | ` nearest_unoccupied_space_to_chosen_origin ` |
| ` magically_formed ` | ` true ` |
| ` controlled_by_user ` | ` false ` |
| ` behavior ` | ` normal ` |
| ` ends_on ` | ` zero_hp ` |

###### Summon roll

<!-- splat:values path=splat_rule/parameters/effects/8/effect/summon_roll -->

| Field | Value |
| --- | --- |
| ` die ` | ` 4 ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/effects/8/effect/summon_roll/rows
key: Creature
order:
- rhinoceros
- elephant
- rat
columns:
  Creature:
    path: creature
    type: reference
  Range:
    path: range
    type: yaml
-->

| Creature | Range |
| --- | --- |
| [Rhinoceros](<../creatures/rhinoceros.md> "dnd2024:creatures/rhinoceros") | ` [1, 1] ` |
| [Elephant](<../creatures/elephant.md> "dnd2024:creatures/elephant") | ` [2, 2] ` |
| [Rat](<../creatures/rat.md> "dnd2024:creatures/rat") | ` [3, 4] ` |

###### Duration

<!-- splat:values path=splat_rule/parameters/effects/8/effect/duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 1 ` |
| ` unit ` | ` hours ` |

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/9/effect -->

```yaml
create: grass
area:
  shape: circle
  radius_ft: 60
  ground: true
  center: nearest_to_chosen_origin
existing_grass:
  size_multiplier: 10
  overgrowth_duration:
    amount: 1
    unit: minutes
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/10/effect -->

```yaml
target:
  kind: object
  choice: gm
  worn_or_carried: false
  distance_from_origin_ft_max: 120
  maximum_each_dimension_ft: 10
transport_plane: ethereal
no_eligible_target: no_effect
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/11/effect -->

```yaml
spell:
  key: dnd2024:spells/enlarge_reduce
mode: reduce
target: self
duration:
  amount: 1
  unit: minutes
chosen_origin_effect: false
```

##### Effect

<!-- splat:values path=splat_rule/parameters/effects/12/effect -->

| Field | Value |
| --- | --- |
| ` target ` | ` closest_creature_to_chosen_origin ` |
| ` grow ` | ` leaves ` |
| ` ends_on ` | ` ["leaves_picked_off", "24_hours_elapsed"] ` |

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/13/effect -->

```yaml
chosen_origin_effect: false
area:
  shape: emanation
  origin: user
  radius_ft: 30
save:
  ability: constitution
  dc: 15
  failure:
    condition: blinded
    duration:
      amount: 1
      unit: minutes
  repeat:
    event: target.turn_end
    success: end_effect
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/14/effect -->

```yaml
spell:
  key: dnd2024:spells/invisibility
target: self
chosen_origin_effect: false
```

##### Effect

<!-- splat:yaml path=splat_rule/parameters/effects/15/effect -->

```yaml
chosen_origin_effect: false
gem_count:
  multiply:
  - roll:
      count: 1
      sides: 4
  - 10
gem_value_gp: 1
area:
  shape: line
  origin: wand_tip
  toward: chosen_origin
  length_ft: 30
  width_ft: 5
damage:
  type: bludgeoning
  amount_per_gem: 1
  total: gem_count
  allocation: divide_equally_among_all_creatures_in_line
```

##### Effect

<!-- splat:values path=splat_rule/parameters/effects/16/effect -->

| Field | Value |
| --- | --- |
| ` target ` | ` closest_creature_to_chosen_origin ` |

###### Spell

<!-- splat:values path=splat_rule/parameters/effects/16/effect/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/polymorph ` |

###### Form roll

<!-- splat:values path=splat_rule/parameters/effects/16/effect/form_roll -->

| Field | Value |
| --- | --- |
| ` die ` | ` 4 ` |

###### Rows

<!-- splat:table
path: splat_rule/parameters/effects/16/effect/form_roll/rows
key: Creature
order:
- black_bear
- giant_wasp
- frog
columns:
  Creature:
    path: creature
    type: reference
  Range:
    path: range
    type: yaml
-->

| Creature | Range |
| --- | --- |
| [Black Bear](<../creatures/black_bear.md> "dnd2024:creatures/black_bear") | ` [1, 1] ` |
| [Giant Wasp](<../creatures/giant_wasp.md> "dnd2024:creatures/giant_wasp") | ` [2, 2] ` |
| [Frog](<../creatures/frog.md> "dnd2024:creatures/frog") | ` [3, 4] ` |

##### Effect

<!-- splat:values path=splat_rule/parameters/effects/17/effect -->

| Field | Value |
| --- | --- |
| ` target ` | ` closest_creature_to_chosen_origin ` |

###### Save

<!-- splat:values path=splat_rule/parameters/effects/17/effect/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 15 ` |

###### Failure

<!-- splat:values path=splat_rule/parameters/effects/17/effect/save/failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` restrained ` |
| ` repeat_at ` | ` target.next_turn_end ` |

###### Repeat save

<!-- splat:values path=splat_rule/parameters/effects/17/effect/save/failure/repeat_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 15 ` |

###### Requires

<!-- splat:values path=splat_rule/parameters/effects/17/effect/save/failure/repeat_save/requires -->

| Field | Value |
| --- | --- |
| ` condition ` | ` restrained ` |
| ` source ` | ` this_effect ` |
| ` active ` | ` true ` |

###### Failure

<!-- splat:values path=splat_rule/parameters/effects/17/effect/save/failure/repeat_save/failure -->

| Field | Value |
| --- | --- |
| ` replace_condition ` | ` petrified ` |

###### Ends on

<!-- splat:table
path: splat_rule/parameters/effects/17/effect/save/failure/repeat_save/failure/ends_on
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Spell:
    path: spell
    type: reference
    optional: true
  Magic equivalent:
    path: magic_equivalent
    type: symbol
    optional: true
  Adjudicator:
    path: adjudicator
    type: symbol
    optional: true
-->

| Key | Spell | Magic equivalent | Adjudicator |
| --- | --- | --- | --- |
| ` row_1 ` | [Greater Restoration](<../spells/greater_restoration.md> "dnd2024:spells/greater_restoration") |  |  |
| ` row_2 ` |  | ` Greater restoration ` | ` Gm ` |

###### Success

<!-- splat:values path=splat_rule/parameters/effects/17/effect/save/failure/repeat_save/success -->

| Field | Value |
| --- | --- |
| ` end_effect ` | ` true ` |

#### Gem damage rounding

<!-- splat:values path=splat_rule/parameters/gem_damage_rounding -->

| Field | Value |
| --- | --- |
| ` scope ` | ` gem_total_divided_by_creature_count ` |
| ` policy ` | ` system_round_down ` |
| ` reason ` | ` general_round_down_rule ` |
