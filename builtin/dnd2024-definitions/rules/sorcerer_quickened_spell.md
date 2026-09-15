---
splat_schema: 1
splat_id: acc8a3a9-1c08-5c1c-8569-ebb1df44a31c
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_quickened_spell
splat_name: Quickened Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_quickened_spell
  parameters:
    cost:
      sorcery_points: 2
    requires:
      spell_casting_time: action
      level_1_plus_spell_cast_this_turn: false
    casting_time_for_this_cast: bonus_action
    forbid_later_level_1_plus_spell_cast_this_turn: true
  implementation: dnd2024:rules_sorcerer_quickened_spell
splat_contracts:
- id: dnd2024:rules_sorcerer_quickened_spell
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_pending
  capabilities:
  - metamagic
  - action_economy
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_quickened_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: d5f11adec1150a8eaf680770bc3fbe828b7dc06092e23098697716c92eb7cf3d
  parent_name: Sorcerer
  line_span:
    first: 5673
    count: 9
splat_clauses:
- key: source_rules
  text: '# Quickened Spell Cost: 2 Sorcery Points When you cast a spell that has a casting time of an
    action, you can spend 2 Sorcery Points to change the casting time to a Bonus Action for this casting.
    You can’t modify a spell in this way if you’ve already cast a level 1+ spell on the current turn,
    nor can you cast a level 1+ spell on this turn after modifying a spell in this way.'
  sha256: 183a4386b8ddc0b2dce19bc067dc4457ec0b0fd83304cb0eb1b1428aba2e55d5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Quickened Spell

<!-- SRD page 67 -->

Cost: 2 Sorcery Points\
When you cast a spell that has a casting time of an\
action, you can spend 2 Sorcery Points to change\
the casting time to a Bonus Action for this casting.\
You can’t modify a spell in this way if you’ve already\
cast a level 1+ spell on the current turn, nor can you\
cast a level 1+ spell on this turn after modifying a\
spell in this way.\
