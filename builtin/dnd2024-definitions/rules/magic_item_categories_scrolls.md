---
splat_schema: 1
splat_id: 327b0eb2-2a5c-5a52-ad34-23315690edb5
splat_kind: rule
splat_key: dnd2024:rules/magic_item_categories_scrolls
splat_name: Scrolls
splat_origin:
  key: dnd2024:reference/pages/205
splat_sources:
- title: SRD 5.2.1
  locator: pp. 205
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_item_categories_scrolls
  parameters:
    category: scroll
    consumable: true
    activation_requires: read_scroll
    reader_eligibility: understands_any_written_language
    item_specific_eligibility_overrides: true
    on_magic_invoked:
      reuse_prohibited: true
      destruction_modes:
      - words_fade
      - crumble_to_dust
  implementation: dnd2024:rules_magic_item_categories_scrolls
splat_contracts:
- id: dnd2024:rules_magic_item_categories_scrolls
  inputs:
    subject:
      type: participant
      required: true
    scroll:
      type: item_instance
      required: true
  outputs:
  - activation_eligibility
  - consumed_item
  events:
  - scroll.activated
  capabilities:
  - scrolls
  - languages
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 205
  engine_gaps: &id001
  - dnd2024:rules_magic_item_categories_scrolls
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 9a370e426284e6e62d5a6fcc1752c9ba9c428fa3f9688f08c132299fa00dd12a
  parent_name: Magic Item Categories
  line_span:
    first: 10483
    count: 14
splat_clauses:
- key: source_rules
  text: '# Scrolls An item in the Scroll category is a roll of paper or parchment, sometimes attached
    to wooden rods and typically kept safe in a tube of ivory, jade, leather, metal, or wood. The most
    prevalent scroll is the Spell Scroll, a spell stored in written form. Using a Scroll. Scrolls are
    consumable items. Unleashing the magic in a scroll requires the user to read the scroll. When its
    magic has been invoked, the scroll can’t be used again. Its words fade, or it crumbles into dust.
    Any creature that can understand a written language can read a scroll and attempt to activate it unless
    its description notes otherwise.'
  sha256: 4ea5b2b2546a177764824208febf11fbd152c79a86766e71cafeb132968186f1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Scrolls

<!-- SRD page 205 -->

An item in the Scroll category is a roll of paper or\
parchment, sometimes attached to wooden rods\
and typically kept safe in a tube of ivory, jade,\
leather, metal, or wood. The most prevalent scroll is\
the Spell Scroll, a spell stored in written form.\
Using a Scroll. Scrolls are consumable items. Un-\
leashing the magic in a scroll requires the user to\
read the scroll. When its magic has been invoked,\
the scroll can’t be used again. Its words fade, or it\
crumbles into dust.\
Any creature that can understand a written lan-\
guage can read a scroll and attempt to activate it\
unless its description notes otherwise.\
