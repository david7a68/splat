---
splat_schema: 1
splat_id: ff60f5c3-1473-525c-9dac-3aab5d4d9736
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_proficiency_bonus_3
splat_name: Proficiency Bonus
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_proficiency_bonus_3
  parameters:
    add_proficiency_bonus_when:
      any:
      - proficient_with_used_weapon
      - spell_attack
  implementation: dnd2024:rule_d20_tests_proficiency_bonus_3
splat_contracts:
- id: dnd2024:rule_d20_tests_proficiency_bonus_3
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - attack_modifiers
  events: []
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
  - dnd2024:rule_d20_tests_proficiency_bonus_3
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ac8aa56e203308d326730b7dd0b63c7119db5e7f73a169ec113acba5abeb7a2a
  parent_name: D20 Tests
  line_span:
    first: 247
    count: 6
splat_clauses:
- key: source_rules
  text: '# Proficiency Bonus You add your Proficiency Bonus to your attack roll when you attack using
    a weapon you have proficiency with, as well as when you attack with a spell. See “Proficiency” later
    in “Playing the Game” for more information about weapon proficiencies.'
  sha256: fd212d901a3cdeb8eb6641a5e23339f296c638a0a9fa51721f0148a42c152024
  status: contract_specified
splat_requires_mechanics: *id001
---

# Proficiency Bonus

<!-- SRD page 7 -->

You add your Proficiency Bonus to your attack roll\
when you attack using a weapon you have profi-\
ciency with, as well as when you attack with a spell.\
See “Proficiency” later in “Playing the Game” for\
more information about weapon proficiencies.\
