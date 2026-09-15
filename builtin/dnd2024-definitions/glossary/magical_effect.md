---
splat_schema: 1
splat_id: 56db16e5-094d-575f-a106-16569982da7c
splat_kind: rule
splat_key: dnd2024:glossary/magical_effect
splat_name: Magical Effect
splat_origin:
  key: dnd2024:reference/glossary/magical_effect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/magical_effect
  parameters:
    magical_when_any:
    - created_by_spell
    - created_by_magic_item
    - phenomenon_explicitly_labeled_magical
  implementation: dnd2024:glossary_magical_effect
splat_contracts:
- id: dnd2024:glossary_magical_effect
  inputs:
    effect:
      type: effect
      required: true
  outputs:
  - is_magical
  events: []
  capabilities:
  - effects
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 49c9f4128c9ace17c628205df21d445b2bd513633cfa96ff57f56005a58687bf
  source_path: builtin/srd-5.2.1/glossary/magical_effect.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_magical_effect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: fb2bcae9ae87d2bccc51b036dddb86b0c102bd0763145b19951c7affa17add9f
splat_clauses:
- key: source_rules
  text: '# Magical Effect An effect is magical if it is created by a spell, a magic item, or a phenomenon
    that a rule labels as magical.'
  sha256: ec1e45a577e059153e208a97e62c7b630aebbd65a01a03b58d8a02506430544c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magical Effect

<!-- SRD page 185 -->

An effect is magical if it is created by a spell, a magic\
item, or a phenomenon that a rule labels as magical.\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
