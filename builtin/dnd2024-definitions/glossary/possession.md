---
splat_schema: 1
splat_id: 5d029d8a-e7b6-5585-9abe-2c13e96f0527
splat_kind: rule
splat_key: dnd2024:glossary/possession
splat_name: Possession
splat_origin:
  key: dnd2024:reference/glossary/possession
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/possession
  parameters:
    operation: possessing_effect_defined
    prevention_source:
      key: dnd2024:spells/protection_from_evil_and_good
    ending_source:
      key: dnd2024:spells/dispel_evil_and_good
    spell_specific_eligibility_still_applies: true
  implementation: dnd2024:glossary_possession
splat_contracts:
- id: dnd2024:glossary_possession
  inputs:
    subject:
      type: participant
      required: true
    possession:
      type: effect
      required: true
    counter_effect:
      required: false
      type: effect
  outputs:
  - possession_state
  events:
  - possession.pending
  - effect.removal_pending
  capabilities:
  - effects
  - possession
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 48eda4219087b05fcee9142e5cbcbc5fc757ac6d216688dfaa72e5ff24297d37
  source_path: builtin/srd-5.2.1/glossary/possession.md
  pages:
  - 186
  engine_gaps: &id001
  - dnd2024:glossary_possession
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: dda0c78daa284cf89ec8bdd81443e6a3b9b05435d6a8cd9fbd09e2be8a394325
splat_clauses:
- key: source_rules
  text: '# Possession Some effects cause a creature to be possessed by another creature or entity. A possessing
    effect defines how the possession operates. Possession can be prevented by the Protection from Evil
    and Good spell and ended by the Dispel Evil and Good spell.'
  sha256: 40b23ad39185d7d497d0ddf61bfcfaf67a8781c4efe88664e30325a1c0f19389
  status: contract_specified
splat_requires_mechanics: *id001
---

# Possession

<!-- SRD page 186 -->

Some effects cause a creature to be possessed by\
another creature or entity. A possessing effect de-\
fines how the possession operates. Possession can\
be prevented by the Protection from Evil and Good\
spell and ended by the Dispel Evil and Good spell.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].
