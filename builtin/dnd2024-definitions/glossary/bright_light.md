---
splat_schema: 1
splat_id: 864c0a59-7a94-5209-ad0a-531f3efcc301
splat_kind: rule
splat_key: dnd2024:glossary/bright_light
splat_name: Bright Light
splat_origin:
  key: dnd2024:reference/glossary/bright_light
splat_sources:
- title: SRD 5.2.1
  locator: pp. 178
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/bright_light
  parameters:
    illumination: normal
  implementation: dnd2024:glossary_bright_light
splat_contracts:
- id: dnd2024:glossary_bright_light
  inputs:
    location:
      type: location
      required: true
  outputs:
  - illumination
  events: []
  capabilities:
  - visibility
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 82899e1ff29437920374fd799db71aebecbc8e3e7ba4f99048a63bcc07a478aa
  source_path: builtin/srd-5.2.1/glossary/bright_light.md
  pages:
  - 178
  engine_gaps: &id001
  - dnd2024:glossary_bright_light
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: fa876b837486e88e82f9690280c8622bb23291f3a9fb23ab56795c5af0a441bf
splat_clauses:
- key: source_rules
  text: '# Bright Light Bright Light is normal illumination. See also “Playing the Game” (“Exploration”).'
  sha256: 54e64df5bcc00b4f3c735911a9d7445b832d51f2499a2e5d7602a8bc1acffae7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bright Light

<!-- SRD page 178 -->

Bright Light is normal illumination. See also “Play-\
ing the Game” (“Exploration”).\

Original layout: [[../../srd-5.2.1/pages/178|page 178]].
