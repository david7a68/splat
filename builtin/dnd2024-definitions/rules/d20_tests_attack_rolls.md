---
splat_schema: 1
splat_id: 95464330-7ecb-512a-a334-c52988da0c7d
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_attack_rolls
splat_name: Attack Rolls
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_attack_rolls
  parameters:
    hit:
      op: gte
      left: attack_roll_total
      right: target_armor_class
    combat_required: false
  implementation: dnd2024:rules_d20_tests_attack_rolls
splat_contracts:
- id: dnd2024:rules_d20_tests_attack_rolls
  inputs:
    actor:
      type: participant
      required: true
    test:
      type: resolution
      required: true
  outputs:
  - test_modifiers
  - test_result
  events: []
  capabilities:
  - d20_tests
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_attack_rolls
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 7405d29298ba321707d2ce6f5c8527aacf22eacffdef42aad8c403788e631821
  parent_name: D20 Tests
  line_span:
    first: 224
    count: 8
splat_clauses:
- key: source_rules
  text: '# Attack Rolls An attack roll determines whether an attack hits a target. An attack roll hits
    if the roll equals or exceeds the target’s Armor Class. Attack rolls usually occur in battle, described
    in “Combat” later in “Playing the Game,” but the GM might also ask for an attack roll in other situations,
    such as an archery competition.'
  sha256: 8845417265d8d7bf87d84fc9713ae51167f1d48e340ccea34957f4941ce81fac
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attack Rolls

<!-- SRD page 7 -->

An attack roll determines whether an attack hits\
a target. An attack roll hits if the roll equals or\
exceeds the target’s Armor Class. Attack rolls usu-\
ally occur in battle, described in “Combat” later in\
“Playing the Game,” but the GM might also ask for\
an attack roll in other situations, such as an archery\
competition.\
