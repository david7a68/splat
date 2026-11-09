---
splat_schema: 2
splat_id: a70a357a-c4ad-54f7-b8d4-d7db71b23f5a
splat_kind: spell
splat_key: dnd2024:spells/befuddlement
splat_name: Befuddlement
splat_origin: dnd2024:reference/spell/befuddlement
splat_sources:
- SRD 5.2.1, pp. 112, https://www.dndbeyond.com/srd
---

# Befuddlement


## Spell

| Field | Value |
| --- | --- |
| Level | 8 |
| School | Enchantment |
| Classes | Bard, Druid, Warlock, Wizard |
| Casting Time | Action |
| Range | 150 feet |
| Components | V, S, M (a key ring with no keys) |
| Duration | Instantaneous |
| Ritual | No |
| Concentration | No |


## Source text

<!-- SRD page 112 -->

Level 8 Enchantment (Bard, Druid, Warlock, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (a key ring with no keys)\
Duration: Instantaneous

You blast the mind of a creature that you can see within range. The target makes an Intelligence saving throw.

On a failed save, the target takes 10d12 Psychic damage and can’t cast spells or take the Magic action. At the end of every 30 days, the target repeats the save, ending the effect on a success. The effect can also be ended by the Greater Restoration, Heal, or Wish spell.

On a successful save, the target takes half as much damage only.

Original layout: [[../../srd-5.2.1/pages/112|page 112]].







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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_spell_extras -->
> 
> ```yaml
> material:
>   consumed: false
> ```

