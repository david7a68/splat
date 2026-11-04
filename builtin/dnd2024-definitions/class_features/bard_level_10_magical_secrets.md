---
splat_schema: 2
splat_id: e7c656be-c514-5e20-b917-3ba1d417f0f5
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_10_magical_secrets
splat_name: 'Level 10: Magical Secrets'
splat_origin: dnd2024:reference/pages/033
splat_sources:
- SRD 5.2.1, pp. 33, https://www.dndbeyond.com/srd
---

# Level 10: Magical Secrets

## Source text

<!-- SRD page 33 -->

You’ve learned secrets from various magical traditions. Whenever you reach a Bard level (including this level) and the Prepared Spells number in the Bard Features table increases, you can choose any of your new prepared spells from the Bard, Cleric, Druid, and Wizard spell lists, and the chosen spells count as Bard spells for you (see a class’s section for its spell list). In addition, whenever you replace a spell prepared for this class, you can replace it with a spell from those lists.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_10_magical_secrets ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_10_magical_secrets ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` spell_list_expansion ` | ` ["bard", "cleric", "druid", "wizard"] ` |
| ` counts_as_class ` | ` bard ` |
| ` other_preparation_constraints_unchanged ` | ` true ` |

#### Applies to

<!-- splat:yaml path=splat_rule/parameters/applies_to -->

```yaml
new_preparations:
  event: bard.level_gained
  minimum_level: 10
  prepared_count_increased: true
replacements: true
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 10 ` |
