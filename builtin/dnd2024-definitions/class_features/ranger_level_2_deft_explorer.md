---
splat_schema: 1
splat_id: 0949c6f4-683b-58b9-bcd3-d9ed5eb62ff7
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_2_deft_explorer
splat_name: 'Level 2: Deft Explorer'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 2: Deft Explorer

## Source text

<!-- SRD page 59 -->

Thanks to your travels, you gain the following benefits.

Expertise. Choose one of your skill proficiencies with which you lack Expertise. You gain Expertise in that skill.

Languages. You know two languages of your choice from the language tables in “Character Creation.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_2_deft_explorer ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_2_deft_explorer ` |

### Parameters

#### Expertise

<!-- splat:yaml path=splat_rule/parameters/expertise -->

```yaml
count: 1
eligible:
  proficient: true
  expertise: false
grant: expertise
```

#### Languages

<!-- splat:values path=splat_rule/parameters/languages -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` selection ` | ` actor_choice ` |
| ` eligible_tables ` | ` ["standard_languages", "rare_languages"] ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 2 ` |
