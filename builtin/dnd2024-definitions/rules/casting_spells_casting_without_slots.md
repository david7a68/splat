---
splat_schema: 1
splat_id: 025908ba-2216-559f-b006-19bfcdd9f906
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_casting_without_slots
splat_name: Casting without Slots
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
---

# Casting without Slots

## Source text

<!-- SRD page 104 -->

There are several ways to cast a spell without expending a spell slot:

Cantrips. A cantrip is cast without a spell slot.

Rituals. Certain spells have the Ritual tag in the Casting Time entry. Such a spell can be cast following the normal rules for spellcasting, or it can be cast as a Ritual. The Ritual version of a spell takes 10 minutes longer to cast than normal, but it doesn’t expend a spell slot. To cast a spell as a Ritual, a spellcaster must have it prepared.

Special Abilities. Some characters and monsters have special abilities that allow them to cast specific spells without a spell slot. This casting is usually limited in another way, such as being able to cast the spell a limited number of times per day.

Magic Items. Spell Scrolls and some other magic items contain spells that can be cast without a spell slot. The description of such an item specifies how many times a spell can be cast from it.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_casting_without_slots ` |
| ` implementation ` | ` dnd2024:rule_casting_spells_casting_without_slots ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` ritual_spell_may_use_normal_casting ` | ` true ` |

#### Slot free methods

<!-- splat:yaml path=splat_rule/parameters/slot_free_methods -->

```yaml
cantrip:
  level: 0
ritual:
  requires_ritual_tag: true
  requires_prepared: true
  added_casting_minutes: 10
  other_casting_rules_apply: true
special_ability:
  spells_and_usage_limits: feature_definition
magic_item:
  spells_and_usage_limits: item_definition
```
