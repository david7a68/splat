---
splat_schema: 2
splat_id: 8810a2db-3adb-5a4b-a668-b8554c1f841f
splat_kind: spell
splat_key: dnd2024:spells/comprehend_languages
splat_name: Comprehend Languages
splat_origin: dnd2024:reference/spell/comprehend_languages
splat_sources:
- SRD 5.2.1, pp. 117, https://www.dndbeyond.com/srd
---

# Comprehend Languages


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Divination |
| Classes | Bard, Sorcerer, Warlock, Wizard |
| Casting Time | Action or Ritual |
| Range |  |
| Components | V, S, M (a pinch of soot and salt) |
| Duration | 1 hour |


## Source text

<!-- SRD page 117 -->

Level 1 Divination (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S, M (a pinch of soot and salt)\
Duration: 1 hour

For the duration, you understand the literal meaning of any language that you hear or see signed. You also understand any written language that you see, but you must be touching the surface on which the words are written. It takes about 1 minute to read one page of text. This spell doesn’t decode symbols or secret messages.

Original layout: [[../../srd-5.2.1/pages/117|page 117]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/comprehend_languages ` |
| ` implementation ` | ` dnd2024:spell_comprehend_languages ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` target ` | ` caster ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 3600 ` |

#### Comprehend

<!-- splat:yaml path=splat_rule/parameters/comprehend -->

```yaml
meaning: literal
heard_languages: true
seen_signed_languages: true
written_language:
  visible: true
  touching_written_surface_required: true
  reading_seconds_per_page_approximate: 60
```

#### Decode

<!-- splat:values path=splat_rule/parameters/decode -->

| Field | Value |
| --- | --- |
| ` symbols ` | ` false ` |
| ` secret_messages ` | ` false ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> range:
>   mode: self
> ```

