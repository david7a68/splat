---
splat_schema: 2
splat_id: 98dbb838-e5ce-5565-9713-e103b8a8d837
splat_kind: spell
splat_key: dnd2024:spells/geas
splat_name: Geas
splat_origin: dnd2024:reference/spell/geas
splat_sources:
- SRD 5.2.1, pp. 135, https://www.dndbeyond.com/srd
---

# Geas


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Enchantment |
| Classes | Bard, Cleric, Druid, Paladin, Wizard |
| Casting Time | 1 minute |
| Range | 60 feet |
| Components | V |
| Duration | 30 days |


## Source text

<!-- SRD page 135 -->

Level 5 Enchantment (Bard, Cleric, Druid, Paladin, Wizard)\
Casting Time: 1 minute\
Range: 60 feet\
Components: V\
Duration: 30 days

You give a verbal command to a creature that you can see within range, ordering it to carry out some service or refrain from an action or a course of activity as you decide. The target must succeed on a Wisdom saving throw or have the Charmed condition for the duration. The target automatically succeeds if it can’t understand your command.

While Charmed, the creature takes 5d10 Psychic damage if it acts in a manner directly counter to your command. It takes this damage no more than once each day.

You can issue any command you choose, short of an activity that would result in certain death. Should you issue a suicidal command, the spell ends. A Remove Curse, Greater Restoration, or Wish spell ends this spell.

Using a Higher-Level Spell Slot. If you use a level 7 or 8 spell slot, the duration is 365 days. If you use a level 9 spell slot, the spell lasts until it is ended by one of the spells mentioned above.

Original layout: [[../../srd-5.2.1/pages/135|page 135]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/geas ` |
| ` implementation ` | ` dnd2024:spell_geas ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` automatic_success_if ` | ` cannot_understand_command ` |
| ` end_by_spells ` | ` ["remove_curse", "greater_restoration", "wish"] ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` 1 ` |
| ` visible_to_caster ` | ` true ` |

#### Command

<!-- splat:values path=splat_rule/parameters/command -->

| Field | Value |
| --- | --- |
| ` verbal ` | ` true ` |
| ` choice ` | ` ["perform_service", "refrain_action_or_activity"] ` |
| ` certain_death_command ` | ` end_spell ` |

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

#### Violation

<!-- splat:yaml path=splat_rule/parameters/violation -->

```yaml
event: target.acts_directly_counter_to_command
damage:
  amount:
    dice:
      count: 5
      sides: 10
      add: 0
  type: psychic
maximum_per_day: 1
requires: charmed_by_spell
```

#### Duration by slot

<!-- splat:yaml path=splat_rule/parameters/duration_by_slot -->

```yaml
'5':
  days: 30
'6':
  days: 30
'7':
  days: 365
'8':
  days: 365
'9':
  until: named_ending_spell
```
