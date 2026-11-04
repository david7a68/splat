---
splat_schema: 2
splat_id: dafa5e63-f21b-5e6c-b07f-be909cd51747
splat_kind: spell
splat_key: dnd2024:spells/augury
splat_name: Augury
splat_origin: dnd2024:reference/spell/augury
splat_sources:
- SRD 5.2.1, pp. 111, https://www.dndbeyond.com/srd
---

# Augury


## Spell

| Field | Value |
| --- | --- |
| Level | 2 |
| School | Divination |
| Classes | Cleric, Druid, Wizard |
| Casting Time | 1 minute or Ritual |
| Range |  |
| Components | V, S, M (specially marked sticks, bones, cards, or other divinatory tokens worth 25+ GP) |
| Duration | Instantaneous |


## Source text

<!-- SRD page 111 -->

Level 2 Divination (Cleric, Druid, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (specially marked sticks, bones, cards, or other divinatory tokens worth 25+ GP)\
Duration: Instantaneous

You receive an omen from an otherworldly entity about the results of a course of action that you plan to take within the next 30 minutes. The GM chooses the omen from the Omens table.

**Omens**

| Omen | For Results That Will Be … |
| --- | --- |
| Weal | Good |
| Woe | Bad |
| Weal and woe | Good and bad |
| Indifference | Neither good nor bad |

The spell doesn’t account for circumstances, such as other spells, that might change the results.

If you cast the spell more than once before finishing a Long Rest, there is a cumulative 25 percent chance for each casting after the first that you get no answer.

Original layout: [[../../srd-5.2.1/pages/111|page 111]].







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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

