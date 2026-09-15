---
splat_schema: 1
splat_id: b67983cc-4eb9-5253-8c2a-8480d86cc59d
splat_kind: spell
splat_key: dnd2024:spells/glibness
splat_name: Glibness
splat_origin:
  key: dnd2024:reference/spell/glibness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 136
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 8
  school: enchantment
  classes:
  - bard
  - warlock
  components:
  - verbal
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: self
splat_rule:
  mechanic: dnd2024:spells/glibness
  implementation: dnd2024:spell_glibness
  parameters:
    spell_lifetime:
      concentration: false
      maximum_seconds: 3600
    target: caster
    roll_replacement:
      event: check.rolled
      ability: charisma
      optional: true
      chooser: caster
      replace: die_result
      value: 15
    truth_detection:
      source_kind: magic
      actual_statement_truth: any
      reported_truth: truthful
splat_contracts:
- id: dnd2024:spell_glibness
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    replace_roll:
      required: false
      type: boolean
  outputs:
  - effects
  - information
  events:
  - check.rolled
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - information
splat_conversion:
  status: specified
  category: spell
  source_sha256: c2cbebe1be93017cb7657f11fce757d889648f41ecb027da5e774d6012c1395b
  source_path: builtin/srd-5.2.1/spell/glibness.md
  pages:
  - 136
  engine_gaps: &id001
  - dnd2024:spell_glibness
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: fd6ae9fd2589f3dd983a1e1ddac7d32ca823d0801623eb6614fe90c09f50eb23
splat_clauses:
- key: source_rules
  text: '# Glibness Level 8 Enchantment (Bard, Warlock) Casting Time: Action Range: Self Components: V
    Duration: 1 hour Until the spell ends, when you make a Charisma check, you can replace the number
    you roll with a 15. Additionally, no matter what you say, magic that would determine if you are telling
    the truth indicates that you are being truthful.'
  sha256: ff0876100c558611570fc55ca9851e794570148dd0d044a9ddef68f3a0695c96
  status: contract_specified
splat_requires_mechanics: *id001
---

# Glibness

<!-- SRD page 136 -->

Level 8 Enchantment (Bard, Warlock)\
Casting Time: Action\
Range: Self\
Components: V\
Duration: 1 hour\
Until the spell ends, when you make a Charisma\
check, you can replace the number you roll with a\
15. Additionally, no matter what you say, magic that\
would determine if you are telling the truth indi-\
cates that you are being truthful.\

Original layout: [[../../srd-5.2.1/pages/136|page 136]].
