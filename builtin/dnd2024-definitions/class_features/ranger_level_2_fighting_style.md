---
splat_schema: 1
splat_id: 04c58699-471e-574d-a658-a0587f19a46e
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_2_fighting_style
splat_name: 'Level 2: Fighting Style'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
---

# Level 2: Fighting Style

## Source text

<!-- SRD page 59 -->

You gain a Fighting Style feat of your choice (see\
“Feats”). Instead of choosing one of those feats, you\
can choose the option below.\
Druidic Warrior. You learn two Druid cantrips of\
your choice (see the Druid class’s section for a list of\
Druid spells). Guidance and Starry Wisp are recom-\
mended. The chosen cantrips count as Ranger spells\
for you, and Wisdom is your spellcasting ability for\
them. Whenever you gain a Ranger level, you can\
replace one of these cantrips with another Druid\
cantrip.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/ranger_level_2_fighting_style ` |
| ` implementation ` | ` dnd2024:class_features_ranger_level_2_fighting_style ` |

### Parameters

#### Choice

<!-- splat:yaml path=splat_rule/parameters/choice -->

```yaml
count: 1
options:
- id: fighting_style
  feat_choice:
    category: fighting_style
    count: 1
- id: druidic_warrior
  cantrip_choice:
    list: druid
    count: 2
  count_as_class_spells: ranger
  ability: wisdom
  replace:
    event: character.ranger_level_gained
    maximum: 1
    list: druid
    spell_level: 0
```
