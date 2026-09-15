---
splat_schema: 1
splat_id: 02dda062-ceba-5fb0-bdc9-b6c308f56dde
splat_kind: spell
splat_key: dnd2024:spells/dominate_beast
splat_name: Dominate Beast
splat_origin:
  key: dnd2024:reference/spell/dominate_beast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
---

# Dominate Beast

## Source text

<!-- SRD page 125 -->

Level 4 Enchantment (Druid, Ranger, Sorcerer)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
One Beast you can see within range must succeed\
on a Wisdom saving throw or have the Charmed\
condition for the duration. The target has Advan-\
tage on the save if you or your allies are fighting it.\
Whenever the target takes damage, it repeats the\
save, ending the spell on itself on a success.\
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
tion can last longer with a spell slot of level 5 (up to\
10 minutes), 6 (up to 1 hour), or 7+ (up to 8 hours).\

Original layout: [[../../srd-5.2.1/pages/125|page 125]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["druid", "ranger", "sorcerer"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 1 minute ` |
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
| ` mechanic ` | ` dnd2024:spells/dominate_beast ` |
| ` implementation ` | ` dnd2024:spell_dominate_beast ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` true ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |
| ` creature_type ` | ` beast ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: charmed
  duration: spell_duration
success:
  effect: none
```

#### Save advantage if

<!-- splat:values path=splat_rule/parameters/save_advantage_if -->

| Field | Value |
| --- | --- |
| ` caster_or_allies_fighting_target ` | ` true ` |

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` target_takes_damage ` |
| ` ability ` | ` wisdom ` |
| ` success ` | ` end_spell_on_target ` |

#### Control

<!-- splat:yaml path=splat_rule/parameters/control -->

```yaml
requires: target_charmed_by_spell
telepathy:
  requires_same_plane: true
command:
  event: caster_turn
  cost: none
  obey: best_effort_on_target_turn
completed_without_new_order:
  behavior: target_choice
  priority: self_protection
reaction_command:
  cost: caster_reaction
  target_cost: reaction
```

#### Duration by slot

<!-- splat:yaml path=splat_rule/parameters/duration_by_slot -->

```yaml
'4':
  minutes: 1
'5':
  minutes: 10
'6':
  hours: 1
7_plus:
  hours: 8
```
