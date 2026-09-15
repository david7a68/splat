---
splat_schema: 1
splat_id: 5b970f1a-1808-5912-bea8-762c21675dee
splat_kind: rule
splat_key: dnd2024:feats/origin_feats_magic_initiate
splat_name: Magic Initiate
splat_origin:
  key: dnd2024:reference/pages/087
splat_sources:
- title: SRD 5.2.1
  locator: pp. 87
  url: https://www.dndbeyond.com/srd
---

# Magic Initiate

## Source text

<!-- SRD page 87 -->

Origin Feat\
You gain the following benefits.\
Two Cantrips. You learn two cantrips of your\
choice from the Cleric, Druid, or Wizard spell list.\
Intelligence, Wisdom, or Charisma is your spellcast-\
ing ability for this feat’s spells (choose when you\
select this feat).\
Level 1 Spell. Choose a level 1 spell from the same\
list you selected for this feat’s cantrips. You always\
have that spell prepared. You can cast it once with-\
out a spell slot, and you regain the ability to cast it\
in that way when you finish a Long Rest. You can\
also cast the spell using any spell slots you have.\
Spell Change. Whenever you gain a new level, you\
can replace one of the spells you chose for this feat\
with a different spell of the same level from the cho-\
sen spell list.\
Repeatable. You can take this feat more than\
once, but you must choose a different spell list each\
time.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:feats/origin_feats_magic_initiate ` |
| ` implementation ` | ` dnd2024:feats_origin_feats_magic_initiate ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` feat_category ` | ` origin ` |
| ` repeatable ` | ` true ` |
| ` repeat_requirement ` | ` different_spell_list_each_selection ` |

#### Spell list choice

<!-- splat:values path=splat_rule/parameters/spell_list_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["cleric", "druid", "wizard"] ` |

#### Spellcasting ability choice

<!-- splat:values path=splat_rule/parameters/spellcasting_ability_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["intelligence", "wisdom", "charisma"] ` |

#### Cantrip choice

<!-- splat:values path=splat_rule/parameters/cantrip_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 2 ` |
| ` spell_level ` | ` 0 ` |
| ` list ` | ` chosen_spell_list ` |

#### Level 1 spell choice

<!-- splat:yaml path=splat_rule/parameters/level_1_spell_choice -->

```yaml
count: 1
spell_level: 1
list: chosen_spell_list
always_prepared: true
free_cast:
  spell_slot_cost: 0
  uses:
    maximum: 1
    recover:
      event: rest.long_rest_completed
      amount: all
can_cast_with_spell_slots: true
```

#### Replace spell

<!-- splat:values path=splat_rule/parameters/replace_spell -->

| Field | Value |
| --- | --- |
| ` event ` | ` character.level_gained ` |
| ` maximum ` | ` 1 ` |
| ` same_spell_level ` | ` true ` |
| ` list ` | ` chosen_spell_list ` |
