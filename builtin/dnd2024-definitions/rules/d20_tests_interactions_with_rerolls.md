---
splat_schema: 1
splat_id: e49dd73b-6cc4-5e8c-9ab2-6abfa3cfdd17
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_interactions_with_rerolls
splat_name: Interactions with Rerolls
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_interactions_with_rerolls
  parameters:
    when:
    - advantage_or_disadvantage
    - reroll_or_replace_d20
    maximum_affected_dice: 1
    die_selection: roller_choice
  implementation: dnd2024:rule_d20_tests_interactions_with_rerolls
splat_contracts:
- id: dnd2024:rule_d20_tests_interactions_with_rerolls
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - dice_selection
  events: []
  capabilities:
  - dice
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 8
  engine_gaps: &id001
  - dnd2024:rule_d20_tests_interactions_with_rerolls
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: c91b674c84d029257c9a1779f95ffd31c720fd800524daca1b4c3a6cee54e71b
  parent_name: D20 Tests
  line_span:
    first: 326
    count: 10
splat_clauses:
- key: source_rules
  text: '# Interactions with Rerolls When you have Advantage or Disadvantage and something in the game
    lets you reroll or replace the d20, you can reroll or replace only one die, not both. You choose which
    one. For example, if you have Heroic Inspiration (see the sidebar below) and roll a 3 and an 18 on
    an ability check that has Advantage or Disadvantage, you could expend your Heroic Inspiration to reroll
    one of those dice, not both of them.'
  sha256: d7c62989adfc3ecc4f90ba1918a158b7dc8302a5fe455681f7878707abe856f0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Interactions with Rerolls

<!-- SRD page 8 -->

When you have Advantage or Disadvantage and\
something in the game lets you reroll or replace the\
d20, you can reroll or replace only one die, not both.\
You choose which one.\
For example, if you have Heroic Inspiration (see\
the sidebar below) and roll a 3 and an 18 on an abil-\
ity check that has Advantage or Disadvantage, you\
could expend your Heroic Inspiration to reroll one\
of those dice, not both of them.\
