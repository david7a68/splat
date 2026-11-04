---
splat_schema: 2
splat_id: 9e7714b0-18eb-5ea7-a688-8839f21ef7ac
splat_kind: rule
splat_key: dnd2024:glossary/target
splat_name: Target
splat_origin: dnd2024:reference/glossary/target
splat_sources:
- SRD 5.2.1, pp. 190, https://www.dndbeyond.com/srd
---

# Target

## Source text

<!-- SRD page 190 -->

A target is the creature or object targeted by an attack roll, forced to make a saving throw by an effect, or selected to receive the effects of a spell or another phenomenon.

Original layout: [[../../srd-5.2.1/pages/190|page 190]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/target ` |
| ` implementation ` | ` dnd2024:glossary_target ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligible_kinds ` | ` ["creature", "object"] ` |
| ` targeted_when_any ` | ` ["attack_roll_target", "forced_save_target", "selected_spell_recipient", "selected_other_effect_recipient"] ` |
