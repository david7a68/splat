---
splat_schema: 1
splat_id: 48ec9594-2caf-545a-b733-4bafc310bca1
splat_kind: rule
splat_key: dnd2024:glossary/cover
splat_name: Cover
splat_origin:
  key: dnd2024:reference/glossary/cover
splat_sources:
- title: SRD 5.2.1
  locator: pp. 179
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/cover
  parameters:
    degrees:
      half:
        ac_bonus: 2
        dexterity_save_bonus: 2
      three_quarters:
        ac_bonus: 5
        dexterity_save_bonus: 5
      total:
        direct_targeting_allowed: false
    combining_degrees: most_protective_only
  implementation: dnd2024:glossary_cover
splat_contracts:
- id: dnd2024:glossary_cover
  inputs:
    subject:
      type: object
      required: true
    cover:
      type: choice
      required: true
  outputs:
  - cover_benefits
  events:
  - attack.pending
  - saving_throw.pending
  - effect.targeting_pending
  capabilities:
  - targeting
  - checks
  - geometry
splat_conversion:
  status: specified
  category: glossary
  source_sha256: d8aa5db7a01035d73f285bc08cc6c2f04fca5a4392110a93098e40dd31bcee31
  source_path: builtin/srd-5.2.1/glossary/cover.md
  pages:
  - 179
  engine_gaps: &id001
  - dnd2024:glossary_cover
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 53991dda54f478476a37a2f19f9c1e173a3ed6c941e498e39b53dd6cfe99efb7
splat_clauses:
- key: source_rules
  text: '# Cover Cover provides a degree of protection to a target behind it. There are three degrees
    of cover, each of which provides a different benefit to a target: Half Cover (+2 bonus to AC and Dexterity
    saving throws), Three-Quarters Cover (+5 bonus to AC and Dexterity saving throws), and Total Cover
    (can’t be targeted directly). If behind more than one degree of cover, a target benefits only from
    the most protective degree. See also “Playing the Game” (“Combat”).'
  sha256: b0f093410965c22ee045efa86d2b08111333f3fb47de3e018ff4fb5a5f1844b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cover

<!-- SRD page 179 -->

Cover provides a degree of protection to a target\
behind it. There are three degrees of cover, each of\
which provides a different benefit to a target: Half\
Cover (+2 bonus to AC and Dexterity saving throws),\
Three-Quarters Cover (+5 bonus to AC and Dexter-\
ity saving throws), and Total Cover (can’t be tar-\
geted directly). If behind more than one degree of\
cover, a target benefits only from the most protec-\
tive degree. See also “Playing the Game” (“Combat”).\

Original layout: [[../../srd-5.2.1/pages/179|page 179]].
