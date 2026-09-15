---
splat_schema: 1
splat_id: 8ad9c718-ec25-51ed-ac7a-e385f2a232e5
splat_kind: rule
splat_key: dnd2024:glossary/illusions
splat_name: Illusions
splat_origin:
  key: dnd2024:reference/glossary/illusions
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/illusions
  parameters:
    deceived_senses_and_faculties: source_effect_defined
    spatial_manifestation:
      substantial: false
      weight: 0
      environment_response_appearance: as_if_real
      default_visual_responses:
      - shadows
      - reflections
      - wind
      default_audible_responses:
      - echo
      source_effect_can_override: true
  implementation: dnd2024:glossary_illusions
splat_contracts:
- id: dnd2024:glossary_illusions
  inputs:
    illusion:
      type: effect
      required: true
    environment:
      type: resolution
      required: true
  outputs:
  - illusion_properties
  events: []
  capabilities:
  - illusions
  - perception
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 632564c14e3c838b0d2c767e9bcc65b6033cee76f84ad027bfb98c4cf3fb9f32
  source_path: builtin/srd-5.2.1/glossary/illusions.md
  pages:
  - 183
  engine_gaps: &id001
  - dnd2024:glossary_illusions
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 45c73d47d165a0bb8ed35e7d041fa1d7cbf2f09a531d0076cb960503281da0b7
splat_clauses:
- key: source_rules
  text: '# Illusions Spells and other effects sometimes create magical illusions. Such an effect defines
    what the illusion does and which senses or mental faculties it deceives. If an illusion manifests
    in space, the illusion is insubstantial and weightless, yet it seems to be affected by the environment
    as if the illusion were real unless the effect that created it specifies otherwise. For example, a
    visual illusion of a creature casts shadows and reflections, and wind appears to affect the illusory
    creature. Similarly, an audible illusion echoes in an echoey space.'
  sha256: 4044b3751658dd12524a4a30889371949be736cad25beb78f4663babe58c99b8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Illusions

<!-- SRD page 183 -->

Spells and other effects sometimes create magical il-\
lusions. Such an effect defines what the illusion does\
and which senses or mental faculties it deceives.\
If an illusion manifests in space, the illusion is\
insubstantial and weightless, yet it seems to be\
affected by the environment as if the illusion were\
real unless the effect that created it specifies oth-\
erwise. For example, a visual illusion of a creature\
casts shadows and reflections, and wind appears\
to affect the illusory creature. Similarly, an audible\
illusion echoes in an echoey space.\

Original layout: [[../../srd-5.2.1/pages/183|page 183]].
