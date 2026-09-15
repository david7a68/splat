---
splat_schema: 1
splat_id: 35969c95-4982-594e-a08b-627c6b2f1b2f
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_lose_temporary_hit_points_first
splat_name: Lose Temporary Hit Points First
splat_origin:
  key: dnd2024:reference/pages/018
splat_sources:
- title: SRD 5.2.1
  locator: pp. 18
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_lose_temporary_hit_points_first
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 18
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: Damage and Healing
  line_span:
    first: 1292
    count: 6
splat_clauses:
- key: source_rules
  text: '# Lose Temporary Hit Points First If you have Temporary Hit Points and take damage, those points
    are lost first, and any leftover damage carries over to your Hit Points. For example, if you have
    5 Temporary Hit Points and take 7 damage, you lose those points and then lose 2 Hit Points.'
  sha256: 80864106c2cee2e5c110329f67c4d1c57892ab491f301cb7dce00452e68f414a
  status: needs_semantic_conversion
---

# Lose Temporary Hit Points First

<!-- SRD page 18 -->

If you have Temporary Hit Points and take damage,\
those points are lost first, and any leftover damage\
carries over to your Hit Points. For example, if you\
have 5 Temporary Hit Points and take 7 damage,\
you lose those points and then lose 2 Hit Points.\
