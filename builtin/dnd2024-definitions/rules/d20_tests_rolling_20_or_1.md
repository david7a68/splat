---
splat_schema: 1
splat_id: d7b49410-af5d-54d4-8696-15e5b1c901ad
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_rolling_20_or_1
splat_name: Rolling 20 or 1
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
---

# Rolling 20 or 1

## Source text

<!-- SRD page 7 -->

If you roll a 20 on the d20 (called a “natural 20”) for\
an attack roll, the attack hits regardless of any mod-\
ifiers or the target’s AC. This is called a Critical Hit\
(see “Combat” later in “Playing the Game”).\
If you roll a 1 on the d20 (a “natural 1”) for an at-\
tack roll, the attack misses regardless of any modifi-\
ers or the target’s AC.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests_rolling_20_or_1 ` |
| ` implementation ` | ` dnd2024:rules_d20_tests_rolling_20_or_1 ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test ` | ` attack_roll ` |

#### Natural results

<!-- splat:yaml path=splat_rule/parameters/natural_results -->

```yaml
'20':
  hit: true
  critical_hit: true
  ignore_ac_and_modifiers: true
'1':
  hit: false
  ignore_ac_and_modifiers: true
```
