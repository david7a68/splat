---
splat_schema: 1
splat_id: 5f47ae82-dfc2-5688-9344-bdeb2186d47b
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_longer_casting_times
splat_name: Longer Casting Times
splat_origin:
  key: dnd2024:reference/pages/105
splat_sources:
- title: SRD 5.2.1
  locator: pp. 105
  url: https://www.dndbeyond.com/srd
---

# Longer Casting Times

## Source text

<!-- SRD page 105 -->

Certain spells—including a spell cast as a Rit-\
ual—require more time to cast: minutes or even\
hours. While you cast a spell with a casting time of\
1 minute or more, you must take the Magic action\
on each of your turns, and you must maintain Con-\
centration (see “Rules Glossary”) while you do so. If\
your Concentration is broken, the spell fails, but you\
don’t expend a spell slot. To cast the spell again, you\
must start over.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_longer_casting_times ` |
| ` implementation ` | ` dnd2024:rules_casting_spells_longer_casting_times ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` casting_time_minimum_seconds ` | ` 60 ` |
| ` each_turn_action ` | ` magic ` |
| ` requires_concentration_during_casting ` | ` true ` |
| ` includes_rituals ` | ` true ` |

#### Concentration broken

<!-- splat:values path=splat_rule/parameters/concentration_broken -->

| Field | Value |
| --- | --- |
| ` spell_fails ` | ` true ` |
| ` slot_expended ` | ` false ` |
| ` restart_required ` | ` true ` |
