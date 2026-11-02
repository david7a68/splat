---
splat_schema: 1
splat_id: b6d6c420-1fb1-536c-a676-667071e4697f
splat_kind: rule
splat_key: dnd2024:glossary/dash_action
splat_name: Dash [Action]
splat_origin:
  key: dnd2024:reference/glossary/dash_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 180
  url: https://www.dndbeyond.com/srd
---

# Dash [Action]

## Source text

<!-- SRD page 180 -->

When you take the Dash action, you gain extra movement for the current turn. The increase equals your Speed after applying any modifiers. With a Speed of 30 feet, for example, you can move up to 60 feet on your turn if you Dash. If your Speed of 30 feet is reduced to 15 feet, you can move up to 30 feet this turn if you Dash.

If you have a special speed, such as a Fly Speed or Swim Speed, you can use that speed instead of your Speed when you take this action. You choose which speed to use each time you take it. See also “Speed.”

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/dash_action ` |
| ` implementation ` | ` dnd2024:glossary_dash_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` duration ` | ` current_turn ` |

#### Extra movement

<!-- splat:values path=splat_rule/parameters/extra_movement -->

| Field | Value |
| --- | --- |
| ` var ` | ` selected_speed_after_modifiers ` |

#### Speed selection

<!-- splat:values path=splat_rule/parameters/speed_selection -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` subject ` |
| ` timing ` | ` each_dash ` |
| ` options ` | ` all_available_speeds ` |
