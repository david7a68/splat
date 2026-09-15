---
splat_schema: 1
splat_id: 8e4dbe10-3c5c-5109-8de2-9f59553a8b4a
splat_kind: spell
splat_key: dnd2024:spells/dominate_monster
splat_name: Dominate Monster
splat_origin:
  key: dnd2024:reference/spell/dominate_monster
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125, 126
  url: https://www.dndbeyond.com/srd
---

# Dominate Monster

## Source text

<!-- SRD page 125 -->

Level 8 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 hour\
One creature you can see within range must\
succeed on a Wisdom saving throw or have the\
Charmed condition for the duration. The target has\
Advantage on the save if you or your allies are fight-\
ing it. Whenever the target takes damage, it repeats\
the save, ending the spell on itself on a success.\

<!-- SRD page 126 -->

You have a telepathic link with the Charmed tar-\
get while the two of you are on the same plane of\
existence. On your turn, you can use this link to\
issue commands to the target (no action required),\
such as “Attack that creature,” “Move over there,” or\
“Fetch that object.” The target does its best to obey\
on its turn. If it completes an order and doesn’t re-\
ceive further direction from you, it acts and moves\
as it likes, focusing on protecting itself.\
You can command the target to take a Reaction\
but must take your own Reaction to do so.\
Using a Higher-Level Spell Slot. Your Concentra-\
tion can last longer with a level 9 spell slot (up to 8\
hours).\

Original layout: [[../../srd-5.2.1/pages/125|page 125]], [[../../srd-5.2.1/pages/126|page 126]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 8 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 hour ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dominate_monster ` |
| ` implementation ` | ` dnd2024:spell_dominate_monster ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |

#### Initial save

<!-- splat:values path=splat_rule/parameters/initial_save -->

| Field | Value |
| --- | --- |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` advantage_if ` | ` caster_or_allies_fighting_target ` |
| ` success ` | ` unaffected ` |

#### Failure

<!-- splat:values path=splat_rule/parameters/failure -->

| Field | Value |
| --- | --- |
| ` condition ` | ` charmed ` |
| ` source ` | ` caster ` |

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.applied ` |
| ` subject ` | ` target ` |
| ` ability ` | ` wisdom ` |
| ` dc ` | ` spell_save_dc ` |
| ` success ` | ` end_on_target ` |
| ` inherits_initial_save_advantage_rule ` | ` true ` |

#### Telepathic link

<!-- splat:values path=splat_rule/parameters/telepathic_link -->

| Field | Value |
| --- | --- |
| ` requires ` | ` same_plane ` |
| ` while ` | ` charmed_by_spell ` |

#### Commands

<!-- splat:yaml path=splat_rule/parameters/commands -->

```yaml
economy: none
timing: caster_turn
medium: telepathic_link
compliance: best_effort_on_target_turn
after_completion_without_new_order:
  autonomous: true
  focus: self_protection
```

#### Command reaction

<!-- splat:values path=splat_rule/parameters/command_reaction -->

| Field | Value |
| --- | --- |
| ` caster_economy ` | ` reaction ` |
| ` target_economy ` | ` reaction ` |

#### Slot duration

<!-- splat:rows path=splat_rule/parameters/slot_duration -->

| Row | levels | seconds |
| --- | --- | --- |
| ` 0 ` | ` [8] ` | ` 3600 ` |
| ` 1 ` | ` [9] ` | ` 28800 ` |
