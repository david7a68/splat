---
splat_schema: 1
splat_id: 7f2fd838-f544-55fe-96c6-0d192341287d
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_saving_throws
splat_name: Saving Throws
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_saving_throws
  parameters:
    save_ability_and_outcome_source: spell
    dc:
      op: sum
      terms:
      - 8
      - spellcasting_ability_modifier
      - proficiency_bonus
  implementation: dnd2024:rules_casting_spells_saving_throws
splat_contracts:
- id: dnd2024:rules_casting_spells_saving_throws
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
  - 106
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_saving_throws
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: de98bbd1ea86b0bba28cd133a731146feb6ded46fe1987fc88b63919c63d316f
  parent_name: Casting Spells
  line_span:
    first: 9291
    count: 8
splat_clauses:
- key: source_rules
  text: '# Saving Throws Many spells specify that a target makes a saving throw to avoid some or all of
    a spell’s effects. The spell specifies the ability that the target uses for the save and what happens
    on a success or failure. Here’s how to calculate the DC for your spells: Spell save DC = 8 + your
    spellcasting ability modifier + your Proficiency Bonus'
  sha256: d65ab8e9c45ab222391c4488e9d33c491c6f3a775482810cfdde182cff79051b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saving Throws

<!-- SRD page 106 -->

Many spells specify that a target makes a saving\
throw to avoid some or all of a spell’s effects. The\
spell specifies the ability that the target uses for\
the save and what happens on a success or failure.\
Here’s how to calculate the DC for your spells:\
Spell save DC = 8 + your spellcasting ability modifier\
+ your Proficiency Bonus\
