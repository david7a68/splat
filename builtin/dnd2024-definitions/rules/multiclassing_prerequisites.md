---
splat_schema: 1
splat_id: 752ca5af-c9f9-50d5-827f-6c64627e1d8f
splat_kind: rule
splat_key: dnd2024:rules/multiclassing_prerequisites
splat_name: Prerequisites
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24, 25
  url: https://www.dndbeyond.com/srd
---

# Prerequisites

## Source text

<!-- SRD page 24 -->

To qualify for a new class, you must have a score of at least 13 in the primary ability of the new class and your current classes. For example, a Barbarian who decides to multiclass into the Druid class must have Strength and Wisdom scores of 13 or higher, since Strength is the primary ability for Barbarians and Wisdom is the primary ability for Druids.

**Starting Equipment at Higher Levels**

| Starting Level | Equipment and Money | Magic Items |
| --- | --- | --- |
| 2–4 | Normal starting equipment | 1 Common |
| 5–10 | 500 GP plus 1d10 × 25 GP plus normal starting equipment | 1 Common, 1 Uncommon |
| 11–16 | 5,000 gp plus 1d10 × 250 GP plus normal starting equipment | 2 Common, 3 Uncommon, 1 Rare |
| 17–20 | 20,000 GP plus 1d10 × 250 GP plus normal starting equipment | 2 Common, 4 Uncommon, 3 Rare, 1 Very Rare |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/multiclassing_prerequisites ` |
| ` implementation ` | ` dnd2024:rule_multiclassing_prerequisites ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` minimum_primary_ability_score ` | ` 13 ` |
| ` classes_to_check ` | ` new_and_all_current ` |

#### Primary abilities

<!-- splat:yaml path=splat_rule/parameters/primary_abilities -->

```yaml
source:
  key: dnd2024:rules/create_your_character_step_1_choose_class
```

#### Interleaved starting equipment table

<!-- splat:yaml path=splat_rule/parameters/interleaved_starting_equipment_table -->

```yaml
source:
  key: dnd2024:rules/starting_at_higher_levels_creating_your_character
field: starting_equipment_table
```
