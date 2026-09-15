---
splat_schema: 1
splat_id: a80aaf92-6a84-5d6f-9321-47b7aed615a0
splat_kind: rule
splat_key: dnd2024:glossary/exhaustion_condition
splat_name: Exhaustion [Condition]
splat_origin:
  key: dnd2024:reference/glossary/exhaustion_condition
splat_sources:
- title: SRD 5.2.1
  locator: pp. 181
  url: https://www.dndbeyond.com/srd
---

# Exhaustion [Condition]

## Source text

<!-- SRD page 181 -->

While you have the Exhaustion condition, you\
experience the following effects.\
Exhaustion Levels. This condition is cumulative.\
Each time you receive it, you gain 1 Exhaustion\
level. You die if your Exhaustion level is 6.\
D20 Tests Affected. When you make a D20 Test,\
the roll is reduced by 2 times your Exhaustion level.\
Speed Reduced. Your Speed is reduced by a num-\
ber of feet equal to 5 times your Exhaustion level.\
Removing Exhaustion Levels. Finishing a Long\
Rest removes 1 of your Exhaustion levels. When\
your Exhaustion level reaches 0, the condition ends.\

Original layout: [[../../srd-5.2.1/pages/181|page 181]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/exhaustion_condition ` |
| ` implementation ` | ` dnd2024:glossary_exhaustion_condition ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` condition ` | ` exhaustion ` |
| ` stacking ` | ` true ` |
| ` levels_per_application ` | ` 1 ` |
| ` death_at_level ` | ` 6 ` |
| ` long_rest_levels_removed ` | ` 1 ` |
| ` ends_at_level ` | ` 0 ` |

#### D20 modifier

<!-- splat:yaml path=splat_rule/parameters/d20_modifier -->

```yaml
op: multiply
args:
- -2
- var: exhaustion_level
```

#### Speed delta feet

<!-- splat:yaml path=splat_rule/parameters/speed_delta_feet -->

```yaml
op: multiply
args:
- -5
- var: exhaustion_level
```
