---
splat_schema: 1
splat_id: 61e01774-725e-58c2-ba42-f192091a2fdf
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_difficulty_class_2
splat_name: Difficulty Class
splat_origin:
  key: dnd2024:reference/pages/007
splat_sources:
- title: SRD 5.2.1
  locator: pp. 7
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_difficulty_class_2
  parameters:
    save_dc_source_priority:
    - effect
    - gm_choice
    spell_dc:
      op: add
      args:
      - 8
      - variable: spellcasting_ability_modifier
      - variable: proficiency_bonus
    monster_ability_dc: stat_block_value
  implementation: dnd2024:rule_d20_tests_difficulty_class_2
splat_contracts:
- id: dnd2024:rule_d20_tests_difficulty_class_2
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    proficiency_bonus:
      type: number
      required: true
    spellcasting_ability_modifier:
      type: number
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - save_resolution
  events: []
  capabilities:
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 7
  engine_gaps: &id001
  - dnd2024:rule_d20_tests_difficulty_class_2
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 63703178f137f889883062944eb5be2e03ac3ac217ea2a9184ee00f24e3043b6
  parent_name: D20 Tests
  line_span:
    first: 217
    count: 7
splat_clauses:
- key: source_rules
  text: '# Difficulty Class The Difficulty Class for a saving throw is determined by the effect that causes
    it or by the GM. For example, if a spell forces you to make a save, the DC is determined by the caster’s
    spellcasting ability and Proficiency Bonus. Monster abilities that call for saves specify the DC.'
  sha256: b5ac7c08d184deeaa26aac943b59aa8e00feee7e9f27ff695a168254f984d7a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Difficulty Class

<!-- SRD page 7 -->

The Difficulty Class for a saving throw is deter-\
mined by the effect that causes it or by the GM. For\
example, if a spell forces you to make a save, the DC\
is determined by the caster’s spellcasting ability\
and Proficiency Bonus. Monster abilities that call\
for saves specify the DC.\
