---
splat_schema: 1
splat_id: 523247e2-233b-5d10-ab0e-df2df3b7968f
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_combining_spell_effects
splat_name: Combining Spell Effects
splat_origin:
  key: dnd2024:reference/pages/106
splat_sources:
- title: SRD 5.2.1
  locator: pp. 106
  url: https://www.dndbeyond.com/srd
---

# Combining Spell Effects

## Source text

<!-- SRD page 106 -->

The effects of different spells add together while\
their durations overlap. In contrast, the effects of\
the same spell cast multiple times don’t combine.\
Instead, the most potent effect—such as the high-\
est bonus—from those castings applies while their\
durations overlap. The most recent effect applies if\
the castings are equally potent and their durations\
overlap. For example, if two Clerics cast Bless on\
the same target, that target gains the spell’s benefit\
only once; the target doesn’t receive two bonus dice.\
But if the durations of the spells overlap, the effect\
continues until the duration of the second Bless\
ends.\
idEntiFyinG an onGoinG spEll\
You can try to identify a non-instantaneous spell by\
its observable effects if its duration is ongoing. To\
identify it, you must take the Study action and suc-\
ceed on a DC 15 Intelligence (Arcana) check.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/casting_spells_combining_spell_effects ` |
| ` implementation ` | ` dnd2024:rule_casting_spells_combining_spell_effects ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` different_spell_effects ` | ` combine ` |

#### Same spell effects

<!-- splat:values path=splat_rule/parameters/same_spell_effects -->

| Field | Value |
| --- | --- |
| ` stack ` | ` false ` |
| ` selection ` | ` ["most_potent", "most_recent_if_equal"] ` |
| ` scope ` | ` overlapping_durations ` |
| ` independent_durations_continue ` | ` true ` |

#### Identify ongoing spell

<!-- splat:values path=splat_rule/parameters/identify_ongoing_spell -->

| Field | Value |
| --- | --- |
| ` requires ` | ` ["noninstantaneous", "ongoing_duration", "observable_effects"] ` |
| ` action ` | ` study ` |
| ` ability ` | ` intelligence ` |
| ` skill ` | ` arcana ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` identify_spell ` |
