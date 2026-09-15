---
splat_schema: 1
splat_id: 110dcec7-06c8-5220-b28d-12b591119bd0
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_limited_usage
splat_name: Limited Usage
splat_origin:
  key: dnd2024:reference/pages/257
splat_sources:
- title: SRD 5.2.1
  locator: pp. 257
  url: https://www.dndbeyond.com/srd
---

# Limited Usage

## Source text

<!-- SRD page 257 -->

Some parts of a stat block have restrictions on the\
number of times they can be used. Here are the\
most common ways that usage is limited:\
X/Day. This notation means the stat block part can\
be used a certain number of times (represented\
by X) and that a monster must finish a Long Rest\
to regain expended uses. For example, a Reaction\
that includes “1/Day” means the Reaction can be\
taken once and that the monster must finish a\
Long Rest to take it again.\
Recharge X–Y. This notation means a monster\
can use the stat block part once. At the start of\
each of the monster’s turns, roll 1d6. If the roll\
is within the number range given in the notation\
(represented by X–Y), the monster regains the use\
of that part, which also recharges when the mon-\
ster finishes a Short or Long Rest. For example,\
“Recharge 5–6” in an action means a monster can\
take the action once. Then, at the start of each of\
the monster’s turns, it regains the use of that ac-\
tion if it rolls a 5 or 6 on 1d6.\
Recharge after a Short or Long Rest. This nota-\
tion means the monster can use the stat block\
part once and must then finish a Short or Long\
Rest to use it again.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_limited_usage ` |
| ` implementation ` | ` dnd2024:rules_parts_of_a_stat_block_limited_usage ` |

### Parameters

#### Usage modes

<!-- splat:yaml path=splat_rule/parameters/usage_modes -->

```yaml
x_per_day:
  maximum_uses: x
  recovery_event: long_rest.finished
  recovery: all
recharge_x_y:
  maximum_uses: 1
  roll_event: actor_turn.started
  roll:
    count: 1
    sides: 6
    add: 0
  recharge_when:
    minimum: x
    maximum: y
  rest_recovery:
  - short_rest.finished
  - long_rest.finished
recharge_after_rest:
  maximum_uses: 1
  recovery_events:
  - short_rest.finished
  - long_rest.finished
```
