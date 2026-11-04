---
splat_schema: 2
splat_id: 5a6d8845-22a6-51bb-9166-0feb2dca92e0
splat_kind: rule
splat_key: dnd2024:rules/poison_oil_of_taggit_400_gp
splat_name: Oil of Taggit (400 GP)
splat_origin: dnd2024:reference/pages/198
splat_sources:
- SRD 5.2.1, pp. 198, https://www.dndbeyond.com/srd
---

# Oil of Taggit (400 GP)

## Source text

<!-- SRD page 198 -->

Contact Poison

A creature subjected to Oil of Taggit must succeed on a DC 13 Constitution saving throw or have the Poisoned condition for 24 hours. The creature also has the Unconscious condition while Poisoned in this way. It wakes up if it takes damage.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_oil_of_taggit_400_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_oil_of_taggit_400_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` wake_triggers ` | ` ["damage_taken"] ` |
| ` wake_ends_poisoned ` | ` false ` |

#### Poison

<!-- splat:values path=splat_rule/parameters/poison -->

| Field | Value |
| --- | --- |
| ` type ` | ` contact ` |
| ` cost_gp ` | ` 400 ` |

#### Save

<!-- splat:values path=splat_rule/parameters/save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 13 ` |

#### Failure

<!-- splat:yaml path=splat_rule/parameters/failure -->

```yaml
condition:
  id: poisoned
  duration:
    value: 24
    unit: hours
unconscious_while_poisoned: true
```
