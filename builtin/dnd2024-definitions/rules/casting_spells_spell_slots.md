---
splat_schema: 2
splat_id: fd9a24ad-a71d-5d25-9333-8fadfd706869
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_spell_slots
splat_name: Spell Slots
splat_origin: dnd2024:reference/pages/104
splat_sources:
- SRD 5.2.1, pp. 104, https://www.dndbeyond.com/srd
---

# Spell Slots

## Source text

<!-- SRD page 104 -->

Spellcasting is taxing, so a spellcaster can cast only a limited number of level 1+ spells before resting. Spell slots are the main way a spellcaster’s magical potential is represented. Each spellcasting class gives its members a limited number of spell slots of certain spell levels. For example, a level 3 Wizard has four level 1 spell slots and two level 2 slots.

When you cast a spell, you expend a slot of that spell’s level or higher, effectively “filling” a slot with the spell. Imagine a spell slot is a groove of a certain size—small for a level 1 slot and larger for a higher-level spell. A level 1 spell fits into a slot of any size, but a level 2 spell fits only into a slot that’s at least level 2. So when a level 3 Wizard casts Magic Missile, a level 1 spell, that Wizard spends one of four level 1 slots and has three remaining.

Finishing a Long Rest restores any expended spell slots.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_spell_slots ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_spell_slots ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` slot_level_min ` | ` spell_level ` |
| ` slots_expended_per_cast ` | ` 1 ` |
| ` slot_capacity_source ` | ` class_spellcasting_table ` |
| ` restore_all_expended_slots ` | ` long_rest_finished ` |
| ` specific_feature_exceptions ` | ` true ` |

#### Spell slot required default for spell levels

<!-- splat:values path=splat_rule/parameters/spell_slot_required_default_for_spell_levels -->

| Field | Value |
| --- | --- |
| ` min ` | ` 1 ` |
| ` max ` | ` 9 ` |
