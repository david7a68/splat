---
splat_schema: 1
splat_id: 24cfa087-f9ed-5f95-997f-36352318e52c
splat_kind: rule
splat_key: dnd2024:rules/magic_items_attune_during_a_short_rest
splat_name: Attune during a Short Rest
splat_origin:
  key: dnd2024:reference/pages/102
splat_sources:
- title: SRD 5.2.1
  locator: pp. 102
  url: https://www.dndbeyond.com/srd
---

# Attune during a Short Rest

## Source text

<!-- SRD page 102 -->

Attuning to an item requires you to spend a Short Rest focused on only that item while being in physical contact with it (this can’t be the same Short Rest used to learn the item’s properties). This focus can take the form of weapon practice (for a Weapon), meditation (for a Wand), or some other appropriate activity. If the Short Rest is interrupted, the Attunement attempt fails. Otherwise, at the end of the Short Rest, you’re attuned to the magic item and can access its full magical capabilities.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magic_items_attune_during_a_short_rest ` |
| ` implementation ` | ` dnd2024:rules_magic_items_attune_during_a_short_rest ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` activity ` | ` short_rest ` |
| ` focus_items_max ` | ` 1 ` |
| ` requires_physical_contact ` | ` true ` |
| ` cannot_simultaneously_identify_properties ` | ` true ` |
| ` interrupted_rest ` | ` attunement_fails ` |

#### Completed rest

<!-- splat:values path=splat_rule/parameters/completed_rest -->

| Field | Value |
| --- | --- |
| ` attuned ` | ` true ` |
| ` full_magic_access ` | ` true ` |
