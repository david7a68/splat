---
splat_schema: 1
splat_id: 9d7ca33f-dea6-55d9-bc68-5e8e1725f3cb
splat_kind: rule
splat_key: dnd2024:rules/traps_poisoned_needle
splat_name: Poisoned Needle
splat_origin:
  key: dnd2024:reference/pages/200
splat_sources:
- title: SRD 5.2.1
  locator: pp. 200, 201
  url: https://www.dndbeyond.com/srd
---

# Poisoned Needle

## Source text

<!-- SRD page 200 -->

Nuisance Trap (Levels 1–4)\
Trigger: A creature opens the trap’s lock improperly\
or fails to disarm the trap\
Duration: Instantaneous\
A poisoned needle is hidden in a lock. When a crea-\
ture opens the lock with any object other than the\
proper key, the needle springs out and stabs the\
creature. The creature makes a DC 11 Constitution\
saving throw. On a failed save, the creature takes\
5 (1d10) Poison damage and has the Poisoned\

<!-- SRD page 201 -->

condition for 1 hour. On a successful save, the crea-\
ture takes half as much damage only.\
Avoid. The trap doesn’t trigger if the lock is\
opened using a Knock spell or similar magic.\
Detect and Disarm. As a Search action, a creature\
can examine the trapped lock and make a DC 15\
Wisdom (Perception) check, detecting the needle on\
a successful check. Once the trap is detected, a char-\
acter can take an action to try to disarm the trap,\
doing so with a successful DC 15 Dexterity (Sleight\
of Hand) check. On a failed check, the creature trig-\
gers the trap.\
At Higher Levels. You can scale the trap for higher\
levels by increasing the damage and the save DC, as\
shown in the following table.\
Levels Poison Damage Save DC\
5–10 11 (2d10) 13\
11–16 22 (4d10) 15\
17–20 55 (10d10) 17\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/traps_poisoned_needle ` |
| ` implementation ` | ` dnd2024:rules_traps_poisoned_needle ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` classification ` | ` nuisance ` |
| ` triggers ` | ` ["lock_opened_with_object_other_than_correct_key", "disarm_failure"] ` |
| ` duration ` | ` instantaneous ` |
| ` magical_opening_exemptions ` | ` ["knock", "similar_magic"] ` |

#### Base levels

<!-- splat:values path=splat_rule/parameters/base_levels -->

| Field | Value |
| --- | --- |
| ` min ` | ` 1 ` |
| ` max ` | ` 4 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 11 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
damage:
  dice:
    count: 1
    sides: 10
  average: 5
  type: poison
condition: poisoned
condition_duration:
  value: 1
  unit: hour
```

#### Success

<!-- splat:values path=splat_rule/parameters/success -->

| Field | Value |
| --- | --- |
| ` damage ` | ` half_round_down ` |
| ` condition ` | ` none ` |

#### Detect

<!-- splat:values path=splat_rule/parameters/detect -->

| Field | Value |
| --- | --- |
| ` action ` | ` search ` |
| ` ability ` | ` wisdom ` |
| ` skill ` | ` perception ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` needle_detected ` |

#### Disarm

<!-- splat:values path=splat_rule/parameters/disarm -->

| Field | Value |
| --- | --- |
| ` requires_detected ` | ` true ` |
| ` action_cost ` | ` 1 ` |
| ` ability ` | ` dexterity ` |
| ` skill ` | ` sleight_of_hand ` |
| ` dc ` | ` 15 ` |
| ` failure ` | ` trigger_trap ` |

#### Optional level scaling

<!-- splat:table
path: splat_rule/parameters/optional_level_scaling
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Levels min:
    path: levels/min
    type: integer
  Levels max:
    path: levels/max
    type: integer
  Damage dice count:
    path: damage/dice/count
    type: integer
  Damage dice sides:
    path: damage/dice/sides
    type: integer
  Damage average:
    path: damage/average
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Key | Levels min | Levels max | Damage dice count | Damage dice sides | Damage average | Save dc |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 5 ` | ` 10 ` | ` 2 ` | ` 10 ` | ` 11 ` | ` 13 ` |
| ` row_2 ` | ` 11 ` | ` 16 ` | ` 4 ` | ` 10 ` | ` 22 ` | ` 15 ` |
| ` row_3 ` | ` 17 ` | ` 20 ` | ` 10 ` | ` 10 ` | ` 55 ` | ` 17 ` |
