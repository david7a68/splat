---
splat_schema: 1
splat_id: 668748bb-2334-5900-958a-e36aa5809c31
splat_kind: rule
splat_key: dnd2024:rules/d20_tests_they_don_t_stack
splat_name: They Don’t Stack
splat_origin:
  key: dnd2024:reference/pages/008
splat_sources:
- title: SRD 5.2.1
  locator: pp. 8
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/d20_tests_they_don_t_stack
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 8
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: D20 Tests
  line_span:
    first: 315
    count: 11
splat_clauses:
- key: source_rules
  text: '# They Don’t Stack If multiple situations affect a roll and they all grant Advantage on it, you
    still roll only two d20s. Similarly, if multiple situations impose Disadvantage on a roll, you roll
    only two d20s. If circumstances cause a roll to have both Advantage and Disadvantage, the roll has
    neither of them, and you roll one d20. This is true even if multiple circumstances impose Disadvantage
    and only one grants Advantage or vice versa. In such a situation, you have neither Advantage nor Disadvantage.'
  sha256: 544cec0f9701e78d5e8f64a69cd8719c83c92797289af50134cf86656fc43f1a
  status: needs_semantic_conversion
---

# They Don’t Stack

<!-- SRD page 8 -->

If multiple situations affect a roll and they all grant\
Advantage on it, you still roll only two d20s. Simi-\
larly, if multiple situations impose Disadvantage on\
a roll, you roll only two d20s.\
If circumstances cause a roll to have both Advan-\
tage and Disadvantage, the roll has neither of them,\
and you roll one d20. This is true even if multiple\
circumstances impose Disadvantage and only one\
grants Advantage or vice versa. In such a situation,\
you have neither Advantage nor Disadvantage.\
