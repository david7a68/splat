---
splat_schema: 1
splat_id: 4018583f-4bb2-56f8-89f2-41176e0b6184
splat_kind: rule
splat_key: dnd2024:glossary/proficiency
splat_name: Proficiency
splat_origin:
  key: dnd2024:reference/glossary/proficiency
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Proficiency

## Source text

<!-- SRD page 186 -->

If you have proficiency with something, you can add\
your Proficiency Bonus to any D20 Test you make\
using that thing. A creature might have proficiency\
in a skill or saving throw or with a weapon or tool.\
See also “Playing the Game” (“Proficiency”).\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/proficiency ` |
| ` implementation ` | ` dnd2024:glossary_proficiency ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` categories ` | ` ["skill", "saving_throw", "weapon", "tool"] ` |

#### D20 test bonus

<!-- splat:yaml path=splat_rule/parameters/d20_test_bonus -->

```yaml
requires: test_uses_proficiency
add:
  var: proficiency_bonus
```
