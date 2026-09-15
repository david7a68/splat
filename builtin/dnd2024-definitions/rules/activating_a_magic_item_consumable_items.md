---
splat_schema: 1
splat_id: 8c1b2f53-03f0-5262-bebd-66ac70912d60
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_consumable_items
splat_name: Consumable Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/activating_a_magic_item_consumable_items
  parameters:
    consumed_on_activation: true
    on_consumed:
      magic_lost: true
  implementation: dnd2024:rules_activating_a_magic_item_consumable_items
splat_contracts:
- id: dnd2024:rules_activating_a_magic_item_consumable_items
  inputs:
    item:
      type: item_instance
      required: true
  outputs:
  - consumed_item
  events:
  - item.activated
  capabilities:
  - magic_items
  - inventory
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rules_activating_a_magic_item_consumable_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 19a7f727a3b6ec6f4769ec3557dbe27bb4b4e02d397fc8a43491dd44f9e6a92f
  parent_name: Activating a Magic Item
  line_span:
    first: 10604
    count: 6
splat_clauses:
- key: source_rules
  text: '# Consumable Items Some items are consumed—used up, in other words—when they are activated. A
    Potion of Healing must be swallowed, for example, while the writing vanishes from a scroll when it
    is read. Once used, a consumable item loses its magic.'
  sha256: 74050db394bc903b060794c19c7721d3ff2fab761b49143217ef03487d29e708
  status: contract_specified
splat_requires_mechanics: *id001
---

# Consumable Items

<!-- SRD page 206 -->

Some items are consumed—used up, in other\
words—when they are activated. A Potion of Healing\
must be swallowed, for example, while the writing\
vanishes from a scroll when it is read. Once used, a\
consumable item loses its magic.\
