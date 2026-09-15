---
splat_schema: 1
splat_id: dafa5e63-f21b-5e6c-b07f-be909cd51747
splat_kind: spell
splat_key: dnd2024:spells/augury
splat_name: Augury
splat_origin:
  key: dnd2024:reference/spell/augury
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111
  url: https://www.dndbeyond.com/srd
---

# Augury

## Source text

<!-- SRD page 111 -->

Level 2 Divination (Cleric, Druid, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (specially marked sticks, bones,\
cards, or other divinatory tokens worth 25+ GP)\
Duration: Instantaneous\
You receive an omen from an otherworldly entity\
about the results of a course of action that you plan\
to take within the next 30 minutes. The GM chooses\
the omen from the Omens table.\
Omens\
Omen For Results That Will Be …\
Weal Good\
Woe Bad\
Weal and woe Good and bad\
Indifference Neither good nor bad\
The spell doesn’t account for circumstances, such as\
other spells, that might change the results.\
If you cast the spell more than once before finish-\
ing a Long Rest, there is a cumulative 25 percent\
chance for each casting after the first that you get\
no answer.\

Original layout: [[../../srd-5.2.1/pages/111|page 111]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["cleric", "druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 1 minute or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` specially marked sticks, bones, cards, or other divinatory tokens worth 25+ GP ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/augury ` |
| ` implementation ` | ` dnd2024:spell_augury ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Inquiry

<!-- splat:values path=splat_rule/parameters/inquiry -->

| Field | Value |
| --- | --- |
| ` kind ` | ` planned_course_of_action ` |
| ` horizon_minutes ` | ` 30 ` |

#### Omen

<!-- splat:yaml path=splat_rule/parameters/omen -->

```yaml
chooser: gm
mapping:
  good: weal
  bad: woe
  good_and_bad: weal_and_woe
  neither: indifference
account_for_future_changed_circumstances: false
```

#### Repetition

<!-- splat:values path=splat_rule/parameters/repetition -->

| Field | Value |
| --- | --- |
| ` counter ` | ` casts_since_long_rest ` |
| ` reset_event ` | ` rest.long_completed ` |
| ` first_cast_no_answer_percent ` | ` 0 ` |
| ` extra_cast_increment_percent ` | ` 25 ` |
| ` maximum_percent ` | ` 100 ` |
| ` failure_output ` | ` no_answer ` |
