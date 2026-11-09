---
splat_schema: 2
splat_id: 74a70df4-dd74-55d0-bd7f-1efc59873ae8
splat_kind: spell
splat_key: dnd2024:spells/dominate_person
splat_name: Dominate Person
splat_origin: dnd2024:reference/spell/dominate_person
splat_sources:
- SRD 5.2.1, pp. 126, https://www.dndbeyond.com/srd
---

# Dominate Person


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Enchantment |
| Classes | Bard, Sorcerer, Wizard |
| Casting Time | Action |
| Range | 60 feet |
| Components | V, S |
| Duration | Concentration, up to 1 minute |
| Ritual | No |
| Concentration | Yes |


## Source text

<!-- SRD page 126 -->

Level 5 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute

One Humanoid you can see within range must succeed on a Wisdom saving throw or have the Charmed condition for the duration. The target has Advantage on the save if you or your allies are fighting it. Whenever the target takes damage, it repeats the save, ending the spell on itself on a success.

You have a telepathic link with the Charmed target while the two of you are on the same plane of existence. On your turn, you can use this link to issue commands to the target (no action required), such as “Attack that creature,” “Move over there,” or “Fetch that object.” The target does its best to obey on its turn. If it completes an order and doesn’t receive further direction from you, it acts and moves as it likes, focusing on protecting itself.

You can command the target to take a Reaction but must take your own Reaction to do so.

Using a Higher-Level Spell Slot. Your Concentration can last longer with a spell slot of level 6 (up to 10 minutes), 7 (up to 1 hour), or 8+ (up to 8 hours).

Original layout: [[../../srd-5.2.1/pages/126|page 126]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/dominate_person ` |
| ` implementation ` | ` dnd2024:spell_dominate_person ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` initial_save_advantage_if ` | ` caster_or_allies_fighting_target ` |
| ` concentration ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` humanoid ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: wisdom
dc:
  binding: save_dc
failure:
  condition: charmed
success:
  effect: none
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` damage.applied ` |
| ` subject ` | ` target ` |
| ` ability ` | ` wisdom ` |
| ` success ` | ` end_spell_on_self ` |

#### Link

<!-- splat:values path=splat_rule/parameters/link -->

| Field | Value |
| --- | --- |
| ` telepathic ` | ` true ` |
| ` same_plane_required ` | ` true ` |
| ` command_on ` | ` caster_turn ` |
| ` command_action_cost ` | ` none ` |
| ` obey ` | ` best_effort_on_own_turn ` |
| ` after_completed_without_new_order ` | ` free_actions_focused_on_self_protection ` |
| ` reaction_command_cost ` | ` caster_reaction ` |

#### Duration by slot

<!-- splat:yaml path=splat_rule/parameters/duration_by_slot -->

```yaml
'5':
  minutes: 1
'6':
  minutes: 10
'7':
  hours: 1
'8':
  hours: 8
'9':
  hours: 8
```
