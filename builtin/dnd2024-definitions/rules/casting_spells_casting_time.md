---
splat_schema: 1
splat_id: f358b1f0-0e7c-5853-9dd4-4b52a8da47f5
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_casting_time
splat_name: Casting Time
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_casting_time
  parameters:
    casting_economy_source: spell_casting_time
    default_economy: magic_action
    other_casting_times:
    - bonus_action
    - reaction
    - one_minute_or_longer
  implementation: dnd2024:rules_casting_spells_casting_time
splat_contracts:
- id: dnd2024:rules_casting_spells_casting_time
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - spell_state
  - casting_requirements
  events: []
  capabilities:
  - spellcasting
  - concentration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 105
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_casting_time
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 853efa737c471a290bcde49548884cb6b1a821697baf73302bb6ebaf539cc420
  parent_name: Casting Spells
  line_span:
    first: 9149
    count: 5
splat_clauses:
- key: source_rules
  text: '# Casting Time Most spells require the Magic action to cast, but some spells require a Bonus
    Action, a Reaction, or 1 minute or more. A spell’s Casting Time entry specifies which of those is
    required.'
  sha256: 82fcee3e0063f1f2d66c780280c1e522f121ecbdf84c11c219b8c835623c17d9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Casting Time

<!-- SRD page 105 -->

Most spells require the Magic action to cast, but\
some spells require a Bonus Action, a Reaction, or 1\
minute or more. A spell’s Casting Time entry speci-\
fies which of those is required.\
