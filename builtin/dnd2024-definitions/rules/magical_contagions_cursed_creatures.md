---
splat_schema: 2
splat_id: a9b47655-7a15-5c66-8022-12a7e5f76f86
splat_kind: rule
splat_key: dnd2024:rules/magical_contagions_cursed_creatures
splat_name: Cursed Creatures
splat_origin: dnd2024:reference/pages/193
splat_sources:
- SRD 5.2.1, pp. 193, https://www.dndbeyond.com/srd
---

# Cursed Creatures

## Source text

<!-- SRD page 193 -->

Some monsters are associated with curses, whether as part of their origins or due to their ability to spread curses—werewolves being a prime example.

You decide how a spell like Remove Curse affects a creature with accursed origins. For example, you might decide that a mummy was created through a curse and it can be destroyed permanently only by casting Remove Curse on its corpse.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/magical_contagions_cursed_creatures ` |
| ` implementation ` | ` dnd2024:rule_magical_contagions_cursed_creatures ` |

### Parameters

#### Curse origin interaction

<!-- splat:values path=splat_rule/parameters/curse_origin_interaction -->

| Field | Value |
| --- | --- |
| ` actor ` | ` gm ` |
| ` scope ` | ` remove_curse_on_accursed_origin_creature ` |
| ` choice ` | ` source_specific_curse_removal_effect ` |

#### Optional mummy rule

<!-- splat:yaml path=splat_rule/parameters/optional_mummy_rule -->

```yaml
enabled_by: gm
permanent_destruction_requires:
  cast:
    key: dnd2024:spells/remove_curse
  target: corpse
```
