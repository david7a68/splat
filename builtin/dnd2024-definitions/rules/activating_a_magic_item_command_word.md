---
splat_schema: 2
splat_id: b0d4b9dc-d37a-53af-ab6c-490900516ab4
splat_kind: rule
splat_key: dnd2024:rules/activating_a_magic_item_command_word
splat_name: Command Word
splat_origin: dnd2024:reference/pages/206
splat_sources:
- SRD 5.2.1, pp. 206, https://www.dndbeyond.com/srd
---

# Command Word

## Source text

<!-- SRD page 206 -->

A command word is a word or short phrase that must be spoken or signed for an item to work. Spoken command words must be audible and fail to work in areas where all sound is suppressed, as in the area of the Silence spell.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/activating_a_magic_item_command_word ` |
| ` implementation ` | ` dnd2024:rules_activating_a_magic_item_command_word ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` command_word_modes ` | ` ["spoken", "signed"] ` |

#### Spoken

<!-- splat:values path=splat_rule/parameters/spoken -->

| Field | Value |
| --- | --- |
| ` audible_required ` | ` true ` |
| ` sound_suppression_prevents_activation ` | ` true ` |
