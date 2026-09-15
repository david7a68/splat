---
splat_schema: 1
splat_id: 10682190-5b2d-5668-bc9a-5e9efe7ba78d
splat_kind: rule
splat_key: dnd2024:rules/poison_burnt_othur_fumes_500_gp
splat_name: Burnt Othur Fumes (500 GP)
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
---

# Burnt Othur Fumes (500 GP)

## Source text

<!-- SRD page 197 -->

Inhaled Poison\
A creature subjected to Burnt Othur Fumes must\
succeed on a DC 13 Constitution saving throw or\
take 10 (3d6) Poison damage, and it must repeat the\
save at the start of each of its turns. On each succes-\
sive failed save, the creature takes 3 (1d6) Poison\
damage. After three successful saves, the poison\
ends.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_burnt_othur_fumes_500_gp ` |
| ` implementation ` | ` dnd2024:rules_poison_burnt_othur_fumes_500_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost_gp ` | ` 500 ` |
| ` exposure_route ` | ` inhaled ` |

#### Initial save

<!-- splat:values path=splat_rule/parameters/initial_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 13 ` |

#### Initial failure

<!-- splat:yaml path=splat_rule/parameters/initial_failure -->

```yaml
damage:
  dice:
    count: 3
    sides: 6
  average: 10
  type: poison
```

#### Repeat

<!-- splat:yaml path=splat_rule/parameters/repeat -->

```yaml
timing: subject_turn_start
ability: constitution
dc: 13
failure_damage:
  dice:
    count: 1
    sides: 6
  average: 3
  type: poison
requires_initial_failure: false
```

#### Ends

<!-- splat:values path=splat_rule/parameters/ends -->

| Field | Value |
| --- | --- |
| ` successful_saves ` | ` 3 ` |
| ` consecutive_required ` | ` false ` |
| ` count_initial_save_success ` | ` true ` |
