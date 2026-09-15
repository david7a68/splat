---
splat_schema: 1
splat_id: 2c5a7c14-4c74-5ae4-ac3d-b8e915dced6f
splat_kind: rule
splat_key: dnd2024:rules/combat_dropping_prone
splat_name: Dropping Prone
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14
  url: https://www.dndbeyond.com/srd
---

# Dropping Prone

## Source text

<!-- SRD page 14 -->

On your turn, you can give yourself the Prone condi-\
tion (see “Rules Glossary”) without using an action\
or any of your Speed, but you can’t do so if your\
Speed is 0.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_dropping_prone ` |
| ` implementation ` | ` dnd2024:rules_combat_dropping_prone ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` timing ` | ` own_turn ` |
| ` requires_speed_greater_than ` | ` 0 ` |
| ` condition ` | ` prone ` |
| ` action_cost ` | ` 0 ` |
| ` movement_cost ` | ` 0 ` |
| ` optional ` | ` true ` |
