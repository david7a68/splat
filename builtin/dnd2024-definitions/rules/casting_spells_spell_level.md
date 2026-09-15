---
splat_schema: 1
splat_id: e5459183-a32f-5c7f-8a3c-9d0db511de4e
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_spell_level
splat_name: Spell Level
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_spell_level
  parameters:
    spell_level_minimum: 0
    spell_level_maximum: 9
    cantrip_level: 0
    level_access_source: class_spellcasting_rules
  implementation: dnd2024:rules_casting_spells_spell_level
splat_contracts:
- id: dnd2024:rules_casting_spells_spell_level
  inputs:
    definition:
      type: definition
      required: true
  outputs:
  - typed_attributes
  events: []
  capabilities:
  - definition_catalogs
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_spell_level
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: cfbaeef6bd039d4197caebc7574576b36868e7394505c066d9ac13c035a1d5c4
  parent_name: Casting Spells
  line_span:
    first: 9071
    count: 7
splat_clauses:
- key: source_rules
  text: '# Spell Level Every spell has a level from 0 to 9, which is indicated in a spell’s description.
    A spell’s level is an indicator of how powerful it is. Cantrips—simple spells that can be cast almost
    by rote—are level 0. The rules for each spellcasting class say when its members gain access to spells
    of certain levels.'
  sha256: 2f9fc90b17214de2d04530a8a01dbe4f3910259c4b53e12cadaa249032dea0d7
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spell Level

<!-- SRD page 104 -->

Every spell has a level from 0 to 9, which is indi-\
cated in a spell’s description. A spell’s level is an\
indicator of how powerful it is. Cantrips—simple\
spells that can be cast almost by rote—are level 0.\
The rules for each spellcasting class say when its\
members gain access to spells of certain levels.\
