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
splat_rule:
  mechanic: dnd2024:glossary/dash_action
  parameters:
    cost: action
    duration: current_turn
    extra_movement:
      var: selected_speed_after_modifiers
    speed_selection:
      chooser: subject
      timing: each_dash
      options: all_available_speeds
  implementation: dnd2024:glossary_dash_action
splat_contracts:
- id: dnd2024:glossary_dash_action
  inputs:
    subject:
      type: participant
      required: true
    selected_speed:
      type: choice
      required: true
  outputs:
  - extra_movement
  events:
  - action.requested
  capabilities:
  - action_economy
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: c60287c31797ff2e616693a05272b7b680f5d2885a8bf8159e2bffb90d863546
  source_path: builtin/srd-5.2.1/glossary/dash_action.md
  pages:
  - 180
  engine_gaps: &id001
  - dnd2024:glossary_dash_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d3a158a7c3b0cd0758c9a4e05120c63fd294ea93ff245936454ae9423a48b546
splat_clauses:
- key: source_rules
  text: '# Dash [Action] When you take the Dash action, you gain extra movement for the current turn.
    The increase equals your Speed after applying any modifiers. With a Speed of 30 feet, for example,
    you can move up to 60 feet on your turn if you Dash. If your Speed of 30 feet is reduced to 15 feet,
    you can move up to 30 feet this turn if you Dash. If you have a special speed, such as a Fly Speed
    or Swim Speed, you can use that speed instead of your Speed when you take this action. You choose
    which speed to use each time you take it. See also “Speed.”'
  sha256: affe17b500418f42ff208b79bf2f35be37ff2452d47fee021bcf14d13005e8da
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dash [Action]

<!-- SRD page 180 -->

When you take the Dash action, you gain extra\
movement for the current turn. The increase equals\
your Speed after applying any modifiers. With a\
Speed of 30 feet, for example, you can move up to\
60 feet on your turn if you Dash. If your Speed of 30\
feet is reduced to 15 feet, you can move up to 30 feet\
this turn if you Dash.\
If you have a special speed, such as a Fly Speed or\
Swim Speed, you can use that speed instead of your\
Speed when you take this action. You choose which\
speed to use each time you take it. See also “Speed.”\

Original layout: [[../../srd-5.2.1/pages/180|page 180]].
