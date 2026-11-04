---
splat_schema: 2
splat_id: e015a36b-6bda-52b0-81e1-60e659459416
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_2_scholar
splat_name: 'Level 2: Scholar'
splat_origin: dnd2024:reference/pages/078
splat_sources:
- SRD 5.2.1, pp. 78, https://www.dndbeyond.com/srd
---

# Level 2: Scholar

## Source text

<!-- SRD page 78 -->

While studying magic, you also specialized in another field of study. Choose one of the following skills in which you have proficiency: Arcana, History, Investigation, Medicine, Nature, or Religion. You have Expertise in the chosen skill.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_2_scholar ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_2_scholar ` |

### Parameters

#### Expertise

<!-- splat:yaml path=splat_rule/parameters/expertise -->

```yaml
count: 1
eligible:
  proficient: true
  expertise: false
  skills:
  - arcana
  - history
  - investigation
  - medicine
  - nature
  - religion
grant: expertise
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` wizard ` |
| ` minimum_level ` | ` 2 ` |
