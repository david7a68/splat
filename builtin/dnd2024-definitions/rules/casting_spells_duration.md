---
splat_schema: 1
splat_id: c17989ab-620a-5a65-9901-b4f1dd984243
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_duration
splat_name: Duration
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_duration
  parameters:
    duration_forms:
      concentration:
        key: dnd2024:glossary/concentration
      instantaneous:
        persists: false
      time_span:
        ends_at: elapsed_duration
        caster_dismissal:
          economy: none
          blocked_by: incapacitated
  implementation: dnd2024:rules_casting_spells_duration
splat_contracts:
- id: dnd2024:rules_casting_spells_duration
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
  - 106
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_duration
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: a6f1d5fa39781804d57f5f11cdb3e88c2882e97c8be168389cf1ee907f4c431f
  parent_name: Casting Spells
  line_span:
    first: 9230
    count: 18
splat_clauses:
- key: source_rules
  text: '# Duration A spell’s duration is the length of time the spell persists after it is cast. A duration
    typically takes one of the following forms: Concentration. A duration that requires Concentration
    follows the Concentration rules (see “Rules Glossary”). Instantaneous. An instantaneous duration means
    the spell’s magic appears only for a moment and then disappears. Time Span. A duration that provides
    a time span specifies how long the spell lasts in rounds, minutes, hours, or the like. For example,
    a Duration entry might say “1 minute,” meaning the spell ends after 1 minute has passed. While a time-span
    spell that you cast is ongoing, you can dismiss it (no action required) if you don’t have the Incapacitated
    condition.'
  sha256: 98a6c61805233ad06935ebedb1303994ef733c3aadeb42d9cd28f6c988331359
  status: contract_specified
splat_requires_mechanics: *id001
---

# Duration

<!-- SRD page 106 -->

A spell’s duration is the length of time the spell per-\
sists after it is cast. A duration typically takes one of\
the following forms:\
Concentration. A duration that requires Concen-\
tration follows the Concentration rules (see “Rules\
Glossary”).\
Instantaneous. An instantaneous duration means\
the spell’s magic appears only for a moment and\
then disappears.\
Time Span. A duration that provides a time span\
specifies how long the spell lasts in rounds, min-\
utes, hours, or the like. For example, a Duration\
entry might say “1 minute,” meaning the spell\
ends after 1 minute has passed. While a time-span\
spell that you cast is ongoing, you can dismiss it\
(no action required) if you don’t have the Incapac-\
itated condition.\
