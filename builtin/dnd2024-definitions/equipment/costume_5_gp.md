---
splat_schema: 1
splat_id: 167d0da6-5679-5113-811a-486cf5968c01
splat_kind: item
splat_key: dnd2024:equipment/costume_5_gp
splat_name: Costume (5 GP)
splat_origin:
  key: dnd2024:reference/pages/097
splat_sources:
- title: SRD 5.2.1
  locator: pp. 97
  url: https://www.dndbeyond.com/srd
splat_item:
  category: adventuring_gear
  equipment:
    attunement: none
  price:
    value: 5
    unit: gp
  mass:
    value: 4.0
    unit: lb
splat_rule:
  mechanic: dnd2024:equipment/costume_5_gp
  implementation: dnd2024:equipment_costume_5_gp
  parameters:
    while_worn:
      roll: ability_check
      mode: advantage
      scope: impersonate_represented_person_or_person_type
    mass:
      value: 4.0
      unit: lb
splat_contracts:
- id: dnd2024:equipment_costume_5_gp
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
  outputs:
  - item_properties
  - conditional_modifiers
  events: []
  capabilities:
  - equipment
  - conditional_modifiers
splat_conversion:
  status: specified
  category: equipment
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 97
  engine_gaps: &id001
  - dnd2024:equipment_costume_5_gp
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/equipment_tables.md
  specification_sha256: c578ed838520bba16afadbf5e4da0bf25dd09867204541cd2e36632999a692c3
  line_span:
    first: 8370
    count: 4
splat_clauses:
- key: source_rules
  text: '# Costume (5 GP) While wearing a Costume, you have Advantage on any ability check you make to
    impersonate the person or type of person it represents.'
  sha256: 47a4191cc194b53748228445160142ed42a4a22795a30df79d9b979822e780a7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Costume (5 GP)

<!-- SRD page 97 -->

While wearing a Costume, you have Advantage on\
any ability check you make to impersonate the per-\
son or type of person it represents.\
