---
splat_schema: 1
splat_id: 474292db-4ba5-521b-8568-7b91557c05cd
splat_kind: rule
splat_key: dnd2024:glossary/ritual
splat_name: Ritual
splat_origin:
  key: dnd2024:reference/glossary/ritual
splat_sources:
- title: SRD 5.2.1
  locator: pp. 187
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/ritual
  parameters:
    eligibility:
      spell_prepared: true
      ritual_tag: true
    casting_time_added_minutes: 10
    slot_expended: false
    upcast_allowed: false
  implementation: dnd2024:glossary_ritual
splat_contracts:
- id: dnd2024:glossary_ritual
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
  outputs:
  - eligibility
  - casting_time
  - slot_cost
  - cast_level
  events:
  - spell.cast_pending
  capabilities:
  - spellcasting
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 02784dcb33f70a64a674382d432f4df0450a67108075505c0c34ddef21af7a32
  source_path: builtin/srd-5.2.1/glossary/ritual.md
  pages:
  - 187
  engine_gaps: &id001
  - dnd2024:glossary_ritual
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 0a3a9e7cdb49be197caddc4420e1860fc4859d3d9fd44b00d799684ffe4ce887
splat_clauses:
- key: source_rules
  text: '# Ritual If you have a spell prepared that has the Ritual tag, you can cast that spell as a Ritual.
    The Ritual version of a spell takes 10 minutes longer to cast than normal. It also doesn’t expend
    a spell slot, which means the ritual version of a spell can’t be cast at a higher level. See also
    “Spells.”'
  sha256: ee2c0e9c53210c6ae266700acd803beb7c2d89a768d41b01dbdf3fa160f05d8c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ritual

<!-- SRD page 187 -->

If you have a spell prepared that has the Ritual tag,\
you can cast that spell as a Ritual. The Ritual ver-\
sion of a spell takes 10 minutes longer to cast than\
normal. It also doesn’t expend a spell slot, which\
means the ritual version of a spell can’t be cast at a\
higher level. See also “Spells.”\

Original layout: [[../../srd-5.2.1/pages/187|page 187]].
