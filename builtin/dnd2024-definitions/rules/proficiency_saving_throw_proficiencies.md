---
splat_schema: 1
splat_id: 9c5c8008-8320-5d7f-8938-30a3f5e7036e
splat_kind: rule
splat_key: dnd2024:rules/proficiency_saving_throw_proficiencies
splat_name: Saving Throw Proficiencies
splat_origin:
  key: dnd2024:reference/pages/009
splat_sources:
- title: SRD 5.2.1
  locator: pp. 9
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/proficiency_saving_throw_proficiencies
  parameters:
    saving_throw_bonus:
      base: ability_modifier
      proficient_add: proficiency_bonus
    starting_class_save_proficiencies_minimum: 2
    monster_proficiencies: stat_block
  implementation: dnd2024:rule_proficiency_saving_throw_proficiencies
splat_contracts:
- id: dnd2024:rule_proficiency_saving_throw_proficiencies
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - save_resolution
  events: []
  capabilities:
  - saving_throws
  - proficiency
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 9
  engine_gaps: &id001
  - dnd2024:rule_proficiency_saving_throw_proficiencies
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 0537fce63de0826fdc8a3b1423fe4a9c33dfc26be7521007dbedeb42dfdc556c
  parent_name: Proficiency
  line_span:
    first: 423
    count: 12
splat_clauses:
- key: source_rules
  text: '# Saving Throw Proficiencies Proficiency in a saving throw lets a character add their Proficiency
    Bonus to saves that use a particular ability. For example, proficiency in Wisdom saves lets you add
    your Proficiency Bonus to your Wisdom saves. Some monsters also have saving throw proficiencies, as
    noted in their stat blocks. Each class gives proficiency in at least two saving throws, representing
    that class’s training in evading or resisting certain threats. Wizards, for example, are proficient
    in Intelligence and Wisdom saves; they train to resist mental assault.'
  sha256: 3e5c2e83e94afa39f69d37b8a9f4d3a32a6b82abd0f8dfcfc1e05d506a23587a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saving Throw Proficiencies

<!-- SRD page 9 -->

Proficiency in a saving throw lets a character add\
their Proficiency Bonus to saves that use a partic-\
ular ability. For example, proficiency in Wisdom\
saves lets you add your Proficiency Bonus to your\
Wisdom saves. Some monsters also have saving\
throw proficiencies, as noted in their stat blocks.\
Each class gives proficiency in at least two saving\
throws, representing that class’s training in evading\
or resisting certain threats. Wizards, for example,\
are proficient in Intelligence and Wisdom saves;\
they train to resist mental assault.\
