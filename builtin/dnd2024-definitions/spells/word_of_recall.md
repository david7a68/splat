---
splat_schema: 2
splat_id: f01125ab-d5e2-5211-9b98-faa790036b8c
splat_kind: spell
splat_key: dnd2024:spells/word_of_recall
splat_name: Word of Recall
splat_origin: dnd2024:reference/spell/word_of_recall
splat_sources:
- SRD 5.2.1, pp. 175, https://www.dndbeyond.com/srd
---

# Word of Recall


## Spell

| Field | Value |
| --- | --- |
| Level | 6 |
| School | Conjuration |
| Classes | Cleric |
| Casting Time | Action |
| Range | 5 feet |
| Components | V |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 175 -->

Level 6 Conjuration (Cleric)\
Casting Time: Action\
Range: 5 feet\
Components: V\
Duration: Instantaneous

You and up to five willing creatures within 5 feet of you instantly teleport to a previously designated sanctuary. You and any creatures that teleport with you appear in the nearest unoccupied space to the spot you designated when you prepared your sanctuary (see below). If you cast this spell without first preparing a sanctuary, the spell has no effect.

You must designate a location, such as a temple, as a sanctuary by casting this spell there.

Original layout: [[../../srd-5.2.1/pages/175|page 175]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/word_of_recall ` |
| ` implementation ` | ` dnd2024:spell_word_of_recall ` |

### Parameters

#### Sanctuary preparation

<!-- splat:values path=splat_rule/parameters/sanctuary_preparation -->

| Field | Value |
| --- | --- |
| ` method ` | ` cast_this_spell_at_chosen_location ` |
| ` persists_as ` | ` designated_sanctuary ` |

#### Targeting

<!-- splat:yaml path=splat_rule/parameters/targeting -->

```yaml
caster: included
other_creatures:
  max_count: 5
  willing: true
  within_caster_feet: 5
```

#### Teleport

<!-- splat:values path=splat_rule/parameters/teleport -->

| Field | Value |
| --- | --- |
| ` timing ` | ` instantaneous ` |
| ` requires ` | ` previously_prepared_sanctuary ` |
| ` destination ` | ` nearest_unoccupied_spaces_to_designated_sanctuary_spot ` |
| ` no_prepared_sanctuary ` | ` no_teleport_effect ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
