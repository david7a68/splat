---
splat_schema: 1
splat_id: 40d2509b-e855-5fb9-a1a6-26dd032d01a1
splat_kind: rule
splat_key: dnd2024:class_features/circle_of_the_land_level_6_natural_recovery
splat_name: 'Level 6: Natural Recovery'
splat_origin:
  key: dnd2024:reference/pages/046
splat_sources:
- title: SRD 5.2.1
  locator: pp. 46
  url: https://www.dndbeyond.com/srd
---

# Level 6: Natural Recovery

## Source text

<!-- SRD page 46 -->

You can cast one of the level 1+ spells that you have\
prepared from your Circle Spells feature without\
expending a spell slot, and you must finish a Long\
Rest before you do so again.\
In addition, when you finish a Short Rest, you can\
choose expended spell slots to recover. The spell\
slots can have a combined level that is equal to or\
less than half your Druid level (round up), and none\
of them can be level 6+. For example, if you’re a level\
6 Druid, you can recover up to three levels’ worth of\
spell slots. You can recover a level 3 spell slot, a level\
2 and a level 1 spell slot, or three level 1 spell slots.\
Once you recover spell slots with this feature, you\
can’t do so again until you finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/circle_of_the_land_level_6_natural_recovery ` |
| ` implementation ` | ` dnd2024:class_features_circle_of_the_land_level_6_natural_recovery ` |

### Parameters

#### Free cast

<!-- splat:yaml path=splat_rule/parameters/free_cast -->

```yaml
resource:
  id: land.natural_recovery_cast
  maximum: 1
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
spell_eligibility:
  prepared_by:
    key: dnd2024:class_features/circle_of_the_land_level_3_circle_of_the_land_spells
  minimum_spell_level: 1
expend_slot: false
```

#### Slot recovery

<!-- splat:yaml path=splat_rule/parameters/slot_recovery -->

```yaml
event: rest.short_completed
resource:
  id: land.natural_recovery_slots
  maximum: 1
  recovery:
  - event: rest.long_completed
    amount: all
cost: 1
choose_expended_slots: true
maximum_combined_levels:
  op: ceil
  args:
  - op: divide
    args:
    - var: actor.class_levels.druid
    - 2
maximum_slot_level: 5
```

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` druid ` |
| ` minimum_level ` | ` 6 ` |
