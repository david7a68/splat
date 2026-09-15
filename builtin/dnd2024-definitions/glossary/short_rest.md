---
splat_schema: 1
splat_id: 53fba91c-b91a-55db-a4d9-951e4bab37bd
splat_kind: rule
splat_key: dnd2024:glossary/short_rest
splat_name: Short Rest
splat_origin:
  key: dnd2024:reference/glossary/short_rest
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
---

# Short Rest

## Source text

<!-- SRD page 187 -->

A Short Rest is a 1-hour period of downtime, during\
which a creature does nothing more strenuous than\
reading, talking, eating, or standing watch. To start\
a Short Rest, you must have at least 1 Hit Point.\
Benefits of the Rest. When you finish the rest, you\
gain the following benefits:\
Spend Hit Point Dice. You can spend one or more\
of your Hit Point Dice to regain Hit Points. For\
each Hit Point Die you spend in this way, roll the\
die and add your Constitution modifier to it. You\
regain Hit Points equal to the total (minimum of 1\
Hit Point). You can decide to spend an additional\
Hit Point Die after each roll.\
Special Feature. Some features are recharged by a\
Short Rest. If you have such a feature, it recharges\
in the way specified in its description.\
Interrupting the Rest. A Short Rest is stopped by\
the following interruptions:\
•  Rolling Initiative\
•  Casting a spell other than a cantrip\
•  Taking any damage\
An interrupted Short Rest confers no benefits.\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/short_rest ` |
| ` implementation ` | ` dnd2024:glossary_short_rest ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` duration_hours ` | ` 1 ` |
| ` maximum_exertion ` | ` light_activity ` |
| ` allowed_examples ` | ` ["read", "talk", "eat", "stand_watch"] ` |
| ` start_minimum_hit_points ` | ` 1 ` |

#### Completion

<!-- splat:yaml path=splat_rule/parameters/completion -->

```yaml
hit_dice:
  optional: true
  spend_available_only: true
  one_at_a_time: true
  healing_per_die:
    op: max
    args:
    - 1
    - op: add
      args:
      - var: hit_die_roll
      - var: constitution_modifier
  decide_next_after_each_roll: true
feature_recharge: feature_description_defined
```

#### Interruptions

<!-- splat:rows path=splat_rule/parameters/interruptions -->

| Row | event | spell_level_min |
| --- | --- | --- |
| ` 0 ` | ` initiative.rolled ` |  |
| ` 1 ` | ` spell.cast ` | ` 1 ` |
| ` 2 ` | ` damage.taken ` |  |

#### Interruption result

<!-- splat:values path=splat_rule/parameters/interruption_result -->

| Field | Value |
| --- | --- |
| ` rest_stops ` | ` true ` |
| ` benefits ` | ` false ` |
