---
splat_schema: 1
splat_id: be712871-bf75-5349-b321-b51143649e3d
splat_kind: rule
splat_key: dnd2024:glossary/magic_action
splat_name: Magic [Action]
splat_origin:
  key: dnd2024:reference/glossary/magic_action
splat_sources:
- title: SRD 5.2.1
  locator: pp. 185
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/magic_action
  parameters:
    cost: action
    options:
    - cast_action_casting_time_spell
    - activate_magic_action_feature
    - activate_magic_action_item
    long_cast:
      casting_time_minutes_min: 1
      magic_action_each_casting_turn: true
      concentration_required: true
      concentration_loss:
        spell_fails: true
        slot_expended: false
  implementation: dnd2024:glossary_magic_action
splat_contracts:
- id: dnd2024:glossary_magic_action
  inputs:
    subject:
      type: participant
      required: true
    magic_source:
      type: definition
      required: true
  outputs:
  - magic_activation
  - casting_progress
  - slot_cost
  events:
  - action.requested
  - spell.cast_started
  - concentration.ended
  capabilities:
  - action_economy
  - spellcasting
  - concentration
  - equipment
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 4d815526c46006392d98d88a5f3acf25aaf2816ff4511c63891b17fd890233f8
  source_path: builtin/srd-5.2.1/glossary/magic_action.md
  pages:
  - 185
  engine_gaps: &id001
  - dnd2024:glossary_magic_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: 05c2b0acbb3384e26eefd357a820f879b332109fe785a03598dec494685f02ad
splat_clauses:
- key: source_rules
  text: '# Magic [Action] When you take the Magic action, you cast a spell that has a casting time of
    an action or use a feature or magic item that requires a Magic action to be activated. If you cast
    a spell that has a casting time of 1 minute or longer, you must take the Magic action on each turn
    of that casting, and you must maintain Concentration while you do so. If your Concentration is broken,
    the spell fails, but you don’t expend a spell slot. See also “Concentration.”'
  sha256: 285127dcdb44b296a5242cb811ffe25fe0292ea156019916843ae43fe366684f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Magic [Action]

<!-- SRD page 185 -->

When you take the Magic action, you cast a spell\
that has a casting time of an action or use a feature\
or magic item that requires a Magic action to be\
activated.\
If you cast a spell that has a casting time of 1 min-\
ute or longer, you must take the Magic action on\
each turn of that casting, and you must maintain\
Concentration while you do so. If your Concentra-\
tion is broken, the spell fails, but you don’t expend a\
spell slot. See also “Concentration.”\

Original layout: [[../../srd-5.2.1/pages/185|page 185]].
