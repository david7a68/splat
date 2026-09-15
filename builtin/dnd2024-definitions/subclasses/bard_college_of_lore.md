---
splat_schema: 1
splat_id: 3a669937-792d-537d-bc50-56c3344e7a1f
splat_kind: rule_group
splat_key: dnd2024:subclasses/bard_college_of_lore
splat_name: College of Lore
splat_origin:
  key: dnd2024:reference/pages/035
splat_sources:
- title: SRD 5.2.1
  locator: pp. 35
  url: https://www.dndbeyond.com/srd
---

# College of Lore

## Source text

<!-- SRD page 35 -->

Plumb the Depths of Magical Knowledge\
Bards of the College of Lore collect spells and se-\
crets from diverse sources, such as scholarly tomes,\
mystical rites, and peasant tales. The college’s mem-\
bers gather in libraries and universities to share\
their lore with one another. They also meet at festi-\
vals or affairs of state, where they can expose cor-\
ruption, unravel lies, and poke fun at self-important\
figures of authority.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` subclass ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:subclasses/bard_college_of_lore ` |
| ` implementation ` | ` dnd2024:subclasses_bard_college_of_lore ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_class_level ` | ` 3 ` |

#### Feature grants

<!-- splat:table
path: splat_rule/parameters/feature_grants
key: Item
order:
- college_of_lore_level_14_peerless_skill
- college_of_lore_level_3_bonus_proficiencies
- college_of_lore_level_3_cutting_words
- college_of_lore_level_6_magical_discoveries
columns:
  Item:
    path: definition
    type: reference
  Class level:
    path: class_level
    type: integer
-->

| Item | Class level |
| --- | --- |
| [Level 14: Peerless Skill](<../class_features/college_of_lore_level_14_peerless_skill.md> "dnd2024:class_features/college_of_lore_level_14_peerless_skill") | ` 14 ` |
| [Level 3: Bonus Proficiencies](<../class_features/college_of_lore_level_3_bonus_proficiencies.md> "dnd2024:class_features/college_of_lore_level_3_bonus_proficiencies") | ` 3 ` |
| [Level 3: Cutting Words](<../class_features/college_of_lore_level_3_cutting_words.md> "dnd2024:class_features/college_of_lore_level_3_cutting_words") | ` 3 ` |
| [Level 6: Magical Discoveries](<../class_features/college_of_lore_level_6_magical_discoveries.md> "dnd2024:class_features/college_of_lore_level_6_magical_discoveries") | ` 6 ` |

#### Grant eligible features

<!-- splat:values path=splat_rule/parameters/grant_eligible_features -->

| Field | Value |
| --- | --- |
| ` "on" ` | ` ["subclass.selected", "character.level_gained"] ` |
| ` comparison ` | ` class_level_at_least_threshold ` |
| ` persistent ` | ` true ` |
