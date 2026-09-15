---
splat_schema: 1
splat_id: 13adc30e-253e-5245-b9b9-33c8e2f929a4
splat_kind: rule
splat_key: dnd2024:rules/combat_range
splat_name: Range
splat_origin:
  key: dnd2024:reference/pages/015
splat_sources:
- title: SRD 5.2.1
  locator: pp. 15
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/combat_range
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 15
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: Combat
  line_span:
    first: 1012
    count: 11
splat_clauses:
- key: source_rules
  text: '# Range You can make ranged attacks only against targets within a specified range. If a ranged
    attack, such as one made with a spell, has a single range, you can’t attack a target beyond this range.
    Some ranged attacks, such as those made with a Longbow, have two ranges. The smaller number is the
    normal range, and the larger number is the long range. Your attack roll has Disadvantage when your
    target is beyond normal range, and you can’t attack a target beyond long range.'
  sha256: 55085275fa8e68f3537d36114ab99fb0a70f8d1dd0db16aa2d7fb5970779d9fa
  status: needs_semantic_conversion
---

# Range

<!-- SRD page 15 -->

You can make ranged attacks only against targets\
within a specified range. If a ranged attack, such as\
one made with a spell, has a single range, you can’t\
attack a target beyond this range.\
Some ranged attacks, such as those made with a\
Longbow, have two ranges. The smaller number is\
the normal range, and the larger number is the long\
range. Your attack roll has Disadvantage when your\
target is beyond normal range, and you can’t attack\
a target beyond long range.\
