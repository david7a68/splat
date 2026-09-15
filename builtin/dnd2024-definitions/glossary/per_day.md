---
splat_schema: 1
splat_id: 191c5e14-4406-50f6-bd02-5344a2611871
splat_kind: rule
splat_key: dnd2024:glossary/per_day
splat_name: Per Day
splat_origin:
  key: dnd2024:reference/glossary/per_day
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/per_day
  parameters:
    usage_limit: feature_specified
    reset: long_rest_completed
    exhausted_uses_require_reset: true
  implementation: dnd2024:glossary_per_day
splat_contracts:
- id: dnd2024:glossary_per_day
  inputs:
    subject:
      type: participant
      required: true
    feature:
      type: definition
      required: true
    maximum_uses:
      type: integer
      required: true
  outputs:
  - remaining_uses
  events:
  - feature.used
  - long_rest.completed
  capabilities:
  - resources
  - rests
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 79e4e62d8a545eae3932dff938c03391fa876fc7979ddfda5615de042030510a
  source_path: builtin/srd-5.2.1/glossary/per_day.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_per_day
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: c02cbb7ff634114fc1f19810ab26b8da8c34135e07dc52798e4c586f3612f12f
splat_clauses:
- key: source_rules
  text: '# Per Day If a rule says you can use something a certain number of times per day, that means
    you must finish a Long Rest to use it again after you run out of uses.'
  sha256: 8e648d54d90b6aaac88e43c1fb89096ad0e7284927bbbde5c58b9631b0e7b2d7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Per Day

<!-- SRD page 186 -->

If a rule says you can use something a certain num-\
ber of times per day, that means you must finish a\
Long Rest to use it again after you run out of uses.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
