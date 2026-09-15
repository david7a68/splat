---
splat_schema: 1
splat_id: 5ca3b212-eeda-5ac9-8547-56357a6eb8a1
splat_kind: rule
splat_key: dnd2024:rules/poison_crawler_mucus_200_gp
splat_name: Crawler Mucus (200 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
---

# Crawler Mucus (200 GP)

## Source text

<!-- SRD page 197 -->

Contact Poison\
A creature subjected to Crawler Mucus must suc-\
ceed on a DC 13 Constitution saving throw or have\
the Poisoned condition for 1 minute. The creature\
also has the Paralyzed condition while Poisoned in\
this way. The creature repeats the save at the end\
of each of its turns, ending the effect on itself on a\
success.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_crawler_mucus_200_gp ` |
| ` implementation ` | ` dnd2024:rule_poison_crawler_mucus_200_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` price_gp_per_dose ` | ` 200 ` |
| ` poison_type ` | ` contact ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc: 13
failure:
  condition: poisoned
  duration_minutes: 1
```

#### While poisoned by this effect

<!-- splat:values path=splat_rule/parameters/while_poisoned_by_this_effect -->

| Field | Value |
| --- | --- |
| ` condition ` | ` paralyzed ` |

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` timing ` | ` end_of_each_target_turn ` |
| ` success ` | ` effect_ends ` |
