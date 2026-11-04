---
splat_schema: 2
splat_id: a095ba0d-f74e-5f96-8cf6-b48a1f75092a
splat_kind: rule
splat_key: dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe
splat_name: Prerequisites for the Scribe
splat_origin: dnd2024:reference/pages/103
splat_sources:
- SRD 5.2.1, pp. 103, https://www.dndbeyond.com/srd
---

# Prerequisites for the Scribe

## Source text

<!-- SRD page 103 -->

To scribe a scroll, you must have proficiency in the Arcana skill or with Calligrapher’s Supplies and have the spell prepared on each day of the inscription. You must also have at hand any Material components required by the spell; if the spell consumes its Material components, they are consumed only when you complete the scroll. The scroll’s spell uses your spell save DC and spell attack bonus.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/scribing_spell_scrolls_prerequisites_for_the_scribe ` |
| ` implementation ` | ` dnd2024:rule_scribing_spell_scrolls_prerequisites_for_the_scribe ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` proficiency_any ` | ` ["arcana", "calligraphers_supplies"] ` |
| ` spell_prepared_each_work_day ` | ` true ` |
| ` material_components_available ` | ` true ` |
| ` consumed_components_consumption_timing ` | ` scroll_completion ` |
| ` scroll_save_dc ` | ` scribe_spell_save_dc ` |
| ` scroll_attack_bonus ` | ` scribe_spell_attack_bonus ` |
