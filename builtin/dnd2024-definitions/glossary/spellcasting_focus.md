---
splat_schema: 1
splat_id: 6770be22-3278-5613-aa2c-eeeb4cd26a97
splat_kind: rule
splat_key: dnd2024:glossary/spellcasting_focus
splat_name: Spellcasting Focus
splat_origin:
  key: dnd2024:reference/glossary/spellcasting_focus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188
  url: https://www.dndbeyond.com/srd
---

# Spellcasting Focus

## Source text

<!-- SRD page 188 -->

A Spellcasting Focus is an object that certain crea-\
tures can use in place of a spell’s Material compo-\
nents if those materials aren’t consumed by the\
spell and don’t have a cost specified. Some classes\
allow its members to use certain types of Spellcast-\
ing Focuses. See also “Spells” (“Casting Spells”).\

Original layout: [[../../srd-5.2.1/pages/188|page 188]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/spellcasting_focus ` |
| ` implementation ` | ` dnd2024:glossary_spellcasting_focus ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` replaces ` | ` material_components ` |
| ` class_focus_permissions ` | ` class_description_defined ` |

#### Eligibility

<!-- splat:values path=splat_rule/parameters/eligibility -->

| Field | Value |
| --- | --- |
| ` creature_can_use_focus_type ` | ` true ` |
| ` material_consumed ` | ` false ` |
| ` material_cost_specified ` | ` false ` |
