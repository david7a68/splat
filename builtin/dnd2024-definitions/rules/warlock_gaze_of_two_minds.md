---
splat_schema: 1
splat_id: 681c6d78-669a-52f2-9a64-72e6f5ce979c
splat_kind: rule
splat_key: dnd2024:rules/warlock_gaze_of_two_minds
splat_name: Gaze of Two Minds
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Gaze of Two Minds

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock\
You can use a Bonus Action to touch a willing crea-\
ture and perceive through its senses until the end\
of your next turn. As long as the creature is on the\
same plane of existence as you, you can take a Bo-\
nus Action on subsequent turns to maintain this\
connection, extending the duration until the end\
of your next turn. The connection ends if you don’t\
maintain it in this way.\
While perceiving through the other creature’s\
senses, you benefit from any special senses pos-\
sessed by that creature, and you can cast spells as if\
you were in your space or the other creature’s space\
if the two of you are within 60 feet of each other.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_gaze_of_two_minds ` |
| ` implementation ` | ` dnd2024:rules_warlock_gaze_of_two_minds ` |

### Parameters

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_minimum ` | ` 5 ` |

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` range ` | ` touch ` |
| ` target_willing ` | ` true ` |

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` until ` | ` actor_next_turn_end ` |

#### Maintenance

<!-- splat:values path=splat_rule/parameters/maintenance -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` "on" ` | ` subsequent_turn ` |
| ` same_plane_required ` | ` true ` |
| ` extend_until ` | ` actor_next_turn_end ` |
| ` failure_to_maintain ` | ` end_connection ` |

#### Perception

<!-- splat:values path=splat_rule/parameters/perception -->

| Field | Value |
| --- | --- |
| ` use_target_senses ` | ` true ` |
| ` benefit_special_senses ` | ` true ` |

#### Spell origin choice

<!-- splat:values path=splat_rule/parameters/spell_origin_choice -->

| Field | Value |
| --- | --- |
| ` options ` | ` ["actor_space", "target_space"] ` |
| ` maximum_separation_feet ` | ` 60 ` |
