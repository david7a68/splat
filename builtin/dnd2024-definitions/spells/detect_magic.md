---
splat_schema: 1
splat_id: ac9606ad-e79a-552d-9f6e-f3a401571dfe
splat_kind: spell
splat_key: dnd2024:spells/detect_magic
splat_name: Detect Magic
splat_origin:
  key: dnd2024:reference/spell/detect_magic
splat_sources:
- title: SRD 5.2.1
  locator: pp. 123
  url: https://www.dndbeyond.com/srd
---

# Detect Magic

## Source text

<!-- SRD page 123 -->

Level 1 Divination (Bard, Cleric, Druid, Paladin,\
Ranger, Sorcerer, Warlock, Wizard)\
Casting Time: Action or Ritual\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
For the duration, you sense the presence of magical\
effects within 30 feet of yourself. If you sense such\
effects, you can take the Magic action to see a faint\
aura around any visible creature or object in the\
area that bears the magic, and if an effect was cre-\
ated by a spell, you learn the spell’s school of magic.\
The spell is blocked by 1 foot of stone, dirt, or\
wood; 1 inch of metal; or a thin sheet of lead.\

Original layout: [[../../srd-5.2.1/pages/123|page 123]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` divination ` |
| ` classes ` | ` ["bard", "cleric", "druid", "paladin", "ranger", "sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Action or Ritual ` |
| ` ritual ` | ` true ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/detect_magic ` |
| ` implementation ` | ` dnd2024:spell_detect_magic ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### Sense

<!-- splat:values path=splat_rule/parameters/sense -->

| Field | Value |
| --- | --- |
| ` radius_feet ` | ` 30 ` |
| ` subject ` | ` magical_effect_presence ` |
| ` duration ` | ` spell_duration ` |

#### Reveal

<!-- splat:values path=splat_rule/parameters/reveal -->

| Field | Value |
| --- | --- |
| ` requires ` | ` magic_sensed ` |
| ` economy ` | ` magic_action ` |
| ` aura_on ` | ` visible_creatures_and_objects_bearing_magic ` |
| ` spell_created_effect ` | ` reveal_school ` |

#### Blocked by

<!-- splat:rows path=splat_rule/parameters/blocked_by -->

| Row | material | thickness_inches | thickness |
| --- | --- | --- | --- |
| ` 0 ` | ` stone ` | ` 12 ` |  |
| ` 1 ` | ` dirt ` | ` 12 ` |  |
| ` 2 ` | ` wood ` | ` 12 ` |  |
| ` 3 ` | ` metal ` | ` 1 ` |  |
| ` 4 ` | ` lead ` |  | ` thin_sheet ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```
