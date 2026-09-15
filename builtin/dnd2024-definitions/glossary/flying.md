---
splat_schema: 1
splat_id: f8b2a361-4eb9-5095-a2fc-165a470cecb6
splat_kind: rule
splat_key: dnd2024:glossary/flying
splat_name: Flying
splat_origin:
  key: dnd2024:reference/glossary/flying
splat_sources:
- title: SRD 5.2.1
  locator: pp. 182
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/flying
  parameters:
    fall_when_any:
    - incapacitated
    - prone
    - fly_speed_zero
    hover_prevents_these_falls: true
  implementation: dnd2024:glossary_flying
splat_contracts:
- id: dnd2024:glossary_flying
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - fall_required
  events:
  - condition.changed
  - speed.changed
  capabilities:
  - movement
  - conditions
splat_conversion:
  status: specified
  category: glossary
  source_sha256: a104c7963c43dffc3b1f185f8c473de7d966b84ce67b3969b36b1138807c4ce8
  source_path: builtin/srd-5.2.1/glossary/flying.md
  pages:
  - 182
  engine_gaps: &id001
  - dnd2024:glossary_flying
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 3f5d3591c5a1887b076c0229531cf5413d1aef95a5887b71e29b8fdd85d6aa65
splat_clauses:
- key: source_rules
  text: '# Flying A variety of effects allow a creature to fly. While flying, you fall if you have the
    Incapacitated or Prone condition or your Fly Speed is reduced to 0. You can stay aloft in those circumstances
    if you can hover. See also “Falling” and “Fly Speed.”'
  sha256: 6963eb5f3e47f9262f75117fb7e2b68a533e531063c0d3d974da3ca8a5267300
  status: contract_specified
splat_requires_mechanics: *id001
---

# Flying

<!-- SRD page 182 -->

A variety of effects allow a creature to fly. While fly-\
ing, you fall if you have the Incapacitated or Prone\
condition or your Fly Speed is reduced to 0. You can\
stay aloft in those circumstances if you can hover.\
See also “Falling” and “Fly Speed.”\

Original layout: [[../../srd-5.2.1/pages/182|page 182]].
