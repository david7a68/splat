---
splat_schema: 1
splat_id: 323e3e17-d106-5b63-80c4-13a44c6f983b
splat_kind: spell
splat_key: dnd2024:spells/mass_healing_word
splat_name: Mass Healing Word
splat_origin:
  key: dnd2024:reference/spell/mass_healing_word
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
---

# Mass Healing Word

## Source text

<!-- SRD page 148 -->

Level 3 Abjuration (Bard, Cleric)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous

Up to six creatures of your choice that you can see within range regain Hit Points equal to 2d4 plus your spellcasting ability modifier.

Using a Higher-Level Spell Slot. The healing increases by 1d4 for each spell slot level above 3.

Original layout: [[../../srd-5.2.1/pages/148|page 148]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 3 ` |
| ` school ` | ` abjuration ` |
| ` classes ` | ` ["bard", "cleric"] ` |
| ` components ` | ` ["verbal"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Bonus Action ` |
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
| ` mechanic ` | ` dnd2024:spells/mass_healing_word ` |
| ` implementation ` | ` dnd2024:spell_mass_healing_word ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures ` |
| ` max_count ` | ` 6 ` |
| ` chosen ` | ` true ` |
| ` visible ` | ` true ` |
| ` within_range ` | ` true ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
dice: 2d4
add:
  stat: caster_spellcasting_ability_modifier
```

#### Slot scaling

<!-- splat:values path=splat_rule/parameters/slot_scaling -->

| Field | Value |
| --- | --- |
| ` above ` | ` 3 ` |
| ` healing_per_level ` | ` 1d4 ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
