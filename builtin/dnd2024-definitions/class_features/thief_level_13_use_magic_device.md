---
splat_schema: 1
splat_id: 42c8436d-7c90-5f4a-9173-e1661a79b358
splat_kind: rule
splat_key: dnd2024:class_features/thief_level_13_use_magic_device
splat_name: 'Level 13: Use Magic Device'
splat_origin:
  key: dnd2024:reference/pages/064
splat_sources:
- title: SRD 5.2.1
  locator: pp. 64
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/thief_level_13_use_magic_device
  parameters:
    maximum_attuned_items: 4
    charged_item_use:
      roll:
        dice:
          count: 1
          sides: 6
      result_6:
        use_property: true
        expend_charges: false
    spell_scrolls:
      allowed_spell_lists: any
      spellcasting_ability: intelligence
      automatic_cast_maximum_spell_level: 1
      higher_levels_check:
        ability: intelligence
        skill: arcana
        dc:
          op: add
          args:
          - 10
          - stat: scroll.spell_level
        success: cast_scroll_spell
        failure: scroll_disintegrates
  implementation: dnd2024:class_features_thief_level_13_use_magic_device
splat_contracts:
- id: dnd2024:class_features_thief_level_13_use_magic_device
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - spell_grants
  events:
  - item.charges_spending_pending
  - scroll.cast_pending
  capabilities:
  - magic_items
  - attunement
  - ability_checks
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 64
  engine_gaps: &id001
  - dnd2024:class_features_thief_level_13_use_magic_device
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: bb40891f4a81dd86cb489785e2b072e97a290216f550187656fb0b03fd8dc96a
  parent_name: Thief
  line_span:
    first: 5379
    count: 17
splat_clauses:
- key: source_rules
  text: '# Level 13: Use Magic Device You’ve learned how to maximize use of magic items, granting you
    the following benefits. Attunement. You can attune to up to four magic items at once. Charges. Whenever
    you use a magic item property that expends charges, roll 1d6. On a roll of 6, you use the property
    without expending the charges. Scrolls. You can use any Spell Scroll, using Intelligence as your spellcasting
    ability for the spell. If the spell is a cantrip or a level 1 spell, you can cast it reliably. If
    the scroll contains a higher-level spell, you must first succeed on an Intelligence (Arcana) check
    (DC 10 plus the spell’s level). On a successful check, you cast the spell from the scroll. On a failed
    check, the scroll disintegrates.'
  sha256: 5bbbf670881ce2b5dee9b165062847f0f1efd28b2aea1154317e1bd70676545d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 13: Use Magic Device

<!-- SRD page 64 -->

You’ve learned how to maximize use of magic items,\
granting you the following benefits.\
Attunement. You can attune to up to four magic\
items at once.\
Charges. Whenever you use a magic item prop-\
erty that expends charges, roll 1d6. On a roll of\
6, you use the property without expending the\
charges.\
Scrolls. You can use any Spell Scroll, using Intelli-\
gence as your spellcasting ability for the spell. If the\
spell is a cantrip or a level 1 spell, you can cast it re-\
liably. If the scroll contains a higher-level spell, you\
must first succeed on an Intelligence (Arcana) check\
(DC 10 plus the spell’s level). On a successful check,\
you cast the spell from the scroll. On a failed check,\
the scroll disintegrates.\
