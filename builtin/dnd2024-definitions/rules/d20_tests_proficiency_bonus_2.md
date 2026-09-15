---
splat_schema: 1
splat_id: e96d0346-2e14-54db-8a74-6b9a0e43bbac
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_proficiency_bonus_2
splat_name: Proficiency Bonus
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_proficiency_bonus_2
  parameters:
    test: saving_throw
    bonus:
      when: proficient_in_save_ability
      value:
        var: subject.proficiency_bonus
  implementation: dnd2024:rules_d20_tests_proficiency_bonus_2
splat_contracts:
- id: dnd2024:rules_d20_tests_proficiency_bonus_2
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - saving_throw.calculate
  capabilities:
  - checks
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rules_d20_tests_proficiency_bonus_2
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: b0a9b501da758f80565efbf1357be9e47ed89661564a36e111afa412425ca690
  parent_name: D20 Tests
  line_span:
    first: 213
    count: 4
splat_clauses:
- key: source_rules
  text: '# Proficiency Bonus You add your Proficiency Bonus to your saving throw if you have proficiency
    in that kind of save. See “Proficiency” later in “Playing the Game.”'
  sha256: b6c8dee05c9a173061da3ecf5cd468dd1cf231904b0413581285da13a92d52b7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiency Bonus

<!-- SRD page 7 -->

You add your Proficiency Bonus to your saving\
throw if you have proficiency in that kind of save.\
See “Proficiency” later in “Playing the Game.”\
