---
splat_schema: 1
splat_id: 24cfa087-f9ed-5f95-997f-36352318e52c
splat_kind: rule
splat_key: dnd2024:rules/magic_items_attune_during_a_short_rest
splat_name: Attune during a Short Rest
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/magic_items_attune_during_a_short_rest
  parameters:
    activity: short_rest
    focus_items_max: 1
    requires_physical_contact: true
    cannot_simultaneously_identify_properties: true
    interrupted_rest: attunement_fails
    completed_rest:
      attuned: true
      full_magic_access: true
  implementation: dnd2024:rules_magic_items_attune_during_a_short_rest
splat_contracts:
- id: dnd2024:rules_magic_items_attune_during_a_short_rest
  inputs:
    subject:
      type: participant
      required: true
    item:
      type: item_instance
      required: true
    rest:
      type: resolution
      required: true
  outputs:
  - attunement
  events:
  - rest.short_finished
  - rest.interrupted
  capabilities:
  - attunement
  - rests
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 102
  engine_gaps: &id001
  - dnd2024:rules_magic_items_attune_during_a_short_rest
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d0536fe932bb920bf21cbf5274556fd6881333c36f88d68ceb0e8a6b265c672f
  parent_name: Magic Items
  line_span:
    first: 8909
    count: 11
splat_clauses:
- key: source_rules
  text: '# Attune during a Short Rest Attuning to an item requires you to spend a Short Rest focused on
    only that item while being in physical contact with it (this can’t be the same Short Rest used to
    learn the item’s properties). This focus can take the form of weapon practice (for a Weapon), meditation
    (for a Wand), or some other appropriate activity. If the Short Rest is interrupted, the Attunement
    attempt fails. Otherwise, at the end of the Short Rest, you’re attuned to the magic item and can access
    its full magical capabilities.'
  sha256: 7ae03111478654ee8226ae48fcac27389e1b3c9921f1cf82c562a7cbc5952eeb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Attune during a Short Rest

<!-- SRD page 102 -->

Attuning to an item requires you to spend a Short\
Rest focused on only that item while being in physi-\
cal contact with it (this can’t be the same Short Rest\
used to learn the item’s properties). This focus can\
take the form of weapon practice (for a Weapon),\
meditation (for a Wand), or some other appropriate\
activity. If the Short Rest is interrupted, the At-\
tunement attempt fails. Otherwise, at the end of the\
Short Rest, you’re attuned to the magic item and can\
access its full magical capabilities.\
