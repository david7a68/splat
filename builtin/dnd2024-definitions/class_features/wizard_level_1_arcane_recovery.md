---
splat_schema: 1
splat_id: d5b5dd29-2ac9-53a0-83a0-18df525a6d87
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_1_arcane_recovery
splat_name: 'Level 1: Arcane Recovery'
splat_origin:
  key: dnd2024:reference/pages/078
splat_sources:
- title: SRD 5.2.1
  locator: pp. 78
  url: https://www.dndbeyond.com/srd
---

# Level 1: Arcane Recovery

## Source text

<!-- SRD page 78 -->

You can regain some of your magical energy by\
studying your spellbook. When you finish a Short\
Rest, you can choose expended spell slots to re-\
cover. The spell slots can have a combined level\
equal to no more than half your Wizard level (round\
up), and none of the slots can be level 6 or higher.\
For example, if you’re a level 4 Wizard, you can re-\
cover up to two levels’ worth of spell slots, regain-\
ing either one level 2 spell slot or two level 1 spell\
slots.\
Once you use this feature, you can’t do so again\
until you finish a Long Rest.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/wizard_level_1_arcane_recovery ` |
| ` implementation ` | ` dnd2024:class_features_wizard_level_1_arcane_recovery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` requires_activity ` | ` study_own_spellbook_during_short_rest ` |

#### Recover choice

<!-- splat:yaml path=splat_rule/parameters/recover_choice -->

```yaml
resource: expended_spell_slots
maximum_slot_level: 5
maximum_combined_slot_levels:
  op: ceil
  args:
  - op: divide
    args:
    - stat: classes.wizard.level
    - 2
```

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
recover:
  event: rest.long_rest_completed
  amount: all
```
