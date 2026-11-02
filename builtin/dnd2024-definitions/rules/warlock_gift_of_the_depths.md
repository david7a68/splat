---
splat_schema: 1
splat_id: 691823bb-24ef-5b0c-a455-5a9cb9598b9e
splat_kind: rule
splat_key: dnd2024:rules/warlock_gift_of_the_depths
splat_name: Gift of the Depths
splat_origin:
  key: dnd2024:reference/pages/073
splat_sources:
- title: SRD 5.2.1
  locator: pp. 73
  url: https://www.dndbeyond.com/srd
---

# Gift of the Depths

## Source text

<!-- SRD page 73 -->

Prerequisite: Level 5+ Warlock

You can breathe underwater, and you gain a Swim Speed equal to your Speed.

You can also cast Water Breathing once without expending a spell slot. You regain the ability to cast it in this way again when you finish a Long Rest.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/warlock_gift_of_the_depths ` |
| ` implementation ` | ` dnd2024:rules_warlock_gift_of_the_depths ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` breathe_underwater ` | ` true ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` warlock_level_min ` | ` 5 ` |

#### Swim speed

<!-- splat:values path=splat_rule/parameters/swim_speed -->

| Field | Value |
| --- | --- |
| ` var ` | ` subject.speed ` |

#### Grant cast

<!-- splat:yaml path=splat_rule/parameters/grant_cast -->

```yaml
spell:
  key: dnd2024:spells/water_breathing
spell_slot_cost: 0
uses: 1
reset: long_rest_finished
```
