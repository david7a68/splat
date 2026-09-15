---
splat_schema: 1
splat_id: afcaf519-b6e3-5f9b-b77d-e596032335a2
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_name: Spells Cast from Items
splat_origin:
  key: dnd2024:reference/pages/206
splat_sources:
- title: SRD 5.2.1
  locator: pp. 206
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
  parameters:
    spell_cast_defaults:
      spell_level: lowest_possible
      caster_level: lowest_possible
      user_spell_slots_expended: 0
      components_required: false
      casting_time: normal_spell
      range: normal_spell
      duration: normal_spell
      concentration: user_if_spell_requires
    item_specific_overrides: true
    confer_effect_without_casting:
      duration: normal_spell
      cast_event: false
    user_spellcasting_ability_if_required:
      multiple: user_choice
      none_modifier: 0
      add_user_proficiency_bonus: true
  implementation: dnd2024:rule_activating_a_magic_item_spells_cast_from_items
splat_contracts:
- id: dnd2024:rule_activating_a_magic_item_spells_cast_from_items
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - item_changes
  events: []
  capabilities:
  - magic_items
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 206
  engine_gaps: &id001
  - dnd2024:rule_activating_a_magic_item_spells_cast_from_items
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 862ac87175935fc526b57021bb00e77c86b871472aa8678da66202cecefd09af
  parent_name: Activating a Magic Item
  line_span:
    first: 10610
    count: 20
splat_clauses:
- key: source_rules
  text: '# Spells Cast from Items Some magic items allow the user to cast a spell from the item. The spell
    is cast at the lowest possible spell and caster level, doesn’t expend any of the user’s spell slots,
    and requires no components unless the item’s description notes otherwise. The spell uses its normal
    casting time, range, and duration, and the user of the item must concentrate if the spell requires
    Concentration. Many items, such as Potions, bypass the casting of a spell and confer the spell’s effects
    with its usual duration. Certain items make exceptions to these rules, changing the casting time,
    duration, or other parts of a spell. A magic item may require the user to use their own spellcasting
    ability when casting a spell from the item. If the user has more than one spellcasting ability, the
    user chooses which one to use with the item. If the user doesn’t have a spellcasting ability, their
    spellcasting ability modifier is +0 for the item, and the user’s Proficiency Bonus applies.'
  sha256: 38cb21e7f4248dd43f184932cfa7ac3ff5657e5897de6aeade7d80bc8bb355af
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spells Cast from Items

<!-- SRD page 206 -->

Some magic items allow the user to cast a spell from\
the item. The spell is cast at the lowest possible\
spell and caster level, doesn’t expend any of the us-\
er’s spell slots, and requires no components unless\
the item’s description notes otherwise. The spell\
uses its normal casting time, range, and duration,\
and the user of the item must concentrate if the\
spell requires Concentration. Many items, such as\
Potions, bypass the casting of a spell and confer the\
spell’s effects with its usual duration. Certain items\
make exceptions to these rules, changing the cast-\
ing time, duration, or other parts of a spell.\
A magic item may require the user to use their\
own spellcasting ability when casting a spell from\
the item. If the user has more than one spellcasting\
ability, the user chooses which one to use with the\
item. If the user doesn’t have a spellcasting ability,\
their spellcasting ability modifier is +0 for the item,\
and the user’s Proficiency Bonus applies.\
