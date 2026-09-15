---
splat_schema: 1
splat_id: 823c2e47-2e1a-54e0-b527-5c297213e020
splat_kind: rule
splat_key: dnd2024:rules/multiclassing
splat_name: Multiclassing
splat_origin:
  key: dnd2024:reference/pages/024
splat_sources:
- title: SRD 5.2.1
  locator: pp. 24
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/multiclassing
  parameters:
    on_level_gain_choose:
    - advance_existing_class
    - gain_new_class
    multiple_classes_allowed: true
  implementation: dnd2024:rules_multiclassing
splat_contracts:
- id: dnd2024:rules_multiclassing
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - character.level_gained
  capabilities:
  - multiclassing
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 24
  engine_gaps: &id001
  - dnd2024:rules_multiclassing
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: e2cb0050e483a6666c5bd5b9073189b3836d5ff22ab93ce99ac0aa0248849896
  parent_name: Multiclassing
  line_span:
    first: 1894
    count: 8
splat_clauses:
- key: source_rules
  text: '# Multiclassing Multiclassing allows you to gain levels in multiple classes. With this rule,
    you have the option of gaining a level in a new class whenever you advance in level instead of gaining
    a level in your current class. Doing so lets you mix the abilities of those classes to realize a character
    concept that might not be reflected in a single class.'
  sha256: cf666835bf7b59d563e7375c6967c95912f3af29bb1a9f38540c022e38922acb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Multiclassing

<!-- SRD page 24 -->

Multiclassing allows you to gain levels in multiple\
classes. With this rule, you have the option of gain-\
ing a level in a new class whenever you advance in\
level instead of gaining a level in your current class.\
Doing so lets you mix the abilities of those classes\
to realize a character concept that might not be re-\
flected in a single class.\
