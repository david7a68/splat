---
splat_schema: 1
splat_id: 39a14f5c-4402-5aac-afc3-30d18aa67992
splat_kind: rule
splat_key: dnd2024:class_features/warlock_level_11_mystic_arcanum
splat_name: 'Level 11: Mystic Arcanum'
splat_origin:
  key: dnd2024:reference/pages/072
splat_sources:
- title: SRD 5.2.1
  locator: pp. 72
  url: https://www.dndbeyond.com/srd
---

# Level 11: Mystic Arcanum

## Source text

<!-- SRD page 72 -->

Your patron grants you a magical secret called an\
arcanum. Choose one level 6 Warlock spell as this\
arcanum.\
You can cast your arcanum spell once without ex-\
pending a spell slot, and you must finish a Long Rest\
before you can cast it in this way again.\
As shown in the Warlock Features table, you gain\
another Warlock spell of your choice that can be\
cast in this way when you reach Warlock levels\
13 (level 7 spell), 15 (level 8 spell), and 17 (level 9\
spell). You regain all uses of your Mystic Arcanum\
when you finish a Long Rest.\
Whenever you gain a Warlock level, you can re-\
place one of your arcanum spells with another War-\
lock spell of the same level.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/warlock_level_11_mystic_arcanum ` |
| ` implementation ` | ` dnd2024:class_features_warlock_level_11_mystic_arcanum ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` count_each ` | ` 1 ` |
| ` spell_list ` | ` warlock ` |

#### Arcanum choices

<!-- splat:rows path=splat_rule/parameters/arcanum_choices -->

| Row | warlock_level | spell_level |
| --- | --- | --- |
| ` 0 ` | ` 11 ` | ` 6 ` |
| ` 1 ` | ` 13 ` | ` 7 ` |
| ` 2 ` | ` 15 ` | ` 8 ` |
| ` 3 ` | ` 17 ` | ` 9 ` |

#### Each spell

<!-- splat:yaml path=splat_rule/parameters/each_spell -->

```yaml
spell_slot_cost: 0
uses:
  maximum: 1
  recover:
    event: rest.long_rest_completed
    amount: all
```

#### Replace

<!-- splat:values path=splat_rule/parameters/replace -->

| Field | Value |
| --- | --- |
| ` event ` | ` character.warlock_level_gained ` |
| ` maximum ` | ` 1 ` |
| ` same_spell_level ` | ` true ` |
| ` list ` | ` warlock ` |
