---
splat_schema: 1
splat_id: 14c9bb02-561e-58c7-9632-ca9ced09ac95
splat_kind: spell
splat_key: dnd2024:spells/divination
splat_name: Divination
splat_origin:
  key: dnd2024:reference/spell/divination
splat_sources:
- title: SRD 5.2.1
  locator: pp. 125
  url: https://www.dndbeyond.com/srd
---

# Divination

## Source text

<!-- SRD page 125 -->

Level 4 Divination (Cleric, Druid, Wizard)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (incense worth 25+ GP, which the spell consumes)\
Duration: Instantaneous

This spell puts you in contact with a god or a god’s servants. You ask one question about a specific goal, event, or activity to occur within 7 days. The GM offers a truthful reply, which might be a short phrase or cryptic rhyme. The spell doesn’t account for circumstances that might change the answer, such as the casting of other spells.

If you cast the spell more than once before finishing a Long Rest, there is a cumulative 25 percent chance for each casting after the first that you get no answer.

Original layout: [[../../srd-5.2.1/pages/125|page 125]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 4 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["cleric", "druid", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action or Ritual ` |
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
| ` description ` | ` incense worth 25+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/divination ` |
| ` implementation ` | ` dnd2024:spell_divination ` |

### Parameters

#### Question

<!-- splat:values path=splat_rule/parameters/question -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 1 ` |
| ` subjects ` | ` ["specific_goal", "event", "activity"] ` |
| ` future_limit_days ` | ` 7 ` |

#### Reply

<!-- splat:values path=splat_rule/parameters/reply -->

| Field | Value |
| --- | --- |
| ` provider ` | ` gm ` |
| ` truthful ` | ` true ` |
| ` form_choices ` | ` ["short_phrase", "cryptic_rhyme"] ` |
| ` accounts_for_later_changed_circumstances ` | ` false ` |

#### Repeat cast

<!-- splat:values path=splat_rule/parameters/repeat_cast -->

| Field | Value |
| --- | --- |
| ` reset ` | ` caster_long_rest_completed ` |
| ` no_answer_chance_percent_per_cast_after_first ` | ` 25 ` |
| ` cumulative ` | ` true ` |
