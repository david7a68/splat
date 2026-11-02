---
splat_schema: 1
splat_id: dd2dee6b-6886-5627-8f43-68108c51a0d0
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_3_evocation_savant
splat_name: 'Level 3: Evocation Savant'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
---

# Level 3: Evocation Savant

## Source text

<!-- SRD page 82 -->

Choose two Wizard spells from the Evocation school, each of which must be no higher than level 2, and add them to your spellbook for free.

In addition, whenever you gain access to a new level of spell slots in this class, you can add one Wizard spell from the Evocation school to your spellbook for free. The chosen spell must be of a level for which you have spell slots.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/evoker_level_3_evocation_savant ` |
| ` implementation ` | ` dnd2024:class_features_evoker_level_3_evocation_savant ` |

### Parameters

#### Initial spellbook additions

<!-- splat:values path=splat_rule/parameters/initial_spellbook_additions -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` spell_list ` | ` wizard ` |
| ` school ` | ` evocation ` |
| ` minimum_spell_level ` | ` 1 ` |
| ` maximum_spell_level ` | ` 2 ` |
| ` cost_gp ` | ` 0 ` |

#### Subsequent spellbook additions

<!-- splat:values path=splat_rule/parameters/subsequent_spellbook_additions -->

| Field | Value |
| --- | --- |
| ` event ` | ` wizard.new_spell_slot_level_access ` |
| ` count ` | ` 1 ` |
| ` spell_list ` | ` wizard ` |
| ` school ` | ` evocation ` |
| ` requires_available_slot_level ` | ` true ` |
| ` cost_gp ` | ` 0 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 3 ` |
