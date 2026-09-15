---
splat_schema: 1
splat_id: 3e97a3ad-aafe-501f-bfa2-a24ce32f01c1
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_careful_spell
splat_name: Careful Spell
splat_origin:
  key: dnd2024:reference/pages/066
splat_sources:
- title: SRD 5.2.1
  locator: pp. 66, 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_careful_spell
  parameters:
    cost_sorcery_points: 1
    trigger: casting_spell_forcing_other_creatures_to_save
    selected_creatures_maximum:
      op: max
      args:
      - 1
      - variable: charisma_modifier
    selected_creatures: spell_save_targets
    selected_effects:
      saving_throw: automatic_success
      half_damage_on_save_becomes: 0
  implementation: dnd2024:rule_sorcerer_careful_spell
splat_contracts:
- id: dnd2024:rule_sorcerer_careful_spell
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    charisma_modifier:
      type: number
      required: true
  outputs:
  - spell_modifications
  events:
  - spell.cast_pending
  capabilities:
  - metamagic
  - saving_throws
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 66
  - 67
  engine_gaps: &id001
  - dnd2024:rule_sorcerer_careful_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: f1d956bb433ca6ddad9bb5d113b9f8bca59680946630fb0ba7a01ce104622deb
  parent_name: Sorcerer
  line_span:
    first: 5632
    count: 11
splat_clauses:
- key: source_rules
  text: '# Careful Spell Cost: 1 Sorcery Point When you cast a spell that forces other creatures to make
    a saving throw, you can protect some of those creatures from the spell’s full force. To do so, spend
    1 Sorcery Point and choose a number of those creatures up to your Charisma modifier (minimum of one
    creature). A chosen creature automatically succeeds on its saving throw against the spell, and it
    takes no damage if it would normally take half damage on a successful save.'
  sha256: e67487c10d1da8fbe208e694505563f87efd09628767d4dfe7f43b0597e6da37
  status: contract_specified
splat_requires_mechanics: *id001
---

# Careful Spell

<!-- SRD page 66 -->

Cost: 1 Sorcery Point\
When you cast a spell that forces other creatures\
to make a saving throw, you can protect some of\
those creatures from the spell’s full force. To do so,\
spend 1 Sorcery Point and choose a number of those\
creatures up to your Charisma modifier (minimum\
of one creature). A chosen creature automatically\
succeeds on its saving throw against the spell, and\

<!-- SRD page 67 -->

it takes no damage if it would normally take half\
damage on a successful save.\
