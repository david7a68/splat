---
splat_schema: 1
splat_id: 51fd8389-77a9-585b-9710-04d4924f3723
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_components
splat_name: Components
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
---

# Components

## Source text

<!-- SRD page 105 -->

A spell’s components are physical requirements the\
spellcaster must meet to cast the spell. Each spell’s\
description indicates whether it requires Verbal\
(V), Somatic (S), or Material (M) components. If the\
spellcaster can’t provide one or more of a spell’s\
components, the spellcaster can’t cast the spell.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_components ` |
| ` implementation ` | ` dnd2024:rule_casting_spells_components ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` component_types ` | ` ["verbal", "somatic", "material"] ` |
| ` required_components ` | ` spell_definition ` |
| ` cast_allowed_only_if_all_required_components_available ` | ` true ` |
