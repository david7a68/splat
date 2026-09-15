---
splat_schema: 1
splat_id: 0a6c8f2a-a31d-54c2-84f6-c2804a638acf
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_charges
splat_name: Charges
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/activating_a_magic_item_charges
  parameters:
    activation_expends_specified_charges: true
    identify_reveals: remaining_charges
    attuned_bearer_knows:
    - remaining_charges
    - charge_recovery
  implementation: dnd2024:rules_activating_a_magic_item_charges
splat_contracts:
- id: dnd2024:rules_activating_a_magic_item_charges
  inputs:
    actor:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    choice:
      type: choice
      required: true
    event:
      required: false
      type: event
  outputs:
  - item_updates
  - cost
  - effects
  events: []
  capabilities:
  - items
  - attunement
  - crafting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rules_activating_a_magic_item_charges
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: b9f4ff43254fa94808c3f3b823321fbd49bbb0c3f02f9f97cd9014ec7ae703d3
  parent_name: Activating a Magic Item
  line_span:
    first: 10630
    count: 7
splat_clauses:
- key: source_rules
  text: '# Charges Some magic items have charges that must be expended to activate their properties. The
    number of charges an item has remaining is revealed when the Identify spell is cast on it. A creature
    attuned to an item knows how many charges the item has and how many it regains.'
  sha256: b9bbf960c6e9a696eb832162afb0358bc39e03da4a603d2d83462b150ceee16d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Charges

<!-- SRD page 206 -->

Some magic items have charges that must be ex-\
pended to activate their properties. The number\
of charges an item has remaining is revealed when\
the Identify spell is cast on it. A creature attuned to\
an item knows how many charges the item has and\
how many it regains.\
