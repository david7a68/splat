---
splat_schema: 1
splat_id: fe3733c3-62f2-5995-895a-68b824d0007f
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls
splat_name: Scribing Spell Scrolls
splat_origin:
  key: dnd2024:reference/pages/103
splat_sources:
- title: SRD 5.2.1
  locator: pp. 103
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/scribing_spell_scrolls
  parameters:
    creator_requires_spellcaster: true
    operation: scribe_spell_scroll
    detailed_rules_source: scribing_spell_scrolls_subsections
    detailed_rules:
    - key: dnd2024:rules/scribing_spell_scrolls_cantrips
    - key: dnd2024:rules/scribing_spell_scrolls_time_and_cost
    - key: dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe
  implementation: dnd2024:rules_scribing_spell_scrolls
splat_contracts:
- id: dnd2024:rules_scribing_spell_scrolls
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
  - 103
  engine_gaps: &id001
  - dnd2024:rules_scribing_spell_scrolls
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: e5df07874039bb84981f959a12bb0057cf2d9fb5f4284fd6f12b7dc7d6a9d893
  parent_name: Scribing Spell Scrolls
  line_span:
    first: 8996
    count: 3
splat_clauses:
- key: source_rules
  text: '# Scribing Spell Scrolls A spellcaster can transfer a spell to a scroll and create a Spell Scroll,
    using the rules below.'
  sha256: 186346ced7d8ba7de8677f3f2919274b45bcee97c145e0b33400c1fc006e1c16
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scribing Spell Scrolls

<!-- SRD page 103 -->

A spellcaster can transfer a spell to a scroll and cre-\
ate a Spell Scroll, using the rules below.\
