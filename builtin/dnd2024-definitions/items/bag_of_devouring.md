---
splat_schema: 1
splat_id: 9f360395-d296-5962-a00e-3baef5a65710
splat_kind: item
splat_key: dnd2024:items/bag_of_devouring
splat_name: Bag of Devouring
splat_origin:
  key: dnd2024:reference/magic_item/bag_of_devouring
splat_sources:
- title: SRD 5.2.1
  locator: pp. 212
  url: https://www.dndbeyond.com/srd
---

# Bag of Devouring

## Source text

<!-- SRD page 212 -->

Wondrous Item, Very Rare\
This bag resembles a Bag of Holding but is a feeding\
orifice for a gigantic extradimensional creature.\
Turning the bag inside out closes the orifice.\
The extradimensional creature attached to the\
bag can sense whatever is placed inside the bag.\
Animal or vegetable matter placed wholly in the\
bag is devoured and lost forever. When part of a liv-\
ing creature is placed in the bag, as happens when\
someone reaches inside it, there is a 50 percent\
chance that the creature is pulled inside the bag. A\
creature inside the bag can take an action to try to\
escape, doing so with a successful DC 15 Strength\
(Athletics) check. Another creature can take an ac-\
tion to reach into the bag to pull a creature out, do-\
ing so with a successful DC 20 Strength (Athletics)\
check, provided the puller isn’t pulled inside the bag\
first. Any creature that starts its turn inside the bag\
is devoured, its body destroyed.\
Inanimate objects can be stored in the bag, which\
can hold a cubic foot of such material. However,\
once each day, the bag swallows any objects inside\
it and spits them out into another plane of existence.\
The GM determines the time and plane.\
If the bag is pierced or torn, it is destroyed, and\
anything contained within it is transported to a\
random location on the Astral Plane.\

Original layout: [[../../srd-5.2.1/pages/212|page 212]].

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
| ` mechanic ` | ` dnd2024:items/bag_of_devouring ` |
| ` implementation ` | ` dnd2024:item_bag_of_devouring ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` false ` |

#### Appearance

<!-- splat:values path=splat_rule/parameters/appearance -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:items/bag_of_holding ` |

#### Orifice

<!-- splat:values path=splat_rule/parameters/orifice -->

| Field | Value |
| --- | --- |
| ` inside_out ` | ` closed ` |
| ` senses_contents ` | ` true ` |

#### Organic contents

<!-- splat:values path=splat_rule/parameters/organic_contents -->

| Field | Value |
| --- | --- |
| ` types ` | ` ["animal_matter", "vegetable_matter"] ` |
| ` wholly_inserted ` | ` devoured_permanently ` |

#### Partial living insertion

<!-- splat:values path=splat_rule/parameters/partial_living_insertion -->

| Field | Value |
| --- | --- |
| ` pull_inside_probability ` | ` [1, 2] ` |

#### Escape

<!-- splat:yaml path=splat_rule/parameters/escape -->

```yaml
action: action
check:
  ability: strength
  skill: athletics
  dc: 15
```

#### Rescue

<!-- splat:yaml path=splat_rule/parameters/rescue -->

```yaml
action: action
reach_inside_triggers_pull: true
check_only_if_rescuer_not_pulled: true
check:
  ability: strength
  skill: athletics
  dc: 20
```

#### Turn start inside

<!-- splat:values path=splat_rule/parameters/turn_start_inside -->

| Field | Value |
| --- | --- |
| ` devoured ` | ` true ` |
| ` body_destroyed ` | ` true ` |

#### Storage

<!-- splat:yaml path=splat_rule/parameters/storage -->

```yaml
inanimate_volume_cubic_ft: 1
daily_ejection:
  frequency_per_day: 1
  time_choice: gm
  plane_choice: gm
  contents: all_inanimate_objects
```

#### On damage kind

<!-- splat:yaml path=splat_rule/parameters/on_damage_kind -->

```yaml
pierced_or_torn:
  destroy_item: true
  transport_contents:
    plane: astral
    location: random
```
