---
splat_schema: 1
splat_id: f7a92c05-ad3a-51ee-8ee8-a47c492ddaeb
splat_kind: rule
splat_key: dnd2024:feats/epic_boon_feats_boon_of_spell_recall
splat_name: Boon of Spell Recall
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
---

# Boon of Spell Recall

## Source text

<!-- SRD page 88 -->

Epic Boon Feat (Prerequisite: Level 19+, Spellcasting\
Feature)\
You gain the following benefits.\
Ability Score Increase. Increase your Intelligence,\
Wisdom, or Charisma score by 1, to a maximum of\
30.\
Free Casting. Whenever you cast a spell with a\
level 1–4 spell slot, roll 1d4. If the number you roll is\
the same as the slot’s level, the slot isn’t expended.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/epic_boon_feats_boon_of_spell_recall ` |
| ` implementation ` | ` dnd2024:feats_epic_boon_feats_boon_of_spell_recall ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` epic_boon ` |

#### Prerequisites

<!-- splat:values path=splat_rule/parameters/prerequisites -->

| Field | Value |
| --- | --- |
| ` minimum_character_level ` | ` 19 ` |
| ` feature ` | ` spellcasting ` |

#### Ability increase

<!-- splat:values path=splat_rule/parameters/ability_increase -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` amount ` | ` 1 ` |
| ` choices ` | ` ["intelligence", "wisdom", "charisma"] ` |
| ` maximum_score ` | ` 30 ` |

#### Free casting

<!-- splat:yaml path=splat_rule/parameters/free_casting -->

```yaml
trigger: actor.casts_spell_with_slot
slot_levels:
- 1
- 2
- 3
- 4
roll:
  count: 1
  sides: 4
if:
  roll_equals: slot.level
then:
  slot_expended: false
```
