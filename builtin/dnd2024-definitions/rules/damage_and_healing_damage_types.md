---
splat_schema: 1
splat_id: 7381fa3f-5a33-57a1-8315-b62b2f9033c6
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_damage_types
splat_name: Damage Types
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16, 17
  url: https://www.dndbeyond.com/srd
---

# Damage Types

## Source text

<!-- SRD page 16 -->

Each instance of damage has a type, like Fire or Slashing. Damage types are listed in “Rules Glossary” and have no rules of their own, but other rules, such as Resistance, rely on damage types.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_damage_types ` |
| ` implementation ` | ` dnd2024:rule_damage_and_healing_damage_types ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` each_damage_instance_has_type ` | ` true ` |
| ` inherent_type_rules ` | ` false ` |

#### Apply definitions

<!-- splat:rows path=splat_rule/parameters/apply_definitions -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/damage_types ` |
