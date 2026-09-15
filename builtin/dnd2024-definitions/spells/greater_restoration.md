---
splat_schema: 1
splat_id: 55c81461-36e7-5f03-97ce-b861b2cdc156
splat_kind: spell
splat_key: dnd2024:spells/greater_restoration
splat_name: Greater Restoration
splat_origin:
  key: dnd2024:reference/spell/greater_restoration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 137
  url: https://www.dndbeyond.com/srd
---

# Greater Restoration

## Source text

<!-- SRD page 137 -->

Level 5 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (diamond dust worth 100+ GP,\
which the spell consumes)\
Duration: Instantaneous\
You touch a creature and magically remove one of\
the following effects from it:\
•  1 Exhaustion level\
•  The Charmed or Petrified condition\
•  A curse, including the target’s Attunement to a\
cursed magic item\
•  Any reduction to one of the target’s ability scores\
•  Any reduction to the target’s Hit Point maximum\

Original layout: [[../../srd-5.2.1/pages/137|page 137]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid", "paladin", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` diamond dust worth 100+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/greater_restoration ` |
| ` implementation ` | ` dnd2024:spell_greater_restoration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` remove_choice_count ` | ` 1 ` |

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` touch ` | ` true ` |
| ` count ` | ` 1 ` |

#### Choices

<!-- splat:rows path=splat_rule/parameters/choices -->

| Row | kind | levels | choices | include | ability_count | reduction |
| --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` exhaustion ` | ` 1 ` |  |  |  |  |
| ` 1 ` | ` condition ` |  | ` ["charmed", "petrified"] ` |  |  |  |
| ` 2 ` | ` curse ` |  |  | ` cursed_item_attunement ` |  |  |
| ` 3 ` | ` ability_score_reduction ` |  |  |  | ` 1 ` | ` any ` |
| ` 4 ` | ` hp_maximum_reduction ` |  |  |  |  | ` any ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
