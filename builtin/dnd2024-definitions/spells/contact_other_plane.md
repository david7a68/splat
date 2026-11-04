---
splat_schema: 2
splat_id: ab4d6e06-0f81-5659-80d3-47c3b45a54ef
splat_kind: spell
splat_key: dnd2024:spells/contact_other_plane
splat_name: Contact Other Plane
splat_origin: dnd2024:reference/spell/contact_other_plane
splat_sources:
- SRD 5.2.1, pp. 119, https://www.dndbeyond.com/srd
---

# Contact Other Plane


## Spell

| Field | Value |
| --- | --- |
| Level | 5 |
| School | Divination |
| Classes | Warlock, Wizard |
| Casting Time | 1 minute or Ritual |
| Range |  |
| Components | V |
| Duration | 1 minute |


## Source text

<!-- SRD page 119 -->

Level 5 Divination (Warlock, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Self\
Components: V\
Duration: 1 minute

You mentally contact a demigod, the spirit of a long-dead sage, or some other knowledgeable entity from another plane. Contacting this otherworldly intelligence can break your mind. When you cast this spell, make a DC 15 Intelligence saving throw. On a successful save, you can ask the entity up to five questions. You must ask your questions before the spell ends. The GM answers each question with one word, such as “yes,” “no,” “maybe,” “never,” “irrelevant,” or “unclear” (if the entity doesn’t know the answer to the question). If a one-word answer would be misleading, the GM might instead offer a short phrase as an answer.

On a failed save, you take 6d6 Psychic damage and have the Incapacitated condition until you finish a Long Rest. A Greater Restoration spell cast on you ends this effect.

Original layout: [[../../srd-5.2.1/pages/119|page 119]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/contact_other_plane ` |
| ` implementation ` | ` dnd2024:spell_contact_other_plane ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |
| ` contact ` | ` knowledgeable_extraplanar_entity ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: intelligence
dc: 15
success:
  questions_max: 5
  deadline: spell_end
  responder: gm
  answer_words: 1
  unknown_answer: unclear
  misleading_one_word: short_phrase_allowed
failure:
  damage:
    dice: 6d6
    type: psychic
  condition: incapacitated
  ends_on:
  - long_rest_finished
  - greater_restoration_applied
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
> range:
>   mode: self
> ```

