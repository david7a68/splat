---
splat_schema: 1
splat_id: 05503f6b-dd1c-5f81-9dcd-1c48d8a93280
splat_kind: rule
splat_key: dnd2024:glossary/action
splat_name: Action
splat_origin:
  key: dnd2024:reference/glossary/action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 176
  url: https://www.dndbeyond.com/srd
---

# Action

## Source text

<!-- SRD page 176 -->

On your turn, you can take one action. Choose which action to take from those below or from the special actions provided by your features. See also “Playing the Game” (“Actions”). These actions are defined elsewhere in this glossary:

|  |  |  |  |
| --- | --- | --- | --- |
| Attack | Dodge | Influence | Search |
| Dash | Help | Magic | Study |
| Disengage | Hide | Ready | Utilize |

Original layout: [[../../srd-5.2.1/pages/176|page 176]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/action ` |
| ` implementation ` | ` dnd2024:glossary_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` additional_options ` | ` granted_feature_actions ` |

#### Budget

<!-- splat:values path=splat_rule/parameters/budget -->

| Field | Value |
| --- | --- |
| ` per_turn ` | ` 1 ` |
| ` timing ` | ` own_turn ` |

#### Options

<!-- splat:rows path=splat_rule/parameters/options -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:glossary/attack_action ` |
| ` 1 ` | ` dnd2024:glossary/dash_action ` |
| ` 2 ` | ` dnd2024:glossary/disengage_action ` |
| ` 3 ` | ` dnd2024:glossary/dodge_action ` |
| ` 4 ` | ` dnd2024:glossary/help_action ` |
| ` 5 ` | ` dnd2024:glossary/hide_action ` |
| ` 6 ` | ` dnd2024:glossary/influence_action ` |
| ` 7 ` | ` dnd2024:glossary/magic_action ` |
| ` 8 ` | ` dnd2024:glossary/ready_action ` |
| ` 9 ` | ` dnd2024:glossary/search_action ` |
| ` 10 ` | ` dnd2024:glossary/study_action ` |
| ` 11 ` | ` dnd2024:glossary/utilize_action ` |
