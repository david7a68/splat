---
splat_schema: 2
splat_id: 8578a387-b95e-5caf-a050-54fdf7e0c833
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_2_fighting_style
splat_name: 'Level 2: Fighting Style'
splat_origin: dnd2024:reference/pages/054
splat_sources:
- SRD 5.2.1, pp. 54, https://www.dndbeyond.com/srd
---

# Level 2: Fighting Style

## Source text

<!-- SRD page 54 -->

You gain a Fighting Style feat of your choice (see “Feats” for feats). Instead of choosing one of those feats, you can choose the option below.

Blessed Warrior. You learn two Cleric cantrips of your choice (see the Cleric class’s section for a list of Cleric spells). Guidance and Sacred Flame are recommended. The chosen cantrips count as Paladin spells for you, and Charisma is your spellcasting ability for them. Whenever you gain a Paladin level, you can replace one of these cantrips with another Cleric cantrip.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/paladin_level_2_fighting_style ` |
| ` implementation ` | ` dnd2024:class_features_paladin_level_2_fighting_style ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` choose ` | ` 1 ` |

#### Options

<!-- splat:yaml path=splat_rule/parameters/options -->

```yaml
fighting_style:
  feat_category: fighting_style
  count: 1
blessed_warrior:
  cantrips:
    count: 2
    list: cleric
  counts_as_class: paladin
  ability: charisma
  replace:
    event: paladin.level_gained
    maximum: 1
    list: cleric
    spell_level: 0
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` paladin ` |
| ` minimum_level ` | ` 2 ` |
