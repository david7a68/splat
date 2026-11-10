---
splat_schema: 2
splat_id: 9119be78-3637-5b22-a703-f1ddd9209758
splat_kind: spell
splat_key: dnd2024:spells/illusory_script
splat_name: Illusory Script
splat_origin: dnd2024:reference/spell/illusory_script
splat_sources:
- SRD 5.2.1, pp. 142, https://www.dndbeyond.com/srd
---

# Illusory Script


## Spell

| Field | Value |
| --- | --- |
| Level | 1 |
| School | Illusion |
| Classes | Bard, Warlock, Wizard |
| Casting Time | 1 minute or Ritual |
| Range |  |
| Components | S, M (ink worth 10+ GP, which the spell consumes) |
| Duration | 10 days |
| Ritual | Yes |
| Concentration | No |


## Source text

<!-- SRD page 142 -->

Level 1 Illusion (Bard, Warlock, Wizard)\
Casting Time: 1 minute or Ritual\
Range: Touch\
Components: S, M (ink worth 10+ GP, which the spell consumes)\
Duration: 10 days

You write on parchment, paper, or another suitable material and imbue it with an illusion that lasts for the duration. To you and any creatures you designate when you cast the spell, the writing appears normal, seems to be written in your hand, and conveys whatever meaning you intended when you wrote the text. To all others, the writing appears as if it were written in an unknown or magical script that is unintelligible. Alternatively, the illusion can alter the meaning, handwriting, and language of the text, though the language must be one you know.

If the spell is dispelled, the original script and the illusion both disappear.

A creature that has Truesight can read the hidden message.

Original layout: [[../../srd-5.2.1/pages/142|page 142]].







## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/illusory_script ` |
| ` implementation ` | ` dnd2024:spell_illusory_script ` |

### Parameters

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` maximum_seconds ` | ` 864000 ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` writing_surface ` |
| ` suitable_for_writing ` | ` true ` |
| ` touch ` | ` true ` |

#### Message

<!-- splat:values path=splat_rule/parameters/message -->

| Field | Value |
| --- | --- |
| ` real_meaning ` | ` caster_authored ` |
| ` real_handwriting ` | ` caster ` |

#### Authorized readers

<!-- splat:yaml path=splat_rule/parameters/authorized_readers -->

```yaml
caster: true
additional_creatures:
  chooser: caster
  timing: cast
```

#### Unauthorized display choice

<!-- splat:values path=splat_rule/parameters/unauthorized_display_choice -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` caster ` |
| ` options ` | ` ["unintelligible_script", "alternative_text"] ` |

#### Unintelligible script

<!-- splat:values path=splat_rule/parameters/unintelligible_script -->

| Field | Value |
| --- | --- |
| ` appearance_choices ` | ` ["unknown_script", "magical_script"] ` |
| ` readable ` | ` false ` |

#### Alternative text

<!-- splat:yaml path=splat_rule/parameters/alternative_text -->

```yaml
meaning: caster_choice
handwriting: caster_choice
language:
  chooser: caster
  constraint: known_to_caster
```

#### Truesight

<!-- splat:values path=splat_rule/parameters/truesight -->

| Field | Value |
| --- | --- |
| ` can_read_hidden_message ` | ` true ` |

#### On dispel

<!-- splat:values path=splat_rule/parameters/on_dispel -->

| Field | Value |
| --- | --- |
| ` remove ` | ` ["original_script", "illusion"] ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: true
> range:
>   mode: touch
> ```

