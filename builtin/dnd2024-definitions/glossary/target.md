---
splat_schema: 1
splat_id: 9e7714b0-18eb-5ea7-a688-8839f21ef7ac
splat_kind: rule
splat_key: dnd2024:glossary/target
splat_name: Target
splat_origin:
  key: dnd2024:reference/glossary/target
splat_sources:
- title: SRD 5.2.1
  locator: pp. 190
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/target
  parameters:
    eligible_kinds:
    - creature
    - object
    targeted_when_any:
    - attack_roll_target
    - forced_save_target
    - selected_spell_recipient
    - selected_other_effect_recipient
  implementation: dnd2024:glossary_target
splat_contracts:
- id: dnd2024:glossary_target
  inputs:
    subject:
      type: object
      required: true
    effect:
      type: effect
      required: true
  outputs:
  - is_target
  events: []
  capabilities:
  - targeting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 4e2670d24fdda1d2a1c81034bc0444657cdf5ca908163e77ac4cd9331b3b06ea
  source_path: builtin/srd-5.2.1/glossary/target.md
  pages:
  - 190
  engine_gaps: &id001
  - dnd2024:glossary_target
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: ae8885e89018d7eff14f417a5ff3bb8e1ae18a18e0ad8cc616d2a62fccc89b99
splat_clauses:
- key: source_rules
  text: '# Target A target is the creature or object targeted by an attack roll, forced to make a saving
    throw by an effect, or selected to receive the effects of a spell or another phenomenon.'
  sha256: 42b7af75d79d6630e977de7653cf904f50ea0efed3098092ca7a98e7f9c52bfe
  status: contract_specified
splat_requires_mechanics: *id001
---

# Target

<!-- SRD page 190 -->

A target is the creature or object targeted by an\
attack roll, forced to make a saving throw by an\
effect, or selected to receive the effects of a spell or\
another phenomenon.\

Original layout: [[../../srd-5.2.1/pages/190|page 190]].
