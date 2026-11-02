---
splat_schema: 1
splat_id: 8afc8feb-d590-59b8-bf92-05c5842b3fff
splat_kind: spell
splat_key: dnd2024:spells/mass_heal
splat_name: Mass Heal
splat_origin:
  key: dnd2024:reference/spell/mass_heal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 147
  url: https://www.dndbeyond.com/srd
---

# Mass Heal

## Source text

<!-- SRD page 147 -->

Level 9 Abjuration (Cleric)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

A flood of healing energy flows from you into creatures around you. You restore up to 700 Hit Points, divided as you choose among any number of creatures that you can see within range. Creatures healed by this spell also have the Blinded, Deafened, and Poisoned conditions removed from them.

Original layout: [[../../srd-5.2.1/pages/147|page 147]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 9 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["cleric"] ` |
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
| ` mechanic ` | ` dnd2024:spells/mass_heal ` |
| ` implementation ` | ` dnd2024:spell_mass_heal ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` healed_targets_remove_conditions ` | ` ["blinded", "deafened", "poisoned"] ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` count ` | ` any_number ` |
| ` visible_to_caster ` | ` true ` |

#### Healing

<!-- splat:values path=splat_rule/parameters/healing -->

| Field | Value |
| --- | --- |
| ` pool_maximum ` | ` 700 ` |
| ` allocation_chooser ` | ` caster ` |
| ` integer_nonnegative_per_target ` | ` true ` |
