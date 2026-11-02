---
splat_schema: 1
splat_id: ca2807a9-3c37-510a-9d4b-cf175549dbbd
splat_kind: spell
splat_key: dnd2024:spells/phantom_steed
splat_name: Phantom Steed
splat_origin:
  key: dnd2024:reference/spell/phantom_steed
splat_sources:
- title: SRD 5.2.1
  locator: pp. 152
  url: https://www.dndbeyond.com/srd
---

# Phantom Steed

## Source text

<!-- SRD page 152 -->

Level 3 Illusion (Wizard)\
Casting Time: 1 minute or Ritual\
Range: 30 feet\
Components: V, S\
Duration: 1 hour

A Large, quasi-real, horselike creature appears on the ground in an unoccupied space of your choice within range. You decide the creature’s appearance, and it is equipped with a saddle, bit, and bridle. Any of the equipment created by the spell vanishes in a puff of smoke if it is carried more than 10 feet away from the steed.

For the duration, you or a creature you choose can ride the steed. The steed uses the Riding Horse stat block (see “Monsters”), except it has a Speed of 100 feet and can travel 13 miles in an hour. When the spell ends, the steed gradually fades, giving the rider 1 minute to dismount. The spell ends early if the steed takes any damage.

Original layout: [[../../srd-5.2.1/pages/152|page 152]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` illusion ` |
| ` classes ` | ` ["wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` 1 hour ` |
| ` casting_time ` | ` 1 minute or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 30 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/phantom_steed ` |
| ` implementation ` | ` dnd2024:spell_phantom_steed ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Steed

<!-- splat:yaml path=splat_rule/parameters/steed -->

```yaml
size: large
creature_kind: quasi_real_horselike
space:
  ground: true
  unoccupied: true
  range_feet: 30
appearance: caster_choice
stat_block:
  key: dnd2024:creatures/riding_horse
speed_feet: 100
travel_miles_per_hour: 13
```

#### Equipment

<!-- splat:values path=splat_rule/parameters/equipment -->

| Field | Value |
| --- | --- |
| ` grants ` | ` ["saddle", "bit", "bridle"] ` |
| ` vanishes_if_distance_from_steed_greater_than_feet ` | ` 10 ` |

#### Rider

<!-- splat:values path=splat_rule/parameters/rider -->

| Field | Value |
| --- | --- |
| ` caster_or_chosen_creature ` | ` true ` |

#### End early

<!-- splat:values path=splat_rule/parameters/end_early -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.applied ` |
| ` subject ` | ` steed ` |
| ` minimum_damage ` | ` 1 ` |

#### Spell end

<!-- splat:values path=splat_rule/parameters/spell_end -->

| Field | Value |
| --- | --- |
| ` fade_seconds ` | ` 60 ` |
| ` rider_dismount_seconds ` | ` 60 ` |
