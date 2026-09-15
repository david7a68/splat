---
splat_schema: 1
splat_id: fca817d4-2b03-5651-a738-ef18537f23b7
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_no_stacking
splat_name: No Stacking
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_no_stacking
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: Damage and Healing
  line_span:
    first: 1180
    count: 6
splat_clauses:
- key: source_rules
  text: '# No Stacking Multiple instances of Resistance or Vulnerability that affect the same damage type
    count as only one instance. For example, if you have Resistance to Necrotic damage as well as Resistance
    to all damage, Necrotic damage is reduced by half against you.'
  sha256: 3e091eca54da7cb9acc8431799355663ca84fb4a83dfa9a491bcee9e83011f0e
  status: needs_semantic_conversion
---

# No Stacking

<!-- SRD page 17 -->

Multiple instances of Resistance or Vulnerability\
that affect the same damage type count as only one\
instance. For example, if you have Resistance to Ne-\
crotic damage as well as Resistance to all damage,\
Necrotic damage is reduced by half against you.\
