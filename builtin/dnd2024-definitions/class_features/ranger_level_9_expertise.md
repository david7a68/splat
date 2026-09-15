---
splat_schema: 1
splat_id: 396730dc-1242-5fec-a91f-c27d1d67b7c2
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_9_expertise
splat_name: 'Level 9: Expertise'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 9: Expertise

## Source text

<!-- SRD page 59 -->

Choose two of your skill proficiencies with which\
you lack Expertise. You gain Expertise in those\
skills.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_9_expertise ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_9_expertise ` |

### Parameters

#### Expertise

<!-- splat:yaml path=splat_rule/parameters/expertise -->

```yaml
count: 2
eligible:
  proficient: true
  expertise: false
grant: expertise
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` ranger ` |
| ` minimum_level ` | ` 9 ` |
