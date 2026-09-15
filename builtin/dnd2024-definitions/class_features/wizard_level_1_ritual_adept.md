---
splat_schema: 1
splat_id: 6820a578-3015-57bd-9d12-ad2876ffa85b
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_1_ritual_adept
splat_name: 'Level 1: Ritual Adept'
splat_origin:
  key: dnd2024:reference/pages/078
splat_sources:
- title: SRD 5.2.1
  locator: pp. 78
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_1_ritual_adept
  parameters:
    ritual_casting:
      eligible:
        ritual_tag: true
        in_actor_spellbook: true
      must_be_prepared: false
      must_read_from_book: true
    class_scope:
      class: wizard
      minimum_level: 1
  implementation: dnd2024:class_features_wizard_level_1_ritual_adept
splat_contracts:
- id: dnd2024:class_features_wizard_level_1_ritual_adept
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - spell.ritual_cast_pending
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 78
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_1_ritual_adept
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 060d4ac73f1235df50161cd5f80b900e31e2bfe2cd4e31c5266cefa038e5efe6
  parent_name: Wizard
  line_span:
    first: 6670
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 1: Ritual Adept You can cast any spell as a Ritual if that spell has the Ritual tag and
    the spell is in your spellbook. You needn’t have the spell prepared, but you must read from the book
    to cast a spell in this way.'
  sha256: cacd2237e136eb0dc73b7342f8df16a654dfe74089100302e4eb319d33a0816e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Ritual Adept

<!-- SRD page 78 -->

You can cast any spell as a Ritual if that spell has\
the Ritual tag and the spell is in your spellbook. You\
needn’t have the spell prepared, but you must read\
from the book to cast a spell in this way.\
