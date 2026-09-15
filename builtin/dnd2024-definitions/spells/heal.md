---
splat_schema: 1
splat_id: 33726bf6-56aa-5b4e-adfd-4bde35115996
splat_kind: spell
splat_key: dnd2024:spells/heal
splat_name: Heal
splat_origin:
  key: dnd2024:reference/spell/heal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
---

# Heal

## Source text

<!-- SRD page 139 -->

Level 6 Abjuration (Cleric, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
Choose a creature that you can see within range.\
Positive energy washes through the target, restor-\
ing 70 Hit Points. This spell also ends the Blinded,\
Deafened, and Poisoned conditions on the target.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 10 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/heal ` |
| ` implementation ` | ` dnd2024:spell_heal ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` end_conditions ` | ` ["blinded", "deafened", "poisoned"] ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` visible ` | ` true ` |
| ` range_feet ` | ` 60 ` |

#### Healing

<!-- splat:values path=splat_rule/parameters/healing -->

| Field | Value |
| --- | --- |
| ` base ` | ` 70 ` |
| ` increment_per_slot ` | ` 10 ` |
| ` base_slot ` | ` 6 ` |
