---
splat_schema: 1
splat_id: d7b49410-af5d-54d4-8696-15e5b1c901ad
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_rolling_20_or_1
splat_name: Rolling 20 or 1
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_rolling_20_or_1
  parameters:
    test: attack_roll
    natural_results:
      '20':
        hit: true
        critical_hit: true
        ignore_ac_and_modifiers: true
      '1':
        hit: false
        ignore_ac_and_modifiers: true
  implementation: dnd2024:rules_d20_tests_rolling_20_or_1
splat_contracts:
- id: dnd2024:rules_d20_tests_rolling_20_or_1
  inputs:
    attack:
      type: resolution
      required: true
  outputs:
  - attack_result
  events:
  - attack.roll_resolved
  capabilities:
  - attacks
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_rolling_20_or_1
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 988374b8b5e564dd38b91b489622b8ad6cab9cc9ebe2b7a56fc6c70c5612a939
  parent_name: D20 Tests
  line_span:
    first: 270
    count: 8
splat_clauses:
- key: source_rules
  text: '# Rolling 20 or 1 If you roll a 20 on the d20 (called a “natural 20”) for an attack roll, the
    attack hits regardless of any modifiers or the target’s AC. This is called a Critical Hit (see “Combat”
    later in “Playing the Game”). If you roll a 1 on the d20 (a “natural 1”) for an attack roll, the attack
    misses regardless of any modifiers or the target’s AC.'
  sha256: 4669701416b00ef2b8f7325bce9aaa434c6c978b8fb1b1f823d83cfa5a267ded
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rolling 20 or 1

<!-- SRD page 7 -->

If you roll a 20 on the d20 (called a “natural 20”) for\
an attack roll, the attack hits regardless of any mod-\
ifiers or the target’s AC. This is called a Critical Hit\
(see “Combat” later in “Playing the Game”).\
If you roll a 1 on the d20 (a “natural 1”) for an at-\
tack roll, the attack misses regardless of any modifi-\
ers or the target’s AC.\
