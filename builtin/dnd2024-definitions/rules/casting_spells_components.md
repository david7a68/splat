---
splat_schema: 1
splat_id: 51fd8389-77a9-585b-9710-04d4924f3723
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_components
splat_name: Components
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_components
  parameters:
    component_types:
    - verbal
    - somatic
    - material
    required_components: spell_definition
    cast_allowed_only_if_all_required_components_available: true
  implementation: dnd2024:rule_casting_spells_components
splat_contracts:
- id: dnd2024:rule_casting_spells_components
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rule_casting_spells_components
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 90fb10d10abd2c10ec5c07c81b565832e7520a9fe678ca9f958c8ef3a6c54076
  parent_name: Casting Spells
  line_span:
    first: 9190
    count: 7
splat_clauses:
- key: source_rules
  text: '# Components A spell’s components are physical requirements the spellcaster must meet to cast
    the spell. Each spell’s description indicates whether it requires Verbal (V), Somatic (S), or Material
    (M) components. If the spellcaster can’t provide one or more of a spell’s components, the spellcaster
    can’t cast the spell.'
  sha256: 62a4421c2bab66ad26646e483f3dbe0b071b79db3fa330731ff514cd1af3689a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Components

<!-- SRD page 105 -->

A spell’s components are physical requirements the\
spellcaster must meet to cast the spell. Each spell’s\
description indicates whether it requires Verbal\
(V), Somatic (S), or Material (M) components. If the\
spellcaster can’t provide one or more of a spell’s\
components, the spellcaster can’t cast the spell.\
