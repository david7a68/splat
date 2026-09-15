---
splat_schema: 1
splat_id: e9563af3-b932-5670-9fdf-d4b1221e188f
splat_kind: rule
splat_key: dnd2024:glossary/hover
splat_name: Hover
splat_origin:
  key: dnd2024:reference/glossary/hover
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/hover
  parameters:
    grant_sources:
    - stat_block
    - spell
    - other_effect
    while_flying_prevents_fall_from:
    - incapacitated
    - prone
    - fly_speed_zero
    base_rule:
      key: dnd2024:glossary/flying
  implementation: dnd2024:glossary_hover
splat_contracts:
- id: dnd2024:glossary_hover
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - fall_prevention
  events:
  - condition.changed
  - speed.changed
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 10e3fa239c1dfce41ece1add3d1af31fefd518ee84f80ce0f9ebabdb8825463e
  source_path: builtin/srd-5.2.1/glossary/hover.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_hover
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: f86ab9ab6664506dde5c4a29171c84b83aaba9bfc126a6f0d1b402070c673078
splat_clauses:
- key: source_rules
  text: '# Hover Some creatures can hover, as noted in their stat blocks, and some spells and other effects
    grant the ability to hover. Hovering while flying prevents you from falling in certain circumstances.
    See also “Flying.”'
  sha256: 91991fcbb445511506b31c42e4301a92b3877420dd298255257f68d3889da209
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hover

<!-- SRD page 183 -->

Some creatures can hover, as noted in their stat\
blocks, and some spells and other effects grant the\
ability to hover. Hovering while flying prevents\
you from falling in certain circumstances. See also\
“Flying.”\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
