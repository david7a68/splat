---
splat_schema: 1
splat_id: 025908ba-2216-559f-b006-19bfcdd9f906
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_casting_without_slots
splat_name: Casting without Slots
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_casting_without_slots
  parameters:
    slot_free_methods:
      cantrip:
        level: 0
      ritual:
        requires_ritual_tag: true
        requires_prepared: true
        added_casting_minutes: 10
        other_casting_rules_apply: true
      special_ability:
        spells_and_usage_limits: feature_definition
      magic_item:
        spells_and_usage_limits: item_definition
    ritual_spell_may_use_normal_casting: true
  implementation: dnd2024:rule_casting_spells_casting_without_slots
splat_contracts:
- id: dnd2024:rule_casting_spells_casting_without_slots
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - spell_resolution
  events: []
  capabilities:
  - spellcasting
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rule_casting_spells_casting_without_slots
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: 7f837acc459cb6b4d0e121eae5c54297761fb1ad2b9cc00f77360efcad0c33b6
  parent_name: Casting Spells
  line_span:
    first: 9097
    count: 20
splat_clauses:
- key: source_rules
  text: '# Casting without Slots There are several ways to cast a spell without expending a spell slot:
    Cantrips. A cantrip is cast without a spell slot. Rituals. Certain spells have the Ritual tag in the
    Casting Time entry. Such a spell can be cast following the normal rules for spellcasting, or it can
    be cast as a Ritual. The Ritual version of a spell takes 10 minutes longer to cast than normal, but
    it doesn’t expend a spell slot. To cast a spell as a Ritual, a spellcaster must have it prepared.
    Special Abilities. Some characters and monsters have special abilities that allow them to cast specific
    spells without a spell slot. This casting is usually limited in another way, such as being able to
    cast the spell a limited number of times per day. Magic Items. Spell Scrolls and some other magic
    items contain spells that can be cast without a spell slot. The description of such an item specifies
    how many times a spell can be cast from it.'
  sha256: 02075735c1180c9cd55a32563a18964d0d5aa8ed6ac53ebe4d5f057ae07f51a0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Casting without Slots

<!-- SRD page 104 -->

There are several ways to cast a spell without\
expending a spell slot:\
Cantrips. A cantrip is cast without a spell slot.\
Rituals. Certain spells have the Ritual tag in the\
Casting Time entry. Such a spell can be cast fol-\
lowing the normal rules for spellcasting, or it can\
be cast as a Ritual. The Ritual version of a spell\
takes 10 minutes longer to cast than normal, but it\
doesn’t expend a spell slot. To cast a spell as a Rit-\
ual, a spellcaster must have it prepared.\
Special Abilities. Some characters and monsters\
have special abilities that allow them to cast spe-\
cific spells without a spell slot. This casting is usu-\
ally limited in another way, such as being able to\
cast the spell a limited number of times per day.\
Magic Items. Spell Scrolls and some other magic\
items contain spells that can be cast without a\
spell slot. The description of such an item speci-\
fies how many times a spell can be cast from it.\
