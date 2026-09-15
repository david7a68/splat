---
splat_schema: 1
splat_id: 5f2dfc80-4a5b-5ea3-b79d-0984f19dffc8
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_they_don_t_stack
splat_name: They Don’t Stack
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
---

# They Don’t Stack

## Source text

<!-- SRD page 18 -->

Temporary Hit Points can’t be added together. If\
you have Temporary Hit Points and receive more\
of them, you decide whether to keep the ones you\
have or to gain the new ones. For example, if a spell\
grants you 12 Temporary Hit Points when you al-\
ready have 10, you can have 12 or 10, not 22.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_they_don_t_stack ` |
| ` implementation ` | ` dnd2024:rule_damage_and_healing_they_don_t_stack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` temporary_hit_points_stacking ` | ` false ` |
| ` new_grant_choice ` | ` ["retain_existing", "replace_with_new"] ` |
