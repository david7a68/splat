---
splat_schema: 1
splat_id: 87ec9e0b-ffb6-579c-bf84-8139ff091e19
splat_kind: rule
splat_key: dnd2024:glossary/long_rest
splat_name: Long Rest
splat_origin:
  key: dnd2024:reference/glossary/long_rest
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
---

# Long Rest

## Source text

<!-- SRD page 185 -->

A Long Rest is a period of extended downtime—at\
least 8 hours—available to any creature. During a\
Long Rest, you sleep for at least 6 hours and per-\
form no more than 2 hours of light activity, such as\
reading, talking, eating, or standing watch.\
During sleep, you have the Unconscious condition.\
After you finish a Long Rest, you must wait at least\
16 hours before starting another one.\
Benefits of the Rest. To start a Long Rest, you\
must have at least 1 Hit Point. When you finish the\
rest, you gain the following benefits:\
Regain All HP. You regain all lost Hit Points and all\
spent Hit Point Dice. If your Hit Point maximum\
was reduced, it returns to normal.\
Ability Scores Restored. If any of your ability\
scores were reduced, they return to normal.\
Exhaustion Reduced. If you have the Exhaustion\
condition, its level decreases by 1.\
Special Feature. Some features are recharged by a\
Long Rest. If you have such a feature, it recharges\
in the way specified in its description.\
Interrupting the Rest. A Long Rest is stopped by\
the following interruptions:\
•  Rolling Initiative\
•  Casting a spell other than a cantrip\
•  Taking any damage\
•  1 hour of walking or other physical exertion\
If you rested at least 1 hour before the interruption,\
you gain the benefits of a Short Rest. See also “Short\
Rest.”\
You can resume a Long Rest immediately after an\
interruption. If you do so, the rest requires 1 addi-\
tional hour per interruption to finish.\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/long_rest ` |
| ` implementation ` | ` dnd2024:glossary_long_rest ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` minimum_hours ` | ` 8 ` |
| ` minimum_sleep_hours ` | ` 6 ` |
| ` maximum_light_activity_hours ` | ` 2 ` |
| ` light_activities ` | ` ["read", "talk", "eat", "stand_watch"] ` |
| ` sleep_condition ` | ` unconscious ` |
| ` minimum_hours_after_previous_long_rest_finish_before_start ` | ` 16 ` |
| ` start_minimum_hit_points ` | ` 1 ` |

#### Completion

<!-- splat:values path=splat_rule/parameters/completion -->

| Field | Value |
| --- | --- |
| ` regain_lost_hit_points ` | ` all ` |
| ` regain_spent_hit_dice ` | ` all ` |
| ` restore_reduced_hit_point_maximum ` | ` true ` |
| ` restore_reduced_ability_scores ` | ` true ` |
| ` exhaustion_levels_removed ` | ` 1 ` |
| ` feature_recharge ` | ` each_feature_description ` |

#### Interruptions

<!-- splat:table
path: splat_rule/parameters/interruptions
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Event:
    path: event
    type: string
    optional: true
  Spell level min:
    path: spell_level_min
    type: integer
    optional: true
  Activity any:
    path: activity_any
    type: yaml
    optional: true
  Hours:
    path: hours
    type: integer
    optional: true
-->

| Key | Event | Spell level min | Activity any | Hours |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` initiative.rolled ` |  |  |  |
| ` row_2 ` | ` spell.cast ` | ` 1 ` |  |  |
| ` row_3 ` | ` damage.taken ` |  |  |  |
| ` row_4 ` |  |  | ` ["walking", "physical_exertion"] ` | ` 1 ` |

#### Interruption result

<!-- splat:values path=splat_rule/parameters/interruption_result -->

| Field | Value |
| --- | --- |
| ` rest_stops ` | ` true ` |
| ` short_rest_benefits_if_prior_rest_hours_at_least ` | ` 1 ` |

#### Resume

<!-- splat:values path=splat_rule/parameters/resume -->

| Field | Value |
| --- | --- |
| ` immediate_allowed ` | ` true ` |
| ` additional_hours_per_interruption ` | ` 1 ` |
