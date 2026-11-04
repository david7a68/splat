---
splat_schema: 2
splat_id: be712871-bf75-5349-b321-b51143649e3d
splat_kind: rule
splat_key: dnd2024:glossary/magic_action
splat_name: Magic [Action]
splat_origin: dnd2024:reference/glossary/magic_action
splat_sources:
- SRD 5.2.1, pp. 185, https://www.dndbeyond.com/srd
---

# Magic [Action]

## Source text

<!-- SRD page 185 -->

When you take the Magic action, you cast a spell that has a casting time of an action or use a feature or magic item that requires a Magic action to be activated.

If you cast a spell that has a casting time of 1 minute or longer, you must take the Magic action on each turn of that casting, and you must maintain Concentration while you do so. If your Concentration is broken, the spell fails, but you don’t expend a spell slot. See also “Concentration.”

Original layout: [[../../srd-5.2.1/pages/185|page 185]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/magic_action ` |
| ` implementation ` | ` dnd2024:glossary_magic_action ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` cost ` | ` action ` |
| ` options ` | ` ["cast_action_casting_time_spell", "activate_magic_action_feature", "activate_magic_action_item"] ` |

#### Long cast

<!-- splat:yaml path=splat_rule/parameters/long_cast -->

```yaml
casting_time_minutes_min: 1
magic_action_each_casting_turn: true
concentration_required: true
concentration_loss:
  spell_fails: true
  slot_expended: false
```
