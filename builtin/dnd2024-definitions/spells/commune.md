---
splat_schema: 2
splat_id: e417ecb8-8600-5893-a71a-34610f59657a
splat_kind: spell
splat_key: dnd2024:spells/commune
splat_name: Commune
splat_origin: dnd2024:reference/spell/commune
splat_sources:
- SRD 5.2.1, pp. 116, https://www.dndbeyond.com/srd
---

# Commune


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Divination |
| Classes | Cleric |
| Casting Time | 1 minute or Ritual |
| Range |  |
| Components | V, S, M (incense) |
| Duration | 1 minute |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 116 -->

Level 5 Divination (Cleric)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V, S, M (incense)\
Duration: 1 minute

You contact a deity or a divine proxy and ask up to three questions that can be answered with yes or no. You must ask your questions before the spell ends. You receive a correct answer for each question.

Divine beings aren’t necessarily omniscient, so you might receive “unclear” as an answer if a question pertains to information that lies beyond the deity’s knowledge. In a case where a one-word answer could be misleading or contrary to the deity’s interests, the GM might offer a short phrase as an answer instead.

If you cast the spell more than once before finishing a Long Rest, there is a cumulative 25 percent chance for each casting after the first that you get no answer.

Original layout: [[../../srd-5.2.1/pages/116|page 116]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/commune ` |
| ` implementation ` | ` dnd2024:spell_commune ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` contact ` | ` deity_or_divine_proxy ` |

#### Questions

<!-- splat:yaml path=splat_rule/parameters/questions -->

```yaml
max: 3
deadline: spell_end
form: yes_or_no
answers:
  correct: true
  beyond_entity_knowledge: unclear
  gm_short_phrase_allowed_when:
  - one_word_misleading
  - one_word_contrary_to_deity_interests
```

#### Repeated casting

<!-- splat:yaml path=splat_rule/parameters/repeated_casting -->

```yaml
reset: long_rest_finished
no_answer_chance_percent:
  base: 0
  additional_per_cast_after_first: 25
  maximum: 100
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  value: 1
  unit: minute
```

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

