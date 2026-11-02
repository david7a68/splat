---
splat_schema: 1
splat_id: 9851c2a4-c7cb-5db3-b080-a0ac719edce1
splat_kind: spell
splat_key: dnd2024:spells/legend_lore
splat_name: Legend Lore
splat_origin:
  key: dnd2024:reference/spell/legend_lore
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
---

# Legend Lore

## Source text

<!-- SRD page 144 -->

Level 5 Divination (Bard, Cleric, Wizard)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (incense worth 250+ GP, which the spell consumes, and four ivory strips worth 50+ GP each)\
Duration: Instantaneous

Name or describe a famous person, place, or object. The spell brings to your mind a brief summary of the significant lore about that famous thing, as described by the GM.

The lore might consist of important details, amusing revelations, or even secret lore that has never been widely known. The more information you already know about the thing, the more precise and detailed the information you receive is. That information is accurate but might be couched in figurative language or poetry, as determined by the GM.

If the famous thing you chose isn’t actually famous, you hear sad musical notes played on a trombone, and the spell fails.

Original layout: [[../../srd-5.2.1/pages/144|page 144]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` incense worth 250+ GP, which the spell consumes, and four ivory strips worth 50+ GP each ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/legend_lore ` |
| ` implementation ` | ` dnd2024:spell_legend_lore ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` named_or_described_subject ` |
| ` categories ` | ` ["person", "place", "object"] ` |
| ` requires ` | ` famous ` |

#### Lore

<!-- splat:values path=splat_rule/parameters/lore -->

| Field | Value |
| --- | --- |
| ` recipient ` | ` caster_mind ` |
| ` responder ` | ` gm ` |
| ` content ` | ` significant_lore_summary ` |
| ` eligible_information ` | ` ["important_details", "amusing_revelations", "secret_lore"] ` |
| ` accuracy ` | ` true ` |
| ` precision_increases_with ` | ` prior_subject_knowledge ` |
| ` language_style_choice ` | ` ["literal", "figurative", "poetry"] ` |

#### Not famous

<!-- splat:values path=splat_rule/parameters/not_famous -->

| Field | Value |
| --- | --- |
| ` spell_fails ` | ` true ` |
| ` auditory_feedback ` | ` sad_trombone_notes ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
