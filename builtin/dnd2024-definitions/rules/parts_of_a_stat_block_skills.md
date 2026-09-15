---
splat_schema: 1
splat_id: 8d552b00-bc66-5976-a196-42d2616cf6cb
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_skills
splat_name: Skills
splat_origin:
  key: dnd2024:reference/pages/255
splat_sources:
- title: SRD 5.2.1
  locator: pp. 255
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_skills
  parameters:
    listed_skills_are_proficiencies: true
    skill_bonus:
      op: add
      args:
      - variable: ability_modifier
      - variable: proficiency_bonus
      - variable: other_modifiers
  implementation: dnd2024:rule_parts_of_a_stat_block_skills
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_skills
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    ability_modifier:
      type: number
      required: true
    other_modifiers:
      type: number
      required: true
    proficiency_bonus:
      type: number
      required: true
  outputs:
  - monster_parameters
  events: []
  capabilities:
  - monsters
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 255
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_skills
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: db9b4f4b5ff9ee85250aa16519f9c0d4b7fd1f598bed9c0ef4ee547fd54de9c1
  parent_name: Parts of a Stat Block
  line_span:
    first: 10998
    count: 8
splat_clauses:
- key: source_rules
  text: '# Skills The Skills entry specifies a monster’s skill proficiencies, if any. For example, a monster
    that is very perceptive and stealthy might have bonuses to Wisdom (Perception) and Dexterity (Stealth)
    checks. A skill bonus is the sum of a monster’s relevant ability modifier and its Proficiency Bonus.
    Other modifiers might apply.'
  sha256: 7901fd44e667defcf3456d05cf9b3ef7ac1f5cb731537b7efb056e44f782db47
  status: contract_specified
splat_requires_mechanics: *id001
---

# Skills

<!-- SRD page 255 -->

The Skills entry specifies a monster’s skill profi-\
ciencies, if any. For example, a monster that is very\
perceptive and stealthy might have bonuses to Wis-\
dom (Perception) and Dexterity (Stealth) checks. A\
skill bonus is the sum of a monster’s relevant ability\
modifier and its Proficiency Bonus. Other modifiers\
might apply.\
