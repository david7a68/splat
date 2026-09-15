---
splat_schema: 1
splat_id: f560dd00-d196-560f-8de0-5edf974c6b13
splat_kind: rule
splat_key: dnd2024:rules/sorcerer_empowered_spell
splat_name: Empowered Spell
splat_origin:
  key: dnd2024:reference/pages/067
splat_sources:
- title: SRD 5.2.1
  locator: pp. 67
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/sorcerer_empowered_spell
  parameters:
    trigger: spell_damage_roll
    cost:
      sorcery_points: 1
    reroll_dice:
      selection: caster_choice
      maximum:
        op: max
        args:
        - 1
        - var: subject.charisma_modifier
      use_new_results: true
    compatible_with_other_metamagic: true
  implementation: dnd2024:rules_sorcerer_empowered_spell
splat_contracts:
- id: dnd2024:rules_sorcerer_empowered_spell
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.damage_rolled
  capabilities:
  - metamagic
  - dice
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 67
  engine_gaps: &id001
  - dnd2024:rules_sorcerer_empowered_spell
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 0a3f98ad2699596f3eee3c799d7bccb318cd71391c92d78f955d78a4dc776012
  parent_name: Sorcerer
  line_span:
    first: 5650
    count: 9
splat_clauses:
- key: source_rules
  text: '# Empowered Spell Cost: 1 Sorcery Point When you roll damage for a spell, you can spend 1 Sorcery
    Point to reroll a number of the damage dice up to your Charisma modifier (minimum of one), and you
    must use the new rolls. You can use Empowered Spell even if you’ve already used a different Metamagic
    option during the casting of the spell.'
  sha256: 24741a0946b9dc32e5f515c02510d769567aec4e5c9f431bc34cd1d26e25c086
  status: contract_specified
splat_requires_mechanics: *id001
---

# Empowered Spell

<!-- SRD page 67 -->

Cost: 1 Sorcery Point\
When you roll damage for a spell, you can spend 1\
Sorcery Point to reroll a number of the damage dice\
up to your Charisma modifier (minimum of one),\
and you must use the new rolls.\
You can use Empowered Spell even if you’ve al-\
ready used a different Metamagic option during the\
casting of the spell.\
