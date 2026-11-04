---
splat_schema: 2
splat_id: f9b3f7dc-6ffe-5839-9d14-e8ffd8a220ea
splat_kind: rule
splat_key: dnd2024:class_features/college_of_lore_level_6_magical_discoveries
splat_name: 'Level 6: Magical Discoveries'
splat_origin: dnd2024:reference/pages/035
splat_sources:
- SRD 5.2.1, pp. 35, https://www.dndbeyond.com/srd
---

# Level 6: Magical Discoveries

## Source text

<!-- SRD page 35 -->

You learn two spells of your choice. These spells can come from the Cleric, Druid, or Wizard spell list or any combination thereof (see a class’s section for its spell list). A spell you choose must be a cantrip or a spell for which you have spell slots, as shown in the Bard Features table.

You always have the chosen spells prepared, and whenever you gain a Bard level, you can replace one of the spells with another spell that meets these requirements.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/college_of_lore_level_6_magical_discoveries ` |
| ` implementation ` | ` dnd2024:class_features_college_of_lore_level_6_magical_discoveries ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` always_prepared ` | ` true ` |
| ` counts_as_class_spell ` | ` bard ` |
| ` counts_toward_bard_prepared_limit ` | ` false ` |

#### Spell choice

<!-- splat:yaml path=splat_rule/parameters/spell_choice -->

```yaml
count: 2
lists:
- cleric
- druid
- wizard
mix_lists: true
allowed_level:
  cantrip: true
  otherwise_has_bard_progression_slot: true
```

#### Replace

<!-- splat:values path=splat_rule/parameters/replace -->

| Field | Value |
| --- | --- |
| ` event ` | ` character.bard_level_gained ` |
| ` maximum ` | ` 1 ` |
| ` same_eligibility ` | ` true ` |
