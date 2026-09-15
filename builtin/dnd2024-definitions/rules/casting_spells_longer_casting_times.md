---
splat_schema: 1
splat_id: 5f47ae82-dfc2-5688-9344-bdeb2186d47b
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_longer_casting_times
splat_name: Longer Casting Times
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_longer_casting_times
  parameters:
    casting_time_minimum_seconds: 60
    each_turn_action: magic
    requires_concentration_during_casting: true
    concentration_broken:
      spell_fails: true
      slot_expended: false
      restart_required: true
    includes_rituals: true
  implementation: dnd2024:rules_casting_spells_longer_casting_times
splat_contracts:
- id: dnd2024:rules_casting_spells_longer_casting_times
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
  events:
  - turn.started
  - concentration.ended
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
  - dnd2024:rules_casting_spells_longer_casting_times
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: caf2d1563304e94083178e488cc5b33edb09015d7fffb3d8370a1966f7198b7c
  parent_name: Casting Spells
  line_span:
    first: 9165
    count: 10
splat_clauses:
- key: source_rules
  text: '# Longer Casting Times Certain spells—including a spell cast as a Ritual—require more time to
    cast: minutes or even hours. While you cast a spell with a casting time of 1 minute or more, you must
    take the Magic action on each of your turns, and you must maintain Concentration (see “Rules Glossary”)
    while you do so. If your Concentration is broken, the spell fails, but you don’t expend a spell slot.
    To cast the spell again, you must start over.'
  sha256: d66a12d3524142edcb028b406d04e9aa3665b6ea4b06dfed045fc627461a69de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Longer Casting Times

<!-- SRD page 105 -->

Certain spells—including a spell cast as a Rit-\
ual—require more time to cast: minutes or even\
hours. While you cast a spell with a casting time of\
1 minute or more, you must take the Magic action\
on each of your turns, and you must maintain Con-\
centration (see “Rules Glossary”) while you do so. If\
your Concentration is broken, the spell fails, but you\
don’t expend a spell slot. To cast the spell again, you\
must start over.\
