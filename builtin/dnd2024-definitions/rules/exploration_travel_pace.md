---
splat_schema: 1
splat_id: 6ca2382f-1678-568e-a35c-43212eb127df
splat_kind: rule
splat_key: dnd2024:rules/exploration_travel_pace
splat_name: Travel Pace
splat_origin:
  key: dnd2024:reference/pages/012
splat_sources:
- title: SRD 5.2.1
  locator: pp. 12
  url: https://www.dndbeyond.com/srd
---

# Travel Pace

## Source text

<!-- SRD page 12 -->

While traveling outside combat, a group can move at a Fast, Normal, or Slow pace, as shown on the Travel Pace table. The table states how far the party can move in a period of time; if riding horses or other mounts, the group can move twice that distance for 1 hour, after which the mounts need a Short or Long Rest before they can move at that increased pace again (see “Equipment” for a selection of mounts for sale). “Gameplay Toolbox” has rules that affect which pace you can choose in certain types of terrain.

**Travel Pace**

*Distance Traveled Per*

| Pace | Minute | Hour | Day |
| --- | --- | --- | --- |
| Fast | 400 feet | 4 miles | 30 miles |
| Normal | 300 feet | 3 miles | 24 miles |
| Slow | 200 feet | 2 miles | 18 miles |

Each travel pace has a game effect, as defined below.

Fast. Traveling at a Fast pace imposes Disadvantage on a traveler’s Wisdom (Perception or Survival) and Dexterity (Stealth) checks.

Normal. Traveling at a Normal pace imposes Disadvantage on Dexterity (Stealth) checks.

Slow. Traveling at a Slow pace grants Advantage on Wisdom (Perception or Survival) checks.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/exploration_travel_pace ` |
| ` implementation ` | ` dnd2024:rule_exploration_travel_pace ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` pace_limit_source ` | ` terrain ` |

#### Paces

<!-- splat:table
path: splat_rule/parameters/paces
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Pace:
    path: pace
    type: symbol
  Feet per minute:
    path: feet_per_minute
    type: integer
  Miles per hour:
    path: miles_per_hour
    type: integer
  Miles per day:
    path: miles_per_day
    type: integer
  Disadvantage:
    path: disadvantage
    type: yaml
    optional: true
  Advantage:
    path: advantage
    type: yaml
    optional: true
-->

| Key | Pace | Feet per minute | Miles per hour | Miles per day | Disadvantage | Advantage |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` Fast ` | ` 400 ` | ` 4 ` | ` 30 ` | ` ["wisdom_perception", "wisdom_survival", "dexterity_stealth"] ` |  |
| ` row_2 ` | ` Normal ` | ` 300 ` | ` 3 ` | ` 24 ` | ` ["dexterity_stealth"] ` |  |
| ` row_3 ` | ` Slow ` | ` 200 ` | ` 2 ` | ` 18 ` |  | ` ["wisdom_perception", "wisdom_survival"] ` |

#### Mounted

<!-- splat:values path=splat_rule/parameters/mounted -->

| Field | Value |
| --- | --- |
| ` distance_multiplier ` | ` 2 ` |
| ` maximum_hours ` | ` 1 ` |
| ` recharge ` | ` ["short_rest", "long_rest"] ` |
