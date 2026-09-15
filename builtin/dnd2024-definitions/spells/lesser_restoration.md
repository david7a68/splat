---
splat_schema: 1
splat_id: f9ed276d-2674-5d36-ae3b-0e5c34192ee7
splat_kind: spell
splat_key: dnd2024:spells/lesser_restoration
splat_name: Lesser Restoration
splat_origin:
  key: dnd2024:reference/spell/lesser_restoration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 144
  url: https://www.dndbeyond.com/srd
---

# Lesser Restoration

## Source text

<!-- SRD page 144 -->

Level 2 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Bonus Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
You touch a creature and end one condition on it:\
Blinded, Deafened, Paralyzed, or Poisoned.\

Original layout: [[../../srd-5.2.1/pages/144|page 144]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 2 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid", "paladin", "ranger"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/lesser_restoration ` |
| ` implementation ` | ` dnd2024:spell_lesser_restoration ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` touched ` | ` true ` |

#### Remove condition

<!-- splat:values path=splat_rule/parameters/remove_condition -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` choices ` | ` ["blinded", "deafened", "paralyzed", "poisoned"] ` |
