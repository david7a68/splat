---
splat_schema: 1
splat_id: 83fba512-5429-5d13-a438-117acd034f20
splat_kind: rule
splat_key: dnd2024:rules/poison_harvesting_poison
splat_name: Harvesting Poison
splat_origin:
  key: dnd2024:reference/pages/197
splat_sources:
- title: SRD 5.2.1
  locator: pp. 197
  url: https://www.dndbeyond.com/srd
---

# Harvesting Poison

## Source text

<!-- SRD page 197 -->

A character can attempt to harvest poison from a\
venomous creature that is dead or has the Incapaci-\
tated condition. The effort takes 1d6 minutes, after\
which the character makes a DC 20 Intelligence\
(Nature) check using a Poisoner’s Kit. On a success-\
ful check, the character harvests enough poison\
for a single dose, and no additional poison can be\
harvested from that creature. On a failed check,\
the character is unable to extract any poison. If the\
character fails the check by 5 or more, the character\
is subjected to the creature’s poison.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/poison_harvesting_poison ` |
| ` implementation ` | ` dnd2024:rules_poison_harvesting_poison ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` required_tool ` | ` poisoners_kit ` |

#### Target requirements

<!-- splat:values path=splat_rule/parameters/target_requirements -->

| Field | Value |
| --- | --- |
| ` venomous ` | ` true ` |
| ` any_state ` | ` ["dead", "incapacitated"] ` |
| ` already_harvested ` | ` false ` |

#### Time

<!-- splat:yaml path=splat_rule/parameters/time -->

```yaml
dice:
  count: 1
  sides: 6
unit: minute
```

#### Check

<!-- splat:values path=splat_rule/parameters/check -->

| Field | Value |
| --- | --- |
| ` ability ` | ` intelligence ` |
| ` skill ` | ` nature ` |
| ` dc ` | ` 20 ` |

#### Success

<!-- splat:values path=splat_rule/parameters/success -->

| Field | Value |
| --- | --- |
| ` poison_doses ` | ` 1 ` |
| ` target_further_harvest_prohibited ` | ` true ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` poison_doses ` | ` 0 ` |

#### Failure margin at least 5

<!-- splat:values path=splat_rule/parameters/failure_margin_at_least_5 -->

| Field | Value |
| --- | --- |
| ` subjected_to ` | ` target_poison ` |
