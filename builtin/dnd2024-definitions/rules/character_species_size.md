---
splat_schema: 1
splat_id: 04416556-8bb8-5721-876a-6c88dbbfe67a
splat_kind: rule
splat_key: dnd2024:rules/character_species_size
splat_name: Size
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83, 84
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_species_size
  parameters:
    size_source: species_definition
    if_species_allows_size_choice:
      choose_count: 1
      options:
      - small
      - medium
  implementation: dnd2024:rules_character_species_size
splat_contracts:
- id: dnd2024:rules_character_species_size
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - size
  events: []
  capabilities:
  - character_creation
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  - 84
  engine_gaps: &id001
  - dnd2024:rules_character_species_size
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 3177c4c0fdf52f95f8df43c6bb3e3e793ab3fce06cd9f3b2957de9c4d4cc943c
  parent_name: Character Species
  line_span:
    first: 7150
    count: 6
splat_clauses:
- key: source_rules
  text: '# Size Your character’s species determines the character’s size. Individuals within a species
    cover a wide range of heights, and some species include such diversity of size that you can choose
    whether your character is Small or Medium.'
  sha256: a7372a2c846b7820a646a771c1cda8eb7befb98aa6936f2cc9a803f335bd6fb6
  status: contract_specified
splat_requires_mechanics: *id001
---

# Size

<!-- SRD page 83 -->

Your character’s species determines the character’s\
size. Individuals within a species cover a wide range\
of heights, and some species include such diversity\

<!-- SRD page 84 -->

of size that you can choose whether your character\
is Small or Medium.\
