---
splat_schema: 1
splat_id: d46db1d1-82f1-5006-a208-795a00cf1d51
splat_kind: rule
splat_key: dnd2024:rules/character_backgrounds_equipment
splat_name: Equipment
splat_origin:
  key: dnd2024:reference/pages/083
splat_sources:
- title: SRD 5.2.1
  locator: pp. 83
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/character_backgrounds_equipment
  parameters:
    choose_one:
    - background_equipment_package
    - currency: gp
      amount: 50
  implementation: dnd2024:rule_character_backgrounds_equipment
splat_contracts:
- id: dnd2024:rule_character_backgrounds_equipment
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - character_changes
  events: []
  capabilities:
  - character_creation
  - equipment
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 83
  engine_gaps: &id001
  - dnd2024:rule_character_backgrounds_equipment
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: f4177a4d38d4c0474a0a65445a903d83c04e5e8fa6fbb82429f4a435ea8dd304
  parent_name: Character Backgrounds
  line_span:
    first: 7083
    count: 3
splat_clauses:
- key: source_rules
  text: '# Equipment Each background offers a choice between a package of equipment and 50 GP.'
  sha256: b7b6ee2c8e21d96784655afebe1d68bceaea75b93b9ab1a91ebaeb50821237c2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Equipment

<!-- SRD page 83 -->

Each background offers a choice between a package\
of equipment and 50 GP.\
