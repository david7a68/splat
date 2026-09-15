---
splat_schema: 1
splat_id: 1cbdda58-5f19-5a08-a3df-a91b20c6817a
splat_kind: rule
splat_key: dnd2024:glossary/round_down
splat_name: Round Down
splat_origin:
  key: dnd2024:reference/glossary/round_down
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/round_down
  parameters:
    operations:
    - multiply
    - divide
    fractional_result_rounding: floor
    fraction_magnitude_irrelevant: true
    explicit_round_up_rule_overrides: true
  implementation: dnd2024:glossary_round_down
splat_contracts:
- id: dnd2024:glossary_round_down
  inputs:
    value:
      type: number
      required: true
    rounding_override:
      required: false
      type: choice
  outputs:
  - rounded_result
  events: []
  capabilities:
  - arithmetic
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 7f1390d14ec427d8b5cc41668212837bf06243dbd9be57a0f370186ec9ff1d38
  source_path: builtin/srd-5.2.1/glossary/round_down.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_round_down
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 46685d4bf34ffccf95e1c393e3e984de44f5e5cda5708c33504b572eef132fef
splat_clauses:
- key: source_rules
  text: '# Round Down Whenever you divide or multiply a number in the game, round down if you end up with
    a fraction, even if the fraction is one-half or greater. Some rules make an exception and tell you
    to round up.'
  sha256: 7e5cfe1816bfb1737576b25b7383f5bb7d49e245cacf0106c204343f12f63b48
  status: contract_specified
splat_requires_mechanics: *id001
---

# Round Down

<!-- SRD page 187 -->

Whenever you divide or multiply a number in the\
game, round down if you end up with a fraction,\
even if the fraction is one-half or greater. Some\
rules make an exception and tell you to round up.\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
