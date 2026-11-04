---
splat_schema: 2
splat_id: afcaf519-b6e3-5f9b-b77d-e596032335a2
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_spells_cast_from_items
splat_name: Spells Cast from Items
splat_origin: dnd2024:reference/pages/206
splat_sources:
- SRD 5.2.1, pp. 206, https://www.dndbeyond.com/srd
---

# Spells Cast from Items

## Source text

<!-- SRD page 206 -->

Some magic items allow the user to cast a spell from the item. The spell is cast at the lowest possible spell and caster level, doesn’t expend any of the user’s spell slots, and requires no components unless the item’s description notes otherwise. The spell uses its normal casting time, range, and duration, and the user of the item must concentrate if the spell requires Concentration. Many items, such as Potions, bypass the casting of a spell and confer the spell’s effects with its usual duration. Certain items make exceptions to these rules, changing the casting time, duration, or other parts of a spell.

A magic item may require the user to use their own spellcasting ability when casting a spell from the item. If the user has more than one spellcasting ability, the user chooses which one to use with the item. If the user doesn’t have a spellcasting ability, their spellcasting ability modifier is +0 for the item, and the user’s Proficiency Bonus applies.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
| ` implementation ` | ` dnd2024:rule_activating_a_magic_item_spells_cast_from_items ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` item_specific_overrides ` | ` true ` |

#### Spell cast defaults

<!-- splat:values path=splat_rule/parameters/spell_cast_defaults -->

| Field | Value |
| --- | --- |
| ` spell_level ` | ` lowest_possible ` |
| ` caster_level ` | ` lowest_possible ` |
| ` user_spell_slots_expended ` | ` 0 ` |
| ` components_required ` | ` false ` |
| ` casting_time ` | ` normal_spell ` |
| ` range ` | ` normal_spell ` |
| ` duration ` | ` normal_spell ` |
| ` concentration ` | ` user_if_spell_requires ` |

#### Confer effect without casting

<!-- splat:values path=splat_rule/parameters/confer_effect_without_casting -->

| Field | Value |
| --- | --- |
| ` duration ` | ` normal_spell ` |
| ` cast_event ` | ` false ` |

#### User spellcasting ability if required

<!-- splat:values path=splat_rule/parameters/user_spellcasting_ability_if_required -->

| Field | Value |
| --- | --- |
| ` multiple ` | ` user_choice ` |
| ` none_modifier ` | ` 0 ` |
| ` add_user_proficiency_bonus ` | ` true ` |
