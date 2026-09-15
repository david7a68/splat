---
splat_schema: 1
splat_id: 38032089-6d47-58ee-827b-02e1b3bdbf00
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_18_elusive
splat_name: 'Level 18: Elusive'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
---

# Level 18: Elusive

## Source text

<!-- SRD page 63 -->

You’re so evasive that attackers rarely gain the\
upper hand against you. No attack roll can have\
Advantage against you unless you have the Incapac-\
itated condition.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/rogue_level_18_elusive ` |
| ` implementation ` | ` dnd2024:class_features_rogue_level_18_elusive ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attacks_against_actor_can_have_advantage ` | ` false ` |

#### Unless

<!-- splat:values path=splat_rule/parameters/unless -->

| Field | Value |
| --- | --- |
| ` actor_condition ` | ` incapacitated ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` rogue ` |
| ` minimum_level ` | ` 18 ` |
