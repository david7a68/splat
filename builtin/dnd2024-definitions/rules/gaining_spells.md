---
splat_schema: 1
splat_id: 80c6e5dc-0a3c-5a4a-a6d5-52f6f3381221
splat_kind: rule
splat_key: dnd2024:rules/gaining_spells
splat_name: Gaining Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/gaining_spells
  parameters:
    cast_access_any:
    - prepared_spell
    - magic_item_spell_access
    feature_controls:
    - spell_access
    - always_prepared
    - prepared_list_changes
  implementation: dnd2024:rule_gaining_spells
splat_contracts:
- id: dnd2024:rule_gaining_spells
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rule_gaining_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: ba2aa4e743c50082947ec6ac27f1a87d962352385a4fb8996b56f5e618cf369b
  parent_name: Gaining Spells
  line_span:
    first: 9030
    count: 8
splat_clauses:
- key: source_rules
  text: '# Gaining Spells Before you can cast a spell, you must have the spell prepared in your mind or
    have access to the spell from a magic item, such as a Spell Scroll. Your features specify which spells
    you have access to, if any; whether you always have certain spells prepared; and whether you can change
    the list of spells you have prepared.'
  sha256: ba44bf4d7baf2056ff349410bb300b0d2b7d91dce58336bca5889df67489562a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gaining Spells

<!-- SRD page 104 -->

Before you can cast a spell, you must have the spell\
prepared in your mind or have access to the spell\
from a magic item, such as a Spell Scroll. Your fea-\
tures specify which spells you have access to, if any;\
whether you always have certain spells prepared;\
and whether you can change the list of spells you\
have prepared.\
