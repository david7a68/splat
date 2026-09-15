---
splat_schema: 1
splat_id: a70a357a-c4ad-54f7-b8d4-d7db71b23f5a
splat_kind: spell
splat_key: dnd2024:spells/befuddlement
splat_name: Befuddlement
splat_origin:
  key: dnd2024:reference/spell/befuddlement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
---

# Befuddlement

## Source text

<!-- SRD page 112 -->

Level 8 Enchantment (Bard, Druid, Warlock, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a key ring with no keys)\
Duration: Instantaneous\
You blast the mind of a creature that you can see\
within range. The target makes an Intelligence sav-\
ing throw.\
On a failed save, the target takes 10d12 Psychic\
damage and can’t cast spells or take the Magic ac-\
tion. At the end of every 30 days, the target repeats\
the save, ending the effect on a success. The effect\
can also be ended by the Greater Restoration, Heal,\
or Wish spell.\
On a successful save, the target takes half as much\
damage only.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 8 ` |
| ` school ` | ` enchantment ` |
| ` classes ` | ` ["bard", "druid", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 150 ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a key ring with no keys ` |
| ` consumed ` | ` false ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/befuddlement ` |
| ` implementation ` | ` dnd2024:spell_befuddlement ` |

### Parameters

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature ` |
| ` maximum ` | ` 1 ` |
| ` within_spell_range ` | ` true ` |
| ` visible ` | ` true ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: intelligence
dc:
  binding: save_dc
failure:
  damage:
    dice:
      count: 10
      sides: 12
      add: 0
    type: psychic
  prohibit:
  - cast_spells
  - magic_action
  repeat_save:
    interval_days: 30
    success: end_effect
  end_by_spells:
  - key: dnd2024:spells/greater_restoration
  - key: dnd2024:spells/heal
  - key: dnd2024:spells/wish
success:
  damage_fraction:
    numerator: 1
    denominator: 2
  damage:
    dice:
      count: 10
      sides: 12
      add: 0
    type: psychic
```
