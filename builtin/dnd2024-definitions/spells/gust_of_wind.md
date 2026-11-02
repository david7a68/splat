---
splat_schema: 1
splat_id: 62806d64-6e3b-5709-9112-2ec2ac050032
splat_kind: spell
splat_key: dnd2024:spells/gust_of_wind
splat_name: Gust of Wind
splat_origin:
  key: dnd2024:reference/spell/gust_of_wind
splat_sources:
- title: SRD 5.2.1
  locator: pp. 138
  url: https://www.dndbeyond.com/srd
---

# Gust of Wind

## Source text

<!-- SRD page 138 -->

Level 2 Evocation (Druid, Ranger, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a legume seed)\
Duration: Concentration, up to 1 minute

A Line of strong wind 60 feet long and 10 feet wide blasts from you in a direction you choose for the duration. Each creature in the Line must succeed on a Strength saving throw or be pushed 15 feet away from you in a direction following the Line. A creature that ends its turn in the Line must make the same save.

Any creature in the Line must spend 2 feet of movement for every 1 foot it moves when moving closer to you.

The gust disperses gas or vapor, and it extinguishes candles and similar unprotected flames in the area. It causes protected flames, such as those of lanterns, to dance wildly and has a 50 percent chance to extinguish them.

As a Bonus Action on your later turns, you can change the direction in which the Line blasts from you.

Original layout: [[../../srd-5.2.1/pages/138|page 138]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["druid", "ranger", "sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a legume seed ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/gust_of_wind ` |
| ` implementation ` | ` dnd2024:spell_gust_of_wind ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` save_triggers ` | ` ["initial_creature_in_line", "creature_turn_ends_in_line"] ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` line ` |
| ` length_feet ` | ` 60 ` |
| ` width_feet ` | ` 10 ` |
| ` origin ` | ` caster ` |
| ` direction_choice ` | ` caster ` |
| ` duration ` | ` spell_duration ` |
| ` wind_strength ` | ` strong ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: strength
dc:
  binding: save_dc
failure:
  push_feet: 15
  direction: away_from_caster_along_line
success:
  effect: none
```

#### Movement

<!-- splat:yaml path=splat_rule/parameters/movement -->

```yaml
moving_closer_to_caster_in_line:
  cost_feet_per_foot: 2
```

#### Environment

<!-- splat:yaml path=splat_rule/parameters/environment -->

```yaml
disperse:
- gas
- vapor
unprotected_flames: extinguish
protected_flames:
  flicker_wildly: true
  extinguish_chance_percent: 50
```

#### Redirect

<!-- splat:values path=splat_rule/parameters/redirect -->

| Field | Value |
| --- | --- |
| ` cost ` | ` bonus_action ` |
| ` timing ` | ` caster_later_turns ` |
