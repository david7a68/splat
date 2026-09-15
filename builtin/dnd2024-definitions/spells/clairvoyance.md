---
splat_schema: 1
splat_id: c26b332e-3a61-5f59-9596-b170d4d63562
splat_kind: spell
splat_key: dnd2024:spells/clairvoyance
splat_name: Clairvoyance
splat_origin:
  key: dnd2024:reference/spell/clairvoyance
splat_sources:
- title: SRD 5.2.1
  locator: pp. 115
  url: https://www.dndbeyond.com/srd
---

# Clairvoyance

## Source text

<!-- SRD page 115 -->

Level 3 Divination (Bard, Cleric, Sorcerer, Wizard)\
Casting Time: 10 minutes\
Range: 1 mile\
Components: V, S, M (a focus worth 100+ GP, either a\
jeweled horn for hearing or a glass eye for seeing)\
Duration: Concentration, up to 10 minutes\
You create an Invisible sensor within range in a\
location familiar to you (a place you have visited\
or seen before) or in an obvious location that is\
unfamiliar to you (such as behind a door, around a\
corner, or in a grove of trees). The intangible, invul-\
nerable sensor remains in place for the duration.\
When you cast the spell, choose seeing or hearing.\
You can use the chosen sense through the sensor as\
if you were in its space. As a Bonus Action, you can\
switch between seeing and hearing.\
A creature that sees the sensor (such as a creature\
benefiting from See Invisibility or Truesight) sees a\
luminous orb about the size of your fist.\

Original layout: [[../../srd-5.2.1/pages/115|page 115]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` 1 mile ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a focus worth 100+ GP, either a jeweled horn for hearing or a glass eye for seeing ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/clairvoyance ` |
| ` implementation ` | ` dnd2024:spell_clairvoyance ` |

### Parameters

#### Sensor

<!-- splat:values path=splat_rule/parameters/sensor -->

| Field | Value |
| --- | --- |
| ` location_choice ` | ` ["visited_or_seen", "unfamiliar_obvious_location"] ` |
| ` invisible ` | ` true ` |
| ` intangible ` | ` true ` |
| ` invulnerable ` | ` true ` |
| ` stationary ` | ` true ` |
| ` visible_appearance ` | ` fist_sized_luminous_orb ` |

#### Sense

<!-- splat:values path=splat_rule/parameters/sense -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["seeing", "hearing"] ` |
| ` perception_origin ` | ` sensor ` |
| ` switch_action ` | ` bonus_action ` |

#### Focus

<!-- splat:values path=splat_rule/parameters/focus -->

| Field | Value |
| --- | --- |
| ` seeing ` | ` glass_eye ` |
| ` hearing ` | ` jeweled_horn ` |
| ` minimum_gp ` | ` 100 ` |
