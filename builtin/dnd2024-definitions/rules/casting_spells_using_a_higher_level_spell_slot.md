---
splat_schema: 1
splat_id: d75517f9-42e5-5267-9737-8008c218e36c
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_using_a_higher_level_spell_slot
splat_name: Using a Higher-Level Spell Slot
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104, 105
  url: https://www.dndbeyond.com/srd
---

# Using a Higher-Level Spell Slot

## Source text

<!-- SRD page 104 -->

When a spellcaster casts a spell using a slot that is\
of a higher level than the spell, the spell takes on\
the higher level for that casting. For instance, if a\
Wizard casts Magic Missile using a level 2 slot, that\
Magic Missile is level 2. Effectively, the spell expands\
to fill the slot it is put into.\

<!-- SRD page 105 -->

Some spells, such as Magic Missile and Cure\
Wounds, have more powerful effects when cast at a\
higher level, as detailed in a spell’s description.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_using_a_higher_level_spell_slot ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_using_a_higher_level_spell_slot ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cast_level ` | ` expended_slot_level ` |
| ` upcast_effects ` | ` spell_specified_only ` |
