---
splat_schema: 1
splat_id: cde6ea23-885b-5ef4-a0f4-848d6d0447bd
splat_kind: rule
splat_key: dnd2024:glossary/jumping
splat_name: Jumping
splat_origin:
  key: dnd2024:reference/glossary/jumping
splat_sources:
- title: SRD 5.2.1
  locator: pp. 184
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/jumping
  parameters:
    select_one:
    - direction: horizontal
      rule:
        key: dnd2024:glossary/long_jump
    - direction: vertical
      rule:
        key: dnd2024:glossary/high_jump
  implementation: dnd2024:glossary_jumping
splat_contracts:
- id: dnd2024:glossary_jumping
  inputs:
    subject:
      type: participant
      required: true
    direction:
      type: choice
      required: true
  outputs:
  - jump_rule
  events: []
  capabilities:
  - movement
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 8ca422bbdae4f3377bf3b4a6f7f1bb21448054d1618ab13e7072f2669e755961
  source_path: builtin/srd-5.2.1/glossary/jumping.md
  pages:
  - 184
  engine_gaps: &id001
  - dnd2024:glossary_jumping
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 7a0090e3f5bae336bde0dbe0b1e458b855c8f2d3edc37e876634a7f08f6f5ae4
splat_clauses:
- key: source_rules
  text: '# Jumping When you jump, you make either a Long Jump (horizontal) or a High Jump (vertical).
    See also “Long Jump” and “High Jump.”'
  sha256: 269d2efdfbff5f636b2f2f618f96c803c0f0e25327cce2725dae110ec1e1c38e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Jumping

<!-- SRD page 184 -->

When you jump, you make either a Long Jump (hor-\
izontal) or a High Jump (vertical). See also “Long\
Jump” and “High Jump.”\

Original layout: [[../../srd-5.2.1/pages/184|page 184]].
